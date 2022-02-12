use macroquad::prelude::*;
use crate::player::*;
use crate::bullet::*;

fn bg_draw(texture: &Texture2D) {
    draw_texture(*texture, 0.0, 0.0, WHITE);
}

pub fn game_logic(player: &mut Player, bullet: &mut Bullet, texture_bg: &Texture2D, texture_player: &Texture2D) {
    bg_draw(&texture_bg);
    player.draw(&texture_player);
    if is_key_down(KeyCode::A) {
        player.left();
    }
    if is_key_down(KeyCode::D) {
        player.right();
    }
    if is_mouse_button_pressed(MouseButton::Left) {
        if bullet.is_ready {
            bullet.x = player.x + player.w / 2.0;
        }
        bullet.fire();
    }
    if !bullet.is_ready {
        bullet.update();
        bullet.draw();
    }
    if bullet.y < 0.0 {
        bullet.ready();
        bullet.y = player.y - 25.0;
    }
    if player.x > 800.0 - player.w {
        player.x = 800.0 - player.w;
    }
    if player.x < 0.0 {
        player.x = 0.0;
    }
}