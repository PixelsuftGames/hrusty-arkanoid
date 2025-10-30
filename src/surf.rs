use sdl3_sys::surface;

#[derive(Clone)]
pub struct Surface {
    pub handle: *mut surface::SDL_Surface
}

impl Surface {
    pub unsafe fn from(h: *mut surface::SDL_Surface) -> Surface {
        Surface { handle: h }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            surface::SDL_DestroySurface(self.handle);
        }
    }
}
