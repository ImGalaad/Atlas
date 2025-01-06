use std::{array, sync::Arc};

use fastnoise_lite::FastNoiseLite;
use raylib::{color::Color, prelude::RaylibDraw};

use crate::{assets::Assets, context::Context, game::Camera, Surface};


pub const SIZE: i32 = 16;

type Tile = i8;

pub struct Chunk {
    pub tiles: [Tile; (SIZE * SIZE) as usize]
}

impl Chunk {
    pub fn new(x: i32, y: i32, noise: &Arc<FastNoiseLite>) -> Self {

        let tiles = array::from_fn(|i| {
            let v1 = noise.get_noise_2d(
                (i as i32 % SIZE + x * SIZE) as f32 * 1.25,
                (i as i32 / SIZE + y * SIZE) as f32 * 1.25
            ) * 128.0;

            let v2 = noise.get_noise_2d(
                (i as i32 % SIZE + x * SIZE) as f32 * 6.0,
                (i as i32 / SIZE + y * SIZE) as f32 * 6.0
            ) * 8.0;

            (v1 + v2) as Tile
        });

        println!("Chunk generated at ({x}, {y})");

        Self { tiles }
    }

    pub fn update(&self, _ctx: &mut Context) {}
    
    pub fn render(&self, cx: i32, cy: i32, camera: &Camera, assets: &Assets, surf: &mut Surface) {
        for x in 0..SIZE {
            for y in 0..SIZE {
                let pos = camera.iso(
                    x + cx * SIZE,
                    y + cy * SIZE,
                    self.tiles[(x + y * SIZE) as usize] as i32,
                );

                surf.draw_texture(&assets.block, pos.0, pos.1, Color::WHITE);
            }
        }
    }

}
