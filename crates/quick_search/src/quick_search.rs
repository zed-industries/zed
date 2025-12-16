mod grouped_list;
pub(crate) mod history;
pub(crate) mod match_list;
pub(crate) mod types;

#[path = "quick_search_preview.rs"]
mod preview;
#[path = "quick_search_source.rs"]
mod source;

use crate::{
    grouped_list::{GroupedFileHeader, GroupedListState, GroupedRow},
    match_list::MatchList,
    preview::PreviewState,
    preview::PreviewRequest,
    types::{MatchId, MatchKey, PatchValue, QuickMatch},
};
use async_channel::Receiver;
use file_icons::FileIcons;
use gpui::AsyncApp;
use project::search::SearchResult;
use std::path;
use std::path::Path;
use std::{sync::Arc, time::Duration};
use std::sync::atomic::{AtomicBool, Ordering};

use search::{
    SearchOptions, ToggleCaseSensitive, ToggleIncludeIgnored, ToggleRegex, ToggleWholeWord,
    search_bar::{input_base_styles, render_text_input},
};
use editor::{Editor, EditorSettings, SelectionEffects, scroll::Autoscroll};
use gpui::{
    Action, App, Context, DismissEvent, Entity, EntityInputHandler, FocusHandle,
    Focusable, Render, Styled, Task, WeakEntity, Window,
};
use gpui::SharedString;
use language::Buffer;
use log::debug;
use picker::{Picker, PickerDelegate};
use project::{Project, debounced_delay::DebouncedDelay};
use settings::{Settings, SettingsStore};
use ::git::GitRemote;
use git_ui::commit_tooltip::CommitAvatar;
use ui::{
    Button, ButtonStyle, Chip, Color, DiffStat, HighlightedLabel, Icon, IconButton, IconButtonShape,
    IconName, IconPosition, IconSize, KeyBinding, Label, LabelCommon as _, LabelSize, ListItem,
    ListItemSpacing, Modal, ModalHeader, Section, SpinnerLabel, Tooltip, prelude::*, rems_from_px,
};
use workspace::{ModalView, Workspace, item::PreviewTabsSettings};

pub(crate) const MIN_QUERY_LEN: usize = 2;
const RESULTS_BATCH_SIZE: usize = 1024;

const MODAL_SIZE_FRAC: f32 = 0.75;

const STACK_BREAKPOINT_PX: f32 = 800.;

const H_LIST_FRAC: f32 = 0.35;

const PREVIEW_MIN_WIDTH_REM: f32 = 30.;
const PREVIEW_MIN_HEIGHT_REM: f32 = 12.;
const MAX_SNIPPET_CHARS: usize = 240;
const MAX_RESULTS: usize = 20_000;
const HIGHLIGHT_WINDOW: usize = 40;
const QUERY_DEBOUNCE_MS: u64 = 80;

pub fn init(cx: &mut App) {
    cx.observe_new(QuickSearch::register).detach();
}

pub struct QuickSearch {
    picker: Entity<Picker<QuickSearchDelegate>>,
    preview: PreviewState,
    source_registry: source::SourceRegistry,
}

impl QuickSearch {
    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        cx: &mut Context<Workspace>,
    ) {
        let workspace_handle = cx.entity().downgrade();
        workspace.register_action(
            move |workspace, _: &workspace::ToggleQuickSearch, window, cx| {
                let selected_text = workspace
                    .active_item(cx)
                    .and_then(|item| item.act_as::<Editor>(cx))
                    .and_then(|editor| {
                        editor.update(cx, |editor, cx| {
                            let selection = editor.selected_text_range(true, window, cx)?;
                            if selection.range.is_empty() {
                                return None;
                            }
                            let mut adjusted = None;
                            let text = editor.text_for_range(
                                selection.range,
                                &mut adjusted,
                                window,
                                cx,
                            )?;
                            if text.contains('\n') {
                                return None;
                            }
                            let trimmed = text.trim().to_string();
                            (!trimmed.is_empty()).then_some(trimmed)
                        })
                    });
                let project = workspace.project().clone();
                let workspace_handle = workspace_handle.clone();
                workspace.toggle_modal(window, cx, move |window, cx| {
                    QuickSearch::new(workspace_handle.clone(), project, selected_text, window, cx)
                })
            },
        );

        workspace.register_action(move |workspace, _: &ToggleRegex, window, cx| {
            if let Some(active) = workspace.active_modal::<QuickSearch>(cx) {
                active.update(cx, |qs, cx| {
                    qs.toggle_search_option(SearchOptions::REGEX, window, cx)
                });
            } else {
                cx.propagate();
            }
        });
        workspace.register_action(move |workspace, _: &ToggleCaseSensitive, window, cx| {
            if let Some(active) = workspace.active_modal::<QuickSearch>(cx) {
                active.update(cx, |qs, cx| {
                    qs.toggle_search_option(SearchOptions::CASE_SENSITIVE, window, cx)
                });
            } else {
                cx.propagate();
            }
        });
        workspace.register_action(move |workspace, _: &ToggleWholeWord, window, cx| {
            if let Some(active) = workspace.active_modal::<QuickSearch>(cx) {
                active.update(cx, |qs, cx| {
                    qs.toggle_search_option(SearchOptions::WHOLE_WORD, window, cx)
                });
            } else {
                cx.propagate();
            }
        });
        workspace.register_action(move |workspace, _: &ToggleIncludeIgnored, window, cx| {
            if let Some(active) = workspace.active_modal::<QuickSearch>(cx) {
                active.update(cx, |qs, cx| {
                    qs.toggle_search_option(SearchOptions::INCLUDE_IGNORED, window, cx)
                });
            } else {
                cx.propagate();
            }
        });
    }

    fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        initial_query: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let preview_project = project.clone();
        let initial_buffer = cx.new(|cx| Buffer::local("", cx));
        let editor_settings = EditorSettings::get_global(cx);
        let search_options = SearchOptions::from_settings(&editor_settings.search);
        let preview = PreviewState::new(preview_project, initial_buffer, window, cx);
        let source_registry = source::SourceRegistry::default();
        let initial_source_id = history::last_source_id();
        let initial_source = source_registry
            .available_sources()
            .iter()
            .map(|source| source.spec())
            .find(|spec| spec.id.0 == initial_source_id)
            .map(|spec| spec.id.clone())
            .unwrap_or_else(source::default_source_id);
        let mut delegate = QuickSearchDelegate::new(
            cx.entity().downgrade(),
            workspace,
            window.window_handle(),
            project,
            search_options,
            source_registry.clone(),
        );
        delegate.search_engine.set_active_source(initial_source);
        let picker = cx.new(|cx| {
            let picker = Picker::uniform_list(delegate, window, cx)
                .modal(false)
                .show_scrollbar(true)
                .max_height(None);
            if let Some(query) = initial_query {
                picker.set_query(query, window, cx);
            }
            picker
        });

        Self {
            picker,
            preview,
            source_registry,
        }
    }

    fn toggle_search_option(
        &mut self,
        option: SearchOptions,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            picker.delegate.search_engine.search_options.toggle(option);
            let options = picker.delegate.search_engine.search_options;
            let mut settings = EditorSettings::get_global(cx).clone();
            settings.search.case_sensitive = options.contains(SearchOptions::CASE_SENSITIVE);
            settings.search.whole_word = options.contains(SearchOptions::WHOLE_WORD);
            settings.search.include_ignored = options.contains(SearchOptions::INCLUDE_IGNORED);
            settings.search.regex = options.contains(SearchOptions::REGEX);
            SettingsStore::update(cx, |store, _| {
                store.override_global(settings);
            });
            picker.refresh(window, cx);
        });
    }

    fn set_search_source(
        &mut self,
        source_id: source::SourceId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        history::set_last_source_id(source_id.0.clone());
        let owner = cx.entity().downgrade();
        self.preview
            .request_preview(PreviewRequest::Empty, &owner, window, cx);
        self.picker.update(cx, |picker, cx| {
            picker
                .delegate
                .search_engine
                .set_active_source(source_id.clone());
            picker.delegate.match_list.clear();
            picker.delegate.selection = None;
            picker.delegate.reset_scroll = true;
            picker.delegate.total_results = 0;
            picker.delegate.is_streaming = false;
            picker.delegate.stream_finished = true;
            picker.delegate.clear_grouped_list_state();
            picker.delegate.rebuild_rows_after_match_list_changed(cx);
            picker.refresh_placeholder(window, cx);
            let query = picker.query(cx);
            picker
                .delegate
                .history_nav
                .on_query_changed(&picker.delegate.active_source_id());
            let _scheduled = picker.delegate.search_engine.schedule_search_with_delay(
                query,
                Duration::from_millis(0),
                cx,
            );
            picker.refresh(window, cx);
        });
    }
}

impl ModalView for QuickSearch {}

impl gpui::EventEmitter<DismissEvent> for QuickSearch {}

impl Focusable for QuickSearch {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

struct Layout {
    modal_width: Pixels,
    modal_height: Pixels,
    content_height: Pixels,
    is_horizontal: bool,
    show_preview: bool,
}

impl Layout {
    fn compute(window: &Window) -> Self {
        let viewport = window.viewport_size();
        let rem_size = window.rem_size();

        let modal_width = viewport.width * MODAL_SIZE_FRAC;
        let modal_height = viewport.height * MODAL_SIZE_FRAC;

        let content_height = modal_height - px(60.);

        let preview_min_width_px = rems(PREVIEW_MIN_WIDTH_REM).to_pixels(rem_size);
        let preview_min_height_px = rems(PREVIEW_MIN_HEIGHT_REM).to_pixels(rem_size);

        let preview_width_in_horiz = modal_width * (1.0 - H_LIST_FRAC);
        let is_horizontal = viewport.width > px(STACK_BREAKPOINT_PX)
            && preview_width_in_horiz >= preview_min_width_px;

        let show_preview = if is_horizontal {
            modal_height >= preview_min_height_px
        } else {
            modal_height * 0.5 >= preview_min_height_px
        };

        Self {
            modal_width,
            modal_height,
            content_height,
            is_horizontal,
            show_preview,
        }
    }
}

impl Render for QuickSearch {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let layout = Layout::compute(window);

        div()
            .w(layout.modal_width)
            .h(layout.modal_height)
            .overflow_hidden()
            .elevation_3(cx)
            .key_context("QuickSearch")
            .child(
                Modal::new("quick-search-modal", None)
                    .header(ModalHeader::new().headline("Quick Search"))
                    .section(
                        Section::new()
                            .padded(true)
                            .child(self.render_content(&layout, window, cx)),
                    ),
            )
    }
}

impl QuickSearch {
    fn render_content(
        &self,
        layout: &Layout,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let list = self.render_list_panel(layout, cx);
        let preview = self.render_preview_panel(layout, window, cx);

        if layout.is_horizontal {
            h_flex()
                .w_full()
                .h(layout.content_height)
                .overflow_hidden()
                .gap_3()
                .child(list)
                .child(preview)
        } else {
            v_flex()
                .w_full()
                .h(layout.content_height)
                .overflow_hidden()
                .gap_3()
                .child(list)
                .child(preview)
        }
    }

    fn render_list_panel(&self, layout: &Layout, _cx: &mut Context<Self>) -> Div {
        let list_content = div()
            .flex_1()
            .h_full()
            .overflow_hidden()
            .child(self.picker.clone());
        if layout.is_horizontal {
            let list_width = layout.modal_width * H_LIST_FRAC;
            v_flex()
                .w(list_width)
                .h(layout.content_height)
                .flex_shrink_0()
                .overflow_hidden()
                .child(list_content)
        } else {
            let list_height = layout.content_height * 0.4;
            v_flex()
                .w_full()
                .h(list_height)
                .overflow_hidden()
                .child(list_content)
        }
    }

    fn render_preview_panel(
        &self,
        layout: &Layout,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Div {
        let selected = self
            .picker
            .read(cx)
            .delegate
            .selected_match()
            .cloned();
        let content = if layout.show_preview {
            self.render_preview_content(selected, window, cx)
        } else {
            Self::render_placeholder("Preview hidden (window too small)")
        };

        let base = v_flex()
            .bg(cx.theme().colors().panel_background)
            .border_1()
            .border_color(cx.theme().colors().border_variant)
            .rounded_lg()
            .p_2()
            .overflow_hidden()
            .child(content);

        if layout.is_horizontal {
            base.flex_1().h(layout.content_height)
        } else {
            let preview_height = layout.content_height * 0.55;
            base.w_full().h(preview_height)
        }
    }

    fn render_git_commit_avatar(
        sha: &SharedString,
        remote: Option<&GitRemote>,
        size: impl Into<gpui::AbsoluteLength>,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let size = size.into();
        let avatar = CommitAvatar::new(sha, remote);

        v_flex()
            .w(size)
            .h(size)
            .border_1()
            .border_color(cx.theme().colors().border)
            .rounded_full()
            .justify_center()
            .items_center()
            .child(
                avatar
                    .avatar(window, cx)
                    .map(|a| a.size(size).into_any_element())
                    .unwrap_or_else(|| {
                        Icon::new(IconName::Person)
                            .color(Color::Muted)
                            .size(IconSize::Medium)
                            .into_any_element()
                    }),
            )
            .into_any()
    }

    fn render_preview_content(
        &self,
        selected: Option<QuickMatch>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        if let Some(error) = &self.preview.error_message {
            return Self::render_error(error);
        }

        let Some(selected) = selected else {
            return Self::render_placeholder("Select a match to preview");
        };

        if selected.is_likely_binary() {
            return Self::render_non_text_placeholder(&selected.file_name);
        }

        let project = self.preview.project();
        match self
            .source_registry
            .preview_panel_ui_for_match(&selected, &project, cx)
        {
            source::PreviewPanelUi::GitCommit { meta } => {
                self.render_git_commit_preview(meta, &selected, window, cx)
            }
            source::PreviewPanelUi::Standard { path_text, highlights } => {
                self.render_standard_preview(path_text, highlights, &selected, window, cx)
            }
        }
    }

    fn render_standard_preview(
        &self,
        path_text: Arc<str>,
        highlights: Vec<usize>,
        selected: &QuickMatch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        v_flex()
            .size_full()
            .overflow_hidden()
            .gap_1()
            .child(Self::render_preview_header(path_text, selected, highlights))
            .child(self.render_preview_editor(
                selected,
                selected.blame.clone(),
                true,
                window,
                cx,
            ))
            .into_any_element()
    }

    fn render_git_commit_preview(
        &self,
        meta: source::GitCommitPreviewMeta,
        selected: &QuickMatch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let subject = meta
            .subject
            .as_ref()
            .lines()
            .next()
            .unwrap_or("")
            .trim()
            .to_string();
        let author = meta.author.as_ref().trim().to_string();
        let full_sha = meta.sha.as_ref().to_string();

        let commit_date = time::OffsetDateTime::from_unix_timestamp(meta.commit_timestamp)
            .unwrap_or_else(|_| time::OffsetDateTime::now_utc());
        let local_offset = time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC);
        let date_string = time_format::format_localized_timestamp(
            commit_date,
            time::OffsetDateTime::now_utc(),
            local_offset,
            time_format::TimestampFormat::MediumAbsolute,
        );

        let remote = meta.remote.clone();
        let github_url = meta.github_url.clone();

        let sha_shared = SharedString::from(full_sha.clone());
        let avatar = Self::render_git_commit_avatar(
            &sha_shared,
            remote.as_ref(),
            rems_from_px(48.),
            window,
            cx,
        );

        let commit_diff_stat = self.commit_diff_stat_for_preview(cx);

        let header = v_flex()
            .gap_1p5()
            .flex_shrink_0()
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        div()
                            .w(rems_from_px(48.))
                            .h(rems_from_px(48.))
                            .rounded_full()
                            .border_1()
                            .border_color(cx.theme().colors().border)
                            .items_center()
                            .justify_center()
                            .child(avatar),
                    )
                    .child(
                        v_flex()
                            .overflow_hidden()
                            .child(
                                Label::new(subject)
                                    .size(LabelSize::Small)
                                    .single_line()
                                    .truncate(),
                            )
                            .child(
                                h_flex()
                                    .gap_1()
                                    .child(Label::new(author).color(Color::Default))
                                    .child(
                                        Label::new(format!("Commit:{}", full_sha))
                                            .color(Color::Muted)
                                            .size(LabelSize::Small)
                                            .truncate()
                                            .buffer_font(cx),
                                    ),
                            ),
                    ),
            )
            .child(
                h_flex()
                    .gap_1p5()
                    .child(Label::new(date_string).color(Color::Muted).size(LabelSize::Small))
                    .child(Label::new("•").color(Color::Ignored).size(LabelSize::Small))
                    .children(commit_diff_stat)
                    .when(!meta.repo_label.trim().is_empty(), |this| {
                        this.child(
                            Label::new(meta.repo_label.as_ref().to_string())
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                                .single_line()
                                .truncate(),
                        )
                    })
                    .when_some(github_url.as_ref(), |this, url| {
                        this.flex_1().justify_end().child(
                            Button::new("quick-search-view-on-github", "View on GitHub")
                                .icon(IconName::Github)
                                .icon_color(Color::Muted)
                                .icon_size(IconSize::Small)
                                .icon_position(IconPosition::Start)
                                .on_click({
                                    let url = url.to_string();
                                    move |_, _, cx| cx.open_url(&url)
                                }),
                        )
                    }),
            );

        v_flex()
            .size_full()
            .overflow_hidden()
            .gap_1()
            .child(header)
            .child(self.render_preview_editor(selected, None, false, window, cx))
            .into_any_element()
    }

    fn commit_diff_stat_for_preview(&self, cx: &App) -> Option<DiffStat> {
        let editor = self.preview.preview_editor();
        let snapshot = editor.read(cx).buffer().read(cx).snapshot(cx);
        let mut total_additions = 0u32;
        let mut total_deletions = 0u32;

        let mut seen_buffers = std::collections::HashSet::new();
        for (_, buffer, _) in snapshot.excerpts() {
            let buffer_id = buffer.remote_id();
            if !seen_buffers.insert(buffer_id) {
                continue;
            }

            let Some(diff) = snapshot.diff_for_buffer_id(buffer_id) else {
                continue;
            };
            let base_text = diff.base_text();
            for hunk in diff.hunks_intersecting_range(
                language::Anchor::MIN..language::Anchor::MAX,
                buffer,
            ) {
                let added_rows = hunk.range.end.row.saturating_sub(hunk.range.start.row);
                total_additions += added_rows;

                let base_start = base_text.offset_to_point(hunk.diff_base_byte_range.start).row;
                let base_end = base_text.offset_to_point(hunk.diff_base_byte_range.end).row;
                let deleted_rows = base_end.saturating_sub(base_start);
                total_deletions += deleted_rows;
            }
        }

        if total_additions == 0 && total_deletions == 0 {
            return None;
        }

        Some(DiffStat::new(
            "quick-search-commit-diff-stat",
            total_additions as usize,
            total_deletions as usize,
        ))
    }

    fn render_placeholder(message: &str) -> AnyElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(Label::new(message.to_string()).color(Color::Muted))
            .into_any_element()
    }

    fn render_non_text_placeholder(file_name: &str) -> AnyElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(
                v_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        Icon::new(IconName::File)
                            .color(Color::Muted)
                            .size(ui::IconSize::Medium),
                    )
                    .child(
                        Label::new(format!("Preview not available for {}", file_name))
                            .color(Color::Muted)
                            .size(LabelSize::Small),
                    ),
            )
            .into_any_element()
    }

    fn render_error(message: &str) -> AnyElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(
                v_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        Icon::new(IconName::Warning)
                            .color(Color::Warning)
                            .size(ui::IconSize::Medium),
                    )
                    .child(
                        Label::new(message.to_string())
                            .color(Color::Warning)
                            .size(LabelSize::Small),
                    ),
            )
            .into_any_element()
    }

    fn render_preview_header(
        path_text: Arc<str>,
        selected: &QuickMatch,
        highlights: Vec<usize>,
    ) -> Div {
        h_flex()
            .gap_1()
            .items_center()
            .flex_shrink_0()
            .child(
                HighlightedLabel::new(path_text, highlights)
                    .size(LabelSize::XSmall)
                    .color(Color::Muted)
                    .truncate()
                    .single_line(),
            )
            .when_some(selected.location_label.as_ref(), |this, location| {
                this.child(
                    Label::new(location)
                        .size(LabelSize::XSmall)
                        .color(Color::Muted)
                        .single_line(),
                )
            })
    }

    fn render_preview_editor(
        &self,
        selected: &QuickMatch,
        blame: Option<Arc<str>>,
        show_meta: bool,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Div {
        if !show_meta {
            return div()
                .flex_1()
                .size_full()
                .bg(cx.theme().colors().editor_background)
                .rounded_md()
                .border_1()
                .border_color(cx.theme().colors().border_variant)
                .p_1()
                .overflow_hidden()
                .child(self.preview.preview_editor());
        }

        let mut column = v_flex().gap_1().flex_1().size_full();

        let mut meta = h_flex().gap_2().items_center();
        let mut has_meta = false;
        if let Some(loc) = selected.location_label.clone() {
            meta = meta.child(
                Label::new(loc)
                    .size(LabelSize::XSmall)
                    .color(Color::Muted)
                    .single_line()
                    .truncate(),
            );
            has_meta = true;
        }
        if let Some(bl) = blame {
            meta = meta.child(
                Label::new(bl)
                    .size(LabelSize::XSmall)
                    .color(Color::Muted)
                    .single_line()
                    .truncate(),
            );
            has_meta = true;
        }
        if !has_meta {
            meta = meta.child(
                Label::new("Context not available yet")
                    .size(LabelSize::XSmall)
                    .color(Color::Muted)
                    .single_line(),
            );
        }
        column = column.child(meta);

        column.child(
            div()
                .flex_1()
                .size_full()
                .bg(cx.theme().colors().editor_background)
                .rounded_md()
                .border_1()
                .border_color(cx.theme().colors().border_variant)
                .p_1()
                .overflow_hidden()
                .child(self.preview.preview_editor()),
        )
    }
}

struct QuickSearchDelegate {
    match_list: MatchList,
    selection: Option<MatchKey>,
    quick_search: WeakEntity<QuickSearch>,
    workspace: WeakEntity<Workspace>,
    window_handle: gpui::AnyWindowHandle,
    project: Entity<Project>,
    query_error: Option<String>,
    query_notice: Option<String>,
    search_engine: SearchEngine,
    source_registry: source::SourceRegistry,
    current_query: String,
    reset_scroll: bool,
    is_streaming: bool,
    total_results: usize,
    notify_pending: bool,
    notify_debouncer: DebouncedDelay<Picker<QuickSearchDelegate>>,
    notify_scheduled: bool,
    notify_interval_ms: u64,
    next_match_id: MatchId,
    stream_finished: bool,
    history_nav: history::HistoryNavState,
    grouped_list: GroupedListState,
}

impl QuickSearchDelegate {
    fn new(
        quick_search: WeakEntity<QuickSearch>,
        workspace: WeakEntity<Workspace>,
        window_handle: gpui::AnyWindowHandle,
        project: Entity<Project>,
        search_options: SearchOptions,
        source_registry: source::SourceRegistry,
    ) -> Self {
        Self {
            match_list: MatchList::new(MAX_RESULTS),
            selection: None,
            quick_search,
            workspace,
            window_handle,
            project,
            query_error: None,
            query_notice: None,
            search_engine: SearchEngine::new(search_options, QUERY_DEBOUNCE_MS),
            source_registry,
            current_query: String::new(),
            reset_scroll: false,
            is_streaming: false,
            total_results: 0,
            notify_pending: false,
            notify_debouncer: DebouncedDelay::new(),
            notify_scheduled: false,
            notify_interval_ms: 32,
            next_match_id: 0,
            stream_finished: false,
            history_nav: Default::default(),
            grouped_list: Default::default(),
        }
    }

    fn active_source_id(&self) -> Arc<str> {
        self.search_engine.active_source.0.clone()
    }

    fn selected_match(&self) -> Option<&QuickMatch> {
        let key = self.selection?;
        let id = self.match_list.id_by_key(key)?;
        self.match_list.item_by_id(id)
    }

    fn selected_match_index(&self) -> Option<usize> {
        let key = self.selection?;
        let id = self.match_list.id_by_key(key)?;
        self.match_list.index_by_id(id)
    }

    fn selected_row_index(&self) -> usize {
        if self.is_grouped_list_active() {
            let Some(key) = self.selection else {
                return self
                    .grouped_list
                    .rows
                    .iter()
                    .position(|row| matches!(row, GroupedRow::LineMatch { .. }))
                    .unwrap_or(0);
            };
            let Some(id) = self.match_list.id_by_key(key) else {
                return 0;
            };
            self.grouped_list
                .row_index_for_match_id(id)
                .or_else(|| {
                    self.grouped_list
                        .rows
                        .iter()
                        .position(|row| matches!(row, GroupedRow::LineMatch { .. }))
                })
                .unwrap_or(0)
        } else {
            self.selected_match_index().unwrap_or(0)
        }
    }

    fn is_grouped_list_active(&self) -> bool {
        let Some(spec) = self
            .source_registry
            .spec_for_id(&self.search_engine.active_source)
        else {
            return false;
        };
        match spec.list_presentation {
            source::ListPresentation::Grouped => {
                self.current_query.trim().len() >= spec.min_query_len
            }
            source::ListPresentation::Flat => false,
        }
    }

    fn clear_grouped_list_state(&mut self) {
        self.grouped_list.clear();
    }

    fn rebuild_rows_after_match_list_changed(&mut self, cx: &App) {
        if self.is_grouped_list_active() {
            self.rebuild_grouped_rows(cx);
        } else {
            self.clear_grouped_list_state();
        }
    }

    fn rebuild_grouped_rows(&mut self, cx: &App) {
        let selected_id = self.selection.and_then(|k| self.match_list.id_by_key(k));
        let selected_row = self.grouped_list.rebuild(
            &mut self.match_list,
            selected_id,
            &self.project,
            cx,
        );
        if selected_row.is_none() {
            self.selection = self
                .grouped_list
                .rows
                .iter()
                .find_map(|row| match row {
                    GroupedRow::LineMatch { match_id } => self.match_list.key_by_id(*match_id),
                    _ => None,
                });
        }
    }

    fn toggle_group_collapsed(&mut self, key: types::GroupKey, cx: &App) {
        let selected_id = self.selection.and_then(|k| self.match_list.id_by_key(k));
        let selected_row = self.grouped_list.toggle_group_collapsed(
            &mut self.match_list,
            selected_id,
            &self.project,
            key,
            cx,
        );
        if selected_row.is_none() {
            self.selection = self
                .grouped_list
                .rows
                .iter()
                .find_map(|row| match row {
                    GroupedRow::LineMatch { match_id } => self.match_list.key_by_id(*match_id),
                    _ => None,
                });
        }
    }

    fn toggle_all_groups_collapsed(&mut self, clicked: types::GroupKey, cx: &App) {
        let selected_id = self.selection.and_then(|k| self.match_list.id_by_key(k));
        let selected_row = self.grouped_list.toggle_all_groups_collapsed(
            &mut self.match_list,
            selected_id,
            &self.project,
            clicked,
            cx,
        );
        if selected_row.is_none() {
            self.selection = self
                .grouped_list
                .rows
                .iter()
                .find_map(|row| match row {
                    GroupedRow::LineMatch { match_id } => self.match_list.key_by_id(*match_id),
                    _ => None,
                });
        }
    }
}

pub(crate) fn highlight_indices(text: &str, query: &str, case_sensitive: bool) -> Vec<usize> {
    if query.is_empty() {
        return Vec::new();
    }
    if !case_sensitive {
        if query.is_ascii() && text.is_ascii() {
            let mut positions = Vec::new();
            let needle = query.as_bytes();
            let hay = text.as_bytes();
            let mut i = 0;
            while i + needle.len() <= hay.len() {
                if hay[i..i + needle.len()]
                    .iter()
                    .zip(needle.iter())
                    .all(|(h, n)| h.eq_ignore_ascii_case(n))
                {
                    positions.extend(i..i + needle.len());
                }
                i += 1;
            }
            positions.sort_unstable();
            positions.dedup();
            return positions;
        } else {
            return find_case_insensitive_unicode(text, query);
        }
    }

    let mut positions = Vec::new();
    let mut start = 0;
    while let Some(pos) = text[start..].find(query) {
        let abs = start + pos;
        let end = abs + query.len();
        positions.extend(text[abs..end].char_indices().map(|(ix, _)| abs + ix));

        let step = text[abs..].chars().next().map(|c| c.len_utf8()).unwrap_or(1);
        start = abs + step;
    }
    positions.sort_unstable();
    positions.dedup();
    positions
}

fn byte_index_for_column(text: &str, column: usize) -> usize {
    if column == 0 {
        return 0;
    }
    let mut col = 0usize;
    for (ix, _ch) in text.char_indices() {
        if col == column {
            return ix;
        }
        col = col.saturating_add(1);
        if col > column {
            return ix;
        }
    }
    text.len()
}

pub(crate) fn highlight_match_range_in_snippet(
    entry: &QuickMatch,
    snippet_text: &str,
) -> Option<Vec<usize>> {
    let ((start_row, start_col), (end_row, end_col)) =
        match (entry.position(), entry.position_end()) {
            (Some(a), Some(b)) => (a, b),
            _ => return None,
        };

    if start_row != end_row {
        return None;
    }

    let start_byte = byte_index_for_column(snippet_text, start_col as usize);
    if start_byte >= snippet_text.len() {
        return None;
    }

    let mut end_byte = byte_index_for_column(snippet_text, end_col as usize);
    if end_byte <= start_byte {
        end_byte = start_byte
            + snippet_text[start_byte..]
                .chars()
                .next()
                .map(|c| c.len_utf8())
                .unwrap_or(1);
    }
    end_byte = end_byte.min(snippet_text.len());

    let mut indices = Vec::new();
    for (ix, _ch) in snippet_text.char_indices() {
        if ix < start_byte {
            continue;
        }
        if ix >= end_byte {
            break;
        }
        indices.push(ix);
    }
    (!indices.is_empty()).then_some(indices)
}

fn find_case_insensitive_unicode(text: &str, query: &str) -> Vec<usize> {
    if query.is_empty() {
        return Vec::new();
    }

    let mut folded_chars: Vec<char> = Vec::new();
    let mut folded_to_orig_byte: Vec<usize> = Vec::new();
    for (orig_byte, ch) in text.char_indices() {
        for lower_ch in ch.to_lowercase() {
            folded_chars.push(lower_ch);
            folded_to_orig_byte.push(orig_byte);
        }
    }

    let needle: Vec<char> = query.chars().flat_map(|c| c.to_lowercase()).collect();
    if needle.is_empty() {
        return Vec::new();
    }
    let mut positions = Vec::new();
    let mut start = 0;
    while start + needle.len() <= folded_chars.len() {
        if folded_chars[start..start + needle.len()]
            .iter()
            .zip(needle.iter())
            .all(|(a, b)| a == b)
        {
            positions.extend(folded_to_orig_byte[start..start + needle.len()].iter().copied());
        }
        start += 1;
    }
    positions.sort_unstable();
    positions.dedup();
    positions
}

fn stable_source_button_id(id: &source::SourceId) -> u32 {
    let mut hash: u32 = 0x811c_9dc5;
    for b in id.0.as_bytes() {
        hash ^= *b as u32;
        hash = hash.wrapping_mul(0x0100_0193);
    }
    hash
}

impl PickerDelegate for QuickSearchDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        if self.is_grouped_list_active() {
            self.grouped_list.rows.len()
        } else {
            self.match_list.match_count()
        }
    }

    fn selected_index(&self) -> usize {
        if self.is_grouped_list_active() {
            self.selected_row_index()
        } else {
            self.selected_match_index().unwrap_or(0)
        }
    }

    fn can_select(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> bool {
        if !self.is_grouped_list_active() {
            return true;
        }

        matches!(
            self.grouped_list.rows.get(ix),
            Some(GroupedRow::LineMatch { .. })
        )
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        if self.is_grouped_list_active() {
            set_selected_index_grouped(self, ix, cx);
        } else {
            let ix = ix.min(self.match_list.match_count().saturating_sub(1));
            self.selection = self.match_list.item(ix).map(|m| m.key);
            cx.notify();
        }
    }

    fn selected_index_changed(
        &self,
        _ix: usize,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Box<dyn Fn(&mut Window, &mut App) + 'static>> {
        let selected = self.selected_match().cloned();
        if let Some(selected) = selected.clone() {
            self.schedule_enrichment_for(selected, cx);
        }
        let weak_preview_anchors = selected
            .as_ref()
            .map(|selected| {
                self.source_registry.weak_preview_ranges_for_match(
                    self,
                    selected,
                    &self.current_query,
                )
            })
            .unwrap_or_default();
        let use_diff_preview = self
            .source_registry
            .spec_for_id(&self.search_engine.active_source)
            .map(|spec| spec.use_diff_preview)
            .unwrap_or(false);
        let request = selected.as_ref().map_or(PreviewRequest::Empty, |selected| {
            self.source_registry.preview_request_for_match(
                selected,
                weak_preview_anchors.clone(),
                use_diff_preview,
                &self.current_query,
            )
        });
        let quick_search = self.quick_search.clone();

        Some(Box::new(move |window, cx| {
            let Some(quick_search) = quick_search.upgrade() else {
                return;
            };

            quick_search.update(cx, |quick_search, cx| {
                let owner = cx.entity().downgrade();
                quick_search
                    .preview
                    .request_preview(request.clone(), &owner, window, cx);
            });
        }))
    }

    fn separators_after_indices(&self) -> Vec<usize> {
        Vec::new()
    }

    fn placeholder_text(&self, _: &mut Window, _: &mut App) -> Arc<str> {
        self.source_registry
            .spec_for_id(&self.search_engine.active_source)
            .map(|spec| spec.placeholder.clone())
            .unwrap_or_else(|| Arc::from("Search..."))
    }

    fn render_editor(
        &self,
        editor: &Entity<Editor>,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Div {
        let search_options = self.search_engine.search_options;
        let query_error = self.query_error.clone();
        let active_source = self.search_engine.active_source.clone();
        let active_spec = self
            .source_registry
            .spec_for_id(&active_source)
            .or_else(|| self.source_registry.available_sources().first().map(|s| s.spec()));

        let toggle_button = |icon: IconName,
                             active: bool,
                             tooltip: Arc<str>,
                             option: SearchOptions|
         -> IconButton {
            IconButton::new(("quick-search-toggle", icon as u32), icon)
                .shape(IconButtonShape::Square)
                .style(ButtonStyle::Subtle)
                .toggle_state(active)
                .on_click({
                    let qs = self.quick_search.clone();
                    move |_, window, cx| {
                        if let Some(qs) = qs.upgrade() {
                            qs.update(cx, |qs, cx| qs.toggle_search_option(option, window, cx));
                        }
                    }
                })
                .tooltip(Tooltip::text(tooltip))
        };

        let source_button = |id: u32,
                             spec: &'static source::SourceSpec,
                             active: bool,
                             source_id: source::SourceId|
         -> IconButton {
                IconButton::new(("quick-search-source", id), spec.icon)
                    .shape(IconButtonShape::Square)
                    .style(ButtonStyle::Subtle)
                    .toggle_state(active)
                    .on_click({
                        let qs = self.quick_search.clone();
                        move |_, window, cx| {
                            if let Some(qs) = qs.upgrade() {
                                qs.update(cx, |qs, cx| {
                                    qs.set_search_source(source_id.clone(), window, cx)
                                });
                            }
                        }
                    })
                    .tooltip(Tooltip::text(spec.title.to_string()))
            };

        let mut toggles = h_flex().gap_1();
        let supported = active_spec
            .map(|spec| spec.supported_options)
            .unwrap_or_else(SearchOptions::empty);
        if supported.contains(SearchOptions::REGEX) {
            toggles = toggles.child(toggle_button(
                IconName::Regex,
                search_options.contains(SearchOptions::REGEX),
                Arc::from("Use Regular Expressions"),
                SearchOptions::REGEX,
            ));
        }
        if supported.contains(SearchOptions::CASE_SENSITIVE) {
            toggles = toggles.child(toggle_button(
                IconName::CaseSensitive,
                search_options.contains(SearchOptions::CASE_SENSITIVE),
                Arc::from("Match Case Sensitivity"),
                SearchOptions::CASE_SENSITIVE,
            ));
        }
        if supported.contains(SearchOptions::WHOLE_WORD) {
            toggles = toggles.child(toggle_button(
                IconName::WholeWord,
                search_options.contains(SearchOptions::WHOLE_WORD),
                Arc::from("Match Whole Words"),
                SearchOptions::WHOLE_WORD,
            ));
        }
        if supported.contains(SearchOptions::INCLUDE_IGNORED) {
            toggles = toggles.child(toggle_button(
                IconName::Sliders,
                search_options.contains(SearchOptions::INCLUDE_IGNORED),
                Arc::from("Include ignored files"),
                SearchOptions::INCLUDE_IGNORED,
            ));
        }

        let mut sources = h_flex().gap_1();
        for source in self.source_registry.available_sources().iter() {
            let spec = source.spec();
            let id = stable_source_button_id(&spec.id);
            sources = sources.child(source_button(
                id,
                spec,
                active_source == spec.id,
                spec.id.clone(),
            ));
        }

        let controls = h_flex().gap_2().child(sources).child(toggles);

        let border_color = if query_error.is_some() {
            cx.theme().status().error
        } else {
            cx.theme().colors().border_variant
        };

        let bar = input_base_styles(border_color, |container| {
            container
                .h_10()
                .items_center()
                .px_2()
                .gap_2()
                .bg(cx.theme().colors().toolbar_background)
                .child(div().flex_1().child(render_text_input(editor, None, cx)))
                .child(controls)
        });

        v_flex()
            .gap_1()
            .child(bar)
            .when_some(query_error.as_ref(), |this, error| {
                this.child(
                    Label::new(error.clone())
                        .size(LabelSize::Small)
                        .color(Color::Error)
                        .ml_1(),
                )
            })
            .when_some(self.query_notice.as_ref(), |this, notice| {
                this.child(
                    Label::new(notice.clone())
                        .size(LabelSize::XSmall)
                        .color(Color::Muted)
                        .ml_1()
                        .single_line()
                        .truncate(),
                )
            })
    }

    fn select_history(
        &mut self,
        direction: picker::Direction,
        query: &str,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<String> {
        let source_id = self.active_source_id();

        match direction {
            picker::Direction::Up => {
                let should_use_history = self.selected_index() == 0
                    || self.history_nav.index(&source_id).is_some();
                if !should_use_history {
                    return None;
                }

                let history = history::history_list_for_source_id(&source_id);
                if history.is_empty() {
                    return None;
                }

                if self.history_nav.prefix(&source_id).is_none() {
                    self.history_nav.set_prefix(&source_id, query.to_string());
                }

                let next_index = match self.history_nav.index(&source_id) {
                    None => 0,
                    Some(ix) => (ix + 1).min(history.len().saturating_sub(1)),
                };
                self.history_nav.set_index(&source_id, Some(next_index));
                self.history_nav.suppress_reset_once(&source_id);
                Some(history[next_index].clone())
            }
            picker::Direction::Down => {
                let Some(ix) = self.history_nav.index(&source_id) else {
                    return None;
                };

                let history = history::history_list_for_source_id(&source_id);
                if history.is_empty() {
                    let restored = self.history_nav.take_prefix(&source_id).unwrap_or_default();
                    self.history_nav.reset(&source_id);
                    self.history_nav.suppress_reset_once(&source_id);
                    return Some(restored);
                }

                if ix == 0 {
                    let restored = self.history_nav.take_prefix(&source_id).unwrap_or_default();
                    self.history_nav.reset(&source_id);
                    self.history_nav.suppress_reset_once(&source_id);
                    Some(restored)
                } else {
                    let new_ix = ix.saturating_sub(1).min(history.len().saturating_sub(1));
                    self.history_nav.set_index(&source_id, Some(new_ix));
                    self.history_nav.suppress_reset_once(&source_id);
                    Some(history[new_ix].clone())
                }
            }
        }
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> gpui::Task<()> {
        let query = query.trim().to_string();
        if let Some(qs) = self.quick_search.upgrade() {
            let qs = qs.downgrade();
            window.defer(cx, move |window, cx| {
                let Some(qs) = qs.upgrade() else {
                    return;
                };
                qs.update(cx, |qs, cx| {
                    let owner = cx.entity().downgrade();
                    qs.preview
                        .request_preview(PreviewRequest::Empty, &owner, window, cx);
                });
            });
        }
        self.history_nav.on_query_changed(&self.active_source_id());

        let min_len = self
            .source_registry
            .spec_for_id(&self.search_engine.active_source)
            .map(|spec| spec.min_query_len)
            .unwrap_or(crate::MIN_QUERY_LEN);
        if query.len() < min_len {
            self.reset_scroll = false;
            self.is_streaming = false;
            self.total_results = 0;
            self.stream_finished = true;
            self.query_error = None;
            self.query_notice = None;
            self.match_list.clear();
            self.selection = None;
            self.clear_grouped_list_state();
            let _cancel_task = self.search_engine.cancel_pending_debounced_search(cx);
            cx.notify();
            return Task::ready(());
        }

        self.reset_scroll = true;
        self.is_streaming = true;
        self.total_results = 0;
        self.search_engine.schedule_search(query, cx)
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(selected) = self.selected_match().cloned() else {
            if let Some(quick_search) = self.quick_search.upgrade() {
                quick_search.update(cx, |_, cx| cx.emit(DismissEvent));
            } else {
                cx.emit(DismissEvent);
            }
            return;
        };

        let outcome = self
            .source_registry
            .confirm_outcome_for_match(&selected, cx);

        let source_id = self.active_source_id();
        history::push_query_history(&source_id, &self.current_query);
        self.history_nav.reset(&source_id);

        let Some(workspace) = self.workspace.upgrade() else {
            cx.emit(DismissEvent);
            return;
        };

        let window_handle = window.window_handle().downcast::<Workspace>();
        let (project_path, point_range) = match outcome {
            source::ConfirmOutcome::OpenProjectPath {
                project_path,
                point_range,
            } => (Some(project_path), point_range),
            source::ConfirmOutcome::OpenGitCommit {
                repo_workdir,
                sha,
            } => {
                let project = self.project.clone();
                let workspace = workspace.downgrade();
                window.defer(cx, move |window, cx| {
                    let repository = project
                        .read(cx)
                        .git_store()
                        .read(cx)
                        .repositories()
                        .values()
                        .find(|repo| {
                            repo.read(cx).work_directory_abs_path.as_ref() == repo_workdir.as_ref()
                        })
                        .cloned();
                    let Some(repository) = repository else {
                        return;
                    };

                    git_ui::commit_view::CommitView::open(
                        sha.to_string(),
                        repository.downgrade(),
                        workspace,
                        None,
                        None,
                        window,
                        cx,
                    );
                });
                (None, None)
            }
            source::ConfirmOutcome::Dismiss => (None, None),
        };

        if let Some(project_path) = project_path {
            let open_task = workspace.update(cx, |workspace, cx| {
                let allow_preview =
                    PreviewTabsSettings::get_global(cx).enable_preview_from_file_finder;
                if secondary {
                    workspace.split_path_preview(
                        project_path.clone(),
                        allow_preview,
                        None,
                        window,
                        cx,
                    )
                } else {
                    workspace.open_path_preview(
                        project_path.clone(),
                        None,
                        true,
                        allow_preview,
                        true,
                        window,
                        cx,
                    )
                }
            });

            if let Some(window_handle) = window_handle {
                cx.spawn(move |_, app: &mut gpui::AsyncApp| {
                    let mut app = app.clone();
                    async move {
                        let Ok(item) = open_task.await else {
                            return;
                        };
                        let Some(editor) = item.downcast::<Editor>() else {
                            return;
                        };
                        if let Err(update_err) = window_handle.update(
                            &mut app,
                            |_workspace, window, cx| {
                                editor.update(cx, |editor, cx| {
                                    if let Some(point_range) = &point_range {
                                        editor.unfold_ranges(
                                            std::slice::from_ref(point_range),
                                            false,
                                            true,
                                            cx,
                                        );
                                        editor.change_selections(
                                            SelectionEffects::scroll(Autoscroll::fit()),
                                            window,
                                            cx,
                                            |selections| selections.select_ranges([point_range.clone()]),
                                        );
                                    }
                                });
                            },
                        )
                        {
                            debug!(
                                "quick_search: window handle dropped before selection highlight: {:?}",
                                update_err
                            );
                        }
                    }
                })
                .detach();
            }
        }

        if let Some(quick_search) = self.quick_search.upgrade() {
            quick_search.update(cx, |_, cx| cx.emit(DismissEvent));
        } else {
            cx.emit(DismissEvent);
        }
    }

    fn confirm_input(
        &mut self,
        secondary: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        self.confirm(secondary, window, cx);
    }

    fn confirm_completion(
        &mut self,
        _query: String,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<String> {
        None
    }

    fn confirm_update_query(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<String> {
        None
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        if self.is_grouped_list_active() {
            let row = self.grouped_list.rows.get(ix)?.clone();
            match row {
                GroupedRow::FileHeader(header) => {
                    return render_grouped_file_header(self, header, ix, _window, cx);
                }
                GroupedRow::LineMatch { match_id, .. } => {
                    return render_grouped_match_row(
                        self,
                        match_id,
                        ix,
                        selected,
                        _window,
                        cx,
                    );
                }
            }
        }
        render_flat_match_row(self, ix, selected, _window, cx)
    }

    fn render_header(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        if let Some(err) = &self.query_error {
            return Some(
                h_flex()
                    .w_full()
                    .px_2()
                    .py_1()
                    .child(
                        Label::new(err.clone())
                            .size(LabelSize::Small)
                            .color(Color::Error),
                    )
                    .into_any(),
            );
        }

        let truncated = self.match_list.is_truncated();
        let suffix = if truncated { " (truncated)" } else { "" };
        let (results, files) = if self.is_grouped_list_active() {
            let file_count = self
                .grouped_list
                .rows
                .iter()
                .filter(|r| matches!(r, GroupedRow::FileHeader(_)))
                .count();
            (self.total_results, Some(file_count))
        } else {
            (self.total_results, None)
        };

        if results == 0 && !self.is_streaming {
            return None;
        }

        let mut label = if let Some(files) = files {
            let result_word = if results == 1 { "result" } else { "results" };
            let file_word = if files == 1 { "file" } else { "files" };
            format!("{results} {result_word} in {files} {file_word}{suffix}")
        } else {
            format!("{results} results{suffix}")
        };
        if self.is_streaming {
            label.push_str(" (searching)");
        }

        Some(
            h_flex()
                .w_full()
                .px_2()
                .py_1()
                .gap_2()
                .items_center()
                .child(Label::new(label).size(LabelSize::Small).color(Color::Muted))
                .when(self.is_streaming, |this| {
                    this.child(
                        SpinnerLabel::new()
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                })
                .into_any(),
        )
    }

    fn render_footer(
        &self,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        Some(
            h_flex()
                .w_full()
                .p_1p5()
                .gap_1()
                .justify_end()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(
                    Button::new("quick-search-open-split", "Open in Split")
                        .key_binding(
                            KeyBinding::for_action(&menu::SecondaryConfirm, cx)
                                .size(rems_from_px(12.)),
                        )
                        .on_click(|_, window, cx| {
                            window.dispatch_action(menu::SecondaryConfirm.boxed_clone(), cx);
                        }),
                )
                .child(
                    Button::new("quick-search-open", "Open")
                        .key_binding(
                            KeyBinding::for_action(&menu::Confirm, cx).size(rems_from_px(12.)),
                        )
                        .on_click(|_, window, cx| {
                            window.dispatch_action(menu::Confirm.boxed_clone(), cx);
                        }),
                )
                .into_any(),
        )
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.search_engine.cancel();
        self.is_streaming = false;
        self.notify_pending = false;
        self.next_match_id = 0;
        self.clear_grouped_list_state();
        self.notify_scheduled = false;
        self.notify_debouncer = DebouncedDelay::new();
        if let Some(quick_search) = self.quick_search.upgrade() {
            quick_search.update(cx, |_, cx| {
                cx.emit(DismissEvent);
            });
        } else {
            cx.emit(DismissEvent);
        }
    }
}

pub(crate) type PickerHandle = picker::Picker<QuickSearchDelegate>;

#[derive(Clone)]
pub(crate) struct GenerationGuard {
    generation: usize,
    cancel_flag: Arc<AtomicBool>,
}

impl GenerationGuard {
    pub(crate) fn new() -> Self {
        Self {
            generation: 0,
            cancel_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    pub(crate) fn generation(&self) -> usize {
        self.generation
    }

    pub(crate) fn cancel_flag(&self) -> Arc<AtomicBool> {
        self.cancel_flag.clone()
    }

    pub(crate) fn cancel(&self) {
        self.cancel_flag.store(true, Ordering::SeqCst);
    }

    pub(crate) fn begin_request(&mut self) -> (usize, Arc<AtomicBool>) {
        self.cancel_flag.store(true, Ordering::SeqCst);
        self.cancel_flag = Arc::new(AtomicBool::new(false));
        self.generation = self.generation.saturating_add(1);
        (self.generation, self.cancel_flag.clone())
    }
}

pub struct SearchEngine {
    pub search_options: SearchOptions,
    pub active_source: crate::source::SourceId,
    generation_guard: GenerationGuard,
    inflight_results: Option<Receiver<SearchResult>>,
    debouncer: DebouncedDelay<PickerHandle>,
    debounce_ms: u64,
}

pub(crate) enum SourceEvent {
    AppendMatches(Vec<QuickMatch>),
    ApplyPatches(Vec<(MatchId, types::QuickMatchPatch)>),
    Error(String),
    FinishStream,
}

pub(crate) fn apply_source_event(
    picker: WeakEntity<PickerHandle>,
    generation: usize,
    update: SourceEvent,
    app: &mut AsyncApp,
) {
    let Some(picker_entity) = picker.upgrade() else {
        return;
    };

    let mut reset_window_handle: Option<gpui::AnyWindowHandle> = None;

    if let Err(err) = app.update_entity(&picker_entity, |picker, cx| {
        if picker.delegate.search_engine.generation() != generation {
            return;
        }

        match update {
            SourceEvent::Error(message) => {
                picker.delegate.query_error = Some(message);
                picker.delegate.query_notice = None;
                picker.delegate.match_list.clear();
                picker.delegate.selection = None;
                picker.delegate.clear_grouped_list_state();
                picker.delegate.is_streaming = false;
                picker.delegate.total_results = 0;
                picker.delegate.search_engine.inflight_results = None;
                picker.delegate.stream_finished = true;
                picker.delegate.notify_pending = false;
                picker.delegate.notify_scheduled = false;
                picker.delegate.notify_debouncer = DebouncedDelay::new();
                cx.notify();
            }
            SourceEvent::FinishStream => {
                if picker.delegate.stream_finished {
                    return;
                }

                if let Some(spec) = picker
                    .delegate
                    .source_registry
                    .spec_for_id(&picker.delegate.search_engine.active_source)
                {
                    if matches!(spec.sort_policy, crate::source::SortPolicy::FinalSort) {
                        if let Some(source) = picker
                            .delegate
                            .source_registry
                            .source_for_id(&picker.delegate.search_engine.active_source)
                        {
                            picker
                                .delegate
                                .match_list
                                .sort_by(|a, b| source.cmp_matches(a, b));
                            picker.delegate.rebuild_rows_after_match_list_changed(cx);
                        }
                    }
                }

                picker.delegate.stream_finished = true;
                picker.delegate.is_streaming = false;
                picker.delegate.search_engine.inflight_results = None;
                picker.delegate.notify_pending = false;
                picker.delegate.notify_scheduled = false;
                picker.delegate.notify_debouncer = DebouncedDelay::new();
                cx.notify();
            }
            SourceEvent::AppendMatches(mut matches) => {
                let previous_render_rows = picker.delegate.match_count();
                let previous_total = picker.delegate.total_results;

                for m in &mut matches {
                    m.key = types::compute_match_key(m);
                    m.id = picker.delegate.next_match_id;
                    picker.delegate.next_match_id = picker.delegate.next_match_id.saturating_add(1);
                }

                let reached_cap = picker.delegate.match_list.extend(matches);
                picker.delegate.total_results = picker.delegate.match_list.total_results();
                picker.delegate.rebuild_rows_after_match_list_changed(cx);

                let new_render_rows = picker.delegate.match_count();
                let need_notify = new_render_rows != previous_render_rows
                    || picker.delegate.total_results != previous_total;
                picker.delegate.notify_pending = need_notify;

                if picker.delegate.reset_scroll && picker.delegate.match_count() > 0 {
                    reset_window_handle = Some(picker.delegate.window_handle);
                }

                if reached_cap {
                    picker.delegate.search_engine.cancel();
                    if let Some(rx) = picker.delegate.search_engine.inflight_results.take() {
                        rx.close();
                    }
                }

                if picker.delegate.notify_pending && !picker.delegate.notify_scheduled {
                    picker.delegate.notify_scheduled = true;
                    let interval_ms = picker.delegate.notify_interval_ms;
                    picker.delegate.notify_debouncer.fire_new(
                        Duration::from_millis(interval_ms),
                        cx,
                        move |picker, cx| {
                            if picker.delegate.search_engine.generation() != generation {
                                picker.delegate.notify_pending = false;
                                picker.delegate.notify_scheduled = false;
                                return Task::ready(());
                            }

                            if picker.delegate.notify_pending {
                                cx.notify();
                            }
                            picker.delegate.notify_pending = false;
                            picker.delegate.notify_scheduled = false;
                            Task::ready(())
                        },
                    );
                }
            }
            SourceEvent::ApplyPatches(patches) => {
                if patches.is_empty() {
                    return;
                }

                let mut changed = false;
                for (id, patch) in patches {
                    if picker.delegate.match_list.update_by_id(id, patch) {
                        changed = true;
                    }
                }

                if changed {
                    picker.delegate.notify_pending = true;
                    if !picker.delegate.notify_scheduled {
                        picker.delegate.notify_scheduled = true;
                        let interval_ms = picker.delegate.notify_interval_ms;
                        picker.delegate.notify_debouncer.fire_new(
                            Duration::from_millis(interval_ms),
                            cx,
                            move |picker, cx| {
                                if picker.delegate.search_engine.generation() != generation {
                                    picker.delegate.notify_pending = false;
                                    picker.delegate.notify_scheduled = false;
                                    return Task::ready(());
                                }

                                if picker.delegate.notify_pending {
                                    cx.notify();
                                }
                                picker.delegate.notify_pending = false;
                                picker.delegate.notify_scheduled = false;
                                Task::ready(())
                            },
                        );
                    }
                }
            }
        }
    }) {
        debug!("quick_search: apply_ui_update failed: {:?}", err);
        return;
    }

    let Some(window_handle) = reset_window_handle else {
        return;
    };
    let picker_for_reset = picker_entity.clone();
    if let Err(err) = app.update_window(window_handle, move |_, window, cx| {
        picker_for_reset.update(cx, |picker, cx| {
            if picker.delegate.search_engine.generation() != generation {
                return;
            }
            if !picker.delegate.reset_scroll || picker.delegate.match_count() == 0 {
                return;
            }
            picker.delegate.reset_scroll = false;
            let previous_index = picker.delegate.selected_index();
            picker.set_selected_index(0, Some(picker::Direction::Down), true, window, cx);
            let current_index = picker.delegate.selected_index();
            if previous_index == current_index {
                if let Some(action) = picker
                    .delegate
                    .selected_index_changed(current_index, window, cx)
                {
                    action(window, cx);
                }
            }
        });
    }) {
        debug!("quick_search: reset scroll window update failed: {:?}", err);
    }
}

impl SearchEngine {
    pub fn new(search_options: SearchOptions, debounce_ms: u64) -> Self {
        Self {
            search_options,
            active_source: crate::source::default_source_id(),
            generation_guard: GenerationGuard::new(),
            inflight_results: None,
            debouncer: DebouncedDelay::new(),
            debounce_ms,
        }
    }

    pub fn cancel(&mut self) {
        self.generation_guard.cancel();
        if let Some(rx) = self.inflight_results.take() {
            rx.close();
        }
    }

    pub(crate) fn generation(&self) -> usize {
        self.generation_guard.generation()
    }

    pub(crate) fn cancel_flag(&self) -> Arc<AtomicBool> {
        self.generation_guard.cancel_flag()
    }

    pub(crate) fn begin_request(&mut self) -> (usize, Arc<AtomicBool>) {
        self.cancel();
        self.generation_guard.begin_request()
    }

    pub(crate) fn set_inflight_results(&mut self, rx: Receiver<SearchResult>) {
        self.inflight_results = Some(rx);
    }

    pub(crate) fn schedule_search(
        &mut self,
        query: String,
        cx: &mut Context<PickerHandle>,
    ) -> Task<()> {
        self.schedule_search_with_delay(query, Duration::from_millis(self.debounce_ms), cx)
    }

    pub(crate) fn schedule_search_with_delay(
        &mut self,
        query: String,
        delay: Duration,
        cx: &mut Context<PickerHandle>,
    ) -> Task<()> {
        let query = query.trim().to_string();
        self.cancel();
        let query_to_run = query;
        self.debouncer.fire_new(delay, cx, move |picker, cx| {
            picker.delegate.start_search(query_to_run, cx);
            Task::ready(())
        });
        Task::ready(())
    }

    pub(crate) fn cancel_pending_debounced_search(
        &mut self,
        cx: &mut Context<PickerHandle>,
    ) -> Task<()> {
        self.cancel();
        self.debouncer
            .fire_new(Duration::from_millis(0), cx, |_picker, _cx| Task::ready(()));
        Task::ready(())
    }

    #[allow(dead_code)]
    pub fn set_active_source(&mut self, source: crate::source::SourceId) {
        self.active_source = source;
    }
}

impl QuickSearchDelegate {
    pub fn schedule_enrichment_for(&self, m: QuickMatch, cx: &mut Context<PickerHandle>) {
        if m.blame.is_some() {
            return;
        }
        let Some(buffer) = m.buffer().cloned() else {
            return;
        };
        let picker = cx.entity().downgrade();
        let generation = self.search_engine.generation();
        let project = self.project.clone();
        let position_for_cache = m.position();
        let cancel_flag = self.search_engine.cancel_flag();
        let match_id = m.id;
        cx.spawn(move |_, app: &mut gpui::AsyncApp| {
            let mut app = app.clone();
            async move {
                if cancel_flag.load(Ordering::Relaxed) {
                    return;
                }
                if let Some((row, _col)) = position_for_cache {
                    let blame = enrich_blame(&mut app, &project, &buffer, row)
                        .await
                        .map(Arc::from);

                    if cancel_flag.load(Ordering::Relaxed) {
                        return;
                    }

                    let Some(blame_text) = blame else {
                        return;
                    };

                    let patch = types::QuickMatchPatch {
                        blame: PatchValue::SetTo(blame_text),
                        ..Default::default()
                    };
                    apply_patch_to_match(picker, generation, match_id, patch, &mut app);
                }
            }
        })
        .detach();
    }

    pub fn start_search(&mut self, query: String, cx: &mut Context<PickerHandle>) {
        let trimmed = query.trim().to_string();
        let (generation, cancel_flag) = self.search_engine.begin_request();

        self.next_match_id = 0;
        self.current_query = trimmed.clone();
        self.query_error = None;
        self.query_notice = None;
        self.match_list.clear();
        self.selection = None;
        self.clear_grouped_list_state();
        self.notify_pending = false;
        self.notify_scheduled = false;
        self.notify_debouncer = DebouncedDelay::new();
        self.reset_scroll = true;
        self.is_streaming = false;
        self.total_results = 0;
        self.stream_finished = true;
        cx.notify();

        if trimmed.is_empty() {
            self.is_streaming = false;
            self.search_engine.inflight_results = None;
            self.total_results = 0;
            self.stream_finished = true;
            self.selection = None;
            cx.notify();
            return;
        }

        let min_len = self
            .source_registry
            .spec_for_id(&self.search_engine.active_source)
            .map(|s| s.min_query_len)
            .unwrap_or(crate::MIN_QUERY_LEN);
        if trimmed.len() < min_len {
            self.is_streaming = false;
            self.search_engine.inflight_results = None;
            self.total_results = 0;
            self.stream_finished = true;
            self.query_error = None;
            self.match_list.clear();
            self.selection = None;
            self.clear_grouped_list_state();
            cx.notify();
            return;
        }

        self.is_streaming = true;
        self.stream_finished = false;
        cx.notify();

        let picker = cx.entity().downgrade();
        let source_id = self.search_engine.active_source.clone();

        debug!(
            "quick_search: start search source={} query_len={}",
            source_id.0,
            trimmed.len()
        );
        let Some(source) = self.source_registry.source_for_id(&source_id) else {
            self.query_error = Some(format!("Unknown source: {}", source_id.0));
            self.query_notice = None;
            self.match_list.clear();
            self.selection = None;
            self.clear_grouped_list_state();
            self.is_streaming = false;
            self.total_results = 0;
            self.search_engine.inflight_results = None;
            self.stream_finished = true;
            self.notify_pending = false;
            self.notify_scheduled = false;
            self.notify_debouncer = DebouncedDelay::new();
            cx.notify();
            return;
        };
        source.start_search(self, trimmed, generation, cancel_flag, picker, cx);
    }
}

pub(crate) fn record_error(
    picker: WeakEntity<PickerHandle>,
    generation: usize,
    message: String,
    app: &mut AsyncApp,
) {
    apply_source_event(picker, generation, SourceEvent::Error(message), app);
}

pub(crate) fn finish_stream(picker: WeakEntity<PickerHandle>, generation: usize, app: &mut AsyncApp) {
    apply_source_event(picker, generation, SourceEvent::FinishStream, app);
}

pub(crate) fn flush_batch(
    picker: WeakEntity<PickerHandle>,
    generation: usize,
    batch: &mut Vec<QuickMatch>,
    app: &mut AsyncApp,
) {
    if batch.is_empty() {
        return;
    }
    let drained = std::mem::take(batch);
    apply_source_event(picker, generation, SourceEvent::AppendMatches(drained), app);
}

pub(crate) fn split_path_segments(path_label: &str) -> Arc<[Arc<str>]> {
    if path_label.is_empty() {
        return Arc::from(Box::<[Arc<str>]>::default());
    }
    let mut segments: Vec<Arc<str>> = path_label
        .split(|c| c == '/' || c == path::MAIN_SEPARATOR)
        .filter(|part| !part.is_empty())
        .map(|part| Arc::<str>::from(part))
        .collect();
    if segments.is_empty() {
        segments.push(Arc::<str>::from(path_label));
    }
    Arc::from(segments.into_boxed_slice())
}

async fn enrich_blame(
    app: &mut AsyncApp,
    project: &Entity<Project>,
    buffer: &Entity<language::Buffer>,
    row: u32,
) -> Option<String> {
    let blame_task = app.update_entity(project, |project, cx| {
        project.blame_buffer(buffer, None, cx)
    });
    let Ok(blame_task) = blame_task else {
        return None;
    };
    let blame = match blame_task.await {
        Ok(Some(blame)) => blame,
        _ => return None,
    };
    let entry = blame
        .entries
        .iter()
        .find(|entry| entry.range.contains(&row))?;
    Some(format_blame_entry(
        entry.author.as_deref(),
        entry.summary.as_deref(),
        &entry.sha.to_string(),
        entry.original_line_number,
    ))
}

fn format_blame_entry(
    author: Option<&str>,
    summary: Option<&str>,
    sha: &str,
    original_line: u32,
) -> String {
    let author = author.unwrap_or("unknown");
    let summary = summary.unwrap_or("");
    let short_sha = sha.get(..8).unwrap_or(sha);
    if summary.is_empty() {
        format!("{author} · {short_sha} · L{original_line}")
    } else {
        format!("{author} · {summary} · {short_sha} · L{original_line}")
    }
}

fn apply_patch_to_match(
    picker: WeakEntity<PickerHandle>,
    generation: usize,
    id: MatchId,
    patch: types::QuickMatchPatch,
    app: &mut AsyncApp,
) {
    apply_source_event(
        picker,
        generation,
        SourceEvent::ApplyPatches(vec![(id, patch)]),
        app,
    );
}

fn set_selected_index_grouped(
    delegate: &mut QuickSearchDelegate,
    ix: usize,
    cx: &mut Context<Picker<QuickSearchDelegate>>,
) {
    if delegate.grouped_list.rows.is_empty() {
        delegate.selection = None;
        cx.notify();
        return;
    }

    let ix = ix.min(delegate.grouped_list.rows.len().saturating_sub(1));
    let previous_row = delegate.selected_row_index();
    let going_down = ix >= previous_row;

    let mut chosen_row = None;
    if matches!(
        delegate.grouped_list.rows.get(ix),
        Some(GroupedRow::LineMatch { .. })
    ) {
        chosen_row = Some(ix);
    } else {
        if going_down {
            for i in ix..delegate.grouped_list.rows.len() {
                if matches!(delegate.grouped_list.rows[i], GroupedRow::LineMatch { .. }) {
                    chosen_row = Some(i);
                    break;
                }
            }
        } else {
            for i in (0..=ix).rev() {
                if matches!(delegate.grouped_list.rows[i], GroupedRow::LineMatch { .. }) {
                    chosen_row = Some(i);
                    break;
                }
            }
        }
    }

    let Some(row_ix) = chosen_row else {
        cx.notify();
        return;
    };

    let match_id = match &delegate.grouped_list.rows[row_ix] {
        GroupedRow::LineMatch { match_id, .. } => *match_id,
        _ => return,
    };

    delegate.selection = delegate.match_list.key_by_id(match_id);
    cx.notify();
}

fn render_grouped_file_header(
    delegate: &QuickSearchDelegate,
    header: GroupedFileHeader,
    ix: usize,
    _window: &mut Window,
    _cx: &mut Context<Picker<QuickSearchDelegate>>,
) -> Option<ListItem> {
    let is_collapsed = delegate
        .grouped_list
        .collapsed_groups
        .contains(&header.key);
    let chevron_icon = if is_collapsed {
        IconName::ChevronRight
    } else {
        IconName::ChevronDown
    };

    let file_icon = header
        .header
        .icon_path
        .clone()
        .map(Icon::from_path)
        .unwrap_or_else(|| Icon::new(header.header.icon_name));

    let quick_search = delegate.quick_search.clone();
    let key = header.key;

    let right = h_flex()
        .gap_1()
        .items_center()
        .child(
            Label::new(format!("{}", header.match_count))
                .size(LabelSize::XSmall)
                .color(Color::Muted),
        )
        .when_some(header.worktree_name.clone(), |this, name| {
            let label_color = if header.emphasize_worktree {
                Color::Accent
            } else {
                Color::Muted
            };
            this.child(Chip::new(name).label_color(label_color))
        });

    Some(
        ListItem::new(("quick-search-group-header", ix))
            .inset(true)
            .spacing(ListItemSpacing::Sparse)
            .child(
                h_flex()
                    .w_full()
                    .min_w_0()
                    .justify_between()
                    .items_center()
                    .child(
                        h_flex()
                            .min_w_0()
                            .gap_1()
                            .items_center()
                            .cursor_pointer()
                            .child(
                                Icon::new(chevron_icon)
                                    .color(Color::Muted)
                                    .size(ui::IconSize::Small),
                            )
                            .child(file_icon.color(Color::Muted).size(ui::IconSize::Small))
                            .child(
                                Label::new(header.header.title.clone())
                                    .size(LabelSize::Small)
                                    .single_line()
                                    .truncate(),
                            )
                            .when_some(header.header.subtitle.as_ref(), |this, subtitle| {
                                this.child(
                                    Label::new(subtitle.clone())
                                        .size(LabelSize::Small)
                                        .color(Color::Muted)
                                        .single_line()
                                        .truncate(),
                                )
                            }),
                    )
                    .child(right),
            )
            .on_click(move |event, _window, cx| {
                cx.stop_propagation();
                if let Some(qs) = quick_search.upgrade() {
                    qs.update(cx, |qs, cx| {
                        qs.picker.update(cx, |picker, cx| {
                            if event.modifiers().alt {
                                picker
                                    .delegate
                                    .toggle_all_groups_collapsed(key, cx);
                            } else {
                                picker.delegate.toggle_group_collapsed(key, cx);
                            }
                            cx.notify();
                        });
                    });
                }
            }),
    )
}

fn render_grouped_match_row(
    delegate: &QuickSearchDelegate,
    match_id: MatchId,
    ix: usize,
    selected: bool,
    _window: &mut Window,
    cx: &mut Context<Picker<QuickSearchDelegate>>,
) -> Option<ListItem> {
    let entry = delegate.match_list.item_by_id(match_id)?;
    let line_label: String = entry
        .position()
        .map(|(row, _)| format!("{}", row + 1))
        .unwrap_or_default();

    let snippet_known = entry.location_label.is_some()
        || entry.first_line_snippet.is_some()
        || entry.snippet.is_some();
    let snippet_text = entry
        .first_line_snippet
        .as_deref()
        .or_else(|| entry.snippet.as_deref().and_then(|s| s.lines().next()))
        .unwrap_or("");

    let snippet_display = snippet_text.trim_start().to_string();
    let is_blank_line = snippet_known && snippet_display.trim().is_empty();

    let case_sensitive = delegate
        .search_engine
        .search_options
        .contains(SearchOptions::CASE_SENSITIVE);
    let do_highlights = delegate.current_query.len() >= 3
        && !delegate
            .search_engine
            .search_options
            .contains(SearchOptions::REGEX);
    let snippet_highlights = if do_highlights && !is_blank_line {
        highlight_indices(&snippet_display, &delegate.current_query, case_sensitive)
    } else {
        Vec::new()
    };

    let content = h_flex()
        .gap_2()
        .items_center()
        .min_w_0()
        .child(
            div().w(rems(3.0)).flex_shrink_0().child(
                Label::new(line_label)
                    .size(LabelSize::XSmall)
                    .color(Color::Muted)
                    .single_line(),
            ),
        )
        .child(if !snippet_known {
            Label::new("(loading…)")
                .size(LabelSize::Small)
                .color(Color::Muted)
                .single_line()
                .into_any_element()
        } else if is_blank_line {
            Label::new("(blank line)")
                .size(LabelSize::Small)
                .color(Color::Muted)
                .single_line()
                .into_any_element()
        } else {
            HighlightedLabel::new(snippet_display, snippet_highlights)
                .size(LabelSize::Small)
                .single_line()
                .truncate()
                .into_any_element()
        });

    Some(
        ListItem::new(("quick-search-group-match", ix))
            .spacing(ListItemSpacing::Sparse)
            .inset(true)
            .toggle_state(selected)
            .child(content)
            .on_click(
                cx.listener(move |picker, event: &gpui::ClickEvent, window, cx| {
                    cx.stop_propagation();
                    window.prevent_default();
                    picker.set_selected_index(ix, None, false, window, cx);
                    if event.click_count() >= 2 {
                        window.dispatch_action(menu::Confirm.boxed_clone(), cx);
                    }
                }),
            ),
    )
}

fn render_flat_match_row(
    delegate: &QuickSearchDelegate,
    ix: usize,
    selected: bool,
    _window: &mut Window,
    cx: &mut Context<Picker<QuickSearchDelegate>>,
) -> Option<ListItem> {
    let entry = delegate.match_list.item(ix)?;

    let distance = delegate.selected_match_index().unwrap_or(0).abs_diff(ix);
    let within_window = distance <= HIGHLIGHT_WINDOW;

    let case_sensitive = delegate
        .search_engine
        .search_options
        .contains(SearchOptions::CASE_SENSITIVE);
    let do_highlights = within_window && delegate.current_query.len() >= 3;

    let (start_icon, content) = match &entry.kind {
        types::QuickMatchKind::ProjectPath { .. } => {
            let icon = if within_window {
                FileIcons::get_icon(Path::new(&*entry.file_name), cx)
                    .map(|icon_path| Icon::from_path(icon_path).color(Color::Muted))
            } else {
                None
            };

            let file_name_positions = entry
                .file_name_positions
                .as_deref()
                .map(|p| p.to_vec())
                .unwrap_or_default();
            let dir_positions = entry
                .display_path_positions
                .as_deref()
                .map(|p| p.to_vec())
                .unwrap_or_default();

            let content = v_flex()
                .gap_1()
                .overflow_hidden()
                .child(
                    HighlightedLabel::new(entry.file_name.clone(), file_name_positions)
                        .size(LabelSize::Small)
                        .single_line()
                        .truncate(),
                )
                .child(
                    HighlightedLabel::new(entry.display_path.clone(), dir_positions)
                        .size(LabelSize::XSmall)
                        .color(Color::Muted)
                        .single_line()
                        .truncate(),
                );
            (icon, content.into_any_element())
        }
        types::QuickMatchKind::Buffer { .. } => {
            let icon = if within_window {
                FileIcons::get_icon(Path::new(&*entry.file_name), cx)
                    .map(|icon_path| Icon::from_path(icon_path).color(Color::Muted))
            } else {
                None
            };

            let snippet_text = entry
                .first_line_snippet
                .as_ref()
                .map(|s| s.as_ref().to_string())
                .or_else(|| {
                    entry
                        .snippet
                        .as_ref()
                        .map(|s| s.lines().next().unwrap_or("").to_string())
                })
                .unwrap_or_default();
            let snippet_highlights = if within_window {
                highlight_match_range_in_snippet(entry, &snippet_text).unwrap_or_else(|| {
                    if do_highlights {
                        highlight_indices(&snippet_text, &delegate.current_query, case_sensitive)
                    } else {
                        Vec::new()
                    }
                })
            } else {
                Vec::new()
            };

            let content = v_flex()
                .gap_1()
                .overflow_hidden()
                .child(
                    h_flex()
                        .gap_1()
                        .items_center()
                        .overflow_hidden()
                        .child(
                            Label::new(entry.file_name.clone())
                                .size(LabelSize::XSmall)
                                .color(Color::Muted)
                                .single_line()
                                .truncate(),
                        )
                        .when_some(entry.location_label.as_ref(), |this, location| {
                            this.child(
                                Label::new(location.clone())
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted)
                                    .single_line(),
                            )
                        })
                        .child(
                            Label::new(entry.path_label.clone())
                                .size(LabelSize::XSmall)
                                .color(Color::Muted)
                                .single_line()
                                .truncate(),
                        ),
                )
                .child(
                    HighlightedLabel::new(snippet_text, snippet_highlights)
                        .size(LabelSize::Small)
                        .single_line()
                        .truncate(),
                );
            (icon, content.into_any_element())
        }
        types::QuickMatchKind::GitCommit { branch, .. } => {
            let icon = Some(Icon::new(IconName::GitBranchAlt).color(Color::Muted));
            let subject = entry
                .snippet
                .clone()
                .unwrap_or_else(|| Arc::<str>::from(""));
            let subject_highlights = if do_highlights {
                highlight_indices(&subject, &delegate.current_query, case_sensitive)
            } else {
                Vec::new()
            };

            let author = entry
                .location_label
                .clone()
                .unwrap_or_else(|| Arc::<str>::from(""));
            let sha = entry.file_name.clone();
            let repo = entry.display_path.clone();

            let mut meta_parts = Vec::<String>::new();
            if let Some(branch) = branch.as_ref().filter(|b| !b.is_empty()) {
                meta_parts.push(branch.to_string());
            }
            if !author.is_empty() {
                meta_parts.push(author.to_string());
            }
            if !sha.is_empty() {
                meta_parts.push(sha.to_string());
            }
            if !repo.is_empty() {
                meta_parts.push(repo.to_string());
            }
            let meta = meta_parts.join(" · ");
            let content = v_flex()
                .gap_1()
                .overflow_hidden()
                .child(
                    HighlightedLabel::new(subject, subject_highlights)
                        .size(LabelSize::Small)
                        .single_line()
                        .truncate(),
                )
                .child(
                    Label::new(meta)
                        .size(LabelSize::XSmall)
                        .color(Color::Muted)
                        .single_line()
                        .truncate(),
                );

            (icon, content.into_any_element())
        }
    };

    Some(
        ListItem::new(("quick-search-item", ix))
            .spacing(ListItemSpacing::Sparse)
            .inset(true)
            .start_slot::<Icon>(start_icon)
            .toggle_state(selected)
            .child(content)
            .on_click(
                cx.listener(move |picker, event: &gpui::ClickEvent, window, cx| {
                    cx.stop_propagation();
                    window.prevent_default();
                    picker.set_selected_index(ix, None, false, window, cx);
                    if event.click_count() >= 2 {
                        window.dispatch_action(menu::Confirm.boxed_clone(), cx);
                    }
                }),
            ),
    )
}
