use collections::HashMap;
use wayland_client::{Proxy, protocol::wl_surface};

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) enum SerialKind {
    DataDevice,
    InputMethod,
    MouseEnter,
    MousePress,
    KeyPress,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Serial(u32);

impl Serial {
    pub(super) fn as_raw(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SelectionSerial(Serial);

impl SelectionSerial {
    pub(super) fn as_raw(self) -> u32 {
        self.0.as_raw()
    }
}

/// The user interaction an `xdg_activation_v1` token request must reference.
#[derive(Clone, Debug)]
pub(crate) struct ActivationContext {
    pub(crate) serial: Serial,
    pub(crate) requesting_surface: wl_surface::WlSurface,
}

#[derive(Debug)]
struct Press {
    serial: Serial,
    focused_surface: Option<wl_surface::WlSurface>,
}

#[derive(Debug)]
/// Helper for tracking of different serial kinds.
pub(crate) struct SerialTracker {
    serials: HashMap<SerialKind, Serial>,
    latest_press: Option<Press>,
}

impl SerialTracker {
    pub fn new() -> Self {
        Self {
            serials: HashMap::default(),
            latest_press: None,
        }
    }

    pub fn update(&mut self, kind: SerialKind, value: u32) {
        self.update_with_focused_surface(kind, value, None);
    }

    /// Like [`Self::update`], but for key and pointer presses also records the surface
    /// that had focus, which activation token requests must name alongside the serial.
    pub fn update_with_focused_surface(
        &mut self,
        kind: SerialKind,
        value: u32,
        focused_surface: Option<wl_surface::WlSurface>,
    ) {
        let serial = Serial(value);

        if matches!(&kind, SerialKind::KeyPress | SerialKind::MousePress) {
            self.latest_press = Some(Press {
                serial,
                focused_surface,
            });
        }

        self.serials.insert(kind, serial);
    }

    /// Returns the latest tracked serial of the provided [`SerialKind`].
    ///
    /// Returns a serial with a raw value of 0 if the kind has not been tracked.
    pub fn get(&self, kind: SerialKind) -> Serial {
        self.serials.get(&kind).copied().unwrap_or(Serial(0))
    }

    pub fn selection_serial(&self) -> Option<SelectionSerial> {
        self.latest_press
            .as_ref()
            .map(|press| SelectionSerial(press.serial))
    }

    /// Returns the latest press if a still-alive surface had focus when it arrived.
    pub fn activation_context(&self) -> Option<ActivationContext> {
        let press = self.latest_press.as_ref()?;
        let surface = press
            .focused_surface
            .as_ref()
            .filter(|surface| surface.is_alive())?;
        Some(ActivationContext {
            serial: press.serial,
            requesting_surface: surface.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn raw_selection_serial(serial_tracker: &SerialTracker) -> Option<u32> {
        serial_tracker
            .selection_serial()
            .map(SelectionSerial::as_raw)
    }

    #[test]
    fn test_selection_serial_ignores_unrelated_serial_kinds() {
        let mut serial_tracker = SerialTracker::new();
        serial_tracker.update(SerialKind::MousePress, 3783);
        serial_tracker.update(SerialKind::InputMethod, 5011);
        serial_tracker.update(SerialKind::KeyPress, 3787);
        serial_tracker.update(SerialKind::MouseEnter, 6000);
        serial_tracker.update(SerialKind::DataDevice, 7000);

        assert_eq!(raw_selection_serial(&serial_tracker), Some(3787));
    }

    #[test]
    fn test_selection_serial_uses_mouse_press_without_key_press() {
        let mut serial_tracker = SerialTracker::new();
        serial_tracker.update(SerialKind::MousePress, 3783);

        assert_eq!(raw_selection_serial(&serial_tracker), Some(3783));
    }

    #[test]
    fn test_selection_serial_uses_event_arrival_order_across_rollover() {
        let mut serial_tracker = SerialTracker::new();
        serial_tracker.update(SerialKind::KeyPress, 0xffff_fff0);
        serial_tracker.update(SerialKind::MousePress, 0x0000_0010);

        assert_eq!(raw_selection_serial(&serial_tracker), Some(0x0000_0010));
    }

    #[test]
    fn test_selection_serial_is_unavailable_without_eligible_input() {
        let mut serial_tracker = SerialTracker::new();

        assert_eq!(raw_selection_serial(&serial_tracker), None);

        serial_tracker.update(SerialKind::InputMethod, 5011);
        serial_tracker.update(SerialKind::MouseEnter, 6000);
        serial_tracker.update(SerialKind::DataDevice, 7000);

        assert_eq!(raw_selection_serial(&serial_tracker), None);
    }

    #[test]
    fn test_zero_is_a_valid_selection_serial() {
        let mut serial_tracker = SerialTracker::new();
        serial_tracker.update(SerialKind::KeyPress, 0);

        assert_eq!(raw_selection_serial(&serial_tracker), Some(0));
    }
}
