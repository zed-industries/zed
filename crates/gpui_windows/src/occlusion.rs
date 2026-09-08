use parking_lot::{Condvar, Mutex};

/// Whether anything can actually see this process's windows.
pub(crate) struct OcclusionState {
    inputs: Mutex<OcclusionInputs>,
    /// Signalled when the last thing hiding the windows goes away.
    became_visible: Condvar,
}

struct OcclusionInputs {
    /// Whether this session's display is powered on, from
    /// `GUID_SESSION_DISPLAY_STATUS`.
    display_on: bool,
}

impl Default for OcclusionInputs {
    fn default() -> Self {
        Self { display_on: true }
    }
}

impl OcclusionInputs {
    fn is_occluded(&self) -> bool {
        !self.display_on
    }
}

impl OcclusionState {
    pub(crate) fn new() -> Self {
        Self {
            inputs: Mutex::new(OcclusionInputs::default()),
            became_visible: Condvar::new(),
        }
    }

    pub(crate) fn set_display_on(&self, display_on: bool) {
        self.update(|inputs| inputs.display_on = display_on);
    }

    /// Applies a change to the inputs and releases anything parked in
    /// [`Self::wait_until_visible`] if that made the windows visible again.
    fn update(&self, change: impl FnOnce(&mut OcclusionInputs)) {
        let mut inputs = self.inputs.lock();
        let was_occluded = inputs.is_occluded();
        change(&mut inputs);
        if was_occluded && !inputs.is_occluded() {
            drop(inputs);
            self.became_visible.notify_all();
        }
    }

    pub(crate) fn wait_until_visible(&self) {
        let mut inputs = self.inputs.lock();
        while inputs.is_occluded() {
            self.became_visible.wait(&mut inputs);
        }
    }
}
