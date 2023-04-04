use gpui::MutableAppContext;

#[derive(Debug, Default)]
pub struct StaffMode(pub bool);

impl std::ops::Deref for StaffMode {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Despite what the type system requires me to tell you, the init function will only be called a once
/// as soon as we know that the staff mode is enabled.
pub fn staff_mode<S: StaffModeConfiguration, F: FnMut(&mut MutableAppContext) + 'static>(
    cx: &mut MutableAppContext,
    mut init: F,
) {
    if !S::staff_only() || **cx.default_global::<StaffMode>() {
        init(cx)
    } else {
        let mut once = Some(());
        cx.observe_global::<StaffMode, _>(move |cx| {
            if **cx.global::<StaffMode>() && once.take().is_some() {
                init(cx);
            }
        })
        .detach();
    }
}

/// Immediately checks and runs the init function if the staff mode is not enabled.
pub fn not_staff_mode<S: StaffModeConfiguration, F: FnOnce(&mut MutableAppContext) + 'static>(
    cx: &mut MutableAppContext,
    init: F,
) {
    if !S::staff_only() || !**cx.default_global::<StaffMode>() {
        init(cx)
    }
}

pub trait StaffModeConfiguration {
    fn staff_only() -> bool {
        true
    }
}

pub enum Copilot {}

impl StaffModeConfiguration for Copilot {}
