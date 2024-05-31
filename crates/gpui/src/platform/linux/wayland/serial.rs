use std::time::Instant;

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
    time: Instant,
}

impl SerialData {
    fn new(value: u32) -> Self {
        Self {
            serial: value,
            time: Instant::now(),
        }
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

    /// Returns the newest serial of any of the provided [`SerialKind`]
    pub fn get_newest_of(&self, kinds: &[SerialKind]) -> u32 {
        kinds
            .iter()
            .filter_map(|kind| self.serials.get(&kind))
            .max_by_key(|serial_data| serial_data.time)
            .map(|serial_data| serial_data.serial)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serial_tracker() {
        let mut tracker = SerialTracker::new();

        tracker.update(SerialKind::KeyPress, 100);
        tracker.update(SerialKind::MousePress, 50);
        tracker.update(SerialKind::MouseEnter, 300);

        assert_eq!(
            tracker.get_newest_of(&[SerialKind::KeyPress, SerialKind::MousePress]),
            50
        );
        assert_eq!(tracker.get(SerialKind::DataDevice), 0);

        tracker.update(SerialKind::KeyPress, 2000);
        assert_eq!(tracker.get(SerialKind::KeyPress), 2000);
        assert_eq!(
            tracker.get_newest_of(&[SerialKind::KeyPress, SerialKind::MousePress]),
            2000
        );
    }
}
