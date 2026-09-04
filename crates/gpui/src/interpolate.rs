use crate::{AbsoluteLength, Background, DefiniteLength, Fill, Hsla, Length, Pixels, Rems, Rgba};

/// Computes presentation values between two endpoints.
pub trait Interpolate: Sized {
    /// Returns the value at `phase`.
    ///
    /// A phase of `0.0` returns `from`, and `1.0` returns `to`. Numerical implementations may
    /// extrapolate outside that range. Other implementations may hold `from` until completion
    /// when the endpoints use incompatible representations.
    fn interpolate(from: Self, to: Self, phase: f32) -> Self;
}

impl Interpolate for f32 {
    fn interpolate(from: Self, to: Self, phase: f32) -> Self {
        from + (to - from) * phase
    }
}

impl Interpolate for usize {
    fn interpolate(from: Self, to: Self, phase: f32) -> Self {
        let value = from as f64 + (to as f64 - from as f64) * f64::from(phase);
        value.round().clamp(usize::MIN as f64, usize::MAX as f64) as Self
    }
}

impl Interpolate for Rgba {
    fn interpolate(from: Self, to: Self, phase: f32) -> Self {
        Self {
            r: f32::interpolate(from.r, to.r, phase),
            g: f32::interpolate(from.g, to.g, phase),
            b: f32::interpolate(from.b, to.b, phase),
            a: f32::interpolate(from.a, to.a, phase),
        }
    }
}

impl Interpolate for Rems {
    fn interpolate(from: Self, to: Self, phase: f32) -> Self {
        Self(f32::interpolate(from.0, to.0, phase))
    }
}

impl Interpolate for Pixels {
    fn interpolate(from: Self, to: Self, phase: f32) -> Self {
        Self(f32::interpolate(from.0, to.0, phase))
    }
}

impl Interpolate for Hsla {
    fn interpolate(from: Self, to: Self, phase: f32) -> Self {
        let hue_delta = (to.h - from.h + 0.5).rem_euclid(1.0) - 0.5;
        Self {
            h: (from.h + hue_delta * phase).rem_euclid(1.0),
            s: f32::interpolate(from.s, to.s, phase),
            l: f32::interpolate(from.l, to.l, phase),
            a: f32::interpolate(from.a, to.a, phase),
        }
    }
}

impl Interpolate for AbsoluteLength {
    fn interpolate(from: Self, to: Self, phase: f32) -> Self {
        match (from, to) {
            (Self::Pixels(from), Self::Pixels(to)) => {
                Self::Pixels(Pixels::interpolate(from, to, phase))
            }
            (Self::Rems(from), Self::Rems(to)) => Self::Rems(Rems::interpolate(from, to, phase)),
            (from, Self::Pixels(to)) if from.is_zero() => {
                Self::Pixels(Pixels::interpolate(Pixels::ZERO, to, phase))
            }
            (from, Self::Rems(to)) if from.is_zero() => {
                Self::Rems(Rems::interpolate(Rems::ZERO, to, phase))
            }
            (Self::Pixels(from), to) if to.is_zero() => {
                Self::Pixels(Pixels::interpolate(from, Pixels::ZERO, phase))
            }
            (Self::Rems(from), to) if to.is_zero() => {
                Self::Rems(Rems::interpolate(from, Rems::ZERO, phase))
            }
            (_, to) if phase >= 1.0 => to,
            (from, _) => from,
        }
    }
}

impl Interpolate for DefiniteLength {
    fn interpolate(from: Self, to: Self, phase: f32) -> Self {
        match (from, to) {
            (Self::Absolute(from), Self::Absolute(to)) => {
                Self::Absolute(AbsoluteLength::interpolate(from, to, phase))
            }
            (Self::Fraction(from), Self::Fraction(to)) => {
                Self::Fraction(f32::interpolate(from, to, phase))
            }
            (_, to) if phase >= 1.0 => to,
            (from, _) => from,
        }
    }
}

impl Interpolate for Length {
    fn interpolate(from: Self, to: Self, phase: f32) -> Self {
        match (from, to) {
            (Self::Definite(from), Self::Definite(to)) => {
                Self::Definite(DefiniteLength::interpolate(from, to, phase))
            }
            (_, to) if phase >= 1.0 => to,
            (from, _) => from,
        }
    }
}

impl Interpolate for Background {
    fn interpolate(from: Self, to: Self, phase: f32) -> Self {
        if phase <= 0.0 {
            return from;
        }
        if phase >= 1.0 {
            return to;
        }

        match (from.as_solid(), to.as_solid()) {
            (Some(from), Some(to_color)) => {
                Background::from(Hsla::interpolate(from, to_color, phase))
                    .color_space(to.color_space)
            }
            _ => from,
        }
    }
}

impl Interpolate for Fill {
    fn interpolate(from: Self, to: Self, phase: f32) -> Self {
        match (from, to) {
            (Self::Color(from), Self::Color(to)) => {
                Self::Color(Background::interpolate(from, to, phase))
            }
        }
    }
}
