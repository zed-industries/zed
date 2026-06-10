use std::time::Duration;

use editor::{Editor, ToPoint as _};
use git::blame::BlameEntry;
use gpui::{
    App, Context, Empty, Entity, IntoElement, ParentElement, Render, SharedString, Styled,
    Subscription, Task, WeakEntity, Window,
};
use multi_buffer::MultiBufferRow;
use project::project_settings::ProjectSettings;
use settings::Settings as _;
use ui::{Label, h_flex, prelude::*};
use workspace::{HideStatusItem, StatusItemView, item::ItemHandle};

use crate::commit_tooltip::blame_entry_relative_timestamp;

pub struct BlameIndicator {
    active_editor: Option<WeakEntity<Editor>>,
    current_blame: Option<SharedString>,
    _observe_active_editor: Option<Subscription>,
    blame_update: Task<()>,
}

impl BlameIndicator {
    pub fn new(cx: &mut Context<Self>) -> Self {
        cx.observe_global::<settings::SettingsStore>(|this, cx| {
            this.refresh(cx);
            cx.notify();
        })
        .detach();
        Self {
            active_editor: None,
            current_blame: None,
            _observe_active_editor: None,
            blame_update: Task::ready(()),
        }
    }

    fn on_editor_event(
        &mut self,
        _editor: Entity<Editor>,
        event: &editor::EditorEvent,
        cx: &mut Context<Self>,
    ) {
        if let editor::EditorEvent::SelectionsChanged { .. } = event {
            // Debounce so rapid cursor movement doesn't run a blame lookup per
            // keystroke; replacing the task drops the previous timer.
            self.blame_update = cx.spawn(async move |this, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(50))
                    .await;
                this.update(cx, |this, cx| {
                    this.update_blame(cx);
                    cx.notify();
                })
                .ok();
            });
        }
    }

    fn refresh(&mut self, cx: &mut Context<Self>) {
        let enabled = ProjectSettings::get_global(cx).git.status_bar_blame.enabled;
        let editor = self
            .active_editor
            .as_ref()
            .and_then(|editor| editor.upgrade());

        if let Some(editor) = editor.filter(|_| enabled) {
            self._observe_active_editor = Some(cx.subscribe(&editor, Self::on_editor_event));
            self.update_blame(cx);
        } else {
            self._observe_active_editor = None;
            self.current_blame = None;
        }
    }

    fn update_blame(&mut self, cx: &mut App) {
        let Some(editor) = self
            .active_editor
            .as_ref()
            .and_then(|editor| editor.upgrade())
        else {
            self.current_blame = None;
            return;
        };

        let row = {
            let editor = editor.read(cx);
            let cursor = editor.selections.newest_anchor().head();
            let snapshot = editor.buffer().read(cx).read(cx);
            cursor.to_point(&snapshot).row
        };
        let entry = editor.update(cx, |editor, cx| {
            editor.blame_entry_for_row(MultiBufferRow(row), cx)
        });
        let show_summary = ProjectSettings::get_global(cx)
            .git
            .status_bar_blame
            .show_commit_summary;

        self.current_blame = entry.map(|entry| {
            let relative = blame_entry_relative_timestamp(&entry);
            Self::format_blame(&entry, &relative, show_summary)
        });
    }

    fn format_blame(entry: &BlameEntry, relative: &str, show_summary: bool) -> SharedString {
        let author = entry.author.as_deref().unwrap_or_default();

        match entry.summary.as_deref() {
            Some(summary) if show_summary => {
                let first_line = summary.lines().next().unwrap_or(summary);
                format!("{author}, {relative} - {first_line}")
            }
            _ => format!("{author}, {relative}"),
        }
        .into()
    }
}

impl Render for BlameIndicator {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let Some(text) = self.current_blame.clone() else {
            return Empty.into_any_element();
        };

        h_flex()
            .min_w_0()
            .overflow_x_hidden()
            .child(Label::new(text).size(LabelSize::Small).truncate())
            .into_any_element()
    }
}

impl StatusItemView for BlameIndicator {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.active_editor = active_pane_item
            .and_then(|item| item.act_as::<Editor>(cx))
            .map(|editor| editor.downgrade());
        self.refresh(cx);
        cx.notify();
    }

    fn hide_setting(&self, _: &App) -> Option<HideStatusItem> {
        Some(HideStatusItem::new(|settings| {
            settings
                .git
                .get_or_insert_default()
                .status_bar_blame
                .get_or_insert_default()
                .enabled = Some(false);
        }))
    }
}
