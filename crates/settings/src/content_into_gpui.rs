use gpui::{
    FontFeatures, FontStyle, FontWeight, Modifiers, Pixels, SharedString,
    WindowBackgroundAppearance, px,
};
use settings_content::{
    FontFamilyName, FontFeaturesContent, FontSize, FontStyleContent, FontWeightContent,
    ModifiersContent, WindowBackgroundContent,
};
use std::sync::Arc;

/// A trait for converting settings content types into their GPUI equivalents.
pub trait IntoGpui {
    type Output;
    fn into_gpui(self) -> Self::Output;
}

impl IntoGpui for FontStyleContent {
    type Output = FontStyle;

    fn into_gpui(self) -> Self::Output {
        match self {
            FontStyleContent::Normal => FontStyle::Normal,
            FontStyleContent::Italic => FontStyle::Italic,
            FontStyleContent::Oblique => FontStyle::Oblique,
        }
    }
}

impl IntoGpui for FontWeightContent {
    type Output = FontWeight;

    fn into_gpui(self) -> Self::Output {
        FontWeight(self.0.clamp(100., 950.))
    }
}

impl IntoGpui for FontFeaturesContent {
    type Output = FontFeatures;

    fn into_gpui(self) -> Self::Output {
        FontFeatures(Arc::new(self.0.into_iter().collect()))
    }
}

impl IntoGpui for WindowBackgroundContent {
    type Output = WindowBackgroundAppearance;

    fn into_gpui(self) -> Self::Output {
        match self {
            WindowBackgroundContent::Opaque => WindowBackgroundAppearance::Opaque,
            WindowBackgroundContent::Transparent => WindowBackgroundAppearance::Transparent,
            WindowBackgroundContent::Blurred => WindowBackgroundAppearance::Blurred,
        }
    }
}

impl IntoGpui for ModifiersContent {
    type Output = Modifiers;

    fn into_gpui(self) -> Self::Output {
        Modifiers {
            control: self.control,
            alt: self.alt,
            shift: self.shift,
            platform: self.platform,
            function: self.function,
        }
    }
}

impl IntoGpui for FontSize {
    type Output = Pixels;

    fn into_gpui(self) -> Self::Output {
        px(self.0)
    }
}

impl IntoGpui for FontFamilyName {
    type Output = SharedString;

    fn into_gpui(self) -> Self::Output {
        SharedString::from(self.0)
    }
}

#[cfg(test)]
mod tests {
    use gpui::FontWeight;
    use settings_content::FontWeightContent;

    #[test]
    fn test_font_weight_content_constants_match_gpui() {
        assert_eq!(FontWeightContent::THIN.0, FontWeight::THIN.0);
        assert_eq!(FontWeightContent::EXTRA_LIGHT.0, FontWeight::EXTRA_LIGHT.0);
        assert_eq!(FontWeightContent::LIGHT.0, FontWeight::LIGHT.0);
        assert_eq!(FontWeightContent::NORMAL.0, FontWeight::NORMAL.0);
        assert_eq!(FontWeightContent::MEDIUM.0, FontWeight::MEDIUM.0);
        assert_eq!(FontWeightContent::SEMIBOLD.0, FontWeight::SEMIBOLD.0);
        assert_eq!(FontWeightContent::BOLD.0, FontWeight::BOLD.0);
        assert_eq!(FontWeightContent::EXTRA_BOLD.0, FontWeight::EXTRA_BOLD.0);
        assert_eq!(FontWeightContent::BLACK.0, FontWeight::BLACK.0);
    }
}
