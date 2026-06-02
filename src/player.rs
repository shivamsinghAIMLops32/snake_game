pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Player {
   pub x: i32,
pub y: i32,
    pub direction: Direction,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 5,
        y: 5,
            direction: Direction::Right,
        }
    }

  pub fn update(&mut self) {
    match self.direction {
        Direction::Up => self.y -= 1,
        Direction::Down => self.y += 1,
        Direction::Left => self.x -= 1,
        Direction::Right => self.x += 1,
    }
}
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn reset(&mut self) {
        self.x = 5;
        self.y = 5;
    }
}
