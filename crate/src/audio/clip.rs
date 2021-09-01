use crate::errors::Error;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::window::same_origin;
use web_sys::{AudioBuffer, AudioBufferSourceNode, AudioNode, MediaElementAudioSourceNode, HtmlAudioElement, AudioContext};
use std::sync::atomic::{AtomicU64, Ordering};
use std::{rc::Rc, cell::RefCell};

pub enum AudioClip {
    Regular(AudioClipState),
    OneShot(Rc<RefCell<Option<AudioClipState>>>)
}



pub enum AudioSource {
    Url(String),
    Buffer(AudioBuffer),
}

pub struct AudioClipOptions <F>
where
    F: FnMut() -> () + 'static,

{
    pub auto_play: bool,
    pub is_loop: bool,
    pub on_ended: Option<F>
}


pub struct AudioClipState {
    destination: AudioNode,
    ctx: AudioContext,
    is_loop: bool,
    source_node: AudioSourceNode, 
    on_ended: Option<OnEndedCallback>,
    _start_offset: AtomicU64,
    _start_time: AtomicU64, 
}


type OnEndedCallback = Closure<dyn FnMut() -> ()>;

enum AudioSourceNode {
    Buffer(AudioBuffer, RefCell<Option<AudioBufferSourceNode>>),
    Element(String, HtmlAudioElement, MediaElementAudioSourceNode),
    //AudioWorkletNode for streaming?
}

impl AudioClip {
    pub fn new<F, A: Into<AudioSource>>(ctx: &AudioContext, source: A, destination: AudioNode, options: AudioClipOptions<F>) -> Result<Self, Error>  
    where
        F: FnMut() -> () + 'static,
    {
        Ok(Self::Regular(AudioClipState::new(ctx, source, destination, options)?))
    }

    pub fn new_oneshot<F, A: Into<AudioSource>>(ctx: &AudioContext, source: A, destionation: AudioNode, options: AudioClipOptions<F>) -> Result<Self, Error>  
    where
        F: FnMut() -> () + 'static,
    {
        Ok(Self::OneShot(AudioClipState::new_oneshot(ctx, source, destionation, options)?))
    }

    // oneshots are usually meant to just be fired off and forgotten
    // but if they really need to be killed, dropping isn't enough
    // because they manage their own lifecycle
    // dropping the clip and calling this will do the trick though
    // (it's intentionally not implementd on Drop so that it won't be killed immediately)
    pub fn force_kill_oneshot(&self) {
        match self {
            Self::OneShot(state) => {
                state.borrow_mut().take();
            },
            _ => {}
        }
    }

    pub fn pause(&self) -> Option<Result<(), Error>> {
        self.with_state(|state| {
            state.pause()
        })
    }

    pub fn play(&self) -> Option<Result<(), Error>> {
        self.with_state(|state| {
            state.play()
        })
    }

    fn with_state<A>(&self, f: impl FnOnce(&AudioClipState) -> A) -> Option<A> {
        match &self {
            AudioClip::Regular(state) => Some(f(state)),
            AudioClip::OneShot(clip) => {
                if let Some(audio) = clip.borrow().as_ref() {
                    Some(f(audio))
                } else {
                    None
                }
            }
        }
    }
}


impl AudioClipState {

    pub fn new<F, A: Into<AudioSource>>(ctx: &AudioContext, source: A, destination: AudioNode, options: AudioClipOptions<F>) -> Result<Self, Error> 
    where
        F: FnMut() -> () + 'static,
    {

        let _self = match source.into() {
            AudioSource::Url(url) => {
                //uses an element internally
                //maybe in the future use streaming api?


                //seems a bit more stable in terms of CORS issues to set
                //src only after setting cross_origin
                let elem = HtmlAudioElement::new()?;

                let has_same_origin = same_origin(&url)?;
                if !has_same_origin {
                    elem.set_cross_origin(Some(&"anonymous"));
                }
                elem.set_autoplay(options.auto_play);
               
                elem.set_loop(options.is_loop);

                let on_ended:Option<OnEndedCallback> = options.on_ended.map(|f| Closure::wrap(Box::new(f) as _));

                if let Some(on_ended) = on_ended.as_ref() {
                    elem.set_onended(Some(on_ended.as_ref().unchecked_ref()));
                }

                let node = ctx.create_media_element_source(&elem)?;

                node.connect_with_audio_node(&destination)?;

                elem.set_src(&url);

                Self { 
                    destination,
                    _start_offset: AtomicU64::new(0),
                    _start_time: AtomicU64::new(if options.auto_play { ctx.current_time().to_bits() } else { 0 }),
                    ctx: ctx.clone(),
                    is_loop: options.is_loop,
                    on_ended,
                    source_node: AudioSourceNode::Element(url, elem, node), 
                }
            },
            AudioSource::Buffer(buffer) => {

                let on_ended:Option<OnEndedCallback> = options.on_ended.map(|f| Closure::wrap(Box::new(f) as _));

                let _self = Self { 
                    destination,
                    _start_offset: AtomicU64::new(0),
                    //need this too? _start_time: AtomicU64::new(if options.auto_play { ctx.current_time().to_bits() } else { 0 }),
                    _start_time: AtomicU64::new(0),
                    ctx: ctx.clone(),
                    is_loop: options.is_loop,
                    on_ended,
                    source_node: AudioSourceNode::Buffer(
                        buffer, 
                        RefCell::new(None),
                    ), 
                };


                if options.auto_play {
                    _self.play()?; 
                }

                _self

            }
        };

        Ok(_self)
    }

    fn get_start_offset(&self) -> f64 {
        let value:u64 = self._start_offset.load(Ordering::SeqCst);
        f64::from_bits(value)
    }

    fn get_start_time(&self) -> f64 {
        let value:u64 = self._start_time.load(Ordering::SeqCst);
        f64::from_bits(value)
    }

    fn set_start_offset(&self, value: f64) {
        let value = value.to_bits();
        self._start_offset.store(value, Ordering::SeqCst);
    }

    fn set_start_time(&self, value: f64) {
        let value = value.to_bits();
        self._start_time.store(value, Ordering::SeqCst);
    }

    //A regular audio clip is effectively a one-shot since dropping will stop it
    //But it can be annoying to need to keep it around in memory until playing is finished
    //So this one-shot will drop itself when finished
    //(the state is imperatively dropped via the clip's Drop impl too)
    pub fn new_oneshot<F, A: Into<AudioSource>>(ctx: &AudioContext, source: A, destination: AudioNode, options: AudioClipOptions<F>) -> Result<Rc<RefCell<Option<Self>>>, Error>
    where
        F: FnMut() -> () + 'static,
    {
        let state = Rc::new(RefCell::new(None));
        let on_ended = Rc::new(RefCell::new(options.on_ended));

        let _state = Self::new(
            ctx, 
            source, 
            destination,
            AudioClipOptions {
                auto_play: options.auto_play,
                is_loop: options.is_loop,
                on_ended: Some({
                    let state = Rc::clone(&state);
                    move || {
                        on_ended.borrow_mut().as_mut().map(|cb| cb());
                        state.borrow_mut().take();
                    }
                })
            }
        )?;

        *state.borrow_mut() = Some(_state);

        Ok(state)

    }


    // https://books.google.co.il/books?id=eSPyRuL8b7UC&pg=PA14&lpg=PA14&dq=WebAudioApi+pause+source&source=bl&ots=ZezP65-Qtk&sig=ACfU3U1Kx9N2vBtXbxt1ysq6RLTkfuwvBA&hl=en&sa=X&ved=2ahUKEwj5oYyf9qDxAhWH-aQKHQ8tBGQQ6AEwCXoECBcQAw#v=onepage&q=WebAudioApi%20pause%20source&f=false
    pub fn pause(&self) -> Result<(), Error> {
        let diff = self.ctx.current_time() - self.get_start_time();
        let start_offset = self.get_start_offset() + diff;
        self.set_start_offset(start_offset);

        match &self.source_node {
            AudioSourceNode::Buffer(buffer, node_ref) => {
                if let Some(node) = node_ref.borrow_mut().take() {
                    node.stop()?;
                }
            }
               
            AudioSourceNode::Element(url, element, node) => {
                //TODO - update state based on promise resolution?
                element.pause();
            }
        }

        Ok(())
    }

    pub fn play(&self) -> Result<(), Error> {
        self.set_start_time(self.ctx.current_time());
        let start_offset = self.get_start_offset();

        match &self.source_node {
            AudioSourceNode::Buffer(buffer, node_ref) => {
                node_ref.borrow_mut().take();

                let ctx = &self.ctx;
                let node = ctx.create_buffer_source()?;

                node.set_buffer(Some(&buffer));
                node.connect_with_audio_node(&self.destination)?;

                if self.is_loop { 
                    node.set_loop(true);
                }

                if let Some(on_ended) = self.on_ended.as_ref() {
                    node.set_onended(Some(on_ended.as_ref().unchecked_ref()));
                }


                if start_offset > 0.0 {
                    let playhead = start_offset % buffer.duration();
                    node.start_with_when_and_grain_offset(0.0, playhead).unwrap_throw();
                } else {
                    node.start()?;
                }
                
                *node_ref.borrow_mut() = Some(node);
            }
               
            AudioSourceNode::Element(url, element, node) => {
                if start_offset > 0.0 {
                    let playhead = start_offset % element.duration();
                    element.set_current_time(playhead);
                }
                //TODO - update state based on promise resolution?
                element.play();
            }
        }

        Ok(())
    }

}


impl Drop for AudioClipState {
    fn drop(&mut self) {
        match &self.source_node {
            AudioSourceNode::Buffer(buffer, node_ref) => {
                if let Some(node) = node_ref.borrow_mut().as_mut() {
                    node.disconnect();
                    node.stop().unwrap_throw();
                    node.set_onended(None);
                } else {
                    log::info!("no node!!");
                }
            },
            AudioSourceNode::Element(url, element, node) => {
                node.disconnect();
                element.pause();
                element.set_onended(None);
            }
        };

    }
}

