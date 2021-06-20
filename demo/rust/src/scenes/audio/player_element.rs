use crate::router::get_static_href;
use awsm_web::audio::{AudioMixer, AudioHandle, AudioSource, AudioClip, AudioClipOptions};
use awsm_web::loaders::fetch::fetch_url;
use gloo_events::EventListener;
use log::info;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use web_sys::{Document, Element, HtmlElement, Window};

struct State {
    bg_loop: bool,
    regular: bool,
    oneshot: bool,
    stop_is_pause: bool,
    pause_is_clip: bool,
}

impl State {
    fn new() -> Self {
        Self {
            bg_loop: false,
            regular: false,
            oneshot: false,
            stop_is_pause: false,
            pause_is_clip: false
        }
    }
}

pub fn start(_window: Window, document: Document, body: HtmlElement) -> Result<(), JsValue> {
    let container: Element = document.create_element("div")?.into();
    container.set_class_name("audio-player");
    body.append_child(&container)?;

    let loading: Element = document.create_element("div")?.into();
    loading.set_class_name("audio-player-loading");
    loading.set_text_content(Some("loading audio..."));
    container.append_child(&loading)?;

    let mixer = Rc::new(AudioMixer::new(None));

    let future = async move {
        let ctx = mixer.clone_audio_ctx();

        //let bg_loop_url = get_static_href("loop.mp3");
        let bg_loop_url = get_static_href("count.mp3");
        let one_shot_url = get_static_href("oneshot.mp3");
        let regular_url = get_static_href("oneshot.mp3");

        container.remove_child(&loading)?;

        let play_loop = create_button(&document, &container, "")?;
        let play_regular = create_button(&document, &container, "")?;
        let play_oneshot = create_button(&document, &container, "")?;
        let stop_is_pause = create_button(&document, &container, "")?;
        let pause_is_clip = create_button(&document, &container, "")?;

        let render_state = {
            let play_loop = play_loop.clone();
            let play_regular = play_regular.clone();
            let play_oneshot = play_oneshot.clone();
            let stop_is_pause = stop_is_pause.clone();
            let pause_is_clip = pause_is_clip.clone();

            move |state: &State| {
                match state.bg_loop {
                    true => play_loop.set_text_content(Some("stop loop")),
                    false => play_loop.set_text_content(Some("play loop")),
                };
                match state.regular {
                    true => play_regular.set_text_content(Some("stop regular")),
                    false => play_regular.set_text_content(Some("play regular")),
                };

                match state.oneshot {
                    true => play_oneshot.set_text_content(Some("stop oneshot")),
                    false => play_oneshot.set_text_content(Some("play oneshot")),
                };

                match state.stop_is_pause {
                    true => stop_is_pause.set_text_content(Some("stop is pause")),
                    false => stop_is_pause.set_text_content(Some("stop is drop")),
                };
                match state.pause_is_clip {
                    true => pause_is_clip.set_text_content(Some("pause is clip")),
                    false => pause_is_clip.set_text_content(Some("pause is suspend")),
                };
            }
        };

        let state = Rc::new(RefCell::new(State::new()));
        render_state(&state.borrow());

        let mut bg_handle: Option<AudioHandle> = None;
        let mut regular_handle: Option<AudioHandle> = None;
        let mut oneshot_clip: Rc<RefCell<Option<AudioClip>>> = Rc::new(RefCell::new(None));

        let handle_loop = {
            let state = Rc::clone(&state);
            let render_state = render_state.clone();
            let mixer = mixer.clone(); 
            move |_: &_| {
                {
                    let mut state_obj = state.borrow_mut();
                    state_obj.bg_loop = !state_obj.bg_loop;
                    match state_obj.bg_loop {
                        true => {
                            info!("should be playing loop...");

                            if let Some(handle) = bg_handle.as_ref() {
                                if state_obj.pause_is_clip {
                                    handle.play();
                                } else {
                                    mixer.resume();
                                }
                            } else if bg_handle.is_none() {
                                let handle = mixer.add_source( 
                                    AudioSource::Url(bg_loop_url.clone()),
                                    AudioClipOptions {
                                        auto_play: true,
                                        is_loop: true,
                                        on_ended: Some({
                                            let state = Rc::clone(&state);
                                            let render_state = render_state.clone();
                                            move || {
                                                info!("loop ended!");
                                                //this won't ever actually happen
                                                let mut state = state.borrow_mut();
                                                state.bg_loop = false;
                                                render_state(&state);
                                            }
                                        })
                                    }
                                )
                                .unwrap_throw();


                                bg_handle = Some(handle);

                            }
                        }
                        false => {
                            if state_obj.stop_is_pause {
                                if state_obj.pause_is_clip {
                                    if let Some(handle) = bg_handle.as_ref() {
                                        handle.pause();
                                    }
                                } else {
                                    mixer.suspend();
                                }
                            } else {
                                bg_handle.take();
                            }
                        }
                    }
                }
                render_state(&state.borrow());
            }
        };
        let handle_regular = {
            let state = Rc::clone(&state);
            let render_state = render_state.clone();
            let mixer = mixer.clone(); 
            move |_: &_| {
                {
                    let mut state_obj = state.borrow_mut();
                    state_obj.regular = !state_obj.regular ;
                    match state_obj.regular {
                        true => {
                            info!("should be playing regular...");

                            if let Some(handle) = regular_handle.as_ref() {
                                if state_obj.pause_is_clip {
                                    handle.play();
                                } else {
                                    mixer.resume();
                                }
                            } else if regular_handle.is_none() {
                                let handle = mixer.add_source( 
                                    AudioSource::Url(regular_url.clone()),
                                    AudioClipOptions {
                                        auto_play: true,
                                        is_loop: false,
                                        on_ended: Some({
                                            let state = Rc::clone(&state);
                                            let render_state = render_state.clone();
                                            move || {
                                                info!("regular ended! - explicitly stop/drop!");
                                            }
                                        })
                                    }
                                )
                                .unwrap_throw();


                                regular_handle = Some(handle);
                            }
                        }
                        false => {
                            if state_obj.stop_is_pause {
                                if state_obj.pause_is_clip {
                                    if let Some(handle) = regular_handle.as_ref() {
                                        handle.pause();
                                    }
                                } else {
                                    mixer.suspend();
                                }
                            } else {
                                regular_handle.take();
                            }
                        }
                    }
                }
                render_state(&state.borrow());
            }
        };

        let handle_oneshot = {
            let state = Rc::clone(&state);
            let render_state = render_state.clone();
            let mixer = mixer.clone();
            let oneshot_clip = oneshot_clip.clone();
            move |_: &_| {
                {
                    let mut state_obj = state.borrow_mut();
                    state_obj.oneshot = !state_obj.oneshot;
                    match state_obj.oneshot {
                        true => {
                            info!("should be playing oneshot...");
                            let clip = mixer.play_oneshot(
                                AudioSource::Url(one_shot_url.clone()),
                                Some({
                                    let state = state.clone();
                                    let render_state = render_state.clone();
                                        move || {
                                            info!("oneshot ended!");
                                            let mut state = state.borrow_mut();
                                            state.oneshot = false;
                                            render_state(&state);
                                        }
                                })
                            )
                            .unwrap_throw();

                            *oneshot_clip.borrow_mut() = Some(clip);
                        }
                        false => {
                            if let Some(clip) = oneshot_clip.borrow_mut().take() {
                                //Need to do this
                                //in most real-world scenarios, if we wanted to hold a handle
                                //we'd do that... oneshot is really just for kicking off
                                clip.force_kill_oneshot(); 
                            }
                        }
                    }
                }
                render_state(&state.borrow());
            }
        };

        let handle_stop_is_pause = {
            let state = Rc::clone(&state);
            let render_state = render_state.clone();
            move |_: &_| {
                {
                    let mut state_obj = state.borrow_mut();
                    state_obj.stop_is_pause = !state_obj.stop_is_pause;
                }
                render_state(&state.borrow());
            }
        };
        let handle_pause_is_clip = {
            let state = Rc::clone(&state);
            let render_state = render_state.clone();
            move |_: &_| {
                {
                    let mut state_obj = state.borrow_mut();
                    state_obj.pause_is_clip = !state_obj.pause_is_clip;
                }
                render_state(&state.borrow());
            }
        };

        EventListener::new(&play_loop, "click", handle_loop).forget();
        EventListener::new(&play_regular, "click", handle_regular).forget();
        EventListener::new(&play_oneshot, "click", handle_oneshot).forget();
        EventListener::new(&stop_is_pause, "click", handle_stop_is_pause).forget();
        EventListener::new(&pause_is_clip, "click", handle_pause_is_clip).forget();

        Ok(JsValue::null())
    };

    //we don't handle errors here because they are exceptions
    //hope you're running in an environment where uncaught rejects/exceptions are reported!
    let _ = future_to_promise(future);

    Ok(())
}

fn create_button(document: &Document, root: &Element, label: &str) -> Result<HtmlElement, JsValue> {
    let item: HtmlElement = document.create_element("div")?.dyn_into()?;
    item.set_class_name("button audio-player-button");
    item.set_text_content(Some(label));
    root.append_child(&item)?;
    Ok(item)
}

