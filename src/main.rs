mod player;

use graphics::{clear, rectangle};
use piston_window::*;
use player::{Direction, Player};

fn main() {
    let mut player = Player::new();

    let mut window: PistonWindow = WindowSettings::new("Snake Game", [600, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        // Game update
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up => player.set_direction(Direction::Up),
                Key::Down => player.set_direction(Direction::Down),
                Key::Left => player.set_direction(Direction::Left),
                Key::Right => player.set_direction(Direction::Right),
                _ => {}
            }
        }
        if event.update_args().is_some() {
            player.update();
        }

        // Draw
        window.draw_2d(&event, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            rectangle(
                [0.0, 1.0, 0.0, 1.0],
                [player.x, player.y, 50.0, 50.0],
                c.transform,
                g,
            );
        });
    }
}
