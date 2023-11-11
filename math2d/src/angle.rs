pub trait Angle: Copy {
    fn cos(self) -> f32;
    fn sin(self) -> f32;
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Radians(pub f32);

impl From<f32> for Radians {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<Radians> for f32 {
    fn from(Radians(value): Radians) -> Self {
        value
    }
}

impl Angle for Radians {
    fn cos(self) -> f32 {
        libm::cosf(self.0)
    }

    fn sin(self) -> f32 {
        libm::sinf(self.0)
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Degrees(pub f32);

impl From<f32> for Degrees {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<Degrees> for f32 {
    fn from(Degrees(value): Degrees) -> Self {
        value
    }
}

impl From<Degrees> for Radians {
    fn from(Degrees(deg): Degrees) -> Self {
        Radians(deg.to_radians())
    }
}

impl From<Radians> for Degrees {
    fn from(Radians(rads): Radians) -> Self {
        Degrees(rads.to_degrees())
    }
}

impl Angle for Degrees {
    fn cos(self) -> f32 {
        Radians::from(self).cos()
    }

    fn sin(self) -> f32 {
        Radians::from(self).sin()
    }
}
