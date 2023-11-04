use gpui2::SharedString;

use crate::Icon;

#[derive(Debug, Clone)]
/// A slot utility that provides a way to to pass either
/// an icon or an image to a component.
///
/// Can be filled with a []
pub enum GraphicSlot {
    Icon(Icon),
    Avatar(SharedString),
    PublicActor(SharedString),
}
