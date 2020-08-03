use crate::errors::Error;
//Don't know why awsm_web needs FutureExt but awsm_renderer doesn't...
use js_sys::ArrayBuffer;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{ Request, AbortController, AbortSignal,RequestInit, };

#[cfg(feature = "serde")]
use serde::{Serialize, de::DeserializeOwned};

/** core fetch primitives. Thanks Pauan! 
 **
 ** these fetches will automatically abort when dropped :D
 */

struct Abort {
    controller: AbortController,
}

impl Abort {
    fn new() -> Result<Self, JsValue> {
        Ok(Self {
            controller: AbortController::new()?,
        })
    }

    fn signal(&self) -> AbortSignal {
        self.controller.signal()
    }
}

impl Drop for Abort {
    fn drop(&mut self) {
        self.controller.abort();
    }
}

pub struct Response {
    response: web_sys::Response,
    _abort: Abort,
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
    pub async fn json<T: DeserializeOwned>(self) -> Result<T, Error> { 
        let data = self.json_raw().await?;

        serde_wasm_bindgen::from_value(data).map_err(|err| Error::from(JsValue::from(err)))
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

/// Warning: Will overwrite the init's signal!
/// Generally, this is for internal use and it's recommended to use the other helper functions
/// It's made pub to avoid needing a helper function to cover *every* scenario
pub async fn fetch_req(req: &Request, init:&mut RequestInit) -> Result<Response, Error> {
    let abort = Abort::new().map_err(|err| Error::from(err))?;

    init.signal(Some(&abort.signal()));

    let future = web_sys::window()
        .unwrap_throw()
        .fetch_with_request_and_init(req, init);

    let response = JsFuture::from(future)
        .await?
        .unchecked_into::<web_sys::Response>();

    if !response.ok() {
        Err(js_sys::Error::new("Fetch failed with bad HTTP code").into())
    } else {
        Ok(Response { response, _abort: abort })
    }
}

pub async fn fetch_url(url:&str) -> Result<Response, Error> {
    fetch_req(&Request::new_with_str(url)?, &mut RequestInit::new()).await
}

pub async fn fetch_with_headers<A: AsRef<str>, B: AsRef<str>>(url: &str, method:&str, include_credentials: bool, pairs: &[(A, B)]) -> Result<Response, Error> {
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

    fetch_req(&req, &mut req_init).await
}

#[cfg(feature = "serde_json")]
pub async fn fetch_with_data(url: &str, method:&str, include_credentials: bool, data:Option<impl Serialize>) -> Result<Response, Error> {
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

    fetch_req(&req, &mut req_init).await
}


#[cfg(feature = "serde_json")]
pub async fn fetch_with_headers_and_data<A: AsRef<str>, B: AsRef<str>>(url: &str, method:&str, include_credentials: bool, pairs: &[(A, B)], data:Option<impl Serialize>) -> Result<Response, Error> {
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

    fetch_req(&req, &mut req_init).await
}

