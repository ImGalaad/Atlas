use raylib::prelude::*;


const W: i32 = 1280;
const H: i32 = 720;

mod context;
use context::Context;

mod assets;
use assets::Assets;

mod game;
use game::GameScene;

type Surface<'a> = RaylibTextureMode<'a, RaylibDrawHandle<'a>>;


#[tokio::main]
async fn main() {
    let (mut rl, thread) =
        raylib::init()
            .size(W, H)
            .title("Spicy Journey")
            .resizable()
            .vsync()
            .transparent()
            .build();
    rl.set_exit_key(None);

    let mut assets = Assets::new(&mut rl, &thread);
    let mut ctx = Context::new(W, H, &mut rl, &thread);

    let mut game = GameScene::new(&mut rl, &thread, &ctx, &mut assets);

    while !rl.window_should_close() {
        game.update(&mut rl, &thread, &mut ctx, &mut assets);
        game.render(&mut rl, &thread, &ctx, &assets);
    }
}
