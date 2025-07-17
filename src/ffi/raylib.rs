use super::clay::{Clay_Dimensions, Clay_StringSlice, Clay_TextElementConfig, Clay_Vector2};

use std::ffi::c_void;

pub use crate::bindings::raylib::{
    BeginDrawing, BeginScissorMode, ClearBackground, CloseWindow, Color, ColorBrightness,
    ConfigFlags_FLAG_WINDOW_RESIZABLE, DrawLineEx, DrawLineV, DrawPixelV, DrawRectangle, DrawRectangleLinesEx,
    DrawRectangleRounded, DrawRectangleV, DrawRing, DrawText, DrawTextEx, DrawTriangle, EndDrawing, EndScissorMode,
    Font, GenImageFontAtlas, GetFPS, GetFrameTime, GetMousePosition, GetMouseWheelMoveV, GetScreenHeight,
    GetScreenWidth, InitWindow, IsFontValid, IsKeyDown, IsKeyPressed, IsMouseButtonDown, KeyboardKey_KEY_D,
    KeyboardKey_KEY_M, KeyboardKey_KEY_Q, KeyboardKey_KEY_R, LoadFontData, LoadFontFromMemory, Rectangle,
    SetConfigFlags, SetExitKey, SetTargetFPS, SetTraceLogLevel, TextFormat, TraceLogLevel_LOG_ALL,
    TraceLogLevel_LOG_NONE, Vector2, WindowShouldClose,
};

crate::impl_default!(Font);

impl From<Vector2> for Clay_Vector2 {
    fn from(value: Vector2) -> Self {
        Self { x: value.x, y: value.y }
    }
}

pub unsafe extern "C" fn raylib_measure_text(
    text: Clay_StringSlice,
    config: *mut Clay_TextElementConfig,
    user_data: *mut c_void,
) -> Clay_Dimensions {
    let mut txt_size = Clay_Dimensions::default();

    let font: *mut Font = user_data as *mut Font;
    let font = unsafe {
        match font.as_mut() {
            None => {
                eprintln!("Font pointer is NULL, using default measure text");
                return txt_size;
            }
            Some(f) => {
                if IsFontValid(*f) {
                    f
                } else {
                    eprintln!("Font is invalid, using default measure text");
                    return txt_size;
                }
            }
        }
    };

    let mut max_text_width: f32 = 0.0;
    let mut line_text_width: f32 = 0.0;
    let text_height = unsafe { (*config).fontSize };

    let scale = unsafe { (*config).fontSize as f32 } / (font.baseSize as f32);
    for i in 0..text.length {
        let ch = unsafe { text.chars.offset(i as isize) };
        if ch.is_null() || !ch.is_aligned() {
            continue;
        }

        let bt = unsafe { *ch } as u8;
        if bt <= 31 {
            dbg!("ASCII control character {}", bt);
            continue;
        }
        if bt == b'\n' {
            max_text_width = max_text_width.max(line_text_width);
            line_text_width = 0.0;
            continue;
        }

        let idx = (bt - 32) as isize;
        let glyph = unsafe { font.glyphs.offset(idx) };
        if glyph.is_null() {
            continue;
        }

        let info = unsafe { *glyph };
        if info.advanceX != 0 {
            line_text_width += info.advanceX as f32;
        } else {
            let recs = unsafe { font.recs.offset(idx) };
            if recs.is_null() {
                continue;
            }

            let recs = unsafe { *recs };
            line_text_width += recs.width + info.offsetX as f32;
        }
    }

    max_text_width = max_text_width.max(line_text_width);
    txt_size.width = max_text_width * scale;
    txt_size.height = text_height as f32;

    txt_size
}
