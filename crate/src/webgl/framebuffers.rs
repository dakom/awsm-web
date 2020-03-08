use super::{BufferTarget, BufferUsage, Id, WebGlCommon, WebGlRenderer, FrameBufferTarget};
use crate::errors::{Error, NativeError};
use std::marker::PhantomData;
use web_sys::WebGlBuffer;
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext, WebGlFramebuffer};

pub trait PartialWebGlFrameBuffer {
    fn awsm_bind_framebuffer(&self, target: FrameBufferTarget, buffer: &WebGlFramebuffer);
    fn awsm_delete_framebuffer(&self, buffer: &WebGlFramebuffer);
    fn awsm_is_framebuffer(&self, buffer: &WebGlFramebuffer) -> bool;
    fn awsm_create_framebuffer(&self) -> Result<WebGlFramebuffer, Error>;
    fn awsm_release_framebuffer(&self, target: FrameBufferTarget);
    fn awsm_check_framebuffer_status(&self, target:FrameBufferTarget) -> Result<(), Error>;
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
            fn awsm_release_framebuffer(&self, target:FrameBufferTarget) {
                self.bind_framebuffer(target as u32, None);
            }

            fn awsm_check_framebuffer_status(&self, target:FrameBufferTarget) -> Result<(), Error> {
                //TODO!
                self.check_framebuffer(target as u32) === 
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
    pub fn create_framebuffer(&mut self) -> Result<Id, Error> {
        let framebuffer = self.gl.awsm_create_framebuffer()?;
        let id = self.framebuffer_lookup.insert(framebuffer);

        Ok(id)
    }

    pub fn delete_framebuffer(&self, framebuffer_id: Id) -> Result<(), Error> {
        if Some(framebuffer_id) == self.current_framebuffer_id.get() {
            if let Some(target) = self.current_framebuffer_target.get() {
                self.gl.awsm_release_framebuffer(target);
            }
            self.current_framebuffer_id.set(None);
            self.current_framebuffer_target.set(None);
        }

        let framebuffer = self
            .framebuffer_lookup
            .get(framebuffer_id)
            .ok_or(Error::from(NativeError::MissingFrameBuffer))?;

        self.gl.awsm_delete_framebuffer(&framebuffer);

        Ok(())
    }

    pub fn release_framebuffer(&self, target: FrameBufferTarget) {
        self.current_framebuffer_id.set(None);
        self.current_framebuffer_target.set(Some(target));

        self.gl.awsm_release_framebuffer(target);
    }

    //only pub within the module - used elsewhere like attributes
    pub(super) fn _bind_framebuffer_nocheck(
        &self,
        framebuffer_id: Id,
        target: FrameBufferTarget,
    ) -> Result<(), Error> {
        self.current_framebuffer_id.set(Some(framebuffer_id));
        self.current_framebuffer_target.set(Some(target));

        let framebuffer = self
            .framebuffer_lookup
            .get(framebuffer_id)
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;
        self.gl.awsm_bind_framebuffer(target, &framebuffer);

        Ok(())
    }

    #[cfg(feature = "disable_webgl_opt")]
    pub fn bind_framebuffer(&self, framebuffer_id: Id, target: FrameBufferTarget) -> Result<(), Error> {
        self._bind_framebuffer_nocheck(framebuffer_id, target)
    }

    #[cfg(not(feature = "disable_webgl_opt"))]
    pub fn bind_framebuffer(&self, framebuffer_id: Id, target: FrameBufferTarget) -> Result<(), Error> {
        if Some(framebuffer_id) != self.current_framebuffer_id.get()
            || Some(target) != self.current_framebuffer_target.get()
        {
            self._bind_framebuffer_nocheck(framebuffer_id, target)
        } else {
            Ok(())
        }
    }
}