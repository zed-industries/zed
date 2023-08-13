pub mod color {
    use crate::color::{scale, ColorScale, Hsla};

    pub fn ramp(color: impl Into<Hsla>) -> ColorScale {
        let color = color.into();
        let end_color = color.desaturate(0.1).brighten(0.5);
        let start_color = color.desaturate(0.1).darken(0.4);
        scale([start_color, color, end_color])
    }
}
