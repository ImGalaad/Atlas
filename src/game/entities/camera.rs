use raylib::{prelude::KeyboardKey, math::Vector2, shaders::RaylibShader, RaylibHandle};

use crate::{assets::Assets, context::Context, game::projection::isof};

use super::Player;


const DEFAULT_ZOOM: f32 = 2.0;
const SPEED: f32 = 0.0625;
const ZOOM_SENSITIVITY: f32 = 0.1;

pub struct Camera {
    x: f32,
    y: f32,

    pub xoff: i32,
    pub yoff: i32,
    pub norm_xoff: f32,
    pub norm_yoff: f32,

    pub zoom: f32,
    loc_zoom: i32,
    loc_cam_pos: i32,
}

impl Camera {
    pub fn new(/* state: &State */ w: i32, h: i32, assets: &mut Assets) -> Self {
        let loc_zoom = assets.shader.get_shader_location("zoom");
        assets.shader.set_shader_value(loc_zoom, DEFAULT_ZOOM);

        let loc_cam_pos = assets.shader.get_shader_location("cam_pos");
        let xoff = w / 2;
        let yoff = h / 2;
        assets.shader.set_shader_value(loc_cam_pos, Vector2::new(xoff as f32, yoff as f32));

        Self {
            x: 0.0,
            y: 0.0,

            xoff,
            yoff,
            norm_xoff: 0.0,
            norm_yoff: 0.0,

            zoom: DEFAULT_ZOOM,
            loc_zoom,
            loc_cam_pos
        }
    }

    pub fn update(&mut self, target:&mut  Player, rl: &mut RaylibHandle, ctx: &mut Context, assets: &mut Assets) {
        self.x += (target.x - self.x) * SPEED;
        self.y += (target.y - self.y) * SPEED;

        let pos = isof(-self.x, -self.y, 0.0, ctx.w as f32 / 2.0, ctx.h as f32 / 2.0);
        self.xoff = pos.0 as i32 - 16;  // Player sprite offset
        self.yoff = pos.1 as i32;
        self.norm_xoff = (pos.0 - 16.0) - self.xoff as f32;
        self.norm_yoff = pos.1 - self.yoff as f32;

        assets.shader.set_shader_value(self.loc_cam_pos, Vector2::new(pos.0, -pos.1));

        if rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) {
            let movement = rl.get_mouse_wheel_move();

            if movement != 0.0 {
                self.zoom = f32::clamp(self.zoom + movement + ZOOM_SENSITIVITY, 1.0, 4.0);
                assets.shader.set_shader_value(self.loc_zoom, self.zoom);
            }
        }
    }

    pub fn iso(&self, x: i32, y: i32, z: i32) -> (i32, i32) {
        return (
            self.xoff + x * 16 - y * 16,
            self.yoff + x * 8 + y * 8 + z
        )
    }

    pub fn isof(&self, x: f32, y: f32, z: f32) -> (f32, f32) {
        return (
            self.xoff as f32 + x * 16.0 - y * 16.0,
            self.yoff as f32 + x * 8.0 + y * 8.0 + z
        )
    }

    pub fn smooth_isof(&self, x: f32, y: f32, z: f32) -> (f32, f32) {
        return (
            self.xoff as f32 + self.norm_xoff * self.zoom + x * 16.0 - y * 16.0,
            self.yoff as f32 + self.norm_yoff * self.zoom + x * 8.0 + y * 8.0 + z
        )
    }
}
