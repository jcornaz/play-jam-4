use core::ptr;

use playdate_sys::ffi::{LCDColor, LCDSolidColor};

/// A color that can be used when drawing
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
    /// A uniform color
    Solid(Solid),
    /// A 8x8 pattern
    Pattern(Pattern),
}

impl Color {
    /// Solid Black color
    #[must_use]
    pub const fn black() -> Self {
        Self::Solid(Solid::Black)
    }

    /// Solid white color
    #[must_use]
    pub const fn white() -> Self {
        Self::Solid(Solid::White)
    }

    #[must_use]
    pub const fn clear() -> Self {
        Self::Solid(Solid::Clear)
    }

    #[must_use]
    pub const fn xor() -> Self {
        Self::Solid(Solid::Xor)
    }
}

impl From<Solid> for Color {
    fn from(value: Solid) -> Self {
        Self::Solid(value)
    }
}

/// A uniform color
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Solid {
    /// Black
    Black,
    /// White
    White,
    Clear,
    Xor,
}

impl From<Solid> for LCDSolidColor {
    fn from(value: Solid) -> Self {
        match value {
            Solid::Black => LCDSolidColor::kColorBlack,
            Solid::White => LCDSolidColor::kColorWhite,
            Solid::Clear => LCDSolidColor::kColorClear,
            Solid::Xor => LCDSolidColor::kColorXOR,
        }
    }
}

/// A 8x8 Pattern used for drawing
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Pattern([u8; 16]);

impl Pattern {
    /// Create a black and white pattern from black bits
    ///
    /// The argument is an array of 8 numbers describing the bitmap for each row;
    /// for example, `[0xaa, 0x55, 0xaa, 0x55, 0xaa, 0x55, 0xaa, 0x55]` specifies a checkerboard pattern
    #[must_use]
    pub fn from_black(black: [u8; 8]) -> Self {
        let mut inner = [0xFF; 16];
        inner[0..8].copy_from_slice(&black);
        Self(inner)
    }

    /// Create pattern from black bits and mask
    ///
    /// The argument is an array of 8 numbers describing the bitmap for each row;
    /// for example, `[0xaa, 0x55, 0xaa, 0x55, 0xaa, 0x55, 0xaa, 0x55]` specifies a checkerboard pattern
    #[must_use]
    pub fn from_black_and_mask(black: [u8; 8], mask: [u8; 8]) -> Self {
        let mut inner = [0xFF; 16];
        inner[0..8].copy_from_slice(&black);
        inner[8..16].copy_from_slice(&mask);
        Self(inner)
    }
}

impl From<Pattern> for Color {
    fn from(value: Pattern) -> Self {
        Self::Pattern(value)
    }
}

pub(super) fn with_lcd_color<T>(color: impl Into<Color>, action: impl FnOnce(LCDColor) -> T) -> T {
    match color.into() {
        Color::Solid(solid) => action(LCDSolidColor::from(solid) as LCDColor),
        Color::Pattern(pattern) => action(ptr::addr_of!(pattern.0) as LCDColor),
    }
}
