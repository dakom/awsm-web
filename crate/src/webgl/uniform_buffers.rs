use super::{
    BufferData, BufferDataImpl, BufferSubData, 
    BufferSubDataImpl, BufferTarget, BufferUsage, Id, WebGlRenderer,
    UniformBlockActiveQuery, ProgramQuery, UniformBlockQuery,
    shader::PartialWebGlShaders
};
use crate::errors::{Error, NativeError};
use web_sys::WebGl2RenderingContext;
use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;
pub struct UniformBufferLookup {
    pub block_index: u32,
    pub buffer_slot:u32,
    pub offsets: FxHashMap<String, u32>
}

pub type UniformIndex = u32;
pub type BlockOffset = u32;
pub type BufferSlot = u32;
impl WebGlRenderer<WebGl2RenderingContext> {

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
    pub fn cache_uniform_buffer_name(&mut self, program_id: Id, name:&str) -> Result<(BlockOffset, BufferSlot, bool), Error> {
        let (block_index, buffer_slot, fresh) = {
            let program_info = self
                .program_lookup
                .get_mut(program_id)
                .ok_or(Error::from(NativeError::MissingShaderProgram))?;


            let entry = program_info.uniform_buffer_lookup.entry(name.to_string());

            match entry {
                Entry::Occupied(entry) => {
                    //#[cfg(feature = "debug_log")]
                    //log::info!("skipping uniform buffer cache for [{}] (already exists)", &name);
                    let lookup = entry.get();
                    (lookup.block_index.clone(), lookup.buffer_slot.clone(), false)
                }
                Entry::Vacant(entry) => {
                    //placeholder values
                    let lookup = UniformBufferLookup {
                        block_index: 0,
                        buffer_slot: 0,
                        offsets: FxHashMap::default()
                    };

                    entry.insert(lookup);

                    (0, 0, true)
                }
            }
        };

        if fresh {

            //Need to get the current max via a mutable borrow...
            let buffer_slot = {
                let program_info = self
                    .program_lookup
                    .get(program_id)
                    .ok_or(Error::from(NativeError::MissingShaderProgram))?;

                program_info.uniform_buffer_lookup.len() as u32
            };

            let block_index = {
                let program_info = self
                    .program_lookup
                    .get(program_id)
                    .ok_or(Error::from(NativeError::MissingShaderProgram))?;

                let block_index = self.gl.get_uniform_block_index(&program_info.program, &name);

                self.gl.uniform_block_binding(&program_info.program, block_index, buffer_slot);

                block_index
            };

            let mut lookup:&mut UniformBufferLookup = self
                .program_lookup
                .get_mut(program_id)
                .ok_or(Error::from(NativeError::MissingShaderProgram))?
                .uniform_buffer_lookup.get_mut(name)
                .unwrap();

            lookup.block_index = block_index;
            lookup.buffer_slot = buffer_slot;

            #[cfg(feature = "debug_log")]
            log::info!(
                "caching uniform buffer [{}] at buffer index {} and slot {}",
                &name, block_index, buffer_slot 
            );
            
            Ok((block_index, buffer_slot, true))
        } else {
            Ok((block_index, buffer_slot, false))
        }

    }

    pub fn cache_uniform_buffer_block_offset_name(&mut self, program_id: Id, uniform_name:&str, block_name:&str) -> Result<(BlockOffset, bool), Error> {


        self.cache_uniform_buffer_name(program_id, uniform_name)?;

        let offset:Option<u32> = {
            self
                .program_lookup
                .get(program_id)
                .ok_or(Error::from(NativeError::MissingShaderProgram))?
                .uniform_buffer_lookup
                .get(uniform_name)
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

            let mut offsets = &mut self
                .program_lookup
                .get_mut(program_id)
                .ok_or(Error::from(NativeError::MissingShaderProgram))?
                .uniform_buffer_lookup
                .get_mut(uniform_name)
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

    pub fn get_uniform_buffer_location_name(&mut self, name: &str) -> Result<(BlockOffset, BufferSlot), Error> {
        let program_id = self
            .current_program_id
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;

        self.cache_uniform_buffer_name(program_id, name)
            .map(|(offset, slot, _cached)| (offset, slot))
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

    pub fn activate_uniform_buffer_name(&mut self, id: Id, name: &str) -> Result<(), Error> {
        let (_, slot) = self.get_uniform_buffer_location_name(&name)?;
        self.bind_buffer_base(id, slot, BufferTarget::UniformBuffer)
    }

    ///upload buffer data and set to uniform buffer
    pub fn upload_buffer_to_uniform_buffer_name<B: BufferDataImpl>(
        &mut self,
        name: &str,
        id: Id,
        buffer_data: B,
    ) -> Result<(), Error> {
        match buffer_data.get_target() {
            BufferTarget::UniformBuffer => {
                self.upload_buffer(id, buffer_data)?;
                self.activate_uniform_buffer_name(id, name)
            }
            _ => Err(Error::from(NativeError::UniformBufferTarget)),
        }
    }

    ///upload buffer data from sub slice and set to uniform buffer
    pub fn upload_buffer_sub_to_uniform_buffer_name<B: BufferSubDataImpl>(
        &mut self,
        uniform_name: &str,
        block_name: &str,
        id: Id,
        buffer_data: B,
    ) -> Result<(), Error> {
        match buffer_data.get_target() {
            BufferTarget::UniformBuffer => {
                let dest_byte_offset = self.get_uniform_buffer_block_offset_name(uniform_name, block_name)?;
                self.upload_buffer_sub(id, dest_byte_offset, buffer_data)?;
                self.activate_uniform_buffer_name(id, uniform_name)
            }
            _ => Err(Error::from(NativeError::UniformBufferTarget)),
        }
    }
    ///convenience function
    pub fn upload_buffer_to_uniform_buffer_f32_name(
        &mut self,
        name: &str,
        id: Id,
        values: &[f32],
        buffer_usage: BufferUsage,
    ) -> Result<(), Error> {
        self.upload_buffer_to_uniform_buffer_name(
            name,
            id,
            BufferData::new(values, BufferTarget::UniformBuffer, buffer_usage),
        )
    }

    ///convenience function
    pub fn upload_buffer_to_uniform_buffer_u8_name(
        &mut self,
        name: &str,
        id: Id,
        values: &[u8],
        buffer_usage: BufferUsage,
    ) -> Result<(), Error> {
        self.upload_buffer_to_uniform_buffer_name(
            name,
            id,
            BufferData::new(values, BufferTarget::UniformBuffer, buffer_usage),
        )
    }

    pub fn upload_buffer_sub_to_uniform_buffer_f32_name(
        &mut self,
        uniform_name: &str,
        block_name: &str,
        id: Id,
        values: &[f32],
    ) -> Result<(), Error> {
        self.upload_buffer_sub_to_uniform_buffer_name(
            uniform_name,
            block_name,
            id,
            BufferSubData::new(values, BufferTarget::UniformBuffer),
        )
    }

    pub fn upload_buffer_sub_to_uniform_buffer_u8_name(
        &mut self,
        uniform_name: &str,
        block_name: &str,
        id: Id,
        values: &[u8],
    ) -> Result<(), Error> {
        self.upload_buffer_sub_to_uniform_buffer_name(
            uniform_name,
            block_name,
            id,
            BufferSubData::new(values, BufferTarget::UniformBuffer),
        )
    }
}
