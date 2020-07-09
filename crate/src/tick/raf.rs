use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Raf {
    state: Rc<RefCell<Option<RafState>>>,
}

struct RafState {
    id: i32,
    closure: Closure<dyn FnMut(f64)>,
}

impl Raf {
    pub fn new<F>(mut callback: F) -> Self where F: FnMut(f64) + 'static {
        let state: Rc<RefCell<Option<RafState>>> = Rc::new(RefCell::new(None));

        fn schedule(callback: &Closure<dyn FnMut(f64)>) -> i32 {
            web_sys::window()
                .unwrap_throw()
                .request_animation_frame(callback.as_ref().unchecked_ref())
                .unwrap_throw()
        }

        let closure = {
            let state = state.clone();

            Closure::wrap(Box::new(move |time| {
                {
                    let mut state = state.borrow_mut();
                    let state = state.as_mut().unwrap_throw();
                    state.id = schedule(&state.closure);
                }

                callback(time);
            }) as Box<dyn FnMut(f64)>)
        };

        *state.borrow_mut() = Some(RafState {
            id: schedule(&closure),
            closure
        });

        Self { state }
    }
}

impl Drop for Raf {
    fn drop(&mut self) {
        // The take is necessary in order to prevent an Rc leak
        let state = self.state.borrow_mut().take().unwrap_throw();

        web_sys::window()
            .unwrap_throw()
            .cancel_animation_frame(state.id)
            .unwrap_throw();
    }
}