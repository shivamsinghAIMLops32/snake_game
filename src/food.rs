use crate::block::Block;
use rand::RngExt;

pub struct Food {
    pub position: Block,
}

impl Food {
    pub fn new(width: i32, height: i32, snake_body: &[Block]) -> Self {
        let mut rng = rand::rng();
        let mut x;
        let mut y;

        loop {
            x = rng.random_range(0..width);
            y = rng.random_range(0..height);
            let pos = Block { x, y };

            if !snake_body.contains(&pos) {
                return Self { position: pos };
            }
        }
    }
}
