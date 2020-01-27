use piston::input::{
    Key
};
use piston::input::UpdateArgs;

pub struct Player {
    pub x_pos: f64,
    pub y_pos: f64,
    pub render_x: f64,
    pub render_y: f64,
    pub scale: f64,
    pub outline_scale: f64,
    pub color: [f32;4],
    keys: Keys,
    pub direction: i8
}

struct Keys {
    left: bool,
    right: bool
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
            direction: 1
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let mut x_vel = 0.0;
        if self.keys.left {
            x_vel += -150.0;
        } else if self.keys.right {
            x_vel += 150.0;
        }
        
        if x_vel > 0.0 {
            self.direction = 1;
        } else if x_vel < 0.0 {
            self.direction = -1;
        }

        self.x_pos += x_vel * args.dt;

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