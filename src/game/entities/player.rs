use raylib::prelude::*;

use crate::{assets::Assets, Surface};

use super::Camera;


pub struct Player {
    pub x: f32,
    pub y: f32,
    pub z: f32,

    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
}

impl Player {
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, vx: 0.0, vy: 0.0, vz: 0.0 }
    }

    const SPEED: f32 = 3.5;
    
    pub fn update(&mut self, rl: &RaylibHandle, dt: f32) {
        let mut dir = Vector2::new(0.0, 0.0);

        if rl.is_key_down(KeyboardKey::KEY_W) {
            dir.x -= 1.0;
            dir.y -= 1.0;
            // self.animation = Animation::Up
        }

        if rl.is_key_down(KeyboardKey::KEY_S) {
            dir.x += 1.0;
            dir.y += 1.0;
            // self.animation = Animation::Down
        }

        if rl.is_key_down(KeyboardKey::KEY_A) {
            dir.x -= 1.0;
            dir.y += 1.0;
            // self.animation = Animation::Left
        }

        if rl.is_key_down(KeyboardKey::KEY_D) {
            dir.x += 1.0;
            dir.y -= 1.0;
            // self.animation = Animation::Right
        }

        if dir.x != 0.0 || dir.y != 0.0 {
            let mut speed = Self::SPEED * dt;

            // Sprinting
            if rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) {
                speed *= 1.6;
            }

            dir.normalize();
            self.x += dir.x * speed;
            self.y += dir.y * speed;

            // self.idle = false
        }
        else {
            // self.idle = true
        }
    }

    pub fn render(&self, camera: &Camera, assets: &Assets, surf: &mut Surface) {
        let pos = camera.isof(self.x, self.y, self.z);

        surf.draw_texture(&assets.player, pos.0 as i32 + 4, pos.1 as i32 - assets.player.height, Color::WHITE);
    }
}
