use macroquad::prelude::*;

pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub color: Color,
}

impl Enemy {
    
    pub fn new(x: f32, y: f32, speed: f32, color: Color) -> Self {
        Self {
            x,
            y,
            speed,
            color,
        }
    }

    pub fn update(&mut self, direction: bool) {
        self.y += self.speed;
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, 10.0, 10.0, self.color);
    }
    
}