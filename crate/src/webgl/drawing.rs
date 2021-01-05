use super::{BeginMode, BufferMask, DataType, WebGlCommon, WebGlRenderer, DrawBuffer, Buffer};
use crate::errors::Error;
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext};
use crate::data::{TypedData};
use js_sys::Array;

pub trait PartialWebGlDrawing {
    fn awsm_clear(&self, bits: &[BufferMask]);
    fn awsm_draw_arrays(&self, mode: BeginMode, first: u32, count: u32);
    fn awsm_draw_elements(&self, mode: BeginMode, count: u32, data_type: DataType, offset: u32);
}

pub trait PartialWebGl2Drawing {
    fn awsm_clear_draw_buffer_vf32(&self, buffer:Buffer, draw_buffer_index: usize, values:&[f32]);
    fn awsm_clear_draw_buffer_vi32(&self, buffer:Buffer, draw_buffer_index: usize, values:&[i32]);
    fn awsm_clear_draw_buffer_vu32(&self, buffer:Buffer, draw_buffer_index: usize, values:&[u32]);
    fn awsm_clear_draw_buffer_fi(&self, buffer:Buffer, draw_buffer_index: usize, depth:f32, stencil:i32);
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

impl PartialWebGl2Drawing for WebGl2RenderingContext {
    fn awsm_clear_draw_buffer_vf32(&self, buffer:Buffer, draw_buffer_index: usize, values:&[f32]) {
        self.clear_bufferfv_with_f32_array(buffer as u32, draw_buffer_index as i32, values);
    }
    fn awsm_clear_draw_buffer_vi32(&self, buffer:Buffer, draw_buffer_index: usize, values:&[i32]) {
        self.clear_bufferiv_with_i32_array(buffer as u32, draw_buffer_index as i32, values);
    }
    fn awsm_clear_draw_buffer_vu32(&self, buffer:Buffer, draw_buffer_index: usize, values:&[u32]) {
        self.clear_bufferuiv_with_u32_array(buffer as u32, draw_buffer_index as i32, values);
    }
    fn awsm_clear_draw_buffer_fi(&self, buffer:Buffer, draw_buffer_index: usize, depth:f32, stencil:i32) {
        self.clear_bufferfi(buffer as u32, draw_buffer_index as i32, depth, stencil);
    }
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

    //The "reset" values are all according to spec

    pub fn clear_draw_buffer_vf32_values(&self, buffer:Buffer, draw_buffer_index: usize, values:&[f32]) {
        self.gl.awsm_clear_draw_buffer_vf32(buffer, draw_buffer_index, values);
    }
    pub fn reset_color_draw_buffer_vf32(&self, draw_buffer_index: usize) {
        self.clear_draw_buffer_vf32_values(Buffer::Color, draw_buffer_index, &[0.0, 0.0, 0.0, 0.0]);
    }

    pub fn clear_draw_buffer_vi32_values(&self, buffer:Buffer, draw_buffer_index: usize, values:&[i32]) {
        self.gl.awsm_clear_draw_buffer_vi32(buffer, draw_buffer_index, values);
    }
    pub fn reset_color_draw_buffer_vi32(&self, draw_buffer_index: usize) {
        self.clear_draw_buffer_vi32_values(Buffer::Color, draw_buffer_index, &[0, 0, 0, 0]);
    }

    pub fn clear_draw_buffer_vu32_values(&self, buffer:Buffer, draw_buffer_index: usize, values:&[u32]) {
        self.gl.awsm_clear_draw_buffer_vu32(buffer, draw_buffer_index, values);
    }
    pub fn reset_color_draw_buffer_vu32(&self, draw_buffer_index: usize) {
        self.clear_draw_buffer_vu32_values(Buffer::Color, draw_buffer_index, &[0, 0, 0, 0]);
    }

    pub fn clear_draw_buffer_fi(&self, buffer:Buffer, draw_buffer_index: usize, depth:f32, stencil:i32) {
        self.gl.awsm_clear_draw_buffer_fi(buffer, draw_buffer_index, depth, stencil);
    }

    pub fn clear_draw_buffer_depth_stencil_values(&self, depth:f32, stencil:i32) {
        //spec says index must be 0 here
        self.clear_draw_buffer_fi(Buffer::DepthStencil, 0, depth, stencil);
    }
    pub fn reset_depth_stencil_draw_buffer(&self) {
        self.clear_draw_buffer_depth_stencil_values(1.0, 0)
    }
}
