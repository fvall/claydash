use std::convert::TryFrom;
use std::ffi::c_void;
use std::ops::Deref;

pub use crate::bindings::clay::{
    Clay_BeginLayout, Clay_BorderElementConfig, Clay_BorderRenderData, Clay_BorderWidth, Clay_BoundingBox,
    Clay_ChildAlignment, Clay_Color, Clay_Context, Clay_CornerRadius, Clay_CreateArenaWithCapacityAndMemory,
    Clay_CustomElementConfig, Clay_Dimensions, Clay_ElementData, Clay_ElementDeclaration, Clay_ElementId,
    Clay_EndLayout, Clay_ErrorData, Clay_ErrorHandler, Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_CENTER_BOTTOM,
    Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_CENTER_CENTER,
    Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_CENTER_TOP,
    Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_LEFT_BOTTOM,
    Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_LEFT_CENTER,
    Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_LEFT_TOP,
    Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_RIGHT_BOTTOM,
    Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_RIGHT_CENTER,
    Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_RIGHT_TOP, Clay_FloatingAttachPoints,
    Clay_FloatingAttachToElement_CLAY_ATTACH_TO_ELEMENT_WITH_ID, Clay_FloatingAttachToElement_CLAY_ATTACH_TO_NONE,
    Clay_FloatingAttachToElement_CLAY_ATTACH_TO_PARENT, Clay_FloatingAttachToElement_CLAY_ATTACH_TO_ROOT,
    Clay_FloatingElementConfig, Clay_GetElementData, Clay_GetElementId, Clay_ImageElementConfig, Clay_Initialize,
    Clay_LayoutAlignmentX_CLAY_ALIGN_X_CENTER, Clay_LayoutAlignmentX_CLAY_ALIGN_X_LEFT,
    Clay_LayoutAlignmentX_CLAY_ALIGN_X_RIGHT, Clay_LayoutAlignmentY_CLAY_ALIGN_Y_BOTTOM,
    Clay_LayoutAlignmentY_CLAY_ALIGN_Y_CENTER, Clay_LayoutAlignmentY_CLAY_ALIGN_Y_TOP, Clay_LayoutConfig,
    Clay_LayoutDirection, Clay_LayoutDirection_CLAY_LEFT_TO_RIGHT, Clay_LayoutDirection_CLAY_TOP_TO_BOTTOM,
    Clay_MinMemorySize, Clay_OnHover, Clay_Padding, Clay_PointerData,
    Clay_PointerDataInteractionState_CLAY_POINTER_DATA_PRESSED,
    Clay_PointerDataInteractionState_CLAY_POINTER_DATA_PRESSED_THIS_FRAME,
    Clay_PointerDataInteractionState_CLAY_POINTER_DATA_RELEASED,
    Clay_PointerDataInteractionState_CLAY_POINTER_DATA_RELEASED_THIS_FRAME, Clay_PointerOver, Clay_RenderCommand,
    Clay_RenderCommandArray, Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_BORDER,
    Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_CUSTOM, Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_IMAGE,
    Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_NONE, Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_RECTANGLE,
    Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_SCISSOR_END,
    Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_SCISSOR_START,
    Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_TEXT, Clay_ScrollElementConfig, Clay_SetCurrentContext,
    Clay_SetDebugModeEnabled, Clay_SetLayoutDimensions, Clay_SetMeasureTextFunction, Clay_SetPointerState, Clay_Sizing,
    Clay_SizingAxis, Clay_SizingAxis__bindgen_ty_1, Clay_SizingMinMax, Clay_String, Clay_StringSlice,
    Clay_TextElementConfig, Clay_UpdateScrollContainers, Clay_Vector2, Clay__CloseElement, Clay__ConfigureOpenElement,
    Clay__HashString, Clay__OpenElement, Clay__OpenTextElement, Clay__SizingType_CLAY__SIZING_TYPE_FIT,
    Clay__SizingType_CLAY__SIZING_TYPE_FIXED, Clay__SizingType_CLAY__SIZING_TYPE_GROW,
    Clay__SizingType_CLAY__SIZING_TYPE_PERCENT, Clay__StoreTextElementConfig,
};

use crate::{bindings::clay::Clay_Arena, impl_default};
impl_default!(Clay_ChildAlignment);
impl_default!(Clay_Color);
impl_default!(Clay_Dimensions);
impl_default!(Clay_ElementDeclaration);
impl_default!(Clay_ElementId);
impl_default!(Clay_FloatingElementConfig);
impl_default!(Clay_LayoutConfig);
impl_default!(Clay_SizingAxis);
impl_default!(Clay_String);
impl_default!(Clay_TextElementConfig);
impl_default!(Clay_BorderWidth);

impl Default for Clay_BorderElementConfig {
    fn default() -> Self {
        Self {
            color: Clay_Color { r: 255.0, g: 0.0, b: 0.0, a: 255.0 },
            width: Clay_BorderWidth::default(),
        }
    }
}

pub const CLAY_WHITE: Clay_Color = Clay_Color { r: 255.0, g: 255.0, b: 255.0, a: 255.0 };
pub const CLAY_RED: Clay_Color = Clay_Color { r: 255.0, g: 0.0, b: 0.0, a: 255.0 };

pub struct ClayArena<'a> {
    _memory: &'a mut Vec<c_void>,
    pub arena: Clay_Arena,
}

impl<'a> ClayArena<'a> {
    pub fn new(memory: &'a mut Vec<c_void>) -> Self {
        let arena = unsafe { Clay_CreateArenaWithCapacityAndMemory(memory.capacity(), memory.as_mut_ptr()) };
        Self { _memory: memory, arena }
    }
}

pub struct ClayContext<'a> {
    pub arena: ClayArena<'a>,
    pub ctx: *mut Clay_Context,
}

impl<'a> ClayContext<'a> {
    pub fn new(arena: ClayArena<'a>, dim: Clay_Dimensions, handler: Clay_ErrorHandler) -> Self {
        let ctx = unsafe { Clay_Initialize(arena.arena, dim, handler) };
        Self { arena, ctx }
    }
}

impl From<Clay_Color> for super::raylib::Color {
    fn from(value: Clay_Color) -> Self {
        Self {
            r: value.r.round() as u8,
            g: value.g.round() as u8,
            b: value.b.round() as u8,
            a: value.a.round() as u8,
        }
    }
}

impl PartialEq for Clay_Color {
    fn eq(&self, other: &Self) -> bool {
        (self.r == other.r) && (self.g == other.g) && (self.b == other.b) && (self.a == other.a)
    }
}

impl Clay_Padding {
    pub fn padding_all(pad: u16) -> Self {
        Self { left: pad, right: pad, top: pad, bottom: pad }
    }
}

impl Clay_CornerRadius {
    pub fn all(radius: f32) -> Self {
        Self {
            topLeft: radius,
            topRight: radius,
            bottomLeft: radius,
            bottomRight: radius,
        }
    }
}

impl Clay_SizingAxis {
    pub const fn sizing_grow(size: f32) -> Self {
        Clay_SizingAxis {
            size: Clay_SizingAxis__bindgen_ty_1 { minMax: Clay_SizingMinMax { min: 0.0, max: size } },
            type_: ClaySizingType::Grow as u8,
        }
    }
    pub const fn sizing_fixed(size: f32) -> Self {
        Clay_SizingAxis {
            size: Clay_SizingAxis__bindgen_ty_1 { minMax: Clay_SizingMinMax { min: size, max: size } },
            type_: ClaySizingType::Fixed as u8,
        }
    }
}

impl Clay_Sizing {
    pub const fn grow(size: f32) -> Self {
        Self {
            width: Clay_SizingAxis::sizing_grow(size),
            height: Clay_SizingAxis::sizing_grow(size),
        }
    }
    pub fn fixed(size: f32) -> Self {
        Self {
            width: Clay_SizingAxis::sizing_fixed(size),
            height: Clay_SizingAxis::sizing_fixed(size),
        }
    }
}

impl Clay_String {
    pub fn from_str(s: &'static str) -> Self {
        Self {
            isStaticallyAllocated: true,
            length: s.len() as i32,
            chars: s.as_ptr() as *const i8,
        }
    }
}

pub struct ClayElementId<'s> {
    _data: &'s str,
    inner: Clay_ElementId,
}

impl std::ops::Deref for ClayElementId<'_> {
    type Target = Clay_ElementId;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for ClayElementId<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub fn clay_id(id: &str) -> ClayElementId<'_> {
    let inner_string = Clay_String {
        isStaticallyAllocated: true,
        length: id.len() as i32,
        chars: id.as_ptr() as *const i8,
    };

    // SAFETY: safety is at the clay level
    let hash = unsafe { Clay__HashString(inner_string, 0, 0) };
    ClayElementId { _data: id, inner: hash }
}

pub struct ClayElementBuilder {
    ele: Clay_ElementDeclaration,
}

impl ClayElementBuilder {
    pub fn new() -> Self {
        Self { ele: Clay_ElementDeclaration::default() }
    }

    pub fn with_id(mut self, id: &'static str) -> Self {
        let id = clay_id(id);
        self.ele.id = *id.deref();
        self
    }

    pub fn with_layout(mut self, layout: Clay_LayoutConfig) -> Self {
        self.ele.layout = layout;
        self
    }

    pub fn with_background_color(mut self, color: Clay_Color) -> Self {
        self.ele.backgroundColor = color;
        self
    }

    pub fn with_background_colour(mut self, colour: Clay_Color) -> Self {
        self.ele.backgroundColor = colour;
        self
    }

    pub fn with_corner_radius(mut self, corner: Clay_CornerRadius) -> Self {
        self.ele.cornerRadius = corner;
        self
    }

    pub fn with_floating(mut self, floating: Clay_FloatingElementConfig) -> Self {
        self.ele.floating = floating;
        self
    }

    // pub fn with_custom(mut self, custom: ClayCustomElement) -> Self {
    //     self.ele.custom = custom.to_custom_ele_config();
    //     self
    // }

    pub fn with_border(mut self, border: Clay_BorderElementConfig) -> Self {
        self.ele.border = border;
        self
    }
    pub fn build(self) -> Clay_ElementDeclaration {
        self.ele
    }
}

#[repr(u8)]
pub enum ClayPointerDataInteractionState {
    Pressed = Clay_PointerDataInteractionState_CLAY_POINTER_DATA_PRESSED,
    Released = Clay_PointerDataInteractionState_CLAY_POINTER_DATA_RELEASED,
    PressedThisFrame = Clay_PointerDataInteractionState_CLAY_POINTER_DATA_PRESSED_THIS_FRAME,
    ReleasedThisFrame = Clay_PointerDataInteractionState_CLAY_POINTER_DATA_RELEASED_THIS_FRAME,
}

impl From<&ClayPointerDataInteractionState> for &u8 {
    fn from(value: &ClayPointerDataInteractionState) -> Self {
        match value {
            ClayPointerDataInteractionState::Pressed => &(ClayPointerDataInteractionState::Pressed as u8),
            ClayPointerDataInteractionState::Released => &(ClayPointerDataInteractionState::Released as u8),
            ClayPointerDataInteractionState::PressedThisFrame => {
                &(ClayPointerDataInteractionState::PressedThisFrame as u8)
            }
            ClayPointerDataInteractionState::ReleasedThisFrame => {
                &(ClayPointerDataInteractionState::ReleasedThisFrame as u8)
            }
        }
    }
}

impl From<ClayPointerDataInteractionState> for u8 {
    fn from(value: ClayPointerDataInteractionState) -> Self {
        match value {
            ClayPointerDataInteractionState::Pressed => ClayPointerDataInteractionState::Pressed as u8,
            ClayPointerDataInteractionState::Released => ClayPointerDataInteractionState::Released as u8,
            ClayPointerDataInteractionState::PressedThisFrame => {
                ClayPointerDataInteractionState::PressedThisFrame as u8
            }
            ClayPointerDataInteractionState::ReleasedThisFrame => {
                ClayPointerDataInteractionState::ReleasedThisFrame as u8
            }
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ClayLayoutDirection {
    LeftToRight = Clay_LayoutDirection_CLAY_LEFT_TO_RIGHT,
    TopToBottom = Clay_LayoutDirection_CLAY_TOP_TO_BOTTOM,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ClaySizingType {
    Fit = Clay__SizingType_CLAY__SIZING_TYPE_FIT,
    Grow = Clay__SizingType_CLAY__SIZING_TYPE_GROW,
    Percent = Clay__SizingType_CLAY__SIZING_TYPE_PERCENT,
    Fixed = Clay__SizingType_CLAY__SIZING_TYPE_FIXED,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ClayChildAlignmentX {
    Left = Clay_LayoutAlignmentX_CLAY_ALIGN_X_LEFT,
    Right = Clay_LayoutAlignmentX_CLAY_ALIGN_X_RIGHT,
    Center = Clay_LayoutAlignmentX_CLAY_ALIGN_X_CENTER,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ClayChildAlignmentY {
    Top = Clay_LayoutAlignmentY_CLAY_ALIGN_Y_TOP,
    Bottom = Clay_LayoutAlignmentY_CLAY_ALIGN_Y_BOTTOM,
    Center = Clay_LayoutAlignmentY_CLAY_ALIGN_Y_CENTER,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ClayFloatingAttachToElement {
    None = Clay_FloatingAttachToElement_CLAY_ATTACH_TO_NONE,
    Parent = Clay_FloatingAttachToElement_CLAY_ATTACH_TO_PARENT,
    ElementWithId = Clay_FloatingAttachToElement_CLAY_ATTACH_TO_ELEMENT_WITH_ID,
    Root = Clay_FloatingAttachToElement_CLAY_ATTACH_TO_ROOT,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClayFloatingAttachPointType {
    LeftTop = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_LEFT_TOP,
    LeftCenter = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_LEFT_CENTER,
    LeftBottom = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_LEFT_BOTTOM,
    RightTop = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_RIGHT_TOP,
    RightCenter = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_RIGHT_CENTER,
    RightBottom = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_RIGHT_BOTTOM,
    CenterTop = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_CENTER_TOP,
    CenterCenter = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_CENTER_CENTER,
    CenterBottom = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_CENTER_BOTTOM,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClayRenderCommandType {
    None = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_NONE,
    Rectangle = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_RECTANGLE,
    Border = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_BORDER,
    Text = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_TEXT,
    Image = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_IMAGE,
    ScissorStart = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_SCISSOR_START,
    ScissorEnd = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_SCISSOR_END,
    Custom = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_CUSTOM,
}

impl From<ClayRenderCommandType> for u8 {
    fn from(value: ClayRenderCommandType) -> Self {
        match value {
            ClayRenderCommandType::None => ClayRenderCommandType::None as u8,
            ClayRenderCommandType::Rectangle => ClayRenderCommandType::Rectangle as u8,
            ClayRenderCommandType::Border => ClayRenderCommandType::Border as u8,
            ClayRenderCommandType::Text => ClayRenderCommandType::Text as u8,
            ClayRenderCommandType::Image => ClayRenderCommandType::Image as u8,
            ClayRenderCommandType::ScissorStart => ClayRenderCommandType::ScissorStart as u8,
            ClayRenderCommandType::ScissorEnd => ClayRenderCommandType::ScissorEnd as u8,
            ClayRenderCommandType::Custom => ClayRenderCommandType::Custom as u8,
        }
    }
}

impl From<&ClayRenderCommandType> for &u8 {
    fn from(value: &ClayRenderCommandType) -> Self {
        match value {
            ClayRenderCommandType::None => &(ClayRenderCommandType::None as u8),
            ClayRenderCommandType::Rectangle => &(ClayRenderCommandType::Rectangle as u8),
            ClayRenderCommandType::Border => &(ClayRenderCommandType::Border as u8),
            ClayRenderCommandType::Text => &(ClayRenderCommandType::Text as u8),
            ClayRenderCommandType::Image => &(ClayRenderCommandType::Image as u8),
            ClayRenderCommandType::ScissorStart => &(ClayRenderCommandType::ScissorStart as u8),
            ClayRenderCommandType::ScissorEnd => &(ClayRenderCommandType::ScissorEnd as u8),
            ClayRenderCommandType::Custom => &(ClayRenderCommandType::Custom as u8),
        }
    }
}

pub struct ClayRenderCommandTypeError(pub u8);
impl TryFrom<u8> for ClayRenderCommandType {
    type Error = ClayRenderCommandTypeError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            val if val == ClayRenderCommandType::None as u8 => Ok(ClayRenderCommandType::None),
            val if val == ClayRenderCommandType::Rectangle as u8 => Ok(ClayRenderCommandType::Rectangle),
            val if val == ClayRenderCommandType::Border as u8 => Ok(ClayRenderCommandType::Border),
            val if val == ClayRenderCommandType::Text as u8 => Ok(ClayRenderCommandType::Text),
            val if val == ClayRenderCommandType::Image as u8 => Ok(ClayRenderCommandType::Image),
            val if val == ClayRenderCommandType::ScissorStart as u8 => Ok(ClayRenderCommandType::ScissorStart),
            val if val == ClayRenderCommandType::ScissorEnd as u8 => Ok(ClayRenderCommandType::ScissorEnd),
            val if val == ClayRenderCommandType::Custom as u8 => Ok(ClayRenderCommandType::Custom),
            val => Err(ClayRenderCommandTypeError(val)),
        }
    }
}

impl PartialEq<u8> for ClayRenderCommandType {
    fn eq(&self, other: &u8) -> bool {
        let x: &u8 = self.into();
        x == other
    }
}

impl PartialEq<u8> for ClayPointerDataInteractionState {
    fn eq(&self, other: &u8) -> bool {
        let x: &u8 = self.into();
        x == other
    }
}

impl PartialEq<ClayPointerDataInteractionState> for u8 {
    fn eq(&self, other: &ClayPointerDataInteractionState) -> bool {
        other == self
    }
}

pub struct ClayLayoutBuilder {
    cfg: Clay_LayoutConfig,
}

pub struct ClayFloatingBuilder {
    float: Clay_FloatingElementConfig,
}

impl ClayLayoutBuilder {
    pub fn new() -> Self {
        Self { cfg: Clay_LayoutConfig::default() }
    }

    pub fn with_sizing(mut self, sizing: Clay_Sizing) -> Self {
        self.cfg.sizing = sizing;
        self
    }

    pub fn with_padding(mut self, padding: Clay_Padding) -> Self {
        self.cfg.padding = padding;
        self
    }

    pub fn with_child_gap(mut self, gap: u16) -> Self {
        self.cfg.childGap = gap;
        self
    }

    pub fn with_child_alignment(mut self, align: Clay_ChildAlignment) -> Self {
        self.cfg.childAlignment = align;
        self
    }

    pub fn with_layout_direction(mut self, dir: ClayLayoutDirection) -> Self {
        self.cfg.layoutDirection = dir as u8;
        self
    }

    pub fn build(self) -> Clay_LayoutConfig {
        self.cfg
    }
}

impl ClayFloatingBuilder {
    pub fn new() -> Self {
        Self { float: Clay_FloatingElementConfig::default() }
    }

    pub fn with_zindex(mut self, index: i16) -> Self {
        self.float.zIndex = index;
        self
    }

    pub fn with_parent_id(mut self, id: u32) -> Self {
        self.float.parentId = id;
        self
    }

    pub fn with_attach_to(mut self, attach: ClayFloatingAttachToElement) -> Self {
        self.float.attachTo = attach as u8;
        self
    }

    pub fn with_attach_points(
        mut self,
        element: ClayFloatingAttachPointType,
        parent: ClayFloatingAttachPointType,
    ) -> Self {
        self.float.attachPoints = Clay_FloatingAttachPoints { element: element as u8, parent: parent as u8 };
        self
    }
    pub fn build(self) -> Clay_FloatingElementConfig {
        self.float
    }
}
#[macro_export]
macro_rules! clay {
    ($ele: expr $(, $children: expr )* $(,)* ) => {
        {
            $crate::ffi::clay::Clay__OpenElement();
            $crate::ffi::clay::Clay__ConfigureOpenElement($ele);
            $(
                $children;
            )*
            $crate::ffi::clay::Clay__CloseElement();
        }
    };
}

#[macro_export]
macro_rules! clay_text {
    ($text: expr, $cfg: expr) => {{
        {
            let cfg = $crate::ffi::clay::Clay__StoreTextElementConfig($cfg);
            $crate::ffi::clay::Clay__OpenTextElement($text, cfg);
        }
    }};
}

pub use clay;
pub use clay_text;
