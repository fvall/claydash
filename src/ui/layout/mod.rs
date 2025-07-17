mod canvas;
pub mod chart;
pub mod header;
pub mod misc;
pub mod sidebar;

use super::handlers;

use super::{scheme, DropDownState, HoverCallback, MenuState, State};

use crate::chart::{ChartData, ChartDataHistogram, ChartDataLine, CustomElementKind};
use crate::ffi::{clay, raylib};
use crate::math::{self, cut};

pub type CreateLayoutSignature =
    fn(*mut clay::Clay_Context, std::pin::Pin<&mut crate::ui::State>) -> clay::Clay_RenderCommandArray;

pub fn create_chart_data<R>(gen_: &mut R, hist: &mut ChartDataHistogram, line: &mut ChartDataLine)
where
    R: math::Distribution,
    <R as math::Distribution>::Value: Into<f32> + Copy,
{
    let n = 150;
    let bins = 50;
    line.clear();
    hist.data.clear();

    let sim = gen_.random_owned(n * 1000);
    cut(&sim, bins as u16, &mut hist.data);

    gen_.pdf(&mut line.x, &mut line.y);
}

unsafe fn init_layout(ctx: *mut clay::Clay_Context, state: &mut State, frame_time: f32) -> bool {
    unsafe {
        clay::Clay_SetCurrentContext(ctx);

        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                clay::Clay_SetDebugModeEnabled(crate::DEBUGABLE.load(std::sync::atomic::Ordering::Acquire));
            }
        }
        clay::Clay_SetLayoutDimensions(clay::Clay_Dimensions {
            width: state.width as f32,
            height: state.height as f32,
        });

        cfg_if::cfg_if! {
            if #[cfg(feature = "hot_reload")] {
                match state.font.as_mut() {
                    None => {
                        eprintln!("Font is not defined, cannot initialize the layout");
                        return false;
                    },
                    Some(font) => {
                        let font_config = font as *mut raylib::Font as *mut std::ffi::c_void;
                        clay::Clay_SetMeasureTextFunction(state.measure, font_config);
                    }
                }
            }
        }

        let mouse_position = raylib::GetMousePosition();
        let scroll = raylib::GetMouseWheelMoveV();
        clay::Clay_SetPointerState(mouse_position.into(), raylib::IsMouseButtonDown(0));
        clay::Clay_UpdateScrollContainers(true, scroll.into(), frame_time);
    }
    true
}

pub fn create_layout(
    ctx: *mut clay::Clay_Context,
    state: std::pin::Pin<&mut State>,
    scheme: &scheme::SchemeUi,
) -> clay::Clay_RenderCommandArray {
    // SAFETY: we cannot move state and we really should not
    let state_mut_ref = unsafe { state.get_unchecked_mut() };
    unsafe {
        if !init_layout(ctx, state_mut_ref, raylib::GetFrameTime()) {
            clay::Clay_BeginLayout();
            return clay::Clay_EndLayout();
        }
    }
    state_mut_ref.init();

    // ------------------------------------------------------------------------------------------------------
    // SAFETY: we should not create a &mut from this pointer until the end of the function
    // PROVENANCE: need to use the provenance API - https://doc.rust-lang.org/std/ptr/index.html#provenance
    // ------------------------------------------------------------------------------------------------------

    let chart_data_ptr = match state_mut_ref.chart_data.as_ref() {
        None => std::ptr::null(),
        Some(kind) => kind as *const ChartData,
    };

    state_mut_ref.custom_element = Some(CustomElementKind::Chart(chart_data_ptr));

    let state_ptr = state_mut_ref as *mut State;
    let addr = state_ptr.expose_provenance();
    let addr = addr as isize;

    let section_canvas = canvas::create_canvas(scheme);
    let section_chart = chart::create_chart(state_mut_ref, scheme);

    let content = clay::ClayElementBuilder::new()
        .with_id("Content")
        .with_layout(
            clay::ClayLayoutBuilder::new()
                .with_child_gap(scheme.content.child_gap)
                .with_sizing(misc::LAYOUT_EXPAND)
                .build(),
        )
        .build();

    sidebar::compute_sidebar_width(state_mut_ref);
    unsafe {
        clay::Clay_BeginLayout();
        clay::clay!(
            section_canvas,
            clay::Clay_OnHover(Some(handlers::handle_canvas_click), addr),
            header::create_header(state_mut_ref, scheme, addr),
            clay::clay!(
                content,
                sidebar::create_sidebar(state_mut_ref, scheme, addr),
                clay::clay!(section_chart, clay::Clay_OnHover(Some(handlers::handle_chart_click), addr),)
            )
        );
        clay::Clay_EndLayout()
    }
}
