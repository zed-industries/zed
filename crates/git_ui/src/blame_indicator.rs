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

const UPDATE_DEBOUNCE: Duration = Duration::from_millis(50);

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

    fn on_editor_changed(&mut self, _editor: Entity<Editor>, cx: &mut Context<Self>) {
        // The editor notifies on every change (cursor movement, edits, blame
        // data arriving, even cursor blink); debounce so a burst runs one
        // blame lookup, and only re-render when the text actually changed.
        // Replacing the task drops the previous timer.
        self.blame_update = cx.spawn(async move |this, cx| {
            cx.background_executor().timer(UPDATE_DEBOUNCE).await;
            this.update(cx, |this, cx| {
                let previous_blame = this.current_blame.clone();
                this.update_blame(cx);
                if this.current_blame != previous_blame {
                    cx.notify();
                }
            })
            .ok();
        });
    }

    fn refresh(&mut self, cx: &mut Context<Self>) {
        let enabled = ProjectSettings::get_global(cx).git.status_bar_blame.enabled;
        let editor = self
            .active_editor
            .as_ref()
            .and_then(|editor| editor.upgrade());

        if let Some(editor) = editor.filter(|_| enabled) {
            self._observe_active_editor = Some(cx.observe(&editor, Self::on_editor_changed));
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

#[cfg(test)]
mod tests {
    use super::*;
    use editor::EditorMode;
    use git::blame::Blame;
    use git::repository::repo_path;
    use gpui::{Focusable as _, TestAppContext, UpdateGlobal as _};
    use language::language_settings::AllLanguageSettings;
    use multi_buffer::MultiBuffer;
    use project::{FakeFs, Project, WorktreeSettings};
    use serde_json::json;
    use settings::SettingsStore;
    use std::path::Path;
    use theme::LoadThemes;
    use util::path;
    use workspace::WorkspaceSettings;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(LoadThemes::JustBase, cx);
            AllLanguageSettings::register(cx);
            editor::init(cx);
            ProjectSettings::register(cx);
            WorktreeSettings::register(cx);
            WorkspaceSettings::register(cx);
        });
    }

    fn blame_entry(author: Option<&str>, summary: Option<&str>) -> BlameEntry {
        BlameEntry {
            sha: "1b1b1b".parse().unwrap(),
            range: 0..1,
            original_line_number: 0,
            author: author.map(Into::into),
            author_mail: None,
            author_time: Some(1_000_000_000),
            author_tz: Some("+0000".into()),
            committer_name: None,
            committer_email: None,
            committer_time: None,
            committer_tz: None,
            summary: summary.map(Into::into),
            previous: None,
            filename: "file.txt".into(),
        }
    }

    #[test]
    fn format_blame_matches_inline_blame_format() {
        let entry = blame_entry(Some("Alice"), Some("Fix the bug"));
        assert_eq!(
            BlameIndicator::format_blame(&entry, "3 minutes ago", true),
            SharedString::from("Alice, 3 minutes ago - Fix the bug"),
        );
        assert_eq!(
            BlameIndicator::format_blame(&entry, "3 minutes ago", false),
            SharedString::from("Alice, 3 minutes ago"),
        );

        // Multi-line summaries truncate to the first line, and a missing author
        // renders an empty name rather than panicking.
        let entry = blame_entry(None, Some("First line\nSecond line"));
        assert_eq!(
            BlameIndicator::format_blame(&entry, "3 minutes ago", true),
            SharedString::from(", 3 minutes ago - First line"),
        );
    }

    #[gpui::test]
    async fn test_blame_indicator_tracks_setting(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/my-repo"),
            json!({
                ".git": {},
                "file.txt": "fn main() {}\n",
            }),
        )
        .await;
        fs.set_blame_for_repo(
            Path::new(path!("/my-repo/.git")),
            vec![(
                repo_path("file.txt"),
                Blame {
                    entries: vec![blame_entry(Some("Alice"), Some("Initial commit"))],
                    ..Default::default()
                },
            )],
        );

        // Inline blame is disabled to prove the status bar setting starts
        // blame on its own.
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    let git = settings.git.get_or_insert_default();
                    git.inline_blame.get_or_insert_default().enabled = Some(false);
                    let blame = git.status_bar_blame.get_or_insert_default();
                    blame.enabled = Some(true);
                    blame.show_commit_summary = Some(true);
                });
            });
        });

        let project = Project::test(fs, [path!("/my-repo").as_ref()], cx).await;
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/my-repo/file.txt"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        let (editor, cx) = cx.add_window_view(|window, cx| {
            Editor::new(
                EditorMode::full(),
                multi_buffer,
                Some(project.clone()),
                window,
                cx,
            )
        });

        // Attach the indicator before any blame data exists; it should show
        // nothing yet.
        let indicator = cx.new(|cx| BlameIndicator::new(cx));
        indicator.update_in(cx, |indicator, window, cx| {
            indicator.set_active_pane_item(Some(&editor as &dyn ItemHandle), window, cx);
        });
        indicator.read_with(cx, |indicator, _| {
            assert_eq!(indicator.current_blame, None);
        });

        // Focus listeners only fire on a draw of an active window — neither of
        // which test windows do on their own.
        editor.update_in(cx, |editor, window, cx| {
            window.activate_window();
            window.focus(&editor.focus_handle(cx), cx);
        });
        cx.run_until_parked();
        cx.update(|window, cx| {
            let _ = window.draw(cx);
        });
        cx.run_until_parked();

        // The indicator's lookup sits behind a debounce timer, and test time
        // is virtual — advance it explicitly.
        cx.executor().advance_clock(UPDATE_DEBOUNCE);

        let expected = editor
            .update(cx, |editor, cx| {
                editor.blame_entry_for_row(MultiBufferRow(0), cx)
            })
            .expect("row 0 should have a blame entry");

        // Anchor to the fixture so the comparison below can't pass vacuously.
        assert_eq!(expected.author.as_deref(), Some("Alice"));

        // The fixture commit is decades old, so the relative phrasing is stable.
        let relative = blame_entry_relative_timestamp(&expected);

        indicator.read_with(cx, |indicator, _| {
            assert_eq!(
                indicator.current_blame,
                Some(BlameIndicator::format_blame(&expected, &relative, true)),
            );
        });

        // Disabling the setting clears the indicator via its settings observer.
        cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .git
                        .get_or_insert_default()
                        .status_bar_blame
                        .get_or_insert_default()
                        .enabled = Some(false);
                });
            });
        });
        cx.run_until_parked();

        indicator.read_with(cx, |indicator, _| {
            assert_eq!(indicator.current_blame, None);
        });
    }
}
