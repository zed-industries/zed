use std::time::Duration;

use anyhow::Result;

use gpui::{
    percentage, Animation, AnimationExt, EventEmitter, FocusHandle, Focusable, Subscription, Task,
    Transformation,
};
use ui::{
    div, v_flex, Color, Context, Element, Icon, IconName, IconSize, IntoElement, ParentElement,
    Render, Styled,
};

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
        v_flex()
            .size_full()
            .gap_1()
            .items_center()
            .child("Starting a debug adapter")
            .child(
                Icon::new(IconName::ArrowCircle)
                    .color(Color::Info)
                    .with_animation(
                        "arrow-circle",
                        Animation::new(Duration::from_secs(2)).repeat(),
                        |icon, delta| icon.transform(Transformation::rotate(percentage(delta))),
                    )
                    .into_any_element(),
            )
    }
}
