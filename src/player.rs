use crate::world::World;
use piston::input::Key;
use piston::input::UpdateArgs;

extern crate rand;
use rand::Rng;

pub struct Player {
    pub x_pos: f64,
    pub y_pos: f64,
    x_vel: f64,
    y_vel: f64,
    prev_x: f64,
    prev_y: f64,
    pub render_x: f64,
    pub render_y: f64,
    pub scale: f64,
    pub outline_scale: f64,
    pub color: [f32; 4],
    keys: Keys,
    pub direction: f64,
    falling: bool,
    touches: Touches,
    wants_to_jump: bool,
    pub sparks: Vec<Spark>
}
pub struct Spark {
    pub x_pos: f64,
    pub y_pos: f64,
    pub scale: f64,
    x_vel: f64,
    y_vel: f64,
    pub time: f64
}
struct Keys {
    left: bool,
    right: bool,
}

struct Touches {
    left: bool,
    right: bool,
    top: bool,
    bottom: bool,
}

impl Player {
    pub fn new(window_width: &u32, window_height: &u32) -> Player {
        Player {
            x_pos: 0.0,
            y_pos: 0.0,
            x_vel: 0.0,
            y_vel: 0.0,
            prev_x: 0.0,
            prev_y: 0.0,
            render_x: *window_width as f64 / 2.0,
            render_y: *window_height as f64 / 2.0,
            outline_scale: 0.05,
            scale: 50.0,
            color: [1.0, 1.0, 1.0, 1.0],
            keys: Keys {
                left: false,
                right: false,
            },
            direction: 1.0,
            falling: true,
            touches: Touches {
                left: false,
                right: false,
                top: false,
                bottom: false,
            },
            wants_to_jump: false,
            sparks: Vec::new()
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        const X_ACCEL: f64 = 40.0;
        const Y_ACCEL: f64 = 30.0;
        const X_MIN_CAP: f64 = -800.0;
        const X_MAX_CAP: f64 = 800.0;
        const JUMP_START: f64 = -1500.0;

        self.prev_x = self.x_pos;
        self.prev_y = self.y_pos;

        self.falling = !self.touches.bottom;

        if self.keys.left && self.x_vel > X_MIN_CAP {
            self.x_vel += -X_ACCEL;
        }
        if self.keys.right && self.x_vel < X_MAX_CAP {
            self.x_vel += X_ACCEL;
        }

        if !self.keys.right && !self.keys.left && self.x_vel != 0.0 {
            if self.x_vel > 0.0 {
                self.x_vel -= 30.0;
            } else if self.x_vel < 0.0 {
                self.x_vel += 30.0;
            }

            if self.x_vel.abs() < 30.0 {
                self.x_vel = 0.0;
            }
        }

        if self.x_vel < 0.0 && self.touches.left {
            self.x_vel = 0.0;
        }

        if self.x_vel > 0.0 && self.touches.right {
            self.x_vel = 0.0;
        }

        if self.x_vel > 0.0 {
            self.direction = 1.0;
        } else if self.x_vel < 0.0 {
            self.direction = -1.0;
        }

        if self.falling {
            self.y_vel += Y_ACCEL;
        }

        if self.touches.bottom && self.y_vel > 200.0 {
            self.add_collision_sparks();
            self.y_vel = 0.0;
        }
        

        if self.wants_to_jump && !self.falling {
            self.y_vel = JUMP_START;
        }

        self.x_pos += self.x_vel * args.dt;
        self.y_pos += self.y_vel * args.dt;

        self.update_sparks(args);
    }

    fn update_sparks(&mut self, args: &UpdateArgs) {
        self.sparks.retain(|s| s.time <= 1.0);
        for i in 0..self.sparks.len() {
            self.sparks[i].x_pos += self.sparks[i].x_vel * args.dt;
            self.sparks[i].y_pos += self.sparks[i].y_vel * args.dt;
            self.sparks[i].time += args.dt;
        }
    }

    pub fn handle_collisions(&mut self, world: &World) {
        let mut bottom = false;
        //let mut top = false;
        let mut left = false;
        let mut right = false;

        for block in world.blocks.iter() {
            let b_width = block.end_x - block.start_x;
            let b_height = block.end_y - block.start_y;
            let b_x = (block.start_x + block.end_x) / 2.0;
            let b_y = (block.start_y + block.end_y) / 2.0;

            if self.x_pos - self.scale / 2.0 < b_x + b_width / 2.0
                && self.x_pos + self.scale / 2.0 > b_x - b_width / 2.0
                && self.y_pos + self.scale / 2.0 < b_y + b_height / 2.0
                && self.y_pos + self.scale / 2.0 > b_y - b_height / 2.0
                && self.prev_y < b_y - b_height / 2.0
            {
                bottom = true;
                self.y_pos -= (self.y_pos + self.scale / 2.0) - (b_y - b_height / 2.0);
            }

            if self.y_pos + self.scale / 2.0 > b_y - b_height / 2.0
                && self.y_pos - self.scale / 2.0 < b_y + b_height / 2.0
            {
                if self.x_pos + self.scale / 2.0 > b_x - b_width / 2.0
                    && self.x_pos + self.scale / 2.0 < b_x + b_width / 2.0
                    && self.prev_x < b_x - b_width / 2.0
                {
                    right = true;
                    self.x_pos -= (self.x_pos + self.scale / 2.0) - (b_x - b_width / 2.0);
                }

                if self.x_pos - self.scale / 2.0 < b_x + b_width / 2.0
                    && self.x_pos - self.scale / 2.0 > b_x - b_width / 2.0
                    && self.prev_x > b_x + b_width / 2.0
                {
                    left = true;
                    self.x_pos -= (self.x_pos - self.scale / 2.0) - (b_x + b_width / 2.0);
                }
            }
        }

        self.touches.bottom = bottom;
        self.touches.right = right;
        self.touches.left = left;
    }
    fn add_collision_sparks(&mut self) {
        let mut rng = rand::thread_rng();
        for _i in 0..5 {
            for j in (-1..2).step_by(2) {
                println!("{}", j);
                if j == -1 {
                    self.sparks.push(
                        Spark {
                            x_pos: self.x_pos - self.scale / 2.0,
                            y_pos: self.y_pos + self.scale / 2.0,
                            scale: rng.gen_range(1.0, 4.0),
                            x_vel: rng.gen_range(-100.0 * (self.y_vel / 500.0), -10.0),
                            y_vel: rng.gen_range(-100.0, -10.0),
                            time: 0.0
                        }
                    );
                } else {
                    self.sparks.push(
                        Spark {
                            x_pos: self.x_pos + self.scale / 2.0,
                            y_pos: self.y_pos + self.scale / 2.0,
                            scale: rng.gen_range(1.0, 4.0),
                            x_vel: rng.gen_range(10.0, 100.0 * (self.y_vel / 500.0)),
                            y_vel: rng.gen_range(-100.0, -10.0),
                            time: 0.0
                        }
                    );
                }
            }
        }
        //scale: rng.gen_range(5.0, 12.0),
    }
    pub fn handle_press(&mut self, key: Key) {
        match key {
            Key::A => {
                self.keys.left = true;
            }
            Key::D => {
                self.keys.right = true;
            }
            Key::Space => {
                self.wants_to_jump = true;
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
            Key::Space => {
                self.wants_to_jump = false;
            }
            _ => {}
        }
    }
}
