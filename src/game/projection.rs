use super::Camera;


pub fn isof(x: f32, y: f32, z: f32, xoff: f32, yoff: f32) -> (f32, f32) {
    return (
        xoff + x * 16.0 - y * 16.0,
        yoff + x * 8.0 + y * 8.0 + z
    )
}

pub fn smooth_isof(x: f32, y: f32, z: f32, camera: &Camera) -> (f32, f32) {
    return (
        camera.xoff as f32 + camera.norm_xoff * camera.zoom + x * 16.0 - y * 16.0,
        camera.yoff as f32 + camera.norm_yoff * camera.zoom + x * 8.0 + y * 8.0 + z
    )
}

pub fn iso(x: i32, y: i32, z: i32, xoff: i32, yoff: i32) -> (i32, i32) {
    return (
        xoff + x * 16 - y * 16,
        yoff + x * 8 + y * 8 + z
    )
}
