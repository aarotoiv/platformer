use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

pub struct Renderer {
    pub gl: GlGraphics,
}

impl Renderer {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        self.gl.draw(args.viewport(), |c, gl| {
            //clear
            clear(BACKGROUND, gl);

            
        });
    }
}
