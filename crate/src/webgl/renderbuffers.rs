use super::{BufferTarget, BufferUsage, Id, WebGlCommon, WebGlRenderer, RenderBufferFormat};
use crate::errors::{Error, NativeError};
use std::marker::PhantomData;
use web_sys::WebGlBuffer;
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext, WebGlRenderbuffer};

pub trait PartialWebGlRenderBuffer {
    fn awsm_bind_renderbuffer(&self, buffer: &WebGlRenderbuffer);
    fn awsm_delete_renderbuffer(&self, buffer: &WebGlRenderbuffer);
    fn awsm_is_renderbuffer(&self, buffer: &WebGlRenderbuffer) -> bool;
    fn awsm_create_renderbuffer(&self) -> Result<WebGlRenderbuffer, Error>;
    fn awsm_renderbuffer_storage(&self, format: RenderBufferFormat, width: u32, height: u32);
    fn awsm_release_renderbuffer(&self);
}
pub trait PartialWebGl2RenderBuffer {
    fn awsm_renderbuffer_storage_multisample(&self, samples: u32, format: RenderBufferFormat, width:u32, height: u32);
}
//there is only one target supported by webgl
const TARGET:u32 = 0x8D41;

macro_rules! impl_context {
    ($($type:ty { $($defs:tt)* })+) => {
        $(impl PartialWebGlRenderBuffer for $type {
            fn awsm_bind_renderbuffer(&self, buffer:&WebGlRenderbuffer) {
                self.bind_renderbuffer(TARGET, Some(buffer));
            }

            fn awsm_delete_renderbuffer(&self, buffer:&WebGlRenderbuffer) {
                self.delete_renderbuffer(Some(buffer));
            }

            fn awsm_is_renderbuffer(&self, buffer:&WebGlRenderbuffer) -> bool {
                self.is_renderbuffer(Some(buffer))
            }

            fn awsm_create_renderbuffer(&self) -> Result<WebGlRenderbuffer, Error> {
                self.create_renderbuffer().ok_or(Error::from(NativeError::NoCreateRenderBuffer))
            }
            fn awsm_renderbuffer_storage(&self, format: RenderBufferFormat, width: u32, height: u32) {
                self.renderbuffer_storage(TARGET, format as u32, width as i32, height as i32);
            }

            fn awsm_release_renderbuffer(&self) {
                self.bind_renderbuffer(TARGET, None);
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

impl PartialWebGl2RenderBuffer for WebGl2RenderingContext {
        fn awsm_renderbuffer_storage_multisample(&self, samples: u32, format: RenderBufferFormat, width:u32, height: u32) {
        self.renderbuffer_storage_multisample(TARGET, samples as i32, format as u32, width as i32, height as i32);
    } 
}


impl<T: WebGlCommon> WebGlRenderer<T> {
    pub fn create_renderbuffer(&mut self) -> Result<Id, Error> {
        let renderbuffer = self.gl.awsm_create_renderbuffer()?;
        let id = self.renderbuffer_lookup.insert(renderbuffer);

        Ok(id)
    }

    pub fn delete_renderbuffer(&self, renderbuffer_id: Id) -> Result<(), Error> {
        if Some(renderbuffer_id) == self.current_renderbuffer_id.get() {
            self.current_renderbuffer_id.set(None);
            self.gl.awsm_release_renderbuffer();
        }

        let renderbuffer = self
            .renderbuffer_lookup
            .get(renderbuffer_id)
            .ok_or(Error::from(NativeError::MissingRenderBuffer))?;

        self.gl.awsm_delete_renderbuffer(&renderbuffer);

        Ok(())
    }
    pub fn release_renderbuffer(&self) {
        self.current_renderbuffer_id.set(None);
        self.gl.awsm_release_renderbuffer();
    }

    //only pub within the module - used elsewhere like attributes
    pub(super) fn _bind_renderbuffer_nocheck(
        &self,
        renderbuffer_id: Id,
    ) -> Result<(), Error> {
        self.current_renderbuffer_id.set(Some(renderbuffer_id));

        let renderbuffer = self
            .renderbuffer_lookup
            .get(renderbuffer_id)
            .ok_or(Error::from(NativeError::MissingRenderBuffer))?;
        self.gl.awsm_bind_renderbuffer(&renderbuffer);

        Ok(())
    }

    #[cfg(feature = "disable_webgl_opt")]
    pub fn bind_renderbuffer(&self, renderbuffer_id: Id) -> Result<(), Error> {
        self._bind_renderbuffer_nocheck(renderbuffer_id)
    }

    #[cfg(not(feature = "disable_webgl_opt"))]
    pub fn bind_renderbuffer(&self, renderbuffer_id: Id) -> Result<(), Error> {
        if Some(renderbuffer_id) != self.current_renderbuffer_id.get()
        {
            self._bind_renderbuffer_nocheck(renderbuffer_id)
        } else {
            Ok(())
        }
    }

    //TODO: storage, multi-storage
}