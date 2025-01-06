use raylib::prelude::*;

mod projection;

mod world;
use world::World;

mod entities;
pub use entities::Player;
pub use entities::Camera;

use crate::{assets::Assets, context::Context};

pub struct GameScene {
    world: World,
    player: Player,
    camera: Camera,

    target: RenderTexture2D,
}

impl GameScene {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, ctx: &Context, assets: &mut Assets) -> Self {
        let world = World::new();
        let player = Player::new();
        let camera = Camera::new(ctx.w, ctx.h, assets);

        let target = rl.load_render_texture(&thread, ctx.w as u32, ctx.h as u32).unwrap();

        Self { world, player, camera, target }
    }

    pub fn render(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, _ctx: &Context, assets: &Assets) {
        let fps = rl.get_fps();
    
        let mut d = rl.begin_drawing(thread);
    
        {
            // Target drawing
            let mut rtm = d.begin_texture_mode(thread, &mut self.target);
    
            rtm.clear_background(Color::BLACK);
    
            self.world.render(&mut self.player, &self.camera, &assets, &mut rtm);
        }

        {
            // Target post-effect
            let mut rsm = d.begin_shader_mode(&assets.shader);
    
            rsm.draw_texture(&mut self.target, 0, 0, Color::WHITE);
            // rsm.draw_texture(&mut self.target, (self.camera.norm_xoff * self.camera.zoom) as i32, (self.camera.norm_yoff * self.camera.zoom) as i32, Color::WHITE);
        }
            
        {
            // Viewport drawing
            d.draw_text(&format!("Fps: {fps}"), 20, 20, 20, Color::WHITE);
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, ctx: &mut Context, assets: &mut Assets) {
        let dt = rl.get_frame_time();

        if rl.is_window_resized() {
            let w = rl.get_screen_width();
            let h = rl.get_screen_height();

            ctx.h = h;
            ctx.w = w;

            self.target = rl.load_render_texture(thread, w as u32, h as u32).unwrap();
        }

        self.player.update(rl, dt);
        self.world.update(&mut self.player, ctx);
        self.camera.update(&mut self.player, rl, ctx, assets);
    }
}
