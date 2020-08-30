use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WebGlVersion {
    One,
    Two,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum DataType {
    Byte = 0x1400,
    UnsignedByte = 0x1401,
    Short = 0x1402,
    UnsignedShort = 0x1403,
    Int = 0x1404,
    UnsignedInt = 0x1405, //using OES_element_index_uint
    Float = 0x1406,
    HalfFloat = 0x140B, //Webgl2 only
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum BufferTarget {
    ArrayBuffer = 0x8892,
    ElementArrayBuffer = 0x8893,
    //webgl 2 only
    UniformBuffer = 0x8A11,
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum BufferUsage {
    StreamDraw = 0x88E0,
    StaticDraw = 0x88E4,
    DynamicDraw = 0x88E8,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum BeginMode {
    Points = 0x0000,
    Lines = 0x0001,
    LineLoop = 0x0002,
    LineStrip = 0x0003,
    Triangles = 0x0004,
    TriangleStrip = 0x0005,
    TriangleFan = 0x0006,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum TextureParameterName {
    MagFilter = 0x2800,
    MinFilter = 0x2801,
    WrapS = 0x2802,
    WrapT = 0x2803,
    WrapR = 0x8072,
    MinLod = 0x813A,
    MaxLod = 0x813B,
    BaseLevel = 0x813C,
    MaxLevel = 0x813D,
    CompareMode = 0x884C,
    CompareFunc = 0x884D,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum TextureWrapTarget {
    S = 0x2802,
    T = 0x2803,
    R = 0x8072,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum TextureWrapMode {
    Repeat = 0x2901,
    ClampToEdge = 0x812F,
    MirroredRepeat = 0x8370,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum TextureMagFilter {
    Nearest = 0x2600,
    Linear = 0x2601,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum TextureMinFilter {
    Nearest = 0x2600,
    Linear = 0x2601,
    NearestMipMapNearest = 0x2700,
    LinearMipMapNearest = 0x2701,
    NearestMipMapLinear = 0x2702,
    LinearMipMapLinear = 0x2703,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum TextureTarget {
    Texture2d = 0x0DE1,
    Texture3d = 0x806F,
    Array2d = 0x8C1A,
    CubeMap = 0x8513,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum TextureCubeFace {
    PositiveX = 0x8515,
    NegativeX = 0x8516,
    PositiveY = 0x8517,
    NegativeY = 0x8518,
    PositiveZ = 0x8519,
    NegativeZ = 0x851A,
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum TextureQuery {
    //Not actually totally sure that these are queries - but looks like it?
    Array2d = 0x8C1D,
    Texture = 0x1702,
    Binding3d = 0x806A,
    BindingCubeMap = 0x8514,
    MaxCubeTextureSize = 0x851C,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum TextureUnit {
    Texture0 = 0x84C0,
    Texture1 = 0x84C1,
    Texture2 = 0x84C2,
    Texture3 = 0x84C3,
    Texture4 = 0x84C4,
    Texture5 = 0x84C5,
    Texture6 = 0x84C6,
    Texture7 = 0x84C7,
    Texture8 = 0x84C8,
    Texture9 = 0x84C9,
    Texture10 = 0x84CA,
    Texture11 = 0x84CB,
    Texture12 = 0x84CC,
    Texture13 = 0x84CD,
    Texture14 = 0x84CE,
    Texture15 = 0x84CF,
    Texture16 = 0x84D0,
    Texture17 = 0x84D1,
    Texture18 = 0x84D2,
    Texture19 = 0x84D3,
    Texture20 = 0x84D4,
    Texture21 = 0x84D5,
    Texture22 = 0x84D6,
    Texture23 = 0x84D7,
    Texture24 = 0x84D8,
    Texture25 = 0x84D9,
    Texture26 = 0x84DA,
    Texture27 = 0x84DB,
    Texture28 = 0x84DC,
    Texture29 = 0x84DD,
    Texture30 = 0x84DE,
    Texture31 = 0x84DF,
    ActiveTexture = 0x84E0,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum PixelFormat {
    //WebGL1 and 2
    Alpha = 0x1906,
    Rgb = 0x1907,
    Rgba = 0x1908,
    Luminance = 0x1909,
    LuminanceAlpha = 0x190A,

    //When using the WEBGL_depth_texture extension
    DepthComponent = 0x1902,
    DepthStencil = 0x84F9,

    //When using the SRGB extension
    //SrgbExt = 0x8C40, //- same as Srgb for webgl2
    SrgbAlphaExt = 0x8C42,

    //WebGL2 only
    R8 = 0x8229,
    Rg8 = 0x822B,
    R16f = 0x822D,
    R32f = 0x822E,
    RG16f = 0x822F,
    RG32f = 0x8230,
    R8i = 0x8231,
    R8ui = 0x8232,
    R16i = 0x8233,
    R16ui = 0x8234,
    R32i = 0x8235,
    R32ui = 0x8236,
    RG8i = 0x8237,
    RG8ui = 0x8238,
    RG16i = 0x8239,
    RG16ui = 0x823A,
    RG32i = 0x823B,
    RG32ui = 0x823C,
    Srgb = 0x8C40,
    Srgb8 = 0x8C41,
    Srgb8Alpha8 = 0x8C43,
    Rgba32f = 0x8814,
    Rgb32f = 0x8815,
    Rgba16f = 0x881A,
    Rgb16f = 0x881B,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum BufferMask {
    DepthBufferBit = 0x00000100,
    StencilBufferBit = 0x00000400,
    ColorBufferBit = 0x00004000,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum GlToggle {
    Blend = 0x0BE2,
    CullFace = 0x0B44,
    DepthTest = 0x0B71,
    Dither = 0x0BD0,
    PolygonOffsetFill = 0x8037,
    SampleAlphaToCoverage = 0x809E,
    SampleCoverage = 0x80A0,
    ScissorTest = 0x0C11,
    StencilTest = 0x0B90,
    RasterizerDiscard = 0x8C89,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum GlQuery {
    FragmentShader = 0x8B30,
    VertexShader = 0x8B31,
    MaxVertexAttribs = 0x8869,
    MaxVertexUniformVectors = 0x8DFB,
    MaxVaryingVectors = 0x8DFC,
    MaxCombinedTextureImageUnits = 0x8B4D,
    MaxVertexTextureImageUnits = 0x8B4C,
    MaxTextureImageUnits = 0x8872,
    MaxFragmentUniformVectors = 0x8DFD,
    MaxSamples = 0x8D57,
    ShadingLanguageVersion = 0x8B8C,
    CurrentProgram = 0x8B8D,
    BlendColor = 0x8005,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum CmpFunction {
    Never = 0x0200,
    Less = 0x0201,
    Equal = 0x0202,
    Lequal = 0x0203,
    Greater = 0x0204,
    NotEqual = 0x0205,
    Gequal = 0x0206,
    Always = 0x0207,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum BlendEquation {
    Add = 0x8006,
    Subtract = 0x800A,
    ReverseSubtract = 0x800B,
    Min = 0x8007,
    Max = 0x8008,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum BlendFactor {
    Zero = 0,
    One = 1,
    SrcColor = 0x0300,
    OneMinusSrcColor = 0x0301,
    DstColor = 0x0306,
    OneMinusDstColor = 0x0307,
    SrcAlpha = 0x0302,
    OneMinusSrcAlpha = 0x0303,
    DstAlpha = 0x0304,
    OneMinusDstAlpha = 0x0305,
    ConstantColor = 0x8001,
    OneMinusConstantColor = 0x8002,
    ConstantAlpha = 0x8003,
    OneMinusConstantAlpha = 0x8004,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum UniformBlockQuery {
    BindingPoint = 0x8A3F,
    DataSize = 0x8A40,
    ActiveUniforms = 0x8A42,
    ActiveUniformIndices = 0x8A43,
    ReferencedByVertexShader = 0x8A44,
    ReferencedByFragmentShader = 0x8A46,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum UniformBlockActiveQuery {
    Type = 0x8A37,
    Size = 0x8A38,
    BlockIndex = 0x8A3A,
    Offset = 0x8A3B,
    ArrayStride = 0x8A3C,
    MatrixStride = 0x8A3D,
    IsRowMajor = 0x8A3E,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum WebGlSpecific {
    UnpackFlipY = 0x9240,
    UnpackPremultiplyAlpha = 0x9241,
    ContextLost = 0x9242,
    UnpackColorspaceConversion = 0x9243,
    BrowserDefault = 0x9244,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum UniformDataType {
    FloatVec2 = 0x8B50,
    FloatVec3 = 0x8B51,
    FloatVec4 = 0x8B52,
    IntVec2 = 0x8B53,
    IntVec3 = 0x8B54,
    IntVec4 = 0x8B55,
    Bool = 0x8B56,
    BoolVec2 = 0x8B57,
    BoolVec3 = 0x8B58,
    BoolVec4 = 0x8B59,
    FloatMat2 = 0x8B5A,
    FloatMat3 = 0x8B5B,
    FloatMat4 = 0x8B5C,
    Sampler2d = 0x8B5E,
    SamplerCube = 0x8B60,

    //WebGL2 only
    Sampler3d = 0x8B5F,
    Sampler2dShadow = 0x8B62,
    SamplerCubeShadow = 0x8DC5,
    Sampler2dArray = 0x8DC1,
    Sampler2dArrayShadow = 0x8DC4,
    IntSampler2d = 0x8DCA,
    IntSampler3d = 0x8DCB,
    IntSamplerCube = 0x8DCC,
    IntSampler2dArray = 0x8DCF,
    UnsignedIntSampler2d = 0x8DD2,
    UnsignedIntSampler3d = 0x8DD3,
    UnsignedIntSamplerCube = 0x8DD4,
    UnsignedIntSampler2dArray = 0x8DD7,

    //WEBGL_depth_texture extension
    UnsignedInt24_8 = 0x84FA,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum ShaderQuery {
    DeleteStatus = 0x8B80,
    CompileStatus = 0x8B81,
    ShaderType = 0x8B4F,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum ShaderType {
    Fragment = 0x8B30,
    Vertex = 0x8B31,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum ProgramQuery {
    DeleteStatus = 0x8B80,
    LinkStatus = 0x8B82,
    ValidateStatus = 0x8B83,
    AttachedShaders = 0x8B85,
    ActiveUniforms = 0x8B86,
    ActiveAttributes = 0x8B89,
    TransformFeedbackBufferMode = 0x8C7F,
    TransformFeedbackVaryings = 0x8C83,
    ActiveUniformBlocks = 0x8A36,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum RenderBufferFormat {
    Rgba4 = 0x8056,
    Rgb4a1 = 0x8057,
    Rgb565 = 0x8D62,
    DepthComponent16 = 0x81A5,
    StencilIndex8 = 0x8D48,
    DepthStencil = 0x84F9,

    //WebGl2 only
    R8 = 0x8229,
    R8ui = 0x8232,
    R8i= 0x8231,
    R16ui= 0x8234,
    R16i= 0x8233,
    R32ui= 0x8236,
    R32i= 0x8235,
    Rg8= 0x822B,
    Rg8ui= 0x8238,
    Rg8i= 0x8237,
    Rg16ui= 0x823A,
    Rg16i= 0x8239,
    Rg32ui= 0x823C,
    Rg32i= 0x823B,
    Rgb8=0x8051,
    Rgba8= 0x8058,
    Rgb10a2= 0x8059,
    Rgba8ui= 0x8D7C,
    Rba8i= 0x8D8F,
    Rgb10a2ui = 0x906F,
    Rgba16ui= 0x8D76,
    Rgba16i= 0x8D89,
    Rgba32i= 0x8D82,
    Rgba32ui= 0x8D70,
    DepthComponent24= 0x81A6,
    DepthComponent32f= 0x8CAC,
    Depth24Stencil8= 0x88F0,
    Depth32fStencil8= 0x8CAD,
    //WEBGL_color_buffer_float extension
    Rgba32f = 0x8814,
    Rgb32f = 0x8815,
    //if *also* Webgl2
    R16f = 0x822D,
    Rg16f = 0x822F,
    Rgba16f = 0x881A,
    R32f = 0x822E,
    Rg32f = 0x8230,
    R11fG11fB10f = 0x8C3A,
    //sRGB Extension OR Webgl2
    Srgb8Alpha8 =0x8C43,

}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum ReadPixelFormat {
    Alpha = 0x1906,
    Rgb = 0x1907,
    Rgba = 0x1908,

    //WebGl2 only
    Red = 0x1903,
    Rg = 0x8227,
    RedInteger = 0x8D94,
    RgInteger = 0x8228,
    RgbInteger = 0x8D98,
    RgbaInteger = 0x8D99,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum ReadPixelDataType {
    UnsignedByte = 0x1401,
    UnsignedShort5_6_5 = 0x8363,
    UnsignedShort4_4_4_4 = 0x8033,
    UnsignedShort5_5_5_1 = 0x8034,
    Float = 0x1406,

    //WebGl2 only
    Byte = 0x1400,
    HalfFloat = 0x140B,
    Short = 0x1402,
    UnsignedShort = 0x1403,
    Int = 0x1404,
    UnsignedInt = 0x1405,
    UnsignedInt2_10_10_10Rev = 0x8368,
    UnsignedInt10f11f11fRev = 0x8C3B,
    UnsignedInt5_9_9_9Rev = 0x8C3E
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum BlitFilter {
    Nearest = 0x2600,
    Linear = 0x2601,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum FrameBufferTarget {
    FrameBuffer = 0x8D40,
    //webgl 2 only
    DrawFrameBuffer = 0x8CA9,
    ReadFrameBuffer = 0x8CA8,
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum FrameBufferAttachment {
    Color0 = 0x8CE0,
    Depth = 0x8D00,
    Stencil = 0x8D20,

    //only webgl2 or WEBGL_depth_texture extension
    DepthStencil = 0x821A,

    //only webgl2 or WEBGL_draw_buffers extension
    Color1 = 0x8CE1,
    Color2 = 0x8CE2,
    Color3 = 0x8CE3,
    Color4 = 0x8CE4,
    Color5 = 0x8CE5,
    Color6 = 0x8CE6,
    Color7 = 0x8CE7,
    Color8 = 0x8CE8,
    Color9 = 0x8CE9,
    Color10 = 0x8CEA,
    Color11 = 0x8CEB,
    Color12 = 0x8CEC,
    Color13 = 0x8CED,
    Color14 = 0x8CEE,
    Color15 = 0x8CEF,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum ReadBuffer {
    None = 0,
    Back = 0x0405,

    Color0 = 0x8CE0,
    Color1 = 0x8CE1,
    Color2 = 0x8CE2,
    Color3 = 0x8CE3,
    Color4 = 0x8CE4,
    Color5 = 0x8CE5,
    Color6 = 0x8CE6,
    Color7 = 0x8CE7,
    Color8 = 0x8CE8,
    Color9 = 0x8CE9,
    Color10 = 0x8CEA,
    Color11 = 0x8CEB,
    Color12 = 0x8CEC,
    Color13 = 0x8CED,
    Color14 = 0x8CEE,
    Color15 = 0x8CEF,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum DrawBuffer {
    None = 0,
    Back = 0x0405,

    Color0 = 0x8CE0,
    Color1 = 0x8CE1,
    Color2 = 0x8CE2,
    Color3 = 0x8CE3,
    Color4 = 0x8CE4,
    Color5 = 0x8CE5,
    Color6 = 0x8CE6,
    Color7 = 0x8CE7,
    Color8 = 0x8CE8,
    Color9 = 0x8CE9,
    Color10 = 0x8CEA,
    Color11 = 0x8CEB,
    Color12 = 0x8CEC,
    Color13 = 0x8CED,
    Color14 = 0x8CEE,
    Color15 = 0x8CEF,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum FrameBufferTextureTarget {
    Texture2d = 0x0DE1,
    CubeFacePositiveX = 0x8515,
    CubeFaceNegativeX = 0x8516,
    CubeFacePositiveY = 0x8517,
    CubeFaceNegativeY = 0x8518,
    CubeFacePositiveZ = 0x8519,
    CubeFaceNegativeZ = 0x851A,
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum CullFaceMode {
    Front = 0x0404,
    Back = 0x0405,
    FrontAndBack = 0x0408
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum FrameBufferStatus {
    Complete = 0x8CD5,
    IncompleteAttachment = 0x8CD6,
    IncompleteMissingAttachment = 0x8CD7,
    IncompleteDimensions = 0x8CD9,
    Unsupported = 0x8CDD,

    //only webgl2
    IncompleteMultisample = 0x8D56,
    Samples = 0x8CAB,

    //only OVR_multiview2
    IncompleteViewTargetsOvr = 0x9633,
}

impl TryFrom<u32> for FrameBufferStatus {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == (FrameBufferStatus::Complete as u32) {
            Ok(FrameBufferStatus::Complete)
        } else if value == (FrameBufferStatus::IncompleteAttachment as u32) {
            Ok(FrameBufferStatus::IncompleteAttachment)
        } else if value == (FrameBufferStatus::IncompleteMissingAttachment as u32) {
            Ok(FrameBufferStatus::IncompleteMissingAttachment)
        } else if value == (FrameBufferStatus::IncompleteDimensions as u32) {
            Ok(FrameBufferStatus::IncompleteDimensions)
        } else if value == (FrameBufferStatus::Unsupported as u32) {
            Ok(FrameBufferStatus::Unsupported)
        } else if value == (FrameBufferStatus::IncompleteMultisample as u32) {
            Ok(FrameBufferStatus::IncompleteMultisample)
        } else if value == (FrameBufferStatus::Samples as u32) {
            Ok(FrameBufferStatus::Samples)
        } else if value == (FrameBufferStatus::IncompleteViewTargetsOvr as u32) {
            Ok(FrameBufferStatus::IncompleteViewTargetsOvr)
        } else {
            Err("bad value for FrameBufferStatus")
        }
    }
}
