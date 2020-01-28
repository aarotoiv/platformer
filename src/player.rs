use piston::input::{
    Key
};
use piston::input::UpdateArgs;
use crate::world::World;

pub struct Player {
    pub x_pos: f64,
    pub y_pos: f64,
    pub render_x: f64,
    pub render_y: f64,
    pub scale: f64,
    pub outline_scale: f64,
    pub color: [f32;4],
    keys: Keys,
    pub direction: f64,
    falling: bool,
    touches: Touches
}

struct Keys {
    left: bool,
    right: bool
}

struct Touches {
    left: bool,
    right: bool,
    top: bool,
    bottom: bool
}

impl Player {
    pub fn new(window_width: &u32, window_height: &u32) -> Player {
        Player {
            x_pos: 0.0,
            y_pos: 0.0,
            render_x: *window_width as f64 / 2.0,
            render_y: *window_height as f64 / 2.0,
            outline_scale: 0.05,
            scale: 50.0,
            color: [1.0, 1.0, 1.0, 1.0],
            keys: Keys {left: false, right: false},
            direction: 1.0,
            falling: true,
            touches: Touches {left: false, right: false, top: false, bottom: false}
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        const X_ACCEL: f64 = 400.0;
        const Y_ACCEL: f64 = 400.0;
        let mut x_vel = 0.0;
        let mut y_vel = 0.0;
        if self.keys.left {
            x_vel += -X_ACCEL;
        }
        if self.keys.right {
            x_vel += X_ACCEL;
        }
        
        if x_vel > 0.0 {
            self.direction = 1.0;
        } else if x_vel < 0.0 {
            self.direction = -1.0;
        }

        if self.falling {
            y_vel += Y_ACCEL;
        }



        self.x_pos += x_vel * args.dt;
        self.y_pos += y_vel * args.dt;

    }

    pub fn handle_collisions(&mut self, world: &World) {
        for block in world.blocks.iter() {
            let b_width = block.end_x - block.start_x;
            let b_height = block.end_y - block.start_y;
            let b_x = (block.start_x + block.end_x) / 2.0;
            let b_y = (block.start_y + block.end_y) / 2.0;


        }
    }

    pub fn handle_press(&mut self, key: Key) {
        match key {
            Key::A => {
                self.keys.left = true;
            }
            Key::D => {
                self.keys.right = true;
            }
            _ => {}
        }
    }
    pub fn handle_release(&mut self, key: Key) {
        match key {
            Key::A => {
                self.keys.left = false;
            }
            Key::D => {
                self.keys.right = false;
            }
            _ => {}
        }
    }
}