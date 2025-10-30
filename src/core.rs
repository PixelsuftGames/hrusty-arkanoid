use sdl3_sys::{init, error};
use crate::{deb, fatal};
pub struct CoreContext {}

impl Drop for CoreContext {
    fn drop(&mut self) {
        deb!("Quiting SDL");
        unsafe { init::SDL_Quit() };
    }
}

pub unsafe fn create() -> Option<CoreContext> {
    if !init::SDL_Init(init::SDL_INIT_VIDEO | init::SDL_INIT_EVENTS) {
        fatal!("Failed to init SDL (%s)", error::SDL_GetError());
        return None;
    }
    deb!("SDL inited");
    Some(CoreContext {})
}
