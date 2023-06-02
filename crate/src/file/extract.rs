use js_sys::ArrayBuffer;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlInputElement, File, FileList};

use crate::data::ArrayBufferExt;

pub async fn file_to_bytes(file: &File) -> Result<Vec<u8>, JsValue> {
    let file_contents:JsValue = JsFuture::from(file.array_buffer())
        .await?;

    let file_contents:ArrayBuffer = file_contents.unchecked_into();

    Ok(file_contents.to_vec_u8())
}


pub async fn file_list_to_bytes(file_list: &FileList) -> Vec<Result<Vec<u8>, JsValue>> {
    let mut ret = Vec::new();
    for i in 0..file_list.length() {
        if let Some(file) = file_list.item(i) {
            ret.push(file_to_bytes(&file).await);
        }
    }
    ret
}

pub async fn input_file_to_bytes(input: &HtmlInputElement) -> Option<Result<Vec<u8>, JsValue>> {
    let file = input.files().and_then(|files| files.get(0));

    match file {
        None => {
            None
        },
        Some(file) => {
            Some(file_to_bytes(&file).await)
        },
    }
}

pub async fn input_file_list_to_bytes(input: &HtmlInputElement) -> Vec<Result<Vec<u8>, JsValue>> {
    match input.files() {
        None => {
            Vec::new()
        },
        Some(file_list) => {
            file_list_to_bytes(&file_list).await
        }
    }
}