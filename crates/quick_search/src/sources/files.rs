use std::{
    path::PathBuf,
    sync::{Arc, OnceLock},
    time::Duration,
};

use file_icons::FileIcons;
use futures::FutureExt as _;
use gpui::{AnyView, App, AppContext, AsyncApp, Context, Entity, Render, Window};
use search::SearchOptions;
use settings::Settings;
use smol::fs;
use smol::io::AsyncReadExt as _;
use ui::IconName;
use ui::prelude::*;
use ui::{Color, Icon, IconSize, Label, LabelSize, div, h_flex, v_flex};

use crate::types::QuickMatch;
use crate::types::{QuickMatchBuilder, QuickMatchKind};
use project::{PathMatchCandidateSet, ProjectPath, WorktreeId};
use util::rel_path::RelPath;
use util::size::format_file_size;

use crate::core::{
    ListPresentation, MatchBatcher, QuickSearchSource, SearchContext, SearchSink, SearchUiContext,
    SortPolicy, SourceId, SourceSpec, SourceSpecCore, SourceSpecUi,
};
use log::debug;
use theme::ThemeSettings;

pub struct FilesSource;

struct FilesDetailsFooter {
    host_state: Entity<crate::core::PreviewFooterHostState>,
    open: bool,
    project: Option<Entity<project::Project>>,
    project_path: Option<ProjectPath>,
    selected_key: Option<crate::types::MatchKey>,
    cancellation: Option<crate::core::FooterCancellation>,
    abs_path_buf: Option<PathBuf>,
    loaded_for_key: Option<crate::types::MatchKey>,
    abs_path: Arc<str>,
    file_type: Arc<str>,
    encoding: Arc<str>,
    line_endings: Arc<str>,
    file_size: Arc<str>,
    lines: Arc<str>,
    shows_loc: bool,
    loading_overlay_visible: bool,
    loading_overlay_nonce: u64,
    last_loading: bool,
    _subscription: gpui::Subscription,
}

impl FilesDetailsFooter {
    fn clear(&mut self) {
        self.project = None;
        self.project_path = None;
        self.selected_key = None;
        self.cancellation = None;
        self.abs_path_buf = None;
        self.loaded_for_key = None;
        self.abs_path = Arc::from("-");
        self.file_type = Arc::from("File");
        self.encoding = Arc::from("-");
        self.line_endings = Arc::from("-");
        self.file_size = Arc::from("-");
        self.lines = Arc::from("-");
        self.shows_loc = false;
    }

    fn set_context(
        &mut self,
        project: Entity<project::Project>,
        project_path: ProjectPath,
        abs_path: Option<PathBuf>,
        cancellation: crate::core::FooterCancellation,
        selected: &QuickMatch,
    ) {
        self.project = Some(project);
        self.project_path = Some(project_path);
        self.abs_path_buf = abs_path;
        self.cancellation = Some(cancellation);
        self.selected_key = Some(selected.key);
        self.loaded_for_key = None;
    }
}

impl Render for FilesDetailsFooter {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        let host_state = self.host_state.read(cx);
        let show_overlay = host_state.loading && self.loading_overlay_visible;
        let buffer_font_size = ThemeSettings::get_global(cx).buffer_font_size(cx);

        let label_width = rems_from_px(72.);
        let kv_row = |label: &'static str, value: Arc<str>| {
            h_flex()
                .gap_3()
                .items_baseline()
                .child(
                    div().w(label_width).child(
                        Label::new(label)
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                            .buffer_font(cx),
                    ),
                )
                .child(
                    div().flex_1().min_w_0().child(
                        Label::new(value)
                            .size(LabelSize::Small)
                            .color(Color::Default)
                            .truncate()
                            .buffer_font(cx),
                    ),
                )
        };

        let icon_path = self.abs_path_buf.as_deref().or_else(|| {
            self.project_path
                .as_ref()
                .map(|project_path| project_path.path.as_std_path())
        });
        let file_icon = icon_path
            .and_then(|path| FileIcons::get_icon(path, cx))
            .map(|icon_path| Icon::from_path(icon_path).color(Color::Muted))
            .unwrap_or_else(|| Icon::new(IconName::File).color(Color::Muted));

        div()
            .relative()
            .w_full()
            .text_size(buffer_font_size)
            .child({
                let lines_label = if self.shows_loc { "LOC" } else { "Lines" };
                v_flex()
                    .gap_1p5()
                    .child(kv_row("Path", self.abs_path.clone()))
                    .child(
                        h_flex()
                            .gap_6()
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(
                                        h_flex()
                                            .gap_3()
                                            .items_baseline()
                                            .child(
                                                div().w(label_width).child(
                                                    Label::new("Type")
                                                        .size(LabelSize::Small)
                                                        .color(Color::Muted)
                                                        .buffer_font(cx),
                                                ),
                                            )
                                            .child(
                                                div().flex_1().min_w_0().child(
                                                    h_flex()
                                                        .gap_2()
                                                        .items_center()
                                                        .child(file_icon.size(IconSize::Small))
                                                        .child(
                                                            Label::new(self.file_type.clone())
                                                                .size(LabelSize::Small)
                                                                .color(Color::Default)
                                                                .truncate()
                                                                .buffer_font(cx),
                                                        ),
                                                ),
                                            ),
                                    )
                                    .child(kv_row("Encoding", self.encoding.clone()))
                                    .child(kv_row("Endings", self.line_endings.clone())),
                            )
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(kv_row("Size", self.file_size.clone()))
                                    .child(kv_row(lines_label, self.lines.clone())),
                            ),
                    )
                    .p_2()
            })
            .when(show_overlay, |this| {
                this.child(
                    div()
                        .absolute()
                        .top(rems_from_px(8.))
                        .right(rems_from_px(8.))
                        .child(
                            ui::SpinnerLabel::new()
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                )
            })
    }
}

async fn detect_encoding_label(abs_path: &std::path::Path) -> Option<Arc<str>> {
    let mut file = match fs::File::open(abs_path).await {
        Ok(file) => file,
        Err(err) => {
            debug!(
                "quick_search: failed to open file for encoding detection: {:?}",
                err
            );
            return None;
        }
    };
    let mut buf = [0u8; 4];
    let read_len = match file.read(&mut buf).await {
        Ok(read_len) => read_len,
        Err(err) => {
            debug!(
                "quick_search: failed to read file for encoding detection: {:?}",
                err
            );
            return None;
        }
    };

    let buf = &buf[..read_len];
    let label = if buf.starts_with(&[0x00, 0x00, 0xFE, 0xFF]) {
        "UTF-32 BE"
    } else if buf.starts_with(&[0xFF, 0xFE, 0x00, 0x00]) {
        "UTF-32 LE"
    } else if buf.starts_with(&[0xEF, 0xBB, 0xBF]) {
        "UTF-8 (BOM)"
    } else if buf.starts_with(&[0xFE, 0xFF]) {
        "UTF-16 BE"
    } else if buf.starts_with(&[0xFF, 0xFE]) {
        "UTF-16 LE"
    } else {
        "UTF-8"
    };

    Some(Arc::from(label))
}

impl FilesSource {
    fn spec_static() -> &'static SourceSpec {
        static SPEC: OnceLock<SourceSpec> = OnceLock::new();
        SPEC.get_or_init(|| SourceSpec {
            id: SourceId(Arc::from("files")),
            core: SourceSpecCore {
                supported_options: SearchOptions::INCLUDE_IGNORED,
                min_query_len: 1,
                sort_policy: SortPolicy::StreamOrder,
            },
            ui: SourceSpecUi {
                title: Arc::from("Files"),
                icon: IconName::File,
                placeholder: Arc::from("Find files..."),
                list_presentation: ListPresentation::Flat,
                use_diff_preview: false,
            },
        })
    }
}

impl QuickSearchSource for FilesSource {
    fn spec(&self) -> &'static SourceSpec {
        Self::spec_static()
    }

    fn create_preview_footer(
        &self,
        _window: &mut Window,
        cx: &mut App,
    ) -> Option<crate::core::FooterInstance> {
        fn spawn_task(
            footer: gpui::WeakEntity<FilesDetailsFooter>,
            host_state: Entity<crate::core::PreviewFooterHostState>,
            project: Entity<project::Project>,
            project_path: ProjectPath,
            abs_path: Option<PathBuf>,
            cancellation: crate::core::FooterCancellation,
            selected_key: crate::types::MatchKey,
            window: &mut Window,
            cx: &mut App,
        ) {
            window
                .spawn(cx, async move |cx| {
                    let set_loading =
                        |loading: bool, label: Option<Arc<str>>, cx: &mut gpui::AsyncWindowContext| {
                            if let Err(err) = cx.update_entity(&host_state, |state, cx| {
                                state.loading = loading;
                                state.loading_label = label;
                                cx.notify();
                            }) {
                                debug!(
                                    "quick_search: failed to update files footer host state: {:?}",
                                    err
                                );
                            }
                        };

                    set_loading(true, Some(Arc::from("Loading details.")), cx);

                    if cancellation.is_cancelled() {
                        set_loading(false, None, cx);
                        return;
                    }

                    let (file_size, encoding) = if let Some(abs_path) = abs_path.as_ref() {
                        let file_size = match fs::metadata(abs_path).await {
                            Ok(meta) => Arc::<str>::from(format_file_size(meta.len(), false)),
                            Err(_) => Arc::<str>::from("-"),
                        };
                        let encoding = detect_encoding_label(abs_path)
                            .await
                            .unwrap_or_else(|| Arc::<str>::from("-"));
                        (file_size, encoding)
                    } else {
                        (Arc::<str>::from("-"), Arc::<str>::from("-"))
                    };

                    if cancellation.is_cancelled() {
                        set_loading(false, None, cx);
                        return;
                    }

                    if let Err(err) = footer.update(cx, |footer, cx| {
                        if footer.selected_key != Some(selected_key) {
                            return;
                        }
                        footer.file_size = file_size.clone();
                        footer.encoding = encoding.clone();
                        cx.notify();
                    }) {
                        debug!(
                            "quick_search: failed to update files footer disk metadata: {:?}",
                            err
                        );
                    }

                    set_loading(true, Some(Arc::from("Opening file.")), cx);

                    let open_task = match cx.update_entity(&project, |project, cx| {
                        project.open_buffer(project_path.clone(), cx)
                    }) {
                        Ok(task) => task,
                        Err(err) => {
                            debug!(
                                "quick_search: failed to start open_buffer for files footer: {:?}",
                                err
                            );
                            set_loading(false, None, cx);
                            return;
                        }
                    };

                    let buffer = match open_task.await {
                        Ok(buffer) => buffer,
                        Err(err) => {
                            debug!(
                                "quick_search: failed to open buffer for files footer: {:?}",
                                err
                            );
                            set_loading(false, None, cx);
                            return;
                        }
                    };

                    if cancellation.is_cancelled() {
                        set_loading(false, None, cx);
                        return;
                    }

                    let extension = project_path
                        .path
                        .as_std_path()
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|s| Arc::<str>::from(s.to_string()));

                    let (language_name, has_language, line_endings, line_count, loc_count) = cx
                        .read_entity(&buffer, |buffer, _| {
                            let snapshot = buffer.snapshot();
                            let line_endings = match snapshot.text.line_ending() {
                                text::LineEnding::Unix => Arc::<str>::from("LF"),
                                text::LineEnding::Windows => Arc::<str>::from("CRLF"),
                            };

                            let line_count = snapshot.text.row_count();
                            let language_name = buffer.language().map(|lang| lang.name());
                            let has_language = language_name.is_some();

                            let mut loc_count = 0u32;
                            if has_language {
                                let mut lines = snapshot.text.as_rope().chunks().lines();
                                while let Some(line) = lines.next() {
                                    let mut has_non_ws = false;
                                    for ch in line.chars() {
                                        if !ch.is_whitespace() {
                                            has_non_ws = true;
                                            break;
                                        }
                                    }
                                    if has_non_ws {
                                        loc_count = loc_count.saturating_add(1);
                                    }
                                }
                            }

                            (language_name, has_language, line_endings, line_count, loc_count)
                        })
                        .unwrap_or_else(|err| {
                            debug!(
                                "quick_search: failed to read buffer snapshot for files footer: {:?}",
                                err
                            );
                            (None, false, Arc::<str>::from("-"), 0, 0)
                        });

                    if cancellation.is_cancelled() {
                        set_loading(false, None, cx);
                        return;
                    }

                    let file_type = match (language_name, extension) {
                        (Some(name), _) => Arc::<str>::from(name.to_string()),
                        (None, Some(ext)) => Arc::<str>::from(ext.to_string()),
                        (None, None) => Arc::<str>::from("File"),
                    };

                    let lines_value = if has_language {
                        Arc::<str>::from(loc_count.to_string())
                    } else {
                        Arc::<str>::from(line_count.to_string())
                    };

                    if let Err(err) = footer.update(cx, |footer, cx| {
                        if footer.selected_key != Some(selected_key) {
                            return;
                        }
                        footer.file_type = file_type;
                        footer.line_endings = line_endings;
                        footer.lines = lines_value;
                        footer.shows_loc = has_language;
                        footer.loaded_for_key = Some(selected_key);
                        cx.notify();
                    }) {
                        debug!("quick_search: failed to update files footer view: {:?}", err);
                    }

                    set_loading(false, None, cx);
                })
                .detach();
        }

        let host = crate::core::PreviewFooterHost::new(cx);
        let host_state = host.state_entity().clone();
        let footer = cx.new(|cx| {
            let subscription =
                cx.observe(&host_state, move |this: &mut FilesDetailsFooter, state, cx| {
                    let loading = state.read(cx).loading;
                if loading && !this.last_loading {
                    this.last_loading = true;
                    this.loading_overlay_visible = false;
                    this.loading_overlay_nonce = this.loading_overlay_nonce.wrapping_add(1);
                    let nonce = this.loading_overlay_nonce;
                    let footer = cx.entity().downgrade();
                    cx.spawn(move |_, app: &mut AsyncApp| {
                        let mut app = app.clone();
                        async move {
                            smol::Timer::after(Duration::from_millis(75)).await;
                            let Some(footer) = footer.upgrade() else {
                                return;
                            };
                            if let Err(err) = footer.update(&mut app, |footer, cx| {
                                if !footer.last_loading || footer.loading_overlay_nonce != nonce {
                                    return;
                                }
                                footer.loading_overlay_visible = true;
                                cx.notify();
                            }) {
                                debug!(
                                    "quick_search: failed to show files footer loading overlay: {:?}",
                                    err
                                );
                            }
                        }
                    })
                    .detach();
                } else if !loading && this.last_loading {
                    this.last_loading = false;
                    this.loading_overlay_visible = false;
                }

                cx.notify();
                });
            FilesDetailsFooter {
                host_state,
                open: false,
                project: None,
                project_path: None,
                selected_key: None,
                cancellation: None,
                abs_path_buf: None,
                loaded_for_key: None,
                abs_path: Arc::from("-"),
                file_type: Arc::from("File"),
                encoding: Arc::from("-"),
                line_endings: Arc::from("-"),
                file_size: Arc::from("-"),
                lines: Arc::from("-"),
                shows_loc: false,
                loading_overlay_visible: false,
                loading_overlay_nonce: 0,
                last_loading: false,
                _subscription: subscription,
            }
        });
        let footer_view = AnyView::from(footer.clone());
        let footer_weak = footer.downgrade();

        Some(crate::core::FooterInstance {
            spec: crate::core::FooterSpec {
                title: Arc::from("Details"),
                toggleable: true,
                default_open: false,
            },
            host: host.clone(),
            view: footer_view,
            handle_event: Arc::new(move |event, window, cx| match event {
                crate::core::FooterEvent::OpenChanged(open) => {
                    let params = match footer_weak.update(cx, |footer, cx| {
                        footer.open = open;
                        cx.notify();

                        if !open {
                            return None;
                        }

                        let selected_key = footer.selected_key?;
                        if footer.loaded_for_key == Some(selected_key) {
                            return None;
                        }

                        Some((
                            footer.project.clone()?,
                            footer.project_path.clone()?,
                            footer.abs_path_buf.clone(),
                            footer.cancellation.clone()?,
                            selected_key,
                        ))
                    }) {
                        Ok(params) => params,
                        Err(err) => {
                            debug!(
                                "quick_search: failed to update files footer state: {:?}",
                                err
                            );
                            None
                        }
                    };

                    let Some((project, project_path, abs_path, cancellation, selected_key)) =
                        params
                    else {
                        return;
                    };
                    if cancellation.is_cancelled() {
                        return;
                    }

                    spawn_task(
                        footer_weak.clone(),
                        host.state_entity().clone(),
                        project,
                        project_path,
                        abs_path,
                        cancellation,
                        selected_key,
                        window,
                        cx,
                    );
                }
                crate::core::FooterEvent::ContextChanged(ctx) => {
                    host.set_loading(false, cx);
                    host.set_loading_label(None, cx);

                    let has_content = ctx
                        .selected
                        .as_ref()
                        .and_then(|selected| selected.project_path())
                        .is_some();
                    host.set_has_content(has_content, cx);

                    let params = match footer_weak.update(cx, |footer, cx| {
                        let Some(selected) = ctx.selected.as_ref() else {
                            footer.clear();
                            cx.notify();
                            return None;
                        };
                        let Some(project_path) = selected.project_path().cloned() else {
                            footer.clear();
                            cx.notify();
                            return None;
                        };

                        let abs_path_buf = ctx.project.read(cx).absolute_path(&project_path, cx);
                        footer.abs_path_buf = abs_path_buf.clone();
                        footer.abs_path = abs_path_buf
                            .as_ref()
                            .map(|p| Arc::<str>::from(p.to_string_lossy().to_string()))
                            .unwrap_or_else(|| Arc::<str>::from("-"));

                        footer.file_type = Arc::from("File");
                        footer.encoding = Arc::from("-");
                        footer.line_endings = Arc::from("-");
                        footer.file_size = Arc::from("-");
                        footer.lines = Arc::from("-");
                        footer.shows_loc = false;

                        footer.set_context(
                            ctx.project.clone(),
                            project_path.clone(),
                            abs_path_buf.clone(),
                            ctx.cancellation.clone(),
                            selected,
                        );

                        cx.notify();
                        if !footer.open {
                            return None;
                        }

                        Some((
                            ctx.project.clone(),
                            project_path,
                            abs_path_buf,
                            ctx.cancellation.clone(),
                            selected.key,
                        ))
                    }) {
                        Ok(params) => params,
                        Err(err) => {
                            debug!(
                                "quick_search: failed to update files footer context: {:?}",
                                err
                            );
                            None
                        }
                    };

                    let Some((project, project_path, abs_path, cancellation, selected_key)) =
                        params
                    else {
                        return;
                    };
                    if cancellation.is_cancelled() {
                        return;
                    }

                    spawn_task(
                        footer_weak.clone(),
                        host.state_entity().clone(),
                        project,
                        project_path,
                        abs_path,
                        cancellation,
                        selected_key,
                        window,
                        cx,
                    );
                }
            }),
        })
    }

    fn start_search(&self, ctx: SearchContext, sink: SearchSink, cx: &mut SearchUiContext<'_>) {
        let include_ignored = ctx
            .search_options()
            .contains(SearchOptions::INCLUDE_IGNORED);
        let path_style = ctx.path_style();
        let worktrees = ctx
            .project()
            .read(cx)
            .worktree_store()
            .read(cx)
            .visible_worktrees_and_single_files(cx)
            .collect::<Vec<_>>();
        let include_root_name = worktrees.len() > 1;

        let mut set_id_to_worktree_id = std::collections::HashMap::<usize, WorktreeId>::new();
        let candidate_sets = worktrees
            .into_iter()
            .map(|worktree| {
                let worktree = worktree.read(cx);
                let snapshot = worktree.snapshot();
                set_id_to_worktree_id.insert(snapshot.id().to_usize(), worktree.id());
                PathMatchCandidateSet {
                    snapshot,
                    include_ignored,
                    include_root_name,
                    candidates: project::Candidates::Files,
                }
            })
            .collect::<Vec<_>>();

        let executor = ctx.background_executor().clone();
        let source_id = self.spec().id.0.clone();
        let query = ctx.query().clone();
        let cancellation = ctx.cancellation().clone();
        let cancel_flag = cancellation.flag();
        let match_arena = ctx.match_arena().clone();
        crate::core::spawn_source_task(cx, sink, move |app, sink| {
            async move {
                if cancellation.is_cancelled() {
                    return;
                }

                let relative_to: Option<Arc<RelPath>> = None;
                let path_matches = fuzzy::match_path_sets(
                    candidate_sets.as_slice(),
                    query.as_ref(),
                    &relative_to,
                    false,
                    2_000,
                    &cancel_flag,
                    executor,
                )
                .await;

                if cancellation.is_cancelled() {
                    return;
                }

                let mut batcher = MatchBatcher::new(match_arena.clone());
                for pm in path_matches {
                    let Some(worktree_id) = set_id_to_worktree_id.get(&pm.worktree_id).copied()
                    else {
                        continue;
                    };

                    let project_path = ProjectPath {
                        worktree_id,
                        path: pm.path.clone(),
                    };

                    let full_path = pm.path_prefix.join(&pm.path);
                    let file_name_str = full_path.file_name().unwrap_or("");
                    let file_name_start = full_path
                        .as_unix_str()
                        .len()
                        .saturating_sub(file_name_str.len());
                    let mut dir_positions = pm.positions.clone();
                    let file_name_positions = dir_positions
                        .iter()
                        .filter_map(|pos| pos.checked_sub(file_name_start))
                        .collect::<Vec<_>>();

                    let display_path_string = full_path
                        .display(path_style)
                        .trim_end_matches(file_name_str)
                        .to_string();
                    dir_positions.retain(|idx| *idx < display_path_string.len());

                    let mut path_label_string = display_path_string.clone();
                    path_label_string.push_str(file_name_str);
                    let path_label: Arc<str> = Arc::from(path_label_string);
                    let display_path: Arc<str> = Arc::from(display_path_string);

                    let file_name: Arc<str> = if file_name_str.is_empty() {
                        path_label.clone()
                    } else {
                        Arc::from(file_name_str.to_string())
                    };

                    batcher.push(
                        QuickMatchBuilder::new(
                            source_id.clone(),
                            QuickMatchKind::ProjectPath { project_path },
                        )
                        .path_label(path_label)
                        .display_path(display_path)
                        .display_path_positions(Some(Arc::<[usize]>::from(dir_positions)))
                        .path_segments_from_label()
                        .file_name(file_name)
                        .file_name_positions(Some(Arc::<[usize]>::from(file_name_positions)))
                        .build(),
                        &sink,
                        app,
                    );
                }

                if !cancellation.is_cancelled() {
                    batcher.finish(&sink, app);
                }
            }
            .boxed_local()
        });
    }
}
