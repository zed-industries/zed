use anyhow::Result;
use gpui::Hsla;
use palette::FromColor;

#[allow(unused)]
pub(crate) fn try_parse_color(color: &str) -> Result<Hsla> {
    let rgba = gpui::Rgba::try_from(color)?;
    let rgba = palette::rgb::Srgba::from_components((rgba.r, rgba.g, rgba.b, rgba.a));
    let hsla = palette::Hsla::from_color(rgba);

    let hsla = gpui::hsla(
        hsla.hue.into_positive_degrees() / 360.,
        hsla.saturation,
        hsla.lightness,
        hsla.alpha,
    );

    Ok(hsla)
}

#[allow(unused)]
pub(crate) fn pack_color(color: Hsla) -> u32 {
    let hsla = palette::Hsla::from_components((color.h * 360., color.s, color.l, color.a));
    let rgba = palette::rgb::Srgba::from_color(hsla);
    let rgba = rgba.into_format::<u8, u8>();

    u32::from(rgba)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_serialize_color() {
        let color = "#b4637aff";
        let hsla = try_parse_color(color).unwrap();
        let packed = pack_color(hsla);

        assert_eq!(format!("#{:x}", packed), color);
    }

    #[test]
    pub fn test_serialize_color_with_palette() {
        let color = "#b4637aff";

        let rgba = gpui::Rgba::try_from(color).unwrap();
        let rgba = palette::rgb::Srgba::from_components((rgba.r, rgba.g, rgba.b, rgba.a));
        let hsla = palette::Hsla::from_color(rgba);

        let rgba = palette::rgb::Srgba::from_color(hsla);
        let rgba = rgba.into_format::<u8, u8>();

        assert_eq!(format!("#{:x}", rgba), color);
    }
}
