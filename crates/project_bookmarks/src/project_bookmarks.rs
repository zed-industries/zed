use fuzzy_nucleo::{Case, LengthPenalty, StringMatchCandidate, match_strings};
use gpui::{
    App, Context, DismissEvent, Entity, ParentElement, SharedString, Task, WeakEntity, Window,
    actions,
};
use language::Buffer;
use picker::{MatchLocation, Picker, PickerDelegate, PreviewUpdate};
use project::{Project, ProjectPath, bookmark_store::BookmarkStore};
use ui::{HighlightedLabel, ListItem, ListItemSpacing, prelude::*};
use util::ResultExt;
use workspace::Workspace;

actions!(
    project_bookmarks,
    [
        /// Toggles the project bookmarks search.
        #[action(name = "Toggle")]
        ToggleProjectBookmarks,
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _: &mut Context<Workspace>| {
            workspace.register_action(|workspace, _: &ToggleProjectBookmarks, window, cx| {
                let project = workspace.project().clone();
                let handle = cx.entity().downgrade();
                let bookmark_store = project.read(cx).bookmark_store().clone();
                workspace.toggle_modal(window, cx, move |window, cx| {
                    let delegate =
                        ProjectBookmarksDelegate::new(handle, project.clone(), bookmark_store);
                    Picker::uniform_list_with_preview(delegate, project, window, cx)
                })
            });
        },
    )
    .detach();
}

struct Match {
    pub path: ProjectPath,
    pub label: SharedString,
    pub buffer: Entity<Buffer>,
    pub positions: Vec<usize>,
    pub anchor: text::Anchor,
    pub offset: usize,
}

struct ProjectBookmarksDelegate {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    bookmark_store: Entity<BookmarkStore>,
    group_result_by_path: bool,
    selected_match_index: usize,
    matches: Vec<Match>,
}

impl ProjectBookmarksDelegate {
    fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        bookmark_store: Entity<BookmarkStore>,
    ) -> Self {
        Self {
            workspace,
            project,
            bookmark_store,
            group_result_by_path: false,
            selected_match_index: 0,
            matches: Vec::new(),
        }
    }

    fn labels_for_match(
        &self,
        bookmark_match: &Match,
        _window: &mut Window,
        cx: &App,
    ) -> (HighlightedLabel, Label) {
        let path_style = self.project.read(cx).path_style(cx);
        let Match {
            label,
            positions,
            path,
            ..
        } = bookmark_match;

        (
            HighlightedLabel::new(label.clone(), positions.clone()),
            Label::new(path.path.display(path_style))
                .size(LabelSize::Small)
                .color(Color::Muted),
        )
    }
}

impl PickerDelegate for ProjectBookmarksDelegate {
    type ListItem = ListItem;

    fn name() -> &'static str {
        "bookmarks"
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_match_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<picker::Picker<Self>>,
    ) {
        self.selected_match_index = ix;
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> std::sync::Arc<str> {
        "Search for a bookmark...".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) -> Task<()> {
        let bookmark_store = self.bookmark_store.clone();
        let project = self.project.clone();
        cx.spawn_in(window, async move |picker, cx| {
            let bookmarks = BookmarkStore::all_bookmarks(&bookmark_store, cx).await;
            let Ok(bookmarks) = bookmarks else {
                return;
            };

            let candidates: Vec<StringMatchCandidate> = bookmarks
                .iter()
                .enumerate()
                .map(|(i, bookmark)| StringMatchCandidate::new(i, &bookmark.label))
                .collect();

            let matches = match_strings(
                candidates.as_slice(),
                &query,
                Case::Smart,
                LengthPenalty::On,
                100,
            );

            picker
                .update(cx, |this, cx| {
                    let project = project.read(cx);
                    this.delegate.matches = matches
                        .into_iter()
                        .filter_map(|mat| {
                            let Some(project_path) = project.project_path_for_absolute_path(
                                &bookmarks[mat.candidate_id].path,
                                cx,
                            ) else {
                                return None;
                            };

                            Some(Match {
                                path: project_path,
                                label: mat.string,
                                positions: mat.positions,
                                buffer: bookmarks[mat.candidate_id].buffer.clone(),
                                anchor: bookmarks[mat.candidate_id].anchor,
                                offset: bookmarks[mat.candidate_id].offset,
                            })
                        })
                        .collect();
                })
                .ok();
        })
    }

    fn confirm(
        &mut self,
        _secondary: bool,
        window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let Some(selected_bookmark) = self.matches.get(self.selected_match_index) else {
            return;
        };

        let open_task = workspace.update(cx, |workspace, cx| {
            workspace.open_path_preview(
                selected_bookmark.path.clone(),
                None,
                true,
                false,
                true,
                window,
                cx,
            )
        });

        let point = selected_bookmark
            .buffer
            .read(cx)
            .snapshot()
            .summary_for_anchor::<text::Point>(&selected_bookmark.anchor);

        cx.spawn_in(window, async move |_, cx| {
            let item = open_task.await.log_err()?;
            if let Some(active_editor) = item.downcast::<editor::Editor>() {
                active_editor
                    .downgrade()
                    .update_in(cx, |editor, window, cx| {
                        editor.go_to_singleton_buffer_point(point, window, cx);
                    })
                    .log_err();
            }
            Some(())
        })
        .detach();

        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<picker::Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let selected_bookmark = self.matches.get(ix)?;
        let icon = Icon::new(IconName::Bookmark).color(Color::Info);
        let (bookmark_label, full_path_label) =
            self.labels_for_match(selected_bookmark, window, cx);

        Some(
            ListItem::new(ix)
                .spacing(ListItemSpacing::Sparse)
                .inset(true)
                .toggle_state(selected)
                .start_slot(icon)
                .child(
                    if self.group_result_by_path {
                        v_flex()
                    } else {
                        h_flex()
                    }
                    .w_full()
                    .min_w_0()
                    .gap_1p5()
                    .child(bookmark_label.truncate_middle())
                    .child(full_path_label.truncate_start()),
                ),
        )
    }

    fn preview_layout_changed(&mut self, layout_is_horizontal: bool) {
        self.group_result_by_path = layout_is_horizontal;
    }

    fn try_get_preview_data_for_match(&self, _cx: &App) -> Option<PreviewUpdate> {
        let selected_bookmark = self.matches.get(self.selected_match_index)?;
        let anchor = selected_bookmark.anchor;
        let offset = selected_bookmark.offset;
        Some(PreviewUpdate::from_buffer(
            selected_bookmark.buffer.clone(),
            MatchLocation {
                anchor_range: anchor..anchor,
                range: offset..offset,
            },
        ))
    }
}
