#![windows_subsystem = "windows"]

mod logic;
mod player;
mod bullet;
mod enemy;

use macroquad::prelude::*;
use logic::game_logic;
use player::*;
use bullet::*;

fn window_conf() -> Conf {
    Conf {
        window_resizable: false,
        window_width: 800,
        window_height: 600,
        window_title: "game".to_string(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    show_mouse(false);
    let mut player: Player = Player {
        x: Conf::default().window_width as f32 / 2.0 - 60.0 / 2.0,
        y: Conf::default().window_height as f32 - 80.0,
        w: 60.0,
        speed: 5.0,
        color: WHITE,
    };
    let mut bullet: Bullet = Bullet::new(
        player.x + player.w / 2.0,
        player.y - 25.0,
        12.0,
        WHITE,
        true
    );
    let texture_bg = Texture2D::from_file_with_format(
        include_bytes!("../assets/bg.png"),
        None
    );
    let texture_player = Texture2D::from_file_with_format(
        include_bytes!("../assets/spaceship.png"),
        None
    );
    loop {
        game_logic(&mut player, &mut bullet, &texture_bg, &texture_player);
        next_frame().await
    }
}
