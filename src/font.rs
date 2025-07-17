use crate::ffi::raylib::{Font, IsFontValid, LoadFontFromMemory};

// Cannot use a static str in `include_bytes` even if it is constructed at compile time, so using a
// macro to avoid repetition

macro_rules! font_path {
    () => {
        "../resources/Roboto-Regular.ttf"
    };
}
const FONT_PATH: &str = font_path!();
pub const ROBOTO: &[u8] = include_bytes!(font_path!());

pub fn get_font(font_data: &[u8], font_config: &mut Font) -> Result<Font, &'static str> {
    let font = unsafe {
        LoadFontFromMemory(
            ".ttf\0".as_ptr() as *const i8,
            font_data.as_ptr(),
            font_data.len() as i32,
            font_config.baseSize,
            std::ptr::null_mut(),
            font_config.glyphCount,
        )
    };

    unsafe {
        if !IsFontValid(font) {
            eprintln!("Font {:?} is not valid, please check file '{}'", font, font_path!());
            return Err(FONT_PATH);
        }
    };

    Ok(font)
}
