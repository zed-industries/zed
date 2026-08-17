use super::{FontData, FontFamily, FontStyle, LayoutBox};

#[derive(Debug, Clone)]
pub struct FontError;

impl std::fmt::Display for FontError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "General Error")?;
        Ok(())
    }
}

impl std::error::Error for FontError {}

#[derive(Clone)]
pub struct FontDataInternal(String, String);

impl FontData for FontDataInternal {
    type ErrorType = FontError;
    fn new(family: FontFamily, style: FontStyle) -> Result<Self, FontError> {
        Ok(FontDataInternal(
            family.as_str().into(),
            style.as_str().into(),
        ))
    }

    /// Note: This is only a crude estimatation, since for some backend such as SVG, we have no way to
    /// know the real size of the text anyway. Thus using font-kit is an overkill and doesn't helps
    /// the layout.
    fn estimate_layout(&self, size: f64, text: &str) -> Result<LayoutBox, Self::ErrorType> {
        let em = size / 1.24 / 1.24;
        Ok((
            (0, -em.round() as i32),
            (
                (em * 0.7 * text.len() as f64).round() as i32,
                (em * 0.24).round() as i32,
            ),
        ))
    }
}
