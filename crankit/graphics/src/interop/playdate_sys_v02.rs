use core::ptr;

use playdate_sys::ffi::{LCDColor, LCDSolidColor};

use crate::{color::Solid, Color};

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

pub(crate) fn with_lcd_color<T>(color: impl Into<Color>, action: impl FnOnce(LCDColor) -> T) -> T {
    match color.into() {
        Color::Solid(solid) => action(LCDSolidColor::from(solid) as LCDColor),
        Color::Pattern(pattern) => {
            let bits: [u8; 16] = pattern.into();
            action(ptr::addr_of!(bits) as LCDColor)
        }
    }
}
