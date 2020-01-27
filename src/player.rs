pub struct Player {
    pub x_pos: f64,
    pub y_pos: f64,
    pub render_x: f64,
    pub render_y: f64,
    pub scale: f64,
    pub color: [f32;4]
}

impl Player {
    pub fn new(window_width: &u32, window_height: &u32) -> Player {
        Player {
            x_pos: 0.0,
            y_pos: 0.0,
            render_x: *window_width as f64 / 2.0,
            render_y: *window_height as f64 / 2.0,
            scale: 50.0,
            color: [1.0, 1.0, 1.0, 1.0]
        }
    }
}