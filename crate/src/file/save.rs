use js_sys::Uint8Array;
use wasm_bindgen::{prelude::*, JsCast};

pub fn save_file(data: Vec<u8>, filename: &str, mime_type: Option<&str>) -> Result<(), JsValue> {
    // cannot use .view() because the WASM memory changes under our feet
    let content = Uint8Array::new_with_length(data.len() as u32);
    content.copy_from(&data);
    
    let blob_parts = js_sys::Array::new();
    blob_parts.push(&content);

    let blob = match mime_type {
        Some(mime_type) => {
            let mut blob_options = web_sys::BlobPropertyBag::new();
            blob_options.type_(mime_type);
            web_sys::Blob::new_with_u8_array_sequence_and_options(&blob_parts, &blob_options)?
        },
        None => {
            web_sys::Blob::new_with_u8_array_sequence(&blob_parts)?
        }
    };

    let blob_url = web_sys::Url::create_object_url_with_blob(&blob)?;

    let document = web_sys::window().unwrap().document().unwrap();

    let elem:web_sys::HtmlAnchorElement = document.create_element("a")?.unchecked_into();

    elem.set_attribute("href", &blob_url)?;
    elem.set_attribute("download", filename)?;

    let body = document.body().ok_or_else(|| JsValue::from_str("unable to get document body"))?;
    body.append_child(&elem)?;

    elem.click();

    body.remove_child(&elem)?;

    web_sys::Url::revoke_object_url(&blob_url)?;

    Ok(())
}