use gpui::{ImageSource, SharedString};

use crate::Icon;

/// A slot utility that provides a way to to pass either
/// an icon or an image to a component.
#[derive(Debug, Clone)]
pub enum GraphicSlot {
    Icon(Icon),
    Avatar(ImageSource),
    PublicActor(SharedString),
}
