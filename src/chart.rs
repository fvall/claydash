use super::ffi::clay;
use super::ui::consts::{DROP_DOWN_HISTOGRAM, DROP_DOWN_HIST_LINE, DROP_DOWN_LINE};

#[derive(Debug, Default, Clone)]
pub struct ChartDataHistogram {
    pub data: Vec<u32>,
}

#[derive(Debug, Default, Clone)]
pub struct ChartDataLine {
    pub x: Vec<f32>,
    pub y: Vec<f32>,
}

impl ChartDataLine {
    pub fn push(&mut self, x: f32, y: f32) {
        self.x.push(x);
        self.y.push(y);
    }

    pub fn clear(&mut self) {
        self.x.clear();
        self.y.clear();
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum ChartKind {
    #[default]
    Hist,
    Line,
    HistLine,
}

impl ChartKind {
    pub fn from_str(s: &str) -> Self {
        if s == DROP_DOWN_HISTOGRAM {
            Self::Hist
        } else if s == DROP_DOWN_LINE {
            Self::Line
        } else if s == DROP_DOWN_HIST_LINE {
            Self::HistLine
        } else {
            eprintln!("ERROR: Unable to match name '{s}' to a valid ChartKind");
            Self::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChartData {
    pub hist: ChartDataHistogram,
    pub line: ChartDataLine,
    pub kind: ChartKind,
}

#[derive(Debug, Clone)]
pub enum CustomElementKind {
    Chart(*const ChartData),
}

#[derive(Debug, Clone)]
pub struct ClayCustomElement {
    ptr: *mut CustomElementKind,
}

impl ClayCustomElement {
    pub fn new(ptr: *mut CustomElementKind) -> Self {
        Self { ptr }
    }
    pub fn to_custom_ele_config(self) -> clay::Clay_CustomElementConfig {
        let ptr = self.ptr as *mut std::ffi::c_void;
        clay::Clay_CustomElementConfig { customData: ptr }
    }
}
