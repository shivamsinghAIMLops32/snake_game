use crate::block::Block;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

pub struct Snake {
    pub body: Vec<Block>,
    pub direction: Direction,
    pub last_direction: Direction,
    grow: bool,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            body: vec![
                Block { x, y },
                Block { x: x - 1, y },
                Block { x: x - 2, y },
            ],
            direction: Direction::Right,
            last_direction: Direction::Right,
            grow: false,
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

        self.last_direction = self.direction;

        self.body.insert(0, new_head);

        if self.grow {
            self.grow = false;
        } else {
            self.body.pop();
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if direction.opposite() != self.last_direction {
            self.direction = direction;
        }
    }

    pub fn eat(&mut self) {
        self.grow = true;
    }
}
