use std::mem::MaybeUninit;
mod bindings;
mod chart;
mod dll;
pub mod error;
pub mod ffi;
pub mod font;
pub mod math;
pub mod os;
pub mod ui;

fn zero<T>() -> T {
    let inner = MaybeUninit::<T>::zeroed(); // Creates zero-initialized uninitialized memory
    unsafe { inner.assume_init() }
}

macro_rules! impl_default {
    ($type: ident) => {
        impl Default for $type {
            fn default() -> Self {
                $crate::zero::<Self>()
            }
        }
    };
}

pub static DEBUGABLE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub use dll::Library;
pub(crate) use impl_default;

pub use ui::scheme::SCHEME;

#[unsafe(no_mangle)]
pub fn create_layout(
    ctx: *mut ffi::clay::Clay_Context,
    state: std::pin::Pin<&mut ui::State>,
) -> ffi::clay::Clay_RenderCommandArray {
    ui::create_layout(ctx, state, &SCHEME)
}

#[unsafe(no_mangle)]
pub fn render_layout(
    state: std::pin::Pin<&mut ui::State>,
    layout: ffi::clay::Clay_RenderCommandArray,
    font: ffi::raylib::Font,
) {
    ui::render_layout(state, layout, font, &SCHEME)
}
