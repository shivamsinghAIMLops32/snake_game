mod block;
mod snake;

use graphics::{clear, rectangle};
use piston_window::*;
use snake::{Direction, Snake};

const CELL_SIZE: f64 = 30.0;

fn main() {
    let mut snake = Snake::new();

    let mut window: PistonWindow =
        WindowSettings::new("Snake Game", [600, 600])
            .exit_on_esc(true)
            .build()
            .unwrap();

    while let Some(event) = window.next() {

        // Keyboard Input
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up => snake.set_direction(Direction::Up),
                Key::Down => snake.set_direction(Direction::Down),
                Key::Left => snake.set_direction(Direction::Left),
                Key::Right => snake.set_direction(Direction::Right),
                _ => {}
            }
        }

        // Game Update
        if event.update_args().is_some() {
            snake.update();
        }

        // Render
        window.draw_2d(&event, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            for block in &snake.body {
                rectangle(
                    [0.0, 1.0, 0.0, 1.0],
                    [
                        block.x as f64 * CELL_SIZE,
                        block.y as f64 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                    ],
                    c.transform,
                    g,
                );
            }
        });
    }
}