use super::id::Id;
use super::{
    ProgramQuery, ShaderQuery, ShaderType,
    WebGlCommon, WebGlRenderer,
};
use crate::errors::{Error, NativeError};
use rustc_hash::FxHashMap;
use web_sys::{WebGl2RenderingContext, WebGlActiveInfo, WebGlRenderingContext};
use web_sys::{WebGlProgram, WebGlShader, WebGlUniformLocation};
use crate::webgl::uniform_buffers::UniformBufferLookup;

pub struct ProgramInfo {
    pub program: WebGlProgram,
    pub attribute_lookup: FxHashMap<String, u32>,
    pub uniform_lookup: FxHashMap<String, WebGlUniformLocation>,
    pub texture_sampler_slot_lookup: FxHashMap<String, u32>,

    //only needed for webgl2
    pub uniform_buffer_lookup: FxHashMap<String, UniformBufferLookup>,
}


impl ProgramInfo {
    fn new(program: WebGlProgram) -> Self {
        Self {
            program,
            attribute_lookup: FxHashMap::default(),
            uniform_lookup: FxHashMap::default(),
            texture_sampler_slot_lookup: FxHashMap::default(),
            uniform_buffer_lookup: FxHashMap::default(),
        }
    }
}


pub trait PartialWebGlShaders {
    fn awsm_create_program(&self) -> Result<WebGlProgram, Error>;
    fn awsm_create_shader(&self, type_: ShaderType) -> Option<WebGlShader>;
    fn awsm_attach_shader(&self, program: &WebGlProgram, shader: &WebGlShader);
    fn awsm_detach_shader(&self, program: &WebGlProgram, shader: &WebGlShader);
    fn awsm_delete_shader(&self, shader: &WebGlShader);
    fn awsm_delete_program(&self, program: &WebGlProgram);
    fn awsm_shader_source(&self, shader: &WebGlShader, source: &str);
    fn awsm_compile_shader(&self, shader: &WebGlShader);
    fn awsm_link_program(&self, program: &WebGlProgram);
    fn awsm_get_shader_parameter_bool(
        &self,
        shader: &WebGlShader,
        query: ShaderQuery,
    ) -> Result<bool, Error>;
    fn awsm_get_program_parameter_bool(
        &self,
        program: &WebGlProgram,
        query: ProgramQuery,
    ) -> Result<bool, Error>;
    fn awsm_get_program_parameter_u32(
        &self,
        program: &WebGlProgram,
        query: ProgramQuery,
    ) -> Result<u32, Error>;
    fn awsm_get_shader_info_log(&self, shader: &WebGlShader) -> Option<String>;
    fn awsm_get_program_info_log(&self, program: &WebGlProgram) -> Option<String>;
    fn awsm_use_program(&self, program: &WebGlProgram);
    fn awsm_get_active_uniform(
        &self,
        program: &WebGlProgram,
        index: u32,
    ) -> Result<WebGlActiveInfo, Error>;
    fn awsm_get_active_attrib(
        &self,
        program: &WebGlProgram,
        index: u32,
    ) -> Result<WebGlActiveInfo, Error>;

    fn awsm_bind_attrib_location(&self, program: &WebGlProgram, index: u32, name: &str);
}

macro_rules! impl_context {
    ($($type:ty { $($defs:tt)* })+) => {
        $(impl PartialWebGlShaders for $type {
            //Put all the common methods here:

            fn awsm_create_program(&self) -> Result<WebGlProgram, Error> {
                self.create_program().ok_or(Error::Native(NativeError::WebGlProgram))
            }

            fn awsm_create_shader(&self, type_: ShaderType) -> Option<WebGlShader> {
                self.create_shader(type_ as u32)
            }

            fn awsm_attach_shader(&self, program: &WebGlProgram, shader: &WebGlShader) {
                self.attach_shader(program, shader);
            }

            fn awsm_detach_shader(&self, program: &WebGlProgram, shader: &WebGlShader) {
                self.detach_shader(program, shader);
            }

            fn awsm_delete_shader(&self, shader: &WebGlShader) {
                self.delete_shader(Some(shader));
            }

            fn awsm_delete_program(&self, program: &WebGlProgram) {
                self.delete_program(Some(program));
            }

            fn awsm_shader_source(&self, shader: &WebGlShader, source: &str) {
                self.shader_source(shader, source);
            }

            fn awsm_compile_shader(&self, shader:&WebGlShader) {
                self.compile_shader(shader);
            }

            fn awsm_link_program(&self, program: &WebGlProgram) {
                self.link_program(program);
            }

            fn awsm_get_shader_parameter_bool(&self, shader: &WebGlShader, query: ShaderQuery) -> Result<bool, Error> {
                self.get_shader_parameter(shader, query as u32)
                    .as_bool()
                    .ok_or(Error::from(NativeError::JsValueExpectedBool))
            }

            fn awsm_get_program_parameter_bool(&self, program: &WebGlProgram, query: ProgramQuery) -> Result<bool, Error> {
                self.get_program_parameter(program, query as u32)
                    .as_bool()
                    .ok_or(Error::from(NativeError::JsValueExpectedBool))
            }

            fn awsm_get_program_parameter_u32(&self, program:&WebGlProgram, query:ProgramQuery) -> Result<u32, Error> {
                let number = self.get_program_parameter(program, query as u32)
                    .as_f64()
                    .ok_or(Error::from(NativeError::JsValueExpectedNumber))?;

                Ok(number as u32)
            }

            fn awsm_get_shader_info_log(&self, shader: &WebGlShader) -> Option<String> {
                self.get_shader_info_log(shader)
            }

            fn awsm_get_program_info_log(&self, program: &WebGlProgram) -> Option<String> {
                self.get_program_info_log(program)
            }

            fn awsm_use_program(&self, program: &WebGlProgram) {
                self.use_program(Some(program))
            }

            fn awsm_get_active_uniform( &self, program: &WebGlProgram, index: u32) -> Result<WebGlActiveInfo, Error> {
                self.get_active_uniform(program, index)
                    .ok_or(Error::from(NativeError::UniformLocation(None)))
            }

            fn awsm_get_active_attrib(&self, program: &WebGlProgram, index: u32) -> Result<WebGlActiveInfo, Error> {
                self.get_active_attrib(program, index)
                    .ok_or(Error::from(NativeError::AttributeLocation(None)))
            }
    
            fn awsm_bind_attrib_location(&self, program: &WebGlProgram, index: u32, name: &str) {
                self.bind_attrib_location(program, index, name);
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
    pub fn activate_program(&mut self, program_id: Id) -> Result<(), Error> {
        if Some(program_id) != self.current_program_id {
            self.current_program_id = Some(program_id);
            let program_info = self
                .program_lookup
                .get(program_id)
                .ok_or(Error::from(NativeError::MissingShaderProgram))?;
            self.gl.awsm_use_program(&program_info.program);
            Ok(())
        } else {
            Ok(())
        }
    }

    //Compile the shader - and cache it for later use
    pub fn compile_shader(&mut self, source:&str, source_type: ShaderType) -> Result<Id, Error> {
        let shader = compile_shader(&self.gl, source, source_type)?;

        Ok(self.shader_lookup.insert(shader))
    }
}

impl WebGlRenderer<WebGlRenderingContext> {
    //Compile the program and cache it for later use
    pub fn compile_program(&mut self, shaders:&[Id]) -> Result<Id, Error> {
        let shaders:Vec<&WebGlShader> = 
            shaders
                .iter()
                .map(|id| {
                    self.shader_lookup
                        .get(*id)
                        .ok_or(Error::from("can't get shader for id!"))
                }).collect::<Result<Vec<&WebGlShader>, Error>>()?;

        let program = compile_program(&self.gl, &shaders, &self.hardcoded_attribute_locations)?;

        let program_info = ProgramInfo::new(program);

        let id = self.program_lookup.insert(program_info);

        self.activate_program(id)?;

        Ok(id)
    }
}

impl WebGlRenderer<WebGl2RenderingContext> {
    //Compile the program and cache it for later use
    pub fn compile_program(&mut self, shaders:&[Id]) -> Result<Id, Error> {
        let shaders:Vec<&WebGlShader> = 
            shaders
                .iter()
                .map(|id| {
                    self.shader_lookup
                        .get(*id)
                        .ok_or(Error::from("can't get shader for id!"))
                }).collect::<Result<Vec<&WebGlShader>, Error>>()?;

        let program = compile_program(&self.gl, &shaders, &self.hardcoded_attribute_locations)?;

        let program_info = ProgramInfo::new(program);

        let id = self.program_lookup.insert(program_info);

        self.activate_program(id)?;

        Ok(id)
    }
}


//These are free functions since they might
//Be useful for compiling shaders and programs
//Without storing in the renderer cache
pub fn compile_program<T: WebGlCommon>(
    gl: &T,
    shaders: &[&WebGlShader],
    hardcoded_attribute_locations: &FxHashMap<String, u32>
) -> Result<WebGlProgram, Error> {


    gl.awsm_create_program()
        .and_then(|program| {
            //Hardcode our stashed attribute locations
            //TODO - is this necessary?
            //I think it is for WebGl1 or if layout isn't specified in WebGl2
            //So.... yeah?
            //Anyway it's not expensive to do this...
            for (name, loc) in hardcoded_attribute_locations {
                gl.awsm_bind_attrib_location(&program, *loc, name);
            }
            
            for shader in shaders.iter() {
                gl.awsm_attach_shader(&program, shader);
            }

            //Link the program
            gl.awsm_link_program(&program);

            //Check for errors
            check_status_with(
                || gl.awsm_get_program_parameter_bool(&program, ProgramQuery::LinkStatus),
                || gl.awsm_get_program_info_log(&program),
            )
                .map_err(|err| {
                    //Don't delete the shader - technically a delete will be only marked
                    //for GC, but if this is the _first_ use of it, and we want to use
                    //it after, that will collect it before we get the chance
                    for shader in shaders.iter() {
                        gl.awsm_detach_shader(&program, shader);
                    }

                    gl.awsm_delete_program(&program);

                    err
                })
                .map(|_| program)
        })
}

pub fn compile_shader<T: WebGlCommon>(
    gl: &T,
    source: &str,
    source_type: ShaderType
) -> Result<WebGlShader, Error> {
    let shader = gl.awsm_create_shader(source_type).ok_or(Error::from("bad shader (unknown error)"))?;

    gl.awsm_shader_source(&shader, source);
    gl.awsm_compile_shader(&shader);
    check_status_with(
        || gl.awsm_get_shader_parameter_bool(&shader, ShaderQuery::CompileStatus),
        || gl.awsm_get_shader_info_log(&shader),
    )?;

    Ok(shader)

}

fn check_status_with<T, U>(set_status: T, get_status: U) -> Result<(), Error>
where
    T: Fn() -> Result<bool, Error>,
    U: Fn() -> Option<String>,
{
    match set_status() {
        Ok(flag) => {
            if !flag {
                match get_status() {
                    None => Err(String::from("unknown shader compiler error!")),
                    Some(err) => Err(err),
                }
            } else {
                Ok(())
            }
        }

        Err(err) => Err(err.to_string()),
    }
    .map_err(|err| err.into())
}

// see: https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getActiveUniform
// the only type we really need to deal with is "Array of basic type"
// since the others will generate each entry
pub fn parse_uniform_names(input:&str, size:usize) -> Vec<String> {
    //get the base before [N] as well as the value of N
    if let Some(base) = 
        //if it ends with [N]
        input.rfind('[').and_then(|start| {
            input.rfind(']').and_then(|end| {
                if end == input.len()-1 {
                    Some((start, end))
                } else {
                    None
                }
            })
        })
        .and_then(|(start, end)| {
            let base = &input[..start];
            let suffix = &input[start+1..end];
            //and N is a valid number
            suffix.parse::<usize>().ok().map(|n| {
                if n != 0 {
                    panic!("uniform array string index should be 0!");
                }
                base
            })
    }) {
        let mut list = vec![base.to_string()]; 
        for i in 0..size {
            list.push(format!("{}[{}]", base, i))
        }
        list
    } else {
        //otherwise just return the input as-is
        vec![input.to_string()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_uniform_names() {
        assert_eq!(get_uniform_names("hello", 2), ["hello"]);
        assert_eq!(get_uniform_names("hello[0]", 4), ["hello", "hello[0]", "hello[1]", "hello[2]", "hello[3]"]);
    }

    #[test]
    #[should_panic]
    fn test_invalid_uniform_names() {
        assert_eq!(get_uniform_names("hello[4]", 4), ["hello", "hello[0]", "hello[1]", "hello[2]", "hello[3]"]);
    }
}
