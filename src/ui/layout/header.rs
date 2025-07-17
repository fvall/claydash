use super::HoverCallback;
use super::State;
use super::clay;
use super::handlers::{handle_chart_click, handle_exit, handle_reset};
use super::misc;
use super::scheme::SchemeUi;
use super::sidebar::{create_chart_menu, create_dist_menu, create_sim_button};
use crate::math;

pub const BUTTON_RESET: &str = "Reset";
pub const BUTTON_EXIT: &str = "Exit";

fn define_header_component(scheme: &SchemeUi) -> clay::Clay_ElementDeclaration {
    let mut border = clay::Clay_BorderElementConfig::default();
    let mut width = clay::Clay_BorderWidth::default();
    width.top = scheme.header.border.width;
    width.bottom = scheme.header.border.width;
    width.left = scheme.header.border.width;
    width.right = scheme.header.border.width;

    border.width = width;
    border.color = scheme.header.border.colour;

    let hpad = 0;
    let vpad = 3;

    clay::ClayElementBuilder::new()
        .with_id("HeaderBar")
        .with_background_color(scheme.header.background)
        .with_layout(
            clay::ClayLayoutBuilder::new()
                .with_sizing(clay::Clay_Sizing {
                    width: clay::Clay_SizingAxis::sizing_grow(0.0),
                    height: clay::Clay_SizingAxis::sizing_fixed(scheme.header.height),
                })
                .with_padding(clay::Clay_Padding { left: hpad, right: hpad, top: vpad, bottom: vpad })
                .with_child_gap(scheme.header.child_gap)
                .with_child_alignment(clay::Clay_ChildAlignment { x: 0, y: clay::ClayChildAlignmentY::Center as u8 })
                .build(),
        )
        .with_corner_radius(clay::Clay_CornerRadius::all(8.0))
        .with_border(border)
        .build()
}

fn define_button(id: &'static str, scheme: &SchemeUi) -> clay::Clay_ElementDeclaration {
    clay::ClayElementBuilder::new()
        .with_id(id)
        .with_background_color(scheme.header.button.default)
        .with_corner_radius(clay::Clay_CornerRadius::all(5.0))
        .with_layout(
            clay::ClayLayoutBuilder::new()
                .with_padding(clay::Clay_Padding { left: 16, right: 16, top: 8, bottom: 8 })
                .with_sizing(clay::Clay_Sizing {
                    width: clay::Clay_SizingAxis::sizing_fixed(0.0),
                    height: clay::Clay_SizingAxis::sizing_grow(0.0),
                })
                .with_child_alignment(clay::Clay_ChildAlignment {
                    x: clay::ClayChildAlignmentX::Center as u8,
                    y: clay::ClayChildAlignmentY::Center as u8,
                })
                .build(),
        )
        .build()
}

fn header_text_config() -> clay::Clay_TextElementConfig {
    let mut cfg = clay::Clay_TextElementConfig::default();
    cfg.fontId = 0;
    cfg.fontSize = 36;
    cfg.textColor = clay::Clay_Color { r: 255.0, g: 0.0, b: 0.0, a: 255.0 };
    cfg.textColor = clay::CLAY_WHITE;
    cfg
}

fn create_header_button(name: &'static str, action: Option<(HoverCallback, isize)>, scheme: &SchemeUi) {
    let cfg = header_text_config();

    let id = name;
    let mut button = define_button(id, scheme);
    if unsafe { clay::Clay_PointerOver(button.id) } {
        // button.border = clay::Clay_BorderElementConfig {
        //     color: scheme.header.button.hover,
        //     width: clay::Clay_BorderWidth { left: 4, right: 4, top: 4, bottom: 4, betweenChildren: 0 },
        // };
        button.backgroundColor = scheme.header.button.hover;
    }

    let txt = clay::Clay_String::from_str(name);
    unsafe {
        clay::clay!(button, clay::clay_text!(txt, cfg), {
            if let Some((fun, param)) = action {
                clay::Clay_OnHover(Some(fun), param);
            }
        });
    }
}

pub unsafe fn create_header(state: &mut State, scheme: &SchemeUi, opaque_state_ptr: isize) {
    // Hate this opaque ptr but what can we do...

    let txt_cfg = header_text_config();
    let text_alignment = clay::ClayChildAlignmentX::Center;
    let within_sidebar = false;
    unsafe {
        clay::clay!(
            define_header_component(scheme),
            clay::Clay_OnHover(Some(handle_chart_click), opaque_state_ptr),
            create_header_button(BUTTON_RESET, Some((handle_reset, opaque_state_ptr)), scheme),
            {
                if state.sidebar_width < math::EPS {
                    create_chart_menu(state, scheme, txt_cfg, text_alignment, within_sidebar);
                    create_dist_menu(state, scheme, txt_cfg, text_alignment, within_sidebar);
                    create_sim_button(scheme, opaque_state_ptr, txt_cfg, text_alignment, within_sidebar);
                }
            },
            clay::clay!(misc::empty_element()),
            create_header_button(BUTTON_EXIT, Some((handle_exit, opaque_state_ptr)), scheme),
        )
    }
}
