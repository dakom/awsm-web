use std::fmt;
use wasm_bindgen::prelude::JsValue;

pub enum Error {
    Empty,
    String(String),
    Js(JsValue),
    Native(NativeError),
}

pub enum NativeError {
    Abort,
    Canvas2dContext,
    WebGlContext,
    WebGlProgram,
    WebGlCanvas,
    GlobalSelf,
    Window,
    WindowWidth,
    WindowHeight,
    WebGlBufferSourceOneNonZero,
    CanvasCreate,
    AttributeLocation(Option<String>),
    UniformLocation(Option<String>),
    UniformIndex(Option<String>),
    MipsPowerOf2,
    NoExtension,
    MissingShaderProgram,
    NoCreateBuffer,
    NoCreateFrameBuffer,
    NoCreateRenderBuffer,
    NoExistingBuffer,
    NoCreateTexture,
    MissingTexture,
    MissingTextureSampler(Option<String>),
    MissingBuffer,
    MissingRenderBuffer,
    MissingFrameBuffer,
    UniformSize,
    UniformMatrixMustBeFloat,
    UniformType,
    UniformBufferName,
    UniformBufferParameter,
    UniformBufferMissing(Option<String>),
    UniformBufferOffsetMissing(Option<(String, String)>),
    UniformBufferBlockIndexMissing(Option<String>),
    UniformBufferTarget,
    VertexArrayMissing,
    VertexArrayCreate,
    JsValueExpectedBool,
    JsValueExpectedNumber,
    WebGl1TextureArray2d,
    WebGl1Texture3d,
    WebGl1TextureOffsetNonZero,
    WebGlVersion1,
    WebGlVersion2,
    TextureCubeFaceNotCube,
    TextureMissingCubeFace,
    NoTextureTarget,
    Internal,
    FrameBuffer(Option<String>),
}

impl Error {
    pub fn to_js(self: &Self) -> JsValue {
        match self {
            Error::Empty => JsValue::null(),
            Error::String(s) => JsValue::from_str(&s[..]),
            Error::Js(jval) => jval.clone(),
            Error::Native(err) => JsValue::from_str(err.to_string().as_str()),
        }
    }

    pub fn is_abort(&self) -> bool {
        match self {
            Error::Native(err) => std::mem::discriminant(err) == std::mem::discriminant(&NativeError::Abort),
            Error::Js(err) => {
                match js_value_name(err) {
                    Some(name) => name == "AbortError",
                    _ => false
                }
            },
            _ => false,
        }
    }

}

pub fn js_value_name(err:&JsValue) -> Option<String> {
    js_sys::Reflect::get(&err, &JsValue::from_str("name"))
        .ok()
        .and_then(|value| value.as_string())
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Error::Empty => write!(f, "empty error"),
            _ => write!(
                f,
                "{}",
                self.to_js()
                    .as_string()
                    .unwrap_or("unknown error".to_string())
            ),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Error::Empty => write!(f, "empty error"),
            _ => write!(
                f,
                "{}",
                self.to_js()
                    .as_string()
                    .unwrap_or("unknown error".to_string())
            ),
        }
    }
}

impl NativeError {
    pub fn default_str(self: &Self) -> &'static str {
        match self {
            NativeError::Abort => "aborted",
            NativeError::Canvas2dContext => "couldn't create 2d canvas context",
            NativeError::WebGlContext => "couldn't create webgl context",
            NativeError::WebGlProgram => "couldn't create webgl program",
            NativeError::WebGlCanvas => "couldn't get canvas from webgl context",
            NativeError::WebGlVersion1 => "expected webgl version 1",
            NativeError::WebGlVersion2 => "expected webgl version 2",
            NativeError::Window => "couldn't get window",
            NativeError::GlobalSelf => "couldn't get window or worker",
            NativeError::WindowWidth => "couldn't get window width",
            NativeError::WindowHeight => "couldn't get window height",
            NativeError::CanvasCreate => "Couldn't create canvas",
            NativeError::AttributeLocation(_optional_name) => "Couldn't get attribute location",
            NativeError::UniformLocation(_optional_name) => "Couldn't get uniform location",
            NativeError::UniformIndex(_optional_name) => "Couldn't get uniform index",
            NativeError::MipsPowerOf2 => "mipmapping requires that textures be power of 2",
            NativeError::NoExtension => "extension not found",
            NativeError::NoCreateBuffer => "couldn't create buffer",
            NativeError::NoCreateFrameBuffer => "couldn't create frame buffer",
            NativeError::NoCreateRenderBuffer => "couldn't create render buffer",
            NativeError::NoExistingBuffer => "no existing buffer",
            NativeError::MissingShaderProgram => "No shader program activated",
            NativeError::MissingRenderBuffer => "No render buffer",
            NativeError::MissingFrameBuffer => "No frame buffer",
            NativeError::NoCreateTexture => "unable to create texture",
            NativeError::MissingTexture => "couldn't get texture",
            NativeError::MissingTextureSampler(_optional_name) => "couldn't get texture sampler",
            NativeError::MissingBuffer => "couldn't get buffer",
            NativeError::VertexArrayMissing => "no such vertex array",
            NativeError::VertexArrayCreate => "unable to create vertex array",
            NativeError::UniformMatrixMustBeFloat => "uniform matrix must be floats",
            NativeError::UniformType => "wrong uniform type",
            NativeError::UniformSize => "uniform data is not large enough",
            NativeError::UniformBufferName => "couldn't get uniform block name",
            NativeError::UniformBufferParameter => "couldn't get uniform block parameter",
            NativeError::UniformBufferBlockIndexMissing(_optional_name) => "uniform buffer block index is missing",
            NativeError::UniformBufferMissing(_optional_name) => "uniform buffer is missing",
            NativeError::UniformBufferOffsetMissing(_optional_name) => {
                "uniform buffer offset is missing"
            }
            NativeError::UniformBufferTarget => {
                "buffer target must be UniformBuffer for uniform buffers"
            }
            NativeError::WebGlBufferSourceOneNonZero => {
                "webgl 1 only supports sub buffer uploads from 0"
            }
            NativeError::WebGl1TextureOffsetNonZero => {
                "webgl 1 only supports texture uploads from offset 0"
            }
            NativeError::WebGl1TextureArray2d => "webgl 1 doesn't support 2d texture arrays",
            NativeError::WebGl1Texture3d => "webgl 1 doesn't support 3d textures",
            NativeError::JsValueExpectedBool => "expected jsvalue to be a bool",
            NativeError::JsValueExpectedNumber => "expected jsvalue to be a number",
            NativeError::NoTextureTarget => {
                "texture target must be known (call assign before activate)"
            }
            NativeError::TextureCubeFaceNotCube => "texture cube face is set but not cube target",
            NativeError::TextureMissingCubeFace => "texture cube face missing for cube target",
            NativeError::Internal => "internal error",
            NativeError::FrameBuffer(_optional_desc) => "framebuffer error",
        }
    }
    pub fn to_string(self: &Self) -> String {
        match self {
            NativeError::AttributeLocation(optional_name) => match optional_name {
                None => self.default_str().to_string(),
                Some(name) => format!("couldn't get attribute location named {}", name.as_str()),
            },
            NativeError::UniformLocation(optional_name) => match optional_name {
                None => self.default_str().to_string(),
                Some(name) => format!("couldn't get uniform location named {}", name.as_str()),
            },
            NativeError::UniformIndex(optional_name) => match optional_name {
                None => self.default_str().to_string(),
                Some(name) => format!("couldn't get uniform index named {}", name.as_str()),
            },
            NativeError::UniformBufferBlockIndexMissing(optional_name) => match optional_name {
                None => self.default_str().to_string(),
                Some(name) => format!("couldn't get uniform buffer block index named {}", name.as_str()),
            },
            NativeError::UniformBufferMissing(optional_name) => match optional_name {
                None => self.default_str().to_string(),
                Some(name) => format!("couldn't get uniform buffer named {}", name.as_str()),
            },
            NativeError::UniformBufferOffsetMissing(optional_name) => match optional_name {
                None => self.default_str().to_string(),
                Some((block_name, uniform_name)) => format!(
                    "couldn't get offset for uniform named {} in buffer named {}",
                    uniform_name,
                    block_name.as_str()
                ),
            },
            NativeError::MissingTextureSampler(optional_name) => match optional_name {
                None => self.default_str().to_string(),
                Some(name) => format!("couldn't get texture sampler named {}", name.as_str()),
            },
            NativeError::FrameBuffer(optional_desc) => match optional_desc{
                None => self.default_str().to_string(),
                Some(desc) => format!("framebuffer error: {}", desc.as_str()),
            },
            _ => self.default_str().to_string(),
        }
    }
}

impl From<Error> for JsValue {
    fn from(err: Error) -> Self {
        err.to_js()
    }
}

impl From<NativeError> for Error {
    fn from(err: NativeError) -> Self {
        Error::Native(err)
    }
}

impl From<JsValue> for Error {
    fn from(err: JsValue) -> Self {
        Error::Js(err)
    }
}

impl From<js_sys::Error> for Error {
    fn from(err: js_sys::Error) -> Self {
        Error::Js(JsValue::from(err))
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::String(err)
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::String(String::from(err))
    }
}

#[cfg(feature = "serde")]
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::String(format!("{}", err))
    }
}

/* TODO: this doesn't work, but maybe it could!
 * idea is to consolidate str and String into one impl
impl From<Borrow<str>> for Error
{
    fn from(err: &str) -> Self {
        Error::String(String::from(err))
    }
}
*/
