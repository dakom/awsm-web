use web_sys::{GainNode, AudioContext, AudioContextState};
use crate::errors::{Error, NativeError};
use beach_map::{BeachMap, DefaultVersion, ID};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use super::clip::*;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct AudioMixer {
    //All operations need to go through try_with_ctx or with_ctx_unchecked
    //So that we can lazy-load it
    ctx: Rc<RefCell<Option<Context>>>,
    //in the context of a mixer we want to be able to pass around
    //simple handle/ids instead of the clips directly
    //so a lookup is maintained and handles are given out
    clip_lookup: ClipLookup,

    /// Treats a suspended AudioContext as invalid, closes it
    /// and creates a new AudioContext as needed
    /// this is set to true by default (i.e. when creating with new())
    pub close_suspended: bool 

}

#[derive(Clone)]
pub struct AudioHandle {
    pub id: Id,
    pub(super) clip_lookup: ClipLookup,
}

#[derive(Clone)]
pub struct WeakAudioHandle {
    pub id: Id,
    pub(super) clip_lookup: WeakClipLookup,
}

pub type Id = ID<DefaultVersion>;

type ClipLookup = Rc<RefCell<BeachMap<DefaultVersion, AudioClip>>>;
type WeakClipLookup = Weak<RefCell<BeachMap<DefaultVersion, AudioClip>>>;

pub struct Context {
    pub audio: AudioContext,
    pub gain: GainNode
}


impl AudioMixer {

    /// Create a new AudioMixer with optional pre-instantiated AudioContext
    pub fn new(ctx: Option<AudioContext>) -> Self {

        Self {
            ctx: Rc::new(RefCell::new(ctx.map(|audio| Context::new(audio).unwrap_throw()))),
            clip_lookup: Rc::new(RefCell::new(BeachMap::new())),
            close_suspended: true,
        }
    }

    /// Pause all the clips (properly, not via suspend)
    pub fn pause_all(&self) {
        for clip in self.clip_lookup.borrow().iter() {
            clip.pause();
        }
    }

    /// Play all the clips (properly, not via resume)
    pub fn play_all(&self) {
        for clip in self.clip_lookup.borrow().iter() {
            clip.play();
        }
    }

    /// Set the gain/volume
    pub fn try_set_gain(&self, value: f32) -> Result<(), Error> {
        self.try_with_ctx(|ctx| {
            ctx.gain.gain().set_value_at_time(value, ctx.audio.current_time());
        })
    }
    pub fn set_gain(&self, value: f32) {
        self.try_set_gain(value).unwrap_throw();
    }

    /// Helper in case the AudioContext is needed on the outside
    pub fn try_clone_audio_ctx(&self) -> Result<AudioContext, Error> {
        self.try_with_ctx(|ctx| ctx.audio.clone())
    }
    pub fn clone_audio_ctx(&self) -> AudioContext {
        self.try_clone_audio_ctx().unwrap_throw()
    }

    /// Mostly just used for testing.
    pub fn suspend_then(&self, f: impl FnOnce() + 'static) {
        self.try_with_ctx(move |ctx| {
            let promise = ctx.audio.suspend().unwrap_throw();
            spawn_local(async move {
                let _ = JsFuture::from(promise).await;
                f();
            });
        });
    }

    /// Mostly just used for testing.
    pub fn resume_then(&self, f: impl FnOnce() + 'static) {
        self.try_with_ctx(move |ctx| {
            let promise = ctx.audio.resume().unwrap_throw();
            spawn_local(async move {
                let _ = JsFuture::from(promise).await;
                f();
            });
        });
    }

    pub fn context_available(&self) -> bool {
        self.try_with_ctx(|_| true).unwrap_or(false)
    }

    //Lazy-creates the AudioContext and GainNode just in time
    pub fn with_ctx_unchecked<A>(&self, f: impl FnOnce(&Context) -> A) -> A {
        self.try_with_ctx(f).unwrap_throw()
    }

    pub fn try_with_ctx<A>(&self, f: impl FnOnce(&Context) -> A) -> Result<A, Error> {
        let mut lock = self.ctx.borrow_mut();

        if lock.is_none() {
            *lock = Some(Context::new(AudioContext::new().unwrap_throw()).unwrap_throw());
        }

        let ctx = lock.as_ref().unwrap_throw();

        match ctx.audio.state() {
            AudioContextState::Suspended => {
                if self.close_suspended {
                    ctx.audio.close();
                    
                    //try again..
                    let audio_ctx = AudioContext::new().unwrap_throw();
                    match audio_ctx.state() {
                        AudioContextState::Running => {
                            let ctx = Context::new(audio_ctx).unwrap_throw();
                            let ret = f(&ctx);
                            *lock = Some(ctx);
                            Ok(ret)
                        },
                        _ => {
                            *lock = None;
                            Err(Error::Native(NativeError::AudioContext))
                        }
                    }
                } else {
                    Ok(f(&ctx))
                }
            },
            AudioContextState::Running => {
                Ok(f(&ctx))
            },
            _ => {
                *lock = None;
                Err(Error::Native(NativeError::AudioContext))
            }
        }
        
    }

    /// Oneshots are AudioClips because they drop themselves
    /// They're intended solely to be kicked off and not being held anywhere
    /// However, if necessary, they can still be killed imperatively 
    pub fn play_oneshot<A: Into<AudioSource>>(&self, source: A) -> Result<WeakAudioHandle, Error> 
    {
        let handle_ref:Rc<RefCell<Option<WeakAudioHandle>>> = Rc::new(RefCell::new(None));
        let handle_ref_clone = handle_ref.clone();

        let clip = self.try_with_ctx(|ctx| {
            AudioClip::new_oneshot(
                &ctx.audio, 
                source, 
                ctx.gain.clone().unchecked_into(),
                AudioClipOptions {
                    auto_play: true,
                    is_loop: false,
                    on_ended: Some(move || {
                        let mut handle = handle_ref_clone.borrow_mut().take().unwrap();
                        handle.kill();
                    }), 
                })
        })
        .and_then(|x| x)?;

        let handle = self.add_weak_clip(clip)?;

        *handle_ref.borrow_mut() = Some(handle.clone());

        Ok(handle)


    }

    pub fn play_oneshot_on_ended<F, A>(&self, source: A, mut on_ended: F) -> Result<WeakAudioHandle, Error> 
    where
        F: FnMut() -> () + 'static,
        A: Into<AudioSource>
    {
        let handle_ref:Rc<RefCell<Option<WeakAudioHandle>>> = Rc::new(RefCell::new(None));
        let handle_ref_clone = handle_ref.clone();

        let clip = self.try_with_ctx(|ctx| {
            AudioClip::new_oneshot(
                &ctx.audio, 
                source, 
                ctx.gain.clone().unchecked_into(),
                AudioClipOptions {
                    auto_play: true,
                    is_loop: false,
                    on_ended: Some(move || {
                        let mut handle = handle_ref_clone.borrow_mut().take().unwrap();
                        handle.kill();
                        on_ended();
                    }), 
                })
        })
        .and_then(|x| x)?;

        let handle = self.add_weak_clip(clip)?;

        *handle_ref.borrow_mut() = Some(handle.clone());

        Ok(handle)
    }


    /// Play a clip and get a Handle to hold (simple API around add_source)
    pub fn play<A: Into<AudioSource>>(&self, source: A, is_loop: bool) -> Result<AudioHandle, Error> 
    {
        let clip = self.try_with_ctx(|ctx| {
            AudioClip::new(
                &ctx.audio, 
                source, 
                ctx.gain.clone().unchecked_into(),
                AudioClipOptions {
                    auto_play: true,
                    is_loop,
                    on_ended: None::<fn()>, 
                })
        })
        .and_then(|x| x)?;

        self.add_clip(clip)
    }
    /// Play a clip and get a Handle to hold, with on_ended handler (simple API around add_source)
    pub fn play_on_ended<F, A>(&self, source: A, is_loop: bool, on_ended: F) -> Result<AudioHandle, Error>
    where
        F: FnMut() -> () + 'static, 
        A: Into<AudioSource>
    {
        let clip = self.try_with_ctx(|ctx| {
            AudioClip::new(
                &ctx.audio, 
                source, 
                ctx.gain.clone().unchecked_into(),
                AudioClipOptions {
                    auto_play: true,
                    is_loop,
                    on_ended: Some(on_ended), 
                })
        })
        .and_then(|x| x)?;

        self.add_clip(clip)
    }

    /// Add a source with various options and get a Handle to hold
    pub fn add_source<F, A: Into<AudioSource>>(&self, source: A, options: AudioClipOptions<F>) -> Result<AudioHandle, Error> 
    where
        F: FnMut() -> () + 'static,

    {
        let clip = self.try_with_ctx(|ctx| {
            AudioClip::new(&ctx.audio, source, ctx.gain.clone().unchecked_into(), options) 
        })
        .and_then(|x| x)?;

        self.add_clip(clip)
    }

    fn add_clip(&self, clip: AudioClip) -> Result<AudioHandle, Error> {
        let id = self.clip_lookup.borrow_mut().insert(clip);
        let handle = AudioHandle {
            id,
            clip_lookup: self.clip_lookup.clone()
        };

        Ok(handle)
    }
    fn add_weak_clip(&self, clip: AudioClip) -> Result<WeakAudioHandle, Error> {
        let id = self.clip_lookup.borrow_mut().insert(clip);
        let handle = WeakAudioHandle {
            id,
            clip_lookup: Rc::downgrade(&self.clip_lookup)
        };

        Ok(handle)
    }
}

impl Context {
    pub fn new(audio: AudioContext) -> Result<Self, Error> {
        let gain = GainNode::new(&audio)?;
        gain.connect_with_audio_node(&audio.destination())?;

        Ok(Self {
            audio,
            gain,
        })
    }
}



impl AudioHandle {
    /// Convenience method to pause an individual Handle
    pub fn pause(&self) -> Option<Result<(), Error>> {
        self.with_clip(|clip| clip.pause()).flatten()
    }

    /// Convenience method to play an individual Handle
    pub fn play(&self) -> Option<Result<(), Error>> {
        self.with_clip(|clip| clip.play()).flatten()
    }

    fn with_clip<A>(&self, f: impl FnOnce(&AudioClip) -> A) -> Option<A> {
        if let Some(clip) = self.clip_lookup.borrow().get(self.id) {
            Some(f(clip))
        } else {
            None
        }
    }
}

impl Drop for AudioHandle {
    fn drop(&mut self) {
        if let Some(clip) = self.clip_lookup.borrow_mut().remove(self.id) {
            //AudioHandle shouldn't be used to make a one-shot
            //but kill it just in case
            clip.force_kill_oneshot();
        }
    }
}

//WeakAudioHandle is just meant for oneshots
//so the only thing it needs to be able to do is kill imperatively  

impl WeakAudioHandle {
    pub fn kill(&mut self) {
        if let Some(clip_lookup) = self.clip_lookup.upgrade() {
            if let Some(clip) = clip_lookup.borrow_mut().remove(self.id) {
                clip.force_kill_oneshot();
            }
        }
    }
}
