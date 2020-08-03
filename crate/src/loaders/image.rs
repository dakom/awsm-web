use futures::channel::oneshot::{channel, Receiver, Sender};
use std::task::{Context, Poll};
use std::future::Future;
use std::pin::Pin;
use crate::window::get_window;
use crate::data::TypedData;
use crate::data::*;
use crate::errors::{Error, NativeError};
//Don't know why awsm_web needs FutureExt but awsm_renderer doesn't...
use futures::future::{self, TryFutureExt, FutureExt};
use js_sys::{Array, ArrayBuffer, Promise};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    Blob, BlobPropertyBag, Request, Url, AbortController, AbortSignal,RequestInit,HtmlImageElement
};

pub struct Image {
    pub url: String,
    pub img: Option<HtmlImageElement>,
    state: ImageState,
    closure_holders: Option<(Closure<dyn FnMut()>, Closure<dyn FnMut(JsValue)>)>,
}

enum ImageState {
    Empty,
    Loading {
        receiver_err: Receiver<JsValue>,
        receiver_success: Receiver<()>,
    },
}

//See: https://github.com/rustwasm/wasm-bindgen/issues/1126
//
impl Future for Image {
    //impl Future for Image {
    type Output = Result<HtmlImageElement, Error>;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        //fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match &mut self.state {
            ImageState::Empty => {
                let img = HtmlImageElement::new()?;
                let has_same_origin = same_origin(&self.url)?;
                if !has_same_origin {
                    img.set_cross_origin(Some(&"anonymous"));
                }

                img.set_src(&self.url);

                //success callback
                let waker = ctx.waker().clone();
                let (sender_success, receiver_success): (Sender<()>, Receiver<()>) = channel();
                let mut sender_success = Option::from(sender_success);
                let closure_success = Closure::wrap(Box::new(move || {
                    sender_success.take().unwrap_throw().send(()).unwrap_throw();
                    waker.wake_by_ref();
                }) as Box<dyn FnMut()>);

                img.set_onload(Some(closure_success.as_ref().unchecked_ref()));

                //error callback
                let waker = ctx.waker().clone();
                let (sender_err, receiver_err): (Sender<JsValue>, Receiver<JsValue>) = channel();
                let mut sender_err = Option::from(sender_err);
                let closure_err = Closure::wrap(Box::new(move |err| {
                    sender_err.take().unwrap_throw().send(err).unwrap_throw();
                    waker.wake_by_ref();
                }) as Box<dyn FnMut(JsValue)>);

                //self.closure_err = Some(closure_err);
                img.set_onerror(Some(closure_err.as_ref().unchecked_ref()));

                //Assign stuff to myself
                self.img = Some(img);
                self.state = ImageState::Loading {
                    receiver_err,
                    receiver_success,
                };
                self.closure_holders = Some((closure_success, closure_err));

                //notify the task that we're now loading
                ctx.waker().wake_by_ref();

                Poll::Pending
            }

            ImageState::Loading {
                receiver_err,
                receiver_success,
            } => {
                //if let Poll::Ready(value) = Receiver::poll(Pin::new(receiver_err), ctx) {

                let mut is_cancelled = false;

                let error_state = match receiver_err.try_recv() {
                    Ok(result) => result,
                    _ => {
                        is_cancelled = true;
                        None
                    }
                };

                let success_state = match receiver_success.try_recv() {
                    Ok(result) => result,
                    _ => {
                        is_cancelled = true;
                        None
                    }
                };

                if let Some(result) = error_state {
                    Poll::Ready(Err(result.into()))
                } else if let Some(_) = success_state {
                    Poll::Ready(Ok(self.img.as_ref().unwrap_throw().clone()))
                } else {
                    if !is_cancelled {
                        //ctx.waker().wake_by_ref();
                    }
                    Poll::Pending
                }
            }
        }
    }
}

impl Image {
    pub fn new(url: String) -> Self {
        Self {
            url,
            img: None,
            state: ImageState::Empty,
            closure_holders: None,
        }
    }
}

pub fn same_origin(url: &str) -> Result<bool, JsValue> {
    //FOLLOWUP: https://github.com/rustwasm/wasm-bindgen/issues/1150
    if url.starts_with("http://") || url.starts_with("https://") {
        let location_origin = get_window()?.location().origin()?;
        let url_origin = Url::new(url)?.origin();
        Ok(url_origin == location_origin)
    } else {
        Ok(true)
    }
}


pub fn load(url: String) -> impl Future<Output = Result<HtmlImageElement, Error>> {
    Image::new(url)
}

pub fn load_blob(blob: &Blob) -> impl Future<Output = Result<HtmlImageElement, Error>> {
    match Url::create_object_url_with_blob(&blob) {
        Ok(url) => future::ok(url),
        Err(err) => future::err(err.into()),
    }
    .and_then(|url| load(url))
}
pub fn load_js_value(
    data: &JsValue,
    mime_type: &str,
) -> impl Future<Output = Result<HtmlImageElement, Error>> {
    let mut blob_opts = BlobPropertyBag::new();
    blob_opts.type_(mime_type);

    match Blob::new_with_buffer_source_sequence_and_options(
        &Array::of1(data).into(),
        &blob_opts,
    ) {
        Ok(blob) => future::ok(blob),
        Err(err) => future::err(err.into()),
    }
    .and_then(|blob| load_blob(&blob))
}

pub fn load_u8<T: AsRef<[u8]>>(
    data: T,
    mime_type: &str,
) -> impl Future<Output = Result<HtmlImageElement, Error>> {
    let mut blob_opts = BlobPropertyBag::new();
    blob_opts.type_(mime_type);

    match Blob::new_with_buffer_source_sequence_and_options(
        &Array::of1(&TypedData::new(data.as_ref()).into()).into(),
        &blob_opts,
    ) {
        Ok(blob) => future::ok(blob),
        Err(err) => future::err(err.into()),
    }
    .and_then(|blob| load_blob(&blob))
}

pub fn load_u16<T: AsRef<[u16]>>(
    data: T,
    mime_type: &str,
) -> impl Future<Output = Result<HtmlImageElement, Error>> {
    let mut blob_opts = BlobPropertyBag::new();
    blob_opts.type_(mime_type);

    match Blob::new_with_buffer_source_sequence_and_options(
        &Array::of1(&TypedData::new(data.as_ref()).into()).into(),
        &blob_opts,
    ) {
        Ok(blob) => future::ok(blob),
        Err(err) => future::err(err.into()),
    }
    .and_then(|blob| load_blob(&blob))
}

pub fn load_u32<T: AsRef<[u32]>>(
    data: T,
    mime_type: &str,
) -> impl Future<Output = Result<HtmlImageElement, Error>> {
    let mut blob_opts = BlobPropertyBag::new();
    blob_opts.type_(mime_type);

    match Blob::new_with_buffer_source_sequence_and_options(
        &Array::of1(&TypedData::new(data.as_ref()).into()).into(),
        &blob_opts,
    ) {
        Ok(blob) => future::ok(blob),
        Err(err) => future::err(err.into()),
    }
    .and_then(|blob| load_blob(&blob))
}

pub fn load_i8<T: AsRef<[i8]>>(
    data: T,
    mime_type: &str,
) -> impl Future<Output = Result<HtmlImageElement, Error>> {
    let mut blob_opts = BlobPropertyBag::new();
    blob_opts.type_(mime_type);

    match Blob::new_with_buffer_source_sequence_and_options(
        &Array::of1(&TypedData::new(data.as_ref()).into()).into(),
        &blob_opts,
    ) {
        Ok(blob) => future::ok(blob),
        Err(err) => future::err(err.into()),
    }
    .and_then(|blob| load_blob(&blob))
}

pub fn load_i16<T: AsRef<[i16]>>(
    data: T,
    mime_type: &str,
) -> impl Future<Output = Result<HtmlImageElement, Error>> {
    let mut blob_opts = BlobPropertyBag::new();
    blob_opts.type_(mime_type);

    match Blob::new_with_buffer_source_sequence_and_options(
        &Array::of1(&TypedData::new(data.as_ref()).into()).into(),
        &blob_opts,
    ) {
        Ok(blob) => future::ok(blob),
        Err(err) => future::err(err.into()),
    }
    .and_then(|blob| load_blob(&blob))
}

pub fn load_i32<T: AsRef<[i32]>>(
    data: T,
    mime_type: &str,
) -> impl Future<Output = Result<HtmlImageElement, Error>> {
    let mut blob_opts = BlobPropertyBag::new();
    blob_opts.type_(mime_type);

    match Blob::new_with_buffer_source_sequence_and_options(
        &Array::of1(&TypedData::new(data.as_ref()).into()).into(),
        &blob_opts,
    ) {
        Ok(blob) => future::ok(blob),
        Err(err) => future::err(err.into()),
    }
    .and_then(|blob| load_blob(&blob))
}

pub fn load_f32<T: AsRef<[f32]>>(
    data: T,
    mime_type: &str,
) -> impl Future<Output = Result<HtmlImageElement, Error>> {
    let mut blob_opts = BlobPropertyBag::new();
    blob_opts.type_(mime_type);

    match Blob::new_with_buffer_source_sequence_and_options(
        &Array::of1(&TypedData::new(data.as_ref()).into()).into(),
        &blob_opts,
    ) {
        Ok(blob) => future::ok(blob),
        Err(err) => future::err(err.into()),
    }
    .and_then(|blob| load_blob(&blob))
}

pub fn load_f64<T: AsRef<[f64]>>(
    data: T,
    mime_type: &str,
) -> impl Future<Output = Result<HtmlImageElement, Error>> {
    let mut blob_opts = BlobPropertyBag::new();
    blob_opts.type_(mime_type);

    match Blob::new_with_buffer_source_sequence_and_options(
        &Array::of1(&TypedData::new(data.as_ref()).into()).into(),
        &blob_opts,
    ) {
        Ok(blob) => future::ok(blob),
        Err(err) => future::err(err.into()),
    }
    .and_then(|blob| load_blob(&blob))
}
