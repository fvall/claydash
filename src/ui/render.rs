use std::convert::TryInto;
use std::pin::Pin;
use std::time::{Duration, Instant};

use super::State;
use crate::chart::{ChartData, ChartDataHistogram, ChartDataLine, ChartKind, CustomElementKind};
use crate::ffi::{clay, raylib};
use crate::math::{clamp, lerp};
use crate::ui::scheme::SchemeUi;

pub type RenderLayoutSignature = fn(Pin<&mut State>, clay::Clay_RenderCommandArray, raylib::Font);

#[derive(Debug, Clone, Copy, Default)]
pub struct ChartCanvas {
    pub xbgn: f32,
    pub xend: f32,
    pub ybgn: f32,
    pub yend: f32,
    pub thick: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Animation {
    pub start: Instant,
    pub now: Instant,
    pub duration: Duration,
}

impl Animation {
    pub fn end(&self) -> Instant {
        self.start + self.duration
    }

    pub fn percentage(&self) -> f32 {
        if self.now >= self.end() {
            return 1.0;
        }

        if self.now <= self.start {
            return 0.0;
        }

        let dur = match self.now.checked_duration_since(self.start) {
            None => unreachable!("Should have checked `now` and `start` before"),
            Some(d) => d.as_secs_f32(),
        };

        let last = self.duration.as_secs_f32();
        if last <= 0.00001 {
            return 1.0;
        }

        clamp(dur / last, 0.0, 1.0)
    }

    pub fn reset(&mut self) {
        let now = Instant::now();
        self.start = now;
        self.now = now;
    }
}

impl Default for Animation {
    fn default() -> Self {
        let now = Instant::now();
        Self { start: now, duration: Duration::from_millis(1000), now }
    }
}

pub fn render_layout(
    state: Pin<&mut State>,
    cmd_array: clay::Clay_RenderCommandArray,
    font: raylib::Font,
    scheme: &SchemeUi,
) {
    let state = unsafe { state.get_unchecked_mut() };
    for item in 0..cmd_array.length {
        // SAFETY: if this causes a problem, it is an error with Clay
        let ptr = unsafe { cmd_array.internalArray.offset(item as isize) };

        // SAFETY: we own the command array so no one should be pointing to the internal data
        // structure, although this does depend on the clay implementation
        let cmd = match unsafe { ptr.as_mut() } {
            None => {
                eprintln!("Item {item} of our render command array is NULL");
                continue;
            }
            Some(rf) => rf,
        };

        let bbox = cmd.boundingBox;
        let cmd_type: Result<clay::ClayRenderCommandType, clay::ClayRenderCommandTypeError> =
            cmd.commandType.try_into();

        match cmd_type {
            Ok(ctype) => match ctype {
                clay::ClayRenderCommandType::Rectangle => {
                    let cfg = unsafe { cmd.renderData.rectangle };
                    draw_rectangle(bbox, cfg.cornerRadius, cfg.backgroundColor);
                }
                clay::ClayRenderCommandType::Text => {
                    let text_data = unsafe { cmd.renderData.text };
                    let txt = &mut state.text_array;
                    txt.clear();
                    for i in 0..text_data.stringContents.length {
                        txt.push(unsafe { *text_data.stringContents.chars.offset(i as isize) });
                    }
                    txt.push('\0' as i8);
                    let pos = raylib::Vector2 { x: bbox.x, y: bbox.y };
                    unsafe {
                        raylib::DrawTextEx(
                            font,
                            txt.as_ptr(),
                            pos,
                            text_data.fontSize as f32,
                            text_data.letterSpacing as f32,
                            text_data.textColor.into(),
                        );
                    }
                }
                clay::ClayRenderCommandType::Border => {
                    let cfg = unsafe { cmd.renderData.border };
                    draw_raylib_border(bbox, cfg);
                }
                clay::ClayRenderCommandType::ScissorStart => unsafe {
                    raylib::BeginScissorMode(
                        bbox.x.round() as i32,
                        bbox.y.round() as i32,
                        bbox.width.round() as i32,
                        bbox.height.round() as i32,
                    )
                },
                clay::ClayRenderCommandType::ScissorEnd => unsafe { raylib::EndScissorMode() },
                clay::ClayRenderCommandType::Custom => unsafe {
                    let custom: *mut CustomElementKind = std::mem::transmute(cmd.renderData.custom.customData);
                    draw_rectangle(bbox, cmd.renderData.custom.cornerRadius, cmd.renderData.custom.backgroundColor);

                    // SAFETY: at this stage there should be no mut refs to the pointer, even
                    // though it was created from a mut ref
                    if let Some(rf) = custom.as_mut() {
                        match rf {
                            CustomElementKind::Chart(data) => {
                                if let Some(ref_data) = data.as_ref() {
                                    draw_chart(state, ref_data, bbox, scheme);
                                }
                            }
                        }
                    }
                },
                other => {
                    eprintln!("Do not know how to render ClayRenderCommandType: {other:?}");
                }
            },
            Err(err) => {
                eprintln!("do not know how to render type: {}", err.0);
            }
        }
    }
}

fn draw_rectangle(bbox: clay::Clay_BoundingBox, corner: clay::Clay_CornerRadius, background_color: clay::Clay_Color) {
    if corner.topLeft > 0.0 {
        let mut radius = corner.topLeft * 2.0;
        if bbox.width > bbox.height {
            radius /= bbox.height;
        } else {
            radius /= bbox.width;
        }

        unsafe {
            raylib::DrawRectangleRounded(
                raylib::Rectangle { x: bbox.x, y: bbox.y, height: bbox.height, width: bbox.width },
                radius,
                8,
                background_color.into(),
            )
        }
    } else {
        unsafe {
            raylib::DrawRectangle(
                bbox.x as i32,
                bbox.y as i32,
                bbox.width as i32,
                bbox.height as i32,
                background_color.into(),
            );
        }
    }
}

fn draw_chart_canvas(canvas: clay::Clay_BoundingBox, scheme: &SchemeUi) -> ChartCanvas {
    let start = 0.02;
    let end = 1.0 - start;
    let mut output = ChartCanvas::default();

    let min_screen = 360.0;
    let max_screen = 1080.0;

    let thick = lerp(canvas.height.min(canvas.width), min_screen, max_screen, 1.5, 3.0);
    let padding = 5.0;
    let y = canvas.y + canvas.height * end;
    let x1 = canvas.x + (canvas.x * start).min(padding);
    let x2 = canvas.x + canvas.width * end;
    output.yend = y - thick;

    let offset = clamp(canvas.height.min(canvas.width) * start, 5.0, 10.0);

    // x axis
    unsafe {
        raylib::DrawLineEx(
            raylib::Vector2 { x: x1, y },
            raylib::Vector2 { x: x2, y },
            thick,
            scheme.chart.aes.xaxis.into(),
        );

        let xtip = (x2 + 1.5).min(canvas.width + canvas.x);
        let xback = xtip - offset * 1.20;
        let yup = y + offset;
        let ydown = y - offset;

        output.xend = xback;
        raylib::DrawTriangle(
            raylib::Vector2 { x: xtip, y },
            raylib::Vector2 { x: xback, y: ydown },
            raylib::Vector2 { x: xback, y: yup },
            scheme.chart.aes.xaxis.into(),
        );
    }

    let x = x1 + (canvas.width * start).max(3.0);
    let y1 = canvas.y + canvas.height * start;
    let y2 = canvas.y + (canvas.height * end).max(canvas.height - padding);
    output.xbgn = x + thick;

    // y axis
    unsafe {
        raylib::DrawLineEx(
            raylib::Vector2 { x, y: y1 },
            raylib::Vector2 { x, y: y2 },
            thick,
            scheme.chart.aes.yaxis.into(),
        );

        let ytip = y1 - 1.5;
        let ydwn = ytip + offset * 1.20;
        let xfwd = x + offset;
        let xbck = x - offset;
        output.ybgn = ydwn;

        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                raylib::DrawLineV(
                    raylib::Vector2 { x: output.xbgn, y: output.ybgn },
                    raylib::Vector2 { x: output.xend, y: output.ybgn },
                    raylib::Color { r: 255, g: 0, b: 255, a: 255 },
                );
            }
        }

        raylib::DrawTriangle(
            raylib::Vector2 { x, y: ytip },
            raylib::Vector2 { x: xbck, y: ydwn },
            raylib::Vector2 { x: xfwd, y: ydwn },
            scheme.chart.aes.yaxis.into(),
        );
    }
    output.thick = thick;
    output
}

fn draw_chart(state: &State, data: &ChartData, canvas: clay::Clay_BoundingBox, scheme: &SchemeUi) {
    let dim = draw_chart_canvas(canvas, scheme);
    match data.kind {
        ChartKind::Hist => draw_histogram(state, &data.hist, dim, scheme),
        ChartKind::Line => draw_line(state, &data.line, dim, scheme),
        ChartKind::HistLine => {
            draw_histogram(state, &data.hist, dim, scheme);
            draw_line(state, &data.line, dim, scheme);
        }
    }
}

fn draw_histogram(state: &State, hist: &ChartDataHistogram, dim: ChartCanvas, scheme: &SchemeUi) {
    let data = hist.data.as_slice();
    if data.is_empty() {
        return;
    }

    let maxval = *data.iter().max().unwrap_or(&0);
    if maxval == 0 {
        return;
    }

    let size = data.len() as f32;
    let incr = 1.0 / size;
    let maxval = maxval as f32;
    let mut xleft = dim.xbgn;
    let width = (dim.xend - dim.xbgn - dim.thick) / size;
    let height = dim.yend - dim.ybgn;

    let mut exit = false;
    let mut fct = 0.0;
    let mut next = fct;
    let pct = state.animation.percentage();
    let fill_color: raylib::Color = scheme.chart.aes.fill.into();
    let edge_color = unsafe { raylib::ColorBrightness(fill_color, -0.5) };
    for value in data.iter() {
        next = (next + incr).min(1.0);
        let mut factor = 1.0;
        if pct < next {
            factor = lerp(pct, fct, next, 0.0, 1.0).sqrt();
            exit = true;
        }

        let actual_height = factor * (*value as f32) * height / maxval;
        let pos = raylib::Vector2 { x: xleft, y: 0.0 * dim.ybgn + (dim.yend - actual_height) };
        let size = raylib::Vector2 { x: width, y: actual_height };
        unsafe {
            let rect = raylib::Rectangle { x: pos.x, y: pos.y, width: size.x, height: size.y };
            raylib::DrawRectangleV(pos, size, fill_color);
            raylib::DrawRectangleLinesEx(rect, 2.5, edge_color);
        }

        if exit {
            break;
        }
        xleft += width + dim.thick * 0.0;
        fct = next;
    }
}
fn draw_line(state: &State, line: &ChartDataLine, dim: ChartCanvas, scheme: &SchemeUi) {
    let x = line.x.as_slice();
    let y = line.y.as_slice();
    if x.len() != y.len() {
        eprintln!("ERROR: The size of vectors `x` and `y` must be the same: {} - {}", x.len(), y.len());
        return;
    }

    if x.is_empty() {
        return;
    }

    let color: raylib::Color = scheme.chart.aes.colour.into();
    let mut minx = f32::MAX;
    let mut miny = f32::MAX;
    let mut maxx = f32::MIN;
    let mut maxy = f32::MIN;

    for (xval, yval) in x.iter().zip(y.iter()) {
        if minx > *xval {
            minx = *xval;
        }
        if maxx < *xval {
            maxx = *xval;
        }
        if miny > *yval {
            miny = *yval;
        }

        if maxy < *yval {
            maxy = *yval;
        }
    }

    if x.len() < 2 {
        let x0 = lerp(x[0], minx, maxx, dim.xbgn, dim.xend);
        let y0 = lerp(y[0], miny, maxy, dim.yend, dim.ybgn); // Raylib's orientation is top-down
        unsafe {
            raylib::DrawPixelV(raylib::Vector2 { x: x0, y: y0 }, color);
        }
        return;
    }

    let mut prev = 0;
    let mut next = 1;

    while prev < next && next < x.len() {
        let factor = (next as f32) / (x.len() as f32);
        let pct = state.animation.percentage();
        if factor > pct {
            break;
        }

        let xprev: f32;
        let xnext: f32;
        let yprev: f32;
        let ynext: f32;

        // The assumption is that if min == max then we are already in the [0.0 0.1] space
        if (maxx - minx) < crate::math::EPS {
            xprev = lerp(x[prev], 0.0, 1.0, dim.xbgn, dim.xend);
            xnext = lerp(x[next], 0.0, 1.0, dim.xbgn, dim.xend);
        } else {
            xprev = lerp(x[prev], minx, maxx, dim.xbgn, dim.xend);
            xnext = lerp(x[next], minx, maxx, dim.xbgn, dim.xend);
        }

        if (maxy - miny) < crate::math::EPS {
            yprev = lerp(y[prev], 0.0, 1.0, dim.yend, dim.ybgn); // Raylib's orientation is top-down
            ynext = lerp(y[next], 0.0, 1.0, dim.yend, dim.ybgn); // Raylib's orientation is top-down
        } else {
            yprev = lerp(y[prev], miny, maxy, dim.yend, dim.ybgn); // Raylib's orientation is top-down
            ynext = lerp(y[next], miny, maxy, dim.yend, dim.ybgn); // Raylib's orientation is top-down
        }

        let lhs = raylib::Vector2 { x: xprev, y: yprev };
        let rhs = raylib::Vector2 { x: xnext, y: ynext };

        unsafe {
            raylib::DrawLineEx(lhs, rhs, 4.0, color);
        }

        prev += 1;
        next += 1;
    }
}

fn draw_raylib_border(bbox: clay::Clay_BoundingBox, cfg: clay::Clay_BorderRenderData) {
    const N_SEGMENTS: i32 = 10;
    if cfg.width.left > 0 {
        unsafe {
            let posx = bbox.x.round() as i32;
            let posy = (bbox.y + cfg.cornerRadius.topLeft).round() as i32;
            let width = cfg.width.left as i32;
            let height = (bbox.height - cfg.cornerRadius.topLeft - cfg.cornerRadius.bottomLeft).round() as i32;
            raylib::DrawRectangle(posx, posy, width, height, cfg.color.into());
        }
    }
    if cfg.width.right > 0 {
        unsafe {
            let posx = (bbox.x + bbox.width).round() as i32 - (cfg.width.right as i32);
            let posy = (bbox.y + cfg.cornerRadius.topRight).round() as i32;
            let width = cfg.width.right as i32;
            let height = (bbox.height - cfg.cornerRadius.topRight - cfg.cornerRadius.bottomRight).round() as i32;
            raylib::DrawRectangle(posx, posy, width, height, cfg.color.into());
        }
    }
    if cfg.width.top > 0 {
        unsafe {
            let posx = (bbox.x + cfg.cornerRadius.topLeft).round() as i32;
            let posy = bbox.y.round() as i32;
            let width = (bbox.width - cfg.cornerRadius.topLeft - cfg.cornerRadius.topRight).round() as i32;
            let height = cfg.width.top as i32;
            raylib::DrawRectangle(posx, posy, width, height, cfg.color.into());
        }
    }
    if cfg.width.bottom > 0 {
        unsafe {
            let posx = (bbox.x + cfg.cornerRadius.bottomLeft).round() as i32;
            let posy = (bbox.y + bbox.height).round() as i32 - (cfg.width.bottom as i32);
            let width = (bbox.width - cfg.cornerRadius.bottomLeft - cfg.cornerRadius.bottomRight).round() as i32;
            let height = cfg.width.bottom as i32;
            raylib::DrawRectangle(posx, posy, width, height, cfg.color.into());
        }
    }
    if cfg.cornerRadius.topLeft > 0.0 {
        unsafe {
            let center = raylib::Vector2 {
                x: (bbox.x + cfg.cornerRadius.topLeft).round(),
                y: (bbox.y + cfg.cornerRadius.topLeft).round(),
            };
            let ir = (cfg.cornerRadius.topLeft - cfg.width.top as f32).round();
            let or = cfg.cornerRadius.topLeft;
            let start_angle = 180.0;
            let end_angle = 270.0;
            raylib::DrawRing(center, ir, or, start_angle, end_angle, N_SEGMENTS, cfg.color.into());
        }
    }
    if cfg.cornerRadius.topRight > 0.0 {
        unsafe {
            let center = raylib::Vector2 {
                x: (bbox.x + bbox.width - cfg.cornerRadius.topRight).round(),
                y: (bbox.y + cfg.cornerRadius.topRight).round(),
            };
            let ir = (cfg.cornerRadius.topRight - cfg.width.top as f32).round();
            let or = cfg.cornerRadius.topRight;
            let start_angle = 270.0;
            let end_angle = 360.0;
            raylib::DrawRing(center, ir, or, start_angle, end_angle, N_SEGMENTS, cfg.color.into());
        }
    }
    if cfg.cornerRadius.bottomLeft > 0.0 {
        unsafe {
            let center = raylib::Vector2 {
                x: (bbox.x + cfg.cornerRadius.bottomLeft).round(),
                y: (bbox.y + bbox.height - cfg.cornerRadius.bottomLeft).round(),
            };
            let ir = (cfg.cornerRadius.bottomLeft - cfg.width.bottom as f32).round();
            let or = cfg.cornerRadius.bottomLeft;
            let start_angle = 90.0;
            let end_angle = 180.0;
            raylib::DrawRing(center, ir, or, start_angle, end_angle, N_SEGMENTS, cfg.color.into());
        }
    }
    if cfg.cornerRadius.bottomRight > 0.0 {
        unsafe {
            let center = raylib::Vector2 {
                x: (bbox.x + bbox.width - cfg.cornerRadius.bottomRight).round(),
                y: (bbox.y + bbox.height - cfg.cornerRadius.bottomRight).round(),
            };
            let ir = (cfg.cornerRadius.bottomRight - cfg.width.bottom as f32).round();
            let or = cfg.cornerRadius.bottomRight;
            let start_angle = 0.1;
            let end_angle = 90.0;
            raylib::DrawRing(center, ir, or, start_angle, end_angle, N_SEGMENTS, cfg.color.into());
        }
    }
}
