use alacritty_terminal::vte::ansi::Rgb as AlacRgb;
use gpui::Rgba;

//Convenience method to convert from a GPUI color to an alacritty Rgb
pub fn to_alac_rgb(color: impl Into<Rgba>) -> AlacRgb {
    let color = color.into();
    let r = ((color.r * color.a) * 255.) as u8;
    let g = ((color.g * color.a) * 255.) as u8;
    let b = ((color.b * color.a) * 255.) as u8;
    AlacRgb { r, g, b }
}
