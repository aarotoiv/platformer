use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

use crate::player::Player;

pub struct Renderer {
    pub gl: GlGraphics,
}

impl Renderer {
    pub fn render(&mut self, args: &RenderArgs, player: &Player) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        self.gl.draw(args.viewport(), |c, gl| {
            //clear
            clear(BACKGROUND, gl);

            //player
            let player_rect = rectangle::square(0.0, 0.0, player.scale);
            rectangle(
                player.color,
                player_rect,
                c.transform.trans(player.render_x, player.render_y),
                gl
            );
            rectangle(
                BACKGROUND,
                player_rect,
                c.transform.trans(player.render_x, player.render_y).scale(0.95),
                gl,
            );
        });
    }
}
