#![windows_subsystem = "windows"]

mod logic;
mod player;
mod bullet;
mod enemy;

use macroquad::{prelude::*, audio::{load_sound_from_bytes}};
use logic::game_logic;
use player::*;
use bullet::*;
use enemy::*;

fn window_conf() -> Conf {
    Conf {
        window_resizable: false,
        window_width: 800,
        window_height: 600,
        window_title: "SpaceInvaders".to_string(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    show_mouse(false);
    let mut bullet_vec: Vec<Bullet> = vec![];
    let mut enemy_vec: Vec<Enemy> = vec![];
    let mut enemy_count: i32 = 1;
    let mut score: i32 = 0;
    let mut player: Player = Player {
        x: Conf::default().window_width as f32 / 2.0 - 60.0 / 2.0,
        y: Conf::default().window_height as f32 - 80.0,
        w: 60.0,
        speed: 5.0,
        color: WHITE,
        gameover: false,
    };
    let texture_bg = Texture2D::from_file_with_format(
        include_bytes!("../assets/bg.png"),
        None
    );
    let texture_player = Texture2D::from_file_with_format(
        include_bytes!("../assets/spaceship.png"),
        None
    );
    let texture_enemy = Texture2D::from_file_with_format(
        include_bytes!("../assets/enemy.png"),
        None
    );
    let audio_gameover = load_sound_from_bytes(include_bytes!("../assets/gameover.wav")).await.unwrap();
    let audio_laser = load_sound_from_bytes(include_bytes!("../assets/cannon.wav")).await.unwrap();
    let audio_explosion = load_sound_from_bytes(include_bytes!("../assets/explosion.wav")).await.unwrap();
    loop {
        if player.gameover {
            clear_background(BLACK);
            draw_text("GAME OVER", Conf::default().window_width as f32 - 600.0, Conf::default().window_height as f32 / 2.0 + 25.0, 100.0, LIME);
            draw_text(&format!("Score: {}", score), Conf::default().window_width as f32 / 2.0 - 45.0, Conf::default().window_height as f32 / 2.0 + 80.0, 25.0, YELLOW);
            if is_key_pressed(KeyCode::R) {
                player.x = Conf::default().window_width as f32 / 2.0 - 60.0 / 2.0;
                player.y = Conf::default().window_height as f32 - 80.0;
                enemy_vec = vec![];
                bullet_vec = vec![];
                score = 0;
                player.gameover = false;
            }
        }
        else {
            game_logic(&mut player, &mut bullet_vec, &mut enemy_vec, &mut enemy_count, &mut score, &texture_bg, &texture_player, &texture_enemy, &audio_laser, &audio_explosion, &audio_gameover);
        }
        next_frame().await
    }
}
