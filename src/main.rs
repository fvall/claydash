use clay::error::AppError;
use clay::ffi::{self, raylib};
use clay::ui::layout::CreateLayoutSignature;
use clay::ui::render::RenderLayoutSignature;
use clay::ui::State;
use clay::Library;

use std::pin::Pin;

pub const ANIMATION_DURATION: std::time::Duration = std::time::Duration::from_millis(750);

cfg_if::cfg_if! {
    if #[cfg(feature = "hot_reload")] {
        // Need to append the NULL terminator to be used from C code
        const LIB_PATH_DEBUG: &str = "./target/debug/libclay.so\0";
        const LIB_PATH_RELEASE: &str = "./target/release/libclay.so\0";
    }
}

// unsafe fn from_clay_string(data: &clay::Clay_String) -> Result<&str, std::str::Utf8Error> {
//     let slc = unsafe { std::slice::from_raw_parts(data.chars as *const u8, data.length as usize) };
//     std::str::from_utf8(slc)
// }
//
// unsafe fn from_clay_string_slice(data: &clay::Clay_StringSlice) -> Result<&str, std::str::Utf8Error> {
//     let slc = unsafe { std::slice::from_raw_parts(data.chars as *const u8, data.length as usize) };
//     std::str::from_utf8(slc)
// }

#[cfg(feature = "hot_reload")]
fn get_fn_or_error(
    lib: &mut Library,
    create_fn: &mut CreateLayoutSignature,
    render_fn: &mut RenderLayoutSignature,
) -> Result<(), AppError> {
    match lib.get_create_layout() {
        Err(err) => {
            eprintln!("{err}");
            let fn_name = std::any::type_name_of_val(&clay::create_layout);
            return Err(AppError::MissingFunction(fn_name));
        }
        Ok(fun) => {
            *create_fn = fun;
        }
    }
    match lib.get_render_layout() {
        Err(err) => {
            eprintln!("{err}");
            let fn_name = std::any::type_name_of_val(&clay::render_layout);
            return Err(AppError::MissingFunction(fn_name));
        }
        Ok(fun) => {
            *render_fn = fun;
        }
    }
    Ok(())
}

fn main_loop(
    mem: &mut Vec<std::ffi::c_void>,
    mut state: Pin<&mut State>,
    mut dll: Option<Library>,
) -> Result<(), AppError> {
    let screen_width = 1080;
    let screen_height = 720;
    const LIGHTGRAY: raylib::Color = raylib::Color { r: 100, g: 100, b: 100, a: 255 };
    const FPS: i32 = 60;

    unsafe {
        cfg_if::cfg_if! {
            // this is "the way" to check if we are on debug mode
            // https://users.rust-lang.org/t/conditional-compilation-for-debug-release/1098
            if #[cfg(debug_assertions)] {
                raylib::SetTraceLogLevel(raylib::TraceLogLevel_LOG_ALL as i32);
            } else {
                raylib::SetTraceLogLevel(raylib::TraceLogLevel_LOG_NONE as i32);

            }
        }
        raylib::SetConfigFlags(raylib::ConfigFlags_FLAG_WINDOW_RESIZABLE);
        raylib::InitWindow(screen_width, screen_height, "ClayDash\0".as_ptr() as *const i8);
        raylib::SetTargetFPS(FPS);
        raylib::SetExitKey(raylib::KeyboardKey_KEY_Q as i32);
    };

    let mut first_run = true;
    let ctx = unsafe {
        let state_mut_ref = state.as_mut().get_unchecked_mut();

        // FONT MUST BE LOADED after raylib::InitWindow
        let mut font = raylib::Font::default();
        font.baseSize = clay::SCHEME.font_config.base_size;
        font.glyphCount = clay::SCHEME.font_config.glyph_count;
        font.glyphPadding = clay::SCHEME.font_config.glyph_padding;

        let font = clay::font::get_font(clay::SCHEME.font_data, &mut font).map_err(|s| AppError::InvalidFont(s))?;
        state_mut_ref.font = Some(font);
        clay_initialize(state_mut_ref, mem)
    }?;

    // If hot reloading is not enabled, just compile the function statically as it is defined
    // in lib.rs
    cfg_if::cfg_if! {
        if #[cfg(feature = "hot_reload")] {
            let mut create_layout_fun = clay::create_layout as CreateLayoutSignature;
            let mut render_layout_fun = clay::render_layout as RenderLayoutSignature;
            if let Some(ref mut lib) = dll {
                get_fn_or_error(lib, &mut create_layout_fun, &mut render_layout_fun)?;
            }
        } else {
            let create_layout_fun = clay::create_layout;
            let render_layout_fun = clay::render_layout;
        }
    }

    unsafe {
        while !raylib::WindowShouldClose() && !state.should_close {
            raylib::BeginDrawing();
            raylib::ClearBackground(LIGHTGRAY);
            if first_run {
                first_run = false;
                let state_mut_ref = state.as_mut().get_unchecked_mut();

                // FONT MUST BE LOADED after raylib::InitWindow
                let mut font = raylib::Font::default();
                font.baseSize = clay::SCHEME.font_config.base_size;
                font.glyphCount = clay::SCHEME.font_config.glyph_count;
                font.glyphPadding = clay::SCHEME.font_config.glyph_padding;

                let font =
                    clay::font::get_font(clay::SCHEME.font_data, &mut font).map_err(|s| AppError::InvalidFont(s))?;

                state_mut_ref.font = Some(font);

                std::thread::sleep(std::time::Duration::from_secs_f32(1.0 / (FPS as f32)));
                state_mut_ref.simulate();
            }

            cfg_if::cfg_if! {
                // this is "the way" to check if we are on debug mode
                // https://users.rust-lang.org/t/conditional-compilation-for-debug-release/1098
                if #[cfg(debug_assertions)] {
                    let fps = raylib::GetFPS();
                    eprint!("INFO: Current FPS {}\r", fps);
                }
            }

            {
                let state_mut_ref = state.as_mut().get_unchecked_mut();
                state_mut_ref.height = raylib::GetScreenHeight();
                state_mut_ref.width = raylib::GetScreenWidth();
                state_mut_ref.animation.now = std::time::Instant::now();
            }

            let layout = create_layout_fun(ctx.ctx, state.as_mut());
            match state.font {
                None => return Err(AppError::InvalidFont("Font is not defined")),
                Some(font) => {
                    render_layout_fun(state.as_mut(), layout, font);
                }
            }

            cfg_if::cfg_if! {
                if #[cfg(feature = "hot_reload")] {
                    if raylib::IsKeyPressed(raylib::KeyboardKey_KEY_R as i32) {
                        if let Some(ref mut lib) = dll {
                            if let Err(err) = lib.reload() {
                                eprintln!("There was an error when reloading the library: {err}");
                                return Err(AppError::ReloadError(err));
                            }

                            get_fn_or_error(lib, &mut create_layout_fun, &mut render_layout_fun)?;
                        }
                        {
                            let state_mut_ref = state.as_mut().get_unchecked_mut();
                            state_mut_ref.chart.title = None;

                            let mut font = raylib::Font::default();
                            font.baseSize = clay::SCHEME.font_config.base_size;
                            font.glyphCount = clay::SCHEME.font_config.glyph_count;
                            font.glyphPadding = clay::SCHEME.font_config.glyph_padding;

                            let font = clay::font::get_font(clay::SCHEME.font_data, &mut font).map_err(|s| AppError::InvalidFont(s))?;
                            state_mut_ref.font = Some(font);
                        }
                    }
                }
            }

            cfg_if::cfg_if! {
                if #[cfg(debug_assertions)] {
                    if raylib::IsKeyPressed(raylib::KeyboardKey_KEY_D as i32) {
                        clay::DEBUGABLE.fetch_not(std::sync::atomic::Ordering::SeqCst);
                    }
                }
            }
            raylib::EndDrawing();
        }
        raylib::CloseWindow();
    }

    return Ok(());
}

fn clay_initialize<'s, 'm>(
    state: &'s mut State,
    memory: &'m mut Vec<std::ffi::c_void>,
) -> Result<ffi::clay::ClayContext<'m>, AppError> {
    let dim = ffi::clay::Clay_Dimensions { width: state.width as f32, height: state.height as f32 };
    let handler = ffi::clay::Clay_ErrorHandler {
        errorHandlerFunction: Some(clay::ui::handle_error),
        userData: std::ptr::null_mut(),
    };

    let arena = ffi::clay::ClayArena::new(memory);
    let ctx = ffi::clay::ClayContext::new(arena, dim, handler);
    let font = state
        .font
        .as_mut()
        .ok_or(AppError::InvalidFont("Font is not defined"))? as *mut raylib::Font
        as *mut std::ffi::c_void;

    unsafe {
        ffi::clay::Clay_SetMeasureTextFunction(state.measure, font);
    };

    Ok(ctx)
}

fn main() -> Result<(), AppError> {
    let mut state = State::default();
    state.animation.duration = ANIMATION_DURATION;
    state.init();
    state.font = Some(raylib::Font::default());

    let total_memory = unsafe { ffi::clay::Clay_MinMemorySize() };
    let mut mem = Vec::with_capacity(total_memory as usize);

    let lib: Option<Library>;
    cfg_if::cfg_if! {
        if #[cfg(feature = "hot_reload")] {
            cfg_if::cfg_if! {
                if #[cfg(debug_assertions)] {
                    lib = Some(Library::new(LIB_PATH_DEBUG));
                } else {
                    lib = Some(Library::new(LIB_PATH_RELEASE));
                }
            }

            if !lib.as_ref().map(|inner| inner.has_lib()).unwrap_or(false) {
                if let Some(m) = Library::read_error_message() {
                    eprintln!("There was an error when loading the DLL: {m}");
                }

                match lib {
                    Some(l) => {
                        return Err(AppError::InvalidDll(&l.path[..(l.path.len() - 1)]));
                    },
                    None => unreachable!("Variable lib must be `Some` at this stage"),
                }
            }
        } else {
            lib = None;
        }
    }

    // State is self-referential via field `menu`
    let pin = std::pin::pin!(state);
    main_loop(&mut mem, pin, lib)
}
