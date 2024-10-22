use raylib::prelude::*;

const GRAVITY: f32 = 0.5;
const JUMP_FORCE: f32 = -10.0;

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub velocity: f32,
    pub is_jumping: bool,
}

impl Player {
    pub fn new(x: i32, y: i32, velocity: f32, is_jumping: bool) -> Self {
        Self {
            x,
            y,
            velocity,
            is_jumping,
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle<'_>) {
        d.draw_rectangle(self.x, self.y, 50, 50, Color::RED);
    }

    pub fn movements(&mut self, d: &mut RaylibDrawHandle<'_>) {
        if d.is_key_down(KeyboardKey::KEY_D) {
            self.x += 1;
        }

        if d.is_key_pressed(KeyboardKey::KEY_SPACE) && !self.is_jumping {
            self.jump(d.get_screen_height() - 70);
        }
        self.update(d.get_screen_height() - 70);
    }

    fn apply_gravity(&mut self, ground_level: i32, gravity: f32) {
        if self.y < ground_level || self.is_jumping {
            self.velocity += gravity;
        }
    }

    fn jump(&mut self, ground_level: i32) {
        if self.y >= ground_level {
            self.is_jumping = true;
            self.velocity = JUMP_FORCE;
        }
    }
    fn update(&mut self, ground_level: i32) {
        self.apply_gravity(ground_level, GRAVITY);

        self.y += self.velocity as i32;

        // Prevent the player from falling below the ground
        if self.y >= ground_level {
            self.y = ground_level;
            self.velocity = 0.0;
            self.is_jumping = false;
        }
    }
}
