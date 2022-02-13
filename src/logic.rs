use macroquad::prelude::*;
use crate::player::*;
use crate::bullet::*;
use crate::enemy::*;

fn random_color() -> Color {
    let color_vec: Vec<Color> = vec![RED, GREEN, BLUE, YELLOW, PURPLE, ORANGE, PINK, LIME, BROWN, MAROON, GRAY, BLACK, WHITE];
    let color_index: usize = rand::gen_range(0, color_vec.len());
    color_vec[color_index]
}

fn bg_draw(texture: &Texture2D) {
    draw_texture(*texture, 0.0, 0.0, WHITE);
}

fn coordinate_distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}

fn score_renderer(score: &i32) {
    draw_text(
        &format!("Score: {}", score)[..],
        650.0,
        25.0,
        25.0,
        YELLOW
    );
}

pub fn game_logic(player: &mut Player, bullet_vec: &mut Vec<Bullet>, enemy_vec: &mut Vec<Enemy>, enemy_count: &mut i32, score: &mut i32, texture_bg: &Texture2D, texture_player: &Texture2D, texture_enemy: &Texture2D) {
    bg_draw(&texture_bg);
    player.draw(&texture_player);
    score_renderer(&score);
    if is_key_down(KeyCode::A) {
        player.left();
    }
    if is_key_down(KeyCode::D) {
        player.right();
    }
    if is_mouse_button_pressed(MouseButton::Left) {
        bullet_vec.append(&mut vec![Bullet::new(player.x + player.w / 2.0, player.y - 15.0, 10.0, WHITE, true)]);
    }
    if rand::rand() as i32 % 25 == 0 {
        if enemy_count > &mut (enemy_vec.len() as i32) {
            enemy_vec.append(&mut vec![Enemy::new(rand::gen_range(0.0 + player.w, Conf::default().window_width as f32 - player.w), 20.0, rand::gen_range(1.0, 8.0), rand::gen_range(1.0, 8.0), random_color(), false)]);
            *enemy_count += 1;
        }
    }
    if player.x > 800.0 - player.w {
        player.x = 800.0 - player.w;
    }
    if player.x < 0.0 {
        player.x = 0.0;
    }
    for bullet in bullet_vec.iter_mut() {
        if bullet.is_ready {
            bullet.fire();
        }
        bullet.update();
        bullet.draw();
        if bullet.y < 0.0 {
            bullet.ready();
        }
    }
    for enemy in enemy_vec.iter_mut() {
        enemy.update();
        enemy.draw(&texture_enemy);
        if enemy.x > 800.0 - 15.0 {
            enemy.speed_x = -enemy.speed_x;
        }
        if enemy.x < 0.0 {
            enemy.speed_x = -enemy.speed_x;
        }
    }
    for bullet in bullet_vec.iter_mut() {
        for enemy in enemy_vec.iter_mut() {
            if coordinate_distance(bullet.x, bullet.y, enemy.x, enemy.y) < 15.0 / 2.0 + 5.0 / 2.0 {
                enemy.is_erased = true;
                *score += 1;
            }
        }
    }
    for enemy in enemy_vec.iter_mut() {
        if player.x < enemy.x + 15.0 && player.x + 64.0 > enemy.x && player.y < enemy.y + 15.0 && player.y + 64.0 > enemy.y {
            player.gameover = true;
            break;
        }
    }
    bullet_vec.retain(|x| x.y > 0.0);
    enemy_vec.retain(|x| x.y < Conf::default().window_height as f32);
    enemy_vec.retain(|x| !x.is_erased);
}