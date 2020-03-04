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

pub async fn load() -> Result<(), JsValue> {
    //TODO - load stuff...
    Ok(())
}