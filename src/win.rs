use sdl3_sys::{video};
use crate::{cs, deb, fatal, hrust::last_error, ldr, warn};

pub static mut handle: *mut video::SDL_Window = core::ptr::null_mut();

pub struct WinContext {}

impl Drop for WinContext {
    fn drop(&mut self) {
        unsafe { video::SDL_DestroyWindow(handle); };
        deb!("Window destroyed");
    }
}

pub unsafe fn create() -> Option<WinContext> {
    handle = video::SDL_CreateWindow(
        cs!("Arkanoid"), 1024, 768,
        video::SDL_WINDOW_HIGH_PIXEL_DENSITY | video::SDL_WINDOW_RESIZABLE | video::SDL_WINDOW_HIDDEN
    );
    if handle.is_null() {
        fatal!("Failed to create window (%s)", last_error());
        return None;
    }
    let win_icon = ldr::load_surf(cs!("assets/icon.png"));
    if !video::SDL_SetWindowIcon(handle, win_icon.handle) {
        warn!("Failed to set window icon (%s)", last_error());
    }
    drop(win_icon);
    video::SDL_SetWindowMinimumSize(handle, 320, 200);
    deb!("Window created");
    Some(WinContext {})
}

pub unsafe fn set_shown(show: bool) {
    let ret = if show { video::SDL_ShowWindow(handle) } else { video::SDL_HideWindow(handle) };
    if !ret {
        warn!("Failed to set window %s (%s)", last_error(), if show { cs!("shown") } else { cs!("hidden") });
    }
}

pub unsafe fn set_title(title: *const i8) {
    video::SDL_SetWindowTitle(handle, title);
}
