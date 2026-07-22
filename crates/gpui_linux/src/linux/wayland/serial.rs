use collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) enum SerialKind {
    DataDevice,
    InputMethod,
    MouseEnter,
    MousePress,
    KeyPress,
}

#[derive(Debug)]
struct SerialData {
    serial: u32,
}

impl SerialData {
    fn new(value: u32) -> Self {
        Self { serial: value }
    }
}

#[derive(Debug)]
/// Helper for tracking of different serial kinds.
pub(crate) struct SerialTracker {
    serials: HashMap<SerialKind, SerialData>,
}

impl SerialTracker {
    pub fn new() -> Self {
        Self {
            serials: HashMap::default(),
        }
    }

    pub fn update(&mut self, kind: SerialKind, value: u32) {
        self.serials.insert(kind, SerialData::new(value));
    }

    /// Returns the latest tracked serial of the provided [`SerialKind`]
    ///
    /// Will return 0 if not tracked.
    pub fn get(&self, kind: SerialKind) -> u32 {
        self.serials
            .get(&kind)
            .map(|serial_data| serial_data.serial)
            .unwrap_or(0)
    }

    /// Returns the most recent serial across all tracked kinds.
    ///
    /// Wayland compositor serial numbers are monotonically increasing, so the
    /// highest value is always the most recently received one. This is the
    /// correct serial to use for [`set_selection`] when the triggering event
    /// may have been a mouse press rather than a key press: using 0 (the
    /// default when a kind has never been seen) causes compositors to silently
    /// reject the request.
    ///
    /// Returns 0 only if no serial of any kind has been received yet.
    pub fn get_latest(&self) -> u32 {
        self.serials
            .values()
            .map(|serial_data| serial_data.serial)
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_serial_ignores_unrelated_serial_kinds() {
        let mut serial_tracker = SerialTracker::new();
        serial_tracker.update(SerialKind::MousePress, 3783);
        serial_tracker.update(SerialKind::InputMethod, 5011);
        serial_tracker.update(SerialKind::KeyPress, 3787);
        serial_tracker.update(SerialKind::MouseEnter, 6000);
        serial_tracker.update(SerialKind::DataDevice, 7000);

        assert_eq!(serial_tracker.get_latest(), 3787);
    }

    #[test]
    fn test_clipboard_serial_uses_mouse_press_without_key_press() {
        let mut serial_tracker = SerialTracker::new();
        serial_tracker.update(SerialKind::MousePress, 3783);

        assert_eq!(serial_tracker.get_latest(), 3783);
    }

    #[test]
    fn test_clipboard_serial_uses_latest_eligible_event_across_rollover() {
        let mut serial_tracker = SerialTracker::new();
        serial_tracker.update(SerialKind::KeyPress, 0xffff_fff0);
        serial_tracker.update(SerialKind::MousePress, 0x0000_0010);

        assert_eq!(serial_tracker.get_latest(), 0x0000_0010);
    }
}
