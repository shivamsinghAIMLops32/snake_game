use graphics::{clear, rectangle};
use piston_window::*;

mod player;
use player::Player;
fn main() {
    let player = Player::new();

println!("{} {}", player.x, player.y);
    let mut window: PistonWindow = WindowSettings::new(
        "Snake Game",
        [600, 600]
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            rectangle(
                [0.0, 1.0, 0.0, 1.0], // green
                [100.0, 100.0, 50.0, 50.0], // x,y,w,h
                c.transform,
                g,
            );
        });
    }
}