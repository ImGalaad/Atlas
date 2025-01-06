use raylib::prelude::*;


pub struct Context {
    pub w: i32,
    pub h: i32,
    // pub running: bool
}

impl Context {
    pub fn new(w: i32, h: i32, _rl: &mut RaylibHandle, _thread: &RaylibThread) -> Self {
        Self { w, h }
    }
}
