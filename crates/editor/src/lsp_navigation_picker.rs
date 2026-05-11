use crate::{Editor, PreviewTabsSettings};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    AnyElement, App, Context, DismissEvent, Entity, IntoElement, ParentElement, SharedString,
    Styled, Task, WeakEntity, Window, div, rems,
};
use language::{Buffer, Point};
use ordered_float::OrderedFloat;
use picker::{Picker, PickerDelegate};
use settings::Settings as _;
use std::{cmp::Reverse, collections::HashMap, ops::Range, sync::Arc};
use ui::{
    Color, FluentBuilder as _, HighlightedLabel, Label, LabelCommon, LabelSize, ListItem,
    ListItemSpacing, Toggleable, h_flex, vh,
};
use util::{ResultExt as _, paths::PathStyle};
use workspace::Workspace;

/// Opens a modal picker showing the given LSP locations as a compact, type-to-filter list.
///
/// Used as an alternative to `Editor::open_locations_in_multibuffer` when the
/// `lsp_navigation_view` setting is `picker` and the action's
/// `always_open_multibuffer` payload is false.
pub(crate) fn open(
    workspace: &mut Workspace,
    locations: HashMap<Entity<Buffer>, Vec<Range<Point>>>,
    header_title: SharedString,
    split: bool,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let project = workspace.project().clone();
    let path_style = project.read(cx).path_style(cx);

    let mut items = Vec::new();
    for (buffer, ranges) in locations {
        let path_label = path_label_for_buffer(&buffer, path_style, cx);
        let (filename, dir) = split_filename_and_dir(&path_label, path_style);
        for range in ranges {
            let line_number = range.start.row + 1;
            // Filename leads the haystack so fuzzy matches that hit the
            // filename score higher than dir-only matches.
            let haystack = if dir.is_empty() {
                filename.to_lowercase()
            } else {
                format!("{} {}", filename, dir).to_lowercase()
            };
            items.push(LocationItem {
                buffer: buffer.clone(),
                range,
                filename: filename.clone(),
                dir: dir.clone(),
                line: line_number,
                haystack,
            });
        }
    }

    items.sort_by(|a, b| {
        a.dir
            .cmp(&b.dir)
            .then_with(|| a.filename.cmp(&b.filename))
            .then_with(|| a.line.cmp(&b.line))
    });

    let candidates: Vec<StringMatchCandidate> = items
        .iter()
        .enumerate()
        .map(|(id, item)| StringMatchCandidate::new(id, &item.haystack))
        .collect();
    let initial_matches: Vec<StringMatch> = items
        .iter()
        .enumerate()
        .map(|(id, item)| StringMatch {
            candidate_id: id,
            score: 0.0,
            positions: Vec::new(),
            string: item.haystack.clone(),
        })
        .collect();

    let delegate = LspNavigationPickerDelegate {
        workspace: cx.entity().downgrade(),
        items,
        candidates,
        matches: initial_matches,
        selected_index: 0,
        header_title,
        split,
    };

    workspace.toggle_modal(window, cx, |window, cx| {
        let max_height = vh(0.7, window);
        Picker::uniform_list(delegate, window, cx)
            .width(rems(56.))
            .max_height(Some(max_height))
            .show_scrollbar(true)
    });
}

fn path_label_for_buffer(
    buffer: &Entity<Buffer>,
    path_style: PathStyle,
    cx: &App,
) -> SharedString {
    let buffer_ref = buffer.read(cx);
    if let Some(file) = buffer_ref.file() {
        return file.path().display(path_style).into_owned().into();
    }
    SharedString::from("<untitled>")
}

/// Splits a relative path into (filename, directory). For root-level files
/// the directory is empty.
fn split_filename_and_dir(path: &str, path_style: PathStyle) -> (SharedString, SharedString) {
    match path.rfind(path_style.separators_ch()) {
        Some(sep) => (
            SharedString::from(path[sep + 1..].to_owned()),
            SharedString::from(path[..sep].to_owned()),
        ),
        None => (SharedString::from(path.to_owned()), SharedString::default()),
    }
}

#[derive(Clone)]
struct LocationItem {
    buffer: Entity<Buffer>,
    range: Range<Point>,
    filename: SharedString,
    dir: SharedString,
    line: u32,
    haystack: String,
}

pub(crate) struct LspNavigationPickerDelegate {
    workspace: WeakEntity<Workspace>,
    items: Vec<LocationItem>,
    candidates: Vec<StringMatchCandidate>,
    matches: Vec<StringMatch>,
    selected_index: usize,
    header_title: SharedString,
    split: bool,
}

impl PickerDelegate for LspNavigationPickerDelegate {
    type ListItem = ListItem;

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
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        Arc::from("Filter by path or code…")
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some("No matching locations".into())
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        if query.is_empty() {
            self.matches = (0..self.items.len())
                .map(|i| StringMatch {
                    candidate_id: i,
                    score: 0.0,
                    positions: Vec::new(),
                    string: self.items[i].haystack.clone(),
                })
                .collect();
            self.set_selected_index(0, window, cx);
            return Task::ready(());
        }
        let candidates = self.candidates.clone();
        let executor = cx.background_executor().clone();
        cx.spawn_in(window, async move |this, cx| {
            let mut matches = fuzzy::match_strings(
                &candidates,
                &query,
                false,
                true,
                100,
                &Default::default(),
                executor,
            )
            .await;
            matches.sort_unstable_by_key(|m| Reverse(OrderedFloat(m.score)));
            this.update_in(cx, |this, window, cx| {
                this.delegate.matches = matches;
                this.delegate.set_selected_index(0, window, cx);
            })
            .log_err();
        })
    }

    fn render_header(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        let total = self.items.len();
        let visible = self.matches.len();
        let count_label = if visible == total {
            format!("{} results", total)
        } else {
            format!("{}/{} results", visible, total)
        };
        Some(
            h_flex()
                .px_2()
                .py_1()
                .gap_2()
                .child(
                    Label::new(self.header_title.clone())
                        .size(LabelSize::Small)
                        .color(Color::Default),
                )
                .child(
                    Label::new(count_label)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .into_any_element(),
        )
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = self.matches.get(ix)?;
        let item = self.items.get(mat.candidate_id)?;

        // Haystack layout (lowercased): "{filename}" or "{filename} {dir}".
        // Map fuzzy match positions back to byte offsets in the rendered
        // `filename:line` and `dir` strings. The line-number suffix is purely
        // visual and never participates in the haystack, so digit positions
        // produced by user queries hit code, not row numbers.
        let filename_haystack_len = item.filename.len();
        let mut filename_highlights: Vec<Range<usize>> = Vec::new();
        let mut dir_highlights: Vec<Range<usize>> = Vec::new();
        for &pos in &mat.positions {
            if pos < filename_haystack_len {
                filename_highlights.push(pos..pos + 1);
            } else if !item.dir.is_empty() && pos > filename_haystack_len {
                let p = pos - (filename_haystack_len + 1);
                if p < item.dir.len() {
                    dir_highlights.push(p..p + 1);
                }
            }
        }

        let filename_with_line: SharedString = format!("{}:{}", item.filename, item.line).into();

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    h_flex()
                        .gap_2()
                        .w_full()
                        .child(
                            HighlightedLabel::from_ranges(filename_with_line, filename_highlights)
                                .single_line(),
                        )
                        .when(!item.dir.is_empty(), |row| {
                            row.child(
                                // `min_w_0` lets the flex item shrink below intrinsic
                                // width; `truncate` adds overflow-hidden + nowrap +
                                // end-ellipsis so a long directory stays single-line.
                                div().flex_1().min_w_0().truncate().child(
                                    HighlightedLabel::from_ranges(
                                        item.dir.clone(),
                                        dir_highlights,
                                    )
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                                    .truncate(),
                                ),
                            )
                        }),
                ),
        )
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(mat) = self.matches.get(self.selected_index) else {
            return;
        };
        let Some(item) = self.items.get(mat.candidate_id).cloned() else {
            return;
        };
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let split = self.split;

        cx.emit(DismissEvent);

        workspace.update(cx, |workspace, cx| {
            let pane = if secondary || split {
                workspace.adjacent_pane(window, cx)
            } else {
                workspace.active_pane().clone()
            };
            let preview_tabs = PreviewTabsSettings::get_global(cx);
            let target_editor: Entity<Editor> = workspace.open_project_item(
                pane.clone(),
                item.buffer.clone(),
                true,
                true,
                preview_tabs.enable_keep_preview_on_code_navigation,
                preview_tabs.enable_preview_file_from_code_navigation,
                window,
                cx,
            );

            let same_buffer_singleton = target_editor
                .read(cx)
                .buffer()
                .read(cx)
                .as_singleton()
                .as_ref()
                == Some(&item.buffer);

            target_editor.update(cx, |editor, cx| {
                let range = editor.range_for_match(&item.range);
                let range = range.start..range.start;
                if same_buffer_singleton {
                    // change_selections(...nav_history(true)) handles the source-cursor
                    // push (always=true bypasses the row-delta gate; row delta of 0 is
                    // still skipped, which is correct).
                    editor.go_to_singleton_buffer_range(range, window, cx);
                } else {
                    // Suppress the freshly-opened target's pre-jump (line 0) entry.
                    // The source's cursor was already recorded by Editor::deactivated()
                    // when the target was activated above.
                    pane.update(cx, |pane, _| pane.disable_history());
                    editor.go_to_singleton_buffer_range(range, window, cx);
                    pane.update(cx, |pane, _| pane.enable_history());
                }
            });
        });
    }
}

