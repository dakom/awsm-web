use wasm_bindgen::prelude::*;
use web_sys::{Element, DomRect};
use js_sys::Array;
use std::convert::TryInto;

/// Temporary placeholder until https://github.com/rustwasm/wasm-bindgen/issues/2289
/// We don't care about the callback params, so it's left out
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = ResizeObserver)]
    pub type JsResizeObserver;

    #[wasm_bindgen(constructor, js_class = "ResizeObserver")]
    pub fn new(callback: &Closure<dyn FnMut(Array)>, options: JsResizeObserverOptions) -> JsResizeObserver;

    #[wasm_bindgen(method, js_class = "ResizeObserver")]
    pub fn disconnect(this: &JsResizeObserver);

    #[wasm_bindgen(method, js_class = "ResizeObserver")]
    pub fn observe(this: &JsResizeObserver, elem:&Element);

    #[wasm_bindgen(method, js_class = "ResizeObserver")]
    pub fn unobserve(this: &JsResizeObserver, elem:&Element);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Object)]
    pub type JsResizeObserverOptions;

    #[wasm_bindgen(constructor, js_class = "Object")]
    pub fn new() -> JsResizeObserverOptions;

    #[wasm_bindgen(method, setter, js_class = "Object")]
    pub fn set_box(this: &JsResizeObserverOptions, val: JsValue);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = ResizeObserverEntry)]
    pub type JsResizeObserverEntry;

    #[wasm_bindgen(method, getter, js_class = "ResizeObserverEntry", js_name = "borderBoxSize")]
    pub fn border_box_sizes(this: &JsResizeObserverEntry) -> Array;

    #[wasm_bindgen(method, getter, js_class = "ResizeObserverEntry", js_name = "contentBoxSize")]
    pub fn content_box_sizes(this: &JsResizeObserverEntry) -> Array;

    #[wasm_bindgen(method, getter, js_class = "ResizeObserverEntry", js_name = "contentRect")]
    pub fn content_rect(this: &JsResizeObserverEntry) -> DomRect;

    #[wasm_bindgen(method, getter, js_class = "ResizeObserverEntry", js_name = "target")]
    pub fn target(this: &JsResizeObserverEntry) -> Element;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = BorderBoxSize)]
    pub type JsBorderBoxSize;

    #[wasm_bindgen(method, getter, js_class = "BorderBoxSize", js_name = "blockSize")]
    pub fn block_size(this: &JsBorderBoxSize) -> u32;

    #[wasm_bindgen(method, getter, js_class = "BorderBoxSize", js_name = "inlineSize")]
    pub fn inline_size(this: &JsBorderBoxSize) -> u32;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = ContentBoxSize)]
    pub type JsContentBoxSize;

    #[wasm_bindgen(method, getter, js_class = "ContentBoxSize", js_name = "blockSize")]
    pub fn block_size(this: &JsContentBoxSize) -> u32;

    #[wasm_bindgen(method, getter, js_class = "ContentBoxSize", js_name = "inlineSize")]
    pub fn inline_size(this: &JsContentBoxSize) -> u32;
}
/// this has no `disconnect()` since it will call that on Drop
pub struct ResizeObserver {
    observer: JsResizeObserver,
    #[allow(dead_code)]
    closure: Closure<dyn FnMut(Array)>,
}

pub struct ResizeObserverOptions {
    pub box_kind: ResizeObserverBoxKind
}

pub enum ResizeObserverBoxKind {
    Content,
    Border
}

impl ResizeObserverBoxKind {
    pub fn to_js_value(&self) -> JsValue {
        match self {
            Self::Content => JsValue::from_str("content-box"),
            Self::Border => JsValue::from_str("border-box"),
        }
    }
}

#[derive(Debug)]
pub struct ResizeObserverEntry {
    pub border_box_sizes: Vec<BorderBoxSize>,
    pub content_box_sizes: Vec<ContentBoxSize>,
    pub content_rect: DomRect,
    pub target: Element,
}

impl From<JsResizeObserverEntry> for ResizeObserverEntry {
    fn from(entry:JsResizeObserverEntry) -> Self {
        let border_box_sizes = {
            let entries = entry.border_box_sizes();
            let len = entries.length() as usize;
            let mut values:Vec<BorderBoxSize> = Vec::with_capacity(len);
            for i in 0..len {
                let value = JsBorderBoxSize::from(entries.get(i.try_into().unwrap()));
                values.push(value.into());
            }

            values
        };

        let content_box_sizes = {
            let entries = entry.content_box_sizes();
            let len = entries.length() as usize;
            let mut values:Vec<ContentBoxSize> = Vec::with_capacity(len);
            for i in 0..len {
                let value = JsContentBoxSize::from(entries.get(i.try_into().unwrap()));
                values.push(value.into());
            }

            values
        };

        Self {
            border_box_sizes,
            content_box_sizes,
            content_rect: entry.content_rect(),
            target: entry.target()
        }
    }
}
#[derive(Debug)]
pub struct BorderBoxSize {
    pub block_size: u32,
    pub inline_size: u32,
}
impl From<JsBorderBoxSize> for BorderBoxSize {
    fn from(entry:JsBorderBoxSize) -> Self {
        Self {
            block_size: entry.block_size(),
            inline_size: entry.inline_size(),
        }
    }
}
#[derive(Debug)]
pub struct ContentBoxSize {
    pub block_size: u32,
    pub inline_size: u32,
}
impl From<JsContentBoxSize> for ContentBoxSize {
    fn from(entry:JsContentBoxSize) -> Self {
        Self {
            block_size: entry.block_size(),
            inline_size: entry.inline_size(),
        }
    }
}

impl ResizeObserver {
    pub fn new<F: FnMut(Vec<ResizeObserverEntry>) + 'static>(mut callback: F, options: Option<ResizeObserverOptions>) -> Self {
        let closure = Closure::wrap(Box::new(move |entries:Array| {
            let len = entries.length() as usize;
            let mut values:Vec<ResizeObserverEntry> = Vec::with_capacity(len);

            for i in 0..len {
                let value = JsResizeObserverEntry::from(entries.get(i.try_into().unwrap()));
                values.push(value.into());
            }

            callback(values);

        }) as Box<dyn FnMut(Array)>);

        let js_options:JsResizeObserverOptions = JsResizeObserverOptions::new();
        if let Some(options) = options {
            js_options.set_box(options.box_kind.to_js_value());
        }
        Self {
            observer: JsResizeObserver::new(&closure, js_options),
            closure
        }
    }
    //Doesn't marshall or send the entries back
    pub fn new_simple<F: FnMut() + 'static>(mut callback: F) -> Self {
        let closure = Closure::wrap(Box::new(move |_entries:Array| {
            callback();
        }) as Box<dyn FnMut(Array)>);

        let js_options:JsResizeObserverOptions = JsResizeObserverOptions::new();
        //needs nightly: let closure = Closure::new(callback);
        Self {
            observer: JsResizeObserver::new(&closure, js_options),
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
