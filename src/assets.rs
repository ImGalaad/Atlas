use raylib::prelude::*;


fn load_texture_from_memory(rl: &mut RaylibHandle, thread: &RaylibThread, data: &[u8]) -> Result<Texture2D, String> {
    let image = raylib::core::texture::Image::load_image_from_mem(".png", data).unwrap();

    let texture = rl.load_texture_from_image(&thread, &image)?;

    Ok(texture)
}

pub struct Assets {
    pub shader: Shader,

    pub block: Texture2D,
    pub player: Texture2D,
}

impl Assets {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let shader = rl.load_shader_from_memory(
            &thread,
            None,
            Some(include_str!("../assets/shaders/main.frag"))
        );

        let block = load_texture_from_memory(rl, thread, include_bytes!("../assets/images/block.png")).unwrap();
        let player = load_texture_from_memory(rl, thread, include_bytes!("../assets/images/player.png")).unwrap();

        Self { shader, block, player }
    }
}
