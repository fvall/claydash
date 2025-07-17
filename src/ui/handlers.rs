use super::clay;
use super::layout::chart;
use super::layout::header::{BUTTON_EXIT, BUTTON_RESET};
use super::math;
use super::{is_mouse_pointer_over_element, DropDownState, MenuState, RandomGenerator, State};
use crate::chart::ChartKind;

use rand::Rng;

pub type HandlerFn = unsafe extern "C" fn(clay::Clay_ElementId, clay::Clay_PointerData, isize);
pub unsafe extern "C" fn handle_reset(
    id: clay::Clay_ElementId,
    pointer_data: clay::Clay_PointerData,
    user_data: isize,
) {
    if id.stringId.length == 0 {
        return;
    }

    let state: *mut State = std::ptr::with_exposed_provenance_mut(user_data as usize);
    unsafe {
        if let Some(state) = state.as_mut() {
            let reset = clay::Clay_GetElementId(clay::Clay_String::from_str(BUTTON_RESET));
            if reset.id == id.id {
                let ele_data = clay::Clay_GetElementData(reset);
                if is_mouse_pointer_over_element(ele_data, pointer_data)
                    && (pointer_data.state == clay::ClayPointerDataInteractionState::PressedThisFrame)
                {
                    state.reset();
                }
            }
        }
    }
}

pub unsafe extern "C" fn handle_exit(id: clay::Clay_ElementId, pointer_data: clay::Clay_PointerData, user_data: isize) {
    if id.stringId.length == 0 {
        return;
    }

    let state: *mut State = std::ptr::with_exposed_provenance_mut(user_data as usize);
    unsafe {
        if let Some(state) = state.as_mut() {
            let exit = clay::Clay_GetElementId(clay::Clay_String::from_str(BUTTON_EXIT));
            if exit.id == id.id {
                let ele_data = clay::Clay_GetElementData(exit);
                if super::is_mouse_pointer_over_element(ele_data, pointer_data)
                    && (pointer_data.state == clay::ClayPointerDataInteractionState::ReleasedThisFrame)
                {
                    state.should_close = true;
                }
            }
        }
    }
}

pub unsafe extern "C" fn handle_menu_click(
    _id: clay::Clay_ElementId,
    pointer_data: clay::Clay_PointerData,
    user_data: isize,
) {
    let data: *mut MenuState = std::ptr::with_exposed_provenance_mut(user_data as usize);
    if pointer_data.state == 0 {
        unsafe {
            if let Some(menu_state) = data.as_mut() {
                let is_pressed = menu_state.pressed;
                if let Some(state) = menu_state.parent.as_mut() {
                    state.unclick();
                    menu_state.pressed = !is_pressed;
                }
            }
        }
    }
}

unsafe fn is_mouse_over_menus(state: &State, pointer_data: clay::Clay_PointerData) -> bool {
    unsafe {
        let is_mouse_over_chart_menu = state
            .chart
            .menuid
            .map(|id| clay::Clay_GetElementData(id))
            .map(|ele| super::is_mouse_pointer_over_element(ele, pointer_data))
            .unwrap_or(false);

        let is_mouse_over_dist_menu = state
            .dist
            .menuid
            .map(|id| clay::Clay_GetElementData(id))
            .map(|ele| super::is_mouse_pointer_over_element(ele, pointer_data))
            .unwrap_or(false);

        is_mouse_over_chart_menu || is_mouse_over_dist_menu
    }
}

unsafe fn is_mouse_over_chart(pointer_data: clay::Clay_PointerData) -> bool {
    unsafe {
        let chart = clay::Clay_GetElementId(clay::Clay_String::from_str(chart::CHART_ELEMENT_ID));
        let chart = clay::Clay_GetElementData(chart);
        super::is_mouse_pointer_over_element(chart, pointer_data)
    }
}

pub unsafe extern "C" fn handle_sidebar_click(
    _id: clay::Clay_ElementId,
    pointer_data: clay::Clay_PointerData,
    user_data: isize,
) {
    let data: *mut State = std::ptr::with_exposed_provenance_mut(user_data as usize);
    if let Some(state) = unsafe { data.as_mut() } {
        unsafe {
            if is_mouse_over_menus(state, pointer_data) {
                return;
            }
        }
        handle_unclick(state, pointer_data);
    }
}

#[inline]
fn handle_unclick(state: &mut State, pointer_data: clay::Clay_PointerData) {
    if pointer_data.state == 0 {
        state.unclick();
    }
}

pub unsafe extern "C" fn handle_chart_click(
    _id: clay::Clay_ElementId,
    pointer_data: clay::Clay_PointerData,
    user_data: isize,
) {
    let data: *mut State = std::ptr::with_exposed_provenance_mut(user_data as usize);
    unsafe {
        if let Some(state) = data.as_mut() {
            if !is_mouse_over_menus(state, pointer_data) {
                handle_unclick(state, pointer_data);
            }
        }
    }
}

pub unsafe extern "C" fn handle_canvas_click(
    _id: clay::Clay_ElementId,
    pointer_data: clay::Clay_PointerData,
    user_data: isize,
) {
    unsafe {
        if is_mouse_over_chart(pointer_data) {
            return;
        }
    }
    let data: *mut State = std::ptr::with_exposed_provenance_mut(user_data as usize);
    unsafe {
        if let Some(state) = data.as_mut() {
            if !is_mouse_over_menus(state, pointer_data) {
                handle_unclick(state, pointer_data);
            }
        }
    }
}

pub unsafe extern "C" fn handle_chart_dropdown_menu_item_click(
    _id: clay::Clay_ElementId,
    pointer_data: clay::Clay_PointerData,
    user_data: isize,
) {
    let data: *mut DropDownState = std::ptr::with_exposed_provenance_mut(user_data as usize);
    if pointer_data.state == 0 {
        unsafe {
            let menu = (*data).menu;
            (*menu).title = Some((*data).name);
            (*menu).pressed = false;

            let parent = (*menu).parent;
            (*parent).animation.reset();
            if let Some(ref mut chart_data) = (*parent).chart_data {
                let kind = ChartKind::from_str((*data).name);
                chart_data.kind = kind;
            }
        }
    }
}

pub unsafe extern "C" fn handle_distribution_dropdown_menu_item_click(
    _id: clay::Clay_ElementId,
    pointer_data: clay::Clay_PointerData,
    user_data: isize,
) {
    let data: *mut DropDownState = std::ptr::with_exposed_provenance_mut(user_data as usize);
    if pointer_data.state == 0 {
        unsafe {
            let menu = (*data).menu;
            (*menu).pressed = false;
            let dist = (*data).name;
            if dist == (*menu).title.unwrap_or("") {
                return;
            }

            (*menu).title = Some((*data).name);

            let state = (*menu).parent;
            (*state).animation.reset();

            let seed = (*state).seeder.random::<u64>();
            if dist == "Uniform" {
                let gen_ = math::Uniform::new(seed);
                (*state).generator = RandomGenerator::Uniform(gen_);
                (*state).create_chart_data();
            } else if dist == "Normal" {
                let gen_ = math::Normal::new(seed);
                (*state).generator = RandomGenerator::Normal(gen_);
                (*state).create_chart_data();
            } else if dist == "Gamma" {
                let gen_ = math::Gamma::new(seed, 5, 2.0);
                (*state).generator = RandomGenerator::Gamma(gen_);
                (*state).create_chart_data();
            } else {
                eprintln!("ERROR: Does not know how to handle distribution: '{dist}'");
            }
        }
    }
}

pub unsafe extern "C" fn handle_simulate(
    _id: clay::Clay_ElementId,
    pointer_data: clay::Clay_PointerData,
    user_data: isize,
) {
    let data: *mut State = std::ptr::with_exposed_provenance_mut(user_data as usize);
    if pointer_data.state == 0 {
        unsafe {
            if let Some(state) = data.as_mut() {
                state.simulate();
            }
        }
    }
}
