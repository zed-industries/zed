pub use gpui::layer_shell::*;

/// Layer-shell windows with this namespace are created as `ext-session-lock-v1` lock
/// surfaces on their output instead. Their other layer-shell options are ignored.
pub const SESSION_LOCK_NAMESPACE: &str = "gpui-session-lock";

use wayland_protocols_wlr::layer_shell::v1::client::{zwlr_layer_shell_v1, zwlr_layer_surface_v1};

pub(crate) fn wayland_layer(layer: Layer) -> zwlr_layer_shell_v1::Layer {
    match layer {
        Layer::Background => zwlr_layer_shell_v1::Layer::Background,
        Layer::Bottom => zwlr_layer_shell_v1::Layer::Bottom,
        Layer::Top => zwlr_layer_shell_v1::Layer::Top,
        Layer::Overlay => zwlr_layer_shell_v1::Layer::Overlay,
    }
}

pub(crate) fn wayland_anchor(anchor: Anchor) -> zwlr_layer_surface_v1::Anchor {
    zwlr_layer_surface_v1::Anchor::from_bits_truncate(anchor.bits())
}

pub(crate) fn wayland_keyboard_interactivity(
    value: KeyboardInteractivity,
) -> zwlr_layer_surface_v1::KeyboardInteractivity {
    match value {
        KeyboardInteractivity::None => zwlr_layer_surface_v1::KeyboardInteractivity::None,
        KeyboardInteractivity::Exclusive => zwlr_layer_surface_v1::KeyboardInteractivity::Exclusive,
        KeyboardInteractivity::OnDemand => zwlr_layer_surface_v1::KeyboardInteractivity::OnDemand,
    }
}
