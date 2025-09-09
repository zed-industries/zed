use std::rc::Rc;

use collections::HashMap;
use gpui::{Entity, WeakEntity};
use project::debugger::session::{ThreadId, ThreadStatus};
use ui::{CommonAnimationExt, ContextMenu, DropdownMenu, DropdownStyle, Indicator, prelude::*};
use util::{maybe, truncate_and_trailoff};

use crate::{
    debugger_panel::DebugPanel,
    session::{DebugSession, running::RunningState},
};

struct SessionListEntry {
    ancestors: Vec<Entity<DebugSession>>,
    leaf: Entity<DebugSession>,
}

impl SessionListEntry {
    pub(crate) fn label_element(&self, depth: usize, cx: &mut App) -> AnyElement {
        const MAX_LABEL_CHARS: usize = 150;

        let mut label = String::new();
        for ancestor in &self.ancestors {
            label.push_str(&ancestor.update(cx, |ancestor, cx| {
                ancestor.label(cx).unwrap_or("(child)".into())
            }));
            label.push_str(" » ");
        }
        label.push_str(
            &self
                .leaf
                .update(cx, |leaf, cx| leaf.label(cx).unwrap_or("(child)".into())),
        );
        let label = truncate_and_trailoff(&label, MAX_LABEL_CHARS);

        let is_terminated = self
            .leaf
            .read(cx)
            .running_state
            .read(cx)
            .session()
            .read(cx)
            .is_terminated();
        let icon = {
            if is_terminated {
                Some(Indicator::dot().color(Color::Error))
            } else {
                match self
                    .leaf
                    .read(cx)
                    .running_state
                    .read(cx)
                    .thread_status(cx)
                    .unwrap_or_default()
                {
                    project::debugger::session::ThreadStatus::Stopped => {
                        Some(Indicator::dot().color(Color::Conflict))
                    }
                    _ => Some(Indicator::dot().color(Color::Success)),
                }
            }
        };

        h_flex()
            .id("session-label")
            .ml(depth * px(16.0))
            .gap_2()
            .when_some(icon, |this, indicator| this.child(indicator))
            .justify_between()
            .child(
                Label::new(label)
                    .size(LabelSize::Small)
                    .when(is_terminated, |this| this.strikethrough()),
            )
            .into_any_element()
    }
}

impl DebugPanel {
    fn dropdown_label(label: impl Into<SharedString>) -> Label {
        const MAX_LABEL_CHARS: usize = 50;
        let label = truncate_and_trailoff(&label.into(), MAX_LABEL_CHARS);
        Label::new(label).size(LabelSize::Small)
    }

    pub fn render_session_menu(
        &mut self,
        active_session: Option<Entity<DebugSession>>,
        running_state: Option<Entity<RunningState>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        let running_state = running_state?;

        let mut session_entries = Vec::with_capacity(self.sessions_with_children.len() * 3);
        let mut sessions_with_children = self.sessions_with_children.iter().peekable();

        while let Some((root, children)) = sessions_with_children.next() {
            let root_entry = if let Ok([single_child]) = <&[_; 1]>::try_from(children.as_slice())
                && let Some(single_child) = single_child.upgrade()
                && single_child.read(cx).quirks.compact
            {
                sessions_with_children.next();
                SessionListEntry {
                    leaf: single_child.clone(),
                    ancestors: vec![root.clone()],
                }
            } else {
                SessionListEntry {
                    leaf: root.clone(),
                    ancestors: Vec::new(),
                }
            };
            session_entries.push(root_entry);
        }

        let weak = cx.weak_entity();
        let trigger_label = if let Some(active_session) = active_session.clone() {
            active_session.update(cx, |active_session, cx| {
                active_session.label(cx).unwrap_or("(child)".into())
            })
        } else {
            SharedString::new_static("Unknown Session")
        };
        let running_state = running_state.read(cx);

        let is_terminated = running_state.session().read(cx).is_terminated();
        let is_started = active_session
            .is_some_and(|session| session.read(cx).session(cx).read(cx).is_started());

        let session_state_indicator = if is_terminated {
            Indicator::dot().color(Color::Error).into_any_element()
        } else if !is_started {
            Icon::new(IconName::ArrowCircle)
                .size(IconSize::Small)
                .color(Color::Muted)
                .with_rotate_animation(2)
                .into_any_element()
        } else {
            match running_state.thread_status(cx).unwrap_or_default() {
                ThreadStatus::Stopped => Indicator::dot().color(Color::Conflict).into_any_element(),
                _ => Indicator::dot().color(Color::Success).into_any_element(),
            }
        };

        let trigger = h_flex()
            .gap_2()
            .child(session_state_indicator)
            .justify_between()
            .child(
                DebugPanel::dropdown_label(trigger_label)
                    .when(is_terminated, |this| this.strikethrough()),
            )
            .into_any_element();

        let menu = DropdownMenu::new_with_element(
            "debugger-session-list",
            trigger,
            ContextMenu::build(window, cx, move |mut this, _, cx| {
                let context_menu = cx.weak_entity();
                let mut session_depths = HashMap::default();
                for session_entry in session_entries {
                    let session_id = session_entry.leaf.read(cx).session_id(cx);
                    let parent_depth = session_entry
                        .ancestors
                        .first()
                        .unwrap_or(&session_entry.leaf)
                        .read(cx)
                        .session(cx)
                        .read(cx)
                        .parent_id(cx)
                        .and_then(|parent_id| session_depths.get(&parent_id).cloned());
                    let self_depth = *session_depths
                        .entry(session_id)
                        .or_insert_with(|| parent_depth.map(|depth| depth + 1).unwrap_or(0usize));
                    this = this.custom_entry(
                        {
                            let weak = weak.clone();
                            let context_menu = context_menu.clone();
                            let ancestors: Rc<[_]> = session_entry
                                .ancestors
                                .iter()
                                .map(|session| session.downgrade())
                                .collect();
                            let leaf = session_entry.leaf.downgrade();
                            move |window, cx| {
                                Self::render_session_menu_entry(
                                    weak.clone(),
                                    context_menu.clone(),
                                    ancestors.clone(),
                                    leaf.clone(),
                                    self_depth,
                                    window,
                                    cx,
                                )
                            }
                        },
                        {
                            let weak = weak.clone();
                            let leaf = session_entry.leaf.clone();
                            move |window, cx| {
                                weak.update(cx, |panel, cx| {
                                    panel.activate_session(leaf.clone(), window, cx);
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
        .handle(self.session_picker_menu_handle.clone());

        Some(menu)
    }

    fn render_session_menu_entry(
        weak: WeakEntity<DebugPanel>,
        context_menu: WeakEntity<ContextMenu>,
        ancestors: Rc<[WeakEntity<DebugSession>]>,
        leaf: WeakEntity<DebugSession>,
        self_depth: usize,
        _window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let Some(session_entry) = maybe!({
            let ancestors = ancestors
                .iter()
                .map(|ancestor| ancestor.upgrade())
                .collect::<Option<Vec<_>>>()?;
            let leaf = leaf.upgrade()?;
            Some(SessionListEntry { ancestors, leaf })
        }) else {
            return div().into_any_element();
        };

        let id: SharedString = format!(
            "debug-session-{}",
            session_entry.leaf.read(cx).session_id(cx).0
        )
        .into();
        let session_entity_id = session_entry.leaf.entity_id();

        h_flex()
            .w_full()
            .group(id.clone())
            .justify_between()
            .child(session_entry.label_element(self_depth, cx))
            .child(
                IconButton::new("close-debug-session", IconName::Close)
                    .visible_on_hover(id)
                    .icon_size(IconSize::Small)
                    .on_click({
                        move |_, window, cx| {
                            weak.update(cx, |panel, cx| {
                                panel.close_session(session_entity_id, window, cx);
                            })
                            .ok();
                            context_menu
                                .update(cx, |this, cx| {
                                    this.cancel(&Default::default(), window, cx);
                                })
                                .ok();
                        }
                    }),
            )
            .into_any_element()
    }

    pub(crate) fn render_thread_dropdown(
        &self,
        running_state: &Entity<RunningState>,
        threads: Vec<(dap::Thread, ThreadStatus)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<DropdownMenu> {
        const MAX_LABEL_CHARS: usize = 150;

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
                            let entry_name = truncate_and_trailoff(&entry_name, MAX_LABEL_CHARS);

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
