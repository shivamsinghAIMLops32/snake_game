use crate::block::Block;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    pub body: Vec<Block>,
    pub direction: Direction,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            body: vec![
                Block { x: 5, y: 5 },
                Block { x: 4, y: 5 },
                Block { x: 3, y: 5 },
            ],
            direction: Direction::Right,
        }
    }

    pub fn head(&self) -> Block {
        self.body[0]
    }

    pub fn update(&mut self) {
        let mut new_head = self.head();

        match self.direction {
            Direction::Up => new_head.y -= 1,
            Direction::Down => new_head.y += 1,
            Direction::Left => new_head.x -= 1,
            Direction::Right => new_head.x += 1,
        }

        self.body.insert(0, new_head);

        self.body.pop();
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}