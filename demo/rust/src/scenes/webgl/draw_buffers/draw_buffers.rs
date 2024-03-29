use crate::scenes::webgl::common::*;
use crate::start_webgl;
use awsm_web::webgl::{BeginMode, BufferMask, Id, WebGlCommon, WebGlRenderer,
    PixelFormat, SimpleTextureOptions, TextureTarget,
    WebGlTextureSource,
    RenderBufferFormat,
    FrameBufferTarget,
    FrameBufferAttachment,
    FrameBufferTextureTarget,
    ReadPixelFormat,
    ReadPixelDataType,
    WebGlVersion,
    DrawBuffer,
    ShaderType
};
use nalgebra::{Matrix4, Point2, Vector3};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement, Window, MouseEvent};
use wasm_bindgen::JsCast;
use gloo_events::EventListener;

struct State {
    //mutable for each tick
    pub positions: Option<Vec<Point2<f64>>>,
    pub picker: Option<FrameBufferPicker>,
    pub area: Area,
    pub camera_width: f64,
    pub camera_height: f64,
    pub program_id: Option<Id>,
    pub texture_program_id: Option<Id>,
}

impl State {
    pub fn new() -> Self {
        Self {
            positions: None,
            picker: None,
            area: Area::new(100.0, 100.0),
            camera_width: 0.0,
            camera_height: 0.0,
            program_id: None,
            texture_program_id: None,
        }
    }
}

pub fn start(
    window: Window,
    document: Document,
    body: HtmlElement,
    version: WebGlVersion,
) -> Result<(), JsValue> {
    let state = Rc::new(RefCell::new(State::new()));

    let document_clone = document.clone();
    let body_clone = body.clone();
    start_webgl!(
        version,
        window,
        document,
        body,
        {
            let state = Rc::clone(&state);


            move |webgl_renderer, on_ready| {

                //setup renderer buffers 
                {
                    let mut webgl_renderer = webgl_renderer.borrow_mut();

                    if let Ok(webgl_renderer) = webgl_renderer.as_webgl1() {
                        webgl_renderer
                            .register_extension_draw_buffers()
                            .map_err(|err| JsValue::from_str(err.to_string().as_ref()))?;
                    }

                    let program_id = {
                        if webgl_renderer.version == WebGlVersion::One {
                            let shaders = vec![
                                webgl_renderer.compile_shader(include_str!("shaders/draw-buffers-webgl1-vertex.glsl"), ShaderType::Vertex).unwrap(),
                                webgl_renderer.compile_shader(include_str!("shaders/draw-buffers-webgl1-fragment.glsl"), ShaderType::Fragment).unwrap(),
                            ];
                            webgl_renderer.compile_program(&shaders)
                        } else {
                            let shaders = vec![
                                webgl_renderer.compile_shader(include_str!("shaders/draw-buffers-webgl2-vertex.glsl"), ShaderType::Vertex).unwrap(),
                                webgl_renderer.compile_shader(include_str!("shaders/draw-buffers-webgl2-fragment.glsl"), ShaderType::Fragment).unwrap(),
                            ];
                            webgl_renderer.compile_program(&shaders)
                        }
                    }?;

                    state.borrow_mut().program_id = Some(program_id);
                    
                    let shaders = vec![
                        webgl_renderer.compile_shader(include_str!("shaders/draw-buffers-texture-vertex.glsl"), ShaderType::Vertex).unwrap(),
                        webgl_renderer.compile_shader(include_str!("shaders/draw-buffers-texture-fragment.glsl"), ShaderType::Fragment).unwrap(),
                    ];
                    let texture_program_id = webgl_renderer.compile_program(&shaders)?;

                    state.borrow_mut().texture_program_id = Some(texture_program_id);

                    let _buffer_id = create_and_assign_unit_quad_buffer(&mut webgl_renderer)?;
                }

                //start click listener
                let click_handler = {
                    let webgl_renderer = Rc::clone(&webgl_renderer); 
                    let state = Rc::clone(&state); 
                    move |event: &web_sys::Event| {
                        let webgl_renderer = Rc::clone(&webgl_renderer); 
                        let state = Rc::clone(&state); 
                        let event:&MouseEvent = event.dyn_ref().unwrap_throw();
                        update_click_state(webgl_renderer, state, event.client_x() as f64, event.client_y() as f64);
                    }
                };

                EventListener::new(&webgl_renderer.borrow().canvas, "click", click_handler).forget();

                //decoration
                let item: HtmlElement = document_clone.create_element("div")?.dyn_into()?;
                item.set_class_name("button demo-button nohover");
                item.set_text_content(Some("select a square"));
                body_clone.append_child(&item)?;

                //ready
                on_ready();
                Ok(())
            }
        },
        {
            let state = Rc::clone(&state);
            move |width: u32, height: u32| {
                let mut state = state.borrow_mut();
                state.resize(width.into(), height.into());
            }
        },
        {
            let state = Rc::clone(&state);

            move |time, webgl_renderer| {
                {
                    let mut state = state.borrow_mut();
                    state.update(time);
                }

                let mut state = state.borrow_mut();

                let State {
                    positions,
                    picker,
                    area,
                    camera_width,
                    camera_height,
                    program_id,
                    texture_program_id,
                    ..
                } = &mut *state;

                webgl_renderer
                    .activate_program(program_id.unwrap())
                    .unwrap();

                //Build our matrices (must cast to f32)
                let scaling_mat = Matrix4::new_nonuniform_scaling(&Vector3::new(
                    area.width as f32,
                    area.height as f32,
                    0.0,
                ));
                let full_scaling_mat = Matrix4::new_nonuniform_scaling(&Vector3::new(
                    *camera_width as f32,
                    *camera_height as f32,
                    0.0,
                ));
                let camera_mat = Matrix4::new_orthographic(
                    0.0,
                    *camera_width as f32,
                    0.0,
                    *camera_height as f32,
                    0.0,
                    1.0,
                );


                if picker.is_none() {
                    *picker = Some(FrameBufferPicker::new(webgl_renderer, *camera_width as u32, *camera_height as u32).unwrap());
                }

                let picker = picker.as_ref().unwrap();

                //write to the textures
                picker.bind(webgl_renderer, picker.write_framebuffer_id).unwrap();
                webgl_renderer.clear(&[ BufferMask::ColorBufferBit, BufferMask::DepthBufferBit, ]);
                positions.as_ref().map(|positions| {
                    draw_positions(webgl_renderer, &camera_mat, &scaling_mat, positions.as_ref());
                });
                picker.release(webgl_renderer);

                //show the visible texture on the screen
                webgl_renderer
                    .activate_program(texture_program_id.unwrap())
                    .unwrap();
               
                //could easily show the other texture by changing the id 
                //selection would be the same because that comes from hidden buffer (which is bound to the hidden texture)
                blit(webgl_renderer, &camera_mat, &full_scaling_mat, picker.visible_texture_id);
            }
        }
    )
}

fn update_click_state<T: WebGlCommon> (renderer:Rc<RefCell<WebGlRenderer<T>>>, state:Rc<RefCell<State>>, client_x: f64, client_y: f64) {
    if let Some(picker) = &state.borrow().picker {
        let color = picker.get_color(&mut renderer.borrow_mut(), client_x, client_y).unwrap();

        let result = {
            if color.r == 1.0 {
                "left"
            } else if color.g == 1.0 {
                "middle"
            } else if color.b == 1.0 {
                "right"
            } else {
                "none"
            }
        };

        let window = web_sys::window().unwrap();

        window.alert_with_message(&format!("{} selected!", result)).unwrap();

    }
}

pub fn draw_positions<T: WebGlCommon> (renderer:&mut WebGlRenderer<T>, camera_mat: &Matrix4<f32>, scaling_mat: &Matrix4<f32>, positions: &Vec<Point2<f64>>) {

    for (index, pos) in positions.iter().enumerate() {
        let model_mat =
            Matrix4::new_translation(&Vector3::new(pos.x as f32, pos.y as f32, 0.0));
        let mvp_mat = camera_mat * model_mat;

        //Upload them to the GPU
        renderer
            .upload_uniform_mat_4_name("u_size", &scaling_mat.as_slice())
            .unwrap();
        renderer
            .upload_uniform_mat_4_name("u_modelViewProjection", &mvp_mat.as_slice())
            .unwrap();

        renderer
            .upload_uniform_mat_4_name("u_modelViewProjection", &mvp_mat.as_slice())
            .unwrap();
       
        let color_hidden = match index {
            0 => Color::new(1.0, 0.0, 0.0, 1.0),
            1 => Color::new(0.0, 1.0, 0.0, 1.0),
            2 => Color::new(0.0, 0.0, 1.0, 1.0),
            _ => Color::new(0.0, 0.0, 0.0, 1.0)
        };

        let color_visible = Color::new(1.0, 0.0, 0.0, 1.0);

        let color_values = color_hidden.to_vec_f32();
        renderer
            .upload_uniform_fvec_4_name("u_color_hidden", &color_values)
            .unwrap();

        let color_values = color_visible.to_vec_f32();
        renderer
            .upload_uniform_fvec_4_name("u_color_visible", &color_values)
            .unwrap();

        //draw!
        renderer.draw_arrays(BeginMode::TriangleStrip, 0, 4);
    }
}
pub fn blit<T: WebGlCommon> (renderer:&mut WebGlRenderer<T>, camera_mat: &Matrix4<f32>, scaling_mat: &Matrix4<f32>, texture_id: Id) {

        //Upload matrices to the GPU
        let mvp_mat = camera_mat;

        renderer
            .upload_uniform_mat_4_name("u_size", &scaling_mat.as_slice())
            .unwrap();

        renderer
            .upload_uniform_mat_4_name("u_modelViewProjection", &mvp_mat.as_slice())
            .unwrap();
       

        //enable texture
        renderer
            .activate_texture_sampler_name(texture_id, "u_sampler")
            .unwrap();

        //draw!
        renderer.draw_arrays(BeginMode::TriangleStrip, 0, 4);
}
impl State {
    pub fn update(&mut self, _time_stamp: f64) {
    }

    pub fn resize (&mut self, width: f64, height: f64) {
        self.camera_width = width;
        self.camera_height = height;

        let area_width = self.area.width;
        let area_height = self.area.height;

        let mid_height = (height - area_height) / 2.0;
        let mid_width = (width - area_width) / 2.0;
        let margin = 10.0;

        self.positions = Some(vec![
            Point2::new(margin, mid_height),
            Point2::new(mid_width, mid_height),
            Point2::new(width - area_width - 10.0, mid_height),
        ],);
   
        self.picker.take();
    }
}

struct FrameBufferPicker {
    _hidden_texture_id: Id,
    visible_texture_id: Id,
    _renderbuffer_id: Id,
    write_framebuffer_id: Id,
    read_framebuffer_id: Id,
}

//see: https://stackoverflow.com/questions/21841483/webgl-using-framebuffers-for-picking-multiple-objects
impl FrameBufferPicker {
    pub fn new<T: WebGlCommon + 'static> (renderer:&mut WebGlRenderer<T>, width: u32, height: u32) -> Result<Self, awsm_web::errors::Error> {
        //setup a texture to store colors
        let hidden_texture_id = renderer.create_texture()?;
        renderer.assign_simple_texture(
            hidden_texture_id,
            TextureTarget::Texture2d,
            &SimpleTextureOptions {
                pixel_format: PixelFormat::Rgba,
                ..SimpleTextureOptions::default()
            },
            &WebGlTextureSource::EmptyBufferView(width, height, 0),
        )?;

        //and another for drawing
        let visible_texture_id = renderer.create_texture()?;
        renderer.assign_simple_texture(
            visible_texture_id,
            TextureTarget::Texture2d,
            &SimpleTextureOptions {
                pixel_format: PixelFormat::Rgba,
                ..SimpleTextureOptions::default()
            },
            &WebGlTextureSource::EmptyBufferView(width, height, 0),
        )?;


        //setup a renderbuffer to store depth info
        let renderbuffer_id = renderer.create_renderbuffer()?;
        renderer.assign_renderbuffer_storage(renderbuffer_id, RenderBufferFormat::DepthComponent16, width, height)?;

        //setup a framebuffer for offscreen rendering (using both textures and renderbuffer for depth)
        let write_framebuffer_id = renderer.create_framebuffer()?;
        renderer.assign_framebuffer_renderbuffer(write_framebuffer_id, renderbuffer_id, FrameBufferTarget::FrameBuffer, FrameBufferAttachment::Depth)?;
        renderer.assign_framebuffer_texture_2d(write_framebuffer_id, hidden_texture_id, FrameBufferTarget::FrameBuffer, FrameBufferAttachment::Color0, FrameBufferTextureTarget::Texture2d)?;
        renderer.assign_framebuffer_texture_2d(write_framebuffer_id, visible_texture_id, FrameBufferTarget::FrameBuffer, FrameBufferAttachment::Color1, FrameBufferTextureTarget::Texture2d)?;

        //make sure we're good
        renderer.check_framebuffer_status(FrameBufferTarget::FrameBuffer)?;

        //set the multi-draw targets
        //since we don't know here if we're in webgl1 or 2 gotta do both
        if let Ok(renderer) = renderer.as_webgl1() {
            renderer.draw_buffers(&vec![DrawBuffer::Color0, DrawBuffer::Color1])?;
        } else if let Ok(renderer) = renderer.as_webgl2() {
            renderer.draw_buffers(&vec![DrawBuffer::Color0, DrawBuffer::Color1])?;
        }

        //read only needs depth and hidden texture
        let read_framebuffer_id = renderer.create_framebuffer()?;
        renderer.assign_framebuffer_renderbuffer(read_framebuffer_id, renderbuffer_id, FrameBufferTarget::FrameBuffer, FrameBufferAttachment::Depth)?;
        renderer.assign_framebuffer_texture_2d(read_framebuffer_id, hidden_texture_id, FrameBufferTarget::FrameBuffer, FrameBufferAttachment::Color0, FrameBufferTextureTarget::Texture2d)?;
        
        //make sure we're still all good
        renderer.check_framebuffer_status(FrameBufferTarget::FrameBuffer)?;

        //unbind everything (no need to bind the texture to null)
        renderer.release_renderbuffer();
        renderer.release_framebuffer(FrameBufferTarget::FrameBuffer);

        Ok(Self{
            _hidden_texture_id: hidden_texture_id,
            visible_texture_id,
            _renderbuffer_id: renderbuffer_id,
            read_framebuffer_id,
            write_framebuffer_id
        })
    }

    pub fn bind<T: WebGlCommon> (&self, renderer:&mut WebGlRenderer<T>, id: Id) -> Result<(), awsm_web::errors::Error> {
        renderer.bind_framebuffer(id, FrameBufferTarget::FrameBuffer)
        //note - if the framebuffer *didn't* equal window size, set viewport to framebuffer size here
    }
    pub fn release<T: WebGlCommon> (&self, renderer:&mut WebGlRenderer<T>) {
        renderer.release_framebuffer(FrameBufferTarget::FrameBuffer)
        //note - if the framebuffer *didn't* equal window size, restore viewport to canvas size here
    }

    pub fn get_color<T: WebGlCommon> (&self, renderer:&mut WebGlRenderer<T>, client_x: f64, client_y: f64) -> Result<Color, awsm_web::errors::Error> {
        let mut data:[u8;4] = [0;4];

        //bind the read buffer which contains the hidden texture
        self.bind(renderer, self.read_framebuffer_id)?;
        renderer.read_pixels_u8(client_x as u32, client_y as u32, 1, 1, ReadPixelFormat::Rgba, ReadPixelDataType::UnsignedByte, &mut data)?;
        self.release(renderer);

        let color = Color::new(data[0] as f64 / 255.0, data[1] as f64 / 255.0, data[2] as f64 / 255.0, data[3] as f64 / 255.0);
        //log::info!("{} {} {} {}", color.r, color.g, color.b, color.a);
        Ok(color)
    }

    //Drop would delete texture_id, renderbuffer_id, and framebuffer_id
}
