use gpui::Rgba;
use vte::ansi::Rgb as VteRgb;

pub(crate) fn to_vte_rgb(color: impl Into<Rgba>) -> VteRgb {
    let color = color.into();
    let r = ((color.r * color.a) * 255.) as u8;
    let g = ((color.g * color.a) * 255.) as u8;
    let b = ((color.b * color.a) * 255.) as u8;
    VteRgb { r, g, b }
}
