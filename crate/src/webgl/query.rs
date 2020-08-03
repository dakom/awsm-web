use super::enums::GlQuery;
use crate::errors::Error;
use wasm_bindgen::prelude::JsValue;
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext};

pub trait PartialWebGlQueries {
    fn awsm_get_parameter_usize(&self, query: GlQuery) -> Result<usize, Error>;
    fn awsm_get_parameter_vf32(&self, query: GlQuery) -> Result<Vec<f32>, Error>;
}

macro_rules! impl_context {
    ($($type:ty { $($defs:tt)* })+) => {
        $(impl PartialWebGlQueries for $type {
            fn awsm_get_parameter_usize(&self, query:GlQuery) -> Result<usize, Error> {
                self.get_parameter(query as u32)
                    .and_then(|value| {
                              value
                                .as_f64()
                                .map(|val| val as usize)
                                .ok_or(JsValue::null())
                    })
                    .map_err(|err| err.into())
            }

            fn awsm_get_parameter_vf32(&self, query:GlQuery) -> Result<Vec<f32>, Error> {
                self.get_parameter(query as u32)
                        .map(|value| {
                            let value:js_sys::Float32Array = value.into();
                            value.to_vec()
                        })
                        .map_err(|err| err.into())
            }

            $($defs)*
        })+
    };
}

impl_context! {
    WebGlRenderingContext{}
    WebGl2RenderingContext{}
}
