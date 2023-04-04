use gpui::MutableAppContext;

#[derive(Debug, Default)]
pub struct StaffMode(pub bool);

impl std::ops::Deref for StaffMode {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Despite what the type system requires me to tell you, the init function will only ever be called once
pub fn staff_mode<F: FnMut(&mut MutableAppContext) + 'static>(
    cx: &mut MutableAppContext,
    mut init: F,
) {
    if **cx.default_global::<StaffMode>() {
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
