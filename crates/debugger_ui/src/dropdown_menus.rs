use std::time::Duration;

use gpui::{Animation, AnimationExt as _, Entity, Transformation, percentage};
use project::debugger::session::{ThreadId, ThreadStatus};
use ui::{ContextMenu, DropdownMenu, DropdownStyle, Indicator, prelude::*};

use crate::{
    debugger_panel::DebugPanel,
    session::{DebugSession, running::RunningState},
};

impl DebugPanel {
    fn dropdown_label(label: impl Into<SharedString>) -> Label {
        Label::new(label).size(LabelSize::Small)
    }

    pub fn render_session_menu(
        &mut self,
        active_session: Option<Entity<DebugSession>>,
        running_state: Option<Entity<RunningState>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        if let Some(running_state) = running_state {
            let sessions = self.sessions().clone();
            let weak = cx.weak_entity();
            let running_state = running_state.read(cx);
            let label = if let Some(active_session) = active_session.clone() {
                active_session.read(cx).session(cx).read(cx).label()
            } else {
                SharedString::new_static("Unknown Session")
            };

            let is_terminated = running_state.session().read(cx).is_terminated();
            let is_started = active_session
                .is_some_and(|session| session.read(cx).session(cx).read(cx).is_started());

            let session_state_indicator = if is_terminated {
                Indicator::dot().color(Color::Error).into_any_element()
            } else if !is_started {
                Icon::new(IconName::ArrowCircle)
                    .size(IconSize::Small)
                    .color(Color::Muted)
                    .with_animation(
                        "arrow-circle",
                        Animation::new(Duration::from_secs(2)).repeat(),
                        |icon, delta| icon.transform(Transformation::rotate(percentage(delta))),
                    )
                    .into_any_element()
            } else {
                match running_state.thread_status(cx).unwrap_or_default() {
                    ThreadStatus::Stopped => {
                        Indicator::dot().color(Color::Conflict).into_any_element()
                    }
                    _ => Indicator::dot().color(Color::Success).into_any_element(),
                }
            };

            let trigger = h_flex()
                .gap_2()
                .child(session_state_indicator)
                .justify_between()
                .child(
                    DebugPanel::dropdown_label(label)
                        .when(is_terminated, |this| this.strikethrough()),
                )
                .into_any_element();

            Some(
                DropdownMenu::new_with_element(
                    "debugger-session-list",
                    trigger,
                    ContextMenu::build(window, cx, move |mut this, _, cx| {
                        let context_menu = cx.weak_entity();
                        for session in sessions.into_iter() {
                            let weak_session = session.downgrade();
                            let weak_session_id = weak_session.entity_id();

                            this = this.custom_entry(
                                {
                                    let weak = weak.clone();
                                    let context_menu = context_menu.clone();
                                    move |_, cx| {
                                        weak_session
                                            .read_with(cx, |session, cx| {
                                                let context_menu = context_menu.clone();
                                                let id: SharedString = format!(
                                                    "debug-session-{}",
                                                    session.session_id(cx).0
                                                )
                                                .into();
                                                h_flex()
                                                    .w_full()
                                                    .group(id.clone())
                                                    .justify_between()
                                                    .child(session.label_element(cx))
                                                    .child(
                                                        IconButton::new(
                                                            "close-debug-session",
                                                            IconName::Close,
                                                        )
                                                        .visible_on_hover(id.clone())
                                                        .icon_size(IconSize::Small)
                                                        .on_click({
                                                            let weak = weak.clone();
                                                            move |_, window, cx| {
                                                                weak.update(cx, |panel, cx| {
                                                                    panel.close_session(
                                                                        weak_session_id,
                                                                        window,
                                                                        cx,
                                                                    );
                                                                })
                                                                .ok();
                                                                context_menu
                                                                    .update(cx, |this, cx| {
                                                                        this.cancel(
                                                                            &Default::default(),
                                                                            window,
                                                                            cx,
                                                                        );
                                                                    })
                                                                    .ok();
                                                            }
                                                        }),
                                                    )
                                                    .into_any_element()
                                            })
                                            .unwrap_or_else(|_| div().into_any_element())
                                    }
                                },
                                {
                                    let weak = weak.clone();
                                    move |window, cx| {
                                        weak.update(cx, |panel, cx| {
                                            panel.activate_session(session.clone(), window, cx);
                                        })
                                        .ok();
                                    }
                                },
                            );
                        }
                        this
                    }),
                )
                .style(DropdownStyle::Ghost)
                .handle(self.session_picker_menu_handle.clone()),
            )
        } else {
            None
        }
    }

    pub(crate) fn render_thread_dropdown(
        &self,
        running_state: &Entity<RunningState>,
        threads: Vec<(dap::Thread, ThreadStatus)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<DropdownMenu> {
        let running_state = running_state.clone();
        let running_state_read = running_state.read(cx);
        let thread_id = running_state_read.thread_id();
        let session = running_state_read.session();
        let session_id = session.read(cx).session_id();
        let session_terminated = session.read(cx).is_terminated();
        let selected_thread_name = threads
            .iter()
            .find(|(thread, _)| thread_id.map(|id| id.0) == Some(thread.id))
            .map(|(thread, _)| {
                thread
                    .name
                    .is_empty()
                    .then(|| format!("Tid: {}", thread.id))
                    .unwrap_or_else(|| thread.name.clone())
            });

        if let Some(selected_thread_name) = selected_thread_name {
            let trigger = DebugPanel::dropdown_label(selected_thread_name).into_any_element();
            Some(
                DropdownMenu::new_with_element(
                    ("thread-list", session_id.0),
                    trigger,
                    ContextMenu::build(window, cx, move |mut this, _, _| {
                        for (thread, _) in threads {
                            let running_state = running_state.clone();
                            let thread_id = thread.id;
                            let entry_name = thread
                                .name
                                .is_empty()
                                .then(|| format!("Tid: {}", thread.id))
                                .unwrap_or_else(|| thread.name);

                            this = this.entry(entry_name, None, move |window, cx| {
                                running_state.update(cx, |running_state, cx| {
                                    running_state.select_thread(ThreadId(thread_id), window, cx);
                                });
                            });
                        }
                        this
                    }),
                )
                .disabled(session_terminated)
                .style(DropdownStyle::Ghost)
                .handle(self.thread_picker_menu_handle.clone()),
            )
        } else {
            None
        }
    }
}
