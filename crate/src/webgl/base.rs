use super::funcs::FuncSettings;
use super::misc::MiscSettings;
use super::toggles::ToggleFlags;
use super::{ BufferTarget, FrameBufferTarget, GlQuery, Id, ProgramInfo, TextureInfo, WebGlCommon, WebGlVersion, BufferLocation, AttributeLocation};
use super::viewport::ResizeStrategy;
use crate::errors::{Error, NativeError};
use beach_map::{BeachMap, DefaultVersion};
use rustc_hash::FxHashMap;
use std::cell::Cell;
use std::any::Any;
use web_sys::{HtmlCanvasElement, WebGlBuffer, WebGlShader, WebGlVertexArrayObject, WebGlRenderbuffer, WebGlFramebuffer};
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext};

pub type WebGl1Renderer = WebGlRenderer<WebGlRenderingContext>;
pub type WebGl2Renderer = WebGlRenderer<WebGl2RenderingContext>;

/*
 * extension_lookup, attribute_lookup, and uniform_lookup are hashmaps
 * however, they are only populated at junctures where computation is already expensive
 *
 * everything else is a more efficient data structure
 * right now it's somewhat arbitrary that buffer/attribute/uniform setting happens through an
 * immutable api while textures and flags and things are though mutable.
 *
 * however that might come in handy down the line
 */

pub struct WebGlRenderer<T: WebGlCommon> {
    pub gl: T,
    pub canvas: HtmlCanvasElement,

    pub version: WebGlVersion,

    pub hardcoded_attribute_locations: FxHashMap<String, AttributeLocation>,

    //only in webgl2
    pub hardcoded_ubo_locations: FxHashMap<String, BufferLocation>,

    //really just local to the module
    pub(super) last_resize_strategy: Option<ResizeStrategy>, 
    pub(super) viewport: Option<(u32, u32, u32, u32)>,

    pub(super) shader_lookup: BeachMap<DefaultVersion, WebGlShader>,
    
    pub(super) current_program_id: Option<Id>,
    pub(super) program_lookup: BeachMap<DefaultVersion, ProgramInfo>,

    pub(super) current_framebuffer_id: Cell<Option<Id>>,
    pub(super) current_framebuffer_target: Cell<Option<FrameBufferTarget>>,
    pub(super) framebuffer_lookup: BeachMap<DefaultVersion, WebGlFramebuffer>,

    pub(super) current_renderbuffer_id: Cell<Option<Id>>,
    pub(super) renderbuffer_lookup: BeachMap<DefaultVersion, WebGlRenderbuffer>,

    pub(super) current_buffer_id: Cell<Option<Id>>,
    pub(super) current_buffer_target: Cell<Option<BufferTarget>>,
    pub(super) current_buffer_index: Cell<Option<u32>>, //only used for webgl_2
    pub(super) buffer_lookup: BeachMap<DefaultVersion, WebGlBuffer>,

    pub(super) texture_lookup: BeachMap<DefaultVersion, TextureInfo>,

    pub(super) extension_lookup: FxHashMap<String, js_sys::Object>,

    pub(super) current_vao_id: Cell<Option<Id>>,
    pub(super) vao_lookup: BeachMap<DefaultVersion, WebGlVertexArrayObject>,


    pub(super) toggle_flags: ToggleFlags,

    pub(super) func_settings: FuncSettings,
    pub(super) misc_settings: MiscSettings,
}

impl<T: WebGlCommon + 'static> WebGlRenderer<T> {

    pub fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    pub fn as_any(&self) -> &dyn Any {
        self
    }

    pub fn as_webgl1(&mut self) -> Result<&mut WebGl1Renderer, Error> {
        self.as_any_mut().downcast_mut::<WebGl1Renderer>().ok_or(Error::from(NativeError::WebGlVersion1))
    }
    pub fn as_webgl2(&mut self) -> Result<&mut WebGl2Renderer, Error> {
        self.as_any_mut().downcast_mut::<WebGl2Renderer>().ok_or(Error::from(NativeError::WebGlVersion2))
    }

    pub fn new(gl: T) -> Result<Self, Error> {
        let canvas = gl.awsm_get_canvas()?;

        let max_texture_units: usize =
            gl.awsm_get_parameter_usize(GlQuery::MaxTextureImageUnits)?;


        //The webgl docs don't talk about a default value...
        //seems to be 0 for all - but just in case... it's... set by browser? _shrug_
        let blend_color: Vec<f32> = gl.awsm_get_parameter_vf32(GlQuery::BlendColor)?;

        let version = gl.awsm_get_version();
        Ok(Self {
            gl,
            canvas,
            version,
            hardcoded_attribute_locations: FxHashMap::default(),
            hardcoded_ubo_locations: FxHashMap::default(),

            last_resize_strategy: None,
            viewport: None,

            shader_lookup: BeachMap::default(),

            current_program_id: None,
            program_lookup: BeachMap::default(),

            current_framebuffer_id: Cell::new(None),
            current_framebuffer_target: Cell::new(None),
            framebuffer_lookup: BeachMap::default(),
            
            current_renderbuffer_id: Cell::new(None),
            renderbuffer_lookup: BeachMap::default(),

            current_buffer_id: Cell::new(None),
            current_buffer_target: Cell::new(None),
            current_buffer_index: Cell::new(None),
            buffer_lookup: BeachMap::default(),

            texture_lookup: BeachMap::default(),

            extension_lookup: FxHashMap::default(),

            current_vao_id: Cell::new(None),
            vao_lookup: BeachMap::default(),
            
            toggle_flags: ToggleFlags::default(),

            func_settings: FuncSettings {
                blend_color: (
                    blend_color[0],
                    blend_color[1],
                    blend_color[2],
                    blend_color[3],
                ),
                ..FuncSettings::default()
            },

            misc_settings: MiscSettings::default(),
        })
    }
}
