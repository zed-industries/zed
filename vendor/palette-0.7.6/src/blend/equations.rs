use core::ops::{Mul, Sub};

use crate::{
    blend::{BlendFunction, PreAlpha},
    cast::ArrayCast,
    num::{Arithmetics, IsValidDivisor, MinMax, One, Real, Sqrt, Zero},
};

use super::{zip_colors, Premultiply};

/// A pair of blending equations and corresponding parameters.
///
/// The `Equations` type is similar to how blending works in OpenGL, where a
/// blend function has can be written as `e(sp * S, dp * D)`. `e` is the
/// equation (like `s + d`), `sp` and `dp` are the source and destination
/// parameters, and `S` and `D` are the source and destination colors.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Equations {
    /// The equation for the color components.
    pub color_equation: Equation,

    /// The equation for the alpha component.
    pub alpha_equation: Equation,

    /// The parameters for the color components.
    pub color_parameters: Parameters,

    /// The parameters for the alpha component.
    pub alpha_parameters: Parameters,
}

impl Equations {
    /// Create a pair of blending equations, where all the parameters are
    /// `One`.
    pub fn from_equations(color: Equation, alpha: Equation) -> Equations {
        Equations {
            color_equation: color,
            alpha_equation: alpha,
            color_parameters: Parameters {
                source: Parameter::One,
                destination: Parameter::One,
            },
            alpha_parameters: Parameters {
                source: Parameter::One,
                destination: Parameter::One,
            },
        }
    }

    /// Create a pair of additive blending equations with the provided
    /// parameters.
    pub fn from_parameters(source: Parameter, destination: Parameter) -> Equations {
        Equations {
            color_equation: Equation::Add,
            alpha_equation: Equation::Add,
            color_parameters: Parameters {
                source,
                destination,
            },
            alpha_parameters: Parameters {
                source,
                destination,
            },
        }
    }
}

impl<C, S, const N: usize, const M: usize> BlendFunction<C> for Equations
where
    C: Clone
        + Premultiply<Scalar = S>
        + Mul<Output = C>
        + Mul<S, Output = C>
        + ArrayCast<Array = [S; N]>,
    PreAlpha<C>: ArrayCast<Array = [S; M]>,
    S: Real + One + Zero + MinMax + Sqrt + IsValidDivisor + Arithmetics + Clone,
{
    fn apply_to(self, source: PreAlpha<C>, destination: PreAlpha<C>) -> PreAlpha<C> {
        let (src_color, mut dst_color) =
            if matches!(self.color_equation, Equation::Min | Equation::Max) {
                (source.color.clone(), destination.color.clone())
            } else {
                let col_src_param = self
                    .color_parameters
                    .source
                    .apply_to(source.clone(), destination.clone());
                let col_dst_param = self
                    .color_parameters
                    .destination
                    .apply_to(source.clone(), destination.clone());

                (
                    col_src_param.mul_color(source.color.clone()),
                    col_dst_param.mul_color(destination.color.clone()),
                )
            };

        let (src_alpha, dst_alpha) = if matches!(self.alpha_equation, Equation::Min | Equation::Max)
        {
            (source.alpha, destination.alpha)
        } else {
            let alpha_src_param = self
                .alpha_parameters
                .source
                .apply_to(source.clone(), destination.clone());
            let alpha_dst_param = self
                .alpha_parameters
                .destination
                .apply_to(source.clone(), destination.clone());

            (
                alpha_src_param.mul_constant(source.alpha),
                alpha_dst_param.mul_constant(destination.alpha),
            )
        };

        let color_op = match self.color_equation {
            Equation::Add => |src, dst| src + dst,
            Equation::Subtract => |src, dst| src - dst,
            Equation::ReverseSubtract => |src, dst| dst - src,
            Equation::Min => MinMax::min,
            Equation::Max => MinMax::max,
        };

        let alpha_op = match self.alpha_equation {
            Equation::Add => |src, dst| src + dst,
            Equation::Subtract => |src, dst| src - dst,
            Equation::ReverseSubtract => |src, dst| dst - src,
            Equation::Min => MinMax::min,
            Equation::Max => MinMax::max,
        };

        for (src, dst) in zip_colors(src_color, &mut dst_color) {
            *dst = color_op(src, dst.clone());
        }

        PreAlpha {
            color: dst_color,
            alpha: alpha_op(src_alpha, dst_alpha),
        }
    }
}

/// A blending equation.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Equation {
    /// Add the source and destination, according to `sp * S + dp * D`.
    Add,

    /// Subtract the destination from the source, according to `sp * S - dp *
    /// D`.
    Subtract,

    /// Subtract the source from the destination, according to `dp * D - sp *
    /// S`.
    ReverseSubtract,

    /// Create a color where each component is the smallest of each of the
    /// source and destination components. A.k.a. component wise min. The
    /// parameters are ignored.
    Min,

    /// Create a color where each component is the largest of each of the
    /// source and destination components. A.k.a. component wise max. The
    /// parameters are ignored.
    Max,
}

/// A pair of source and destination parameters.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Parameters {
    /// The source parameter.
    pub source: Parameter,

    /// The destination parameter.
    pub destination: Parameter,
}

/// A blending parameter.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Parameter {
    /// A simple 1.
    One,

    /// A simple 0.
    Zero,

    /// The source color, or alpha.
    SourceColor,

    /// One minus the source color, or alpha.
    OneMinusSourceColor,

    /// The destination color, or alpha.
    DestinationColor,

    /// One minus the destination color, or alpha.
    OneMinusDestinationColor,

    /// The source alpha.
    SourceAlpha,

    /// One minus the source alpha.
    OneMinusSourceAlpha,

    /// The destination alpha.
    DestinationAlpha,

    /// One minus the destination alpha.
    OneMinusDestinationAlpha,
}

impl Parameter {
    fn apply_to<C, T, const N: usize>(
        &self,
        source: PreAlpha<C>,
        destination: PreAlpha<C>,
    ) -> ParamOut<C>
    where
        C: Premultiply<Scalar = T>,
        PreAlpha<C>: ArrayCast<Array = [T; N]>,
        T: Real + One + Zero + Sub<Output = T>,
    {
        match *self {
            Parameter::One => ParamOut::Constant(T::one()),
            Parameter::Zero => ParamOut::Constant(T::zero()),
            Parameter::SourceColor => ParamOut::Color(source),
            Parameter::OneMinusSourceColor => {
                ParamOut::Color(<[T; N]>::from(source).map(|a| T::one() - a).into())
            }
            Parameter::DestinationColor => ParamOut::Color(destination),
            Parameter::OneMinusDestinationColor => {
                ParamOut::Color(<[T; N]>::from(destination).map(|a| T::one() - a).into())
            }
            Parameter::SourceAlpha => ParamOut::Constant(source.alpha),
            Parameter::OneMinusSourceAlpha => ParamOut::Constant(T::one() - source.alpha),
            Parameter::DestinationAlpha => ParamOut::Constant(destination.alpha),
            Parameter::OneMinusDestinationAlpha => ParamOut::Constant(T::one() - destination.alpha),
        }
    }
}

enum ParamOut<C: Premultiply> {
    Color(PreAlpha<C>),
    Constant(C::Scalar),
}

impl<C, T> ParamOut<C>
where
    C: Mul<Output = C> + Mul<T, Output = C> + Premultiply<Scalar = T>,
    T: Mul<Output = T> + Clone,
{
    fn mul_constant(self, other: T) -> T {
        match self {
            ParamOut::Color(c) => c.alpha * other,
            ParamOut::Constant(c) => c * other,
        }
    }

    fn mul_color(self, other: C) -> C {
        match self {
            ParamOut::Color(c) => other * c.color,
            ParamOut::Constant(c) => other * c,
        }
    }
}
