#[cfg(target_family = "wasm")]
use std::borrow::Cow;

use gpui::App;

#[cfg(target_family = "wasm")]
pub fn load_fonts(cx: &App) -> bool {
    let fonts = [
        Cow::Borrowed(
            include_bytes!("../../../../assets/fonts/ibm-plex-sans/IBMPlexSans-Regular.ttf")
                .as_slice(),
        ),
        Cow::Borrowed(
            include_bytes!("../../../../assets/fonts/ibm-plex-sans/IBMPlexSans-Italic.ttf")
                .as_slice(),
        ),
        Cow::Borrowed(
            include_bytes!("../../../../assets/fonts/ibm-plex-sans/IBMPlexSans-SemiBold.ttf")
                .as_slice(),
        ),
        Cow::Borrowed(
            include_bytes!("../../../../assets/fonts/ibm-plex-sans/IBMPlexSans-SemiBoldItalic.ttf")
                .as_slice(),
        ),
        Cow::Borrowed(
            include_bytes!("../../../../assets/fonts/lilex/Lilex-Regular.ttf").as_slice(),
        ),
        Cow::Borrowed(include_bytes!("../../../../assets/fonts/lilex/Lilex-Bold.ttf").as_slice()),
        Cow::Borrowed(include_bytes!("../../../../assets/fonts/lilex/Lilex-Italic.ttf").as_slice()),
        Cow::Borrowed(
            include_bytes!("../../../../assets/fonts/lilex/Lilex-BoldItalic.ttf").as_slice(),
        ),
    ];
    if let Err(error) = cx.text_system().add_fonts(fonts.into()) {
        web_sys::console::error_1(&format!("failed to load application fonts: {error:#}").into());
        return false;
    }
    true
}

#[cfg(not(target_family = "wasm"))]
pub fn load_fonts(_cx: &App) -> bool {
    true
}
