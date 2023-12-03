use super::{enums::GlParameter, WebGlRenderer, GlQueryKind, GlQueryResult, GlQueryName};
use crate::errors::Error;
use wasm_bindgen::{prelude::JsValue, JsCast};
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext, WebGlQuery};

pub trait PartialWebGlGetParameter {
    fn awsm_get_parameter_usize(&self, query: GlParameter) -> Result<usize, Error>;
    fn awsm_get_parameter_vf32(&self, query: GlParameter) -> Result<Vec<f32>, Error>;
}

macro_rules! impl_context {
    ($($type:ty { $($defs:tt)* })+) => {
        $(impl PartialWebGlGetParameter for $type {
            fn awsm_get_parameter_usize(&self, query:GlParameter) -> Result<usize, Error> {
                self.get_parameter(query as u32)
                    .and_then(|value| {
                              value
                                .as_f64()
                                .map(|val| val as usize)
                                .ok_or(JsValue::null())
                    })
                    .map_err(|err| err.into())
            }

            fn awsm_get_parameter_vf32(&self, query:GlParameter) -> Result<Vec<f32>, Error> {
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

impl WebGlRenderer<WebGl2RenderingContext> {
    pub fn create_query(&self) -> Result<WebGlQuery, Error> {
        self.gl.create_query().ok_or(Error::from("Unable to create query"))
    }

    pub fn begin_query(&self, kind: GlQueryKind, query: &WebGlQuery) {
        self.gl.begin_query(kind as u32, query);
    }

    pub fn end_query(&self, kind: GlQueryKind) {
        self.gl.end_query(kind as u32);
    }

    pub fn delete_query(&self, query: WebGlQuery) {
        self.gl.delete_query(Some(&query));
    }

    pub fn current_query(&self, kind: GlQueryKind) -> Option<WebGlQuery> {
        let value = self.gl.get_query(kind as u32, GlQueryName::Current as u32);

        if value.is_null() {
            None
        } else {
            Some(value.unchecked_into())
        }
    }

    pub fn query_available(&self, query: &WebGlQuery) -> Result<bool, Error> {
        let value = self.gl.get_query_parameter(query, GlQueryResult::Available as u32);
        let value = value.as_bool().ok_or(Error::from("Unable to get query result"))?;
        Ok(value)
    }

    pub fn query_result(&self, query: &WebGlQuery) -> Result<u32, Error> {
        let value = self.gl.get_query_parameter(query, GlQueryResult::Value as u32);
        let value = value.as_f64().ok_or(Error::from("Unable to get query result"))?;
        Ok(value as u32)
    }
}