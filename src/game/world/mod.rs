use std::sync::Arc;

mod chunk;
use chunk::{Chunk, SIZE};
use dashmap::{DashMap, DashSet};
use fastnoise_lite::{FastNoiseLite, NoiseType};
use tokio::task::spawn_blocking;

use crate::{assets::Assets, context::Context, Surface};

use super::{Camera, Player};

type Pos = (i32, i32);

pub struct World {
    chunks: Arc<DashMap<Pos, Chunk>>,
    generating: Arc<DashSet<Pos>>,
    neighbors: Vec<Pos>,

    noise: Arc<FastNoiseLite>
}

impl World {
    pub fn new() -> Self {
        let chunks = Arc::new(DashMap::new());
        let generating = Arc::new(DashSet::new());
        let neighbors = Self::get_neighbors(6);
        
        let mut noise = FastNoiseLite::new();
        noise.set_noise_type(Some(NoiseType::OpenSimplex2S));
        noise.seed = 0;

        Self { chunks, generating, neighbors, noise: Arc::new(noise) }
    }

    fn get_neighbors(radius: i32) -> Vec<Pos> {
        let mut result = Vec::new();

        for i in -radius..=radius {
            for j in (-radius + i.abs())..=(radius - i.abs()) {
                result.push((i, j));
            }
        }

        result
    }

    pub fn update(&self, player: &mut Player, ctx: &mut Context) {
        let mut to_generate = Vec::new();

        let x = (player.x / (SIZE as f32)).floor() as i32;
        let y = (player.y / (SIZE as f32)).floor() as i32;

        for (nx, ny) in self.neighbors.iter() {
            let cx = x + nx;
            let cy = y + ny;
            let pos = (cx, cy);

            if let Some(chunk) = self.chunks.get(&pos) {
                chunk.update(ctx);
            }
            else {
                to_generate.push(pos);
            }
        }

        // Batch generate missing chunks
        if !to_generate.is_empty() {
            let chunks = Arc::clone(&self.chunks);
            let generating = Arc::clone(&self.generating);
            let noise = Arc::clone(&self.noise);

            tokio::spawn(async move {
                let chunk_batch = spawn_blocking(move || {
                    to_generate
                        .into_iter()
                        .map(
                            |(lx, ly)|
                            (lx, ly, Chunk::new(lx, ly, &noise))
                        )
                        .collect::<Vec<_>>()
                })
                .await
                .unwrap();

                for (cbx, cby, chunk) in chunk_batch {
                    chunks.insert((cbx, cby), chunk);
                    generating.remove(&(cbx, cby));
                }
            });
        }

    }

    pub fn render(&self, player: &mut Player, camera: &Camera, assets: &Assets, surf: &mut Surface) {
        let x = (player.x / (SIZE as f32)).floor() as i32;
        let y = (player.y / (SIZE as f32)).floor() as i32;

        for (nx, ny) in self.neighbors.iter() {
            let cx = x + nx;
            let cy = y + ny;
            let pos = (cx, cy);

            if let Some(chunk) = self.chunks.get(&pos) {
                chunk.render(cx, cy, camera, assets, surf);
            }
        }

        player.render(camera, assets, surf);
    }
}
