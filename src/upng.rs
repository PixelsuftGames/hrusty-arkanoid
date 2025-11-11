#![allow(non_camel_case_types)]
use core::ffi;

pub type upng_error = ffi::c_uint;
pub const UPNG_EPARAM: upng_error = 8;
pub const UPNG_EUNFORMAT: upng_error = 7;
pub const UPNG_EUNINTERLACED: upng_error = 6;
pub const UPNG_EUNSUPPORTED: upng_error = 5;
pub const UPNG_EMALFORMED: upng_error = 4;
pub const UPNG_ENOTPNG: upng_error = 3;
pub const UPNG_ENOTFOUND: upng_error = 2;
pub const UPNG_ENOMEM: upng_error = 1;
pub const UPNG_EOK: upng_error = 0;

pub type upng_format = ffi::c_uint;
pub const UPNG_INDEX8: upng_format = 16;
pub const UPNG_INDEX4: upng_format = 15;
pub const UPNG_INDEX2: upng_format = 14;
pub const UPNG_INDEX1: upng_format = 13;
pub const UPNG_LUMINANCE_ALPHA8: upng_format = 12;
pub const UPNG_LUMINANCE_ALPHA4: upng_format = 11;
pub const UPNG_LUMINANCE_ALPHA2: upng_format = 10;
pub const UPNG_LUMINANCE_ALPHA1: upng_format = 9;
pub const UPNG_LUMINANCE8: upng_format = 8;
pub const UPNG_LUMINANCE4: upng_format = 7;
pub const UPNG_LUMINANCE2: upng_format = 6;
pub const UPNG_LUMINANCE1: upng_format = 5;
pub const UPNG_RGBA16: upng_format = 4;
pub const UPNG_RGBA8: upng_format = 3;
pub const UPNG_RGB16: upng_format = 2;
pub const UPNG_RGB8: upng_format = 1;
pub const UPNG_BADFORMAT: upng_format = 0;

pub type upng_color = ffi::c_uint;
pub const UPNG_RGBA: upng_color = 6;
pub const UPNG_LUMA: upng_color = 4;
pub const UPNG_INDX: upng_color = 3;
pub const UPNG_RGB: upng_color = 2;
pub const UPNG_LUM: upng_color = 0;

pub type upng_state = ffi::c_int;
pub const UPNG_NEW: upng_state = 2;
pub const UPNG_HEADER: upng_state = 1;
pub const UPNG_DECODED: upng_state = 0;
pub const UPNG_ERROR: upng_state = -1;

pub type upng_t = ffi::c_void;

unsafe extern "C" {
    pub unsafe fn upng_new_from_bytes(buffer: *const ffi::c_uchar, size: ffi::c_ulong) -> *mut upng_t;
    pub unsafe fn upng_free(handle: *mut upng_t);
    pub unsafe fn upng_decode(upng: *mut upng_t) -> upng_error;
    pub unsafe fn upng_get_components(upng: *const upng_t) -> ffi::c_uint;
    pub unsafe fn upng_get_width(upng: *const upng_t) -> ffi::c_uint;
    pub unsafe fn upng_get_height(upng: *const upng_t) -> ffi::c_uint;
    pub unsafe fn upng_get_buffer(upng: *const upng_t) -> *const ffi::c_uchar;
    pub unsafe fn upng_get_size(upng: *const upng_t) -> ffi::c_uint;
}
