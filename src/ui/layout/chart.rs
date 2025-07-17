use super::clay;
use super::scheme::SchemeUi;
use super::State;
use crate::chart;

pub const CHART_ELEMENT_ID: &str = "Main";

fn create_chart_element(scheme: &SchemeUi, data: chart::ClayCustomElement) -> clay::Clay_ElementDeclaration {
    let mut ele = clay::ClayElementBuilder::new()
        .with_id(CHART_ELEMENT_ID)
        .with_background_color(scheme.chart.aes.background)
        .with_layout(
            clay::ClayLayoutBuilder::new()
                .with_child_gap(scheme.chart.layout.child_gap)
                .with_padding(clay::Clay_Padding::padding_all(scheme.chart.layout.padding))
                .with_layout_direction(clay::ClayLayoutDirection::TopToBottom)
                .with_sizing(clay::Clay_Sizing::grow(0.0))
                .build(),
        )
        .build();

    ele.custom = data.to_custom_ele_config();
    ele
}

fn create_chart_data_element(state: &mut State) -> chart::ClayCustomElement {
    match state.chart_data.as_ref() {
        None => {
            state.custom_element = None;
            chart::ClayCustomElement::new(std::ptr::null_mut())
        }
        Some(data) => {
            state.custom_element = Some(chart::CustomElementKind::Chart(data as *const chart::ChartData));
            let ptr = state.custom_element.as_mut().unwrap() as *mut chart::CustomElementKind;
            chart::ClayCustomElement::new(ptr)
        }
    }
}

pub fn create_chart(state: &mut State, scheme: &SchemeUi) -> clay::Clay_ElementDeclaration {
    let chart_data = create_chart_data_element(state);
    create_chart_element(scheme, chart_data)
}
