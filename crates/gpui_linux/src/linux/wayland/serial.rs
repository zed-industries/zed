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
}
