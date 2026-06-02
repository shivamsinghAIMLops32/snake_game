pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub direction: Direction,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 100.0,
            y: 100.0,
            direction: Direction::Right,
        }
    }

    pub fn update(&mut self) {
        match self.direction {
            Direction::Up => self.y -= 5.0,
            Direction::Down => self.y += 5.0,
            Direction::Left => self.x -= 5.0,
            Direction::Right => self.x += 5.0,
        }
    }
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn reset(&mut self) {
        self.x = 100.0;
        self.y = 100.0;
    }
}
