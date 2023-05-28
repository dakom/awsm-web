pub fn save_file(data: Vec<u8>, filename: &str, mime_type: Option<&str>) {
    // cannot use .view() because the WASM memory changes under our feet
    let content = Uint8Array::new_with_length(data.len() as u32);
    content.copy_from(&data);
    
    let blob_parts = js_sys::Array::new();
    blob_parts.push(&content);

    let blob = match mime_type {
        Some(mime_type) => {
            let mut blob_options = web_sys::BlobPropertyBag::new();
            blob_options.type_(mime_type);
            web_sys::Blob::new_with_u8_array_sequence_and_options(&blob_parts, &blob_options).unwrap_ext()
        },
        None => {
            web_sys::Blob::new_with_u8_array_sequence(&blob_parts, &blob_options).unwrap_ext()
        }
    };

    let blob_url = web_sys::Url::create_object_url_with_blob(&blob).unwrap_ext();

    let document = web_sys::window().unwrap().document().unwrap();

    let elem:web_sys::HtmlAnchorElement = document.create_element("a").unwrap_ext().unchecked_into();

    elem.set_attribute("href", &blob_url).unwrap_ext();
    elem.set_attribute("download", filename).unwrap_ext();

    let body = document.body().unwrap_ext();
    body.append_child(&elem).unwrap_ext();

    elem.click();

    body.remove_child(&elem).unwrap_ext();

    web_sys::Url::revoke_object_url(&blob_url).unwrap_ext();
}