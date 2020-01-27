use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

use crate::player::Player;
use crate::world::World;

pub struct Renderer {
    pub gl: GlGraphics,
}

impl Renderer {
    pub fn render(&mut self, args: &RenderArgs, player: &Player, world: &World) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        self.gl.draw(args.viewport(), |c, gl| {
            //clear
            clear(BACKGROUND, gl);

            for block in world.blocks.iter() {
                let block_rect = rectangle::rectangle_by_corners(block.start_x, block.start_y, block.end_x, block.end_y);
                rectangle(
                    [1.0, 1.0, 1.0, 1.0],
                    block_rect,
                    c.transform.trans(player.render_x - player.x_pos, player.render_y - player.y_pos),
                    gl
                )
            }

            //player
            let player_rect = rectangle::square(0.0, 0.0, player.scale);
            rectangle(
                player.color,
                player_rect,
                c.transform.trans(player.render_x - player.scale / 2.0, player.render_y - player.scale / 2.0),
                gl
            );
            rectangle(
                BACKGROUND,
                player_rect,
                c.transform.trans(player.render_x - player.scale * (1.0 - player.outline_scale) / 2.0,
                player.render_y - player.scale * (1.0 - player.outline_scale) / 2.0)
                .scale(1.0 - player.outline_scale, 1.0 - player.outline_scale),
                gl,
            );
            let eye_rect = rectangle::square(0.0, 0.0, player.scale * 0.1);
            rectangle(
                player.color,
                eye_rect,
                c.transform.trans(player.render_x - (player.scale * 0.1 / 2.0) - (player.scale * 0.1) * player.direction, player.render_y - (player.scale * 0.1 / 2.0) - player.scale * 0.2),
                gl,
            );
            rectangle(
                player.color,
                eye_rect,
                c.transform.trans(player.render_x - (player.scale * 0.1 / 2.0) + (player.scale * 0.2) * player.direction, player.render_y - (player.scale * 0.1 / 2.0) - player.scale * 0.2),
                gl,
            );

        });
    }
}
