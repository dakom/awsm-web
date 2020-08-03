use crate::data::TypedData;
use crate::data::*;
use crate::errors::{Error, NativeError};
use crate::window::get_window;
//Don't know why awsm_web needs FutureExt but awsm_renderer doesn't...
use futures::future::{self, TryFutureExt, FutureExt};
use std::future::Future;
use js_sys::{Array, ArrayBuffer, Promise};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    Blob, BlobPropertyBag, Request, Url, AbortController, AbortSignal,RequestInit,
};

use web_sys::{ AudioBuffer, AudioContext };


pub fn audio_buffer<'a>( array_buffer: &ArrayBuffer, ctx: &AudioContext,) -> impl Future<Output = Result<AudioBuffer, Error>> {
    match ctx.decode_audio_data(&array_buffer) {
        Ok(promise) => future::ok(promise),
        Err(err) => future::err(err.into()),
    }
    .and_then(|promise| JsFuture::from(promise))
    .map(|res| match res {
        Ok(x) => Ok(AudioBuffer::from(x)),
        Err(x) => Err(Error::from(x)),
    })
}

//convenince helpers for loading slices, vecs, etc.
pub fn audio_u8<T: AsRef<[u8]>>(
    data: T,
    ctx: &AudioContext,
) -> impl Future<Output = Result<AudioBuffer, Error>> {
    let array_buffer: ArrayBuffer = TypedData::new(data.as_ref()).into();
    audio_buffer(&array_buffer, &ctx)
}

pub fn audio_u16<T: AsRef<[u16]>>(
    data: T,
    ctx: &AudioContext,
) -> impl Future<Output = Result<AudioBuffer, Error>> {
    let array_buffer: ArrayBuffer = TypedData::new(data.as_ref()).into();
    audio_buffer(&array_buffer, &ctx)
}
pub fn audio_u32<T: AsRef<[u32]>>(
    data: T,
    ctx: &AudioContext,
) -> impl Future<Output = Result<AudioBuffer, Error>> {
    let array_buffer: ArrayBuffer = TypedData::new(data.as_ref()).into();
    audio_buffer(&array_buffer, &ctx)
}
pub fn audio_i8<T: AsRef<[i8]>>(
    data: T,
    ctx: &AudioContext,
) -> impl Future<Output = Result<AudioBuffer, Error>> {
    let array_buffer: ArrayBuffer = TypedData::new(data.as_ref()).into();
    audio_buffer(&array_buffer, &ctx)
}
pub fn audio_i16<T: AsRef<[i16]>>(
    data: T,
    ctx: &AudioContext,
) -> impl Future<Output = Result<AudioBuffer, Error>> {
    let array_buffer: ArrayBuffer = TypedData::new(data.as_ref()).into();
    audio_buffer(&array_buffer, &ctx)
}
pub fn audio_i32<T: AsRef<[i32]>>(
    data: T,
    ctx: &AudioContext,
) -> impl Future<Output = Result<AudioBuffer, Error>> {
    let array_buffer: ArrayBuffer = TypedData::new(data.as_ref()).into();
    audio_buffer(&array_buffer, &ctx)
}
pub fn audio_f32<T: AsRef<[f32]>>(
    data: T,
    ctx: &AudioContext,
) -> impl Future<Output = Result<AudioBuffer, Error>> {
    let array_buffer: ArrayBuffer = TypedData::new(data.as_ref()).into();
    audio_buffer(&array_buffer, &ctx)
}
pub fn audio_f64<T: AsRef<[f64]>>(
    data: T,
    ctx: &AudioContext,
) -> impl Future<Output = Result<AudioBuffer, Error>> {
    let array_buffer: ArrayBuffer = TypedData::new(data.as_ref()).into();
    audio_buffer(&array_buffer, &ctx)
}

