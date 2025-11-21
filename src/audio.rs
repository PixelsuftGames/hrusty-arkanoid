#![allow(non_camel_case_types)]
use crate::fatal;

unsafe extern "C" {
    unsafe fn load_music(path: *const i8) -> i32;
    unsafe fn free_music();
    unsafe fn play_music() -> i32;
    unsafe fn audio_init() -> i32;
    unsafe fn audio_destroy();
}

pub struct AudioContext {}

impl Drop for AudioContext {
    fn drop(&mut self) {
        unsafe {
            audio_destroy();
        }
    }
}

pub unsafe fn create() -> Option<AudioContext> {
    if audio_init() == 0 {
        fatal!("Failed to init audio device");
        return None;
    }
    Some(AudioContext {})
}

pub unsafe fn music_open(path: *const i8) {
    if load_music(path) == 0 {
        panic!("Failed to load music");
    }
}

pub unsafe fn music_close() {
    free_music();
}

pub unsafe fn music_play() {
    play_music();
}
