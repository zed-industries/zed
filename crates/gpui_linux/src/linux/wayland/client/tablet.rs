use wayland_backend::{client::ObjectId, protocol::WEnum};
use wayland_client::{
    Connection, Dispatch, Proxy, QueueHandle, delegate_noop, event_created_child,
};
use wayland_protocols::wp::tablet::zv2::client::{
    zwp_tablet_pad_group_v2, zwp_tablet_pad_ring_v2, zwp_tablet_pad_strip_v2, zwp_tablet_pad_v2,
    zwp_tablet_seat_v2, zwp_tablet_tool_v2, zwp_tablet_v2,
};

use wayland_client::protocol::wl_surface;

use gpui::{
    CursorStyle, Modifiers, MouseButton, MouseExitEvent, MouseMoveEvent, NavigationDirection,
    PenInput, PenTool, Pixels, PlatformInput, Point, point, px,
};
use wayland_protocols::wp::cursor_shape::v1::client::wp_cursor_shape_device_v1;

use super::{
    Globals, PointerInput, WaylandClientState, WaylandClientStatePtr, dispatch_mouse_button,
    dispatch_pointer_hand_off, get_window, linux_button_to_gpui,
};
use crate::linux::{
    cursor_style_to_icon_names,
    wayland::{
        cursor::{Cursor, CursorTarget},
        serial::SerialKind,
        to_shape,
        window::WaylandWindowStatePtr,
    },
};

/// The protocol reports pressure and distance in `0..=65535`.
const MAX_AXIS_VALUE: f32 = 65535.0;

/// Per-tool state. Tablet tools are translated into mouse events so that pens
/// work anywhere a mouse does, with pressure and tilt attached as [`PenInput`].
///
/// Like GTK, axis changes and button transitions are accumulated until the
/// protocol's `frame` event, so that every event we emit sees the complete
/// state of that hardware frame (e.g. the pressure that arrives after `down`).
pub(in crate::linux::wayland) struct TabletTool {
    proxy: zwp_tablet_tool_v2::ZwpTabletToolV2,
    axes: ToolAxes,
    cursor_shape_device: Option<wp_cursor_shape_device_v1::WpCursorShapeDeviceV1>,
    /// Used instead of `cursor_shape_device` when the compositor lacks
    /// cursor-shape-v1. Each tool owns its surface because a surface can only
    /// ever hold one cursor role, so the pointer's cannot be shared.
    cursor_surface: Option<wl_surface::WlSurface>,
    /// The style last applied to this tool's cursor.
    cursor_style: Option<CursorStyle>,
    focus: Option<TabletToolFocus>,
    axes_changed: bool,
    pending: Vec<PendingToolEvent>,
}

#[derive(Default)]
struct ToolAxes {
    /// `None` for puck-like tools (mouse, lens), which behave like plain mice.
    kind: Option<PenTool>,
    has_pressure: bool,
    has_tilt: bool,
    has_distance: bool,
    pressure: f32,
    tilt: Point<f32>,
    distance: f32,
}

struct TabletToolFocus {
    window: WaylandWindowStatePtr,
    proximity_serial: u32,
    position: Option<Point<Pixels>>,
    /// Most recently pressed last, so the last entry is what we report as the
    /// pressed button while moving.
    pressed_buttons: Vec<MouseButton>,
}

enum PendingToolEvent {
    Button { button: MouseButton, pressed: bool },
    ProximityOut,
}

impl TabletTool {
    fn new(
        proxy: zwp_tablet_tool_v2::ZwpTabletToolV2,
        cursor_shape_device: Option<wp_cursor_shape_device_v1::WpCursorShapeDeviceV1>,
    ) -> Self {
        Self {
            proxy,
            axes: ToolAxes::default(),
            cursor_shape_device,
            cursor_surface: None,
            cursor_style: None,
            focus: None,
            axes_changed: false,
            pending: Vec::new(),
        }
    }

    /// Sets this tool's cursor while it hovers one of our windows. The cursor
    /// theme, size and the window's scale are read at the time of the call,
    /// like for the pointer.
    fn set_cursor(&mut self, style: CursorStyle, cursor: &mut Cursor, globals: &Globals) {
        let Some(focus) = &self.focus else {
            return;
        };
        self.cursor_style = Some(style);
        if let Some(cursor_shape_device) = &self.cursor_shape_device {
            cursor_shape_device.set_shape(focus.proximity_serial, to_shape(style));
            return;
        }
        let surface = self
            .cursor_surface
            .get_or_insert_with(|| globals.compositor.create_surface(&globals.qh, ()))
            .clone();
        cursor.set_icon_on_surface(
            CursorTarget::TabletTool(&self.proxy),
            &surface,
            focus.proximity_serial,
            cursor_style_to_icon_names(style),
            focus.window.primary_output_scale(),
        );
    }

    fn destroy(self) {
        if let Some(cursor_shape_device) = self.cursor_shape_device {
            cursor_shape_device.destroy();
        }
        if let Some(cursor_surface) = self.cursor_surface {
            cursor_surface.destroy();
        }
        self.proxy.destroy();
    }

    pub(super) fn is_over(&self, window: &WaylandWindowStatePtr) -> bool {
        self.focus
            .as_ref()
            .is_some_and(|focus| focus.window.ptr_eq(window))
    }

    /// The move that puts GPUI's pointer at this tool, if it has reported a
    /// position over the window it hovers.
    pub(super) fn move_event(&self, modifiers: Modifiers) -> Option<MouseMoveEvent> {
        let focus = self.focus.as_ref()?;
        Some(MouseMoveEvent {
            position: focus.position?,
            pressed_button: focus.pressed_buttons.last().copied(),
            modifiers,
            pen: self.axes.pen_input(),
        })
    }

    pub(in crate::linux::wayland) fn clear_focus_if_window(
        &mut self,
        window: &WaylandWindowStatePtr,
    ) {
        if self.is_over(window) {
            self.focus = None;
            self.pending.clear();
        }
    }
}

impl ToolAxes {
    fn pen_input(&self) -> Option<PenInput> {
        Some(PenInput {
            tool: self.kind?,
            pressure: self.has_pressure.then_some(self.pressure),
            tilt: self.has_tilt.then_some(self.tilt),
            distance: self.has_distance.then_some(self.distance),
        })
    }
}

/// Applies `style`, computed by `window` for this tool's position, to the
/// tool's cursor.
pub(super) fn set_tool_cursor_style(
    state: &mut WaylandClientState,
    tool_id: &ObjectId,
    window: &WaylandWindowStatePtr,
    style: CursorStyle,
) {
    let WaylandClientState {
        tablet_tools,
        cursor,
        globals,
        ..
    } = state;
    let Some(tool) = tablet_tools.get_mut(tool_id) else {
        return;
    };
    if !tool.is_over(window) || tool.cursor_style == Some(style) {
        return;
    }
    tool.set_cursor(style, cursor, globals);
}

/// Records a press like the pointer does, naming the surface the tool hovers
/// so that activation tokens (e.g. for opening links) can be requested.
fn record_press(state: &mut WaylandClientState, tool_id: &ObjectId, serial: u32) {
    let focused_surface = state
        .tablet_tools
        .get(tool_id)
        .and_then(|tool| tool.focus.as_ref())
        .map(|focus| focus.window.surface());
    state.serial_tracker.update_with_focused_surface(
        SerialKind::MousePress,
        serial,
        focused_surface,
    );
}

fn pen_tool_from_type(tool_type: zwp_tablet_tool_v2::Type) -> Option<PenTool> {
    match tool_type {
        zwp_tablet_tool_v2::Type::Pen => Some(PenTool::Pen),
        zwp_tablet_tool_v2::Type::Eraser => Some(PenTool::Eraser),
        zwp_tablet_tool_v2::Type::Brush => Some(PenTool::Brush),
        zwp_tablet_tool_v2::Type::Pencil => Some(PenTool::Pencil),
        zwp_tablet_tool_v2::Type::Airbrush => Some(PenTool::Airbrush),
        _ => None,
    }
}

fn tablet_tool_button_to_gpui(button: u32) -> Option<MouseButton> {
    // These values are coming from <linux/input-event-codes.h>.
    const BTN_STYLUS3: u32 = 0x149;
    const BTN_STYLUS: u32 = 0x14b;
    const BTN_STYLUS2: u32 = 0x14c;

    // Matches GTK. Mutter applies the user's stylus button mapping before
    // sending these codes and encodes "middle click" as BTN_STYLUS and "right
    // click" as BTN_STYLUS2, so this must stay in sync with that convention.
    match button {
        BTN_STYLUS => Some(MouseButton::Middle),
        BTN_STYLUS2 => Some(MouseButton::Right),
        BTN_STYLUS3 => Some(MouseButton::Navigate(NavigationDirection::Back)),
        _ => linux_button_to_gpui(button),
    }
}

/// Emits everything accumulated for `tool` since the previous frame: a single
/// move carrying the latest axes first, then button transitions in the order
/// they arrived.
fn flush_frame(this: &WaylandClientStatePtr, tool_id: &ObjectId) {
    let client = this.get_client();
    let mut state = client.borrow_mut();
    let Some(tool) = state.tablet_tools.get_mut(tool_id) else {
        return;
    };
    let axes_changed = std::mem::take(&mut tool.axes_changed);
    let pending = std::mem::take(&mut tool.pending);
    let pen = tool.axes.pen_input();
    let this_tool = PointerInput::TabletTool(tool_id.clone());
    // Button transitions, including the release of buttons still held when the
    // tool leaves, happen at the tool's position, so GPUI's pointer has to be
    // there first even if the tool didn't move.
    let holds_buttons = tool
        .focus
        .as_ref()
        .is_some_and(|focus| !focus.pressed_buttons.is_empty());
    let reports_buttons = pending.iter().any(|event| match event {
        PendingToolEvent::Button { .. } => true,
        PendingToolEvent::ProximityOut => holds_buttons,
    });

    if (axes_changed || reports_buttons)
        && let Some(tool) = state.tablet_tools.get(tool_id)
        && let Some(window) = tool.focus.as_ref().map(|focus| focus.window.clone())
        && let Some(event) = tool.move_event(state.modifiers)
        && (state.claim_pointer(&window, this_tool.clone()) || axes_changed)
    {
        drop(state);
        window.handle_input(PlatformInput::MouseMove(event));
        state = client.borrow_mut();
    }

    for event in pending {
        let Some(tool) = state.tablet_tools.get_mut(tool_id) else {
            return;
        };
        match event {
            PendingToolEvent::Button { button, pressed } => {
                let Some(focus) = tool.focus.as_mut() else {
                    continue;
                };
                let Some(position) = focus.position else {
                    continue;
                };
                if pressed {
                    focus.pressed_buttons.retain(|pressed| *pressed != button);
                    focus.pressed_buttons.push(button);
                } else if let Some(index) = focus
                    .pressed_buttons
                    .iter()
                    .position(|pressed| *pressed == button)
                {
                    focus.pressed_buttons.remove(index);
                } else {
                    continue;
                }
                let window = focus.window.clone();
                drop(state);
                dispatch_mouse_button(this, window, button, position, pressed, pen);
                state = client.borrow_mut();
            }
            PendingToolEvent::ProximityOut => {
                let Some(focus) = tool.focus.take() else {
                    continue;
                };
                tool.axes.pressure = 0.0;

                // The protocol sends `up` before `proximity_out`, but barrel
                // buttons may still be held when the pen is lifted away.
                if let Some(position) = focus.position
                    && !focus.pressed_buttons.is_empty()
                {
                    drop(state);
                    for button in focus.pressed_buttons.iter().rev() {
                        dispatch_mouse_button(
                            this,
                            focus.window.clone(),
                            *button,
                            position,
                            false,
                            pen,
                        );
                    }
                    state = client.borrow_mut();
                }

                let exit = focus.position.map(|position| MouseExitEvent {
                    position,
                    pressed_button: None,
                    modifiers: state.modifiers,
                });
                let hand_off = state.release_pointer(&focus.window, &this_tool);
                drop(state);
                dispatch_pointer_hand_off(&focus.window, hand_off, exit);
                state = client.borrow_mut();
            }
        }
    }
}

impl Dispatch<zwp_tablet_tool_v2::ZwpTabletToolV2, ()> for WaylandClientStatePtr {
    fn event(
        this: &mut Self,
        tool_proxy: &zwp_tablet_tool_v2::ZwpTabletToolV2,
        event: zwp_tablet_tool_v2::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let client = this.get_client();
        let mut state = client.borrow_mut();
        let tool_id = tool_proxy.id();

        match event {
            zwp_tablet_tool_v2::Event::Type {
                tool_type: WEnum::Value(tool_type),
            } => {
                if let Some(tool) = state.tablet_tools.get_mut(&tool_id) {
                    tool.axes.kind = pen_tool_from_type(tool_type);
                }
            }
            zwp_tablet_tool_v2::Event::Capability {
                capability: WEnum::Value(capability),
            } => {
                let Some(tool) = state.tablet_tools.get_mut(&tool_id) else {
                    return;
                };
                match capability {
                    zwp_tablet_tool_v2::Capability::Pressure => tool.axes.has_pressure = true,
                    zwp_tablet_tool_v2::Capability::Tilt => tool.axes.has_tilt = true,
                    zwp_tablet_tool_v2::Capability::Distance => tool.axes.has_distance = true,
                    _ => {}
                }
            }
            zwp_tablet_tool_v2::Event::ProximityIn {
                serial, surface, ..
            } => {
                // Moving between surfaces may deliver the old surface's
                // `proximity_out` in the same frame; it must apply to the old
                // focus before we replace it.
                if state
                    .tablet_tools
                    .get(&tool_id)
                    .is_some_and(|tool| !tool.pending.is_empty())
                {
                    drop(state);
                    flush_frame(this, &tool_id);
                    state = client.borrow_mut();
                }
                let Some(window) = get_window(&mut state, &surface.id()) else {
                    return;
                };
                let WaylandClientState {
                    tablet_tools,
                    cursor,
                    globals,
                    ..
                } = &mut *state;
                let Some(tool) = tablet_tools.get_mut(&tool_id) else {
                    return;
                };
                // GPUI updates the style once the tool moves and its position
                // is hit tested.
                let style = tool.cursor_style.unwrap_or(CursorStyle::Arrow);
                tool.focus = Some(TabletToolFocus {
                    window: window.clone(),
                    proximity_serial: serial,
                    position: None,
                    pressed_buttons: Vec::new(),
                });
                // Without a cursor set here the compositor leaves the tool's cursor
                // unset over our surfaces.
                tool.set_cursor(style, cursor, globals);

                drop(state);
                window.set_hovered(true);
            }
            zwp_tablet_tool_v2::Event::ProximityOut => {
                if let Some(tool) = state.tablet_tools.get_mut(&tool_id) {
                    tool.pending.push(PendingToolEvent::ProximityOut);
                }
            }
            zwp_tablet_tool_v2::Event::Motion { x, y } => {
                if let Some(tool) = state.tablet_tools.get_mut(&tool_id)
                    && let Some(focus) = tool.focus.as_mut()
                {
                    focus.position = Some(point(px(x as f32), px(y as f32)));
                    tool.axes_changed = true;
                }
            }
            zwp_tablet_tool_v2::Event::Pressure { pressure } => {
                if let Some(tool) = state.tablet_tools.get_mut(&tool_id) {
                    tool.axes.pressure = (pressure as f32 / MAX_AXIS_VALUE).clamp(0.0, 1.0);
                    tool.axes_changed = true;
                }
            }
            zwp_tablet_tool_v2::Event::Distance { distance } => {
                if let Some(tool) = state.tablet_tools.get_mut(&tool_id) {
                    tool.axes.distance = (distance as f32 / MAX_AXIS_VALUE).clamp(0.0, 1.0);
                    tool.axes_changed = true;
                }
            }
            zwp_tablet_tool_v2::Event::Tilt { tilt_x, tilt_y } => {
                if let Some(tool) = state.tablet_tools.get_mut(&tool_id) {
                    tool.axes.tilt = point(tilt_x as f32, tilt_y as f32);
                    tool.axes_changed = true;
                }
            }
            zwp_tablet_tool_v2::Event::Down { serial } => {
                record_press(&mut state, &tool_id, serial);
                if let Some(tool) = state.tablet_tools.get_mut(&tool_id) {
                    tool.pending.push(PendingToolEvent::Button {
                        button: MouseButton::Left,
                        pressed: true,
                    });
                }
            }
            zwp_tablet_tool_v2::Event::Up => {
                if let Some(tool) = state.tablet_tools.get_mut(&tool_id) {
                    tool.pending.push(PendingToolEvent::Button {
                        button: MouseButton::Left,
                        pressed: false,
                    });
                }
            }
            zwp_tablet_tool_v2::Event::Button {
                serial,
                button,
                state: WEnum::Value(button_state),
            } => {
                let Some(button) = tablet_tool_button_to_gpui(button) else {
                    return;
                };
                let pressed = match button_state {
                    zwp_tablet_tool_v2::ButtonState::Pressed => true,
                    zwp_tablet_tool_v2::ButtonState::Released => false,
                    _ => return,
                };
                // Release serials are declined by requests that reference a press.
                if pressed {
                    record_press(&mut state, &tool_id, serial);
                }
                if let Some(tool) = state.tablet_tools.get_mut(&tool_id) {
                    tool.pending
                        .push(PendingToolEvent::Button { button, pressed });
                }
            }
            zwp_tablet_tool_v2::Event::Frame { .. } => {
                drop(state);
                flush_frame(this, &tool_id);
            }
            zwp_tablet_tool_v2::Event::Removed => match state.tablet_tools.remove(&tool_id) {
                Some(tool) => tool.destroy(),
                None => tool_proxy.destroy(),
            },
            _ => {}
        }
    }
}

impl Dispatch<zwp_tablet_seat_v2::ZwpTabletSeatV2, ()> for WaylandClientStatePtr {
    fn event(
        this: &mut Self,
        _: &zwp_tablet_seat_v2::ZwpTabletSeatV2,
        event: zwp_tablet_seat_v2::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        // The tool's description (type, capabilities) follows this event, so
        // its state has to exist before those arrive.
        let zwp_tablet_seat_v2::Event::ToolAdded { id } = event else {
            return;
        };
        let client = this.get_client();
        let mut state = client.borrow_mut();
        let cursor_shape_device = state
            .globals
            .cursor_shape_manager
            .as_ref()
            .map(|cursor_shape_manager| cursor_shape_manager.get_tablet_tool_v2(&id, qh, ()));
        state
            .tablet_tools
            .insert(id.id(), TabletTool::new(id, cursor_shape_device));
    }

    event_created_child!(WaylandClientStatePtr, zwp_tablet_seat_v2::ZwpTabletSeatV2, [
        zwp_tablet_seat_v2::EVT_TABLET_ADDED_OPCODE => (zwp_tablet_v2::ZwpTabletV2, ()),
        zwp_tablet_seat_v2::EVT_TOOL_ADDED_OPCODE => (zwp_tablet_tool_v2::ZwpTabletToolV2, ()),
        zwp_tablet_seat_v2::EVT_PAD_ADDED_OPCODE => (zwp_tablet_pad_v2::ZwpTabletPadV2, ()),
    ]);
}

impl Dispatch<zwp_tablet_v2::ZwpTabletV2, ()> for WaylandClientStatePtr {
    fn event(
        _: &mut Self,
        tablet: &zwp_tablet_v2::ZwpTabletV2,
        event: zwp_tablet_v2::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let zwp_tablet_v2::Event::Removed = event {
            tablet.destroy();
        }
    }
}

/// A pad group and the objects created from it. Destroying a pad or a group
/// leaves the objects created from it alive, so they are destroyed explicitly
/// along with their pad.
pub(in crate::linux::wayland) struct TabletPadGroup {
    proxy: zwp_tablet_pad_group_v2::ZwpTabletPadGroupV2,
    rings: Vec<zwp_tablet_pad_ring_v2::ZwpTabletPadRingV2>,
    strips: Vec<zwp_tablet_pad_strip_v2::ZwpTabletPadStripV2>,
}

impl TabletPadGroup {
    fn destroy(self) {
        for ring in self.rings {
            ring.destroy();
        }
        for strip in self.strips {
            strip.destroy();
        }
        self.proxy.destroy();
    }
}

// Pads (the express keys on the tablet itself) aren't mapped to any input yet, but
// the compositor creates their objects regardless, so they still need dispatchers
// and have to be released when the pad goes away.
impl Dispatch<zwp_tablet_pad_v2::ZwpTabletPadV2, ()> for WaylandClientStatePtr {
    fn event(
        this: &mut Self,
        pad: &zwp_tablet_pad_v2::ZwpTabletPadV2,
        event: zwp_tablet_pad_v2::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let client = this.get_client();
        let mut state = client.borrow_mut();
        match event {
            zwp_tablet_pad_v2::Event::Group { pad_group } => {
                state
                    .tablet_pads
                    .entry(pad.id())
                    .or_default()
                    .push(TabletPadGroup {
                        proxy: pad_group,
                        rings: Vec::new(),
                        strips: Vec::new(),
                    });
            }
            zwp_tablet_pad_v2::Event::Removed => {
                for group in state.tablet_pads.remove(&pad.id()).unwrap_or_default() {
                    group.destroy();
                }
                pad.destroy();
            }
            _ => {}
        }
    }

    event_created_child!(WaylandClientStatePtr, zwp_tablet_pad_v2::ZwpTabletPadV2, [
        zwp_tablet_pad_v2::EVT_GROUP_OPCODE => (zwp_tablet_pad_group_v2::ZwpTabletPadGroupV2, ()),
    ]);
}

impl Dispatch<zwp_tablet_pad_group_v2::ZwpTabletPadGroupV2, ()> for WaylandClientStatePtr {
    fn event(
        this: &mut Self,
        group_proxy: &zwp_tablet_pad_group_v2::ZwpTabletPadGroupV2,
        event: zwp_tablet_pad_group_v2::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let client = this.get_client();
        let mut state = client.borrow_mut();
        let Some(group) = state
            .tablet_pads
            .values_mut()
            .flatten()
            .find(|group| group.proxy == *group_proxy)
        else {
            return;
        };
        match event {
            zwp_tablet_pad_group_v2::Event::Ring { ring } => group.rings.push(ring),
            zwp_tablet_pad_group_v2::Event::Strip { strip } => group.strips.push(strip),
            _ => {}
        }
    }

    event_created_child!(WaylandClientStatePtr, zwp_tablet_pad_group_v2::ZwpTabletPadGroupV2, [
        zwp_tablet_pad_group_v2::EVT_RING_OPCODE => (zwp_tablet_pad_ring_v2::ZwpTabletPadRingV2, ()),
        zwp_tablet_pad_group_v2::EVT_STRIP_OPCODE => (zwp_tablet_pad_strip_v2::ZwpTabletPadStripV2, ()),
    ]);
}

delegate_noop!(WaylandClientStatePtr: ignore zwp_tablet_pad_ring_v2::ZwpTabletPadRingV2);
delegate_noop!(WaylandClientStatePtr: ignore zwp_tablet_pad_strip_v2::ZwpTabletPadStripV2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tablet_tool_buttons_map_to_mouse_buttons() {
        assert_eq!(tablet_tool_button_to_gpui(0x14b), Some(MouseButton::Middle));
        assert_eq!(tablet_tool_button_to_gpui(0x14c), Some(MouseButton::Right));
        assert_eq!(
            tablet_tool_button_to_gpui(0x149),
            Some(MouseButton::Navigate(NavigationDirection::Back))
        );
        // Puck-shaped tools report ordinary mouse buttons.
        assert_eq!(tablet_tool_button_to_gpui(0x110), Some(MouseButton::Left));
    }

    #[test]
    fn test_pen_input_only_reports_announced_axes() {
        let mut tool = ToolAxes {
            kind: Some(PenTool::Eraser),
            pressure: 0.5,
            tilt: point(30.0, -15.0),
            distance: 0.25,
            ..ToolAxes::default()
        };
        assert_eq!(
            tool.pen_input(),
            Some(PenInput {
                tool: PenTool::Eraser,
                pressure: None,
                tilt: None,
                distance: None,
            })
        );

        tool.has_pressure = true;
        tool.has_tilt = true;
        tool.has_distance = true;
        assert_eq!(
            tool.pen_input(),
            Some(PenInput {
                tool: PenTool::Eraser,
                pressure: Some(0.5),
                tilt: Some(point(30.0, -15.0)),
                distance: Some(0.25),
            })
        );

        // A puck or lens behaves like a plain mouse.
        tool.kind = pen_tool_from_type(zwp_tablet_tool_v2::Type::Mouse);
        assert_eq!(tool.pen_input(), None);
    }
}
