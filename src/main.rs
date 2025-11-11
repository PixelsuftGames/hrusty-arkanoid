#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
mod app;
mod hrust;
mod core;
mod win;
mod ren;
mod col;
mod rect;
mod clock;
mod surf;
mod upng;
mod audio;
mod ldr;
mod scene_base;
mod scene_menu;
mod scene_game;

pub unsafe fn main_func() {
    let _core_ctx = core::create().unwrap();
    let _win_ctx = win::create().unwrap();
    let _ren_ctx = ren::create().unwrap();
    app::init();
    let au_ctx = audio::create().unwrap();
    app::run();
    drop(au_ctx); // Close device before decoder is closed, nice hack!
    app::destroy();
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: i32, _argv: *mut *mut u8) -> i32 {
    main_func();
    0
}
