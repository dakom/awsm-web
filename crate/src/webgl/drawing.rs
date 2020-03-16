use super::{BeginMode, BufferMask, DataType, WebGlCommon, WebGlRenderer, DrawBuffer};
use crate::errors::Error;
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext};
use crate::data::{TypedData};
use js_sys::Array;

pub trait PartialWebGlDrawing {
    fn awsm_clear(&self, bits: &[BufferMask]);
    fn awsm_draw_arrays(&self, mode: BeginMode, first: u32, count: u32);
    fn awsm_draw_elements(&self, mode: BeginMode, count: u32, data_type: DataType, offset: u32);
}

macro_rules! impl_context {
    ($($type:ty { $($defs:tt)* })+) => {
        $(impl PartialWebGlDrawing for $type {

            fn awsm_clear(&self, bits: &[BufferMask]) {
                let mut combined = 0u32;
                for bit in bits {
                    combined = combined | *bit as u32;
                }
                self.clear(combined);
            }

            fn awsm_draw_arrays(&self, mode: BeginMode, first: u32, count: u32) {
                self.draw_arrays(mode as u32, first as i32, count as i32);
            }

            fn awsm_draw_elements(&self, mode: BeginMode, count: u32, data_type:DataType, offset:u32) {
                self.draw_elements_with_i32(mode as u32, count as i32, data_type as u32, offset as i32);
            }

            $($defs)*
        })+
    };
}

impl_context! {
    WebGlRenderingContext{}
    WebGl2RenderingContext{}
}

impl<T: WebGlCommon> WebGlRenderer<T> {
    pub fn clear(&self, bits: &[BufferMask]) {
        self.gl.awsm_clear(&bits);
    }

    pub fn draw_arrays(&self, mode: BeginMode, first: u32, count: u32) {
        self.gl.awsm_draw_arrays(mode, first, count);
    }

    pub fn draw_elements(&self, mode: BeginMode, count: u32, data_type: DataType, offset: u32) {
        self.gl.awsm_draw_elements(mode, count, data_type, offset);
    }
}

impl WebGlRenderer<WebGlRenderingContext> {
    pub fn draw_buffers(&self, target_buffers: &[DrawBuffer]) -> Result<(), Error> {
        let ext = self.get_extension_draw_buffers()?;
        let target_buffers:&[u32] = unsafe { std::mem::transmute(target_buffers) };

        let js_array:Array = TypedData::new(target_buffers).into();

        ext.draw_buffers_webgl(&js_array);

        Ok(())
    }
}

impl WebGlRenderer<WebGl2RenderingContext> {
    pub fn draw_buffers(&self, target_buffers: &[DrawBuffer]) -> Result<(), Error> {
        let target_buffers:&[u32] = unsafe { std::mem::transmute(target_buffers) };

        let js_array:Array = TypedData::new(target_buffers).into();

        self.gl.draw_buffers(&js_array);

        Ok(())
    }
}
