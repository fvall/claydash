pub use super::clay::Clay_Color;

pub mod colour {
    use super::super::{clay::Clay_Color, raylib};

    pub const LIGHT_GRAY: Clay_Color = Clay_Color { r: 210.0, g: 210.0, b: 210.0, a: 255.0 };
    pub const LIGHT_SLATE_GRAY: Clay_Color = Clay_Color { r: 119.0, g: 136.0, b: 153.0, a: 255.0 };
    pub const SLATE_GRAY: Clay_Color = Clay_Color { r: 112.0, g: 128.0, b: 144.0, a: 255.0 };
    pub const MID_GRAY: Clay_Color = Clay_Color { r: 188.0, g: 189.0, b: 220.0, a: 255.0 };
    pub const BLACK: Clay_Color = Clay_Color { r: 0.0, g: 0.0, b: 0.0, a: 255.0 };
    pub const RED: Clay_Color = Clay_Color { r: 240.0, g: 50.0, b: 50.0, a: 255.0 };
    pub const YELLOW: Clay_Color = Clay_Color { r: 190.0, g: 190.0, b: 90.0, a: 255.0 };
    pub const SKY_BLUE: Clay_Color = Clay_Color { r: 135.0, g: 206.0, b: 250.0, a: 255.0 };
    pub const ROYAL_BLUE: Clay_Color = Clay_Color { r: 65.0, g: 105.0, b: 225.0, a: 255.0 };
    pub const STEEL_BLUE: Clay_Color = Clay_Color { r: 70.0, g: 130.0, b: 180.0, a: 255.0 };
    pub const LIGHT_STEEL_BLUE: Clay_Color = Clay_Color { r: 176.0, g: 196.0, b: 222.0, a: 255.0 };
    pub const DARK_STEEL_BLUE: Clay_Color = Clay_Color { r: 57.0, g: 105.0, b: 145.0, a: 255.0 };
    pub const VERY_DARK_STEEL_BLUE: Clay_Color = Clay_Color { r: 46.0, g: 85.0, b: 117.0, a: 255.0 };
    pub const TOMATO: Clay_Color = Clay_Color { r: 255.0, g: 99.0, b: 71.0, a: 255.0 };
    pub const ORANGE_RED: Clay_Color = Clay_Color { r: 255.0, g: 69.0, b: 0.0, a: 255.0 };
    pub const LAVENDER: Clay_Color = Clay_Color { r: 230.0, g: 230.0, b: 250.0, a: 255.0 };

    // pub const CANVAS: Clay_Color = Clay_Color { r: 43.0, g: 41.0, b: 51.0, a: 255.0 };
    pub const CANVAS: Clay_Color = STEEL_BLUE;
    pub const HEADER: Clay_Color = LIGHT_STEEL_BLUE;
    pub const HEADER_BORDER: Clay_Color = DARK_STEEL_BLUE;
    pub const HEADER_BUTTON: Clay_Color = STEEL_BLUE;
    pub const HEADER_BUTTON_PRESSED: Clay_Color = Clay_Color { r: 240.0, g: 140.0, b: 140.0, a: 255.0 };
    pub const HEADER_BUTTON_HOVER: Clay_Color = DARK_STEEL_BLUE;

    pub const SIDEBAR_BACKGROUND: Clay_Color = LIGHT_STEEL_BLUE;
    // pub const SIDEBAR_BUTTON: Clay_Color = Clay_Color { r: 120.0, g: 120.0, b: 120.0, a: 255.0 };
    pub const SIDEBAR_BUTTON: Clay_Color = STEEL_BLUE;
    pub const SIDEBAR_BUTTON_PRESSED: Clay_Color = Clay_Color { r: 50.0, g: 50.0, b: 50.0, a: 255.0 };
    // pub const SIDEBAR_BUTTON_HOVER: Clay_Color = Clay_Color { r: 50.0, g: 50.0, b: 50.0, a: 255.0 };
    pub const SIDEBAR_BUTTON_HOVER: Clay_Color = DARK_STEEL_BLUE;

    // pub const SIDEBAR_MENU_BUTTON: Clay_Color = Clay_Color { r: 190.0, g: 190.0, b: 90.0, a: 255.0 };
    pub const SIDEBAR_MENU_BUTTON: Clay_Color = DARK_STEEL_BLUE;
    pub const SIDEBAR_MENU_BUTTON_PRESSED: Clay_Color = Clay_Color { r: 190.0, g: 90.0, b: 190.0, a: 255.0 };
    pub const SIDEBAR_MENU_BUTTON_HOVER: Clay_Color = VERY_DARK_STEEL_BLUE;

    pub const SEPARTOR: Clay_Color = Clay_Color { r: 200.0, g: 200.0, b: 0.0, a: 255.0 };
    pub const BACKGROUND: Clay_Color = Clay_Color { r: 90.0, g: 90.0, b: 90.0, a: 255.0 };
    pub const SELECTED: Clay_Color = Clay_Color { r: 190.0, g: 190.0, b: 90.0, a: 255.0 };

    // pub const CHART_BACKGROUND: Clay_Color = Clay_Color { r: 90.0, g: 90.0, b: 190.0, a: 255.0 };
    pub const CHART_BACKGROUND: Clay_Color = LAVENDER;
    pub const LINE: raylib::Color = raylib::Color { r: 44, g: 162, b: 95, a: 255 };
    pub const HIST: raylib::Color = raylib::Color { r: 188, g: 189, b: 220, a: 255 };
}

#[derive(Debug, Clone, Copy)]
pub struct SchemeSidebarButton {
    pub default: Clay_Color,
    pub pressed: Clay_Color,
    pub hover: Clay_Color,
}

#[derive(Debug, Clone, Copy)]
pub struct SchemeHeaderButton {
    pub default: Clay_Color,
    pub pressed: Clay_Color,
    pub hover: Clay_Color,
}

#[derive(Debug, Clone, Copy)]
pub struct SchemeSidebarMenu {
    pub button: SchemeSidebarButton,
}

#[derive(Debug, Clone, Copy)]
pub struct SchemeSidebar {
    pub button: SchemeSidebarButton,
    pub background: Clay_Color,
    pub line: Clay_Color,
    pub chart_menu: SchemeSidebarMenu,
    pub distr_menu: SchemeSidebarMenu,
}

#[derive(Debug, Clone, Copy)]
pub struct SchemeHeaderBorder {
    pub colour: Clay_Color,
    pub width: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct SchemeHeader {
    pub button: SchemeHeaderButton,
    pub background: Clay_Color,
    pub border: SchemeHeaderBorder,
    pub height: f32,
    pub child_gap: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct SchemeCanvas {
    pub background: Clay_Color,
    pub padding: u16,
    pub child_gap: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct SchemeChart {
    pub aes: SchemeChartAesthetics,
    pub layout: SchemeChartLayout,
}

#[derive(Debug, Clone, Copy)]
pub struct SchemeChartLayout {
    pub padding: u16,
    pub child_gap: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct SchemeContent {
    pub child_gap: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct SchemeChartAesthetics {
    pub background: Clay_Color,
    pub fill: Clay_Color,
    pub colour: Clay_Color,
    pub yaxis: Clay_Color,
    pub xaxis: Clay_Color,
}

#[derive(Debug, Clone, Copy)]
pub struct SchemeFont {
    pub base_size: i32,
    pub glyph_count: i32,
    pub glyph_padding: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct SchemeUi {
    pub canvas: SchemeCanvas,
    pub header: SchemeHeader,
    pub content: SchemeContent,
    pub sidebar: SchemeSidebar,
    pub chart: SchemeChart,
    pub font_config: SchemeFont,
    pub font_data: &'static [u8],
}

pub const SCHEME: SchemeUi = SchemeUi {
    sidebar: SchemeSidebar {
        button: SchemeSidebarButton {
            default: colour::SIDEBAR_BUTTON,
            pressed: colour::SIDEBAR_BUTTON_PRESSED,
            hover: colour::SIDEBAR_BUTTON_HOVER,
        },
        background: colour::SIDEBAR_BACKGROUND,
        line: colour::VERY_DARK_STEEL_BLUE,
        chart_menu: SchemeSidebarMenu {
            button: SchemeSidebarButton {
                default: colour::SIDEBAR_MENU_BUTTON,
                pressed: colour::SIDEBAR_MENU_BUTTON_PRESSED,
                hover: colour::SIDEBAR_MENU_BUTTON_HOVER,
            },
        },
        distr_menu: SchemeSidebarMenu {
            button: SchemeSidebarButton {
                default: colour::SIDEBAR_MENU_BUTTON,
                pressed: colour::SIDEBAR_MENU_BUTTON_PRESSED,
                hover: colour::SIDEBAR_MENU_BUTTON_HOVER,
            },
        },
    },
    header: SchemeHeader {
        button: SchemeHeaderButton {
            default: colour::HEADER_BUTTON,
            pressed: colour::HEADER_BUTTON_PRESSED,
            hover: colour::HEADER_BUTTON_HOVER,
        },
        background: colour::HEADER,
        border: SchemeHeaderBorder { colour: colour::HEADER_BORDER, width: 4 },
        height: 50.0,
        child_gap: 16,
    },
    canvas: SchemeCanvas { background: colour::CANVAS, padding: 16, child_gap: 16 },
    chart: SchemeChart {
        aes: SchemeChartAesthetics {
            background: colour::CHART_BACKGROUND,
            fill: colour::LIGHT_STEEL_BLUE,
            colour: colour::TOMATO,
            yaxis: colour::BLACK,
            xaxis: colour::BLACK,
        },
        layout: SchemeChartLayout { child_gap: 8, padding: 20 },
    },
    content: SchemeContent { child_gap: 12 },
    font_config: SchemeFont { base_size: 48, glyph_count: 95, glyph_padding: 2 },
    font_data: crate::font::ROBOTO,
};
