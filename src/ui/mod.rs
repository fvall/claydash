mod handlers;
pub mod layout;
pub mod render;
pub mod scheme;

use crate::chart::{ChartData, ChartDataHistogram, ChartDataLine, ChartKind, CustomElementKind};
use crate::ffi::{clay, raylib};
use crate::math::{self, Distribution};
pub use layout::create_layout;
pub use render::render_layout;

use std::time::{SystemTime, UNIX_EPOCH};

use rand::{Rng, SeedableRng};
use render::Animation;

pub mod consts {

    use super::clay;

    pub const DROP_DOWN_HISTOGRAM: &str = "Histogram";
    pub const DROP_DOWN_LINE: &str = "Line";
    pub const DROP_DOWN_HIST_LINE: &str = "Hist+Line";

    pub const BUTTONS: [(&str, clay::Clay_Color); 2] = [
        ("Lorem", clay::Clay_Color { r: 90.0, g: 200.0, b: 90.0, a: 255.0 }),
        ("Ipsum", clay::Clay_Color { r: 200.0, g: 90.0, b: 200.0, a: 255.0 }),
    ];
}

type HoverCallback = unsafe extern "C" fn(clay::Clay_ElementId, clay::Clay_PointerData, isize);
type MeasureFun = unsafe extern "C" fn(
    clay::Clay_StringSlice,
    *mut clay::Clay_TextElementConfig,
    *mut std::ffi::c_void,
) -> clay::Clay_Dimensions;

#[derive(Debug, Clone)]
pub enum RandomGenerator {
    Uniform(math::Uniform),
    Normal(math::Normal),
    Gamma(math::Gamma),
}

impl RandomGenerator {
    pub fn reseed(&mut self, seed: u64) {
        match self {
            Self::Uniform(u) => u.reseed(seed),
            Self::Normal(n) => n.reseed(seed),
            Self::Gamma(n) => n.reseed(seed),
        }
    }

    pub fn seed(&self) -> u64 {
        match self {
            Self::Uniform(u) => u.get_seed(),
            Self::Normal(n) => n.get_seed(),
            Self::Gamma(n) => n.get_seed(),
        }
    }
}

impl Default for RandomGenerator {
    fn default() -> Self {
        Self::Uniform(math::Uniform::default())
    }
}

#[derive(Debug, Clone)]
pub struct MenuState {
    pub title: Option<&'static str>,
    pub menuid: Option<clay::Clay_ElementId>,
    pub dropdown: [DropDownState; 3],
    pub pressed: bool,
    pub parent: *mut State,
}

impl MenuState {
    pub fn init(&mut self) {
        let ptr = self as *mut Self;
        for s in self.dropdown.iter_mut() {
            s.menu = ptr;
        }
    }
    fn dist() -> Self {
        let mut menu = Self::default();
        menu.dropdown[0] = DropDownState::new("Uniform");
        menu.dropdown[1] = DropDownState::new("Normal");
        menu.dropdown[2] = DropDownState::new("Gamma");
        menu.init();
        menu
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DropDownState {
    pub menu: *mut MenuState,
    pub name: &'static str,
}

impl DropDownState {
    pub fn new(name: &'static str) -> Self {
        Self { menu: std::ptr::null_mut(), name }
    }
}

impl Default for MenuState {
    fn default() -> Self {
        let mut menu = Self {
            menuid: None,
            pressed: false,
            title: None,
            dropdown: [
                DropDownState::new(consts::DROP_DOWN_HISTOGRAM),
                DropDownState::new(consts::DROP_DOWN_LINE),
                DropDownState::new(consts::DROP_DOWN_HIST_LINE),
            ],
            parent: std::ptr::null_mut(),
        };

        menu.init();
        menu
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub height: i32,
    pub width: i32,
    pub sidebar_width: f32,
    pub measure: Option<MeasureFun>,
    pub font: Option<raylib::Font>,
    pub chart: MenuState,
    pub dist: MenuState,
    pub generator: RandomGenerator,
    pub chart_data: Option<crate::chart::ChartData>,
    pub custom_element: Option<crate::chart::CustomElementKind>,
    pub should_close: bool,
    pub animation: Animation,
    pub seeder: rand::rngs::SmallRng,
    pub seed: u64,
    pub text_array: Vec<i8>,
    pub chart_x: Vec<f32>,
    pub chart_y: Vec<f32>,
    _pin: std::marker::PhantomPinned, // State is self referential as field menu points to state
}

impl State {
    pub fn unclick(&mut self) {
        self.chart.pressed = false;
        self.dist.pressed = false;
    }
    pub fn reset(&mut self) {
        self.chart = MenuState::default();
        self.chart_data = None;
        self.unclick();
        self.animation.reset();
        self.text_array.clear();
        self.chart_x.clear();
        self.chart_y.clear();
    }

    pub fn init(&mut self) {
        let ptr = self as *mut Self;
        self.chart.parent = ptr;
        self.chart.init();
        self.dist.parent = ptr;
        self.dist.init();
        self.measure = Some(raylib::raylib_measure_text);
    }

    pub fn reseed(&mut self, seed: u64) {
        self.generator.reseed(seed);
    }

    pub fn create_chart_data(&mut self) {
        if let Some(ref mut chart_data) = self.chart_data {
            match &mut self.generator {
                RandomGenerator::Normal(n) => {
                    layout::create_chart_data(n, &mut chart_data.hist, &mut chart_data.line);
                }
                RandomGenerator::Uniform(u) => {
                    layout::create_chart_data(u, &mut chart_data.hist, &mut chart_data.line);
                }
                RandomGenerator::Gamma(g) => {
                    layout::create_chart_data(g, &mut chart_data.hist, &mut chart_data.line);
                }
            }
        } else {
            let mut hist = ChartDataHistogram::default();
            let mut line = ChartDataLine::default();

            match &mut self.generator {
                RandomGenerator::Normal(n) => {
                    layout::create_chart_data(n, &mut hist, &mut line);
                }
                RandomGenerator::Uniform(u) => {
                    layout::create_chart_data(u, &mut hist, &mut line);
                }
                RandomGenerator::Gamma(g) => {
                    layout::create_chart_data(g, &mut hist, &mut line);
                }
            }

            let kind = ChartKind::default();
            let chart_data = ChartData { hist, line, kind };
            self.chart_data = Some(chart_data);
            if let Some(ref mut ele) = self.custom_element {
                match ele {
                    CustomElementKind::Chart(ptr) => {
                        *ptr = self.chart_data.as_ref().unwrap() as *const ChartData;
                    }
                }
            }
        }
    }

    pub fn simulate(&mut self) {
        let seed: u64 = self.seeder.random::<u64>();
        self.reseed(seed);

        self.animation.reset();
        let kind = match self.chart.title {
            None => ChartKind::default(),
            Some(name) => ChartKind::from_str(name),
        };

        self.create_chart_data();
        if let Some(ref mut chart_data) = self.chart_data {
            chart_data.kind = kind;
        }
    }
}

impl Default for State {
    fn default() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|dur| dur.as_secs())
            .unwrap_or(1);

        let mut generator = RandomGenerator::default();
        generator.reseed(seed);

        Self {
            height: 1024,
            width: 1024,
            sidebar_width: layout::sidebar::MAX_SIDEBAR_WIDTH,
            measure: None,
            font: None,
            chart: MenuState::default(),
            dist: MenuState::dist(),
            generator,
            chart_data: None,
            custom_element: None,
            should_close: false,
            text_array: Vec::with_capacity(256),
            chart_x: Vec::with_capacity(256),
            chart_y: Vec::with_capacity(256),
            seed,
            seeder: rand::rngs::SmallRng::seed_from_u64(seed),
            animation: Animation::default(),
            _pin: std::marker::PhantomPinned,
        }
    }
}

pub unsafe extern "C" fn handle_error(data: clay::Clay_ErrorData) {
    if data.errorText.length == 0 {
        eprintln!("There was an error but we cannot print it");
        return;
    }
    let slc = unsafe { std::slice::from_raw_parts(data.errorText.chars as *const u8, data.errorText.length as usize) };

    match std::str::from_utf8(slc) {
        Ok(s) => println!("There was a problem: {s}"),
        Err(_) => println!("There was a problem: cannot convert slice {slc:?} to valid UTF8"),
    }
}

#[inline]
pub fn is_mouse_pointer_over_element(element: clay::Clay_ElementData, pointer_data: clay::Clay_PointerData) -> bool {
    let x = pointer_data.position.x;
    let y = pointer_data.position.y;
    let left = element.boundingBox.x;
    let right = left + element.boundingBox.width;
    let bottom = element.boundingBox.y;
    let top = bottom + element.boundingBox.height;

    (x >= left) && (x <= right) && (y >= bottom) && (y <= top)
}
