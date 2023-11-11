#![no_std]

use core::{
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign},
    ptr,
};
use playdate_sys::{api, ffi::PDButtons};

pub mod ffi {
    pub use playdate_sys::ffi::PDButtons;
}

unsafe fn system() -> &'static playdate_sys::ffi::playdate_sys {
    api()
        .expect("Playdate API not initialized")
        .system
        .as_ref()
        .expect("cannot find playdate system")
}

/// Returns the current [`ButtonState`]
pub fn button_state() -> ButtonState {
    let mut current = ffi::PDButtons(0);
    let mut pushed = ffi::PDButtons(0);
    let mut released = ffi::PDButtons(0);
    unsafe {
        system().getButtonState.unwrap()(
            ptr::addr_of_mut!(current),
            ptr::addr_of_mut!(pushed),
            ptr::addr_of_mut!(released),
        );
    }
    ButtonState {
        current: current.into(),
        pushed: pushed.into(),
        released: released.into(),
    }
}

/// Returns the current position of the crank, in the range 0-360.
///
/// Zero is pointing up, and the value increases as the crank moves clockwise, as viewed from the right side of the device.
pub fn crank_angle() -> f32 {
    unsafe { system().getCrankAngle.unwrap()() }
}

/// Returns the angle change of the crank since the last time this function was called.
///
/// Negative values are anti-clockwise.
pub fn crank_change() -> f32 {
    unsafe { system().getCrankChange.unwrap()() }
}

/// Returns whether or not the crank is folded into the unit.
pub fn is_crank_docked() -> bool {
    unsafe { system().isCrankDocked.unwrap()() == 1 }
}

/// State of the playdate buttons
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ButtonState {
    /// Buttons currently being pressed
    pub current: ButtonSet,
    /// Buttons that are have just started to be pressed
    ///
    /// Meaning they were not pressed last frame, and are now currently pressed
    pub pushed: ButtonSet,
    /// Buttons that have just been released
    ///
    /// Meaning they were pressed last frame, and are no longer pressed
    pub released: ButtonSet,
}

impl ButtonState {
    /// Returns true if the given button is currently pressed
    #[inline]
    #[must_use]
    pub fn is_pressed(self, button: Button) -> bool {
        self.current.contains(button)
    }

    /// Returns true if the given button is has just started to be pressed
    ///
    /// Meaning it was not pressed last frame, and is now currently pressed
    #[inline]
    #[must_use]
    pub fn is_just_pressed(self, button: Button) -> bool {
        self.pushed.contains(button)
    }

    /// Returns true if the given button is has just started to be pressed
    ///
    /// Meaning it was pressed last frame, and is no longer pressed
    #[inline]
    #[must_use]
    pub fn is_just_released(self, button: Button) -> bool {
        self.released.contains(button)
    }

    /// Returns true if any of the given button is currently pressed
    #[inline]
    #[must_use]
    pub fn is_any_pressed(&self, buttons: ButtonSet) -> bool {
        self.current.contains_any(buttons)
    }

    /// Returns true if any of the given button was just pressed
    #[inline]
    #[must_use]
    pub fn is_any_just_pressed(&self, buttons: ButtonSet) -> bool {
        self.pushed.contains_any(buttons)
    }

    /// Returns true if any of the given button was just released
    #[inline]
    #[must_use]
    pub fn is_any_just_released(&self, buttons: ButtonSet) -> bool {
        self.released.contains_any(buttons)
    }

    /// Returns the currently pressed state of the d-pad as a 2d vector
    ///
    /// See [`ButtonSet::d_pad`] for more details
    #[inline]
    #[must_use]
    pub fn d_pad<T: From<i8>>(self) -> [T; 2] {
        self.current.d_pad()
    }

    /// Returns the buttons of the d-pad that have just started to be pressed as a 2d vector
    ///
    /// See [`ButtonSet::d_pad`] for more details
    #[inline]
    #[must_use]
    pub fn d_pad_just_pressed<T: From<i8>>(self) -> [T; 2] {
        self.pushed.d_pad()
    }

    /// Returns the buttons of the d-pad that have just been released as a 2d vector
    ///
    /// See [`ButtonSet::d_pad`] for more details
    #[inline]
    #[must_use]
    pub fn d_pad_just_released<T: From<i8>>(self) -> [T; 2] {
        self.released.d_pad()
    }
}

/// Set of buttons
///
/// Supports `&` and `|` operators.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct ButtonSet(u8);

impl ButtonSet {
    pub const D_PAD: Self = Self(
        (PDButtons::kButtonLeft.0
            | PDButtons::kButtonUp.0
            | PDButtons::kButtonRight.0
            | PDButtons::kButtonDown.0) as u8,
    );

    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    #[must_use]
    pub fn contains(self, button: Button) -> bool {
        self.contains_any(button.into())
    }

    #[inline]
    #[must_use]
    pub fn contains_any(self, buttons: ButtonSet) -> bool {
        (self & buttons).0 > 0
    }

    /// Returns the d-pad buttons contained in this set as a 2d vector
    ///
    /// The axes correspond to the playdate screen coordinate system (`x` is right, and `y` is down):
    /// * Left is [-1, 0]
    /// * Right is [1, 0]
    /// * Down is [0, 1]
    /// * Up is [0, -1]
    ///
    /// If more than one D-Pad button is contained in the set, this method returns the sum of the vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crankit_input::{ButtonSet, Button};
    /// let up_right = ButtonSet::default() | Button::Up | Button::Right;
    /// assert_eq!(up_right.d_pad::<i8>(), [1, -1]);
    ///
    /// let up_down_left = ButtonSet::default() | Button::Up | Button::Down | Button::Left;
    /// assert_eq!(up_down_left.d_pad::<i8>(), [-1, 0]);
    /// ```
    ///
    #[must_use]
    pub fn d_pad<T: From<i8>>(self) -> [T; 2] {
        let mut x = 0;
        let mut y = 0;
        if self.contains(Button::Up) {
            y -= 1;
        }
        if self.contains(Button::Down) {
            y += 1;
        }
        if self.contains(Button::Left) {
            x -= 1;
        }
        if self.contains(Button::Right) {
            x += 1;
        }
        [x.into(), y.into()]
    }
}

impl From<ffi::PDButtons> for ButtonSet {
    fn from(ffi::PDButtons(bits): ffi::PDButtons) -> Self {
        Self(bits.try_into().unwrap_or_default())
    }
}

impl FromIterator<Button> for ButtonSet {
    fn from_iter<T: IntoIterator<Item = Button>>(iter: T) -> Self {
        iter.into_iter().fold(Self::default(), BitOr::bitor)
    }
}

impl From<Button> for ButtonSet {
    fn from(value: Button) -> Self {
        let pd = match value {
            Button::Left => ffi::PDButtons::kButtonLeft,
            Button::Right => ffi::PDButtons::kButtonRight,
            Button::Up => ffi::PDButtons::kButtonUp,
            Button::Down => ffi::PDButtons::kButtonDown,
            Button::B => ffi::PDButtons::kButtonB,
            Button::A => ffi::PDButtons::kButtonA,
        };
        pd.into()
    }
}

impl BitOrAssign for ButtonSet {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitOr for ButtonSet {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}

impl BitOrAssign<Button> for ButtonSet {
    fn bitor_assign(&mut self, rhs: Button) {
        *self |= ButtonSet::from(rhs);
    }
}

impl BitOr<Button> for ButtonSet {
    type Output = Self;
    fn bitor(mut self, rhs: Button) -> Self::Output {
        self |= rhs;
        self
    }
}

impl BitAndAssign for ButtonSet {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitAnd for ButtonSet {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}

impl BitAndAssign<Button> for ButtonSet {
    fn bitand_assign(&mut self, rhs: Button) {
        *self &= ButtonSet::from(rhs);
    }
}

impl BitAnd<Button> for ButtonSet {
    type Output = Self;
    fn bitand(mut self, rhs: Button) -> Self::Output {
        self &= rhs;
        self
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Button {
    Left,
    Right,
    Up,
    Down,
    A,
    B,
}

impl BitOr for Button {
    type Output = ButtonSet;
    fn bitor(self, rhs: Self) -> Self::Output {
        ButtonSet::from(self) | ButtonSet::from(rhs)
    }
}

impl BitAnd for Button {
    type Output = ButtonSet;
    fn bitand(self, rhs: Self) -> Self::Output {
        ButtonSet::from(self) & ButtonSet::from(rhs)
    }
}

impl BitOr<ButtonSet> for Button {
    type Output = ButtonSet;

    fn bitor(self, rhs: ButtonSet) -> Self::Output {
        rhs | self
    }
}

impl BitAnd<ButtonSet> for Button {
    type Output = ButtonSet;
    fn bitand(self, rhs: ButtonSet) -> Self::Output {
        rhs & self
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(ffi::PDButtons::kButtonA, Button::A, true)]
    #[case(ffi::PDButtons::kButtonA, Button::B, false)]
    #[case(ffi::PDButtons::kButtonB, Button::A, false)]
    #[case(ffi::PDButtons::kButtonB, Button::B, true)]
    #[case(ffi::PDButtons::kButtonA | ffi::PDButtons::kButtonB, Button::B, true)]
    #[case(ffi::PDButtons::kButtonA | ffi::PDButtons::kButtonB, Button::A, true)]
    #[case(ffi::PDButtons::kButtonA | ffi::PDButtons::kButtonB, Button::Up, false)]
    #[case(ffi::PDButtons::kButtonA | ffi::PDButtons::kButtonB | ffi::PDButtons::kButtonUp, Button::Up, true)]
    fn test_set_contains(
        #[case] raw_set: ffi::PDButtons,
        #[case] button: Button,
        #[case] expected: bool,
    ) {
        let set: ButtonSet = ButtonSet(raw_set.0.try_into().unwrap());
        assert_eq!(set.contains(button), expected);
        assert_eq!(set.contains_any(button.into()), expected);
    }

    #[rstest]
    #[case(ButtonSet::default(), ButtonSet::from_iter([Button::A]), false)]
    #[case(ButtonSet::default(), ButtonSet::from_iter([Button::A, Button::B]), false)]
    #[case(ButtonSet::default(), ButtonSet::default(), false)]
    #[case(ButtonSet::from_iter([Button::A]), ButtonSet::default(), false)]
    #[case(ButtonSet::from_iter([Button::A]), ButtonSet::from_iter([Button::A]), true)]
    #[case(ButtonSet::from_iter([Button::A, Button::B]), ButtonSet::from_iter([Button::A]), true)]
    #[case(ButtonSet::from_iter([Button::A, Button::B]), ButtonSet::from_iter([Button::A, Button::B]), true)]
    #[case(ButtonSet::from_iter([Button::A]), ButtonSet::from_iter([Button::A, Button::B]), true)]
    fn test_set_contains_any(
        #[case] set: ButtonSet,
        #[case] buttons: ButtonSet,
        #[case] expected: bool,
    ) {
        assert_eq!(set.contains_any(buttons.into()), expected);
    }

    #[rstest]
    #[case(ButtonSet::default(), [0, 0])]
    #[case(ButtonSet::default() | Button::Up, [0, -1])]
    #[case(ButtonSet::default() | Button::Down, [0, 1])]
    #[case(ButtonSet::default() | Button::Left, [-1, 0])]
    #[case(ButtonSet::default() | Button::Right, [1, 0])]
    #[case(ButtonSet::default() | Button::Right | Button::Down | Button::Up, [1, 0])]
    #[case(ButtonSet::default() | Button::Left | Button::Right | Button::Up, [0, -1])]
    #[case(ButtonSet::default() | Button::Left | Button::Right | Button::Up | Button::Down, [0, 0])]
    fn d_pad_vector(#[case] set: ButtonSet, #[case] expected: [i8; 2]) {
        assert_eq!(set.d_pad::<i8>(), expected);
        assert_eq!(set.d_pad::<i32>(), [expected[0] as i32, expected[1] as i32]);
        assert_eq!(set.d_pad::<f32>(), [expected[0] as f32, expected[1] as f32]);
    }
}
