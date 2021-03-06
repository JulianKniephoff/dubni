mod color;


use stdweb::{self, web::html_element::CanvasElement};

pub use self::color::Color;


pub struct Renderer {
    canvas: CanvasElement,
    gl: stdweb::Value,
}

impl Renderer {

    pub fn init(canvas: CanvasElement) -> Self {
        let gl = js! {
            return @{&canvas}.getContext("webgl");
        };
        Self { canvas, gl }
    }

    pub fn resize(&self, width: u32, height: u32) {
        let gl = &self.gl;
        let canvas = &self.canvas;
        canvas.set_width(width);
        canvas.set_height(height);
        js! {
            @{gl}.viewport(0, 0, @{canvas}.width, @{canvas}.height);
        }
    }

    pub fn clear(&self, color: Color) {
        let Color { r, g, b } = color;
        let gl = &self.gl;
        js! {
            @{gl}.clearColor(@{r}, @{g}, @{b}, 1);
            @{gl}.clear(@{gl}.COLOR_BUFFER_BIT);
        }
    }
}
