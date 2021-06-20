use web_sys::{GainNode, AudioContext};
use crate::errors::Error;
use std::{
    rc::Rc,
    cell::RefCell,
    rc::Weak,
};
use beach_map::{BeachMap, DefaultVersion, ID};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use super::clip::*;

pub struct AudioMixer {
    //All operations need to go through with_ctx
    //So that we can lazy-load it
    ctx: RefCell<Option<Context>>,
    //in the context of a mixer we want to be able to pass around
    //simple handle/ids instead of the clips directly
    //so a lookup is maintained and handles are given out
    clip_lookup: ClipLookup,

}

pub struct AudioHandle {
    pub id: Id,
    pub(super) clip_lookup: ClipLookup,
}

pub type Id = ID<DefaultVersion>;

type ClipLookup = Rc<RefCell<BeachMap<DefaultVersion, AudioClip>>>;

pub struct Context {
    pub audio: AudioContext,
    pub gain: GainNode
}


impl AudioMixer {

    /// Create a new AudioMixer with optional pre-instantiated AudioContext
    pub fn new(ctx: Option<AudioContext>) -> Self {

        Self {
            ctx: RefCell::new(ctx.map(|audio| Context::new(audio).unwrap_throw())),
            clip_lookup: Rc::new(RefCell::new(BeachMap::new())),
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
    pub fn set_gain(&self, value: f32) {
        self.with_ctx(|ctx| {
            ctx.gain.gain().set_value_at_time(value, ctx.audio.current_time());
        })
    }

    /// Helper in case the AudioContext is needed on the outside
    pub fn clone_audio_ctx(&self) -> AudioContext {
        self.with_ctx(|ctx| ctx.audio.clone())
    }

    /// Just used for testing mostly
    pub fn suspend(&self) {
        self.with_ctx(|ctx| {
            let promise = ctx.audio.suspend().unwrap_throw();
            spawn_local(async move {
                let _ = JsFuture::from(promise).await;
            });
        });
    }

    /// Just used for testing mostly
    pub fn resume(&self) {
        self.with_ctx(|ctx| {
            let promise = ctx.audio.resume().unwrap_throw();
            spawn_local(async move {
                let _ = JsFuture::from(promise).await;
            });
        });
    }

    //Lazy-creates the AudioContext and GainNode just in time
    pub fn with_ctx<A>(&self, f: impl FnOnce(&Context) -> A) -> A {
        let mut ctx = self.ctx.borrow_mut();
        if let Some(ctx) = ctx.as_ref() {
            f(ctx)
        } else {
            let new_ctx = Context::new(AudioContext::new().unwrap_throw()).unwrap_throw();
            let ret = f(&new_ctx);
            *ctx = Some(new_ctx);
            ret
        }
    }

    /// Oneshots are AudioClips because they drop themselves
    /// They're intended solely to be kicked off and not being held anywhere
    /// However, if necessary, they can still be killed imperatively 
    pub fn play_oneshot<F>(&self, source: AudioSource, on_ended: Option<F>) -> Result<AudioClip, Error> 
    where
        F: FnMut() -> () + 'static,

    {
        self.with_ctx(|ctx| {
            AudioClip::new_oneshot(
                &ctx.audio, 
                source, 
                ctx.gain.clone().unchecked_into(),
                AudioClipOptions {
                    auto_play: true,
                    is_loop: false,
                    on_ended, 
                })
        })

    }


    /// Play a clip and get a Handle to hold (simple API around add_source)
    pub fn play<F>(&self, source: AudioSource, is_loop: bool) -> Result<AudioHandle, Error> 
    where
        F: FnMut() -> () + 'static,

    {
        let clip = self.with_ctx(|ctx| {
            AudioClip::new(
                &ctx.audio, 
                source, 
                ctx.gain.clone().unchecked_into(),
                AudioClipOptions {
                    auto_play: true,
                    is_loop,
                    on_ended: None::<fn()>, 
                })
        })?;

        self.add_clip(clip)
    }

    /// Add a source with various options and get a Handle to hold
    pub fn add_source<F>(&self, source: AudioSource, options: AudioClipOptions<F>) -> Result<AudioHandle, Error> 
    where
        F: FnMut() -> () + 'static,

    {
        let clip = self.with_ctx(|ctx| {
            AudioClip::new(&ctx.audio, source, ctx.gain.clone().unchecked_into(), options) 
        })?;

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
