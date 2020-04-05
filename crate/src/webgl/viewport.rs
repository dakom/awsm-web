use super::{WebGlCommon, WebGlRenderer};
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext};

#[derive(Eq, PartialEq)]
pub enum ResizeStrategy {
    Canvas,
    Viewport,
    All 
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
    pub fn resize(&mut self, width: u32, height: u32, strategy: ResizeStrategy) {
        if self.last_width != width || self.last_height != height {
            let gl = &mut self.gl;
            let canvas = &mut self.canvas;
            if strategy == ResizeStrategy::Canvas || strategy == ResizeStrategy::All {
                canvas.set_width(width);
                canvas.set_height(height);
            }

            if strategy == ResizeStrategy::Viewport || strategy == ResizeStrategy::All {
                //this might be better in some circumstances? Not sure...
                //gl.awsm_viewport( 0, 0, gl.awsm_drawing_buffer_width(), gl.awsm_drawing_buffer_height());
                gl.awsm_viewport( 0, 0, width, height); 
            }

            self.last_width = width;
            self.last_height = height;
        }
    }

    pub fn current_size(&self) -> (u32, u32) {
        (self.last_width, self.last_height)
    }
}
