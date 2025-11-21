use crate::{col::Color, deb, error, fatal, hrust::last_error, rect::Point, surf, warn, win};
use core::ffi::c_void;
use sdl3_sys::{properties, render, surface};

pub static mut handle: *mut render::SDL_Renderer = core::ptr::null_mut();

pub struct RenContext {}

impl Drop for RenContext {
    fn drop(&mut self) {
        unsafe {
            render::SDL_DestroyRenderer(handle);
        };
        deb!("Renderer destroyed");
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Tex {
    pub handle: *mut render::SDL_Texture,
    pub size: Point,
}

impl Tex {
    pub unsafe fn from_surf(s: &surf::Surface) -> Tex {
        let ret = render::SDL_CreateTextureFromSurface(handle, s.handle);
        if ret.is_null() {
            error!("Failed to create texture from surface (%s)", last_error());
            panic!("Failed to create texture");
        }
        Tex {
            handle: ret,
            size: Point::new((*s.handle).w as f32, (*s.handle).h as f32),
        }
    }

    pub unsafe fn draw(&mut self, pos: &Point) {
        let mut dst_rect: sdl3_sys::rect::SDL_FRect = Default::default();
        dst_rect.x = pos.x;
        dst_rect.y = pos.y;
        dst_rect.w = self.size.x;
        dst_rect.h = self.size.y;
        render::SDL_RenderTexture(
            handle,
            self.handle,
            core::ptr::null(),
            &dst_rect as *const sdl3_sys::rect::SDL_FRect,
        );
    }

    pub unsafe fn pixelate(&mut self, should_pixelate: bool) {
        if !render::SDL_SetTextureScaleMode(
            self.handle,
            if should_pixelate {
                surface::SDL_SCALEMODE_NEAREST
            } else {
                surface::SDL_SCALEMODE_LINEAR
            },
        ) {
            warn!("Failed to apply texture scale mode (%s)", last_error());
        }
    }

    pub unsafe fn color(&mut self, col: &Color) {
        render::SDL_SetTextureColorModFloat(self.handle, col.r, col.g, col.b);
    }

    pub unsafe fn alpha(&mut self, alpha: f32) {
        render::SDL_SetTextureAlphaModFloat(self.handle, alpha);
    }

    pub unsafe fn destroy(&mut self) {
        #[cfg(debug_assertions)]
        if self.handle.is_null() {
            panic!("Double free");
        }
        render::SDL_DestroyTexture(self.handle);
        #[cfg(debug_assertions)]
        {
            self.handle = core::ptr::null_mut();
        }
    }
}

pub unsafe fn create() -> Option<RenContext> {
    let props = properties::SDL_CreateProperties();
    properties::SDL_SetPointerProperty(
        props,
        render::SDL_PROP_RENDERER_CREATE_WINDOW_POINTER,
        win::handle as *mut c_void,
    );
    properties::SDL_SetNumberProperty(
        props,
        render::SDL_PROP_RENDERER_CREATE_PRESENT_VSYNC_NUMBER,
        1,
    );
    // properties::SDL_SetStringProperty(props, render::SDL_PROP_RENDERER_CREATE_NAME_STRING, cs!("direct3d"));
    handle = render::SDL_CreateRendererWithProperties(props);
    if handle.is_null() {
        fatal!("Failed to create renderer (%s)", last_error());
        properties::SDL_DestroyProperties(props);
        return None;
    }
    properties::SDL_DestroyProperties(props);
    deb!("Renderer created");
    Some(RenContext {})
}

pub unsafe fn get_size() -> Point {
    let mut wb: i32 = 0;
    let mut hb: i32 = 0;
    if !render::SDL_GetRenderOutputSize(handle, &mut wb as *mut i32, &mut hb as *mut i32) {
        warn!("Failed to get render output size (%s)", last_error());
        return Point::new(800f32, 600f32);
    }
    Point::new(wb as f32, hb as f32)
}

pub unsafe fn update_scale() {
    let size = get_size();
    if !render::SDL_SetRenderScale(handle, size.x / 800f32, size.y / 600f32) {
        warn!("Failed to update renderer scale (%s)", last_error());
    }
}

pub unsafe fn apply_draw_color(col: &Color) {
    render::SDL_SetRenderDrawColorFloat(handle, col.r, col.g, col.b, col.a);
    render::SDL_SetRenderDrawBlendMode(handle, if col.a == 1f32 { 0u32 } else { 1u32 });
}

pub unsafe fn present() {
    render::SDL_RenderPresent(handle);
}

pub unsafe fn clear(col: &Color) {
    apply_draw_color(col);
    render::SDL_RenderClear(handle);
}

pub unsafe fn toggle_vsync() {
    let mut buf: i32 = 0;
    if render::SDL_GetRenderVSync(handle, &mut buf as *mut i32) {
        render::SDL_SetRenderVSync(handle, buf ^ 1);
    }
}
