use wasm_bindgen::prelude::*;
use web_sys::{Blob, BlobPropertyBag, Url, Worker, WorkerOptions};

use js_sys::Array;

pub fn new_worker_from_js(js:&str, options: Option<WorkerOptions>) -> Result<Worker, JsValue> {

    let mut blob_options = BlobPropertyBag::new();
    blob_options.type_("application/javascript");


    let blob_parts = Array::new_with_length(1);
    blob_parts.set(0, JsValue::from_str(js));

    let blob = Blob::new_with_str_sequence_and_options(&blob_parts, &blob_options)?;

    let blob_url = Url::create_object_url_with_blob(&blob)?;

    let worker = match options {
        Some(options) => {
            Worker::new_with_options(&blob_url, &options)?
        },

        None => {
            Worker::new(&blob_url)?
        }
    };

    Url::revoke_object_url(&blob_url)?;

    Ok(worker)
}