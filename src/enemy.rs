use macroquad::prelude::*;

pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub speed_x: f32,
    pub speed_y: f32,
    pub color: Color,
    pub is_erased: bool,
}

impl Enemy {
    
    pub fn new(x: f32, y: f32, speed_x: f32, speed_y: f32, color: Color, is_erased: bool) -> Self {
        Self {
            x,
            y,
            speed_x,
            speed_y,
            color,
            is_erased
        }
    }

    pub fn update(&mut self) {
        self.x += self.speed_x;
        self.y += self.speed_y;
    }

    pub fn draw(&self, texture: &Texture2D) {
        draw_texture(*texture, self.x, self.y, self.color);
    }
    
}