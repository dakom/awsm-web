use crate::errors::{Error};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{AudioBuffer, AudioBufferSourceNode, MediaElementAudioSourceNode, HtmlAudioElement, AudioContext};
use std::rc::Rc;
use std::cell::RefCell;

pub struct AudioPlayer {
    pub source: AudioSource, 
    pub cb: Option<Closure<dyn FnMut() -> ()>>,
}

pub enum AudioSource {
    Buffer(AudioBufferSourceNode),
    Element(HtmlAudioElement, MediaElementAudioSourceNode),
}

impl AudioPlayer {

    //uses an element internally
    //maybe in the future use streaming api?
    pub fn play_url<F>(ctx: &AudioContext, url:&str, on_ended: Option<F>) -> Result<Self, Error>
        
    where
        F: FnMut() -> () + 'static,
    {
        let elem = HtmlAudioElement::new_with_src(url)?;
        elem.set_autoplay(true);
        let cb: Option<Closure<dyn FnMut() -> ()>> = match on_ended {
            Some(f) => {
                let cb = Closure::wrap(Box::new(f) as Box<dyn FnMut() -> ()>);
                elem.set_onended(Some(cb.as_ref().unchecked_ref()));
                Some(cb)
            }
            None => None,
        };

        let node = ctx.create_media_element_source(&elem)?;

        node.connect_with_audio_node(&ctx.destination())?;

        Ok(Self { source: AudioSource::Element(elem, node), cb })
    }


    pub fn play_buffer<F>( ctx: &AudioContext, buffer: &AudioBuffer, on_ended: Option<F>,) -> Result<Self, Error>
    where
        F: FnMut() -> () + 'static,
    {
        let node = ctx.create_buffer_source()?;

        node.set_buffer(Some(buffer));
        node.connect_with_audio_node(&ctx.destination())?;

        let cb: Option<Closure<dyn FnMut() -> ()>> = match on_ended {
            Some(f) => {
                let cb = Closure::wrap(Box::new(f) as Box<dyn FnMut() -> ()>);
                node.set_onended(Some(cb.as_ref().unchecked_ref()));
                Some(cb)
            }
            None => None,
        };


        node.start()?;

        Ok(Self { source: AudioSource::Buffer(node), cb })
    }

    pub fn set_loop(&self, do_loop:bool) {
        match &self.source {
            AudioSource::Buffer(node) => node.set_loop(do_loop),
            AudioSource::Element(element, node) => element.set_loop(do_loop),
        }
    }
    //A regular audio player is effectively a one-shot since dropping will stop it
    //But it can be annoying to need to keep it around in memory until playing is finished
    //So this one-shot will drop itself when finished
    //It can still be force-dropped by calling borrow_mut().take on the result (see example)
    pub fn play_oneshot_buffer<F>(
        ctx: &AudioContext,
        buffer: &AudioBuffer,
        on_ended: Option<F>,
    ) -> Result<Rc<RefCell<Option<AudioPlayer>>>, Error>
    where
        F: FnMut() -> () + 'static,
    {
        let player = Rc::new(RefCell::new(None));
        let on_ended = Rc::new(RefCell::new(on_ended));

        let _player = AudioPlayer::play_buffer(ctx, buffer, Some({
            let player = Rc::clone(&player);
            move || {
                on_ended.borrow_mut().as_mut().map(|cb| cb());
                player.borrow_mut().take();
            }
        }))?;

        *player.borrow_mut() = Some(_player);

        Ok(player)
    }

    pub fn play_oneshot_url<F>(
        ctx: &AudioContext,
        url: &str,
        on_ended: Option<F>,
    ) -> Result<Rc<RefCell<Option<AudioPlayer>>>, Error>
    where
        F: FnMut() -> () + 'static,
    {
        let player = Rc::new(RefCell::new(None));
        let on_ended = Rc::new(RefCell::new(on_ended));

        let _player = AudioPlayer::play_url(ctx, url, Some({
            let player = Rc::clone(&player);
            move || {
                on_ended.borrow_mut().as_mut().map(|cb| cb());
                player.borrow_mut().take();
            }
        }))?;

        *player.borrow_mut() = Some(_player);

        Ok(player)
    }
}

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        match &self.source {
            AudioSource::Buffer(node) => {
                node.disconnect();
                node.stop().unwrap_throw();
                node.set_onended(None);
            },
            AudioSource::Element(element, node) => {
                node.disconnect();
                element.pause();
                element.set_onended(None);
            }
        };

        self.cb.take();
    }
}

/*

pub struct AudioOneShot {
    pub player: Rc<RefCell<Option<AudioPlayer>>>,
}

impl AudioOneShot {
    pub fn play<F>(
        ctx: &AudioContext,
        buffer: &AudioBuffer,
        on_ended: Option<F>,
    ) -> Result<Self, Error>
    where
        F: FnMut() -> () + 'static,
    {

        let player = Rc::new(RefCell::new(None));
        let on_ended = Rc::new(RefCell::new(on_ended));

        let _player = AudioPlayer::play_buffer(ctx, buffer, Some({
            let player = Rc::clone(&player);
            move || {
                on_ended.borrow_mut().as_mut().map(|cb| cb());
                player.borrow_mut().take();
            }
        }))?;

        *player.borrow_mut() = Some(_player);

        Ok(Self{
            player
        })
    }

}
*/
