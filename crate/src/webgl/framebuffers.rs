use super::{BufferTarget, BufferUsage, Id, WebGlCommon, WebGlRenderer, FrameBufferTarget};
use crate::errors::{Error, NativeError};
use std::marker::PhantomData;
use web_sys::WebGlBuffer;
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext, WebGlFramebuffer};
/*
 * The direct uniform uploads are written as traits on this newtype wrapper
 * in order to allow working either f32 or u8
 */

//See: https://users.rust-lang.org/t/different-impls-for-types-of-slices-and-arrays
//
//

pub trait PartialWebGlFrameBuffer {
    fn awsm_bind_framebuffer(&self, target: FrameBufferTarget, buffer: &WebGlFramebuffer);
    fn awsm_delete_framebuffer(&self, buffer: &WebGlFramebuffer);
    fn awsm_is_framebuffer(&self, buffer: &WebGlFramebuffer) -> bool;
    fn awsm_create_framebuffer(&self) -> Result<WebGlFramebuffer, Error>;
}

macro_rules! impl_context {
    ($($type:ty { $($defs:tt)* })+) => {
        $(impl PartialWebGlFrameBuffer for $type {
            fn awsm_bind_framebuffer(&self, target:FrameBufferTarget, buffer:&WebGlFramebuffer) {
                self.bind_framebuffer(target as u32, Some(buffer));
            }

            fn awsm_delete_framebuffer(&self, buffer:&WebGlFramebuffer) {
                self.delete_framebuffer(Some(buffer));
            }

            fn awsm_is_framebuffer(&self, buffer:&WebGlFramebuffer) -> bool {
                self.is_framebuffer(Some(buffer))
            }

            fn awsm_create_framebuffer(&self) -> Result<WebGlFramebuffer, Error> {
                self.create_framebuffer().ok_or(Error::from(NativeError::NoCreateFrameBuffer))
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