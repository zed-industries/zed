use gpui::Node;

use crate::{prelude::*, v_stack};

/// Create an elevated surface.
///
/// Must be used inside of a relative parent element
pub fn elevated_surface<V: 'static>(level: ElevationIndex, cx: &mut ViewContext<V>) -> Node<V> {
    let colors = cx.theme().colors();

    // let shadow = BoxShadow {
    //     color: hsla(0., 0., 0., 0.1),
    //     offset: point(px(0.), px(1.)),
    //     blur_radius: px(3.),
    //     spread_radius: px(0.),
    // };

    v_stack()
        .rounded_lg()
        .bg(colors.elevated_surface_background)
        .border()
        .border_color(colors.border)
        .shadow(level.shadow())
}

pub fn modal<V: 'static>(cx: &mut ViewContext<V>) -> Node<V> {
    elevated_surface(ElevationIndex::ModalSurface, cx)
}
