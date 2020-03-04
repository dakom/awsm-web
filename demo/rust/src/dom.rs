use std::rc::{Rc};
use std::cell::{RefCell};
use web_sys::{HtmlElement, HtmlCanvasElement};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use awsm_web::window::{get_window_size};
use awsm_web::loaders::fetch;
use awsm_web::webgl::{
    get_webgl_context_1, 
    WebGlContextOptions, 
    ClearBufferMask,
    WebGl1Renderer
};
use gloo_events::EventListener;
use std::future::Future;

pub fn init<F>(loader:F) -> Result<js_sys::Promise, JsValue> 
    where F: Future + 'static
{
    
    let window = web_sys::window().ok_or("should have a Window")?;
    let document = window.document().ok_or("should have a Document")?;
    let body = document.body().ok_or("should have a Body")?;

    //use regular html
    let loading: HtmlElement = document.create_element("div")?.dyn_into()?;
    loading.set_class_name("loading"); // from _static/index.css
    loading.set_text_content(Some("Loading..."));
    body.append_child(&loading)?;

    let future = async move {
        loader.await;
        body.remove_child(&loading);
       
        //use web component
        let main: HtmlElement = document.create_element("app-main")?.dyn_into()?;
        body.append_child(&main)?;

        Ok(JsValue::null())
    };

    Ok(future_to_promise(future))
}