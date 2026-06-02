mod block;
mod food;
mod game;
mod snake;

use game::Game;
use piston_window::graphics::{clear, rectangle};
use piston_window::*;

const CELL_SIZE: f64 = 25.0;
const GRID_WIDTH: i32 = 25;
const GRID_HEIGHT: i32 = 25;

pub type Color = [f32; 4];
const SNAKE_HEAD_COLOR: Color = [0.0, 0.9, 0.0, 1.0];
const SNAKE_BODY_COLOR: Color = [0.0, 0.7, 0.0, 1.0];
const FOOD_COLOR: Color = [0.9, 0.1, 0.1, 1.0];
const GAMEOVER_COLOR: Color = [0.8, 0.0, 0.0, 0.5];

fn main() {
    // Workaround for `wgpu-hal WGL Instance Thread` stack overflow on Windows
    unsafe {
        std::env::set_var("WGPU_BACKEND", "vulkan"); // Try "dx12" if vulkan doesn't work
    }

    let window_width = (GRID_WIDTH as f64) * CELL_SIZE;
    let window_height = (GRID_HEIGHT as f64) * CELL_SIZE;

    let mut window: PistonWindow = WindowSettings::new("Snake Game", [window_width, window_height])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(GRID_WIDTH, GRID_HEIGHT);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        if let Some(args) = event.update_args() {
            game.update(args.dt);

            let title = if game.game_over {
                format!(
                    "Game Over! Press Space to Restart - Score: {} - High Score: {}",
                    game.score, game.high_score
                )
            } else {
                format!(
                    "Snake Game - Score: {} - High Score: {}",
                    game.score, game.high_score
                )
            };
            window.set_title(title);
        }

        window.draw_2d(&event, |c, g, _| {
            clear([0.15, 0.15, 0.15, 1.0], g);

            // Draw Food
            rectangle(
                FOOD_COLOR,
                [
                    game.food.position.x as f64 * CELL_SIZE,
                    game.food.position.y as f64 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                ],
                c.transform,
                g,
            );

            // Draw Snake
            for (i, block) in game.snake.body.iter().enumerate() {
                let color = if i == 0 {
                    SNAKE_HEAD_COLOR
                } else {
                    SNAKE_BODY_COLOR
                };
                rectangle(
                    color,
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

            // Game over overlay
            if game.game_over {
                rectangle(
                    GAMEOVER_COLOR,
                    [0.0, 0.0, window_width, window_height],
                    c.transform,
                    g,
                );
            }
        });
    }
}
