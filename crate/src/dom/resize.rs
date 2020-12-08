use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;

/// Temporary placeholder until https://github.com/rustwasm/wasm-bindgen/issues/2289
/// We don't care about the callback params, so it's left out
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = ResizeObserver)]
    pub type JsResizeObserver;

    #[wasm_bindgen(constructor, js_class = "ResizeObserver")]
    pub fn new(callback: &Closure<dyn FnMut()>) -> JsResizeObserver;

    #[wasm_bindgen(method, js_class = "ResizeObserver")]
    pub fn disconnect(this: &JsResizeObserver);

    #[wasm_bindgen(method, js_class = "ResizeObserver")]
    pub fn observe(this: &JsResizeObserver, elem:&Element);

    #[wasm_bindgen(method, js_class = "ResizeObserver")]
    pub fn unobserve(this: &JsResizeObserver, elem:&Element);
}

/// Very simple for now... a more Rusty API might emit a stream
/// of specific changes, or at least pass them to the callback
/// this has no `disconnect()` since it will call that on Drop
pub struct ResizeObserver {
    observer: JsResizeObserver,
    closure: Closure<dyn FnMut()>,
}

impl ResizeObserver {
    pub fn new<F: FnMut() + 'static>(callback: F) -> Self {
        let closure = Closure::wrap(Box::new(callback) as Box<dyn FnMut()>);
        //needs nightly: let closure = Closure::new(callback);
        Self {
            observer: JsResizeObserver::new(&closure),
            closure
        }
    }

    pub fn observe(&self, elem:&Element) {
        self.observer.observe(elem);
    }
    pub fn unobserve(&self, elem:&Element) {
        self.observer.unobserve(elem);
    }

}

impl Drop for ResizeObserver {
    fn drop(&mut self) {
        self.observer.disconnect();
    }
}
