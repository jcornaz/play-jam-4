#![no_std]

pub mod color;
pub mod image;

extern crate alloc;

use core::ptr;

use color::with_lcd_color;
pub use color::Color;
use image::Image;
use playdate_sys::api;

pub struct Rect {
    pub top_left: [i32; 2],
    pub size: [i32; 2],
}

impl Rect {
    pub fn new(top_left: impl Into<[i32; 2]>, size: impl Into<[i32; 2]>) -> Self {
        Self {
            top_left: top_left.into(),
            size: size.into(),
        }
    }
}

unsafe fn gfx() -> &'static playdate_sys::ffi::playdate_graphics {
    api().unwrap().graphics.as_ref().unwrap()
}

/// Clears the entire display, filling it with `color`
pub fn clear(color: impl Into<Color>) {
    with_lcd_color(color, |color| unsafe { gfx().clear.unwrap()(color) });
}

/// Draws an ellipse inside the rectangle width `lineWidth` from `start_angle` to `end_angle`
///
/// line width is inset from rectangle bounds.
///
/// If `startAngle` != `endAngle`, this draws an arc between the given angles.
/// Angles are given in degrees, clockwise from due north.
pub fn draw_ellipse_with_angle(
    rect: Rect,
    line_width: i32,
    start_angle: f32,
    end_angle: f32,
    color: impl Into<Color>,
) {
    let [x, y] = rect.top_left;
    let [w, h] = rect.size;
    with_lcd_color(color, |color| unsafe {
        gfx().drawEllipse.unwrap()(x, y, w, h, line_width, start_angle, end_angle, color);
    });
}

/// Draws an ellipse inside the rectangle width `lineWidth`
///
/// line width is inset from rectangle bounds.
pub fn draw_ellipse(rect: Rect, line_width: i32, color: impl Into<Color>) {
    draw_ellipse_with_angle(rect, line_width, 0., 360., color);
}

/// Draws a line from `p1` to `p2` with a stroke width of `width`
pub fn draw_line(
    p1: impl Into<[i32; 2]>,
    p2: impl Into<[i32; 2]>,
    width: i32,
    color: impl Into<Color>,
) {
    let [x1, y1] = p1.into();
    let [x2, y2] = p2.into();
    with_lcd_color(color, |color| unsafe {
        gfx().drawLine.unwrap()(x1, y1, x2, y2, width, color)
    });
}

/// Error returned when trying to load an image that cannot be found in the pdx
#[derive(Debug, Copy, Clone)]
pub enum LoadError {
    InvalidPath,
    NotFound,
}

pub fn with_draw_context<'a>(target: impl Into<Option<&'a mut Image>>, draw: impl FnOnce()) {
    let ptr = match target.into() {
        Some(img) => unsafe { img.as_ptr() },
        None => ptr::null_mut(),
    };
    unsafe {
        gfx().pushContext.unwrap()(ptr);
    }
    draw();
    unsafe {
        gfx().popContext.unwrap()();
    }
}
