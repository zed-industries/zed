use anyhow::Result;

use gpui::{EventEmitter, FocusHandle, Focusable, Subscription, Task};
use ui::{div, Context, Element, ParentElement, Render, Styled};

pub(super) struct StartingState {
    focus_handle: FocusHandle,
    _notify_parent: Task<Result<()>>,
}

pub(crate) enum StartingEvent {
    Finished(()),
}

impl EventEmitter<StartingEvent> for StartingState {}
impl StartingState {
    pub(crate) fn new(task: Task<Result<()>>, cx: &mut Context<Self>) -> Self {
        let _notify_parent = cx.spawn(move |this, mut cx| async move {
            task.await?;
            this.update(&mut cx, |_, cx| cx.emit(StartingEvent::Finished(())));
            Ok(())
        });
        Self {
            focus_handle: cx.focus_handle(),
            _notify_parent,
        }
    }
}

impl Focusable for StartingState {
    fn focus_handle(&self, cx: &ui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for StartingState {
    fn render(
        &mut self,
        window: &mut ui::Window,
        cx: &mut ui::Context<'_, Self>,
    ) -> impl ui::IntoElement {
        div().size_full().child("Starting a debug adapter")
    }
}
