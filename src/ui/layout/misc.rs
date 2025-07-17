use super::clay;

pub const LAYOUT_EXPAND: clay::Clay_Sizing = clay::Clay_Sizing::grow(0.0);

pub fn empty_element() -> clay::Clay_ElementDeclaration {
    clay::ClayElementBuilder::new()
        .with_layout(clay::ClayLayoutBuilder::new().with_sizing(LAYOUT_EXPAND).build())
        .build()
}

pub fn hline_separator(colour: clay::Clay_Color, min_height: f32, max_height: f32) -> clay::Clay_ElementDeclaration {
    clay::ClayElementBuilder::new()
        .with_layout(
            clay::ClayLayoutBuilder::new()
                .with_sizing(clay::Clay_Sizing {
                    width: clay::Clay_SizingAxis::sizing_grow(0.0),
                    height: clay::Clay_SizingAxis {
                        size: clay::Clay_SizingAxis__bindgen_ty_1 {
                            minMax: clay::Clay_SizingMinMax { min: min_height, max: max_height },
                        },
                        type_: clay::Clay__SizingType_CLAY__SIZING_TYPE_FIXED,
                    },
                })
                .build(),
        )
        .with_background_color(colour)
        .build()
}
