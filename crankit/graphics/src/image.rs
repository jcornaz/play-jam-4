use alloc::ffi::CString;
use core::{
    ffi::{c_char, CStr},
    fmt::{Display, Formatter},
    ptr,
};

use playdate_sys::ffi::{LCDBitmap, LCDBitmapDrawMode, LCDBitmapFlip};

use crate::{gfx, with_draw_context, Color, LoadError};

/// An image that can be loaded from file ([`ImageOwned::from_path`) or created in memory ([`ImageOwned::from_size`]) to be drawn on screen.
pub struct Image {
    ptr: *mut LCDBitmap,
}

impl Image {
    /// Returns the size of the image
    ///
    /// # Panics
    ///
    /// Can panic if not on a valid playdate system
    #[must_use]
    pub fn size(&self) -> [i32; 2] {
        let mut size = [0; 2];
        let mut row_bytes = 0;
        let mut mask: *mut u8 = ptr::null_mut();
        let mut data: *mut u8 = ptr::null_mut();
        unsafe {
            gfx().getBitmapData.unwrap()(
                self.ptr,
                ptr::addr_of_mut!(size[0]),
                ptr::addr_of_mut!(size[1]),
                ptr::addr_of_mut!(row_bytes),
                ptr::addr_of_mut!(mask),
                ptr::addr_of_mut!(data),
            );
        }
        size
    }

    /// Draws the image with its upper-left corner at the given `position`
    pub fn draw(&self, position: impl Into<[i32; 2]>) {
        self.draw_with_flip(position, Flip::default())
    }

    /// Draws the image with its center at the given `position`
    pub fn draw_from_center(&self, position: impl Into<[i32; 2]>) {
        let [w, h] = self.size();
        let [mut x, mut y] = position.into();
        x -= w / 2;
        y -= h / 2;
        self.draw([x, y]);
    }

    /// Draws the image with its upper-left corner at the given `position` and [`Flip`] flag
    pub fn draw_with_flip(&self, position: impl Into<[i32; 2]>, flip: Flip) {
        let [x, y] = position.into();
        let flip = flip.into();
        unsafe { gfx().drawBitmap.unwrap()(self.ptr, x, y, flip) }
    }

    /// Draws the image with its upper-left corner at [position] tiled inside the rectangle of [size]
    pub fn draw_tiled(&self, position: impl Into<[i32; 2]>, size: impl Into<[i32; 2]>) {
        self.draw_tiled_with_flip(position, size, Flip::Unflipped);
    }

    /// Draws the image with its upper-left corner at [position] tiled inside the rectangle of [size]
    pub fn draw_tiled_with_flip(
        &self,
        position: impl Into<[i32; 2]>,
        size: impl Into<[i32; 2]>,
        flip: Flip,
    ) {
        let [x, y] = position.into();
        let [w, h] = size.into();
        let flip = flip.into();
        unsafe {
            gfx().tileBitmap.unwrap()(self.ptr, x, y, w, h, flip);
        }
    }

    /// Draws the image rotated by `degrees` around its `center` at `point`
    pub fn draw_rotated_around_center(&self, point: impl Into<[i32; 2]>, degrees: f32) {
        self.draw_rotated(point, degrees, [0.5, 0.5]);
    }

    /// Draws the image rotated by `degrees` with its center as given by proportions `center` at `point`
    ///
    /// that is: if `center` is `[0.5, 0.5`] the center of the image is at `point`, if `center` is `[0.0, 0.0]` the top left corner of the image (before rotation) is at `point`, etc.
    pub fn draw_rotated(
        &self,
        point: impl Into<[i32; 2]>,
        degrees: f32,
        center: impl Into<[f32; 2]>,
    ) {
        self.draw_rotated_and_scaled(point, degrees, center, [1.0, 1.0]);
    }

    /// Draws the image scaled to `scale` then rotated by `degrees` with its center as given by proportions `center` at `point`
    ///
    /// that is: if `center` is `[0.5, 0.5`] the center of the image is at `point`, if `center` is `[0.0, 0.0]` the top left corner of the image (before rotation) is at `point`, etc.
    pub fn draw_rotated_and_scaled(
        &self,
        point: impl Into<[i32; 2]>,
        degrees: f32,
        center: impl Into<[f32; 2]>,
        scale: impl Into<[f32; 2]>,
    ) {
        let [x, y] = point.into();
        let [center_x, center_y] = center.into();
        let [scale_x, scale_y] = scale.into();
        unsafe {
            gfx().drawRotatedBitmap.unwrap()(
                self.ptr, x, y, degrees, center_x, center_y, scale_x, scale_y,
            );
        }
    }

    pub fn split_columns(&self, columns: usize) -> impl DoubleEndedIterator<Item = Image> + '_ {
        let [width, height] = self.size();
        let part_width = (width as usize / columns) as i32;
        (0..columns)
            .map(move |i| i as i32 * -part_width)
            .map(move |shift| {
                let mut image = Image::from_size([part_width, height]);
                with_draw_context(&mut image, || {
                    self.draw([shift, 0]);
                });
                image
            })
    }

    pub(crate) unsafe fn as_ptr(&self) -> *mut LCDBitmap {
        self.ptr
    }
}

impl Image {
    /// Create a new blank image of the given size.
    ///
    /// Use the [`Color::clear`] for background.
    ///
    /// # Panics
    ///
    /// Panic if the playdate API was not initialized (see: [`Playdate::init`](crate::Playdate::init))
    pub fn from_size(size: impl Into<[i32; 2]>) -> Self {
        Self::from_size_and_color(size, Color::clear())
    }

    /// Create a new blank image of the given size and background color.
    ///
    /// # Panics
    ///
    /// Panic if the playdate API was not initialized (see: [`Playdate::init`](crate::Playdate::init))
    pub fn from_size_and_color(size: impl Into<[i32; 2]>, color: impl Into<Color>) -> Self {
        let [w, h] = size.into();
        let ptr = crate::with_lcd_color(color, |color| unsafe {
            gfx().newBitmap.unwrap()(w, h, color)
        });
        Self { ptr }
    }

    /// Load an image from path.
    ///
    /// # Errors
    ///
    /// Returns [`LoadError`] if the image was not found.
    ///
    /// # Panics
    ///
    /// Panic if the playdate API was not initialized (see: [`Playdate::init`](crate::Playdate::init))
    pub fn load(path: &str) -> Result<Self, LoadError> {
        let c_path = CString::new(path).map_err(|_| LoadError::InvalidPath)?;
        let mut outerr: *const c_char = ptr::null_mut();
        unsafe {
            let ptr = gfx().loadBitmap.unwrap()(c_path.as_ptr(), ptr::addr_of_mut!(outerr));
            if !outerr.is_null() {
                let _ = CString::from(CStr::from_ptr(outerr));
            }
            if ptr.is_null() {
                Err(LoadError::NotFound)
            } else {
                Ok(Self { ptr })
            }
        }
    }
}

impl Clone for Image {
    fn clone(&self) -> Self {
        let ptr = unsafe { gfx().copyBitmap.unwrap()(self.ptr) };
        Self { ptr }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { gfx().freeBitmap.unwrap()(self.ptr) }
    }
}

impl Display for LoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            LoadError::InvalidPath => write!(f, "Invalid path"),
            LoadError::NotFound => write!(f, "File not found"),
        }
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum Flip {
    #[default]
    Unflipped,
    FlippedX,
    FlippedY,
    FlippedXY,
}

impl Flip {
    pub fn new(flip_x: bool, flip_y: bool) -> Self {
        match (flip_x, flip_y) {
            (false, false) => Self::Unflipped,
            (true, false) => Self::FlippedX,
            (false, true) => Self::FlippedY,
            (true, true) => Self::FlippedXY,
        }
    }
}

impl From<Flip> for LCDBitmapFlip {
    fn from(value: Flip) -> Self {
        match value {
            Flip::Unflipped => Self::kBitmapUnflipped,
            Flip::FlippedX => Self::kBitmapFlippedX,
            Flip::FlippedY => Self::kBitmapFlippedY,
            Flip::FlippedXY => Self::kBitmapFlippedXY,
        }
    }
}

/// Temporarly sets the mode used for drawing bitmaps and execute [action] before setting the drawing mode back to its default.
///
/// Note that text drawing uses bitmaps, so this affects how fonts are displayed as well.
pub fn with_draw_mode(mode: DrawMode, action: impl FnOnce()) {
    set_draw_mode(mode);
    action();
    set_draw_mode(DrawMode::default());
}

/// Sets the mode used for drawing bitmaps.
///
/// Note that text drawing uses bitmaps, so this affects how fonts are displayed as well.
pub fn set_draw_mode(mode: DrawMode) {
    unsafe { gfx().setDrawMode.unwrap()(mode.into()) }
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum DrawMode {
    /// Images are drawn exactly as they are (black pixels are drawn black and white pixels are drawn white)
    #[default]
    Copy,
    /// Any white portions of an image are drawn transparent (black pixels are drawn black and white pixels are drawn transparent)
    WhiteTransparent,
    /// Any black portions of an image are drawn transparent (black pixels are drawn transparent and white pixels are drawn white)
    BlackTransparent,
    /// All non-transparent pixels are drawn white (black pixels are drawn white and white pixels are drawn white)
    FillWhite,
    /// All non-transparent pixels are drawn black (black pixels are drawn black and white pixels are drawn black)
    FillBlack,
    /// Pixels are drawn inverted on white backgrounds, creating an effect where any white pixels in the original image will always be visible,
    /// regardless of the background color, and any black pixels will appear transparent (on a white background, black pixels are drawn white and white pixels are drawn black)
    XOR,
    /// Pixels are drawn inverted on black backgrounds, creating an effect where any black pixels in the original image will always be visible,
    /// regardless of the background color, and any white pixels will appear transparent (on a black background, black pixels are drawn white and white pixels are drawn black)
    NXOR,
    /// Pixels are drawn inverted (black pixels are drawn white and white pixels are drawn black)
    Inverted,
}

impl From<DrawMode> for LCDBitmapDrawMode {
    fn from(value: DrawMode) -> Self {
        match value {
            DrawMode::Copy => LCDBitmapDrawMode::kDrawModeCopy,
            DrawMode::WhiteTransparent => LCDBitmapDrawMode::kDrawModeWhiteTransparent,
            DrawMode::BlackTransparent => LCDBitmapDrawMode::kDrawModeBlackTransparent,
            DrawMode::FillWhite => LCDBitmapDrawMode::kDrawModeFillWhite,
            DrawMode::FillBlack => LCDBitmapDrawMode::kDrawModeFillBlack,
            DrawMode::XOR => LCDBitmapDrawMode::kDrawModeXOR,
            DrawMode::NXOR => LCDBitmapDrawMode::kDrawModeNXOR,
            DrawMode::Inverted => LCDBitmapDrawMode::kDrawModeInverted,
        }
    }
}
