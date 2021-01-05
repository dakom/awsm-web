use super::{
    BufferData, BufferDataImpl, BufferSubData, 
    BufferSubDataImpl, BufferTarget, BufferUsage, Id, WebGlRenderer,
    UniformBlockActiveQuery, ProgramQuery, UniformBlockQuery,
    shader::PartialWebGlShaders,
    WebGlSpecific
};
use crate::errors::{Error, NativeError};
use web_sys::WebGl2RenderingContext;
use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;
pub struct UniformBufferActivation {
    pub block_index: u32,
    pub offsets: FxHashMap<String, u32>
}

pub type UniformIndex = u32;
pub type BlockOffset = u32;
pub type BlockIndex = u32;
pub type BufferLocation = u32;
impl WebGlRenderer<WebGl2RenderingContext> {

    //Just used for debugging
    pub fn get_uniform_buffer_offsets(&self, program_id:Id) -> Result<Vec<(String, UniformIndex, BlockOffset)>, Error> {
        let program_info = self
            .program_lookup
            .get(program_id)
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;

        let max: u32 = self
            .gl
            .awsm_get_program_parameter_u32(
                &program_info.program,
                ProgramQuery::ActiveUniformBlocks,
            )
            .unwrap_or(0);

        let mut active_uniform_indices: Vec<u32> = Vec::new();
        if max > 0 {
            for i in 0..max {
                let uniforms: Vec<u32> = self
                    .gl
                    .get_active_uniform_block_parameter(
                        &program_info.program,
                        i,
                        UniformBlockQuery::ActiveUniformIndices as u32,
                    )
                    .map(|vals| {
                        let vals:js_sys::Uint32Array = vals.into();
                        vals.to_vec()
                    })?;
                active_uniform_indices.extend(uniforms);
            }
        }


        let offsets: Vec<u32> = unsafe {
            let values = js_sys::Uint32Array::view(&active_uniform_indices);
            let values = self.gl.get_active_uniforms(
                &program_info.program,
                &values,
                UniformBlockActiveQuery::Offset as u32,
            );
            let values:js_sys::Uint32Array= values.into();
            values.to_vec()
        };

        active_uniform_indices
            .iter()
            .enumerate()
            .map(|(idx, uniform_index)| {
                 self
                    .gl
                    .get_active_uniform(&program_info.program, *uniform_index)
                    .map(|info| (info.name(), info.type_(), info.size()))
                    .ok_or(Error::from(NativeError::UniformLocation(None)))
                    .map(|(u_name, _u_type_, _u_size)| {
                        (u_name, *uniform_index, offsets[idx])
                    })
                
            })
            .collect()

    }

    pub fn get_uniform_buffer_offset(&self, program_id:Id, uniform_index: u32) -> Result<u32, Error> {
        let program_info = self
            .program_lookup
            .get(program_id)
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;

        let active_uniforms:Vec<u32> = vec![uniform_index];

        let offsets: Vec<u32> = unsafe {
            let values = js_sys::Uint32Array::view(&active_uniforms);
            let values = self.gl.get_active_uniforms(
                &program_info.program,
                &values,
                UniformBlockActiveQuery::Offset as u32,
            );
            let values:js_sys::Uint32Array= values.into();
            values.to_vec()
        };

        if offsets.len() > 0 {
            Ok(offsets[0])
        } else {
            Err(Error::from(NativeError::UniformBufferOffsetMissing(None)))
        }

    }
    pub fn cache_uniform_buffer_location(&mut self, program_id: Id, name:&str) -> Result<(BufferLocation, bool), Error> {
        let location = {
            let program_info = self
                .program_lookup
                .get_mut(program_id)
                .ok_or(Error::from(NativeError::MissingShaderProgram))?;


            program_info.uniform_buffer_lookup_location.get(name)
        };

        match location {
            None => {
                let location = {
                    match self.hardcoded_ubo_locations.get(name) {
                        Some(location) => *location,
                        None => {
                            let program_info = self
                                .program_lookup
                                .get_mut(program_id)
                                .ok_or(Error::from(NativeError::MissingShaderProgram))?;

                            let hardcoded_max = 
                                self.hardcoded_ubo_locations
                                    .values()
                                    .fold(0, |acc, curr| {
                                        let curr = *curr;
                                        if curr > acc {
                                            curr
                                        } else {
                                            acc
                                        }
                                    });

                            program_info.non_global_ubo_count += 1;

                            (hardcoded_max + program_info.non_global_ubo_count) as u32
                        }
                    }
                };

                self
                    .program_lookup
                    .get_mut(program_id)
                    .ok_or(Error::from(NativeError::MissingShaderProgram))?
                    .uniform_buffer_lookup_location.insert(name.to_string(), location);


                #[cfg(feature = "debug_log")]
                log::info!(
                    "caching uniform buffer [{}] at location {}",
                    &name, location
                );
            
                Ok((location, true))
            }, 
            Some(location) => {
                Ok((*location, false))
            }
        }

    }
    pub fn cache_uniform_buffer_block_index(&mut self, program_id: Id, name:&str) -> Result<(BlockIndex, bool), Error> {
        let (location, _) = self.cache_uniform_buffer_location(program_id, name)?;

        let block_index = {
            let program_info = self
                .program_lookup
                .get_mut(program_id)
                .ok_or(Error::from(NativeError::MissingShaderProgram))?;


            program_info.uniform_buffer_lookup_activation
                .get(&location)
                .map(|activation| {
                    activation.block_index
                })
        };

        match block_index {
            None => {

                let block_index = {
                    let program_info = self
                        .program_lookup
                        .get(program_id)
                        .ok_or(Error::from(NativeError::MissingShaderProgram))?;

                    let block_index = self.gl.get_uniform_block_index(&program_info.program, &name);

                    if block_index == (WebGlSpecific::InvalidIndex as u32) {
                        return Err(Error::from(NativeError::UniformBufferBlockIndexMissing(Some(name.to_string()))));
                    }


                    block_index
                };

                self
                    .program_lookup
                    .get_mut(program_id)
                    .ok_or(Error::from(NativeError::MissingShaderProgram))?
                    .uniform_buffer_lookup_activation.insert(location, UniformBufferActivation{
                        block_index,
                        offsets: FxHashMap::default()
                    });

                #[cfg(feature = "debug_log")]
                log::info!(
                    "caching uniform buffer block offset for {} at index {}",
                    &name, block_index
                );
            
                Ok((block_index, true))
            },
            Some(block_index) => Ok((block_index, false))
        }

    }

    pub fn cache_uniform_buffer_block_offset_name(&mut self, program_id: Id, uniform_name:&str, block_name:&str) -> Result<(BlockOffset, bool), Error> {


        let (location, _) = self.cache_uniform_buffer_location(program_id, uniform_name)?;
        let (block_index, _) = self.cache_uniform_buffer_block_index(program_id, uniform_name)?;

        let offset:Option<u32> = {
            self
                .program_lookup
                .get(program_id)
                .ok_or(Error::from(NativeError::MissingShaderProgram))?
                .uniform_buffer_lookup_activation
                .get(&location)
                .ok_or(Error::from(NativeError::UniformBufferMissing(Some(uniform_name.to_string()))))?
                .offsets
                .get(block_name)
                .map(|offset| *offset)
        };

        if let Some(offset) = offset {
            //#[cfg(feature = "debug_log")]
            //log::info!("skipping uniform buffer cache for [{}] (already exists)", &name);
            Ok((offset, true))
        } else {
            let index = self.get_uniform_index_name(program_id, block_name)?;

            let offset = self.get_uniform_buffer_offset(program_id, index)
                .map_err(|_| {
                    Error::from(NativeError::UniformBufferOffsetMissing(Some((uniform_name.to_string(), block_name.to_string()))))
                })?;

            let offsets = &mut self
                .program_lookup
                .get_mut(program_id)
                .ok_or(Error::from(NativeError::MissingShaderProgram))?
                .uniform_buffer_lookup_activation
                .get_mut(&location)
                .ok_or(Error::from(NativeError::UniformBufferMissing(Some(uniform_name.to_string()))))?
                .offsets;

            offsets.insert(block_name.to_string(), offset);

            #[cfg(feature = "debug_log")]
            log::info!(
                "caching UBO offset, uniform: {}, block: {}, offset: {} ",
                uniform_name, block_name, offset 
            );

            Ok((offset, false))

            
        }
    }

    pub fn get_uniform_buffer_location_name(&mut self, name: &str) -> Result<BufferLocation, Error> {
        let program_id = self
            .current_program_id
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;

        self.cache_uniform_buffer_location(program_id, name)
            .map(|(location, _cached)| location)
    }
    pub fn get_uniform_buffer_block_index_name(&mut self, name: &str) -> Result<BlockIndex, Error> {
        let program_id = self
            .current_program_id
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;

        self.cache_uniform_buffer_block_index(program_id, name)
            .map(|(index, _cached)| index)
    }

    pub fn get_uniform_buffer_block_offset_name(
        &mut self,
        uniform_name: &str,
        block_name: &str,
    ) -> Result<BlockOffset, Error> {

        let program_id = self
            .current_program_id
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;

        self.cache_uniform_buffer_block_offset_name(program_id, uniform_name, block_name)
            .map(|(offset, _cached)| offset)

    }

    // at shader compilation time
    pub fn init_uniform_buffer_name(&mut self, name:&str) -> Result<(), Error> {
        let location = self.get_uniform_buffer_location_name(name)?;
        let block_index = self.get_uniform_buffer_block_index_name(name)?;
        self.init_uniform_buffer_loc(block_index, location)
    }
    pub fn init_uniform_buffer_loc(&mut self, block_index: BlockIndex, location: BufferLocation) -> Result<(), Error> {
        let program_id = self
            .current_program_id
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self
            .program_lookup
            .get(program_id)
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;

        self.gl.uniform_block_binding(&program_info.program, block_index, location);

        Ok(())
    }
   
    // At render time
    pub fn activate_uniform_buffer_loc(&mut self, id: Id, location:BufferLocation) {
        self.bind_buffer_base(id, location, BufferTarget::UniformBuffer);
    }

    pub fn activate_uniform_buffer_name(&mut self, id: Id, name:&str) -> Result<(), Error> {
        let location = self.get_uniform_buffer_location_name(name)?;
        self.bind_buffer_base(id, location, BufferTarget::UniformBuffer);
        Ok(())
    }

    ///upload buffer data
    pub fn upload_uniform_buffer<B: BufferDataImpl>(
        &mut self,
        id: Id,
        buffer_data: B,
    ) -> Result<(), Error> {
        match buffer_data.get_target() {
            BufferTarget::UniformBuffer => {
                self.upload_buffer(id, buffer_data)
            }
            _ => Err(Error::from(NativeError::UniformBufferTarget)),
        }
    }

    ///upload buffer data from sub slice
    pub fn upload_sub_uniform_buffer<B: BufferSubDataImpl>(
        &mut self,
        block_offset: BlockOffset,
        id: Id,
        buffer_data: B,
    ) -> Result<(), Error> {
        match buffer_data.get_target() {
            BufferTarget::UniformBuffer => {
                self.upload_buffer_sub(id, block_offset, buffer_data)
            }
            _ => Err(Error::from(NativeError::UniformBufferTarget)),
        }
    }

    ///convenience function
    pub fn upload_uniform_buffer_f32(
        &mut self,
        id: Id,
        values: &[f32],
        buffer_usage: BufferUsage,
    ) -> Result<(), Error> {
        self.upload_uniform_buffer(
            id,
            BufferData::new(values, BufferTarget::UniformBuffer, buffer_usage),
        )
    }
    ///convenience function
    pub fn upload_uniform_buffer_u8_loc(
        &mut self,
        id: Id,
        values: &[u8],
        buffer_usage: BufferUsage,
    ) -> Result<(), Error> {
        self.upload_uniform_buffer(
            id,
            BufferData::new(values, BufferTarget::UniformBuffer, buffer_usage),
        )
    }

    pub fn upload_sub_uniform_buffer_f32(
        &mut self,
        block_offset: BlockOffset,
        id: Id,
        values: &[f32],
    ) -> Result<(), Error> {
        self.upload_sub_uniform_buffer(
            block_offset,
            id,
            BufferSubData::new(values, BufferTarget::UniformBuffer),
        )
    }

    pub fn upload_sub_uniform_buffer_u8(
        &mut self,
        block_offset: BlockOffset,
        id: Id,
        values: &[u8],
    ) -> Result<(), Error> {
        self.upload_sub_uniform_buffer(
            block_offset,
            id,
            BufferSubData::new(values, BufferTarget::UniformBuffer),
        )
    }
}
