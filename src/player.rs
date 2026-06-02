pub struct Player {
    pub x: f64,
    pub y: f64,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 100.0,
            y: 100.0,
        }
    }
}