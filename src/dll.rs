use crate::os::unix::{RTLD_LAZY, dlclose, dlerror, dlopen, dlsym};
use crate::ui::layout::CreateLayoutSignature;
use crate::ui::render::RenderLayoutSignature;

use std::ffi::c_void;

#[derive(Debug)]
pub struct Library {
    pub path: &'static str,
    handle: *mut c_void,
}

impl Library {
    pub fn new(path: &'static str) -> Self {
        let handle = Self::load_lib(path)
            .inspect_err(|err| {
                eprintln!("{err}");
            })
            .unwrap_or(std::ptr::null_mut());
        Self { path, handle }
    }

    // SAFETY: path must be null terminated
    fn load_lib(path: &str) -> Result<*mut c_void, String> {
        let handle = unsafe { dlopen(path.as_ptr() as *const i8, RTLD_LAZY) };

        if handle.is_null() {
            return match Self::read_error_message() {
                None => Err(format!("Could not load library '{path}'")),
                Some(msg) => Err(format!("Could not load library '{path}' due to an error: {msg}")),
            };
        }

        Ok(handle)
    }

    pub fn has_lib(&self) -> bool {
        !self.handle.is_null()
    }

    pub fn read_error_message() -> Option<String> {
        let msg = unsafe { dlerror() };
        if msg.is_null() {
            return None;
        }

        const MAX: u8 = 255;
        let mut len = 0;
        let mut i = 0;
        while unsafe { *msg.offset(i) != ('\0' as i8) } && len < MAX {
            i += 1;
            len += 1;
        }

        if len == 0 {
            return None;
        }

        let slc = unsafe { std::slice::from_raw_parts(msg as *const u8, len as usize) };
        let err = match std::str::from_utf8(slc) {
            Err(err) => format!("The error message is not valid UTF8: {err}"),
            Ok(s) => format!("'{s}'"),
        };

        Some(err)
    }

    pub fn close(&mut self) -> Result<(), String> {
        if self.handle.is_null() {
            return Ok(());
        }

        let retcode = unsafe { dlclose(self.handle) };
        self.handle = std::ptr::null_mut();
        if retcode == 0 {
            return Ok(());
        }

        let err = match Self::read_error_message() {
            None => {
                format!(
                    "There was an error when closing the library '{}' but we could not source the error message",
                    self.path
                )
            }
            Some(msg) => {
                format!("There was an error when closing the library '{}': '{}'", self.path, msg)
            }
        };

        Err(err)
    }

    pub fn reload(&mut self) -> Result<(), String> {
        self.close()?;
        let hdl = Self::load_lib(self.path)?;
        self.handle = hdl;
        Ok(())
    }

    fn get_symbol(&self, symbol: &[u8]) -> Result<*mut c_void, String> {
        if self.handle.is_null() {
            let err = match Self::read_error_message() {
                None => "Unable to load render function".to_string(),
                Some(msg) => format!(
                    "Unable to load dll function as the shared library pointer is NULL, the last error is: '{msg}'",
                ),
            };
            return Err(err);
        }

        // SAFETY: The function must match what we have defined in lib.rs otherwise everything will
        // go kaboon. In addition, it is not clear what happens to the function pointer once
        // the library is reloaded. We should always update the function pointer if the library is
        // reloaded, but to potentially avoid an issue, we are copying the bytes when we transmute
        // the *void to the Rust type
        let fun = unsafe { dlsym(self.handle, symbol.as_ptr() as *const i8) };
        if fun.is_null() {
            let msg = match std::str::from_utf8(symbol) {
                Ok(s) => format!("The symbol '{s}' could not be found in shared library"),
                Err(_) => format!("The symbol {symbol:?} could not be found in shared library"),
            };
            return Err(msg);
        }
        Ok(fun)
    }
    pub fn get_create_layout(&self) -> Result<CreateLayoutSignature, String> {
        let symbol = b"create_layout\0";
        let fun = self.get_symbol(symbol)?;
        let fun: CreateLayoutSignature = unsafe { std::mem::transmute_copy(&fun) };
        Ok(fun)
    }

    pub fn get_render_layout(&self) -> Result<RenderLayoutSignature, String> {
        let symbol = b"render_layout\0";
        let fun = self.get_symbol(symbol)?;
        let fun: RenderLayoutSignature = unsafe { std::mem::transmute_copy(&fun) };
        Ok(fun)
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        if let Err(err) = self.close() {
            eprintln!(
                "Could not close library '{}' during `Drop` implementation due to an error: {}",
                self.path, err
            );
        }
    }
}
