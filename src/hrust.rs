#[allow(unused_imports)]
use core::{any::type_name, panic::PanicInfo};
pub use core::ffi::{c_void};
pub use sdl3_sys::error::SDL_GetError as last_error;

// Generate a C string at compile time
#[macro_export]
macro_rules! cs {
    ($l:expr) => {
        concat!($l, "\0").as_ptr() as *const i8
    }
}

unsafe extern "C" {
    // Lol but the only one
    pub unsafe fn abort() -> !;
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! deb {
    ($fmt:literal $($args:tt)*) => {{
        unsafe { sdl3_sys::log::SDL_LogInfo(0, crate::cs!($fmt) $($args)*) }
    }};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! deb {
    ($fmt:literal $($args:tt)*) => {{
        
    }};
}

#[macro_export]
macro_rules! info {
    ($fmt:literal $($args:tt)*) => {{
        unsafe { sdl3_sys::log::SDL_LogInfo(0, crate::cs!($fmt) $($args)*) }
    }};
}

#[macro_export]
macro_rules! warn {
    ($fmt:literal $($args:tt)*) => {{
        unsafe { sdl3_sys::log::SDL_LogWarn(0, crate::cs!($fmt) $($args)*) }
    }};
}

#[macro_export]
macro_rules! error {
    ($fmt:literal $($args:tt)*) => {{
        unsafe { sdl3_sys::log::SDL_LogError(0, crate::cs!($fmt) $($args)*) }
    }};
}

#[macro_export]
macro_rules! fatal {
    ($fmt:literal $($args:tt)*) => {{
        unsafe { sdl3_sys::log::SDL_LogCritical(0, crate::cs!($fmt) $($args)*) }
    }};
}

#[cfg(not(test))]
#[panic_handler]
unsafe fn panic(info: &PanicInfo) -> ! {
    // Panic handler
    fatal!("Panicked!");
    if let Some(location) = info.location() {
        info!("File: %.*s:%d", location.file().len(), location.file().as_ptr(), location.line());
    }
    if let Some(message) = info.message().as_str() {
        info!("Text: %.*s", message.len(), message.as_ptr());
    }
    abort();
}

#[unsafe(no_mangle)]
pub unsafe fn rust_eh_personality() {
    // Not needed since we use panic = "abort"
}

pub unsafe fn alloc_ptr<T>() -> *mut T {
    // Dynamically allocate object
    let ret = sdl3_sys::stdinc::SDL_malloc(size_of::<T>());
    if ret.is_null() {
        fatal!("Failed to allocate %.*s", type_name::<T>().len(), type_name::<T>().as_ptr());
        panic!("Failed to SDL_malloc");
    }
    ret as *mut T
}

pub unsafe fn free_ptr<T>(ptr: *mut T) {
    // Free allocated object
    sdl3_sys::stdinc::SDL_free(ptr as *mut c_void);
}
