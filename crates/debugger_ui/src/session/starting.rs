use std::time::Duration;

use anyhow::Result;

use dap::client::SessionId;
use gpui::{
    Animation, AnimationExt, Entity, EventEmitter, FocusHandle, Focusable, Task, Transformation,
    percentage,
};
use project::debugger::session::Session;
use ui::{Color, Context, Icon, IconName, IntoElement, ParentElement, Render, Styled, v_flex};

pub(crate) struct StartingState {
    focus_handle: FocusHandle,
    pub(super) session_id: SessionId,
    _notify_parent: Task<()>,
}

pub(crate) enum StartingEvent {
    Failed,
    Finished(Entity<Session>),
}

impl EventEmitter<StartingEvent> for StartingState {}

impl StartingState {
    pub(crate) fn new(
        session_id: SessionId,
        task: Task<Result<Entity<Session>>>,
        cx: &mut Context<Self>,
    ) -> Self {
        let _notify_parent = cx.spawn(async move |this, cx| {
            let entity = task.await;

            this.update(cx, |_, cx| {
                if let Ok(entity) = entity {
                    cx.emit(StartingEvent::Finished(entity))
                } else {
                    cx.emit(StartingEvent::Failed)
                }
            })
            .ok();
        });
        Self {
            session_id,
            focus_handle: cx.focus_handle(),
            _notify_parent,
        }
    }
}

impl Focusable for StartingState {
    fn focus_handle(&self, _: &ui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for StartingState {
    fn render(
        &mut self,
        _window: &mut ui::Window,
        _cx: &mut ui::Context<'_, Self>,
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
