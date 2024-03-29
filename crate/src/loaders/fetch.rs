//! These fetches are idiomatic Rusty wrappers around the web_sys fetch primitives
//! They abort when dropped (just like Rust futures should!)
//! To abort imperatively (not via dropping the future), use the _abortable variants and pass in an AbortController

use crate::errors::{Error, NativeError};
use std::ops::Deref;
//Don't know why awsm_web needs FutureExt but awsm_renderer doesn't...
use js_sys::ArrayBuffer;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{ Request, AbortSignal,RequestInit, File, Blob};

#[cfg(feature = "serde")]
use serde::{Serialize, de::DeserializeOwned};
use super::helpers::AbortController;

/** Thanks for the help Pauan! 
 */

pub struct Response {
    response: web_sys::Response,
    //If the abort controller is created here
    //needs to last as long as the response
    _abort: Option<AbortController>,
}

impl Deref for Response {
    type Target = web_sys::Response;

    fn deref(&self) -> &Self::Target {
        &self.response
    }
}

impl Response {
    pub async fn text(self) -> Result<String, Error> {
        JsFuture::from(
            self.response
                .text()
                .map_err(|err| Error::from(err))?
        ).await
            .map_err(|err| Error::from(err))
            .map(|value| value.as_string().unwrap_throw())
    }

    pub async fn json_raw(self) -> Result<JsValue, Error> {
        JsFuture::from(
            self.response
                .json()
                .map_err(|err| Error::from(err))?
        ).await.map_err(|err| Error::from(err))
    }

    pub async fn array_buffer(self) -> Result<ArrayBuffer, Error> { 
        JsFuture::from(
            self.response
                .array_buffer()
                .map_err(|err| Error::from(err))?
        ).await
            .map_err(|err| Error::from(err))
            .map(|value| value.into()) 
    }

    #[cfg(feature = "serde")]
    pub async fn json_from_obj<T: DeserializeOwned>(self) -> Result<T, Error> { 
        let data = self.json_raw().await?;

        serde_wasm_bindgen::from_value(data).map_err(|err| Error::from(JsValue::from(err)))
    }

    #[cfg(feature = "serde")]
    pub async fn json_from_str<T: DeserializeOwned>(self) -> Result<T, Error> { 
        let data = self.text().await?;

        serde_json::from_str(&data).map_err(|err| Error::from(err))
    }

    #[cfg(feature = "audio")]
    pub async fn audio(self, ctx: &web_sys::AudioContext) -> Result<web_sys::AudioBuffer, Error> {
        let buffer = self.array_buffer().await?;

        super::audio::audio_buffer(&buffer, &ctx).await
    }

    #[cfg(feature = "image")]
    pub async fn image(self, mime_type:&str) -> Result<web_sys::HtmlImageElement, Error> { 
        let buffer= self.array_buffer().await?;

        super::image::load_js_value(&buffer, mime_type).await
    }
}

/// Warning: Will overwrite the init's signal for aborting in all cases.
/// Even if abort_controller is None (in this case it creates it locally)
/// This allows aborting the fetch if the future is dropped
///
/// Generally, this is for internal use and it's recommended to use the other helper functions
/// It's made pub to avoid needing a helper function to cover *every* scenario
///
/// The Error type can be checked if it was aborted by calling .is_abort()
pub async fn fetch_req(req: &Request, abort_controller: Option<&AbortController>, init:&mut RequestInit) -> Result<Response, Error> {
    //The Response can only take ownership of a locally created abort_controller
    //But we apply it to the init arg in both cases
    let abort = match abort_controller {
        Some(a) => { 
            init.signal(Some(&a.signal()));
            None
        },
        None => {
            let a = AbortController::new();
            init.signal(Some(&a.signal()));
            Some(a)
        }
    };

    let future = web_sys::window()
        .unwrap_throw()
        .fetch_with_request_and_init(req, init);

    JsFuture::from(future).await
        .map(|response| {
            let response = response.unchecked_into::<web_sys::Response>();
            Response { response, _abort: abort }
        })
        .map_err(|err| err.into())

}

pub async fn fetch_url(url:&str) -> Result<Response, Error> {
    fetch_url_abortable(url, None).await
}
pub async fn fetch_url_abortable(url:&str, abort_controller: Option<&AbortController>) -> Result<Response, Error> {
    fetch_req(&Request::new_with_str(url)?, abort_controller, &mut RequestInit::new()).await
}

pub async fn fetch_with_headers<A: AsRef<str>, B: AsRef<str>>(url: &str, method:&str, include_credentials: bool, pairs: &[(A, B)]) -> Result<Response, Error> {
    fetch_with_headers_abortable(url, method, include_credentials, None, pairs).await
}

pub async fn fetch_with_headers_abortable<A: AsRef<str>, B: AsRef<str>>(url: &str, method:&str, include_credentials: bool, abort_controller: Option<&AbortController>, pairs: &[(A, B)]) -> Result<Response, Error> {
    let mut req_init = web_sys::RequestInit::new();
    req_init.method(method);
    if include_credentials {
        req_init.credentials(web_sys::RequestCredentials::Include);
    }
    

    let req = web_sys::Request::new_with_str_and_init(url, &req_init)?;

    let headers = req.headers();

    for (name, value) in pairs.iter() {
        headers.set(name.as_ref(), value.as_ref())?;
    }

    fetch_req(&req, abort_controller, &mut req_init).await
}

pub async fn fetch_upload_body_with_headers<A: AsRef<str>, B: AsRef<str>>(url: &str, body:&JsValue, method:&str, include_credentials: bool, pairs: &[(A, B)]) -> Result<Response, Error> {
    fetch_upload_body_with_headers_abortable(url, body, method, include_credentials, None, pairs).await
}

pub async fn fetch_upload_body_with_headers_abortable<A: AsRef<str>, B: AsRef<str>>(url: &str, body:&JsValue, method:&str, include_credentials: bool, abort_controller: Option<&AbortController>, pairs: &[(A, B)]) -> Result<Response, Error> {
    let mut req_init = web_sys::RequestInit::new();
    req_init.method(method);
    if include_credentials {
        req_init.credentials(web_sys::RequestCredentials::Include);
    }
    req_init.body(Some(body));
    

    let req = web_sys::Request::new_with_str_and_init(url, &req_init)?;

    let headers = req.headers();

    for (name, value) in pairs.iter() {
        headers.set(name.as_ref(), value.as_ref())?;
    }

    fetch_req(&req, abort_controller, &mut req_init).await
}
pub async fn fetch_upload_body(url:&str, body:&JsValue, method:&str) -> Result<Response, Error> {
    fetch_upload_body_abortable(url, body, method, None).await
}

pub async fn fetch_upload_body_abortable(url:&str, body:&JsValue, method:&str, abort_controller: Option<&AbortController>) -> Result<Response, Error> {
    let mut req_init = web_sys::RequestInit::new();
    req_init.method(method);
    req_init.body(Some(body));

    let req = web_sys::Request::new_with_str_and_init(url, &req_init)?;

    fetch_req(&req, abort_controller, &mut req_init).await

}

pub async fn fetch_upload_blob_with_headers<A: AsRef<str>, B: AsRef<str>>(url: &str, blob:&Blob, method:&str, include_credentials: bool, pairs: &[(A, B)]) -> Result<Response, Error> {
    fetch_upload_blob_with_headers_abortable(url, blob, method, include_credentials, None, pairs).await
}

pub async fn fetch_upload_blob_with_headers_abortable<A: AsRef<str>, B: AsRef<str>>(url: &str, blob:&Blob, method:&str, include_credentials: bool, abort_controller: Option<&AbortController>, pairs: &[(A, B)]) -> Result<Response, Error> {
    fetch_upload_body_with_headers_abortable(url, blob, method, include_credentials, abort_controller, pairs).await
}

pub async fn fetch_upload_blob(url:&str, blob:&Blob, method:&str) -> Result<Response, Error> {
    fetch_upload_blob_abortable(url, blob, method, None).await
}

pub async fn fetch_upload_blob_abortable(url:&str, blob:&Blob, method:&str, abort_controller: Option<&AbortController>) -> Result<Response, Error> {
    fetch_upload_body_abortable(url, blob, method, abort_controller).await
}

pub async fn fetch_upload_file_with_headers<A: AsRef<str>, B: AsRef<str>>(url: &str, file:&File, method:&str, include_credentials: bool, pairs: &[(A, B)]) -> Result<Response, Error> {
    fetch_upload_file_with_headers_abortable(url, file, method, include_credentials, None, pairs).await
}

pub async fn fetch_upload_file_with_headers_abortable<A: AsRef<str>, B: AsRef<str>>(url: &str, file:&File, method:&str, include_credentials: bool, abort_controller: Option<&AbortController>, pairs: &[(A, B)]) -> Result<Response, Error> {
    fetch_upload_body_with_headers_abortable(url, file, method, include_credentials, abort_controller, pairs).await
}

pub async fn fetch_upload_file(url:&str, file:&File, method:&str) -> Result<Response, Error> {
    fetch_upload_file_abortable(url, file, method, None).await
}

pub async fn fetch_upload_file_abortable(url:&str, file:&File, method:&str, abort_controller: Option<&AbortController>) -> Result<Response, Error> {
    fetch_upload_body_abortable(url, file, method, abort_controller).await
}

#[cfg(feature = "serde_json")]
pub async fn fetch_with_data(url: &str, method:&str, include_credentials: bool, data:Option<impl Serialize>) -> Result<Response, Error> {
    fetch_with_data_abortable(url, method, include_credentials, None, data).await
}

#[cfg(feature = "serde_json")]
pub async fn fetch_with_data_abortable(url: &str, method:&str, include_credentials: bool, abort_controller: Option<&AbortController>, data:Option<impl Serialize>) -> Result<Response, Error> {
    let mut req_init = web_sys::RequestInit::new();
    req_init.method(method);
    if include_credentials {
        req_init.credentials(web_sys::RequestCredentials::Include);
    }
    

    let req = match data {
        None => web_sys::Request::new_with_str_and_init(url, &req_init)?,

        Some(data) => {
            let json_str = serde_json::to_string(&data).map_err(|err| JsValue::from_str(&err.to_string()))?;
            //req_init.mode(web_sys::RequestMode::Cors);
            req_init.body(Some(&JsValue::from_str(&json_str)));
            let req = web_sys::Request::new_with_str_and_init(url, &req_init)?;
            req.headers().set("Content-Type", "application/json")?;

            req
        }
    };

    fetch_req(&req, abort_controller, &mut req_init).await
}


#[cfg(feature = "serde_json")]
pub async fn fetch_with_headers_and_data<A: AsRef<str>, B: AsRef<str>>(url: &str, method:&str, include_credentials: bool, pairs: &[(A, B)], data:Option<impl Serialize>) -> Result<Response, Error> {
    fetch_with_headers_and_data_abortable(url, method, include_credentials, None, pairs, data).await
}

#[cfg(feature = "serde_json")]
pub async fn fetch_with_headers_and_data_abortable<A: AsRef<str>, B: AsRef<str>>(url: &str, method:&str, include_credentials: bool, abort_controller: Option<&AbortController>, pairs: &[(A, B)], data:Option<impl Serialize>) -> Result<Response, Error> {
    let mut req_init = web_sys::RequestInit::new();
    req_init.method(method);
    if include_credentials {
        req_init.credentials(web_sys::RequestCredentials::Include);
    }
    

    let req = match data {
        None => web_sys::Request::new_with_str_and_init(url, &req_init)?,

        Some(data) => {
            let json_str = serde_json::to_string(&data).map_err(|err| JsValue::from_str(&err.to_string()))?;
            //req_init.mode(web_sys::RequestMode::Cors);
            req_init.body(Some(&JsValue::from_str(&json_str)));
            let req = web_sys::Request::new_with_str_and_init(url, &req_init)?;
            req.headers().set("Content-Type", "application/json")?;

            req
        }
    };

    let headers = req.headers();

    for (name, value) in pairs.iter() {
        headers.set(name.as_ref(), value.as_ref())?;
    }

    fetch_req(&req, abort_controller, &mut req_init).await
}
