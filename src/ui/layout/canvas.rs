use super::clay;
use super::scheme::SchemeUi;

pub fn create_canvas(scheme: &SchemeUi) -> clay::Clay_ElementDeclaration {
    clay::ClayElementBuilder::new()
        .with_id("Canvas")
        .with_background_color(scheme.canvas.background)
        .with_layout(
            clay::ClayLayoutBuilder::new()
                .with_sizing(clay::Clay_Sizing::grow(0.0))
                .with_padding(clay::Clay_Padding::padding_all(scheme.canvas.padding))
                .with_child_gap(scheme.canvas.child_gap)
                .with_layout_direction(clay::ClayLayoutDirection::TopToBottom)
                .build(),
        )
        .build()
}
