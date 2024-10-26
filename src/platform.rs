use raylib::prelude::*;

pub struct Platform {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub show: bool,
}

impl Platform {
    pub fn new(x: i32, y: i32, width: i32, height: i32, show: bool) -> Self {
        Self {
            x,
            y,
            width,
            height,
            show,
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle<'_>) {
        d.draw_circle(self.x, self.y, self.width as f32, Color::GOLD);
    }

    pub fn move_left(&mut self) {
        self.x -= 1;
        if self.x <= 0 {
            self.show = false
        }
    }
}
