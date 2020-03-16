use super::{WebGlCommon, WebGlRenderer, ReadPixelFormat, ReadPixelDataType};
use crate::data::TypedData;
use crate::errors::Error;
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext};

pub trait PartialWebGlReadPixels {
    fn awsm_read_pixels_u8(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, data: &mut [u8]) -> Result<(), Error>;
    fn awsm_read_pixels_u8_typed(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, data: &mut [u8]) -> Result<(), Error>;
    fn awsm_read_pixels_u16(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, data: &mut [u16]) -> Result<(), Error>;
    fn awsm_read_pixels_f32(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, data: &mut [f32]) -> Result<(), Error>;
    fn _awsm_read_pixels_typed_data(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, data: &js_sys::Object) -> Result<(), Error>;
}
pub trait PartialWebGl2ReadPixels {
    fn awsm_read_pixels_u8_offset(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, offset: usize, data: &mut [u8]) -> Result<(), Error>;
    fn awsm_read_pixels_u8_typed_offset(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, offset: usize, data: &mut [u8]) -> Result<(), Error>;
    fn awsm_read_pixels_u16_offset(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, offset: usize, data: &mut [u16]) -> Result<(), Error>;
    fn awsm_read_pixels_f32_offset(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, offset: usize, data: &mut [f32]) -> Result<(), Error>;
    fn _awsm_read_pixels_typed_data_offset(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, offset: usize, data: &js_sys::Object) -> Result<(), Error>;
}

//impl<T: AsRef<[i8]>> From<TypedData<T, i8>> for Object {
macro_rules! impl_context {
    ($($type:ty { $($defs:tt)* })+) => {
        $(impl PartialWebGlReadPixels for $type {
            fn _awsm_read_pixels_typed_data(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, data: &js_sys::Object) -> Result<(), Error> {
                self.read_pixels_with_opt_array_buffer_view(
                    x as i32,
                    y as i32,
                    width as i32,
                    height as i32,
                    format as u32,
                    data_type as u32,
                    Some(data)
                ).map_err(|err| err.into())
            }
            fn awsm_read_pixels_u8(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, data: &mut [u8]) -> Result<(), Error> {
                self.read_pixels_with_opt_u8_array(
                    x as i32,
                    y as i32,
                    width as i32,
                    height as i32,
                    format as u32,
                    data_type as u32,
                    Some(data)
                ).map_err(|err| err.into())
            }
            fn awsm_read_pixels_u8_typed(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, data: &mut [u8]) -> Result<(), Error> {
                self._awsm_read_pixels_typed_data(x, y, width, height, format, data_type, &TypedData::new(data).into())
            }
            fn awsm_read_pixels_u16(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, data: &mut [u16]) -> Result<(), Error> {
                self._awsm_read_pixels_typed_data(x, y, width, height, format, data_type, &TypedData::new(data).into())
            }
            fn awsm_read_pixels_f32(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, data: &mut [f32]) -> Result<(), Error> {
                self._awsm_read_pixels_typed_data(x, y, width, height, format, data_type, &TypedData::new(data).into())
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

impl PartialWebGl2ReadPixels for WebGl2RenderingContext {
    fn _awsm_read_pixels_typed_data_offset(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, offset: usize, data: &js_sys::Object) -> Result<(), Error> {
        self.read_pixels_with_array_buffer_view_and_dst_offset(
            x as i32,
            y as i32,
            width as i32,
            height as i32,
            format as u32,
            data_type as u32,
            data,
            offset as u32,
        ).map_err(|err| err.into())
    }
    fn awsm_read_pixels_u8_offset(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, offset: usize, data: &mut [u8]) -> Result<(), Error> {
        self.read_pixels_with_u8_array_and_dst_offset(
            x as i32,
            y as i32,
            width as i32,
            height as i32,
            format as u32,
            data_type as u32,
            data,
            offset as u32
        ).map_err(|err| err.into())
    }
    fn awsm_read_pixels_u8_typed_offset(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, offset: usize, data: &mut [u8]) -> Result<(), Error> {
        self._awsm_read_pixels_typed_data_offset(x, y, width, height, format, data_type, offset, &TypedData::new(data).into())
    }
    fn awsm_read_pixels_u16_offset(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, offset: usize, data: &mut [u16]) -> Result<(), Error> {
        self._awsm_read_pixels_typed_data_offset(x, y, width, height, format, data_type, offset, &TypedData::new(data).into())
    }
    fn awsm_read_pixels_f32_offset(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, offset: usize, data: &mut [f32]) -> Result<(), Error> {
        self._awsm_read_pixels_typed_data_offset(x, y, width, height, format, data_type, offset, &TypedData::new(data).into())
    }
}

impl<T: WebGlCommon> WebGlRenderer<T> {
    pub fn read_pixels_u8(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, data: &mut [u8]) -> Result<(), Error> {
        self.gl.awsm_read_pixels_u8(x, y, width, height, format, data_type, data)
    }
    pub fn read_pixels_u8_typed(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, data: &mut [u8]) -> Result<(), Error> {
        self.gl.awsm_read_pixels_u8_typed(x, y, width, height, format, data_type, data)
    }
    pub fn read_pixels_u16(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, data: &mut [u16]) -> Result<(), Error> {
        self.gl.awsm_read_pixels_u16(x, y, width, height, format, data_type, data)
    }
    pub fn read_pixels_f32(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, data: &mut [f32]) -> Result<(), Error> {
        self.gl.awsm_read_pixels_f32(x, y, width, height, format, data_type, data)
    }
}

impl WebGlRenderer<WebGl2RenderingContext> {
    pub fn read_pixels_u8_offset(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, offset: usize, data: &mut [u8]) -> Result<(), Error> {
        self.gl.awsm_read_pixels_u8_offset(x, y, width, height, format, data_type, offset, data)
    }
    pub fn read_pixels_u8_typed_offset(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, offset: usize, data: &mut [u8]) -> Result<(), Error> {
        self.gl.awsm_read_pixels_u8_typed_offset(x, y, width, height, format, data_type, offset, data)
    }
    pub fn read_pixels_u16_offset(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, offset: usize, data: &mut [u16]) -> Result<(), Error> {
        self.gl.awsm_read_pixels_u16_offset(x, y, width, height, format, data_type, offset, data)
    }
    pub fn read_pixels_f32_offset(&self, x: u32, y: u32, width: u32, height: u32, format: ReadPixelFormat, data_type: ReadPixelDataType, offset: usize, data: &mut [f32]) -> Result<(), Error> {
        self.gl.awsm_read_pixels_f32_offset(x, y, width, height, format, data_type, offset, data)
    }
}