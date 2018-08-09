mod player;


use rendering::{Color, Renderer};


pub struct Game {
    glow: f64,
}

impl Game {
    pub fn new() -> Self {
        Game {
            glow: 0.0,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.glow += dt / 1000.0;
        self.glow %= 1.0;
    }

    pub fn render(&self, renderer: &Renderer) {
        renderer.clear(Color {
            r: self.glow,
            g: self.glow,
            b: self.glow,
        });
    }
}
