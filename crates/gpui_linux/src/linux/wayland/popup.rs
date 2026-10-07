pub use gpui::popup::*;

use wayland_protocols::xdg::shell::client::xdg_positioner;

pub(crate) fn wayland_anchor(anchor: PopupAnchor) -> xdg_positioner::Anchor {
    match anchor {
        PopupAnchor::Center => xdg_positioner::Anchor::None,
        PopupAnchor::Top => xdg_positioner::Anchor::Top,
        PopupAnchor::Bottom => xdg_positioner::Anchor::Bottom,
        PopupAnchor::Left => xdg_positioner::Anchor::Left,
        PopupAnchor::Right => xdg_positioner::Anchor::Right,
        PopupAnchor::TopLeft => xdg_positioner::Anchor::TopLeft,
        PopupAnchor::BottomLeft => xdg_positioner::Anchor::BottomLeft,
        PopupAnchor::TopRight => xdg_positioner::Anchor::TopRight,
        PopupAnchor::BottomRight => xdg_positioner::Anchor::BottomRight,
    }
}

pub(crate) fn wayland_gravity(gravity: PopupGravity) -> xdg_positioner::Gravity {
    match gravity {
        PopupGravity::Center => xdg_positioner::Gravity::None,
        PopupGravity::Top => xdg_positioner::Gravity::Top,
        PopupGravity::Bottom => xdg_positioner::Gravity::Bottom,
        PopupGravity::Left => xdg_positioner::Gravity::Left,
        PopupGravity::Right => xdg_positioner::Gravity::Right,
        PopupGravity::TopLeft => xdg_positioner::Gravity::TopLeft,
        PopupGravity::BottomLeft => xdg_positioner::Gravity::BottomLeft,
        PopupGravity::TopRight => xdg_positioner::Gravity::TopRight,
        PopupGravity::BottomRight => xdg_positioner::Gravity::BottomRight,
    }
}

pub(crate) fn wayland_constraint_adjustment(
    adjustment: PopupConstraintAdjustment,
) -> xdg_positioner::ConstraintAdjustment {
    // The flag values match the protocol bitfield, so the bits map across directly.
    xdg_positioner::ConstraintAdjustment::from_bits_truncate(adjustment.bits())
}
