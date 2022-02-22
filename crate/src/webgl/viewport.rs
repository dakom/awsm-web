use super::{WebGlCommon, WebGlRenderer};
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext};

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum ResizeStrategy {
    /// set both canvas and viewport to width, height
    All(u32, u32),
    /// set just canvas to width, height
    Canvas(u32, u32),
    /// set just viewport to width, height
    ViewportSize(u32, u32),
    /// automatically set viewport to 0,0,drawing_buffer_width,drawing_buffer_height
    ViewportMatchSize,
    /// set viewport x,y,width,height
    Viewport(u32, u32, u32, u32),
}
pub trait PartialWebGlViewport {
    fn awsm_viewport(&self, x: u32, y: u32, width: u32, height: u32);
    fn awsm_drawing_buffer_height(&self) -> u32;
    fn awsm_drawing_buffer_width(&self) -> u32;
}

macro_rules! impl_context {
    ($($type:ty { $($defs:tt)* })+) => {
        $(impl PartialWebGlViewport for $type {
            fn awsm_viewport(&self, x: u32, y: u32, width: u32, height: u32) {
                self.viewport(x as i32, y as i32, width as i32, height as i32)
            }
            fn awsm_drawing_buffer_height(&self) -> u32 {
                self.drawing_buffer_height() as u32
            }
            fn awsm_drawing_buffer_width(&self) -> u32 {
                self.drawing_buffer_width() as u32
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
    pub fn resize(&mut self, strategy: ResizeStrategy) -> bool {
        if self.last_resize_strategy != Some(strategy) {
            let gl = &mut self.gl;
            let canvas = &mut self.canvas;

            let mut viewport:Option<(u32, u32, u32, u32)> = None;

            match strategy {
                ResizeStrategy::All(width, height) => {
                    canvas.set_width(width);
                    canvas.set_height(height);
                    viewport = Some(( 0, 0, width, height)); 
                }

                ResizeStrategy::Canvas(width, height) => {
                    canvas.set_width(width);
                    canvas.set_height(height);
                },

                ResizeStrategy::ViewportSize(width, height) => {
                    viewport = Some((0, 0, width, height)); 
                },
                ResizeStrategy::ViewportMatchSize => {
                    viewport = Some(( 0, 0, gl.awsm_drawing_buffer_width(), gl.awsm_drawing_buffer_height()));
                }
                ResizeStrategy::Viewport(x, y, width, height) => {
                    viewport = Some(( x, y, width, height)); 
                }
            };

            if let Some(viewport) = viewport {
                let (x, y, width, height) = viewport;
                gl.awsm_viewport(x, y, width, height);
                self.viewport = Some(viewport);
            } 

            self.last_resize_strategy = Some(strategy);

            true
        } else {
            false
        }
    }

    pub fn get_viewport(&self) -> (u32, u32, u32, u32) {
        self.viewport.unwrap_or((0,0,0,0))
    }
}
