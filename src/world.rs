pub struct World {
    pub blocks: Vec<Block>
}

pub struct Block {
    pub start_x: f64,
    pub start_y: f64,
    pub end_x: f64,
    pub end_y: f64
}

impl World {
    pub fn new() -> World {
        World {
            blocks: Vec::new()
        }
    }
    pub fn initialize(&mut self) {
        self.blocks.push(Block::new());
    }
}

impl Block {
    pub fn new() -> Block {
        Block {
            start_x: -200.0,
            start_y: 300.0,
            end_x: 600.0,
            end_y: 400.0
        }
    }
}