use engine_test::main_game_loop::MainGameLoop;
//use std::ffi::CStr;
/// Macro to get c strings from literals without runtime overhead
/// Literal must not contain any interior nul bytes!
macro_rules! c_str {
    ($literal:expr) => {
        CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    };
}
mod engine_test;
mod render_engine;
mod shaders;
mod textures;
mod models;
mod entities;
mod toolbox;
mod terrains;
mod obj_converter;
mod guis;
mod skybox;

fn main() {
    let mut mge = MainGameLoop::new();
    mge.main_game_loop();
}

