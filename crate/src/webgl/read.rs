use super::{Id, WebGlCommon, WebGlRenderer, TypedData, ReadPixelsFormat, ReadPixelsDataType};
use crate::errors::{Error, NativeError};
use std::marker::PhantomData;
use web_sys::WebGlBuffer;
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext, WebGlFramebuffer, WebGlTexture, WebGlRenderbuffer};
use std::convert::TryInto;

pub trait PartialWebGlReadPixels {
    //TODO
    //next up - create the enums!
    //btw might need to make different versions for the supported TypedData: [u8], [u16], [f32]
    //See: https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/readPixels
    //And: https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGl2RenderingContext.html#method.read_pixels_with_opt_u8_array

    fn awsm_read_pixels(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelsFormat, data_type: ReadPixelsDataType, data: Option<TypedData>, offset: Option<usize>);
}

macro_rules! impl_context {
    ($($type:ty { $($defs:tt)* })+) => {
        $(impl PartialWebGlReadPixels for $type {
            fn awsm_read_pixels(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelsFormat, data_type: ReadPixelsDataType, data: Option<TypedData>, offset: Option<usize>) {
            }

            $($defs)*
        })+
    };
}

impl_context! {
    WebGlRenderingContext{
    }
    WebGl2RenderingContext{

    }
}


impl<T: WebGlCommon> WebGlRenderer<T> {

    //TODO
    pub fn read_pixels(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelsFormat, data_type: ReadPixelsDataType, data: Option<TypedData>, offset: Option<usize>) {
        self.gl.awsm_read_pixels(&self, x, y, width, height, format, data_type, data, offset);
    }

}