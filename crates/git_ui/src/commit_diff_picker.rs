use anyhow::Result;
use buffer_diff::{BufferDiff, BufferDiffSnapshot};
use editor::{Editor, EditorEvent, MultiBuffer};
use fuzzy::StringMatchCandidate;
use git::repository::{FileHistory, FileHistoryEntry};
use gpui::{
    AnyElement, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    IntoElement, ParentElement, Render, SharedString, Styled, Subscription, Task, WeakEntity,
    Window,
};
use language::{Buffer, Capability};
use picker::{Picker, PickerDelegate};
use project::{Project, git_store::Repository};
use std::sync::Arc;
use time::OffsetDateTime;
use ui::{ListItem, ListItemSpacing, prelude::*};
use util::ResultExt;
use workspace::{ItemHandle, ModalView, Workspace};

const PAGE_SIZE: usize = 50;

pub struct CommitDiffPicker {
    pub picker: Entity<Picker<CommitDiffPickerDelegate>>,
    _subscription: Subscription,
}

impl CommitDiffPicker {
    pub fn new(
        history: FileHistory,
        buffer: Entity<Buffer>,
        repo: WeakEntity<Repository>,
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate = CommitDiffPickerDelegate::new(history, buffer, repo, workspace, project, cx);
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));

        let _subscription = cx.subscribe(&picker, |_, _, _, cx| {
            cx.emit(DismissEvent);
        });

        Self {
            picker,
            _subscription,
        }
    }
}

impl ModalView for CommitDiffPicker {}
impl EventEmitter<DismissEvent> for CommitDiffPicker {}

impl Focusable for CommitDiffPicker {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for CommitDiffPicker {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("CommitDiffPicker")
            .w(rems(34.))
            .child(self.picker.clone())
    }
}

pub struct CommitDiffPickerDelegate {
    history: FileHistory,
    matches: Vec<MatchEntry>,
    buffer: Entity<Buffer>,
    repo: WeakEntity<Repository>,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    selected_index: usize,
    last_query: String,
}

struct MatchEntry {
    entry: FileHistoryEntry,
}

impl CommitDiffPickerDelegate {
    fn new(
        history: FileHistory,
        buffer: Entity<Buffer>,
        repo: WeakEntity<Repository>,
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        _cx: &mut Context<CommitDiffPicker>,
    ) -> Self {
        let matches: Vec<MatchEntry> = history
            .entries
            .iter()
            .take(PAGE_SIZE)
            .cloned()
            .map(|entry| MatchEntry { entry })
            .collect();

        Self {
            history,
            matches,
            buffer,
            repo,
            workspace,
            project,
            selected_index: 0,
            last_query: String::new(),
        }
    }

    fn open_diff_view(
        &self,
        entry: &FileHistoryEntry,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        let sha = entry.sha.clone();
        let repo = self.repo.clone();
        let buffer = self.buffer.clone();
        let workspace = self.workspace.clone();
        let project = self.project.clone();
        let path = self.history.path.clone();

        cx.spawn_in(window, async move |_, cx| {
            let old_text = repo
                .update(cx, |repo, _| repo.show_file(sha.to_string(), path.clone()))?
                .await??;

            let new_buffer_snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot())?;

            let old_buffer = cx.new(|cx| {
                let mut buffer = Buffer::local(old_text.unwrap_or_default(), cx);
                buffer.set_capability(Capability::ReadOnly, cx);
                if let Some(language) = new_buffer_snapshot.language() {
                    buffer.set_language(Some(language.clone()), cx);
                }
                buffer
            })?;

            let old_buffer_snapshot = old_buffer.read_with(cx, |buffer, _| buffer.snapshot())?;

            let diff_snapshot = cx
                .update(|_window, cx| {
                    BufferDiffSnapshot::new_with_base_buffer(
                        new_buffer_snapshot.text.clone(),
                        Some(old_buffer_snapshot.text().into()),
                        old_buffer_snapshot.clone(),
                        cx,
                    )
                })?
                .await;

            let buffer_diff = cx.new(|cx| {
                let mut diff = BufferDiff::new(&new_buffer_snapshot.text, cx);
                diff.set_snapshot(diff_snapshot, &new_buffer_snapshot.text, cx);
                diff
            })?;

            workspace.update_in(cx, |workspace, window, cx| {
                let diff_view = cx.new(|cx| {
                    CommitFileDiffView::new(
                        old_buffer,
                        buffer.clone(),
                        buffer_diff,
                        sha.clone(),
                        project.clone(),
                        window,
                        cx,
                    )
                });

                let pane = workspace.active_pane();
                pane.update(cx, |pane, cx| {
                    pane.add_item(Box::new(diff_view), true, true, None, window, cx);
                });
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        cx.emit(DismissEvent);
    }
}

impl PickerDelegate for CommitDiffPickerDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search commits…".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let all_entries = self.history.entries.clone();

        cx.spawn_in(window, async move |picker, cx| {
            let matches: Vec<MatchEntry> = if query.is_empty() {
                all_entries
                    .into_iter()
                    .take(PAGE_SIZE)
                    .map(|entry| MatchEntry { entry })
                    .collect()
            } else {
                let candidates: Vec<StringMatchCandidate> = all_entries
                    .iter()
                    .enumerate()
                    .map(|(ix, entry)| {
                        let searchable = format!(
                            "{} {} {} {}",
                            entry.sha, entry.author_name, entry.subject, entry.message
                        );
                        StringMatchCandidate::new(ix, &searchable)
                    })
                    .collect();

                fuzzy::match_strings(
                    &candidates,
                    &query,
                    true,
                    true,
                    10000,
                    &Default::default(),
                    cx.background_executor().clone(),
                )
                .await
                .into_iter()
                .map(|m| MatchEntry {
                    entry: all_entries[m.candidate_id].clone(),
                })
                .collect()
            };

            picker
                .update(cx, |picker, _cx| {
                    picker.delegate.matches = matches;
                    picker.delegate.last_query = query;
                    if picker.delegate.matches.is_empty() {
                        picker.delegate.selected_index = 0;
                    } else {
                        picker.delegate.selected_index = picker
                            .delegate
                            .selected_index
                            .min(picker.delegate.matches.len() - 1);
                    }
                })
                .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(match_entry) = self.matches.get(self.selected_index) {
            self.open_diff_view(&match_entry.entry, window, cx);
        }
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let match_entry = self.matches.get(ix)?;
        let entry = &match_entry.entry;

        let short_sha = if entry.sha.len() >= 7 {
            &entry.sha[..7]
        } else {
            &entry.sha
        };

        let commit_time = OffsetDateTime::from_unix_timestamp(entry.commit_timestamp)
            .unwrap_or_else(|_| OffsetDateTime::UNIX_EPOCH);
        let relative_timestamp = time_format::format_localized_timestamp(
            commit_time,
            OffsetDateTime::now_utc(),
            time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC),
            time_format::TimestampFormat::Relative,
        );

        Some(
            ListItem::new(SharedString::from(format!("commit-{ix}")))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    h_flex()
                        .w_full()
                        .gap_2()
                        .child(
                            div().w(rems_from_px(52.)).flex_none().child(
                                Label::new(short_sha.to_string())
                                    .size(LabelSize::Small)
                                    .color(Color::Accent),
                            ),
                        )
                        .child(
                            v_flex()
                                .min_w_0()
                                .w_full()
                                .child(
                                    h_flex()
                                        .w_full()
                                        .justify_between()
                                        .child(
                                            Label::new(entry.author_name.clone())
                                                .size(LabelSize::Small)
                                                .color(Color::Default)
                                                .truncate(),
                                        )
                                        .child(
                                            Label::new(relative_timestamp)
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        ),
                                )
                                .child(
                                    Label::new(&entry.subject)
                                        .size(LabelSize::Small)
                                        .color(Color::Muted)
                                        .truncate(),
                                ),
                        ),
                ),
        )
    }

    fn render_header(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        Some(
            h_flex()
                .px_3()
                .pt_2()
                .pb_1()
                .w_full()
                .gap_1p5()
                .child(Icon::new(IconName::GitBranch).size(IconSize::XSmall))
                .child(
                    Label::new("Select commit to diff against")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .into_any_element(),
        )
    }
}

pub struct CommitFileDiffView {
    editor: Entity<Editor>,
    _old_buffer: Entity<Buffer>,
    new_buffer: Entity<Buffer>,
    commit_sha: SharedString,
}

impl CommitFileDiffView {
    pub fn new(
        old_buffer: Entity<Buffer>,
        new_buffer: Entity<Buffer>,
        diff: Entity<BufferDiff>,
        commit_sha: SharedString,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::singleton(new_buffer.clone(), cx);
            multibuffer.add_diff(diff.clone(), cx);
            multibuffer
        });
        let editor = cx.new(|cx| {
            let mut editor =
                Editor::for_multibuffer(multibuffer.clone(), Some(project.clone()), window, cx);
            editor.start_temporary_diff_override();
            editor.disable_diagnostics(cx);
            editor.set_expand_all_diff_hunks(cx);
            editor.set_render_diff_hunk_controls(
                Arc::new(|_, _, _, _, _, _, _, _| gpui::Empty.into_any_element()),
                cx,
            );
            editor
        });

        Self {
            editor,
            _old_buffer: old_buffer,
            new_buffer,
            commit_sha,
        }
    }
}

impl EventEmitter<EditorEvent> for CommitFileDiffView {}

impl Focusable for CommitFileDiffView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl workspace::Item for CommitFileDiffView {
    type Event = EditorEvent;

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::Diff).color(Color::Muted))
    }

    fn tab_content(
        &self,
        params: workspace::item::TabContentParams,
        _window: &Window,
        cx: &App,
    ) -> AnyElement {
        Label::new(self.tab_content_text(params.detail.unwrap_or_default(), cx))
            .color(if params.selected {
                Color::Default
            } else {
                Color::Muted
            })
            .into_any_element()
    }

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        let filename = self
            .new_buffer
            .read(cx)
            .file()
            .and_then(|file| {
                Some(
                    file.full_path(cx)
                        .file_name()?
                        .to_string_lossy()
                        .to_string(),
                )
            })
            .unwrap_or_else(|| "untitled".into());

        let short_sha = if self.commit_sha.len() >= 7 {
            &self.commit_sha[..7]
        } else {
            &self.commit_sha
        };

        format!("{filename} @ {short_sha}").into()
    }

    fn tab_tooltip_text(&self, cx: &App) -> Option<SharedString> {
        let path = self
            .new_buffer
            .read(cx)
            .file()
            .map(|file| file.full_path(cx).to_string_lossy().into_owned())
            .unwrap_or_else(|| "untitled".into());

        Some(format!("Diff: {path} vs commit {}", self.commit_sha).into())
    }

    fn to_item_events(event: &EditorEvent, f: impl FnMut(workspace::item::ItemEvent)) {
        Editor::to_item_events(event, f)
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Commit Diff View")
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor
            .update(cx, |editor, cx| editor.deactivated(window, cx));
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: std::any::TypeId,
        self_handle: &'a Entity<Self>,
        _: &'a App,
    ) -> Option<gpui::AnyEntity> {
        if type_id == std::any::TypeId::of::<Self>() {
            Some(self_handle.clone().into())
        } else if type_id == std::any::TypeId::of::<Editor>() {
            Some(self.editor.clone().into())
        } else {
            None
        }
    }

    fn as_searchable(
        &self,
        _: &Entity<Self>,
        _: &App,
    ) -> Option<Box<dyn workspace::searchable::SearchableItemHandle>> {
        Some(Box::new(self.editor.clone()))
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        self.editor.for_each_project_item(cx, f)
    }

    fn set_nav_history(
        &mut self,
        nav_history: workspace::ItemNavHistory,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, _| {
            editor.set_nav_history(Some(nav_history));
        });
    }

    fn navigate(
        &mut self,
        data: Box<dyn std::any::Any>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.editor
            .update(cx, |editor, cx| editor.navigate(data, window, cx))
    }

    fn breadcrumb_location(&self, _: &App) -> workspace::ToolbarItemLocation {
        workspace::ToolbarItemLocation::PrimaryLeft
    }

    fn breadcrumbs(
        &self,
        theme: &theme::Theme,
        cx: &App,
    ) -> Option<Vec<workspace::item::BreadcrumbText>> {
        self.editor.breadcrumbs(theme, cx)
    }

    fn added_to_workspace(
        &mut self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.added_to_workspace(workspace, window, cx)
        });
    }

    fn can_save(&self, _cx: &App) -> bool {
        false
    }

    fn save(
        &mut self,
        _options: workspace::item::SaveOptions,
        _project: Entity<Project>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn save_as(
        &mut self,
        _project: Entity<Project>,
        _path: project::ProjectPath,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn reload(
        &mut self,
        _project: Entity<Project>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn is_dirty(&self, _cx: &App) -> bool {
        false
    }

    fn has_conflict(&self, _cx: &App) -> bool {
        false
    }
}

impl Render for CommitFileDiffView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.editor.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use git::repository::{FileHistory, FileHistoryEntry, RepoPath};
    use gpui::{TestAppContext, VisualTestContext};
    use picker::PickerDelegate;
    use project::FakeFs;
    use settings::SettingsStore;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme::init(theme::LoadThemes::JustBase, cx);
        });
    }

    fn create_test_history_entry(
        sha: &str,
        author: &str,
        subject: &str,
        message: &str,
        timestamp: i64,
    ) -> FileHistoryEntry {
        FileHistoryEntry {
            sha: SharedString::from(sha.to_string()),
            author_name: SharedString::from(author.to_string()),
            author_email: SharedString::from(format!("{author}@example.com")),
            subject: SharedString::from(subject.to_string()),
            message: SharedString::from(message.to_string()),
            commit_timestamp: timestamp,
        }
    }

    fn create_test_file_history() -> FileHistory {
        FileHistory {
            path: RepoPath::new("test.txt").unwrap(),
            entries: vec![
                create_test_history_entry(
                    "abc1234567890",
                    "Alice",
                    "Fix bug in parser",
                    "Fixed a critical bug",
                    1700000000,
                ),
                create_test_history_entry(
                    "def4567890abc",
                    "Zephyr",
                    "Add new feature",
                    "Implemented auth",
                    1699900000,
                ),
                create_test_history_entry(
                    "ghi7890abcdef",
                    "Charlie",
                    "Refactor database",
                    "Cleaned up queries",
                    1699800000,
                ),
                create_test_history_entry(
                    "jkl0123456789",
                    "Alice",
                    "Initial commit",
                    "First version",
                    1699700000,
                ),
            ],
        }
    }

    async fn init_picker_test(
        cx: &mut TestAppContext,
        history: FileHistory,
    ) -> (VisualTestContext, Entity<CommitDiffPicker>) {
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, None, cx).await;

        let window = cx.add_window(|window, cx| {
            let buffer = cx.new(|cx| Buffer::local("test content", cx));
            CommitDiffPicker::new(
                history,
                buffer,
                WeakEntity::new_invalid(),
                WeakEntity::new_invalid(),
                project.clone(),
                window,
                cx,
            )
        });

        let picker = window.root(cx).unwrap();
        let visual_cx = VisualTestContext::from_window(*window, cx);

        (visual_cx, picker)
    }

    #[gpui::test]
    async fn test_picker_displays_all_entries_with_empty_query(cx: &mut TestAppContext) {
        init_test(cx);

        let history = create_test_file_history();
        let (mut visual_cx, picker) = init_picker_test(cx, history).await;

        picker.update(&mut visual_cx, |picker, cx| {
            picker.picker.update(cx, |picker, _cx| {
                assert_eq!(picker.delegate.match_count(), 4);
            });
        });
    }

    #[gpui::test]
    async fn test_picker_filters_by_author(cx: &mut TestAppContext) {
        init_test(cx);

        let history = create_test_file_history();
        let (mut visual_cx, picker) = init_picker_test(cx, history).await;

        picker
            .update_in(&mut visual_cx, |picker, window, cx| {
                picker.picker.update(cx, |picker, cx| {
                    picker
                        .delegate
                        .update_matches("Zephyr".to_string(), window, cx)
                })
            })
            .await;
        visual_cx.run_until_parked();

        picker.update(&mut visual_cx, |picker, cx| {
            picker.picker.update(cx, |picker, _cx| {
                assert_eq!(picker.delegate.match_count(), 1);
            });
        });
    }

    #[gpui::test]
    async fn test_picker_filters_by_sha(cx: &mut TestAppContext) {
        init_test(cx);

        let history = create_test_file_history();
        let (mut visual_cx, picker) = init_picker_test(cx, history).await;

        picker
            .update_in(&mut visual_cx, |picker, window, cx| {
                picker.picker.update(cx, |picker, cx| {
                    picker
                        .delegate
                        .update_matches("abc123".to_string(), window, cx)
                })
            })
            .await;
        visual_cx.run_until_parked();

        picker.update(&mut visual_cx, |picker, cx| {
            picker.picker.update(cx, |picker, _cx| {
                assert_eq!(picker.delegate.match_count(), 1);
            });
        });
    }

    #[gpui::test]
    async fn test_picker_filters_by_subject(cx: &mut TestAppContext) {
        init_test(cx);

        let history = create_test_file_history();
        let (mut visual_cx, picker) = init_picker_test(cx, history).await;

        picker
            .update_in(&mut visual_cx, |picker, window, cx| {
                picker.picker.update(cx, |picker, cx| {
                    picker
                        .delegate
                        .update_matches("bug".to_string(), window, cx)
                })
            })
            .await;
        visual_cx.run_until_parked();

        picker.update(&mut visual_cx, |picker, cx| {
            picker.picker.update(cx, |picker, _cx| {
                assert_eq!(picker.delegate.match_count(), 1);
            });
        });
    }

    #[gpui::test]
    async fn test_picker_returns_empty_for_no_match(cx: &mut TestAppContext) {
        init_test(cx);

        let history = create_test_file_history();
        let (mut visual_cx, picker) = init_picker_test(cx, history).await;

        picker
            .update_in(&mut visual_cx, |picker, window, cx| {
                picker.picker.update(cx, |picker, cx| {
                    picker
                        .delegate
                        .update_matches("nonexistent_query_xyz".to_string(), window, cx)
                })
            })
            .await;
        visual_cx.run_until_parked();

        picker.update(&mut visual_cx, |picker, cx| {
            picker.picker.update(cx, |picker, _cx| {
                assert_eq!(picker.delegate.match_count(), 0);
            });
        });
    }

    #[gpui::test]
    async fn test_picker_selected_index_updates_on_filter(cx: &mut TestAppContext) {
        init_test(cx);

        let history = create_test_file_history();
        let (mut visual_cx, picker) = init_picker_test(cx, history).await;

        picker.update(&mut visual_cx, |picker, cx| {
            picker.picker.update(cx, |picker, _cx| {
                picker.delegate.selected_index = 3;
            });
        });

        picker
            .update_in(&mut visual_cx, |picker, window, cx| {
                picker.picker.update(cx, |picker, cx| {
                    picker
                        .delegate
                        .update_matches("Zephyr".to_string(), window, cx)
                })
            })
            .await;
        visual_cx.run_until_parked();

        picker.update(&mut visual_cx, |picker, cx| {
            picker.picker.update(cx, |picker, _cx| {
                assert!(picker.delegate.selected_index() < picker.delegate.match_count());
            });
        });
    }

    #[gpui::test]
    async fn test_picker_placeholder_text(cx: &mut TestAppContext) {
        init_test(cx);

        let history = create_test_file_history();
        let (mut visual_cx, picker) = init_picker_test(cx, history).await;

        picker.update_in(&mut visual_cx, |picker, window, cx| {
            picker.picker.update(cx, |picker, cx| {
                let placeholder = picker.delegate.placeholder_text(window, cx);
                assert_eq!(placeholder.as_ref(), "Search commits…");
            });
        });
    }
}
