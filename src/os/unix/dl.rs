use std::os::raw::{c_char, c_int, c_void};

pub const RTLD_LAZY: c_int = 1;
unsafe extern "C" {
    pub fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    pub fn dlclose(handle: *mut c_void) -> c_int;
    pub fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    pub fn dlerror() -> *mut c_char;
}
