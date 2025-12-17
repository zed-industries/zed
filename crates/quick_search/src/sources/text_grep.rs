use std::{
    collections::HashMap,
    ops::Range,
    path,
    path::Path,
    sync::{Arc, OnceLock},
};

use file_icons::FileIcons;
use futures::FutureExt as _;
use gpui::{AnyView, App, AppContext, AsyncApp, Context, Entity, IntoElement, Render, Window, div};
use language::{Buffer, HighlightId, LanguageRegistry};
use markdown::{Markdown, MarkdownElement};
use search::SearchOptions;
use settings::Settings;
use text::{Anchor as TextAnchor, BufferId, Point, ToOffset, ToPoint};
use theme::ThemeSettings;
use ui::{Color, IconName, LabelSize, SpinnerLabel};
use ui::prelude::*;

use crate::types::{GroupHeader, GroupInfo, QuickMatch, QuickMatchBuilder};
use log::debug;
use project::{HoverBlock, HoverBlockKind, ProjectPath};
use project::search::{SearchQuery, SearchResult};
use smol::future::yield_now;
use util::paths::{PathMatcher, PathStyle};

use crate::core::{
    ListPresentation, MatchBatcher, QuickSearchSource, SearchContext, SearchSink, SearchUiContext,
    SortPolicy, SourceId, SourceSpec, SourceSpecCore, SourceSpecUi,
};
use editor::hover_popover::hover_markdown_style;
use editor::hover_popover::open_markdown_url;

pub struct TextGrepSource;

struct GrepHoverFooter {
    host_state: Entity<crate::core::PreviewFooterHostState>,
    markdown: Option<Entity<Markdown>>,
    message: Option<Arc<str>>,
    _subscription: gpui::Subscription,
}

impl GrepHoverFooter {
    fn clear(&mut self) {
        self.markdown = None;
        self.message = None;
    }

    fn set_markdown(&mut self, markdown: Entity<Markdown>) {
        self.markdown = Some(markdown);
        self.message = None;
    }

    fn set_message(&mut self, message: Arc<str>) {
        self.markdown = None;
        self.message = Some(message);
    }
}

impl Render for GrepHoverFooter {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let host_state = self.host_state.read(cx);
        let loading = host_state.loading;
        let loading_label = host_state
            .loading_label
            .clone()
            .unwrap_or_else(|| Arc::<str>::from("Loading details…"));
        let buffer_font_size = ThemeSettings::get_global(cx).buffer_font_size(cx);
        div()
            .w_full()
            .when(loading, |this| {
                this.child(
                    h_flex()
                        .gap_1()
                        .items_center()
                        .child(
                            SpinnerLabel::new()
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                        .child(
                            div()
                                .text_size(buffer_font_size)
                                .text_color(Color::Muted.color(cx))
                                .child(loading_label.to_string()),
                        ),
                )
                .p_2()
            })
            .when_some(self.markdown.clone(), |this, markdown| {
                let mut style = hover_markdown_style(window, cx);
                style.base_text_style.refine(&gpui::TextStyleRefinement {
                    font_size: Some(buffer_font_size.into()),
                    ..Default::default()
                });
                this.child(
                    MarkdownElement::new(markdown, style)
                        .code_block_renderer(markdown::CodeBlockRenderer::Default {
                            copy_button: false,
                            copy_button_on_hover: false,
                            border: false,
                        })
                        .on_url_click(open_markdown_url)
                        .p_2(),
                )
            })
            .when(self.markdown.is_none() && self.message.is_some(), |this| {
                let message = self
                    .message
                    .clone()
                    .unwrap_or_else(|| Arc::<str>::from("No details available"));
                this.child(
                    div()
                        .text_size(buffer_font_size)
                        .text_color(Color::Muted.color(cx))
                        .child(message.to_string()),
                )
                .p_2()
            })
            .when(!loading && self.markdown.is_none() && self.message.is_none(), |this| {
                this.child(
                    div()
                        .text_size(buffer_font_size)
                        .text_color(Color::Muted.color(cx))
                        .child("No details available"),
                )
                .p_2()
            })
    }
}

fn hover_blocks_to_markdown(blocks: &[HoverBlock]) -> String {
    let mut out = String::new();
    for (index, block) in blocks.iter().enumerate() {
        if index > 0 {
            out.push_str("\n\n");
        }
        match &block.kind {
            HoverBlockKind::PlainText | HoverBlockKind::Markdown => {
                out.push_str(block.text.trim());
            }
            HoverBlockKind::Code { language } => {
                out.push_str("```");
                out.push_str(language);
                out.push('\n');
                out.push_str(block.text.trim());
                out.push_str("\n```");
            }
        }
    }
    out
}

#[derive(Clone)]
struct SyntaxEnrichItem {
    key: crate::types::MatchKey,
    row: u32,
    snippet_len: usize,
}

impl TextGrepSource {
    fn spec_static() -> &'static SourceSpec {
        static SPEC: OnceLock<SourceSpec> = OnceLock::new();
        SPEC.get_or_init(|| SourceSpec {
            id: SourceId(Arc::from("grep")),
            core: SourceSpecCore {
                supported_options: SearchOptions::REGEX
                    | SearchOptions::CASE_SENSITIVE
                    | SearchOptions::WHOLE_WORD
                    | SearchOptions::INCLUDE_IGNORED,
                min_query_len: crate::MIN_QUERY_LEN,
                sort_policy: SortPolicy::StreamOrder,
            },
            ui: SourceSpecUi {
                title: Arc::from("Text"),
                icon: IconName::MagnifyingGlass,
                placeholder: Arc::from("Live grep..."),
                list_presentation: ListPresentation::Grouped,
                use_diff_preview: false,
            },
        })
    }
}

impl QuickSearchSource for TextGrepSource {
    fn spec(&self) -> &'static SourceSpec {
        Self::spec_static()
    }

    fn create_preview_footer(&self, _window: &mut Window, cx: &mut App) -> Option<crate::core::FooterInstance> {
        let host = crate::core::PreviewFooterHost::new(cx);
        let host_state = host.state_entity().clone();
        let footer = cx.new(|cx| {
            let subscription = cx.observe(&host_state, |_, _, cx| cx.notify());
            GrepHoverFooter {
                host_state,
                markdown: None,
                message: None,
                _subscription: subscription,
            }
        });
        let footer_view = AnyView::from(footer.clone());
        let footer_weak = footer.downgrade();
        let host_for_events = host.clone();
        let host_state_for_tasks = host.state_entity().clone();

        Some(crate::core::FooterInstance {
            spec: crate::core::FooterSpec {
                title: Arc::from("Details"),
                toggleable: true,
                default_open: true,
            },
            host,
            view: footer_view,
            handle_event: Arc::new(move |event, window, cx| match event {
                crate::core::FooterEvent::OpenChanged(_open) => {}
                crate::core::FooterEvent::ContextChanged(ctx) => {
                    host_for_events.set_loading(false, cx);
                    host_for_events.set_has_content(false, cx);
                    host_for_events.set_loading_label(None, cx);
                    if let Err(err) = footer_weak.update(cx, |footer, cx| {
                        footer.clear();
                        cx.notify();
                    }) {
                        debug!("quick_search: failed to clear grep footer view: {:?}", err);
                    }

                    let Some(selected) = ctx.selected else {
                        return;
                    };
                    let Some(buffer_id) = selected.buffer_id() else {
                        return;
                    };
                    let Some(point) = selected
                        .ranges()
                        .and_then(|ranges| ranges.first())
                        .map(|range| range.start)
                    else {
                        return;
                    };
                    let project_path = selected.project_path().cloned();
                    let worktree_id = project_path.as_ref().map(|path| path.worktree_id);

                    let project = ctx.project.clone();
                    let cancellation = ctx.cancellation.clone();

                    host_for_events.set_loading(true, cx);
                    host_for_events.set_loading_label(Some(Arc::from("Preparing…")), cx);
                    let footer_for_task = footer_weak.clone();
                    let host_state = host_state_for_tasks.clone();
                    window
                        .spawn(cx, async move |cx| {
                            let set_loading = |loading: bool, cx: &mut gpui::AsyncWindowContext| {
                                if let Err(err) = cx.update_entity(&host_state, |state, cx| {
                                    if state.loading == loading {
                                        return;
                                    }
                                    state.loading = loading;
                                    cx.notify();
                                }) {
                                    debug!(
                                        "quick_search: failed to update grep footer loading state: {:?}",
                                        err
                                    );
                                }
                            };

                            let set_has_content =
                                |has_content: bool, cx: &mut gpui::AsyncWindowContext| {
                                    if let Err(err) =
                                        cx.update_entity(&host_state, |state, cx| {
                                            if state.has_content == has_content {
                                                return;
                                            }
                                            state.has_content = has_content;
                                            cx.notify();
                                        })
                                    {
                                        debug!(
                                            "quick_search: failed to update grep footer content state: {:?}",
                                            err
                                        );
                                    }
                                };

                            let set_loading_label = |label: Option<Arc<str>>,
                                                     cx: &mut gpui::AsyncWindowContext| {
                                if let Err(err) = cx.update_entity(&host_state, |state, cx| {
                                    if state.loading_label == label {
                                        return;
                                    }
                                    state.loading_label = label;
                                    cx.notify();
                                }) {
                                    debug!(
                                        "quick_search: failed to update grep footer loading label: {:?}",
                                        err
                                    );
                                }
                            };

                            cx.background_executor()
                                .timer(std::time::Duration::from_millis(50))
                                .await;
                            if cancellation.is_cancelled() {
                                set_loading(false, cx);
                                return;
                            }

                            let buffer = cx
                                .read_entity(&project, |project, cx| {
                                    project.buffer_for_id(buffer_id, cx)
                                })
                                .unwrap_or_else(|err| {
                                    debug!(
                                        "quick_search: failed to read project buffer for grep footer: {:?}",
                                        err
                                    );
                                    None
                                });

                            set_loading_label(Some(Arc::from("Opening file…")), cx);
                            let buffer = if let Some(buffer) = buffer {
                                buffer
                            } else if let Some(project_path) = &project_path {
                                let open_task = match cx.update_entity(&project, |project, cx| {
                                    project.open_buffer(project_path.clone(), cx)
                                }) {
                                    Ok(task) => task,
                                    Err(err) => {
                                        debug!(
                                            "quick_search: failed to start open_buffer for hover footer: {:?}",
                                            err
                                        );
                                        set_loading(false, cx);
                                        set_has_content(false, cx);
                                        return;
                                    }
                                };
                                match open_task.await {
                                    Ok(buffer) => buffer,
                                    Err(err) => {
                                        debug!(
                                            "quick_search: failed to open buffer for hover footer: {:?}",
                                            err
                                        );
                                        set_loading(false, cx);
                                        set_has_content(false, cx);
                                        return;
                                    }
                                }
                            } else {
                                set_loading(false, cx);
                                set_has_content(false, cx);
                                set_loading_label(None, cx);
                                return;
                            };

                            if cancellation.is_cancelled() {
                                set_loading(false, cx);
                                set_loading_label(None, cx);
                                return;
                            }

                            let (language_name, has_relevant_adapters, has_running_relevant, busy_hint) = cx
                                .read_entity(&project, |project, cx| {
                                    let Some(language) = buffer.read(cx).language().cloned() else {
                                        return (None, false, false, None);
                                    };
                                    let language_name = Some(language.name());
                                    let relevant = project
                                        .languages()
                                        .lsp_adapters(&language.name())
                                        .into_iter()
                                        .map(|adapter| adapter.name())
                                        .collect::<std::collections::HashSet<_>>();
                                    if relevant.is_empty() {
                                        return (language_name, false, false, None);
                                    }

                                    let mut busy_hint: Option<String> = None;
                                    let any_running_relevant = project
                                        .language_server_statuses(cx)
                                        .filter_map(|(_, status)| {
                                            let matches_worktree = match worktree_id {
                                                Some(worktree_id) => {
                                                    status.worktree.is_none()
                                                        || status.worktree == Some(worktree_id)
                                                }
                                                None => true,
                                            };
                                            (relevant.contains(&status.name) && matches_worktree)
                                                .then_some(status)
                                        })
                                        .inspect(|status| {
                                            if busy_hint.is_some() {
                                                return;
                                            }
                                            if status.has_pending_diagnostic_updates {
                                                busy_hint =
                                                    Some("updating diagnostics".to_string());
                                                return;
                                            }
                                            if let Some((_token, progress)) =
                                                status.pending_work.iter().next()
                                            {
                                                if let Some(title) = progress
                                                    .title
                                                    .as_ref()
                                                    .filter(|s| !s.trim().is_empty())
                                                {
                                                    if let Some(pct) = progress.percentage {
                                                        busy_hint =
                                                            Some(format!("{title} ({pct}%)"));
                                                    } else {
                                                        busy_hint = Some(title.to_string());
                                                    }
                                                } else if let Some(message) = progress
                                                    .message
                                                    .as_ref()
                                                    .filter(|s| !s.trim().is_empty())
                                                {
                                                    if let Some(pct) = progress.percentage {
                                                        busy_hint = Some(format!(
                                                            "{message} ({pct}%)"
                                                        ));
                                                    } else {
                                                        busy_hint = Some(message.to_string());
                                                    }
                                                } else {
                                                    busy_hint = Some("busy".to_string());
                                                }
                                            }
                                        })
                                        .next()
                                        .is_some();

                                    (
                                        language_name,
                                        true,
                                        any_running_relevant,
                                        busy_hint.map(Arc::<str>::from),
                                    )
                                })
                                .unwrap_or_else(|err| {
                                    debug!(
                                        "quick_search: failed to read language server statuses for footer: {:?}",
                                        err
                                    );
                                    (None, false, false, None)
                                });

                            if language_name.is_none() {
                                if let Err(err) = footer_for_task.update(cx, |footer, cx| {
                                    footer.set_message(Arc::from("No language detected for this file."));
                                    cx.notify();
                                }) {
                                    debug!("quick_search: failed to update grep footer view: {:?}", err);
                                }
                                set_loading(false, cx);
                                set_has_content(true, cx);
                                set_loading_label(None, cx);
                                return;
                            }

                            if !has_relevant_adapters {
                                let language_name = language_name
                                    .map(|name| name.0.to_string())
                                    .unwrap_or_else(|| "this language".to_string());
                                let label =
                                    format!("No language server configured for {language_name}.");
                                if let Err(err) = footer_for_task.update(cx, |footer, cx| {
                                    footer.set_message(Arc::from(label.clone()));
                                    cx.notify();
                                }) {
                                    debug!("quick_search: failed to update grep footer view: {:?}", err);
                                }
                                set_loading(false, cx);
                                set_has_content(true, cx);
                                set_loading_label(None, cx);
                                return;
                            }

                            if has_relevant_adapters && !has_running_relevant {
                                set_loading_label(Some(Arc::from("Starting language server…")), cx);
                            } else {
                                let label = if let Some(hint) = busy_hint {
                                    Arc::<str>::from(format!("Requesting hover… ({hint})"))
                                } else {
                                    Arc::<str>::from("Requesting hover…")
                                };
                                set_loading_label(Some(label), cx);
                            }

                            let hover_task = cx.update_entity(&project, |project, cx| {
                                project.hover(&buffer, point, cx)
                            });
                            let hover_task = match hover_task {
                                Ok(task) => task,
                                Err(err) => {
                                    debug!("quick_search: hover request failed: {:?}", err);
                                    set_loading(false, cx);
                                    set_has_content(true, cx);
                                    set_loading_label(None, cx);
                                    if let Err(err) = footer_for_task.update(cx, |footer, cx| {
                                        footer.set_message(Arc::from("Failed to request hover."));
                                        cx.notify();
                                    }) {
                                        debug!("quick_search: failed to update grep footer view: {:?}", err);
                                    }
                                    return;
                                }
                            };

                            let hover_task = hover_task.fuse();
                            futures::pin_mut!(hover_task);

                            let busy_hint_for_buffer = |cx: &gpui::AsyncWindowContext| {
                                cx.read_entity(&project, |project, cx| {
                                    let Some(language) = buffer.read(cx).language().cloned() else {
                                        return None;
                                    };
                                    let relevant = project
                                        .languages()
                                        .lsp_adapters(&language.name())
                                        .into_iter()
                                        .map(|adapter| adapter.name())
                                        .collect::<std::collections::HashSet<_>>();
                                    if relevant.is_empty() {
                                        return None;
                                    }

                                    for (_id, status) in project.language_server_statuses(cx) {
                                        if !relevant.contains(&status.name) {
                                            continue;
                                        }
                                        if let Some(worktree_id) = worktree_id {
                                            if let Some(status_worktree_id) = status.worktree {
                                                if status_worktree_id != worktree_id {
                                                    continue;
                                                }
                                            }
                                        }
                                        if status.has_pending_diagnostic_updates {
                                            return Some(Arc::<str>::from("updating diagnostics"));
                                        }
                                        if let Some((_token, progress)) = status.pending_work.iter().next() {
                                            if let Some(title) =
                                                progress.title.as_ref().filter(|s| !s.trim().is_empty())
                                            {
                                                if let Some(pct) = progress.percentage {
                                                    return Some(Arc::<str>::from(format!("{title} ({pct}%)")));
                                                }
                                                return Some(Arc::<str>::from(title.to_string()));
                                            }
                                            if let Some(message) =
                                                progress.message.as_ref().filter(|s| !s.trim().is_empty())
                                            {
                                                if let Some(pct) = progress.percentage {
                                                    return Some(Arc::<str>::from(format!("{message} ({pct}%)")));
                                                }
                                                return Some(Arc::<str>::from(message.to_string()));
                                            }
                                            return Some(Arc::<str>::from("busy"));
                                        }
                                    }

                                    None
                                })
                                .unwrap_or_else(|err| {
                                    debug!(
                                        "quick_search: failed to read language server statuses for footer: {:?}",
                                        err
                                    );
                                    None
                                })
                            };

                            let hovers = loop {
                                if cancellation.is_cancelled() {
                                    set_loading(false, cx);
                                    set_loading_label(None, cx);
                                    return;
                                }

                                let poll_timer = cx
                                    .background_executor()
                                    .timer(std::time::Duration::from_millis(250))
                                    .fuse();
                                futures::pin_mut!(poll_timer);

                                futures::select_biased! {
                                    hovers = hover_task => break hovers,
                                    _ = poll_timer => {
                                        let hint = busy_hint_for_buffer(cx);
                                        let label = if let Some(hint) = hint {
                                            Arc::<str>::from(format!("Requesting hover… ({hint})"))
                                        } else {
                                            Arc::<str>::from("Requesting hover…")
                                        };
                                        set_loading_label(Some(label), cx);
                                    }
                                }
                            };
                            if cancellation.is_cancelled() {
                                set_loading(false, cx);
                                set_loading_label(None, cx);
                                return;
                            }

                            let Some(hovers) = hovers else {
                                let hint = busy_hint_for_buffer(cx);
                                let is_terminal = has_running_relevant && hint.is_none();
                                let message = if !has_running_relevant {
                                    set_loading_label(Some(Arc::from("Starting language server…")), cx);
                                    Arc::<str>::from("Language server still starting…")
                                } else if let Some(hint) = hint {
                                    set_loading_label(Some(Arc::<str>::from(format!(
                                        "Language server busy… ({hint})"
                                    ))), cx);
                                    Arc::<str>::from(format!("Language server busy… ({hint})"))
                                } else {
                                    Arc::<str>::from("Hover is not available for this language server.")
                                };
                                if let Err(err) = footer_for_task.update(cx, |footer, cx| {
                                    footer.set_message(message.clone());
                                    cx.notify();
                                }) {
                                    debug!("quick_search: failed to update grep footer view: {:?}", err);
                                }
                                set_has_content(true, cx);
                                if is_terminal {
                                    set_loading(false, cx);
                                    set_loading_label(None, cx);
                                }
                                return;
                            };

                            let mut blocks: Vec<HoverBlock> = Vec::new();
                            let mut language_name: Option<language::LanguageName> = None;
                            for hover in hovers {
                                if hover.is_empty() {
                                    continue;
                                }
                                if language_name.is_none() {
                                    language_name =
                                        hover.language.as_ref().map(|language| language.name());
                                }
                                blocks.extend(hover.contents);
                            }

                            let text = hover_blocks_to_markdown(&blocks);
                            if text.trim().is_empty() {
                                let hint = busy_hint_for_buffer(cx);
                                let is_terminal = has_running_relevant && hint.is_none();
                                let message = if !has_running_relevant {
                                    set_loading_label(Some(Arc::from("Starting language server…")), cx);
                                    Arc::<str>::from("Language server still starting…")
                                } else if let Some(hint) = hint {
                                    set_loading_label(Some(Arc::<str>::from(format!(
                                        "Language server busy… ({hint})"
                                    ))), cx);
                                    Arc::<str>::from(format!(
                                        "Language server busy… ({hint}). Try again shortly."
                                    ))
                                } else {
                                    Arc::<str>::from("No hover information at this position.")
                                };
                                if let Err(err) = footer_for_task.update(cx, |footer, cx| {
                                    footer.set_message(message.clone());
                                    cx.notify();
                                }) {
                                    debug!("quick_search: failed to update grep footer view: {:?}", err);
                                }
                                set_has_content(true, cx);
                                if is_terminal {
                                    set_loading(false, cx);
                                    set_loading_label(None, cx);
                                }
                                return;
                            }

                            let language_registry = cx
                                .read_entity(&project, |project, _| project.languages().clone())
                                .map(Some)
                                .unwrap_or_else(|err| {
                                    debug!(
                                        "quick_search: failed to read language registry for hover footer: {:?}",
                                        err
                                    );
                                    None
                                });

                            let markdown = match cx.new(|cx| {
                                Markdown::new(text.into(), language_registry, language_name, cx)
                            }) {
                                Ok(markdown) => markdown,
                                Err(err) => {
                                    debug!("quick_search: failed to build hover markdown: {:?}", err);
                                    set_loading(false, cx);
                                    set_has_content(false, cx);
                                    set_loading_label(None, cx);
                                    return;
                                }
                            };

                            if let Err(err) = footer_for_task.update(cx, |footer, cx| {
                                footer.set_markdown(markdown);
                                cx.notify();
                            }) {
                                debug!("quick_search: failed to update grep footer view: {:?}", err);
                                set_loading(false, cx);
                                set_has_content(false, cx);
                                set_loading_label(None, cx);
                                return;
                            }

                            set_loading(false, cx);
                            set_has_content(true, cx);
                            set_loading_label(None, cx);
                        })
                        .detach();
                }
            }),
        })
    }

    fn start_search(
        &self,
        ctx: SearchContext,
        sink: SearchSink,
        cx: &mut SearchUiContext<'_>,
    ) {
        let project = ctx.project().clone();
        let search_options = ctx.search_options();
        let source_id = self.spec().id.0.clone();
        let path_style = ctx.path_style();
        let language_registry = ctx.language_registry().clone();
        let query = ctx.query().clone();
        let cancellation = ctx.cancellation().clone();

        cx.spawn(move |_, app: &mut gpui::AsyncApp| {
            let mut app = app.clone();
            async move {
                if cancellation.is_cancelled() {
                    return;
                }

                let search_query = match app.update_entity(&project, |_project, _| {
                    let include = PathMatcher::default();
                    let exclude = PathMatcher::default();
                    if search_options.contains(SearchOptions::REGEX) {
                        SearchQuery::regex(
                            query.as_ref(),
                            search_options.contains(SearchOptions::WHOLE_WORD),
                            search_options.contains(SearchOptions::CASE_SENSITIVE),
                            search_options.contains(SearchOptions::INCLUDE_IGNORED),
                            false,
                            include,
                            exclude,
                            false,
                            None,
                        )
                    } else {
                        SearchQuery::text(
                            query.as_ref(),
                            search_options.contains(SearchOptions::WHOLE_WORD),
                            search_options.contains(SearchOptions::CASE_SENSITIVE),
                            search_options.contains(SearchOptions::INCLUDE_IGNORED),
                            include,
                            exclude,
                            false,
                            None,
                        )
                    }
                }) {
                    Ok(Ok(query)) => query,
                    Ok(Err(err)) => {
                        sink.record_error(err.to_string(), &mut app);
                        return;
                    }
                    Err(err) => {
                        sink.record_error(err.to_string(), &mut app);
                        return;
                    }
                };

                let receiver = match app
                    .update_entity(&project, |project, cx| project.search(search_query, cx))
                {
                    Ok(receiver) => receiver,
                    Err(err) => {
                        sink.record_error(err.to_string(), &mut app);
                        return;
                    }
                };

                sink.set_inflight_results(receiver.clone(), &mut app);

                let mut batcher = MatchBatcher::new();
                let mut syntax_workers: HashMap<BufferId, async_channel::Sender<SyntaxEnrichItem>> =
                    HashMap::new();
                loop {
                    let result = match receiver.recv().await {
                        Ok(r) => r,
                        Err(_) => break,
                    };
                    if cancellation.is_cancelled() {
                        break;
                    }

                    match result {
                        SearchResult::Buffer { buffer, ranges } => {
                            if let Some(out) = build_matches_for_buffer(
                                &mut app,
                                &buffer,
                                ranges,
                                &path_style,
                                &source_id,
                            ) {
                                if !out.pending_syntax.is_empty() {
                                    ensure_syntax_worker(
                                        &mut app,
                                        &mut syntax_workers,
                                        out.buffer_id,
                                        buffer.clone(),
                                        sink.clone(),
                                        language_registry.clone(),
                                    );
                                    if let Some(sender) = syntax_workers.get(&out.buffer_id) {
                                        for item in out.pending_syntax {
                                            if let Err(err) = sender.try_send(item) {
                                                debug!(
                                                    "quick_search: failed to queue syntax enrich item: {:?}",
                                                    err
                                                );
                                                break;
                                            }
                                        }
                                    }
                                }

                                for match_item in out.matches {
                                    batcher.push(match_item, &sink, &mut app);
                                }
                                if cancellation.is_cancelled() {
                                    break;
                                }
                            }
                        }
                        SearchResult::LimitReached => {
                            batcher.flush(&sink, &mut app);
                            if cancellation.is_cancelled() {
                                break;
                            }
                            break;
                        }
                    }

                    yield_now().await;
                }

                drop(syntax_workers);
                if !cancellation.is_cancelled() {
                    batcher.finish(&sink, &mut app);
                }
            }
        })
        .detach();
    }
}

fn elide_path(segments: &[Arc<str>]) -> Arc<str> {
    const MAX_SEGMENTS: usize = 5;
    let Some(head) = segments.first() else {
        return Arc::<str>::from("");
    };
    if segments.len() <= MAX_SEGMENTS {
        return Arc::<str>::from(segments.join("/"));
    }

    let tail_count = MAX_SEGMENTS.saturating_sub(1);
    let tail_start = segments.len().saturating_sub(tail_count);
    let mut parts = Vec::with_capacity(2 + tail_count);
    parts.push(head.clone());
    parts.push(Arc::<str>::from("…"));
    parts.extend_from_slice(&segments[tail_start..]);
    Arc::<str>::from(parts.join("/"))
}

fn clip_snippet(text: &str) -> (String, usize) {
    if text.len() <= crate::MAX_SNIPPET_BYTES {
        return (text.to_string(), text.len());
    }

    let suffix = "…";
    let max_content_bytes = crate::MAX_SNIPPET_BYTES.saturating_sub(suffix.len());
    let mut end = max_content_bytes.min(text.len());
    while end > 0 && !text.is_char_boundary(end) {
        end -= 1;
    }

    let mut out = String::with_capacity(end + suffix.len());
    out.push_str(&text[..end]);
    out.push_str(suffix);
    (out, end)
}

fn coalesce_syntax_runs(runs: &mut Vec<(Range<usize>, HighlightId)>) {
    if runs.len() <= 1 {
        return;
    }
    runs.sort_by_key(|(range, _)| (range.start, range.end));
    let mut out: Vec<(Range<usize>, HighlightId)> = Vec::with_capacity(runs.len());
    for (range, id) in runs.drain(..) {
        if let Some((last_range, last_id)) = out.last_mut() {
            if *last_id == id && last_range.end == range.start {
                last_range.end = range.end;
                continue;
            }
        }
        out.push((range, id));
    }
    *runs = out;
}

struct BuildMatchesOutput {
    matches: Vec<QuickMatch>,
    pending_syntax: Vec<SyntaxEnrichItem>,
    buffer_id: BufferId,
}

fn ensure_syntax_worker(
    app: &mut AsyncApp,
    workers: &mut HashMap<BufferId, async_channel::Sender<SyntaxEnrichItem>>,
    buffer_id: BufferId,
    buffer: gpui::Entity<Buffer>,
    sink: SearchSink,
    language_registry: Arc<LanguageRegistry>,
) {
    if workers.contains_key(&buffer_id) {
        return;
    }

    let (sender, receiver) = async_channel::unbounded();
    workers.insert(buffer_id, sender);

    app.spawn(async move |app| {
        let mut language_attempted = false;
        let mut queued: Vec<SyntaxEnrichItem> = Vec::new();

        loop {
            let first = match receiver.recv().await {
                Ok(item) => item,
                Err(_) => break,
            };
            queued.push(first);
            while let Ok(item) = receiver.try_recv() {
                queued.push(item);
            }

            if sink.is_cancelled() {
                break;
            }

            let snapshot = match app.read_entity(&buffer, |b, _| b.snapshot()) {
                Ok(s) => s,
                Err(_) => break,
            };

            if snapshot.language().is_none() && !language_attempted {
                language_attempted = true;
                let file = match app.read_entity(&buffer, |b, _| b.file().cloned()) {
                    Ok(file) => file,
                    Err(err) => {
                        debug!(
                            "quick_search: failed to read file for syntax enrich worker: {:?}",
                            err
                        );
                        None
                    }
                };
                if let Some(file) = file {
                    let available = match app.update({
                        let language_registry = language_registry.clone();
                        let file = file.clone();
                        move |cx| language_registry.language_for_file(&file, None, cx)
                    }) {
                        Ok(available) => available,
                        Err(err) => {
                            debug!(
                                "quick_search: failed to detect language for syntax enrich worker: {:?}",
                                err
                            );
                            None
                        }
                    };
                    if let Some(available) = available {
                        let language_receiver = language_registry.load_language(&available);
                        if let Ok(Ok(language)) = language_receiver.await {
                            if let Err(err) = app.update_entity(&buffer, |b, cx| {
                                b.set_language_registry(language_registry.clone());
                                b.set_language_async(Some(language.clone()), cx);
                            }) {
                                debug!(
                                    "quick_search: failed to set language for syntax enrich worker: {:?}",
                                    err
                                );
                            }
                        }
                    }
                }
            }

            let parsing_idle = app.read_entity(&buffer, |b, _| b.parsing_idle());
            if let Ok(idle) = parsing_idle {
                idle.await;
            }

            while let Ok(item) = receiver.try_recv() {
                queued.push(item);
            }

            if sink.is_cancelled() {
                break;
            }

            let snapshot = match app.read_entity(&buffer, |b, _| b.snapshot()) {
                Ok(s) => s,
                Err(_) => break,
            };
            if snapshot.language().is_none() {
                queued.clear();
                continue;
            }

            let mut patches: Vec<(crate::types::MatchKey, crate::types::QuickMatchPatch)> =
                Vec::new();

            for item in queued.drain(..) {
                let snippet_len = item.snippet_len;
                if snippet_len == 0 {
                    continue;
                }

                let max_row = snapshot.text.max_point().row;
                let row = item.row.min(max_row);
                let line_start = Point::new(row, 0);
                let line_end = Point::new(row, snapshot.text.line_len(row));
                let line_start_offset = snapshot.text.point_to_offset(line_start);
                let line_end_offset = snapshot.text.point_to_offset(line_end);

                let line_text: String = snapshot
                    .text_for_range(line_start_offset..line_end_offset)
                    .collect();
                let line_trimmed_end = line_text.trim_end();
                let trim_start = line_trimmed_end.len() - line_trimmed_end.trim_start().len();
                let snippet_end_abs = (trim_start + snippet_len).min(line_trimmed_end.len());
                if trim_start >= snippet_end_abs {
                    continue;
                }

                let mut highlight_ids: Vec<(Range<usize>, HighlightId)> = Vec::new();
                let mut current_offset = 0usize;
                for chunk in snapshot.chunks(line_start_offset..line_end_offset, true) {
                    let chunk_len = chunk.text.len();

                    if let Some(highlight_id) = chunk.syntax_highlight_id {
                        let abs_start = current_offset;
                        let abs_end = current_offset + chunk_len;
                        let rel_start = abs_start.saturating_sub(trim_start);
                        let rel_end = abs_end.saturating_sub(trim_start);
                        if rel_end > 0 && rel_start < snippet_len {
                            let clamped_start = rel_start.min(snippet_len);
                            let clamped_end = rel_end.min(snippet_len);
                            if clamped_start < clamped_end {
                                highlight_ids.push((clamped_start..clamped_end, highlight_id));
                            }
                        }
                    }

                    current_offset += chunk_len;
                }

                if highlight_ids.is_empty() {
                    continue;
                }
                coalesce_syntax_runs(&mut highlight_ids);

                patches.push((
                    item.key,
                    crate::types::QuickMatchPatch {
                        snippet_syntax_highlights: crate::types::PatchValue::SetTo(Arc::from(
                            highlight_ids.into_boxed_slice(),
                        )),
                        ..Default::default()
                    },
                ));
            }

            if !patches.is_empty() {
                sink.apply_patches_by_key(patches, app);
            }
        }
    })
    .detach();
}

fn build_matches_for_buffer(
    app: &mut AsyncApp,
    buffer: &gpui::Entity<Buffer>,
    ranges: Vec<Range<TextAnchor>>,
    path_style: &PathStyle,
    source_id: &Arc<str>,
) -> Option<BuildMatchesOutput> {
    let snapshot = match app.read_entity(buffer, |b, _| b.snapshot()) {
        Ok(s) => s,
        Err(_) => return None,
    };
    let buffer_id = snapshot.text.remote_id();

    let (project_path, path_label): (Option<ProjectPath>, Arc<str>) = app
        .read_entity(buffer, |b, cx| {
            let Some(file) = b.file() else {
                return (None, Arc::<str>::from("<untitled>"));
            };
            let project_path = ProjectPath {
                worktree_id: file.worktree_id(cx),
                path: file.path().clone(),
            };
            let path_label: Arc<str> =
                Arc::<str>::from(file.path().display(*path_style).to_string());
            (Some(project_path), path_label)
        })
        .unwrap_or((None, Arc::<str>::from("<untitled>")));

    let file_name: Arc<str> = path_label
        .rsplit_once(path::MAIN_SEPARATOR)
        .map(|(_, name)| Arc::<str>::from(name))
        .or_else(|| {
            path_label
                .rsplit_once('/')
                .map(|(_, name)| Arc::<str>::from(name))
        })
        .unwrap_or_else(|| path_label.clone());

    let path_segments = crate::types::split_path_segments(&path_label);
    let display_path: Arc<str> = elide_path(&path_segments);

    let group: Option<Arc<GroupInfo>> = project_path.as_ref().map(|project_path| {
        let title: Arc<str> = project_path
            .path
            .file_name()
            .map(|name| Arc::<str>::from(name.to_string()))
            .unwrap_or_else(|| Arc::<str>::from(project_path.path.as_unix_str().to_string()));
        let subtitle: Option<Arc<str>> = project_path.path.parent().and_then(|path| {
            let s = path.as_unix_str().to_string();
            (!s.is_empty()).then(|| Arc::<str>::from(s))
        });
        let icon_path = app
            .update({
                let file_name = file_name.clone();
                move |cx| FileIcons::get_icon(Path::new(file_name.as_ref()), cx)
            })
            .unwrap_or_else(|err| {
                debug!("quick_search: failed to get icon for grep group: {:?}", err);
                None
            });

        Arc::new(GroupInfo {
            key: crate::types::compute_group_key_for_project_path(source_id, project_path),
            header: GroupHeader {
                icon_name: IconName::File,
                icon_path,
                title,
                subtitle,
            },
        })
    });

    let mut per_line: std::collections::HashMap<u32, Vec<(u32, Range<TextAnchor>)>> =
        std::collections::HashMap::new();
    let mut line_order: Vec<u32> = Vec::new();
    for range in ranges {
        let start = range.start.to_point(&snapshot.text);
        let row = start.row;
        if !per_line.contains_key(&row) {
            line_order.push(row);
        }
        per_line.entry(row).or_default().push((start.column, range));
    }

    let mut matches = Vec::with_capacity(line_order.len());
    let mut pending_syntax: Vec<SyntaxEnrichItem> = Vec::new();
    for row in line_order {
        let mut items = match per_line.remove(&row) {
            Some(v) => v,
            None => continue,
        };
        items.sort_by_key(|(col, _)| *col);

        let mut ranges_for_line = Vec::with_capacity(items.len());
        for (_, r) in &items {
            ranges_for_line.push(r.clone());
        }

        let Some((first_col, first_range)) = items.first() else {
            continue;
        };
        let start_point = first_range.start.to_point(&snapshot.text);
        let location_label: Option<Arc<str>> =
            Some(format!(":{}:{}", row + 1, first_col + 1).into());

        let max_row = snapshot.text.max_point().row;
        let row = row.min(max_row);
        let line_start = Point::new(row, 0);
        let line_end = Point::new(row, snapshot.text.line_len(row));
        let line_start_offset = snapshot.text.point_to_offset(line_start);
        let line_end_offset = snapshot.text.point_to_offset(line_end);
        let line_text: String = snapshot
            .text_for_range(line_start_offset..line_end_offset)
            .collect();
        let line_trimmed_end = line_text.trim_end();
        let trim_start = line_trimmed_end.len() - line_trimmed_end.trim_start().len();
        let line_trimmed = &line_trimmed_end[trim_start..];
        let (snippet_string, snippet_content_len) = clip_snippet(line_trimmed);
        let snippet: Arc<str> = Arc::<str>::from(snippet_string);

        let mut snippet_match_positions: Vec<Range<usize>> = Vec::new();
        for r in &ranges_for_line {
            let match_start_offset = r.start.to_offset(&snapshot.text);
            let match_end_offset = r.end.to_offset(&snapshot.text);

            let start_in_line = match_start_offset.saturating_sub(line_start_offset);
            let end_in_line = match_end_offset.saturating_sub(line_start_offset);

            let start_in_preview = start_in_line.saturating_sub(trim_start);
            let end_in_preview = end_in_line.saturating_sub(trim_start);

            if start_in_preview >= snippet_content_len || end_in_preview == 0 {
                continue;
            }

            let clamped_start = start_in_preview.min(snippet_content_len);
            let clamped_end = end_in_preview.min(snippet_content_len);
            if clamped_start >= clamped_end {
                continue;
            }

            let snippet_str = snippet.as_ref();
            let mut safe_start = clamped_start.min(snippet_str.len());
            while safe_start > 0 && !snippet_str.is_char_boundary(safe_start) {
                safe_start -= 1;
            }
            let mut safe_end = clamped_end.min(snippet_str.len());
            while safe_end < snippet_str.len() && !snippet_str.is_char_boundary(safe_end) {
                safe_end += 1;
            }

            if safe_start < safe_end {
                snippet_match_positions.push(safe_start..safe_end);
            }
        }
        snippet_match_positions.sort_by_key(|r| (r.start, r.end));
        snippet_match_positions.dedup();

        let mut snippet_syntax_highlights: Vec<(Range<usize>, HighlightId)> = Vec::new();
        if snippet_content_len > 0 && snapshot.language().is_some() {
            let mut rel_offset = 0usize;
            let mut chunks = snapshot.chunks(line_start_offset..line_end_offset, true);
            for chunk in chunks.by_ref() {
                let chunk_len = chunk.text.len();
                let chunk_start = rel_offset;
                let chunk_end = rel_offset + chunk_len;
                rel_offset = chunk_end;

                let chunk_start = chunk_start.min(line_trimmed_end.len());
                let chunk_end = chunk_end.min(line_trimmed_end.len());
                let start_abs = chunk_start.max(trim_start);
                let end_abs = chunk_end.min(trim_start + snippet_content_len);
                if start_abs >= end_abs {
                    continue;
                }

                if let Some(id) = chunk.syntax_highlight_id {
                    let start_rel = start_abs - trim_start;
                    let end_rel = end_abs - trim_start;
                    if start_rel < end_rel {
                        snippet_syntax_highlights.push((start_rel..end_rel, id));
                    }
                }
            }
            coalesce_syntax_runs(&mut snippet_syntax_highlights);
        }

        let ranges_for_line_points = ranges_for_line
            .iter()
            .map(|range| {
                let start = range.start.to_point(&snapshot.text);
                let end = range.end.to_point(&snapshot.text);
                start..end
            })
            .collect::<Vec<_>>();
        let kind = crate::types::QuickMatchKind::Buffer {
            buffer_id,
            ranges: ranges_for_line_points.clone(),
            position: Some((row, start_point.column)),
        };
        let snippet_for_match = snippet.clone();
        let snippet_match_positions = (!snippet_match_positions.is_empty())
            .then(|| Arc::<[Range<usize>]>::from(snippet_match_positions));
        let snippet_syntax_highlights = (!snippet_syntax_highlights.is_empty())
            .then(|| Arc::<[(Range<usize>, HighlightId)]>::from(snippet_syntax_highlights));

        let mut match_item = QuickMatchBuilder::new(source_id.clone(), kind)
            .action(match project_path.clone() {
                Some(project_path) => crate::types::MatchAction::OpenProjectPath {
                    project_path,
                    point_range: ranges_for_line_points.first().cloned(),
                },
                None => crate::types::MatchAction::Dismiss,
            })
            .group(group.clone())
            .path_label(path_label.clone())
            .display_path(display_path.clone())
            .path_segments(path_segments.clone())
            .file_name(file_name.clone())
            .location_label(location_label)
            .snippet(Some(snippet_for_match))
            .first_line_snippet(Some(snippet))
            .snippet_match_positions(snippet_match_positions)
            .snippet_syntax_highlights(snippet_syntax_highlights)
            .build();
        match_item.key = crate::types::compute_match_key(&match_item);
        if match_item.snippet_syntax_highlights.is_none() && snippet_content_len > 0 {
            pending_syntax.push(SyntaxEnrichItem {
                key: match_item.key,
                row,
                snippet_len: snippet_content_len,
            });
        }
        matches.push(match_item);
    }

    Some(BuildMatchesOutput {
        matches,
        pending_syntax,
        buffer_id,
    })
}
