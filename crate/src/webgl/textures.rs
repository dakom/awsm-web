use super::{
    DataType, Id, PixelFormat, TextureCubeFace, TextureMagFilter, TextureMinFilter,
    TextureParameterName, TextureTarget, TextureUnit, TextureWrapMode, TextureWrapTarget,
    WebGlCommon, WebGlRenderer, WebGlSpecific,
    ProgramQuery,
    PixelDataFormat, PixelInternalFormat,
};
use crate::errors::{Error, NativeError};
use web_sys::{
    HtmlCanvasElement, HtmlImageElement, HtmlVideoElement, ImageBitmap, ImageData, WebGlTexture, WebGlUniformLocation
};
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext};
use std::collections::hash_map::Entry;

pub enum WebGlTextureSource<'a> {
    ArrayBufferView(&'a js_sys::Object, u32, u32, u32), //width, height, depth
    EmptyBufferView(u32, u32, u32), //width, height, depth
    ImageBitmap(&'a ImageBitmap),
    ImageData(&'a ImageData),
    ImageElement(&'a HtmlImageElement),
    CanvasElement(&'a HtmlCanvasElement),
    VideoElement(&'a HtmlVideoElement),
}

// SimpleTexutreOptions represents the typical use case
pub struct SimpleTextureOptions {
    pub flip_y: Option<bool>,
    pub premultiply_alpha: Option<bool>,
    pub wrap_s: Option<TextureWrapMode>,
    pub wrap_t: Option<TextureWrapMode>,
    pub wrap_r: Option<TextureWrapMode>,
    pub filter_min: Option<TextureMinFilter>,
    pub filter_mag: Option<TextureMagFilter>,
    pub pixel_format: PixelFormat,
    pub data_type: DataType,
    pub cube_face: Option<TextureCubeFace>,
}

impl Default for SimpleTextureOptions {
    fn default() -> Self {
        Self {
            flip_y: Some(true),
            premultiply_alpha: None,
            wrap_s: Some(TextureWrapMode::ClampToEdge),
            wrap_t: Some(TextureWrapMode::ClampToEdge),
            wrap_r: None,
            filter_min: Some(TextureMinFilter::Linear),
            filter_mag: Some(TextureMagFilter::Linear),
            pixel_format: PixelFormat::Rgb,
            data_type: DataType::UnsignedByte,
            cube_face: None,
        }
    }
}

pub struct TextureOptions {
    pub internal_format: PixelInternalFormat,
    pub data_format: PixelDataFormat,
    pub data_type: DataType,
    pub cube_face: Option<TextureCubeFace>,
}

pub trait PartialWebGlTextures {
    fn awsm_create_texture(&self) -> Result<WebGlTexture, Error>;
    fn awsm_delete_texture(&self, texture:&WebGlTexture);
    fn awsm_bind_texture(&self, bind_target: TextureTarget, texture:&WebGlTexture);

    fn awsm_release_texture_target(&self, bind_target: TextureTarget);
    //needed to accomodate framebuffer target too
    fn awsm_release_texture_target_raw(&self, bind_target: u32);

    fn awsm_texture_set_wrap(
        &self,
        bind_target: TextureTarget,
        wrap_target: TextureWrapTarget,
        wrap_mode: TextureWrapMode,
    );
    fn awsm_texture_set_min_filter(&self, bind_target: TextureTarget, filter: TextureMinFilter);
    fn awsm_texture_set_mag_filter(&self, bind_target: TextureTarget, filter: TextureMagFilter);
    fn awsm_texture_sources_can_mipmap(&self, srcs: &[&WebGlTextureSource]) -> Result<(), Error>;

    fn awsm_assign_simple_texture(
        &self,
        bind_target: TextureTarget,
        opts: &SimpleTextureOptions,
        src: &WebGlTextureSource,
        dest: &WebGlTexture,
    ) -> Result<(), Error>;

    fn awsm_assign_simple_texture_mips(
        &self,
        bind_target: TextureTarget,
        opts: &SimpleTextureOptions,
        srcs: &[&WebGlTextureSource],
        dest: &WebGlTexture,
    ) -> Result<(), Error>;

    fn awsm_simple_parameters(
        &self,
        bind_target: TextureTarget,
        opts: &SimpleTextureOptions,
        use_mips: bool,
    );

    fn awsm_assign_texture(
        &self,
        bind_target: TextureTarget,
        opts: &TextureOptions,
        set_parameters: Option<impl Fn(&Self) -> ()>,
        src: &WebGlTextureSource,
        dest: &WebGlTexture,
    ) -> Result<(), Error>;

    fn awsm_assign_texture_mips(
        &self,
        bind_target: TextureTarget,
        opts: &TextureOptions,
        set_parameters: Option<impl Fn(&Self) -> ()>,
        srcs: &[&WebGlTextureSource],
        dest: &WebGlTexture,
    ) -> Result<(), Error>;

    fn awsm_activate_texture_sampler_index( &self, sampler_index: u32,);

    fn _awsm_assign_texture(
        &self,
        bind_target: TextureTarget,
        mip_level: i32,
        opts: &TextureOptions,
        src: &WebGlTextureSource,
    ) -> Result<(), Error>;
}

macro_rules! impl_context {
    ($($type:ty { $($defs:tt)* })+) => {
        $(impl PartialWebGlTextures for $type {

            fn awsm_create_texture(&self) -> Result<WebGlTexture, Error> {
                self.create_texture().ok_or(Error::from(NativeError::NoCreateTexture))
            }
            fn awsm_delete_texture(&self, texture:&WebGlTexture) {
                self.delete_texture(Some(texture));
            }

            fn awsm_texture_set_wrap(&self, bind_target: TextureTarget, wrap_target:TextureWrapTarget, wrap_mode: TextureWrapMode) {
                self.tex_parameteri(bind_target as u32, wrap_target as u32, wrap_mode as i32);
            }
            fn awsm_texture_set_min_filter(&self, bind_target: TextureTarget, filter: TextureMinFilter) {
                self.tex_parameteri(bind_target as u32, TextureParameterName::MinFilter as u32, filter as i32);
            }

            fn awsm_texture_set_mag_filter(&self, bind_target: TextureTarget, filter: TextureMagFilter) {
                self.tex_parameteri(bind_target as u32, TextureParameterName::MagFilter as u32, filter as i32);
            }

            fn awsm_assign_simple_texture(&self, bind_target: TextureTarget, opts:&SimpleTextureOptions, src:&WebGlTextureSource, dest:&WebGlTexture) -> Result<(), Error> {

                let set_parameters = Some(|_:&$type| {
                    self.awsm_simple_parameters(bind_target, &opts, false);
                });

                self.awsm_assign_texture(bind_target, &get_texture_options_from_simple(&opts), set_parameters, &src, &dest)
            }

            fn awsm_assign_simple_texture_mips(&self, bind_target: TextureTarget, opts:&SimpleTextureOptions, srcs:&[&WebGlTextureSource], dest:&WebGlTexture) -> Result<(), Error> {

                self.awsm_texture_sources_can_mipmap(&srcs)?;
                let set_parameters = Some(|_:&$type| {
                    self.awsm_simple_parameters(bind_target, &opts, true);
                });

                self.awsm_assign_texture_mips(bind_target, &get_texture_options_from_simple(&opts), set_parameters, &srcs, &dest)
            }

            fn awsm_simple_parameters(&self, bind_target: TextureTarget, opts:&SimpleTextureOptions, use_mips: bool) {

                opts.flip_y.map(|flip_y| {
                    if flip_y {
                        self.pixel_storei(WebGlSpecific::UnpackFlipY as u32, 1);
                    } else {
                        self.pixel_storei(WebGlSpecific::UnpackFlipY as u32, 0);
                    }
                });


                opts.premultiply_alpha.map(|premultiply_alpha| {
                    if premultiply_alpha {
                        self.pixel_storei(WebGlSpecific::UnpackPremultiplyAlpha as u32, 1);
                    } else {
                        self.pixel_storei(WebGlSpecific::UnpackPremultiplyAlpha as u32, 0);
                    }
                });

                if use_mips {
                    self.generate_mipmap(bind_target as u32);
                } else {
                    opts.wrap_s.map(|wrap_s| {
                        self.awsm_texture_set_wrap(bind_target, TextureWrapTarget::S, wrap_s);
                    });

                    opts.wrap_t.map(|wrap_t| {
                        self.awsm_texture_set_wrap(bind_target, TextureWrapTarget::T, wrap_t);
                    });

                    opts.wrap_r.map(|wrap_r| {
                        self.awsm_texture_set_wrap(bind_target, TextureWrapTarget::R, wrap_r);
                    });

                    opts.filter_min.map(|filter_min| {
                        self.awsm_texture_set_min_filter(bind_target, filter_min);
                    });

                    opts.filter_mag.map(|filter_mag| {
                        self.awsm_texture_set_mag_filter(bind_target, filter_mag);
                    });
                }
            }

            fn awsm_assign_texture(&self, bind_target: TextureTarget, opts:&TextureOptions,set_parameters:Option<impl Fn(&Self) -> ()>, src:&WebGlTextureSource, dest:&WebGlTexture) -> Result<(), Error> {
                self.awsm_assign_texture_mips(bind_target, &opts, set_parameters, &[src], &dest)
            }

            //This is the only place where things _really_ happen
            fn awsm_assign_texture_mips(&self, bind_target: TextureTarget, opts:&TextureOptions, set_parameters:Option<impl Fn(&Self) -> ()>, srcs:&[&WebGlTextureSource], dest:&WebGlTexture) -> Result<(), Error> {
                

                self.awsm_bind_texture(bind_target, dest);

                set_parameters.map(|f| f(self));

                for (mip_level, src) in srcs.iter().enumerate() {
                    self._awsm_assign_texture(bind_target, mip_level as i32, &opts, &src)?;
                }

                Ok(())
            }

            fn awsm_bind_texture(&self, bind_target: TextureTarget, texture:&WebGlTexture) {
                self.bind_texture(bind_target as u32, Some(texture));
            }

            fn awsm_activate_texture_sampler_index(&self, sampler_index: u32) {
                self.active_texture((TextureUnit::Texture0 as u32) + sampler_index);
            }

            fn awsm_release_texture_target(&self, bind_target: TextureTarget) {
                self.bind_texture(bind_target as u32, None);
            }
            fn awsm_release_texture_target_raw(&self, bind_target: u32) {
                self.bind_texture(bind_target, None);
            }


            $($defs)*
        })+
    };
}

impl_context! {
    WebGlRenderingContext{
        fn awsm_texture_sources_can_mipmap(&self, srcs:&[&WebGlTextureSource]) -> Result<(), Error> {
            match srcs.iter().all(|&src| is_power_of_2(&src)) {
                true => Ok(()),
                false => Err(Error::from(NativeError::MipsPowerOf2))
            }
        }

        fn _awsm_assign_texture(&self, bind_target: TextureTarget, mip_level: i32, opts:&TextureOptions, src:&WebGlTextureSource) -> Result<(), Error> {

            let internal_format = opts.internal_format as i32;
            let data_format = opts.data_format as u32;
            let data_type = opts.data_type as u32;

            let bind_u32 = bind_target as u32;
            let cube_face_u32 = get_cube_face_u32(bind_target, opts.cube_face)?;

            match src {
                WebGlTextureSource::ArrayBufferView(buffer_view, width, height, _depth) => {
                    match bind_target {
                        TextureTarget::Texture2d => {
                            self.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                                bind_u32,
                                mip_level,
                                internal_format,
                                *width as i32,
                                *height as i32,
                                0,
                                data_format,
                                data_type,
                                Some(buffer_view)
                                ).map_err(|err| err.into())
                        },
                        TextureTarget::Texture3d => {
                            Err(Error::from(NativeError::WebGl1Texture3d))
                        },
                        TextureTarget::Array2d => {
                            Err(Error::from(NativeError::WebGl1TextureArray2d))
                        },
                        TextureTarget::CubeMap => {
                            self.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                                cube_face_u32,
                                mip_level,
                                internal_format,
                                *width as i32,
                                *height as i32,
                                0,
                                data_format,
                                data_type,
                                Some(buffer_view)
                                ).map_err(|err| err.into())
                        }
                    }
                },
                WebGlTextureSource::EmptyBufferView(width, height, _depth) => {
                    match bind_target {
                        TextureTarget::Texture2d => {
                            self.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                                bind_u32,
                                mip_level,
                                internal_format,
                                *width as i32,
                                *height as i32,
                                0,
                                data_format,
                                data_type,
                                None, 
                                ).map_err(|err| err.into())
                        },
                        TextureTarget::Texture3d => {
                            Err(Error::from(NativeError::WebGl1Texture3d))
                        },
                        TextureTarget::Array2d => {
                            Err(Error::from(NativeError::WebGl1TextureArray2d))
                        },
                        TextureTarget::CubeMap => {
                            self.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                                cube_face_u32,
                                mip_level,
                                internal_format,
                                *width as i32,
                                *height as i32,
                                0,
                                data_format,
                                data_type,
                                None, 
                                ).map_err(|err| err.into())
                        }
                    }
                },
                WebGlTextureSource::ImageBitmap(bmp) => {
                    match bind_target {
                        TextureTarget::Texture2d => {
                            self.tex_image_2d_with_u32_and_u32_and_image_bitmap(
                                bind_u32,
                                mip_level,
                                internal_format,
                                data_format,
                                data_type,
                                bmp
                                ).map_err(|err| err.into())
                        },
                        TextureTarget::Texture3d => {
                            Err(Error::from(NativeError::WebGl1Texture3d))
                        },
                        TextureTarget::Array2d => {
                            Err(Error::from(NativeError::WebGl1TextureArray2d))
                        },
                        TextureTarget::CubeMap => {
                            self.tex_image_2d_with_u32_and_u32_and_image_bitmap(
                                cube_face_u32,
                                mip_level,
                                internal_format,
                                data_format,
                                data_type,
                                bmp
                                ).map_err(|err| err.into())
                        }
                    }
                },
                WebGlTextureSource::ImageData(data) => {
                    match bind_target {
                        TextureTarget::Texture2d => {
                            self.tex_image_2d_with_u32_and_u32_and_image_data(
                                bind_u32,
                                mip_level,
                                internal_format,
                                data_format,
                                data_type,
                                data
                                ).map_err(|err| err.into())
                        },
                        TextureTarget::Texture3d => {
                            Err(Error::from(NativeError::WebGl1Texture3d))
                        },
                        TextureTarget::Array2d => {
                            Err(Error::from(NativeError::WebGl1TextureArray2d))
                        },
                        TextureTarget::CubeMap => {
                            self.tex_image_2d_with_u32_and_u32_and_image_data(
                                cube_face_u32,
                                mip_level,
                                internal_format,
                                data_format,
                                data_type,
                                data
                                ).map_err(|err| err.into())
                        }
                    }
                },
                WebGlTextureSource::ImageElement(img) => {
                    match bind_target {
                        TextureTarget::Texture2d => {
                            self.tex_image_2d_with_u32_and_u32_and_image(
                                bind_u32,
                                mip_level,
                                internal_format,
                                data_format,
                                data_type,
                                img
                            ).map_err(|err| err.into())
                        },
                        TextureTarget::Texture3d => {
                            Err(Error::from(NativeError::WebGl1Texture3d))
                        },
                        TextureTarget::Array2d => {
                            Err(Error::from(NativeError::WebGl1TextureArray2d))
                        },
                        TextureTarget::CubeMap => {
                            self.tex_image_2d_with_u32_and_u32_and_image(
                                cube_face_u32,
                                mip_level,
                                internal_format,
                                data_format,
                                data_type,
                                img
                            ).map_err(|err| err.into())
                        }
                    }
                },
                WebGlTextureSource::CanvasElement(canvas) => {
                    match bind_target {
                        TextureTarget::Texture2d => {
                            self.tex_image_2d_with_u32_and_u32_and_canvas(
                                bind_u32,
                                mip_level,
                                internal_format,
                                data_format,
                                data_type,
                                canvas
                            ).map_err(|err| err.into())
                        },
                        TextureTarget::Texture3d => {
                            Err(Error::from(NativeError::WebGl1Texture3d))
                        },
                        TextureTarget::Array2d => {
                            Err(Error::from(NativeError::WebGl1TextureArray2d))
                        },
                        TextureTarget::CubeMap => {
                            self.tex_image_2d_with_u32_and_u32_and_canvas(
                                cube_face_u32,
                                mip_level,
                                internal_format,
                                data_format,
                                data_type,
                                canvas
                            ).map_err(|err| err.into())
                        }
                    }
                },
                WebGlTextureSource::VideoElement(video) => {
                    match bind_target {
                        TextureTarget::Texture2d => {
                            self.tex_image_2d_with_u32_and_u32_and_video(
                                bind_u32,
                                mip_level,
                                internal_format,
                                data_format,
                                data_type,
                                video
                            ).map_err(|err| err.into())
                        },
                        TextureTarget::Texture3d => {
                            Err(Error::from(NativeError::WebGl1Texture3d))
                        },
                        TextureTarget::Array2d => {
                            Err(Error::from(NativeError::WebGl1TextureArray2d))
                        },
                        TextureTarget::CubeMap => {
                            self.tex_image_2d_with_u32_and_u32_and_video(
                                cube_face_u32,
                                mip_level,
                                internal_format,
                                data_format,
                                data_type,
                                video
                            ).map_err(|err| err.into())
                        }
                    }
                },

            }
        }
    }

    WebGl2RenderingContext{

        fn _awsm_assign_texture(&self, bind_target: TextureTarget, mip_level: i32, opts:&TextureOptions, src:&WebGlTextureSource) -> Result<(), Error> {

            let internal_format = opts.internal_format as i32;
            let data_format = opts.data_format as u32;
            let data_type = opts.data_type as u32;

            let bind_u32 = bind_target as u32;
            let cube_face_u32 = get_cube_face_u32(bind_target, opts.cube_face)?;

            match src {
                WebGlTextureSource::ArrayBufferView(buffer_view, width, height, depth) => {
                    match bind_target {
                        TextureTarget::Texture2d => {
                            self.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                                bind_u32,
                                mip_level,
                                internal_format,
                                *width as i32,
                                *height as i32,
                                0,
                                data_format,
                                data_type,
                                Some(buffer_view)
                                ).map_err(|err| err.into())
                        },
                        TextureTarget::Texture3d => {
                            self.tex_image_3d_with_opt_array_buffer_view(
                                bind_u32,
                                mip_level,
                                internal_format,
                                *width as i32,
                                *height as i32,
                                *depth as i32,
                                0,
                                data_format,
                                data_type,
                                Some(buffer_view)
                                ).map_err(|err| err.into())
                        },
                        TextureTarget::Array2d => {
                            Err("TODO".into())
                        },
                        TextureTarget::CubeMap => {
                            self.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                                cube_face_u32,
                                mip_level,
                                internal_format,
                                *width as i32,
                                *height as i32,
                                0,
                                data_format,
                                data_type,
                                Some(buffer_view)
                                ).map_err(|err| err.into())
                        }
                    }
                },
                WebGlTextureSource::EmptyBufferView(width, height, depth) => {
                    match bind_target {
                        TextureTarget::Texture2d => {
                            self.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                                bind_u32,
                                mip_level,
                                internal_format,
                                *width as i32,
                                *height as i32,
                                0,
                                data_format,
                                data_type,
                                None, 
                                ).map_err(|err| err.into())
                        },
                        TextureTarget::Texture3d => {
                            self.tex_image_3d_with_opt_array_buffer_view(
                                bind_u32,
                                mip_level,
                                internal_format,
                                *width as i32,
                                *height as i32,
                                *depth as i32,
                                0,
                                data_format,
                                data_type,
                                None, 
                                ).map_err(|err| err.into())
                        },
                        TextureTarget::Array2d => {
                            Err("TODO".into())
                        },
                        TextureTarget::CubeMap => {
                            self.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                                cube_face_u32,
                                mip_level,
                                internal_format,
                                *width as i32,
                                *height as i32,
                                0,
                                data_format,
                                data_type,
                                None, 
                                ).map_err(|err| err.into())
                        }
                    }
                },
                WebGlTextureSource::ImageBitmap(bmp) => {
                    match bind_target {
                        TextureTarget::Texture2d => {
                            self.tex_image_2d_with_u32_and_u32_and_image_bitmap(
                                bind_u32,
                                mip_level,
                                internal_format,
                                data_format,
                                data_type,
                                bmp
                                ).map_err(|err| err.into())
                        },
                        TextureTarget::Texture3d => {
                            Err("TODO".into())
                        },
                        TextureTarget::Array2d => {
                            Err("TODO".into())
                        },
                        TextureTarget::CubeMap => {
                            self.tex_image_2d_with_u32_and_u32_and_image_bitmap(
                                cube_face_u32,
                                mip_level,
                                internal_format,
                                data_format,
                                data_type,
                                bmp
                                ).map_err(|err| err.into())
                        }
                    }
                },
                WebGlTextureSource::ImageData(data) => {
                    match bind_target {
                        TextureTarget::Texture2d => {
                            self.tex_image_2d_with_u32_and_u32_and_image_data(
                                bind_u32,
                                mip_level,
                                internal_format,
                                data_format,
                                data_type,
                                data
                                ).map_err(|err| err.into())
                        },
                        TextureTarget::Texture3d => {
                            Err("TODO".into())
                        },
                        TextureTarget::Array2d => {
                            Err("TODO".into())
                        },
                        TextureTarget::CubeMap => {
                            self.tex_image_2d_with_u32_and_u32_and_image_data(
                                cube_face_u32,
                                mip_level,
                                internal_format,
                                data_format,
                                data_type,
                                data
                                ).map_err(|err| err.into())
                        }
                    }
                },
                WebGlTextureSource::ImageElement(img) => {
                    match bind_target {
                        TextureTarget::Texture2d => {
                            self.tex_image_2d_with_u32_and_u32_and_html_image_element(
                                bind_u32,
                                mip_level,
                                internal_format,
                                data_format,
                                data_type,
                                img
                            ).map_err(|err| err.into())
                        },
                        TextureTarget::Texture3d => {
                            Err("TODO".into())
                        },
                        TextureTarget::Array2d => {
                            Err("TODO".into())
                        },
                        TextureTarget::CubeMap => {
                            self.tex_image_2d_with_u32_and_u32_and_html_image_element(
                                cube_face_u32,
                                mip_level,
                                internal_format,
                                data_format,
                                data_type,
                                img
                            ).map_err(|err| err.into())
                        }
                    }
                },
                WebGlTextureSource::CanvasElement(canvas) => {
                    match bind_target {
                        TextureTarget::Texture2d => {
                            self.tex_image_2d_with_u32_and_u32_and_html_canvas_element(
                                bind_u32,
                                mip_level,
                                internal_format,
                                data_format,
                                data_type,
                                canvas
                            ).map_err(|err| err.into())
                        },
                        TextureTarget::Texture3d => {
                            Err("TODO".into())
                        },
                        TextureTarget::Array2d => {
                            Err("TODO".into())
                        },
                        TextureTarget::CubeMap => {
                            self.tex_image_2d_with_u32_and_u32_and_html_canvas_element(
                                cube_face_u32,
                                mip_level,
                                internal_format,
                                data_format,
                                data_type,
                                canvas
                            ).map_err(|err| err.into())
                        }
                    }
                },
                WebGlTextureSource::VideoElement(video) => {
                    match bind_target {
                        TextureTarget::Texture2d => {
                            self.tex_image_2d_with_u32_and_u32_and_html_video_element(
                                bind_u32,
                                mip_level,
                                internal_format,
                                data_format,
                                data_type,
                                video
                            ).map_err(|err| err.into())

                        },
                        TextureTarget::Texture3d => {
                            Err("TODO".into())
                        },
                        TextureTarget::Array2d => {
                            Err("TODO".into())
                        },
                        TextureTarget::CubeMap => {
                            self.tex_image_2d_with_u32_and_u32_and_html_video_element(
                                cube_face_u32,
                                mip_level,
                                internal_format,
                                data_format,
                                data_type,
                                video
                            ).map_err(|err| err.into())
                        }
                    }
                },

            }
        }

        fn awsm_texture_sources_can_mipmap(&self, _:&[&WebGlTextureSource]) -> Result<(), Error> {
            Ok(())
        }
    }
}

/// get the width, height, and depth of the texture in pixels
pub fn get_texture_size(src: &WebGlTextureSource) -> (u32, u32, u32) {
    match src {
        WebGlTextureSource::ArrayBufferView(_buffer, width, height, depth) => {
            (*width, *height, *depth)
        },
        WebGlTextureSource::EmptyBufferView(width, height, depth) => {
            (*width, *height, *depth)
        }
        WebGlTextureSource::ImageBitmap(bmp) => (bmp.width(), bmp.height(), 0),
        WebGlTextureSource::ImageData(data) => (data.width(), data.height(), 0),
        WebGlTextureSource::ImageElement(img) => (img.width(), img.height(), 0),
        WebGlTextureSource::CanvasElement(canvas) => (canvas.width(), canvas.height(), 0),
        WebGlTextureSource::VideoElement(video) => (video.width(), video.height(), 0),
    }
}

/// check if the texture's width, height, and depth are all power of 2
pub fn is_power_of_2(src: &WebGlTextureSource) -> bool {
    let (width, height, depth) = get_texture_size(&src);
    is_power_of_2_val(width) && is_power_of_2_val(height) && is_power_of_2_val(depth)
}

fn get_texture_options_from_simple(opts: &SimpleTextureOptions) -> TextureOptions {
    TextureOptions {
        internal_format: opts.pixel_format.into(),
        data_format: opts.pixel_format.into(),
        data_type: opts.data_type,
        cube_face: opts.cube_face,
    }
}

fn is_power_of_2_val(val: u32) -> bool {
    val == 0 || (val & (val - 1) == 0)
}

//WebGlRenderer Impl
pub(super) struct TextureInfo {
    texture: WebGlTexture,
    bind_target: Option<TextureTarget>,
}

impl<G: WebGlCommon> WebGlRenderer<G> {
    pub fn create_texture(&mut self) -> Result<Id, Error> {
        let texture = self.gl.awsm_create_texture()?;

        let id = self.texture_lookup.insert(TextureInfo {
            texture,
            bind_target: None,
        });

        Ok(id)
    }

    pub fn delete_texture(&mut self, id:Id) -> Result<(), Error> {


        let info = self
            .texture_lookup
            .get(id)
            .ok_or(Error::from(NativeError::MissingTexture))?;


        let mut release_targets:Vec<u32> = Vec::new();

        release_targets.into_iter().for_each(|target| {
            self.gl.awsm_release_texture_target_raw(target);
        });

        self.gl.awsm_delete_texture(&info.texture);
        self.texture_lookup.remove(id);

        Ok(())
    }

    pub fn release_texture_target(&mut self, bind_target: TextureTarget) {
        self.gl.awsm_release_texture_target(bind_target);
    }

    pub fn get_texture_sampler_names(&self, program_id: Id) -> Result<Vec<String>, Error> {

        let program_info = self
            .program_lookup
            .get(program_id)
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;

        let mut texture_samplers: Vec<String> = Vec::new();
        let max: u32 = 
            self
                .gl
                .awsm_get_program_parameter_u32(&program_info.program, ProgramQuery::ActiveUniforms)
                .unwrap_or(0);

        if max <= 0 {
            return Ok(texture_samplers);
        }

        for i in 0..max {

            let (name, type_, size) = 
                self
                    .gl
                    .awsm_get_active_uniform(&program_info.program, i)
                    .map(|info| (info.name(), info.type_(), info.size()))?;
            


            for name in super::shader::parse_uniform_names(&name, size as usize) {
                match type_ {
                    //Just the sampler types from UniformDataType
                    //matching on enums with casting seems to be a pain point
                    //(or I missed something in Rust)
                    0x8B5E | 0x8B60 | 0x8B5F | 0x8B62 | 0x8DC5 | 0x8DC1 | 0x8DC4 | 0x8DCA
                    | 0x8DCB | 0x8DCC | 0x8DCF | 0x8DD2 | 0x8DD3 | 0x8DD4 | 0x8DD7 => {
                        texture_samplers.push(name)
                    }
                    _ => {}
                };
            }
        }

        Ok(texture_samplers)
    }

    pub fn cache_sampler_index_name(&mut self, program_id: Id, name:&str) -> Result<u32, Error> {
        let program_info = self
            .program_lookup
            .get_mut(program_id)
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;

        let index = {
            program_info.texture_sampler_slot_lookup.len() as u32
        };

        let entry = program_info.texture_sampler_slot_lookup.entry(name.to_string());

        match entry {
            Entry::Occupied(entry) => {
                Ok(entry.get().clone())
            }
            Entry::Vacant(entry) => {
                #[cfg(feature = "debug_log")]
                log::info!("caching sampler index for [{}]: {}", &name, index);

                entry.insert(index);

                Ok(index)
            }
        }
    }

    pub fn get_sampler_index_name(&mut self, name: &str) -> Result<u32, Error> {
        let program_id = self
            .current_program_id
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;

        self.cache_sampler_index_name(program_id, name)
    }

    pub fn get_texture(&self, texture_id:Id) -> Result<&WebGlTexture, Error> {
        self
            .texture_lookup
            .get(texture_id)
            .ok_or(Error::from(NativeError::MissingTexture))
            .map(|info| &info.texture)
    }

    pub fn assign_simple_texture(
        &mut self,
        texture_id: Id,
        bind_target: TextureTarget,
        opts: &SimpleTextureOptions,
        src: &WebGlTextureSource,
    ) -> Result<(), Error> {
        let texture_info = self
            .texture_lookup
            .get_mut(texture_id)
            .ok_or(Error::from(NativeError::MissingTexture))?;

        texture_info.bind_target = Some(bind_target);

        self.gl
            .awsm_assign_simple_texture(bind_target, &opts, &src, &texture_info.texture)
    }

    pub fn assign_simple_texture_mips(
        &mut self,
        texture_id: Id,
        bind_target: TextureTarget,
        opts: &SimpleTextureOptions,
        srcs: &[&WebGlTextureSource],
    ) -> Result<(), Error> {
        let texture_info = self
            .texture_lookup
            .get_mut(texture_id)
            .ok_or(Error::from(NativeError::MissingTexture))?;

        texture_info.bind_target = Some(bind_target);

        self.gl
            .awsm_assign_simple_texture_mips(bind_target, &opts, &srcs, &texture_info.texture)
    }

    pub fn assign_texture(
        &mut self,
        texture_id: Id,
        bind_target: TextureTarget,
        opts: &TextureOptions,
        set_parameters: Option<impl Fn(&G) -> ()>,
        src: &WebGlTextureSource,
    ) -> Result<(), Error> {
        let texture_info = self
            .texture_lookup
            .get_mut(texture_id)
            .ok_or(Error::from(NativeError::MissingTexture))?;

        texture_info.bind_target = Some(bind_target);

        self.gl.awsm_assign_texture(
            bind_target,
            &opts,
            set_parameters,
            &src,
            &texture_info.texture,
        )
    }

    pub fn assign_texture_mips(
        &mut self,
        texture_id: Id,
        bind_target: TextureTarget,
        opts: &TextureOptions,
        set_parameters: Option<impl Fn(&G) -> ()>,
        srcs: &[&WebGlTextureSource],
    ) -> Result<(), Error> {
        let texture_info = self
            .texture_lookup
            .get_mut(texture_id)
            .ok_or(Error::from(NativeError::MissingTexture))?;

        texture_info.bind_target = Some(bind_target);

        self.gl.awsm_assign_texture_mips(
            bind_target,
            &opts,
            set_parameters,
            &srcs,
            &texture_info.texture,
        )
    }

    pub fn activate_texture_sampler_name(
        &mut self,
        texture_id: Id,
        sampler_name: &str,
    ) -> Result<(), Error> {


        let sampler_loc = self.get_uniform_location_name(sampler_name)?;
        //Will assign the slot if necessary too
        let sampler_index = self.get_sampler_index_name(sampler_name)?;
        self.activate_texture_sampler_index(texture_id, sampler_loc, sampler_index)?;
        Ok(())
    }

    pub fn activate_texture_sampler_index(
        &mut self,
        texture_id: Id,
        sampler_loc: WebGlUniformLocation,
        sampler_index: u32,
    ) -> Result<(), Error> {

        let texture_info = self
            .texture_lookup
            .get(texture_id)
            .ok_or(Error::from(NativeError::MissingTexture))?;

        let bind_target = texture_info.bind_target.ok_or(Error::from(NativeError::NoTextureTarget))?;

        self.gl.awsm_uniform1i(&sampler_loc, sampler_index as i32);
        self.gl.awsm_activate_texture_sampler_index(sampler_index);
        self.gl.awsm_bind_texture(bind_target, &texture_info.texture);

        Ok(())
    }
}

fn get_cube_face_u32(
    bind_target: TextureTarget,
    cube_face: Option<TextureCubeFace>,
) -> Result<u32, Error> {
    match cube_face {
        Some(cube_face) => {
            if bind_target != TextureTarget::CubeMap {
                Err(Error::from(NativeError::TextureCubeFaceNotCube))
            } else {
                Ok(cube_face as u32)
            }
        }
        None => {
            if bind_target == TextureTarget::CubeMap {
                Err(Error::from(NativeError::TextureMissingCubeFace))
            } else {
                Ok(0u32)
            }
        }
    }
}
