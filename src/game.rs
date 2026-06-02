use crate::food::Food;
use crate::snake::{Direction, Snake};
use piston_window::Key;
use std::fs;
use std::path::Path;

const MOVE_INTERVAL: f64 = 0.15;

pub struct Game {
    pub snake: Snake,
    pub food: Food,
    pub width: i32,
    pub height: i32,
    pub waiting_time: f64,
    pub game_over: bool,
    pub score: u32,
    pub high_score: u32,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        let snake = Snake::new(width / 2, height / 2);
        let food = Food::new(width, height, &snake.body);
        
        let high_score = if Path::new("highscore.txt").exists() {
            fs::read_to_string("highscore.txt")
                .unwrap_or_else(|_| "0".to_string())
                .trim()
                .parse()
                .unwrap_or(0)
        } else {
            0
        };

        Game {
            snake,
            food,
            width,
            height,
            waiting_time: 0.0,
            game_over: false,
            score: 0,
            high_score,
        }
    }

    pub fn restart(&mut self) {
        self.snake = Snake::new(self.width / 2, self.height / 2);
        self.food = Food::new(self.width, self.height, &self.snake.body);
        self.waiting_time = 0.0;
        self.game_over = false;
        self.score = 0;
    }

    pub fn trigger_game_over(&mut self) {
        self.game_over = true;
        if self.score > self.high_score {
            self.high_score = self.score;
            let _ = fs::write("highscore.txt", self.high_score.to_string());
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            if key == Key::Space || key == Key::Return {
                self.restart();
            }
            return;
        }

        match key {
            Key::Up | Key::W => self.snake.set_direction(Direction::Up),
            Key::Down | Key::S => self.snake.set_direction(Direction::Down),
            Key::Left | Key::A => self.snake.set_direction(Direction::Left),
            Key::Right | Key::D => self.snake.set_direction(Direction::Right),
            _ => (),
        }
    }

    pub fn update(&mut self, dt: f64) {
        if self.game_over {
            return;
        }

        self.waiting_time += dt;

        let current_interval = (MOVE_INTERVAL - (self.score as f64 * 0.003)).max(0.04);

        if self.waiting_time > current_interval {
            self.waiting_time = 0.0;
            self.snake.update();

            let head = self.snake.head();
            
            if head.x < 0 || head.x >= self.width || head.y < 0 || head.y >= self.height {
                self.trigger_game_over();
                return;
            }

            for block in &self.snake.body[1..] {
                if head == *block {
                    self.trigger_game_over();
                    return;
                }
            }

            if head == self.food.position {
                self.snake.eat();
                self.score += 10;
                self.food = Food::new(self.width, self.height, &self.snake.body);
            }
        }
    }
}
