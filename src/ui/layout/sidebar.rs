use super::clay;
use super::handlers::{
    HandlerFn, handle_chart_dropdown_menu_item_click, handle_distribution_dropdown_menu_item_click, handle_menu_click,
    handle_sidebar_click, handle_simulate,
};
use super::math;
use super::misc::hline_separator;
use super::scheme::SchemeUi;
use super::{DropDownState, MenuState, State};

const CHART_MENU_TITLE_ELEMENT_ID: &str = "ChartMenuTitle";
const CHART_MENU_ELEMENT_ID: &str = "ChartMenu";
const DIST_MENU_TITLE_ELEMENT_ID: &str = "DistMenuTitle";
const DIST_MENU_ELEMENT_ID: &str = "DistMenu";

pub const MIN_SIDEBAR_WIDTH: f32 = 220.0;
pub const MAX_SIDEBAR_WIDTH: f32 = 300.0;
pub const MAX_SIDEBAR_SPACE_RATIO: f32 = 0.15;

fn define_side_bar(scheme: &SchemeUi, width: f32) -> clay::Clay_ElementDeclaration {
    clay::ClayElementBuilder::new()
        .with_id("Sidebar")
        .with_background_color(scheme.sidebar.background)
        .with_layout(
            clay::ClayLayoutBuilder::new()
                .with_child_gap(8)
                .with_padding(clay::Clay_Padding::padding_all(10))
                .with_layout_direction(clay::ClayLayoutDirection::TopToBottom)
                .with_sizing(clay::Clay_Sizing {
                    width: clay::Clay_SizingAxis::sizing_grow(width),
                    height: clay::Clay_SizingAxis::sizing_grow(0.0),
                })
                .build(),
        )
        .build()
}

fn create_sidebar_component(
    scheme: &SchemeUi,
    name: &'static str,
    id: &'static str,
    text_alignment: clay::ClayChildAlignmentX,
    within_sidebar: bool,
) -> (clay::Clay_ElementDeclaration, clay::Clay_String) {
    let layout = clay::ClayLayoutBuilder::new().with_child_alignment(clay::Clay_ChildAlignment {
        x: text_alignment as u8,
        y: clay::ClayChildAlignmentY::Center as u8,
    });

    let layout = if within_sidebar {
        layout
            .with_sizing(clay::Clay_Sizing {
                width: clay::Clay_SizingAxis::sizing_grow(0.0),
                height: clay::Clay_SizingAxis::sizing_fixed(0.0),
            })
            .with_padding(clay::Clay_Padding::padding_all(16))
    } else {
        layout
            .with_sizing(clay::Clay_Sizing {
                width: clay::Clay_SizingAxis::sizing_fixed(0.0),
                height: clay::Clay_SizingAxis::sizing_grow(0.0),
            })
            .with_padding(clay::Clay_Padding { left: 16, right: 16, top: 8, bottom: 8 })
    };

    let layout = layout.build();
    let clay_str = clay::Clay_String::from_str(name);

    let element = clay::ClayElementBuilder::new().with_layout(layout).with_id(id);

    let element = if within_sidebar {
        element
            .with_background_color(scheme.sidebar.button.default)
            .with_corner_radius(clay::Clay_CornerRadius::all(8.0))
            .build()
    } else {
        element
            .with_background_color(scheme.header.button.default)
            .with_corner_radius(clay::Clay_CornerRadius::all(5.0))
            .build()
    };

    (element, clay_str)
}

pub fn create_chart_menu(
    state: &mut State,
    scheme: &SchemeUi,
    txt_cfg: clay::Clay_TextElementConfig,
    text_alignment: clay::ClayChildAlignmentX,
    within_sidebar: bool,
) {
    let menu_title = state.chart.title.unwrap_or(state.chart.dropdown[0].name);
    let menu_ptr = &mut state.chart as *mut MenuState;
    create_dropdown_menu(
        menu_ptr,
        menu_title,
        CHART_MENU_TITLE_ELEMENT_ID,
        CHART_MENU_ELEMENT_ID,
        handle_chart_dropdown_menu_item_click,
        scheme,
        txt_cfg,
        text_alignment,
        within_sidebar,
    );
}

pub fn create_dist_menu(
    state: &mut State,
    scheme: &SchemeUi,
    txt_cfg: clay::Clay_TextElementConfig,
    text_alignment: clay::ClayChildAlignmentX,
    within_sidebar: bool,
) {
    let dist_title = state.dist.title.unwrap_or(state.dist.dropdown[0].name);
    let dist_ptr = &mut state.dist as *mut MenuState;
    create_dropdown_menu(
        dist_ptr,
        dist_title,
        DIST_MENU_TITLE_ELEMENT_ID,
        DIST_MENU_ELEMENT_ID,
        handle_distribution_dropdown_menu_item_click,
        scheme,
        txt_cfg,
        text_alignment,
        within_sidebar,
    );
}

fn define_sidebar_menu_front(
    scheme: &SchemeUi,
    menu_title: &'static str,
    menu_title_id: &'static str,
    text_alignment: clay::ClayChildAlignmentX,
    within_sidebar: bool,
) -> (clay::Clay_ElementDeclaration, clay::Clay_String) {
    let (mut ele, clay_str) =
        create_sidebar_component(scheme, menu_title, menu_title_id, text_alignment, within_sidebar);
    if unsafe { clay::Clay_PointerOver(ele.id) } {
        ele.backgroundColor = scheme.sidebar.button.hover;
    }

    (ele, clay_str)
}

fn create_dropdown_menu(
    state: *mut MenuState,
    menu_title: &'static str,
    menu_title_id: &'static str,
    menu_id: &'static str,
    dropdown_item_handler: HandlerFn,
    scheme: &SchemeUi,
    txt_cfg: clay::Clay_TextElementConfig,
    text_alignment: clay::ClayChildAlignmentX,
    within_sidebar: bool,
) {
    let (ele, clay_str) = define_sidebar_menu_front(scheme, menu_title, menu_title_id, text_alignment, within_sidebar);

    unsafe {
        if let Some(s) = state.as_mut() {
            s.menuid = Some(ele.id);
        } else {
            eprintln!("Pointer to MenuState is NULL");
            return;
        }
    }

    let menu = clay::ClayElementBuilder::new()
        .with_id(menu_id)
        .with_floating(
            clay::ClayFloatingBuilder::new()
                .with_attach_to(clay::ClayFloatingAttachToElement::Parent)
                .with_attach_points(
                    clay::ClayFloatingAttachPointType::LeftTop,
                    clay::ClayFloatingAttachPointType::LeftBottom,
                )
                .build(),
        )
        .with_layout(
            clay::ClayLayoutBuilder::new()
                .with_padding(clay::Clay_Padding { left: 0, right: 0, top: 8, bottom: 8 })
                .build(),
        )
        .build();

    let menu_options = clay::ClayElementBuilder::new()
        .with_layout(
            clay::ClayLayoutBuilder::new()
                .with_layout_direction(clay::ClayLayoutDirection::TopToBottom)
                .with_sizing(clay::Clay_Sizing {
                    width: clay::Clay_SizingAxis::sizing_fixed(200.0),
                    height: clay::Clay_SizingAxis::default(),
                })
                .build(),
        )
        .with_background_colour(scheme.sidebar.chart_menu.button.default)
        .with_corner_radius(clay::Clay_CornerRadius::all(8.0))
        .build();

    let user_data = state.expose_provenance();
    let user_data = user_data as isize;
    unsafe {
        clay::clay!(
            ele,
            clay::Clay_OnHover(Some(handle_menu_click), user_data),
            clay::clay_text!(clay_str, txt_cfg),
            {
                if (*state).pressed {
                    clay::clay!(
                        menu,
                        clay::clay!(menu_options, {
                            for dd in (*state).dropdown.iter_mut() {
                                create_dropdown_item(dd, dd.name, dropdown_item_handler, scheme);
                            }
                        })
                    )
                }
            }
        )
    }
}

fn create_dropdown_item(state: &mut DropDownState, name: &'static str, handler: HandlerFn, scheme: &SchemeUi) {
    let clay_str = clay::Clay_String::from_str(name);
    let mut txt_cfg = clay::Clay_TextElementConfig::default();
    txt_cfg.fontId = 0;
    txt_cfg.fontSize = 24;
    txt_cfg.textColor = clay::CLAY_WHITE;

    let mut ele = clay::ClayElementBuilder::new()
        .with_id(name)
        .with_layout(
            clay::ClayLayoutBuilder::new()
                .with_sizing(clay::Clay_Sizing {
                    width: clay::Clay_SizingAxis::sizing_grow(0.0),
                    height: clay::Clay_SizingAxis::sizing_fixed(48.0),
                })
                .with_padding(clay::Clay_Padding::padding_all(16))
                .build(),
        )
        .with_corner_radius(clay::Clay_CornerRadius::all(8.0))
        .build();

    if unsafe { clay::Clay_PointerOver(ele.id) } {
        ele.border = clay::Clay_BorderElementConfig {
            color: scheme.sidebar.chart_menu.button.hover,
            width: clay::Clay_BorderWidth { left: 4, right: 4, top: 4, bottom: 4, betweenChildren: 0 },
        };
    }

    let addr = (state as *mut DropDownState).expose_provenance();
    let addr = addr as isize;
    unsafe { clay::clay!(ele, clay::Clay_OnHover(Some(handler), addr), clay::clay_text!(clay_str, txt_cfg),) }
}

pub fn create_sim_button(
    scheme: &SchemeUi,
    opaque_state_ptr: isize,
    cfg: clay::Clay_TextElementConfig,
    text_alignment: clay::ClayChildAlignmentX,
    within_sidebar: bool,
) {
    let (ele, txt) = define_sidebar_menu_front(scheme, "Simulate", "SimulateID", text_alignment, within_sidebar);
    unsafe {
        clay::clay!(
            ele,
            clay::clay_text!(txt, cfg),
            clay::Clay_OnHover(Some(handle_simulate), opaque_state_ptr)
        );
    }
}

pub fn compute_sidebar_width(state: &mut State) {
    let sidebar_width = (state.width as f32) * MAX_SIDEBAR_SPACE_RATIO;

    state.sidebar_width = sidebar_width.min(MAX_SIDEBAR_WIDTH);
    if state.sidebar_width < MIN_SIDEBAR_WIDTH {
        state.sidebar_width = 0.0;
    }
}

pub unsafe fn create_sidebar(state: &mut State, scheme: &SchemeUi, opaque_state_ptr: isize) {
    if state.sidebar_width < math::EPS {
        return;
    }

    let within_sidebar = true;
    let mut txt_cfg = clay::Clay_TextElementConfig::default();
    txt_cfg.fontId = 0;
    txt_cfg.fontSize = 26;
    txt_cfg.textColor = clay::CLAY_WHITE;

    let text_alignment = clay::ClayChildAlignmentX::Left;
    let (menu_header, title_str) =
        create_sidebar_component(scheme, "Clay Dashboard", "MenuTitle", text_alignment, within_sidebar);

    unsafe {
        clay::clay!(
            define_side_bar(scheme, state.sidebar_width),
            clay::clay!(menu_header, clay::clay_text!(title_str, txt_cfg)),
            clay::clay!(hline_separator(scheme.sidebar.line, 4.0, 10.0)),
            create_chart_menu(state, scheme, txt_cfg, text_alignment, within_sidebar),
            create_dist_menu(state, scheme, txt_cfg, text_alignment, within_sidebar),
            create_sim_button(scheme, opaque_state_ptr, txt_cfg, text_alignment, within_sidebar),
            clay::Clay_OnHover(Some(handle_sidebar_click), opaque_state_ptr),
        )
    }
}
