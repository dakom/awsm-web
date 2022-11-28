use super::{RENDERBUFFER_TARGET, Id, WebGlCommon, WebGlRenderer, ReadBuffer, FrameBufferTarget, FrameBufferStatus, FrameBufferAttachment, FrameBufferTextureTarget, BufferMask, BlitFilter};
use crate::errors::{Error, NativeError};
use crate::data::{TypedData};
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext, WebGlFramebuffer, WebGlTexture, WebGlRenderbuffer};
use js_sys::Array;
use std::convert::TryInto;

pub trait PartialWebGlFrameBuffer {
    fn awsm_bind_framebuffer(&self, target: FrameBufferTarget, buffer: &WebGlFramebuffer);
    fn awsm_delete_framebuffer(&self, buffer: &WebGlFramebuffer);
    fn awsm_is_framebuffer(&self, buffer: &WebGlFramebuffer) -> bool;
    fn awsm_create_framebuffer(&self) -> Result<WebGlFramebuffer, Error>;
    fn awsm_release_framebuffer(&self, target: FrameBufferTarget);
    fn awsm_check_framebuffer_status(&self, target:FrameBufferTarget) -> Result<(), Error>;
    fn awsm_framebuffer_texture_2d(&self, target: FrameBufferTarget, attachment: FrameBufferAttachment, texture_target: FrameBufferTextureTarget, texture: &WebGlTexture);
    fn awsm_framebuffer_renderbuffer(&self, target: FrameBufferTarget, attachment: FrameBufferAttachment, renderbuffer: &WebGlRenderbuffer);
}
pub trait PartialWebGl2FrameBuffer {
    fn awsm_blit_framebuffer(&self, src_x0: u32, src_y0: u32, src_x1: u32, src_y1: u32, dst_x0: u32, dst_y0: u32, dst_x1: u32, dst_y1: u32, mask: BufferMask, filter: BlitFilter);
    fn awsm_framebuffer_texture_layer(&self, target: FrameBufferTarget, attachment: FrameBufferAttachment, texture: &WebGlTexture, mipmap_level: u32, layer:u32);
    fn awsm_invalidate_framebuffer(&self, target: FrameBufferTarget, attachments: &[FrameBufferAttachment]) -> Result<(), Error>;
    fn awsm_invalidate_sub_framebuffer(&self, target: FrameBufferTarget, attachments: &[FrameBufferAttachment], x: u32, y: u32, width: usize, height: usize) -> Result<(), Error>;
    fn awsm_read_buffer(&self, src: ReadBuffer);
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
                let status:FrameBufferStatus = self.check_framebuffer_status(target as u32).try_into()?;

                match status {
                    FrameBufferStatus::Complete => Ok(()),
                    FrameBufferStatus::IncompleteAttachment => Err(NativeError::FrameBuffer(Some("incomplete attachment".to_string())).into()),
                    FrameBufferStatus::IncompleteMissingAttachment => Err(NativeError::FrameBuffer(Some("incomplete missing attachment".to_string())).into()),
                    FrameBufferStatus::IncompleteDimensions => Err(NativeError::FrameBuffer(Some("incomplete dimensions".to_string())).into()),
                    FrameBufferStatus::Unsupported => Err(NativeError::FrameBuffer(Some("unsupported".to_string())).into()),
                    FrameBufferStatus::IncompleteMultisample => Err(NativeError::FrameBuffer(Some("incomplete multisample".to_string())).into()),
                    FrameBufferStatus::Samples => Err(NativeError::FrameBuffer(Some("samples".to_string())).into()),
                    FrameBufferStatus::IncompleteViewTargetsOvr => Err(NativeError::FrameBuffer(Some("incomplete view targets ovr".to_string())).into()),
                }
            }

            fn awsm_framebuffer_texture_2d(&self, target: FrameBufferTarget, attachment: FrameBufferAttachment, texture_target: FrameBufferTextureTarget, texture: &WebGlTexture) {


                self.framebuffer_texture_2d(
                    target as u32,
                    attachment as u32,
                    texture_target as u32,
                    Some(texture),
                    0 //according to spec, this is always 0
                );
            }

            fn awsm_framebuffer_renderbuffer(&self, target: FrameBufferTarget, attachment: FrameBufferAttachment, renderbuffer: &WebGlRenderbuffer) {
                self.framebuffer_renderbuffer(
                    target as u32,
                    attachment as u32,
                    RENDERBUFFER_TARGET,
                    Some(renderbuffer),
                );
            }

            $($defs)*
        })+
    };
}

impl PartialWebGl2FrameBuffer for WebGl2RenderingContext {
    fn awsm_blit_framebuffer(&self, src_x0: u32, src_y0: u32, src_x1: u32, src_y1: u32, dst_x0: u32, dst_y0: u32, dst_x1: u32, dst_y1: u32, mask: BufferMask, filter: BlitFilter) {
        self.blit_framebuffer(
            src_x0 as i32,
            src_y0 as i32,
            src_x1 as i32,
            src_y1 as i32,
            dst_x0 as i32,
            dst_y0 as i32,
            dst_x1 as i32,
            dst_y1 as i32,
            mask as u32,
            filter as u32
        )
    }
    fn awsm_framebuffer_texture_layer(&self, target: FrameBufferTarget, attachment: FrameBufferAttachment, texture: &WebGlTexture, mipmap_level: u32, layer:u32) {
        self.framebuffer_texture_layer(
            target as u32,
            attachment as u32,
            Some(texture),
            mipmap_level as i32,
            layer as i32
        )
    }
    fn awsm_invalidate_framebuffer(&self, target: FrameBufferTarget, attachments: &[FrameBufferAttachment]) -> Result<(), Error> {
        let attachments:&[u32] = unsafe { std::mem::transmute(attachments) };

        let js_array:Array = TypedData::new(attachments).into();

        self.invalidate_framebuffer(
            target as u32, 
            &js_array
        ).map_err(|err| err.into())
    }
    fn awsm_invalidate_sub_framebuffer(&self, target: FrameBufferTarget, attachments: &[FrameBufferAttachment], x: u32, y: u32, width: usize, height: usize) -> Result<(), Error> {
        let attachments:&[u32] = unsafe { std::mem::transmute(attachments) };
        
        let js_array:Array = TypedData::new(attachments).into();

        self.invalidate_sub_framebuffer(
            target as u32,
            &js_array,
            x as i32,
            y as i32,
            width as i32,
            height as i32,
        ).map_err(|err| err.into())
    }
    fn awsm_read_buffer(&self, src: ReadBuffer) {
        self.read_buffer(src as u32)
    }
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

    pub fn delete_framebuffer(&mut self, id: Id) -> Result<(), Error> {
        if Some(id) == self.current_framebuffer_id.get() {
            if let Some(target) = self.current_framebuffer_target.get() {
                self.gl.awsm_release_framebuffer(target);
            }
            self.current_framebuffer_id.set(None);
            self.current_framebuffer_target.set(None);
        }

        let framebuffer = self
            .framebuffer_lookup
            .get(id)
            .ok_or(Error::from(NativeError::MissingFrameBuffer))?;

        self.gl.awsm_delete_framebuffer(&framebuffer);

        self.framebuffer_lookup.remove(id);
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
            .ok_or(Error::from(NativeError::MissingFrameBuffer))?;
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
    
    pub fn assign_framebuffer_texture_2d(&mut self, framebuffer_id: Id, texture_id: Id, target: FrameBufferTarget, attachment: FrameBufferAttachment, texture_target: FrameBufferTextureTarget) -> Result<(), Error> {
        {
            let texture = self.get_texture(texture_id)?;

            self.bind_framebuffer(framebuffer_id, target)?;
            self.gl.awsm_framebuffer_texture_2d(target, attachment, texture_target, texture);
        }
        Ok(())
    }

    pub fn assign_framebuffer_renderbuffer(&self, framebuffer_id: Id, renderbuffer_id: Id, target: FrameBufferTarget, attachment: FrameBufferAttachment) -> Result<(), Error> {
        self.bind_framebuffer(framebuffer_id, target)?;
        let renderbuffer = self.get_renderbuffer(renderbuffer_id)?;
        self.gl.awsm_framebuffer_renderbuffer(target, attachment, renderbuffer);
        Ok(())
    }
    
    pub fn check_framebuffer_status(&self, target:FrameBufferTarget) -> Result<(), Error> {
        self.gl.awsm_check_framebuffer_status(target)
    }
}


impl WebGlRenderer<WebGl2RenderingContext> {
    pub fn blit_framebuffer(&self, src_x0: u32, src_y0: u32, src_x1: u32, src_y1: u32, dst_x0: u32, dst_y0: u32, dst_x1: u32, dst_y1: u32, mask: BufferMask, filter: BlitFilter) {
        self.gl.awsm_blit_framebuffer(src_x0, src_y0, src_x1, src_y1, dst_x0, dst_y0, dst_x1, dst_y1, mask, filter)
    }
    pub fn framebuffer_texture_layer(&mut self, target: FrameBufferTarget, attachment: FrameBufferAttachment, texture_id:Id, mipmap_level: u32, layer:u32) -> Result<(), Error> {
        let texture = self.get_texture(texture_id)?;
        self.gl.awsm_framebuffer_texture_layer(target, attachment, texture, mipmap_level, layer);

        Ok(())
    }
    pub fn invalidate_framebuffer(&self, target: FrameBufferTarget, attachments: &[FrameBufferAttachment]) -> Result<(), Error> {
        self.gl.awsm_invalidate_framebuffer(target, attachments)
    }
    pub fn invalidate_sub_framebuffer(&self, target: FrameBufferTarget, attachments: &[FrameBufferAttachment], x: u32, y: u32, width: usize, height: usize) -> Result<(), Error> {
        self.gl.awsm_invalidate_sub_framebuffer(target, attachments, x, y, width, height)
    }
    pub fn read_buffer(&self, src: ReadBuffer) {
        self.gl.awsm_read_buffer(src)
    }
}
