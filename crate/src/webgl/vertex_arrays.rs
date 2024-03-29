use super::{AttributeOptions, BufferTarget, Id, WebGlRenderer, AttributeLocation};
use crate::errors::{Error, NativeError};
use web_sys::WebGlVertexArrayObject;
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext};

#[derive(Debug)]
pub struct VertexArray<'a> {
    pub attribute: NameOrLoc<'a>,
    pub buffer_id: Id,
    pub opts: AttributeOptions,
}

//Name is convenient and uses a local HashMap
//However - it can't reliably be used for vao's
//In a setup where vao's are loaded separately from shaders
#[derive(Debug)]
pub enum NameOrLoc<'a> {
    Name(&'a str),
    Loc(AttributeLocation)
}

impl <'a> VertexArray<'a> {
    pub fn new (attribute: NameOrLoc<'a>, buffer_id: Id, opts: AttributeOptions) -> Self {
        Self {
            attribute,
            buffer_id,
            opts,
        }
    }
}

macro_rules! impl_renderer {
    ($($type:ty { $($defs:tt)* })+) => {
        $(impl WebGlRenderer<$type> {


            pub fn release_vertex_array(&self) -> Result<(), Error> {
                self._bind_vertex_array(None, None)
            }

            pub fn activate_vertex_array(&self, vao_id:Id) -> Result<(), Error> {
                if Some(vao_id) != self.current_vao_id.get() {
                    if let Some(vao) = self.vao_lookup.get(vao_id) {
                        self._bind_vertex_array(Some(vao_id), Some(&vao))
                    } else {
                        Err(Error::from(NativeError::VertexArrayMissing))
                    }
                } else {
                    Ok(())
                }
            }

            pub fn assign_vertex_array(&mut self, vao_id:Id, element_buffer_id:Option<Id>, configs:&[VertexArray]) -> Result<(), Error> {
                let result = if let Some(vao) = self.vao_lookup.get(vao_id) {
                    self._bind_vertex_array(Some(vao_id), Some(&vao))?;

                    //Skip buffer assignment cache checks
                    if let Some(element_buffer_id) = element_buffer_id {
                        self._bind_buffer_nocheck(element_buffer_id, BufferTarget::ElementArrayBuffer)?;
                    }

                    for config in configs {
                        self._bind_buffer_nocheck(config.buffer_id, BufferTarget::ArrayBuffer)?;
                        match config.attribute {
                            NameOrLoc::Name(name) => {
                                self.activate_attribute_name(&name, &config.opts)?;
                            },
                            NameOrLoc::Loc(loc) => {
                                self.activate_attribute_loc(loc, &config.opts);
                            }
                        }
                    }
                    Ok(())
                } else {
                    Err(Error::from(NativeError::VertexArrayMissing))
                };

                //release it for the next call that might use elements
                self.release_vertex_array()?;

                result
            }

            $($defs)*
        })+
    };
}

impl_renderer! {
    WebGlRenderingContext{

        fn _bind_vertex_array(&self, id:Option<Id>, vao:Option<&WebGlVertexArrayObject>) -> Result<(), Error> {
            let ext = self.get_extension_vertex_array()?;
            ext.bind_vertex_array_oes(vao);
            self.current_vao_id.set(id);
            Ok(())
        }

        pub fn create_vertex_array(&mut self) -> Result<Id, Error> {
            let ext = self.get_extension_vertex_array()?;
            let vao = ext.create_vertex_array_oes().ok_or(Error::from(NativeError::VertexArrayCreate))?;
            let id = self.vao_lookup.insert(vao);
            Ok(id)
        }
    }
    WebGl2RenderingContext{
        fn _bind_vertex_array(&self, id:Option<Id>, vao:Option<&WebGlVertexArrayObject>) -> Result<(), Error> {
            self.gl.bind_vertex_array(vao);
            self.current_vao_id.set(id);
            Ok(())
        }

        pub fn create_vertex_array(&mut self) -> Result<Id, Error> {
            let vao = self.gl.create_vertex_array().ok_or(Error::from(NativeError::VertexArrayCreate))?;
            let id = self.vao_lookup.insert(vao);
            Ok(id)
        }

        pub fn delete_vertex_array(&self, vao_id: Id) -> Result<(), Error> {

            if Some(vao_id) == self.current_vao_id.get() {
                self.release_vertex_array()?;
            }

            if let Some(vao) = self.vao_lookup.get(vao_id) {
                self.gl.delete_vertex_array(Some(vao));
                Ok(())
            } else {
                Err(Error::from(NativeError::VertexArrayMissing))
            }

        }
    }
}
