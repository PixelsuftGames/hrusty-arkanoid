use crate::{
    audio, cs, fatal,
    hrust::{self, last_error},
    ren::Tex,
    surf, upng,
};
use sdl3_sys::{iostream, pixels, stdinc, surface};

#[derive(Debug, Default)]
pub struct Loader {
    tex: [Tex; 8],
}

static mut handle: *mut Loader = core::ptr::null_mut();

#[inline]
pub unsafe fn l() -> &'static mut Loader {
    &mut *handle
}

pub unsafe fn init(h: &mut Loader) {
    handle = h as *mut Loader;
    // Let's load everything at once
    audio::music_open(cs!("assets/main.mp3"));
    l().tex[0] = load_tex(cs!("assets/bg.png"));
    l().tex[1] = load_tex(cs!("assets/brick.png"));
    l().tex[2] = load_tex(cs!("assets/paddle.png"));
    l().tex[2].pixelate(true);
    l().tex[3] = load_tex(cs!("assets/ball.png"));
    l().tex[3].pixelate(true);
    l().tex[4] = load_tex(cs!("assets/heart.png"));
    l().tex[5] = load_tex(cs!("assets/logo.png"));
    l().tex[6] = load_tex(cs!("assets/space.png"));
}

pub unsafe fn destroy() {
    l().tex[6].destroy();
    l().tex[5].destroy();
    l().tex[4].destroy();
    l().tex[3].destroy();
    l().tex[2].destroy();
    l().tex[1].destroy();
    l().tex[0].destroy();
    audio::music_close();
}

#[inline]
pub unsafe fn get_tex(id: i32) -> &'static mut Tex {
    &mut l().tex[id as usize]
}

unsafe fn upng_error_str(status: upng::upng_error) -> *const i8 {
    match status {
        upng::UPNG_EPARAM => cs!("UPNG_EPARAM"),
        upng::UPNG_EUNFORMAT => cs!("UPNG_EUNFORMAT"),
        upng::UPNG_EUNINTERLACED => cs!("UPNG_EUNINTERLACED"),
        upng::UPNG_EUNSUPPORTED => cs!("UPNG_EUNSUPPORTED"),
        upng::UPNG_EMALFORMED => cs!("UPNG_EMALFORMED"),
        upng::UPNG_ENOTPNG => cs!("UPNG_ENOTPNG"),
        upng::UPNG_ENOTFOUND => cs!("UPNG_ENOTFOUND"),
        upng::UPNG_ENOMEM => cs!("UPNG_ENOMEM"),
        upng::UPNG_EOK => cs!("UPNG_EOK"),
        _ => unreachable!(),
    }
}

pub unsafe fn load_surf(path: *const i8) -> surf::Surface {
    // Load image to memory via upng
    let mut size = 0usize;
    let data = iostream::SDL_LoadFile(path, &mut size as *mut usize);
    if data.is_null() {
        fatal!("Failed to read file %s (%s)", path, hrust::last_error());
        panic!("Failed to read file");
    }
    let mut ret: *mut surface::SDL_Surface = core::ptr::null_mut();
    let h = upng::upng_new_from_bytes(data as *const u8, size as core::ffi::c_ulong);
    if h.is_null() {
        fatal!("Failed to create upng");
    } else {
        let err = upng::upng_decode(h);
        if err == upng::UPNG_EOK {
            ret = surface::SDL_CreateSurface(
                upng::upng_get_width(h) as i32,
                upng::upng_get_height(h) as i32,
                match upng::upng_get_components(h) {
                    // 3 => pixels::SDL_PIXELFORMAT_RGBA8888,
                    4 => pixels::SDL_PIXELFORMAT_RGBA32,
                    _ => unimplemented!(),
                },
            );
            if ret.is_null() {
                fatal!("Failed to create SDL surface (%s)", last_error());
            } else {
                match upng::upng_get_components(h) {
                    4 => {
                        stdinc::SDL_memcpy(
                            (*ret).pixels,
                            upng::upng_get_buffer(h) as *const hrust::c_void,
                            upng::upng_get_size(h) as usize,
                        );
                    }
                    _ => unimplemented!(),
                }
            }
        } else {
            fatal!("Failed to decode png image (%s)", upng_error_str(err));
        }
        upng::upng_free(h);
    }
    stdinc::SDL_free(data as *mut hrust::c_void);
    if ret.is_null() {
        panic!("Failed to load surface");
    }
    surf::Surface::from(ret)
}

pub unsafe fn load_tex(path: *const i8) -> Tex {
    // Load texture from file
    let surf = load_surf(path);
    let mut ret = Tex::from_surf(&surf);
    drop(surf);
    ret.pixelate(false);
    ret
}
