use crate::{
    Anchor, Editor, EditorSettings, EditorSnapshot, GotoDefinitionKind, HighlightKey,
    LspNavigationTarget, Navigated, NavigationSource, OpenResultsIn, PointForPosition, SelectPhase,
    scroll::ScrollAmount,
};
use gpui::{
    App, AsyncWindowContext, Context, Entity, HighlightStyle, Modifiers, Pixels, Task,
    UnderlineStyle, Window, px,
};
use language::{Bias, ToOffset};
use linkify::{LinkFinder, LinkKind};
use lsp::LanguageServerId;
use multi_buffer::ToOffset as _;
use project::{InlayId, LocationLink, Project, ResolvedPath};
use regex::Regex;
use settings::Settings;
use std::{
    ops::Range,
    str::FromStr as _,
    sync::{Arc, LazyLock},
};
use text::OffsetRangeExt;
use theme::ActiveTheme as _;
use util::{ResultExt, markdown::source_position_from_fragment, paths::PathWithPosition};

#[derive(Debug)]
pub struct HoveredLinkState {
    pub last_trigger_point: TriggerPoint,
    pub preferred_kind: GotoDefinitionKind,
    pub lsp_data_enabled: bool,
    pub symbol_range: Option<RangeInEditor>,
    pub links: Vec<HoverLink>,
    pub task: Option<Task<Option<()>>>,
}

impl HoveredLinkState {
    fn point_within_range(&self, trigger_point: &TriggerPoint, snapshot: &EditorSnapshot) -> bool {
        if !self.link_range_contains(trigger_point, snapshot) {
            return false;
        }
        if !self.lsp_data_enabled || !matches!(trigger_point, TriggerPoint::Text(_)) {
            return true;
        }
        let mut origins = self
            .links
            .iter()
            .filter_map(|link| match link {
                HoverLink::Text(link) => Some(link.origin.as_ref()),
                _ => None,
            })
            .peekable();
        if origins.peek().is_none() && trigger_point == &self.last_trigger_point {
            return true;
        }
        let fallback = origins.peek().is_none().then_some(None);
        origins.chain(fallback).all(|origin| {
            let buffer = snapshot.buffer_snapshot();
            let range = match origin {
                Some(origin) => buffer.buffer_anchor_range_to_anchor_range(origin.range.clone()),
                None => {
                    let (range, _) =
                        buffer.surrounding_word(*self.last_trigger_point.anchor(), None);
                    Some(buffer.anchor_before(range.start)..buffer.anchor_after(range.end))
                }
            };
            range.is_some_and(|range| {
                RangeInEditor::Text(range).contains_hover_point(trigger_point, snapshot)
            })
        })
    }

    fn link_range_contains(&self, trigger_point: &TriggerPoint, snapshot: &EditorSnapshot) -> bool {
        self.symbol_range.as_ref().is_some_and(|range| {
            if self
                .links
                .iter()
                .any(|link| !matches!(link, HoverLink::Text(_)))
            {
                range.point_within_range(trigger_point, snapshot)
            } else {
                range.contains_hover_point(trigger_point, snapshot)
            }
        })
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum RangeInEditor {
    Text(Range<Anchor>),
    Inlay(InlayHighlight),
}

impl RangeInEditor {
    pub fn as_text_range(&self) -> Option<Range<Anchor>> {
        match self {
            Self::Text(range) => Some(range.clone()),
            Self::Inlay(_) => None,
        }
    }

    pub fn point_within_range(
        &self,
        trigger_point: &TriggerPoint,
        snapshot: &EditorSnapshot,
    ) -> bool {
        match (self, trigger_point) {
            (Self::Text(range), TriggerPoint::Text(point)) => {
                let buffer_snapshot = snapshot.buffer_snapshot();
                if !range.start.is_valid(&buffer_snapshot)
                    || !range.end.is_valid(&buffer_snapshot)
                    || !point.is_valid(&buffer_snapshot)
                {
                    return false;
                }
                let point_after_start = range.start.cmp(point, &buffer_snapshot).is_le();
                let point_after_end = range.end.cmp(point, &buffer_snapshot).is_ge();
                point_after_start && point_after_end
            }
            (Self::Inlay(highlight), TriggerPoint::InlayHint(point, _, _)) => {
                highlight.inlay == point.inlay
                    && highlight.range.contains(&point.range.start)
                    && highlight.range.contains(&point.range.end)
            }
            (Self::Inlay(_), TriggerPoint::Text(_))
            | (Self::Text(_), TriggerPoint::InlayHint(_, _, _)) => false,
        }
    }

    fn contains_hover_point(
        &self,
        trigger_point: &TriggerPoint,
        snapshot: &EditorSnapshot,
    ) -> bool {
        if let (Self::Text(range), TriggerPoint::Text(point)) = (self, trigger_point) {
            let buffer = snapshot.buffer_snapshot();
            range.start.is_valid(buffer)
                && range.end.is_valid(buffer)
                && point.is_valid(buffer)
                && (range.start.to_offset(buffer)..range.end.to_offset(buffer))
                    .contains(&point.to_offset(buffer))
        } else {
            self.point_within_range(trigger_point, snapshot)
        }
    }
}

#[derive(Debug, Clone)]
pub enum HoverLink {
    Url(String),
    LspUrl(String),
    File(ResolvedFileTarget),
    Text(LocationLink),
    /// Navigate to an LSP-given location whose buffer may not be loaded yet.
    /// Used by inlay-hint hover, code-lens references, and document-link
    /// targets that point inside a workspace file (e.g. `file:///foo#9,16`).
    LspLocation(lsp::Location, LanguageServerId),
}

/// Convert a `documentLink` target URI into a [`HoverLink`], reusing the
/// existing navigation paths: `file://` URIs go through the LSP location
/// pipeline (so an optional `#line[,column]` fragment is honored), while
/// any other scheme is opened as a regular URL.
pub fn document_link_target_to_hover_link(target: &str, server_id: LanguageServerId) -> HoverLink {
    if let Ok(url) = url::Url::parse(target)
        && url.scheme() == "file"
        && let Ok(uri) = lsp::Uri::from_str(target)
    {
        let position = url
            .fragment()
            .and_then(source_position_from_fragment)
            .map(|(line, character)| lsp::Position { line, character })
            .unwrap_or_default();
        return HoverLink::LspLocation(
            lsp::Location {
                uri,
                range: lsp::Range::new(position, position),
            },
            server_id,
        );
    }
    HoverLink::LspUrl(target.to_string())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlayHighlight {
    pub inlay: InlayId,
    pub inlay_position: Anchor,
    pub range: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TriggerPoint {
    Text(Anchor),
    InlayHint(InlayHighlight, lsp::Location, LanguageServerId),
}

impl TriggerPoint {
    fn anchor(&self) -> &Anchor {
        match self {
            TriggerPoint::Text(anchor) => anchor,
            TriggerPoint::InlayHint(inlay_range, _, _) => &inlay_range.inlay_position,
        }
    }
}

pub fn exclude_link_to_position(
    buffer: &Entity<language::Buffer>,
    current_position: &text::Anchor,
    location: &LocationLink,
    cx: &App,
) -> bool {
    // Exclude definition links that points back to cursor position.
    // (i.e., currently cursor upon definition).
    let snapshot = buffer.read(cx).snapshot();
    !(buffer == &location.target.buffer
        && current_position
            .bias_right(&snapshot)
            .cmp(&location.target.range.start, &snapshot)
            .is_ge()
        && current_position
            .cmp(&location.target.range.end, &snapshot)
            .is_le())
}

impl Editor {
    pub(crate) fn update_hovered_link(
        &mut self,
        point_for_position: PointForPosition,
        mouse_position: Option<gpui::Point<Pixels>>,
        snapshot: &EditorSnapshot,
        modifiers: Modifiers,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let hovered_link_modifier = Editor::is_cmd_or_ctrl_pressed(&modifiers, cx);
        if !hovered_link_modifier || self.has_pending_selection() {
            self.hide_hovered_link(cx);
            return;
        }

        if !cx.is_cursor_visible() {
            self.hide_hovered_link(cx);
            return;
        }

        match point_for_position.as_valid() {
            Some(point) => {
                let trigger_point = TriggerPoint::Text(
                    snapshot
                        .buffer_snapshot()
                        .anchor_before(point.to_offset(&snapshot.display_snapshot, Bias::Left)),
                );

                show_link_definition(modifiers.shift, self, trigger_point, snapshot, window, cx);
            }
            None => {
                self.update_inlay_link_and_hover_points(
                    snapshot,
                    point_for_position,
                    mouse_position,
                    hovered_link_modifier,
                    modifiers.shift,
                    window,
                    cx,
                );
            }
        }
    }

    pub(crate) fn hide_hovered_link(&mut self, cx: &mut Context<Self>) {
        self.hovered_link_state.take();
        self.clear_highlights(HighlightKey::HoveredLinkState, cx);
    }

    pub(crate) fn handle_click_hovered_link(
        &mut self,
        point: PointForPosition,
        modifiers: Modifiers,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        if !self.lsp_data_enabled() {
            if let Some(state) = self.hovered_link_state.as_mut() {
                state
                    .links
                    .retain(|link| matches!(link, HoverLink::Url(_) | HoverLink::File(_)));
            }
            if self
                .hovered_link_state
                .as_ref()
                .is_none_or(|state| state.links.is_empty())
            {
                self.hide_hovered_link(cx);
                self.select(SelectPhase::End, window, cx);
                return;
            }
        }
        let task = self.cmd_click_reveal_task(point, modifiers, window, cx);
        self.run_navigation_task(task, cx);
    }

    pub fn scroll_hover(
        &mut self,
        amount: ScrollAmount,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let selection = self.selections.newest_anchor().head();
        let snapshot = self.snapshot(window, cx);

        if let Some(popover) = self.hover_state.info_popovers.iter().find(|popover| {
            popover
                .symbol_range
                .point_within_range(&TriggerPoint::Text(selection), &snapshot)
        }) {
            popover.scroll(amount, window, cx);
            true
        } else if let Some(context_menu) = self.context_menu.borrow_mut().as_mut() {
            context_menu.scroll_aside(amount, window, cx);
            true
        } else {
            false
        }
    }

    fn cmd_click_reveal_task(
        &mut self,
        point: PointForPosition,
        modifiers: Modifiers,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Task<anyhow::Result<Navigated>> {
        let split = Self::is_alt_pressed(&modifiers, cx);
        let kind = if modifiers.shift {
            GotoDefinitionKind::Type
        } else {
            GotoDefinitionKind::Symbol
        };
        let hovered_link_state = self.hovered_link_state.take();
        if hovered_link_state.is_some() {
            self.hide_hovered_link(cx);
        }
        let cached_links = hovered_link_state.and_then(|mut state| {
            let mut refresh_definitions = self.lsp_data_enabled()
                && matches!(state.last_trigger_point, TriggerPoint::Text(_))
                && (!state.lsp_data_enabled
                    || state.preferred_kind != kind
                    || state.task.as_ref().is_some_and(|task| !task.is_ready()));
            if matches!(state.last_trigger_point, TriggerPoint::Text(_)) {
                let snapshot = self.snapshot(window, cx);
                let point = point.as_valid()?;
                let trigger_point = TriggerPoint::Text(
                    snapshot
                        .buffer_snapshot()
                        .anchor_before(point.to_offset(&snapshot.display_snapshot, Bias::Left)),
                );
                refresh_definitions |=
                    self.lsp_data_enabled() && !state.point_within_range(&trigger_point, &snapshot);
                if refresh_definitions {
                    state
                        .links
                        .retain(|link| !matches!(link, HoverLink::Text(_)));
                }
                if !state.link_range_contains(&trigger_point, &snapshot) {
                    return None;
                }
            }
            (!state.links.is_empty()).then_some((state, refresh_definitions))
        });
        let snapshot = self.snapshot(window, cx);
        let position = snapshot
            .buffer_snapshot()
            .anchor_before(point.next_valid.to_point(&snapshot.display_snapshot));
        let origin = self.navigation_entry(position, cx);

        let source = self.start_navigation(position, origin, cx);
        let navigate_task = if let Some((hovered_link_state, refresh_definitions)) = cached_links {
            if !self.focus_handle.is_focused(window) {
                window.focus(&self.focus_handle, cx);
            }

            // exclude links pointing back to the current anchor
            let current_position = point
                .next_valid
                .to_point(&self.snapshot(window, cx).display_snapshot);
            let mut links = self
                .buffer()
                .read(cx)
                .text_anchor_for_position(current_position, cx)
                .map(|(buffer, anchor)| {
                    hovered_link_state
                        .links
                        .into_iter()
                        .filter(|link| {
                            if let HoverLink::Text(location) = link {
                                exclude_link_to_position(&buffer, &anchor, location, cx)
                            } else {
                                true
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            if refresh_definitions {
                self.select(
                    SelectPhase::Begin {
                        position: point.next_valid,
                        add: false,
                        click_count: 1,
                    },
                    window,
                    cx,
                );
                let workspace = self
                    .workspace
                    .as_ref()
                    .map(|(workspace, _)| workspace.clone());
                let definitions = self.definition_locations_of_kind_at(kind, position, cx);
                let source = source.clone();
                cx.spawn_in(window, async move |editor, cx| {
                    if let Some(definitions) = definitions {
                        links.extend(
                            definitions
                                .await
                                .log_err()
                                .unwrap_or_default()
                                .into_iter()
                                .map(|target| {
                                    HoverLink::Text(LocationLink {
                                        origin: None,
                                        target,
                                    })
                                }),
                        );
                    }
                    let Ok(Some(navigation)) = editor.update_in(cx, |editor, window, cx| {
                        if !source.is_current_in_workspace(editor, workspace.as_ref(), cx) {
                            return None;
                        }
                        Some(
                            editor
                                .navigate_to_clicked_links(kind, links, split, source, window, cx),
                        )
                    }) else {
                        return Ok(Navigated::No);
                    };
                    navigation.await
                })
            } else {
                self.navigate_to_clicked_links(kind, links, split, source.clone(), window, cx)
            }
        } else {
            // We don't have the correct kind of link cached, set the selection on
            // click and immediately trigger GoToDefinition.
            self.select(
                SelectPhase::Begin {
                    position: point.next_valid,
                    add: false,
                    click_count: 1,
                },
                window,
                cx,
            );
            if point.as_valid().is_none() {
                Task::ready(Ok(Navigated::No))
            } else if !split
                && self.lsp_data_enabled()
                && EditorSettings::get_global(cx).lsp_results_location == OpenResultsIn::Picker
                && self.dispatch_lsp_navigation_for_source(
                    LspNavigationTarget::ClickedDefinition {
                        kind,
                        locations: None,
                    },
                    source.clone(),
                    window,
                    cx,
                )
            {
                Task::ready(Ok(Navigated::Yes))
            } else {
                self.navigate_to_definition_locations(kind, None, source.clone(), split, window, cx)
            }
        };
        self.select(SelectPhase::End, window, cx);
        self.with_definition_fallback(navigate_task, source, true, window, cx)
    }

    fn navigate_to_clicked_links(
        &mut self,
        kind: GotoDefinitionKind,
        links: Vec<HoverLink>,
        split: bool,
        source: Arc<NavigationSource>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<Navigated>> {
        if !source.is_current(self, cx) {
            return Task::ready(Ok(Navigated::No));
        }
        if !split
            && self.lsp_data_enabled()
            && links.iter().all(|link| matches!(link, HoverLink::Text(_)))
            && EditorSettings::get_global(cx).lsp_results_location == OpenResultsIn::Picker
        {
            let locations = links
                .iter()
                .filter_map(|link| match link {
                    HoverLink::Text(link) => Some(link.target.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>();
            if self.dispatch_lsp_navigation_for_source(
                LspNavigationTarget::ClickedDefinition {
                    kind,
                    locations: Some(locations),
                },
                source.clone(),
                window,
                cx,
            ) {
                return Task::ready(Ok(Navigated::Yes));
            }
        }
        self.navigate_to_hover_links_for_source(Some(kind), links, source, split, window, cx)
    }
}

pub fn show_link_definition(
    shift_held: bool,
    editor: &mut Editor,
    trigger_point: TriggerPoint,
    snapshot: &EditorSnapshot,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    let preferred_kind = match trigger_point {
        TriggerPoint::Text(_) if !shift_held => GotoDefinitionKind::Symbol,
        _ => GotoDefinitionKind::Type,
    };

    let mut lsp_data_enabled = editor.lsp_data_enabled();

    let (mut hovered_link_state, is_cached) =
        if let Some(existing) = editor.hovered_link_state.take() {
            (existing, true)
        } else {
            (
                HoveredLinkState {
                    last_trigger_point: trigger_point.clone(),
                    symbol_range: None,
                    preferred_kind,
                    lsp_data_enabled,
                    links: Vec::new(),
                    task: None,
                },
                false,
            )
        };

    if editor.pending_rename.is_some() {
        return;
    }

    let anchor = trigger_point.anchor().bias_left(snapshot.buffer_snapshot());
    let Some((anchor, _)) = snapshot.buffer_snapshot().anchor_to_buffer_anchor(anchor) else {
        return;
    };
    let Some(buffer) = editor.buffer.read(cx).buffer(anchor.buffer_id) else {
        return;
    };
    let same_kind = hovered_link_state.preferred_kind == preferred_kind
        && hovered_link_state.lsp_data_enabled == lsp_data_enabled;

    if same_kind {
        if is_cached && (hovered_link_state.last_trigger_point == trigger_point)
            || hovered_link_state.point_within_range(&trigger_point, snapshot)
        {
            editor.hovered_link_state = Some(hovered_link_state);
            return;
        }
    }
    editor.hide_hovered_link(cx);
    let project = editor.project.clone();
    let provider = editor.semantics_provider.clone();

    // Record the requested position so a mouse move on the same point short-circuits
    // instead of re-querying, even when the server returns no `originSelectionRange`
    // (which would otherwise leave `symbol_range` empty).
    hovered_link_state.links.retain(|link| match link {
        HoverLink::Text(_) => false,
        HoverLink::LspUrl(_) | HoverLink::LspLocation(_, _) => lsp_data_enabled,
        HoverLink::Url(_) | HoverLink::File(_) => true,
    });
    if !hovered_link_state.link_range_contains(&trigger_point, snapshot) {
        hovered_link_state.links.clear();
    }
    if hovered_link_state.links.is_empty() {
        hovered_link_state.symbol_range = None;
    }
    hovered_link_state.last_trigger_point = trigger_point.clone();
    hovered_link_state.preferred_kind = preferred_kind;
    hovered_link_state.lsp_data_enabled = lsp_data_enabled;

    hovered_link_state.task = Some(cx.spawn_in(window, async move |editor, cx| {
        // LSP document links take priority: the server explicitly
        // declares which ranges are clickable, so they are more
        // accurate than the heuristic-based URL/file detection.
        //
        // Resolution is deduplicated by `LspStore`; awaiting here only
        // blocks until either the cached resolved entry is returned or
        // the in-flight `Shared` task completes.
        let resolved_document_links = editor
            .update(cx, |editor, cx| {
                lsp_data_enabled &= editor.lsp_data_enabled();
                if lsp_data_enabled {
                    editor.document_links_at(buffer.clone(), anchor, cx)
                } else {
                    None
                }
            })
            .ok()?;
        let mut resolved_document_links = match resolved_document_links {
            Some(task) => task.await,
            None => Vec::new(),
        };
        // Always also collect LSP definitions so that cmd-click
        // reveals every applicable target (e.g. a position that
        // carries both a document link and a definition).
        let mut definition_result = if matches!(&trigger_point, TriggerPoint::Text(_))
            && let Some(provider) = provider
        {
            let task = editor
                .update(cx, |editor, cx| {
                    lsp_data_enabled &= editor.lsp_data_enabled();
                    if lsp_data_enabled {
                        provider.definitions(&buffer, anchor, preferred_kind, cx)
                    } else {
                        None
                    }
                })
                .ok()?;
            match task {
                Some(task) => task.await.log_err().flatten(),
                None => None,
            }
        } else {
            None
        };
        lsp_data_enabled &= editor
            .read_with(cx, |editor, _| editor.lsp_data_enabled())
            .ok()?;
        if !lsp_data_enabled {
            resolved_document_links.clear();
            definition_result = None;
        }
        let snapshot = editor
            .read_with(cx, |editor, cx| editor.buffer.read(cx).snapshot(cx))
            .ok()?;
        let detected_document_link =
            resolved_document_links
                .into_iter()
                .find_map(|(server_id, link)| {
                    let multi_buffer_range =
                        snapshot.buffer_anchor_range_to_anchor_range(link.range.clone())?;
                    Some((link.range, multi_buffer_range, link.target, server_id))
                });
        drop(snapshot);

        let mut result = match &trigger_point {
            TriggerPoint::Text(_) => {
                let mut links = Vec::new();
                let mut symbol_range = None;

                // LSP-provided document link wins over heuristic URL/file
                // detection at the same position: the server tells us the
                // exact range and target, while `find_url`/`find_file` are
                // best-effort text matches.
                if let Some((_, multi_buffer_range, Some(target), server_id)) =
                    detected_document_link.clone()
                {
                    symbol_range = Some(RangeInEditor::Text(multi_buffer_range));
                    links.push(document_link_target_to_hover_link(&target, server_id));
                } else if let Some((url_range, url)) = find_url(&buffer, anchor, cx) {
                    let snapshot = editor
                        .read_with(cx, |editor, cx| editor.buffer.read(cx).snapshot(cx))
                        .ok()?;
                    if let Some(range) = snapshot.buffer_anchor_range_to_anchor_range(url_range) {
                        symbol_range = Some(RangeInEditor::Text(range));
                    }
                    links.push(HoverLink::Url(url));
                } else if let Some((filename_range, file_target)) =
                    find_file(&buffer, project.clone(), anchor, cx).await
                {
                    let snapshot = editor
                        .read_with(cx, |editor, cx| editor.buffer.read(cx).snapshot(cx))
                        .ok()?;
                    if let Some(range) =
                        snapshot.buffer_anchor_range_to_anchor_range(filename_range)
                    {
                        symbol_range = Some(RangeInEditor::Text(range));
                    }
                    links.push(HoverLink::File(file_target));
                }

                if let Some(definition_result) = definition_result {
                    if symbol_range.is_none() {
                        let snapshot = editor
                            .read_with(cx, |editor, cx| editor.buffer.read(cx).snapshot(cx))
                            .ok()?;
                        symbol_range = definition_result.iter().find_map(|link| {
                            link.origin.as_ref().and_then(|origin| {
                                let range = snapshot
                                    .buffer_anchor_range_to_anchor_range(origin.range.clone())?;
                                Some(RangeInEditor::Text(range))
                            })
                        });
                    }
                    links.extend(definition_result.into_iter().map(HoverLink::Text));
                }

                if links.is_empty() {
                    None
                } else {
                    Some((symbol_range, links))
                }
            }
            TriggerPoint::InlayHint(highlight, lsp_location, server_id) => Some((
                Some(RangeInEditor::Inlay(highlight.clone())),
                vec![HoverLink::LspLocation(lsp_location.clone(), *server_id)],
            )),
        };

        editor
            .update(cx, |editor, cx| {
                let enabled = editor.lsp_data_enabled();
                if !enabled {
                    if let Some((_, links)) = result.as_mut() {
                        links.retain(|link| matches!(link, HoverLink::Url(_) | HoverLink::File(_)));
                    }
                    if result.as_ref().is_none_or(|(_, links)| links.is_empty()) {
                        if editor
                            .hovered_link_state
                            .as_ref()
                            .is_some_and(|state| state.lsp_data_enabled)
                        {
                            editor.hide_hovered_link(cx);
                            return;
                        }
                        result = None;
                    }
                }
                // Clear any existing highlights
                editor.clear_highlights(HighlightKey::HoveredLinkState, cx);
                let Some(hovered_link_state) = editor.hovered_link_state.as_mut() else {
                    editor.hide_hovered_link(cx);
                    return;
                };
                hovered_link_state.preferred_kind = preferred_kind;
                hovered_link_state.lsp_data_enabled = lsp_data_enabled && enabled;
                hovered_link_state.symbol_range = result
                    .as_ref()
                    .and_then(|(symbol_range, _)| symbol_range.clone())
                    .or_else(|| {
                        // Even if we have no click target yet (e.g. an
                        // unresolved document link), record the link's range
                        // so subsequent mouse moves on the same link
                        // short-circuit in `show_link_definition`.
                        detected_document_link
                            .as_ref()
                            .map(|(_, multi_buffer_range, _, _)| {
                                RangeInEditor::Text(multi_buffer_range.clone())
                            })
                    });

                if let Some((symbol_range, definitions)) = result {
                    hovered_link_state.links = definitions;

                    let underline_hovered_link = !hovered_link_state.links.is_empty()
                        || hovered_link_state.symbol_range.is_some();

                    if underline_hovered_link {
                        let style = HighlightStyle {
                            underline: Some(UnderlineStyle {
                                thickness: px(1.),
                                ..UnderlineStyle::default()
                            }),
                            color: Some(cx.theme().colors().link_text_hover),
                            ..HighlightStyle::default()
                        };
                        let highlight_range =
                            symbol_range.unwrap_or_else(|| match &trigger_point {
                                TriggerPoint::Text(trigger_anchor) => {
                                    let snapshot = editor.buffer.read(cx).snapshot(cx);
                                    // If no symbol range returned from language server, use the surrounding word.
                                    let (offset_range, _) =
                                        snapshot.surrounding_word(*trigger_anchor, None);
                                    RangeInEditor::Text(
                                        snapshot.anchor_before(offset_range.start)
                                            ..snapshot.anchor_after(offset_range.end),
                                    )
                                }
                                TriggerPoint::InlayHint(highlight, _, _) => {
                                    RangeInEditor::Inlay(highlight.clone())
                                }
                            });

                        // When the server reports no `originSelectionRange`, fall back
                        // to the highlighted word as the symbol range so that hovering
                        // elsewhere within the same symbol reuses this result instead
                        // of issuing another request.
                        if let Some(hovered_link_state) = editor.hovered_link_state.as_mut()
                            && hovered_link_state.symbol_range.is_none()
                        {
                            hovered_link_state.symbol_range = Some(highlight_range.clone());
                        }

                        match highlight_range {
                            RangeInEditor::Text(text_range) => editor.highlight_text(
                                HighlightKey::HoveredLinkState,
                                vec![text_range],
                                style,
                                cx,
                            ),
                            RangeInEditor::Inlay(highlight) => editor.highlight_inlays(
                                HighlightKey::HoveredLinkState,
                                vec![highlight],
                                style,
                                cx,
                            ),
                        }
                    }
                } else if let Some((_, multi_buffer_range, _, _)) = detected_document_link.as_ref()
                {
                    let style = HighlightStyle {
                        underline: Some(UnderlineStyle {
                            thickness: px(1.),
                            ..UnderlineStyle::default()
                        }),
                        color: Some(cx.theme().colors().link_text_hover),
                        ..HighlightStyle::default()
                    };
                    editor.highlight_text(
                        HighlightKey::HoveredLinkState,
                        vec![multi_buffer_range.clone()],
                        style,
                        cx,
                    );
                } else {
                    // When no links are found, we don't want to completely
                    // throw away the `HoveredLinkState`, we'll want to at least
                    // keep the `trigger_point` around in order to avoid sending
                    // multiple requests for the same point.
                    hovered_link_state.links.clear();
                }
            })
            .ok()?;

        Some(())
    }));

    editor.hovered_link_state = Some(hovered_link_state);
}

pub(crate) fn find_url(
    buffer: &Entity<language::Buffer>,
    position: text::Anchor,
    cx: &AsyncWindowContext,
) -> Option<(Range<text::Anchor>, String)> {
    const LIMIT: usize = 2048;

    let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());

    let offset = position.to_offset(&snapshot);
    let mut token_start = offset;
    let mut token_end = offset;
    let mut found_start = false;
    let mut found_end = false;

    for ch in snapshot.reversed_chars_at(offset).take(LIMIT) {
        if ch.is_whitespace() {
            found_start = true;
            break;
        }
        token_start -= ch.len_utf8();
    }
    // Check if we didn't find the starting whitespace or if we didn't reach the start of the buffer
    if !found_start && token_start != 0 {
        return None;
    }

    for ch in snapshot
        .chars_at(offset)
        .take(LIMIT - (offset - token_start))
    {
        if ch.is_whitespace() {
            found_end = true;
            break;
        }
        token_end += ch.len_utf8();
    }
    // Check if we didn't find the ending whitespace or if we read more or equal than LIMIT
    // which at this point would happen only if we reached the end of buffer
    if !found_end && (token_end - token_start >= LIMIT) {
        return None;
    }

    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::Url]);
    let input = snapshot
        .text_for_range(token_start..token_end)
        .collect::<String>();

    let relative_offset = offset - token_start;
    for link in finder.links(&input) {
        if link.start() <= relative_offset && link.end() >= relative_offset {
            let range = snapshot.anchor_before(token_start + link.start())
                ..snapshot.anchor_after(token_start + link.end());
            return Some((range, link.as_str().to_string()));
        }
    }
    None
}

pub(crate) fn find_url_from_range(
    buffer: &Entity<language::Buffer>,
    range: Range<text::Anchor>,
    cx: &AsyncWindowContext,
) -> Option<String> {
    const LIMIT: usize = 2048;

    let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());

    let start_offset = range.start.to_offset(&snapshot);
    let end_offset = range.end.to_offset(&snapshot);

    let mut token_start = start_offset.min(end_offset);
    let mut token_end = start_offset.max(end_offset);

    let range_len = token_end - token_start;

    if range_len >= LIMIT {
        return None;
    }

    // Skip leading whitespace
    for ch in snapshot.chars_at(token_start).take(range_len) {
        if !ch.is_whitespace() {
            break;
        }
        token_start += ch.len_utf8();
    }

    // Skip trailing whitespace
    for ch in snapshot.reversed_chars_at(token_end).take(range_len) {
        if !ch.is_whitespace() {
            break;
        }
        token_end -= ch.len_utf8();
    }

    if token_start >= token_end {
        return None;
    }

    let text = snapshot
        .text_for_range(token_start..token_end)
        .collect::<String>();

    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::Url]);

    if let Some(link) = finder.links(&text).next()
        && link.start() == 0
        && link.end() == text.len()
    {
        return Some(link.as_str().to_string());
    }

    None
}

#[derive(Debug, Clone)]
pub struct ResolvedFileTarget {
    pub resolved_path: ResolvedPath,
    pub row: Option<u32>,
    pub column: Option<u32>,
}

impl ResolvedFileTarget {
    /// After opening a file, navigate the editor to the row/column position if present.
    pub fn navigate_item_to_position(
        &self,
        item: Box<dyn crate::ItemHandle>,
        cx: &mut AsyncWindowContext,
    ) {
        if let Some(row) = self.row {
            let col = self.column.unwrap_or(0);
            if let Some(active_editor) = item.downcast::<crate::Editor>() {
                active_editor
                    .downgrade()
                    .update_in(cx, |editor, window, cx| {
                        let row = row.saturating_sub(1);
                        let col = col.saturating_sub(1);
                        let Some(buffer) = editor.buffer().read(cx).as_singleton() else {
                            return;
                        };
                        let point = buffer
                            .read(cx)
                            .snapshot()
                            .point_from_external_input(row, col);
                        editor.go_to_singleton_buffer_point_silently(point, window, cx);
                    })
                    .log_err();
            }
        }
    }
}

pub(crate) async fn find_file(
    buffer: &Entity<language::Buffer>,
    project: Option<Entity<Project>>,
    position: text::Anchor,
    cx: &mut AsyncWindowContext,
) -> Option<(Range<text::Anchor>, ResolvedFileTarget)> {
    let project = project?;
    let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());
    let scope = snapshot.language_scope_at(position);
    let (range, candidate_file_path) = surrounding_filename(&snapshot, position)?;
    let candidate_len = candidate_file_path.len();

    async fn check_path(
        candidate_file_path: &str,
        project: &Entity<Project>,
        buffer: &Entity<language::Buffer>,
        cx: &mut AsyncWindowContext,
    ) -> Option<ResolvedPath> {
        project
            .update(cx, |project, cx| {
                project.resolve_path_in_buffer(candidate_file_path, buffer, cx)
            })
            .await
            .filter(|s| s.is_file())
    }

    let pattern_candidates = link_pattern_file_candidates(&candidate_file_path);

    // Compute the highlight range for a pattern_range within the candidate string.
    let make_range = |pattern_range: &Range<usize>| -> Range<text::Anchor> {
        let offset_range = range.to_offset(&snapshot);
        let actual_start = offset_range.start + pattern_range.start;
        let actual_end = offset_range.end - (candidate_len - pattern_range.end);
        snapshot.anchor_before(actual_start)..snapshot.anchor_after(actual_end)
    };

    // For each candidate extracted by link_pattern_file_candidates, try resolving in order:
    // 1. The raw candidate string
    // 2. The path portion after stripping `:row:col` suffix
    // 3. With language-specific file extensions appended to raw candidate
    // 4. With language-specific file extensions appended to stripped path
    for (pattern_candidate, pattern_range) in &pattern_candidates {
        // Try the raw candidate first.
        if let Some(existing_path) = check_path(&pattern_candidate, &project, buffer, cx).await {
            return Some((
                make_range(pattern_range),
                ResolvedFileTarget {
                    resolved_path: existing_path,
                    row: None,
                    column: None,
                },
            ));
        }

        // Parse row:col suffix once per candidate for use in fallback attempts.
        // This handles patterns like `file.rs:83:1`, `file.rs:83`, and `file.rs:20:in`.
        let parsed = PathWithPosition::parse_str(pattern_candidate);
        let parsed_path = parsed.path.to_string_lossy();

        // Try resolving just the path portion (without :row:col).
        if parsed.row.is_some() {
            if let Some(existing_path) = check_path(&parsed_path, &project, buffer, cx).await {
                return Some((
                    make_range(pattern_range),
                    ResolvedFileTarget {
                        resolved_path: existing_path,
                        row: parsed.row,
                        column: parsed.column,
                    },
                ));
            }
        }

        // Try with language-specific suffixes.
        if let Some(scope) = &scope {
            for suffix in scope.path_suffixes() {
                if pattern_candidate.ends_with(format!(".{suffix}").as_str()) {
                    continue;
                }

                let suffixed_candidate = format!("{pattern_candidate}.{suffix}");
                if let Some(existing_path) =
                    check_path(&suffixed_candidate, &project, buffer, cx).await
                {
                    return Some((
                        make_range(pattern_range),
                        ResolvedFileTarget {
                            resolved_path: existing_path,
                            row: None,
                            column: None,
                        },
                    ));
                }
            }

            // Try with language-specific suffixes on the stripped path.
            if parsed.row.is_some() {
                for suffix in scope.path_suffixes() {
                    if parsed_path.ends_with(&format!(".{suffix}")) {
                        continue;
                    }

                    let suffixed_candidate = format!("{parsed_path}.{suffix}");
                    if let Some(existing_path) =
                        check_path(&suffixed_candidate, &project, buffer, cx).await
                    {
                        return Some((
                            make_range(pattern_range),
                            ResolvedFileTarget {
                                resolved_path: existing_path,
                                row: parsed.row,
                                column: parsed.column,
                            },
                        ));
                    }
                }
            }
        }
    }
    None
}

// Generates candidate file paths by stripping common punctuation wrappers.
// Handles markdown patterns like [title](path), `path`, (path), as well as
// partial wrappers where punctuation only appears on one side (e.g. path) or path`).
// Returns candidates ordered from most-specific (most trimmed) to least-specific (raw).
fn link_pattern_file_candidates(candidate: &str) -> Vec<(String, Range<usize>)> {
    static MD_LINK_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"]\(([^)]*)\)").expect("Failed to create REGEX"));

    // Punctuation that commonly wraps file paths in prose/markdown
    const LEADING_PUNCTUATION: &[char] = &['`', '(', '[', '{', '<', '"', '\''];
    const TRAILING_PUNCTUATION: &[char] = &[
        '`', ')', ']', '}', '>', '"', '\'', '.', ',', ':', ';', '!', '?',
    ];

    let candidate_len = candidate.len();
    let mut candidates = Vec::new();

    // Trim leading and trailing punctuation iteratively
    let mut start = 0;
    let mut end = candidate_len;

    // Trim leading punctuation
    for ch in candidate.chars() {
        if LEADING_PUNCTUATION.contains(&ch) {
            start += ch.len_utf8();
        } else {
            break;
        }
    }

    // Trim trailing punctuation
    for ch in candidate.chars().rev() {
        if TRAILING_PUNCTUATION.contains(&ch) {
            end -= ch.len_utf8();
        } else {
            break;
        }
    }

    // Add trimmed candidate first (highest priority) if it differs from original
    if start < end && (start > 0 || end < candidate_len) {
        candidates.push((candidate[start..end].to_string(), start..end));
    }

    // Extract markdown link destination: [title](path) or ](path) -> path
    // This also handles bare (path) wrapping.
    if let Some(captures) = MD_LINK_REGEX.captures(candidate) {
        if let Some(link) = captures.get(1) {
            let link_str = link.as_str().to_string();
            let link_range = link.range();
            // Avoid duplicate if punctuation trimming already found this
            if !candidates.iter().any(|(s, _)| s == &link_str) {
                candidates.push((link_str, link_range));
            }
        }
    }

    // Always include the raw candidate as fallback (lowest priority)
    candidates.push((candidate.to_string(), 0..candidate_len));

    candidates
}

fn surrounding_filename(
    snapshot: &language::BufferSnapshot,
    position: text::Anchor,
) -> Option<(Range<text::Anchor>, String)> {
    const LIMIT: usize = 2048;

    let offset = position.to_offset(&snapshot);
    let mut token_start = offset;
    let mut token_end = offset;
    let mut found_start = false;
    let mut found_end = false;
    let mut inside_quotes = false;

    let mut filename = String::new();

    let mut backwards = snapshot.reversed_chars_at(offset).take(LIMIT).peekable();
    while let Some(ch) = backwards.next() {
        // Escaped whitespace
        if ch.is_whitespace() && backwards.peek() == Some(&'\\') {
            filename.push(ch);
            token_start -= ch.len_utf8();
            backwards.next();
            token_start -= '\\'.len_utf8();
            continue;
        }
        if ch.is_whitespace() {
            found_start = true;
            break;
        }
        // Quote characters open a quoted region that is stripped from the
        // returned filename. Backticks and parens are NOT treated this way —
        // they are kept as part of the token so that downstream candidate
        // generation (link_pattern_file_candidates) can trim them and produce
        // a tight highlight range via make_range.
        if (ch == '"' || ch == '\'') && !inside_quotes {
            found_start = true;
            inside_quotes = true;
            break;
        }

        filename.push(ch);
        token_start -= ch.len_utf8();
    }
    if !found_start && token_start != 0 {
        return None;
    }

    filename = filename.chars().rev().collect();

    let mut forwards = snapshot
        .chars_at(offset)
        .take(LIMIT - (offset - token_start))
        .peekable();
    while let Some(ch) = forwards.next() {
        // Skip escaped whitespace
        if ch == '\\' && forwards.peek().is_some_and(|ch| ch.is_whitespace()) {
            token_end += ch.len_utf8();
            let whitespace = forwards.next().unwrap();
            token_end += whitespace.len_utf8();
            filename.push(whitespace);
            continue;
        }

        if ch.is_whitespace() {
            found_end = true;
            break;
        }
        if ch == '"' || ch == '\'' {
            // If we're inside quotes, we stop when we come across the next quote
            if inside_quotes {
                found_end = true;
                break;
            } else {
                // Otherwise, we skip the quote
                inside_quotes = true;
                token_end += ch.len_utf8();
                continue;
            }
        }
        filename.push(ch);
        token_end += ch.len_utf8();
    }

    if !found_end && (token_end - token_start >= LIMIT) {
        return None;
    }

    if filename.is_empty() {
        return None;
    }

    let range = snapshot.anchor_before(token_start)..snapshot.anchor_after(token_end);

    Some((range, filename))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        DisplayPoint, EditorMode, InlayHintRefreshReason, OpenLspLocations,
        display_map::ToDisplayPoint,
        editor_tests::{init_test, update_test_editor_settings},
        inlays::inlay_hints::tests::{cached_hint_labels, visible_hint_labels},
        test::editor_lsp_test_context::EditorLspTestContext,
    };
    use futures::{FutureExt as _, StreamExt, channel::oneshot};
    use gpui::{InteractiveElement as _, Modifiers, MousePressureEvent, PressureStage};
    use indoc::indoc;
    use itertools::Itertools as _;
    use language::Point;
    use lsp::request::{GotoDefinition, GotoTypeDefinition};
    use multi_buffer::{AnchorRangeExt as _, MultiBufferOffset, PathKey, ToPoint as _};
    use settings::InlayHintSettingsContent;
    use std::cell::RefCell;
    use std::path::PathBuf;
    use std::rc::Rc;
    use std::str::FromStr;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use util::{assert_set_eq, path};
    use workspace::item::Item;

    #[test]
    fn test_document_link_target_to_hover_link_file_uri_with_fragment() {
        let server_id = LanguageServerId(0);
        let target = "file:///Users/me/work/local_test/document-links-test.json#9,16";
        match document_link_target_to_hover_link(target, server_id) {
            HoverLink::LspLocation(location, returned_id) => {
                assert_eq!(returned_id, server_id);
                assert_eq!(
                    location.uri.as_str(),
                    "file:///Users/me/work/local_test/document-links-test.json#9,16",
                );
                assert_eq!(
                    location.range,
                    lsp::Range {
                        start: lsp::Position {
                            line: 8,
                            character: 15,
                        },
                        end: lsp::Position {
                            line: 8,
                            character: 15,
                        },
                    }
                );
            }
            other => panic!("expected LspLocation variant, got {other:?}"),
        }
    }

    #[test]
    fn test_document_link_target_to_hover_link_http_url() {
        let server_id = LanguageServerId(0);
        let target = "https://opensource.org/licenses/MIT";
        match document_link_target_to_hover_link(target, server_id) {
            HoverLink::LspUrl(url) => assert_eq!(url, target),
            other => panic!("expected LspUrl variant, got {other:?}"),
        }
    }

    #[gpui::test]
    async fn test_hover_type_links(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
                type_definition_provider: Some(lsp::TypeDefinitionProviderCapability::Simple(true)),
                ..Default::default()
            },
            cx,
        )
        .await;

        cx.set_state(indoc! {"
            struct A;
            let vˇariable = A;
        "});
        let screen_coord = cx.editor(|editor, _, cx| editor.pixel_position_of_cursor(cx));

        // Basic hold cmd+shift, expect highlight in region if response contains type definition
        let symbol_range = cx.lsp_range(indoc! {"
            struct A;
            let «variable» = A;
        "});
        let target_range = cx.lsp_range(indoc! {"
            struct «A»;
            let variable = A;
        "});

        cx.run_until_parked();

        let mut requests =
            cx.set_request_handler::<GotoTypeDefinition, _, _>(move |url, _, _| async move {
                Ok(Some(lsp::GotoTypeDefinitionResponse::Link(vec![
                    lsp::LocationLink {
                        origin_selection_range: Some(symbol_range),
                        target_uri: url.clone(),
                        target_range,
                        target_selection_range: target_range,
                    },
                ])))
            });

        let modifiers = if cfg!(target_os = "macos") {
            Modifiers::command_shift()
        } else {
            Modifiers::control_shift()
        };

        cx.simulate_mouse_move(screen_coord.unwrap(), None, modifiers);

        requests.next().await;
        cx.run_until_parked();
        cx.assert_editor_text_highlights(
            HighlightKey::HoveredLinkState,
            indoc! {"
            struct A;
            let «variable» = A;
        "},
        );

        cx.simulate_modifiers_change(Modifiers::secondary_key());
        cx.run_until_parked();
        // Assert no link highlights
        cx.assert_editor_text_highlights(
            HighlightKey::HoveredLinkState,
            indoc! {"
            struct A;
            let variable = A;
        "},
        );

        cx.simulate_click(screen_coord.unwrap(), modifiers);

        cx.assert_editor_state(indoc! {"
            struct «Aˇ»;
            let variable = A;
        "});
    }

    #[gpui::test]
    async fn test_hover_link_after_multibuffer_path_changes(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});

        let mut cx = EditorLspTestContext::new_rust(Default::default(), cx).await;
        cx.set_state("https://zed.dev/ˇreleases");
        let old_snapshot = cx.update_editor(|editor, window, cx| editor.snapshot(window, cx));
        let link_start = MultiBufferOffset(17).to_display_point(&old_snapshot.display_snapshot);
        let link_end = MultiBufferOffset(22).to_display_point(&old_snapshot.display_snapshot);
        let point_for_position = |point| PointForPosition {
            previous_valid: point,
            next_valid: point,
            nearest_valid: point,
            exact_unclipped: point,
            column_overshoot_after_line_end: 0,
        };

        let buffer = cx.editor(|editor, _, cx| {
            editor
                .buffer()
                .read(cx)
                .as_singleton()
                .expect("test editor should contain a singleton buffer")
        });
        cx.update_multibuffer(|multibuffer, cx| {
            let max_point = buffer.read(cx).max_point();
            multibuffer.set_excerpts_for_path(
                PathKey::sorted(1),
                buffer,
                [Point::zero()..max_point],
                0,
                cx,
            );
        });
        cx.run_until_parked();

        let modifiers = if cfg!(target_os = "macos") {
            Modifiers::command_shift()
        } else {
            Modifiers::control_shift()
        };
        cx.update_editor(|editor, window, cx| {
            editor.update_hovered_link(
                point_for_position(link_start),
                None,
                &old_snapshot,
                modifiers,
                window,
                cx,
            );
        });
        cx.run_until_parked();

        cx.update_editor(|editor, window, cx| {
            editor.update_hovered_link(
                point_for_position(link_end),
                None,
                &old_snapshot,
                modifiers,
                window,
                cx,
            );
        });
    }

    #[gpui::test]
    async fn test_go_to_definition_link_dedup(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
                definition_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            cx,
        )
        .await;

        cx.set_state(indoc! {"
            fn ˇtest() { do_work(); }
            fn do_work() { test(); }
        "});

        let request_count = Arc::new(AtomicUsize::new(0));
        let _requests = cx.set_request_handler::<GotoDefinition, _, _>({
            let request_count = request_count.clone();
            move |url, _, _| {
                request_count.fetch_add(1, Ordering::SeqCst);
                async move {
                    // Return a bare `Location`, not an `originSelectionRange`
                    // so we can confirm that jiggling the mouse within the same
                    // symbol range does not trigger a second request, even
                    // though `originSelectionRange` was not returned.
                    Ok(Some(lsp::GotoDefinitionResponse::Scalar(lsp::Location {
                        uri: url,
                        range: lsp::Range::default(),
                    })))
                }
            }
        });

        let symbol_start = cx.pixel_position(indoc! {"
            fn test() { ˇdo_work(); }
            fn do_work() { test(); }
        "});
        let symbol_end = cx.pixel_position(indoc! {"
            fn test() { do_worˇk(); }
            fn do_work() { test(); }
        "});
        let other_symbol = cx.pixel_position(indoc! {"
            fn test() { do_work(); }
            fn do_work() { teˇst(); }
        "});

        cx.simulate_mouse_move(symbol_start, None, Modifiers::secondary_key());
        cx.run_until_parked();

        cx.simulate_mouse_move(symbol_end, None, Modifiers::secondary_key());
        cx.run_until_parked();

        cx.simulate_mouse_move(other_symbol, None, Modifiers::secondary_key());
        cx.run_until_parked();

        assert_eq!(
            request_count.load(Ordering::SeqCst),
            2,
            "expected one request per symbol, reused within a symbol"
        );
    }

    #[gpui::test]
    async fn test_go_to_definition_link_dedup_no_link(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
                definition_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            cx,
        )
        .await;

        cx.set_state(indoc! {"
            fn ˇtest() { do_work(); }
            fn do_work() { test(); }
        "});

        let request_count = Arc::new(AtomicUsize::new(0));
        let _requests = cx.set_request_handler::<GotoDefinition, _, _>({
            let request_count = request_count.clone();

            move |_, _, _| {
                request_count.fetch_add(1, Ordering::SeqCst);

                // Simulate response from the language server, reporting
                // that no link was found.
                async move { Ok(None) }
            }
        });

        let first_point = cx.pixel_position(indoc! {"
            fn test() { do_wˇork(); }
            fn do_work() { test(); }
        "});
        let second_point = cx.pixel_position(indoc! {"
            fn test() { do_woˇrk(); }
            fn do_work() { test(); }
        "});

        cx.simulate_mouse_move(first_point, None, Modifiers::secondary_key());
        cx.run_until_parked();

        cx.simulate_mouse_move(second_point, None, Modifiers::secondary_key());
        cx.run_until_parked();

        // Jiggle within the same character should not produce a new request,
        // even though the previous response was empty and produced no link to
        // highlight.
        cx.simulate_mouse_move(second_point, None, Modifiers::secondary_key());
        cx.run_until_parked();

        assert_eq!(
            request_count.load(Ordering::SeqCst),
            2,
            "expected one definition request per distinct position"
        );
    }

    #[gpui::test]
    async fn test_hover_links(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
                definition_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            cx,
        )
        .await;

        cx.set_state(indoc! {"
                fn ˇtest() { do_work(); }
                fn do_work() { test(); }
            "});

        // Basic hold cmd, expect highlight in region if response contains definition
        let hover_point = cx.pixel_position(indoc! {"
                fn test() { do_wˇork(); }
                fn do_work() { test(); }
            "});
        let symbol_range = cx.lsp_range(indoc! {"
                fn test() { «do_work»(); }
                fn do_work() { test(); }
            "});
        let target_range = cx.lsp_range(indoc! {"
                fn test() { do_work(); }
                fn «do_work»() { test(); }
            "});

        let mut requests =
            cx.set_request_handler::<GotoDefinition, _, _>(move |url, _, _| async move {
                Ok(Some(lsp::GotoDefinitionResponse::Link(vec![
                    lsp::LocationLink {
                        origin_selection_range: Some(symbol_range),
                        target_uri: url.clone(),
                        target_range,
                        target_selection_range: target_range,
                    },
                ])))
            });

        cx.simulate_mouse_move(hover_point, None, Modifiers::secondary_key());
        requests.next().await;
        cx.background_executor.run_until_parked();
        cx.assert_editor_text_highlights(
            HighlightKey::HoveredLinkState,
            indoc! {"
                fn test() { «do_work»(); }
                fn do_work() { test(); }
            "},
        );

        // Unpress cmd causes highlight to go away
        cx.simulate_modifiers_change(Modifiers::none());
        cx.assert_editor_text_highlights(
            HighlightKey::HoveredLinkState,
            indoc! {"
                fn test() { do_work(); }
                fn do_work() { test(); }
            "},
        );

        let mut requests =
            cx.set_request_handler::<GotoDefinition, _, _>(move |url, _, _| async move {
                Ok(Some(lsp::GotoDefinitionResponse::Link(vec![
                    lsp::LocationLink {
                        origin_selection_range: Some(symbol_range),
                        target_uri: url.clone(),
                        target_range,
                        target_selection_range: target_range,
                    },
                ])))
            });

        cx.simulate_mouse_move(hover_point, None, Modifiers::secondary_key());
        requests.next().await;
        cx.background_executor.run_until_parked();
        cx.assert_editor_text_highlights(
            HighlightKey::HoveredLinkState,
            indoc! {"
                fn test() { «do_work»(); }
                fn do_work() { test(); }
            "},
        );

        // Moving mouse to location with no response dismisses highlight
        let hover_point = cx.pixel_position(indoc! {"
                fˇn test() { do_work(); }
                fn do_work() { test(); }
            "});
        let mut requests =
            cx.lsp
                .set_request_handler::<GotoDefinition, _, _>(move |_, _| async move {
                    // No definitions returned
                    Ok(Some(lsp::GotoDefinitionResponse::Link(vec![])))
                });
        cx.simulate_mouse_move(hover_point, None, Modifiers::secondary_key());

        requests.next().await;
        cx.background_executor.run_until_parked();

        // Assert no link highlights
        cx.assert_editor_text_highlights(
            HighlightKey::HoveredLinkState,
            indoc! {"
                fn test() { do_work(); }
                fn do_work() { test(); }
            "},
        );

        // // Move mouse without cmd and then pressing cmd triggers highlight
        let hover_point = cx.pixel_position(indoc! {"
                fn test() { do_work(); }
                fn do_work() { teˇst(); }
            "});
        cx.simulate_mouse_move(hover_point, None, Modifiers::none());

        // Assert no link highlights
        cx.assert_editor_text_highlights(
            HighlightKey::HoveredLinkState,
            indoc! {"
                fn test() { do_work(); }
                fn do_work() { test(); }
            "},
        );

        let symbol_range = cx.lsp_range(indoc! {"
                fn test() { do_work(); }
                fn do_work() { «test»(); }
            "});
        let target_range = cx.lsp_range(indoc! {"
                fn «test»() { do_work(); }
                fn do_work() { test(); }
            "});

        let mut requests =
            cx.set_request_handler::<GotoDefinition, _, _>(move |url, _, _| async move {
                Ok(Some(lsp::GotoDefinitionResponse::Link(vec![
                    lsp::LocationLink {
                        origin_selection_range: Some(symbol_range),
                        target_uri: url,
                        target_range,
                        target_selection_range: target_range,
                    },
                ])))
            });

        cx.simulate_modifiers_change(Modifiers::secondary_key());

        requests.next().await;
        cx.background_executor.run_until_parked();

        cx.assert_editor_text_highlights(
            HighlightKey::HoveredLinkState,
            indoc! {"
                fn test() { do_work(); }
                fn do_work() { «test»(); }
            "},
        );

        cx.deactivate_window();
        cx.assert_editor_text_highlights(
            HighlightKey::HoveredLinkState,
            indoc! {"
                fn test() { do_work(); }
                fn do_work() { test(); }
            "},
        );

        cx.simulate_mouse_move(hover_point, None, Modifiers::secondary_key());
        cx.background_executor.run_until_parked();
        cx.assert_editor_text_highlights(
            HighlightKey::HoveredLinkState,
            indoc! {"
                fn test() { do_work(); }
                fn do_work() { «test»(); }
            "},
        );

        // Moving again within the same symbol range doesn't re-request
        let hover_point = cx.pixel_position(indoc! {"
                fn test() { do_work(); }
                fn do_work() { tesˇt(); }
            "});
        cx.simulate_mouse_move(hover_point, None, Modifiers::secondary_key());
        cx.background_executor.run_until_parked();
        cx.assert_editor_text_highlights(
            HighlightKey::HoveredLinkState,
            indoc! {"
                fn test() { do_work(); }
                fn do_work() { «test»(); }
            "},
        );

        // Cmd click with existing definition doesn't re-request and dismisses highlight
        cx.simulate_click(hover_point, Modifiers::secondary_key());
        cx.lsp
            .set_request_handler::<GotoDefinition, _, _>(move |_, _| async move {
                // Empty definition response to make sure we aren't hitting the lsp and using
                // the cached location instead
                Ok(Some(lsp::GotoDefinitionResponse::Link(vec![])))
            });
        cx.background_executor.run_until_parked();
        cx.assert_editor_state(indoc! {"
                fn «testˇ»() { do_work(); }
                fn do_work() { test(); }
            "});

        // Assert no link highlights after jump
        cx.assert_editor_text_highlights(
            HighlightKey::HoveredLinkState,
            indoc! {"
                fn test() { do_work(); }
                fn do_work() { test(); }
            "},
        );

        // Cmd click without existing definition requests and jumps
        let hover_point = cx.pixel_position(indoc! {"
                fn test() { do_wˇork(); }
                fn do_work() { test(); }
            "});
        let target_range = cx.lsp_range(indoc! {"
                fn test() { do_work(); }
                fn «do_work»() { test(); }
            "});

        let mut requests =
            cx.set_request_handler::<GotoDefinition, _, _>(move |url, _, _| async move {
                Ok(Some(lsp::GotoDefinitionResponse::Link(vec![
                    lsp::LocationLink {
                        origin_selection_range: None,
                        target_uri: url,
                        target_range,
                        target_selection_range: target_range,
                    },
                ])))
            });
        cx.simulate_click(hover_point, Modifiers::secondary_key());
        requests.next().await;
        cx.background_executor.run_until_parked();
        cx.assert_editor_state(indoc! {"
                fn test() { do_work(); }
                fn «do_workˇ»() { test(); }
            "});

        // 1. We have a pending selection, mouse point is over a symbol that we have a response for, hitting cmd and nothing happens
        // 2. Selection is completed, hovering
        let hover_point = cx.pixel_position(indoc! {"
                fn test() { do_wˇork(); }
                fn do_work() { test(); }
            "});
        let target_range = cx.lsp_range(indoc! {"
                fn test() { do_work(); }
                fn «do_work»() { test(); }
            "});
        let mut requests =
            cx.set_request_handler::<GotoDefinition, _, _>(move |url, _, _| async move {
                Ok(Some(lsp::GotoDefinitionResponse::Link(vec![
                    lsp::LocationLink {
                        origin_selection_range: None,
                        target_uri: url,
                        target_range,
                        target_selection_range: target_range,
                    },
                ])))
            });

        // create a pending selection
        let selection_range = cx.ranges(indoc! {"
                fn «test() { do_w»ork(); }
                fn do_work() { test(); }
            "})[0]
            .clone();
        cx.update_editor(|editor, window, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let anchor_range = snapshot.anchor_before(MultiBufferOffset(selection_range.start))
                ..snapshot.anchor_after(MultiBufferOffset(selection_range.end));
            editor.change_selections(Default::default(), window, cx, |s| {
                s.set_pending_anchor_range(anchor_range, crate::SelectMode::Character)
            });
        });
        cx.simulate_mouse_move(hover_point, None, Modifiers::secondary_key());
        cx.background_executor.run_until_parked();
        assert!(requests.try_recv().is_err());
        cx.assert_editor_text_highlights(
            HighlightKey::HoveredLinkState,
            indoc! {"
                fn test() { do_work(); }
                fn do_work() { test(); }
            "},
        );
        cx.background_executor.run_until_parked();
    }

    #[gpui::test]
    async fn test_inlay_hover_links(cx: &mut gpui::TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                enabled: Some(true),
                show_value_hints: Some(false),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(true),
                show_parameter_hints: Some(true),
                show_other_hints: Some(true),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            cx,
        )
        .await;
        cx.set_state(indoc! {"
                struct TestStruct;

                fn main() {
                    let variableˇ = TestStruct;
                }
            "});
        let hint_start_offset = cx.ranges(indoc! {"
                struct TestStruct;

                fn main() {
                    let variableˇ = TestStruct;
                }
            "})[0]
            .start;
        let hint_position = cx.to_lsp(MultiBufferOffset(hint_start_offset));
        let target_range = cx.lsp_range(indoc! {"
                struct «TestStruct»;

                fn main() {
                    let variable = TestStruct;
                }
            "});

        let expected_uri = cx.buffer_lsp_url.clone();
        let hint_label = ": TestStruct";
        cx.lsp
            .set_request_handler::<lsp::request::InlayHintRequest, _, _>(move |params, _| {
                let expected_uri = expected_uri.clone();
                async move {
                    assert_eq!(params.text_document.uri, expected_uri);
                    Ok(Some(vec![lsp::InlayHint {
                        position: hint_position,
                        label: lsp::InlayHintLabel::LabelParts(vec![lsp::InlayHintLabelPart {
                            value: hint_label.to_string(),
                            location: Some(lsp::Location {
                                uri: params.text_document.uri,
                                range: target_range,
                            }),
                            ..Default::default()
                        }]),
                        kind: Some(lsp::InlayHintKind::TYPE),
                        text_edits: None,
                        tooltip: None,
                        padding_left: Some(false),
                        padding_right: Some(false),
                        data: None,
                    }]))
                }
            })
            .next()
            .await;
        cx.background_executor.run_until_parked();
        cx.update_editor(|editor, _window, cx| {
            let expected_layers = vec![hint_label.to_string()];
            assert_eq!(expected_layers, cached_hint_labels(editor, cx));
            assert_eq!(expected_layers, visible_hint_labels(editor, cx));
        });

        let inlay_range = cx
            .ranges(indoc! {"
                struct TestStruct;

                fn main() {
                    let variable« »= TestStruct;
                }
            "})
            .first()
            .cloned()
            .unwrap();
        let midpoint = cx.update_editor(|editor, window, cx| {
            let snapshot = editor.snapshot(window, cx);
            let previous_valid = MultiBufferOffset(inlay_range.start).to_display_point(&snapshot);
            let next_valid = MultiBufferOffset(inlay_range.end).to_display_point(&snapshot);
            assert_eq!(previous_valid.row(), next_valid.row());
            assert!(previous_valid.column() < next_valid.column());
            DisplayPoint::new(
                previous_valid.row(),
                previous_valid.column() + (hint_label.len() / 2) as u32,
            )
        });
        // Press cmd to trigger highlight
        let hover_point = cx.pixel_position_for(midpoint);
        cx.simulate_mouse_move(hover_point, None, Modifiers::secondary_key());
        cx.background_executor.run_until_parked();
        cx.update_editor(|editor, window, cx| {
            let snapshot = editor.snapshot(window, cx);
            let actual_highlights = snapshot
                .inlay_highlights(HighlightKey::HoveredLinkState)
                .into_iter()
                .flat_map(|highlights| highlights.values().map(|(_, highlight)| highlight))
                .collect::<Vec<_>>();

            let buffer_snapshot = editor.buffer().update(cx, |buffer, cx| buffer.snapshot(cx));
            let expected_highlight = InlayHighlight {
                inlay: InlayId::Hint(0),
                inlay_position: buffer_snapshot.anchor_after(MultiBufferOffset(inlay_range.start)),
                range: 0..hint_label.len(),
            };
            assert_set_eq!(actual_highlights, vec![&expected_highlight]);
        });

        cx.simulate_mouse_move(hover_point, None, Modifiers::none());
        // Assert no link highlights
        cx.update_editor(|editor, window, cx| {
                let snapshot = editor.snapshot(window, cx);
                let actual_ranges = snapshot
                    .text_highlight_ranges(HighlightKey::HoveredLinkState)
                    .map(|ranges| ranges.as_ref().clone().1)
                    .unwrap_or_default();

                assert!(actual_ranges.is_empty(), "When no cmd is pressed, should have no hint label selected, but got: {actual_ranges:?}");
            });

        cx.simulate_modifiers_change(Modifiers::secondary_key());
        cx.background_executor.run_until_parked();
        cx.simulate_click(hover_point, Modifiers::secondary_key());
        cx.background_executor.run_until_parked();
        cx.assert_editor_state(indoc! {"
                struct «TestStructˇ»;

                fn main() {
                    let variable = TestStruct;
                }
            "});
    }

    #[gpui::test]
    async fn test_urls(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                ..Default::default()
            },
            cx,
        )
        .await;

        cx.set_state(indoc! {"
            Let's test a [complex](https://zed.dev/channel/had-(oops)) caseˇ.
        "});

        let screen_coord = cx.pixel_position(indoc! {"
            Let's test a [complex](https://zed.dev/channel/had-(ˇoops)) case.
            "});

        cx.simulate_mouse_move(screen_coord, None, Modifiers::secondary_key());
        cx.assert_editor_text_highlights(
            HighlightKey::HoveredLinkState,
            indoc! {"
            Let's test a [complex](«https://zed.dev/channel/had-(oops)ˇ») case.
        "},
        );

        cx.simulate_click(screen_coord, Modifiers::secondary_key());
        assert_eq!(
            cx.opened_url(),
            Some("https://zed.dev/channel/had-(oops)".into())
        );
    }

    #[gpui::test]
    async fn test_hover_preconditions(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                ..Default::default()
            },
            cx,
        )
        .await;

        macro_rules! assert_no_highlight {
            ($cx:expr) => {
                // No highlight
                $cx.update_editor(|editor, window, cx| {
                    assert!(
                        editor
                            .snapshot(window, cx)
                            .text_highlight_ranges(HighlightKey::HoveredLinkState)
                            .unwrap_or_default()
                            .1
                            .is_empty()
                    );
                });
            };
        }

        // No link
        cx.set_state(indoc! {"
            Let's test a [complex](https://zed.dev/channel/) caseˇ.
        "});
        assert_no_highlight!(cx);

        // No modifier
        let screen_coord = cx.pixel_position(indoc! {"
            Let's test a [complex](https://zed.dev/channel/ˇ) case.
            "});
        cx.simulate_mouse_move(screen_coord, None, Modifiers::none());
        assert_no_highlight!(cx);

        // Modifier active
        let screen_coord = cx.pixel_position(indoc! {"
            Let's test a [complex](https://zed.dev/channeˇl/) case.
            "});
        cx.simulate_mouse_move(screen_coord, None, Modifiers::secondary_key());
        cx.assert_editor_text_highlights(
            HighlightKey::HoveredLinkState,
            indoc! {"
            Let's test a [complex](«https://zed.dev/channel/ˇ») case.
        "},
        );
    }

    #[gpui::test]
    async fn test_urls_at_beginning_of_buffer(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                ..Default::default()
            },
            cx,
        )
        .await;

        cx.set_state(indoc! {"https://zed.dev/releases is a cool ˇwebpage."});

        let screen_coord =
            cx.pixel_position(indoc! {"https://zed.dev/relˇeases is a cool webpage."});

        cx.simulate_mouse_move(screen_coord, None, Modifiers::secondary_key());
        cx.assert_editor_text_highlights(
            HighlightKey::HoveredLinkState,
            indoc! {"«https://zed.dev/releasesˇ» is a cool webpage."},
        );

        cx.simulate_click(screen_coord, Modifiers::secondary_key());
        assert_eq!(cx.opened_url(), Some("https://zed.dev/releases".into()));
    }

    #[gpui::test]
    async fn test_urls_at_end_of_buffer(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});
        for enable_lsp_data in [true, false] {
            for suffix in ["", " "] {
                for position in ["releˇases", "releaseˇs", "releasesˇ"] {
                    let mut cx =
                        EditorLspTestContext::new_rust(lsp::ServerCapabilities::default(), cx)
                            .await;
                    cx.set_state(&format!(
                        "A cool ˇwebpage is https://zed.dev/releases{suffix}"
                    ));
                    cx.update_editor(|editor, _, _| editor.enable_lsp_data = enable_lsp_data);
                    let point = Point::new(
                        0,
                        ("A cool webpage is https://zed.dev/".len()
                            + position.find('ˇ').expect("URL position"))
                            as u32,
                    );
                    show_test_link_definition(&mut cx, point, false);
                    cx.run_until_parked();
                    cx.assert_editor_text_highlights(
                        HighlightKey::HoveredLinkState,
                        &format!("A cool webpage is «https://zed.dev/releasesˇ»{suffix}"),
                    );
                    cx.update(|_, cx| cx.open_url("https://example.com/before-click"));
                    click_test_link(&mut cx, point, Modifiers::secondary_key());
                    cx.run_until_parked();
                    assert_eq!(
                        cx.opened_url().as_deref(),
                        Some("https://zed.dev/releases"),
                        "enable_lsp_data={enable_lsp_data}, suffix={suffix:?}, position={position}"
                    );
                }
            }
        }
    }

    #[test]
    fn test_link_pattern_file_candidates() {
        // Full markdown link: [LinkTitle](link_file.txt)
        // Trimmed strips [ and ), regex extracts link destination, raw is fallback
        let candidates: Vec<String> = link_pattern_file_candidates("[LinkTitle](link_file.txt)")
            .into_iter()
            .map(|(c, _)| c)
            .collect();
        assert_eq!(
            candidates,
            vec![
                "LinkTitle](link_file.txt",
                "link_file.txt",
                "[LinkTitle](link_file.txt)"
            ]
        );

        // Link title with spaces (token starts mid-link)
        let candidates: Vec<String> = link_pattern_file_candidates("LinkTitle](link_file.txt)")
            .into_iter()
            .map(|(c, _)| c)
            .collect();
        assert_eq!(
            candidates,
            vec![
                "LinkTitle](link_file.txt",
                "link_file.txt",
                "LinkTitle](link_file.txt)"
            ]
        );

        // Link with escaped spaces
        let candidates: Vec<String> = link_pattern_file_candidates("LinkTitle](link\\ _file.txt)")
            .into_iter()
            .map(|(c, _)| c)
            .collect();
        assert_eq!(
            candidates,
            vec![
                "LinkTitle](link\\ _file.txt",
                "link\\ _file.txt",
                "LinkTitle](link\\ _file.txt)"
            ]
        );

        // Bare parentheses: (link_file.txt)
        let candidates: Vec<String> = link_pattern_file_candidates("(link_file.txt)")
            .into_iter()
            .map(|(c, _)| c)
            .collect();
        assert_eq!(candidates, vec!["link_file.txt", "(link_file.txt)"]);

        // Trailing paren only: link_file.txt)
        let candidates: Vec<String> = link_pattern_file_candidates("link_file.txt)")
            .into_iter()
            .map(|(c, _)| c)
            .collect();
        assert_eq!(candidates, vec!["link_file.txt", "link_file.txt)"]);

        // Trailing backtick only: link_file.txt`
        let candidates: Vec<String> = link_pattern_file_candidates("link_file.txt`")
            .into_iter()
            .map(|(c, _)| c)
            .collect();
        assert_eq!(candidates, vec!["link_file.txt", "link_file.txt`"]);

        // Wrapped in backticks: `link_file.txt`
        let candidates: Vec<String> = link_pattern_file_candidates("`link_file.txt`")
            .into_iter()
            .map(|(c, _)| c)
            .collect();
        assert_eq!(candidates, vec!["link_file.txt", "`link_file.txt`"]);

        // Trailing period (sentence ending): link_file.txt.
        let candidates: Vec<String> = link_pattern_file_candidates("link_file.txt.")
            .into_iter()
            .map(|(c, _)| c)
            .collect();
        assert_eq!(candidates, vec!["link_file.txt", "link_file.txt."]);

        // Nested parens - regex finds first (...) capturing inner content
        let candidates: Vec<String> =
            link_pattern_file_candidates("LinkTitle](link_(link_file)file.txt)")
                .into_iter()
                .map(|(c, _)| c)
                .collect();
        assert_eq!(
            candidates,
            vec![
                "LinkTitle](link_(link_file)file.txt",
                "link_(link_file",
                "LinkTitle](link_(link_file)file.txt)"
            ]
        );
    }

    #[gpui::test]
    async fn test_surrounding_filename(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                ..Default::default()
            },
            cx,
        )
        .await;

        let test_cases = [
            ("file ˇ name", None),
            ("ˇfile name", Some("file")),
            ("file ˇname", Some("name")),
            ("fiˇle name", Some("file")),
            ("filenˇame", Some("filename")),
            // Absolute path
            ("foobar ˇ/home/user/f.txt", Some("/home/user/f.txt")),
            ("foobar /home/useˇr/f.txt", Some("/home/user/f.txt")),
            // Windows
            ("C:\\Useˇrs\\user\\f.txt", Some("C:\\Users\\user\\f.txt")),
            // Whitespace
            ("ˇfile\\ -\\ name.txt", Some("file - name.txt")),
            ("file\\ -\\ naˇme.txt", Some("file - name.txt")),
            // Tilde
            ("ˇ~/file.txt", Some("~/file.txt")),
            ("~/fiˇle.txt", Some("~/file.txt")),
            // Double quotes
            ("\"fˇile.txt\"", Some("file.txt")),
            ("ˇ\"file.txt\"", Some("file.txt")),
            ("ˇ\"fi\\ le.txt\"", Some("fi le.txt")),
            // Single quotes
            ("'fˇile.txt'", Some("file.txt")),
            ("ˇ'file.txt'", Some("file.txt")),
            ("ˇ'fi\\ le.txt'", Some("fi le.txt")),
            // Quoted multibyte characters
            (" ˇ\"常\"", Some("常")),
            (" \"ˇ常\"", Some("常")),
            ("ˇ\"常\"", Some("常")),
            // Backticks (surrounding_filename returns the full token including backticks)
            ("`fiˇle.txt`", Some("`file.txt`")),
            ("open `fiˇle.txt` please", Some("`file.txt`")),
            // Parentheses (surrounding_filename returns the full token including parens)
            ("(fiˇle.txt)", Some("(file.txt)")),
            ("open (fiˇle.txt) please", Some("(file.txt)")),
        ];

        for (input, expected) in test_cases {
            cx.set_state(input);

            let (position, snapshot) = cx.editor(|editor, _, cx| {
                let positions = editor
                    .selections
                    .newest_anchor()
                    .head()
                    .expect_text_anchor();
                let snapshot = editor
                    .buffer()
                    .clone()
                    .read(cx)
                    .as_singleton()
                    .unwrap()
                    .read(cx)
                    .snapshot();
                (positions, snapshot)
            });

            let result = surrounding_filename(&snapshot, position);

            if let Some(expected) = expected {
                assert!(result.is_some(), "Failed to find file path: {}", input);
                let (_, path) = result.unwrap();
                assert_eq!(&path, expected, "Incorrect file path for input: {}", input);
            } else {
                assert!(
                    result.is_none(),
                    "Expected no result, but got one: {:?}",
                    result
                );
            }
        }
    }

    #[gpui::test]
    async fn test_hover_filenames(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                ..Default::default()
            },
            cx,
        )
        .await;

        // Insert a new file
        let fs = cx.update_workspace(|workspace, _, cx| workspace.project().read(cx).fs().clone());
        fs.as_fake()
            .insert_file(
                path!("/root/dir/file2.rs"),
                "This is file2.rs".as_bytes().to_vec(),
            )
            .await;
        cx.run_until_parked();

        // Base document with {ABS} placeholder for absolute path prefix.
        // Each test case replaces a specific line to add cursor (ˇ) or highlight («»ˇ) markers.
        #[cfg(not(target_os = "windows"))]
        const ABS: &str = "/root/dir";
        #[cfg(target_os = "windows")]
        const ABS: &str = "C:/root/dir";

        let base = format!(
            "\
You can't go to a file that does_not_exist.txt.
Go to file2.rs if you want.
Or go to ../dir/file2.rs if you want.
Or go to {ABS}/file2.rs if project is local.
Or go to {ABS}/file2 if this is a Rust file.
Or `file2.rs` in backticks.
Or (file2.rs) in parens.
Or [link](file2.rs) markdown style.
A file (named file2.rs) in prose.
Read with `cat file2.rs` command.
Sentence ending file2.rs.
"
        );

        cx.set_state(&format!("{base}ˇ"));

        // Test cases: (original_line, cursor_line, highlight_line)
        // - cursor_line: the line with ˇ to position the mouse
        // - highlight_line: None = expect no highlight, Some(...) = expect this highlight
        let test_cases: &[(&str, &str, Option<&str>)] = &[
            // File does not exist - no highlight
            ("does_not_exist.txt", "dˇoes_not_exist.txt", None),
            // Simple filename
            (
                "Go to file2.rs if",
                "Go to fˇile2.rs if",
                Some("Go to «file2.rsˇ» if"),
            ),
            // Relative path
            (
                "Or go to ../dir/file2.rs if",
                "Or go to ../dir/fˇile2.rs if",
                Some("Or go to «../dir/file2.rsˇ» if"),
            ),
            // Absolute path
            (
                &format!("Or go to {ABS}/file2.rs if"),
                &format!("Or go to {ABS}/fiˇle2.rs if"),
                Some(&format!("Or go to «{ABS}/file2.rsˇ» if")),
            ),
            // Path without extension (language suffix added)
            (
                &format!("Or go to {ABS}/file2 if"),
                &format!("Or go to {ABS}/fiˇle2 if"),
                Some(&format!("Or go to «{ABS}/file2ˇ» if")),
            ),
            // Backticks
            (
                "Or `file2.rs` in backticks",
                "Or `fiˇle2.rs` in backticks",
                Some("Or `«file2.rsˇ»` in backticks"),
            ),
            // Parentheses
            (
                "Or (file2.rs) in parens",
                "Or (fiˇle2.rs) in parens",
                Some("Or («file2.rsˇ») in parens"),
            ),
            // Markdown link
            (
                "Or [link](file2.rs) markdown",
                "Or [link](fiˇle2.rs) markdown",
                Some("Or [link](«file2.rsˇ») markdown"),
            ),
            // Partial wrapper: trailing paren in prose like "(named file2.rs)"
            (
                "A file (named file2.rs) in",
                "A file (named fiˇle2.rs) in",
                Some("A file (named «file2.rsˇ») in"),
            ),
            // Partial wrapper: inside code span like "`cat file2.rs`"
            (
                "Read with `cat file2.rs` command",
                "Read with `cat fiˇle2.rs` command",
                Some("Read with `cat «file2.rsˇ»` command"),
            ),
            // Trailing period at end of sentence
            (
                "Sentence ending file2.rs.",
                "Sentence ending fiˇle2.rs.",
                Some("Sentence ending «file2.rsˇ»."),
            ),
        ];

        for (original, cursor_version, highlight_version) in test_cases {
            let position_text = base.replace(original, cursor_version);
            let screen_coord = cx.pixel_position(&position_text);
            cx.simulate_mouse_move(screen_coord, None, Modifiers::secondary_key());

            if let Some(highlight) = highlight_version {
                let expected = base.replace(original, highlight);
                cx.assert_editor_text_highlights(HighlightKey::HoveredLinkState, &expected);
            } else {
                // Expect no highlight
                cx.update_editor(|editor, window, cx| {
                    assert!(
                        editor
                            .snapshot(window, cx)
                            .text_highlight_ranges(HighlightKey::HoveredLinkState)
                            .unwrap_or_default()
                            .1
                            .is_empty(),
                        "Expected no highlight for cursor at: {}",
                        cursor_version
                    );
                });
            }
        }

        // Test click navigation on markdown link
        let position_text = base.replace(
            "Or [link](file2.rs) markdown",
            "Or [link](fiˇle2.rs) markdown",
        );
        let screen_coord = cx.pixel_position(&position_text);
        cx.simulate_mouse_move(screen_coord, None, Modifiers::secondary_key());
        cx.simulate_click(screen_coord, Modifiers::secondary_key());

        cx.update_workspace(|workspace, _, cx| assert_eq!(workspace.items(cx).count(), 2));
        cx.update_workspace(|workspace, _, cx| {
            let active_editor = workspace.active_item_as::<Editor>(cx).unwrap();

            let buffer = active_editor
                .read(cx)
                .buffer()
                .read(cx)
                .as_singleton()
                .unwrap();

            let file = buffer.read(cx).file().unwrap();
            let file_path = file.as_local().unwrap().abs_path(cx);

            assert_eq!(
                file_path,
                std::path::PathBuf::from(path!("/root/dir/file2.rs"))
            );
        });
    }

    #[gpui::test]
    async fn test_hover_filename_with_row_column(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                ..Default::default()
            },
            cx,
        )
        .await;

        // Insert a new file with multiple lines
        let fs = cx.update_workspace(|workspace, _, cx| workspace.project().read(cx).fs().clone());
        fs.as_fake()
            .insert_file(
                path!("/root/dir/file2.rs"),
                "line 1\nline 2\nline 3\nline 4\nline 5\nline 6\nline 7\nline 8\nline 9\nline 10\n"
                    .as_bytes()
                    .to_vec(),
            )
            .await;
        cx.run_until_parked();

        // file2.rs:5:3 should be highlighted and clickable
        cx.set_state(indoc! {"
            Go to file2.rs:5:3 for the fix.ˇ
        "});

        let screen_coord = cx.pixel_position(indoc! {"
            Go to filˇe2.rs:5:3 for the fix.
        "});

        cx.simulate_mouse_move(screen_coord, None, Modifiers::secondary_key());
        cx.assert_editor_text_highlights(
            HighlightKey::HoveredLinkState,
            indoc! {"
            Go to «file2.rs:5:3ˇ» for the fix.
        "},
        );

        cx.simulate_click(screen_coord, Modifiers::secondary_key());

        cx.update_workspace(|workspace, _, cx| assert_eq!(workspace.items(cx).count(), 2));
        cx.update_workspace(|workspace, window, cx| {
            let active_editor = workspace.active_item_as::<Editor>(cx).unwrap();
            {
                let editor = active_editor.read(cx);
                let buffer = editor.buffer().read(cx).as_singleton().unwrap();
                let file = buffer.read(cx).file().unwrap();
                let file_path = file.as_local().unwrap().abs_path(cx);
                assert_eq!(
                    file_path,
                    std::path::PathBuf::from(path!("/root/dir/file2.rs"))
                );
            }

            // Check that the cursor is at row 5, column 3 (0-indexed: row 4, col 2)
            let (count, snapshot) = active_editor.update(cx, |editor, cx| {
                (editor.selections.count(), editor.snapshot(window, cx))
            });
            assert_eq!(count, 1);
            let selections = active_editor
                .read(cx)
                .selections
                .newest::<language::Point>(&snapshot.display_snapshot);
            assert_eq!(
                selections.head().row,
                4,
                "Expected cursor on row 5 (0-indexed: 4)"
            );
            assert_eq!(
                selections.head().column,
                2,
                "Expected cursor on column 3 (0-indexed: 2)"
            );
        });
    }

    #[gpui::test]
    async fn test_hover_filename_with_row_only(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                ..Default::default()
            },
            cx,
        )
        .await;

        let fs = cx.update_workspace(|workspace, _, cx| workspace.project().read(cx).fs().clone());
        fs.as_fake()
            .insert_file(
                path!("/root/dir/file2.rs"),
                "line 1\nline 2\nline 3\nline 4\nline 5\n"
                    .as_bytes()
                    .to_vec(),
            )
            .await;
        cx.run_until_parked();

        // file2.rs:3 should be highlighted and clickable
        cx.set_state(indoc! {"
            Go to file2.rs:3 please.ˇ
        "});

        let screen_coord = cx.pixel_position(indoc! {"
            Go to filˇe2.rs:3 please.
        "});

        cx.simulate_mouse_move(screen_coord, None, Modifiers::secondary_key());
        cx.assert_editor_text_highlights(
            HighlightKey::HoveredLinkState,
            indoc! {"
            Go to «file2.rs:3ˇ» please.
        "},
        );

        cx.simulate_click(screen_coord, Modifiers::secondary_key());

        cx.update_workspace(|workspace, window, cx| {
            let active_editor = workspace.active_item_as::<Editor>(cx).unwrap();
            let (count, snapshot) = active_editor.update(cx, |editor, cx| {
                (editor.selections.count(), editor.snapshot(window, cx))
            });
            assert_eq!(count, 1);
            let selections = active_editor
                .read(cx)
                .selections
                .newest::<language::Point>(&snapshot.display_snapshot);
            assert_eq!(
                selections.head().row,
                2,
                "Expected cursor on row 3 (0-indexed: 2)"
            );
            assert_eq!(selections.head().column, 0, "Expected cursor on column 0");
        });
    }

    #[gpui::test]
    async fn test_hover_filename_with_non_numeric_suffix(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                ..Default::default()
            },
            cx,
        )
        .await;

        let fs = cx.update_workspace(|workspace, _, cx| workspace.project().read(cx).fs().clone());
        fs.as_fake()
            .insert_file(
                path!("/root/dir/file2.rs"),
                "line 1\nline 2\nline 3\n".as_bytes().to_vec(),
            )
            .await;
        cx.run_until_parked();

        // file2.rs:2:in should resolve to file2.rs line 2 (like Ruby backtraces)
        cx.set_state(indoc! {"
            Error at file2.rs:2:in 'method'ˇ
        "});

        let screen_coord = cx.pixel_position(indoc! {"
            Error at filˇe2.rs:2:in 'method'
        "});

        cx.simulate_mouse_move(screen_coord, None, Modifiers::secondary_key());
        cx.assert_editor_text_highlights(
            HighlightKey::HoveredLinkState,
            indoc! {"
            Error at «file2.rs:2:inˇ» 'method'
        "},
        );

        cx.simulate_click(screen_coord, Modifiers::secondary_key());

        cx.update_workspace(|workspace, window, cx| {
            let active_editor = workspace.active_item_as::<Editor>(cx).unwrap();
            let (count, snapshot) = active_editor.update(cx, |editor, cx| {
                (editor.selections.count(), editor.snapshot(window, cx))
            });
            assert_eq!(count, 1);
            let selections = active_editor
                .read(cx)
                .selections
                .newest::<language::Point>(&snapshot.display_snapshot);
            assert_eq!(
                selections.head().row,
                1,
                "Expected cursor on row 2 (0-indexed: 1)"
            );
        });
    }

    #[gpui::test]
    async fn test_hover_markdown_link_with_row_column(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                ..Default::default()
            },
            cx,
        )
        .await;

        let fs = cx.update_workspace(|workspace, _, cx| workspace.project().read(cx).fs().clone());
        fs.as_fake()
            .insert_file(
                path!("/root/dir/file2.rs"),
                "line 1\nline 2\nline 3\nline 4\nline 5\n"
                    .as_bytes()
                    .to_vec(),
            )
            .await;
        cx.run_until_parked();

        // Markdown link [text](file2.rs:3:2) should highlight only the inner link,
        // not the surrounding markdown syntax.
        cx.set_state(indoc! {"
            See [here](file2.rs:3:2) for details.ˇ
        "});

        let screen_coord = cx.pixel_position(indoc! {"
            See [here](filˇe2.rs:3:2) for details.
        "});

        cx.simulate_mouse_move(screen_coord, None, Modifiers::secondary_key());
        cx.assert_editor_text_highlights(
            HighlightKey::HoveredLinkState,
            indoc! {"
            See [here](«file2.rs:3:2ˇ») for details.
        "},
        );

        cx.simulate_click(screen_coord, Modifiers::secondary_key());

        cx.update_workspace(|workspace, window, cx| {
            let active_editor = workspace.active_item_as::<Editor>(cx).unwrap();
            {
                let editor = active_editor.read(cx);
                let buffer = editor.buffer().read(cx).as_singleton().unwrap();
                let file = buffer.read(cx).file().unwrap();
                let file_path = file.as_local().unwrap().abs_path(cx);
                assert_eq!(
                    file_path,
                    std::path::PathBuf::from(path!("/root/dir/file2.rs"))
                );
            }

            // Check cursor is at row 3, column 2 (0-indexed: row 2, col 1)
            let (count, snapshot) = active_editor.update(cx, |editor, cx| {
                (editor.selections.count(), editor.snapshot(window, cx))
            });
            assert_eq!(count, 1);
            let selections = active_editor
                .read(cx)
                .selections
                .newest::<language::Point>(&snapshot.display_snapshot);
            assert_eq!(
                selections.head().row,
                2,
                "Expected cursor on row 3 (0-indexed: 2)"
            );
            assert_eq!(
                selections.head().column,
                1,
                "Expected cursor on column 2 (0-indexed: 1)"
            );
        });
    }

    #[gpui::test]
    async fn test_hover_directories(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                ..Default::default()
            },
            cx,
        )
        .await;

        // Insert a new file
        let fs = cx.update_workspace(|workspace, _, cx| workspace.project().read(cx).fs().clone());
        fs.as_fake()
            .insert_file("/root/dir/file2.rs", "This is file2.rs".as_bytes().to_vec())
            .await;

        cx.set_state(indoc! {"
            You can't open ../diˇr because it's a directory.
        "});

        // File does not exist
        let screen_coord = cx.pixel_position(indoc! {"
            You can't open ../diˇr because it's a directory.
        "});
        cx.simulate_mouse_move(screen_coord, None, Modifiers::secondary_key());

        // No highlight
        cx.update_editor(|editor, window, cx| {
            assert!(
                editor
                    .snapshot(window, cx)
                    .text_highlight_ranges(HighlightKey::HoveredLinkState)
                    .unwrap_or_default()
                    .1
                    .is_empty()
            );
        });

        // Does not open the directory
        cx.simulate_click(screen_coord, Modifiers::secondary_key());
        cx.update_workspace(|workspace, _, cx| assert_eq!(workspace.items(cx).count(), 1));
    }

    #[gpui::test]
    async fn test_hover_unicode(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                ..Default::default()
            },
            cx,
        )
        .await;

        cx.set_state(indoc! {"
            You can't open ˇ\"🤩\" because it's an emoji.
        "});

        // File does not exist
        let screen_coord = cx.pixel_position(indoc! {"
            You can't open ˇ\"🤩\" because it's an emoji.
        "});
        cx.simulate_mouse_move(screen_coord, None, Modifiers::secondary_key());

        // No highlight, does not panic...
        cx.update_editor(|editor, window, cx| {
            assert!(
                editor
                    .snapshot(window, cx)
                    .text_highlight_ranges(HighlightKey::HoveredLinkState)
                    .unwrap_or_default()
                    .1
                    .is_empty()
            );
        });

        // Does not open the directory
        cx.simulate_click(screen_coord, Modifiers::secondary_key());
        cx.update_workspace(|workspace, _, cx| assert_eq!(workspace.items(cx).count(), 1));
    }

    #[gpui::test]
    async fn test_pressure_links(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
                definition_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            cx,
        )
        .await;

        cx.set_state(indoc! {"
                    fn ˇtest() { do_work(); }
                    fn do_work() { test(); }
                "});

        // Position the mouse over a symbol that has a definition
        let hover_point = cx.pixel_position(indoc! {"
                    fn test() { do_wˇork(); }
                    fn do_work() { test(); }
                "});
        let symbol_range = cx.lsp_range(indoc! {"
                    fn test() { «do_work»(); }
                    fn do_work() { test(); }
                "});
        let target_range = cx.lsp_range(indoc! {"
                    fn test() { do_work(); }
                    fn «do_work»() { test(); }
                "});

        let mut requests =
            cx.set_request_handler::<GotoDefinition, _, _>(move |url, _, _| async move {
                Ok(Some(lsp::GotoDefinitionResponse::Link(vec![
                    lsp::LocationLink {
                        origin_selection_range: Some(symbol_range),
                        target_uri: url.clone(),
                        target_range,
                        target_selection_range: target_range,
                    },
                ])))
            });

        cx.simulate_mouse_move(hover_point, None, Modifiers::none());

        // First simulate Normal pressure to set up the previous stage
        cx.simulate_event(MousePressureEvent {
            pressure: 0.5,
            stage: PressureStage::Normal,
            position: hover_point,
            modifiers: Modifiers::none(),
        });
        cx.background_executor.run_until_parked();

        // Now simulate Force pressure to trigger the force click and go-to definition
        cx.simulate_event(MousePressureEvent {
            pressure: 1.0,
            stage: PressureStage::Force,
            position: hover_point,
            modifiers: Modifiers::none(),
        });
        requests.next().await;
        cx.background_executor.run_until_parked();

        // Assert that we navigated to the definition
        cx.assert_editor_state(indoc! {"
                    fn test() { do_work(); }
                    fn «do_workˇ»() { test(); }
                "});
    }

    #[gpui::test]
    async fn test_document_links(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                document_link_provider: Some(lsp::DocumentLinkOptions {
                    resolve_provider: Some(false),
                    work_done_progress_options: lsp::WorkDoneProgressOptions::default(),
                }),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        cx.set_state(indoc! {"
            // See LICENSE for details
            fn main() {
                println!(\"hello\");
            }ˇ
        "});

        let link_range = cx.lsp_range(indoc! {"
            // See «LICENSE» for details
            fn main() {
                println!(\"hello\");
            }
        "});

        let mut requests = cx
            .lsp
            .set_request_handler::<lsp::request::DocumentLinkRequest, _, _>(
                move |_, _| async move {
                    Ok(Some(vec![lsp::DocumentLink {
                        range: link_range,
                        target: Some(
                            lsp::Uri::from_str("https://opensource.org/licenses/MIT").unwrap(),
                        ),
                        tooltip: Some("Open license".to_string()),
                        data: None,
                    }]))
                },
            );

        // Trigger document link fetch via LSP data refresh
        cx.run_until_parked();
        requests.next().await;
        cx.run_until_parked();

        // Cmd-hover over "LICENSE" should highlight it as a link
        let screen_coord = cx.pixel_position(indoc! {"
            // See LICˇENSE for details
            fn main() {
                println!(\"hello\");
            }
        "});

        cx.simulate_mouse_move(screen_coord, None, Modifiers::secondary_key());
        cx.run_until_parked();

        cx.assert_editor_text_highlights(
            HighlightKey::HoveredLinkState,
            indoc! {"
            // See «LICENSEˇ» for details
            fn main() {
                println!(\"hello\");
            }
        "},
        );

        // Clicking opens the URL
        cx.simulate_click(screen_coord, Modifiers::secondary_key());
        assert_eq!(
            cx.opened_url(),
            Some("https://opensource.org/licenses/MIT".into())
        );
    }

    #[gpui::test]
    async fn test_document_links_take_priority_over_url_detection(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                document_link_provider: Some(lsp::DocumentLinkOptions {
                    resolve_provider: Some(false),
                    work_done_progress_options: lsp::WorkDoneProgressOptions::default(),
                }),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        // Text contains a URL, but the LSP provides a document link that
        // covers a broader range and points to a different target.
        cx.set_state(indoc! {"
            // See https://example.com for more infoˇ
        "});

        let link_range = cx.lsp_range(indoc! {"
            // «See https://example.com for more info»
        "});

        let mut requests = cx
            .lsp
            .set_request_handler::<lsp::request::DocumentLinkRequest, _, _>(
                move |_, _| async move {
                    Ok(Some(vec![lsp::DocumentLink {
                        range: link_range,
                        target: Some(
                            lsp::Uri::from_str("https://lsp-provided.example.com").unwrap(),
                        ),
                        tooltip: None,
                        data: None,
                    }]))
                },
            );

        cx.run_until_parked();
        requests.next().await;
        cx.run_until_parked();

        let screen_coord = cx.pixel_position(indoc! {"
            // See https://examˇple.com for more info
        "});

        cx.simulate_mouse_move(screen_coord, None, Modifiers::secondary_key());
        cx.run_until_parked();

        // LSP document link range is highlighted, not just the URL portion
        cx.assert_editor_text_highlights(
            HighlightKey::HoveredLinkState,
            indoc! {"
            // «See https://example.com for more infoˇ»
        "},
        );

        // Clicking navigates to the LSP-provided target, not the detected URL.
        // (Uri::to_string normalizes "https://host" to "https://host/")
        cx.simulate_click(screen_coord, Modifiers::secondary_key());
        assert_eq!(
            cx.opened_url(),
            Some("https://lsp-provided.example.com/".into())
        );
    }

    #[gpui::test]
    async fn test_cmd_hover_aggregates_document_link_and_definition(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                document_link_provider: Some(lsp::DocumentLinkOptions {
                    resolve_provider: Some(false),
                    work_done_progress_options: lsp::WorkDoneProgressOptions::default(),
                }),
                definition_provider: Some(lsp::OneOf::Left(true)),
                type_definition_provider: Some(lsp::TypeDefinitionProviderCapability::Simple(true)),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;
        let source = indoc! {"
            fn main() { first(); second(); }
            fn definition() {}
            struct Type;
            struct Other;
        "};
        let first_position = source.replace("first()", "firˇst()");
        let second_position = source.replace("second()", "secˇond()");
        cx.set_state(&first_position);
        let link_range = cx.lsp_range(&source.replace("first()", "«first»()"));
        let second_range = cx.lsp_range(&source.replace("second()", "«second»()"));
        let document_range =
            cx.lsp_range(&source.replace("first(); second();", "«first(); second();»"));
        let definition_range = cx.lsp_range(&source.replace("definition()", "«definition»()"));
        let type_range = cx.lsp_range(&source.replace("Type;", "«Type»;"));
        let other_range = cx.lsp_range(&source.replace("Other;", "«Other»;"));
        for (document_range, origin_selection_range) in [
            (document_range, Some(link_range)),
            (document_range, None),
            (link_range, Some(link_range)),
            (link_range, None),
        ] {
            cache_test_document_link(
                &mut cx,
                &first_position,
                lsp::DocumentLink {
                    range: document_range,
                    target: Some(
                        lsp::Uri::from_str("https://opensource.org/licenses/MIT")
                            .expect("document link URI"),
                    ),
                    tooltip: None,
                    data: None,
                },
            )
            .await;
            for (shift, move_mouse, second) in [
                (false, false, false),
                (false, false, true),
                (false, true, true),
                (true, false, false),
                (true, true, false),
            ] {
                cx.simulate_modifiers_change(Modifiers::none());
                cx.set_selections_state(&first_position);
                let (release, response) = oneshot::channel::<()>();
                let response = response.shared();
                let mut definition_requests = cx.set_request_handler::<GotoDefinition, _, _>({
                    let response = response.clone();
                    move |url, params, _| {
                        let response = response.clone();
                        async move {
                            let second =
                                params.text_document_position_params.position >= second_range.start;
                            if second {
                                response.await.expect("release second definition");
                            }
                            let target_range = if second {
                                other_range
                            } else {
                                definition_range
                            };
                            Ok(Some(lsp::GotoDefinitionResponse::Link(vec![
                                lsp::LocationLink {
                                    origin_selection_range: origin_selection_range
                                        .map(|range| if second { second_range } else { range }),
                                    target_uri: url,
                                    target_range,
                                    target_selection_range: target_range,
                                },
                            ])))
                        }
                    }
                });
                cx.set_request_handler::<GotoTypeDefinition, _, _>(move |url, _, _| {
                    let response = response.clone();
                    async move {
                        response.await.expect("release type definition");
                        Ok(Some(lsp::GotoDefinitionResponse::Link(vec![
                            lsp::LocationLink {
                                origin_selection_range,
                                target_uri: url,
                                target_range: type_range,
                                target_selection_range: type_range,
                            },
                        ])))
                    }
                });

                let first = cx.pixel_position(&first_position);
                cx.simulate_mouse_move(first, None, Modifiers::secondary_key());
                definition_requests.next().await.expect("definition hover");
                cx.run_until_parked();
                cx.update_editor(|editor, _, _| {
                    let links = &editor
                        .hovered_link_state
                        .as_ref()
                        .expect("mixed hover")
                        .links;
                    match links.as_slice() {
                        [HoverLink::LspUrl(url), HoverLink::Text(_)] => {
                            assert_eq!(url, "https://opensource.org/licenses/MIT");
                        }
                        links => panic!("expected document link and definition, got {links:?}"),
                    }
                });

                for position in [&first_position, &source.replace("first()", "firsˇt()")] {
                    let point = cx.pixel_position(position);
                    cx.simulate_mouse_move(point, None, Modifiers::secondary_key());
                    cx.run_until_parked();
                    assert_eq!(definition_requests.next().now_or_never(), None);
                }

                let mut modifiers = Modifiers::secondary_key();
                modifiers.shift = shift;
                let clicked_position = if second {
                    &second_position
                } else {
                    &first_position
                };
                let clicked = cx.pixel_position(clicked_position);
                if move_mouse {
                    cx.simulate_mouse_move(clicked, None, modifiers);
                    cx.run_until_parked();
                    if !shift {
                        cx.simulate_mouse_move(first, None, modifiers);
                        cx.run_until_parked();
                        cx.simulate_mouse_move(clicked, None, modifiers);
                        cx.run_until_parked();
                    }
                    cx.assert_editor_text_highlights(HighlightKey::HoveredLinkState, source);
                    cx.update_editor(|editor, _, _| {
                        let state = editor.hovered_link_state.as_ref().expect("pending hover");
                        if !second || document_range.end >= second_range.end {
                            let [HoverLink::LspUrl(url)] = state.links.as_slice() else {
                                panic!("retained document link")
                            };
                            assert_eq!(url, "https://opensource.org/licenses/MIT");
                            assert!(state.symbol_range.is_some());
                        } else {
                            assert!(state.links.is_empty());
                            assert!(state.symbol_range.is_none());
                        }
                        assert_eq!(
                            state.preferred_kind,
                            if shift {
                                GotoDefinitionKind::Type
                            } else {
                                GotoDefinitionKind::Symbol
                            }
                        );
                    });
                }
                cx.simulate_click(clicked, modifiers);
                cx.simulate_modifiers_change(Modifiers::none());
                cx.run_until_parked();
                if shift || second {
                    cx.assert_editor_state(clicked_position);
                } else {
                    assert_eq!(definition_requests.next().now_or_never(), None);
                }
                release.send(()).expect("release pending navigation");
                cx.run_until_parked();
                let expected = if shift {
                    source.replace("Type;", "«Typeˇ»;")
                } else if second {
                    source.replace("Other;", "«Otherˇ»;")
                } else {
                    source.replace("definition()", "«definitionˇ»()")
                };
                cx.assert_editor_state(&expected);
                assert_eq!(cx.opened_url(), None);
            }
        }
    }

    #[gpui::test]
    async fn test_document_link_tooltip_popover(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                document_link_provider: Some(lsp::DocumentLinkOptions {
                    resolve_provider: Some(false),
                    work_done_progress_options: lsp::WorkDoneProgressOptions::default(),
                }),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        cx.set_state(indoc! {"
            // See LICENSE for detailsˇ
        "});

        let link_range = cx.lsp_range(indoc! {"
            // See «LICENSE» for details
        "});

        let mut requests = cx
            .lsp
            .set_request_handler::<lsp::request::DocumentLinkRequest, _, _>(
                move |_, _| async move {
                    Ok(Some(vec![lsp::DocumentLink {
                        range: link_range,
                        target: Some(
                            lsp::Uri::from_str("https://opensource.org/licenses/MIT").unwrap(),
                        ),
                        tooltip: Some("Open license".to_string()),
                        data: None,
                    }]))
                },
            );

        cx.run_until_parked();
        requests.next().await;
        cx.run_until_parked();

        let screen_coord = cx.pixel_position(indoc! {"
            // See LICˇENSE for details
        "});
        // Plain hover (no modifier) is enough; the doc-link tooltip stacks
        // alongside the regular LSP hover popovers.
        cx.simulate_mouse_move(screen_coord, None, Modifiers::none());
        let delay_ms = cx.update(|_, cx| EditorSettings::get_global(cx).hover_popover_delay.0);
        cx.background_executor
            .advance_clock(std::time::Duration::from_millis(delay_ms + 100));
        cx.run_until_parked();

        cx.update_editor(|editor, _, cx| {
            let tooltip_text = editor
                .hover_state
                .info_popovers
                .iter()
                .find_map(|popover| {
                    let parsed = popover.parsed_content.as_ref()?;
                    let text = parsed.read(cx).parsed_markdown().source().to_string();
                    (text == "Open license").then_some(text)
                })
                .expect("doc-link tooltip should appear in info_popovers on plain hover");
            assert_eq!(tooltip_text, "Open license");
        });

        // Move the mouse off the link; `show_hover` re-fires for the new
        // position and rebuilds `info_popovers` without the tooltip.
        let off_link = cx.pixel_position(indoc! {"
            // ˇSee LICENSE for details
        "});
        cx.simulate_mouse_move(off_link, None, Modifiers::none());
        cx.background_executor
            .advance_clock(std::time::Duration::from_millis(delay_ms + 100));
        cx.run_until_parked();
        cx.update_editor(|editor, _, cx| {
            let still_present = editor.hover_state.info_popovers.iter().any(|popover| {
                popover
                    .parsed_content
                    .as_ref()
                    .map(|parsed| *parsed.read(cx).parsed_markdown().source() == "Open license")
                    .unwrap_or(false)
            });
            assert!(
                !still_present,
                "doc-link tooltip should be cleared once the mouse leaves the link"
            );
        });
    }

    #[gpui::test]
    async fn test_document_link_resolve_on_hover(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                document_link_provider: Some(lsp::DocumentLinkOptions {
                    resolve_provider: Some(true),
                    work_done_progress_options: lsp::WorkDoneProgressOptions::default(),
                }),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        cx.set_state(indoc! {"
            // See LICENSE for detailsˇ
        "});

        let link_range = cx.lsp_range(indoc! {"
            // See «LICENSE» for details
        "});
        let resolve_data = serde_json::json!({"id": 42});

        let mut document_link_requests = {
            let resolve_data = resolve_data.clone();
            cx.lsp
                .set_request_handler::<lsp::request::DocumentLinkRequest, _, _>(move |_, _| {
                    let resolve_data = resolve_data.clone();
                    async move {
                        Ok(Some(vec![lsp::DocumentLink {
                            range: link_range,
                            target: None,
                            tooltip: None,
                            data: Some(resolve_data),
                        }]))
                    }
                })
        };

        let mut resolve_requests = cx
            .lsp
            .set_request_handler::<lsp::request::DocumentLinkResolve, _, _>(
                move |req, _| async move {
                    Ok(lsp::DocumentLink {
                        range: req.range,
                        target: Some(
                            lsp::Uri::from_str("https://opensource.org/licenses/MIT").unwrap(),
                        ),
                        tooltip: Some("Resolved tooltip".to_string()),
                        data: None,
                    })
                },
            );

        cx.run_until_parked();
        document_link_requests.next().await;
        cx.run_until_parked();

        let screen_coord = cx.pixel_position(indoc! {"
            // See LICˇENSE for details
        "});
        cx.simulate_mouse_move(screen_coord, None, Modifiers::none());
        let delay_ms = cx.update(|_, cx| EditorSettings::get_global(cx).hover_popover_delay.0);
        cx.background_executor
            .advance_clock(std::time::Duration::from_millis(delay_ms + 100));
        cx.run_until_parked();
        // Hover triggers resolve, not a viewport sweep.
        resolve_requests.next().await;
        cx.run_until_parked();

        cx.update_editor(|editor, _, cx| {
            let tooltip_text = editor
                .hover_state
                .info_popovers
                .iter()
                .find_map(|popover| {
                    let parsed = popover.parsed_content.as_ref()?;
                    let text = parsed.read(cx).parsed_markdown().source().to_string();
                    (text == "Resolved tooltip").then_some(text)
                })
                .expect("resolved doc-link tooltip should appear in info_popovers");
            assert_eq!(tooltip_text, "Resolved tooltip");
        });
    }

    #[gpui::test]
    async fn test_document_link_tooltip_respects_hover_popover_enabled(
        cx: &mut gpui::TestAppContext,
    ) {
        init_test(cx, |_| {});

        cx.update(|cx| {
            use gpui::BorrowAppContext as _;
            cx.update_global::<settings::SettingsStore, _>(|settings, cx| {
                settings.update_user_settings(cx, |settings| {
                    settings.editor.hover_popover_enabled = Some(false);
                });
            });
        });

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                document_link_provider: Some(lsp::DocumentLinkOptions {
                    resolve_provider: Some(false),
                    work_done_progress_options: lsp::WorkDoneProgressOptions::default(),
                }),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        cx.set_state(indoc! {"
            // See LICENSE for detailsˇ
        "});

        let link_range = cx.lsp_range(indoc! {"
            // See «LICENSE» for details
        "});

        let mut requests = cx
            .lsp
            .set_request_handler::<lsp::request::DocumentLinkRequest, _, _>(
                move |_, _| async move {
                    Ok(Some(vec![lsp::DocumentLink {
                        range: link_range,
                        target: Some(
                            lsp::Uri::from_str("https://opensource.org/licenses/MIT").unwrap(),
                        ),
                        tooltip: Some("Open license".to_string()),
                        data: None,
                    }]))
                },
            );

        cx.run_until_parked();
        requests.next().await;
        cx.run_until_parked();

        let screen_coord = cx.pixel_position(indoc! {"
            // See LICˇENSE for details
        "});
        cx.simulate_mouse_move(screen_coord, None, Modifiers::none());
        cx.background_executor
            .advance_clock(std::time::Duration::from_millis(2000));
        cx.run_until_parked();

        cx.update_editor(|editor, _, _| {
            assert!(
                editor.hover_state.info_popovers.is_empty(),
                "no popovers should appear when hover_popover_enabled is false"
            );
        });
    }

    #[gpui::test]
    async fn test_cmd_click_queues_intent_unless_disabled(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});
        update_test_editor_settings(cx, &|settings| {
            settings.lsp_results_location = Some(OpenResultsIn::Picker);
        });
        for cached in [false, true] {
            for (removed, disabled) in [(false, false), (true, false), (false, true)] {
                let mut cx = EditorLspTestContext::new_rust(hover_link_capabilities(), cx).await;
                cx.set_state("fn target() {}\nfn main() { tarˇget(); }\n");
                let requests = track_hover_link_requests(&cx);
                let intents = Rc::new(RefCell::new(Vec::new()));
                let source_id = cx.editor.entity_id();
                cx.update_workspace(|workspace, window, cx| {
                    if removed {
                        workspace.active_pane().update(cx, |pane, cx| {
                            pane.remove_item(source_id, false, false, window, cx);
                        });
                    }
                    workspace.register_action_renderer({
                        let intents = intents.clone();
                        move |div, _, _, _| {
                            let intents = intents.clone();
                            div.capture_action(move |action: &OpenLspLocations, _, cx| {
                                action
                                    .0
                                    .source
                                    .editor
                                    .update(cx, |editor, _| {
                                        assert!(action.0.source.request.is_current(editor));
                                    })
                                    .expect("source borrow released");
                                intents.borrow_mut().push(action.0.clone());
                                cx.stop_propagation();
                            })
                        }
                    });
                    cx.notify();
                });
                cx.run_until_parked();
                requests.store(0, Ordering::SeqCst);
                let position = cx.update_editor(|editor, window, cx| {
                    if disabled {
                        editor.disable_lsp_data();
                    }
                    let position = editor.selections.newest_anchor().head();
                    if cached {
                        let snapshot = editor.buffer.read(cx).snapshot(cx);
                        let symbol_range = snapshot.anchor_before(Point::new(1, 12))
                            ..snapshot.anchor_after(Point::new(1, 18));
                        let buffer = editor.buffer.read(cx).as_singleton().expect("buffer");
                        let snapshot = buffer.read(cx).snapshot();
                        editor.hovered_link_state = Some(HoveredLinkState {
                            last_trigger_point: TriggerPoint::Text(position),
                            preferred_kind: GotoDefinitionKind::Symbol,
                            lsp_data_enabled: true,
                            symbol_range: Some(RangeInEditor::Text(symbol_range)),
                            links: vec![HoverLink::Text(LocationLink {
                                origin: None,
                                target: project::Location {
                                    buffer,
                                    range: snapshot.anchor_before(Point::new(0, 3))
                                        ..snapshot.anchor_after(Point::new(0, 9)),
                                },
                            })],
                            task: None,
                        });
                    }
                    let snapshot = editor.snapshot(window, cx);
                    let point = position.to_display_point(&snapshot.display_snapshot);
                    let request = editor.navigation_request();
                    editor.handle_click_hovered_link(
                        PointForPosition {
                            previous_valid: point,
                            next_valid: point,
                            nearest_valid: point,
                            exact_unclipped: point,
                            column_overshoot_after_line_end: 0,
                        },
                        Modifiers::secondary_key(),
                        window,
                        cx,
                    );
                    assert_eq!(intents.borrow().len(), 0);
                    if disabled {
                        assert!(request.is_current(editor));
                    }
                    position
                });
                cx.run_until_parked();
                assert_eq!(requests.load(Ordering::SeqCst), 0);
                let intents = intents.borrow();
                assert_eq!(intents.len(), usize::from(!disabled));
                if let Some(intent) = intents.first() {
                    cx.editor(|editor, _, cx| {
                        let snapshot = editor.buffer.read(cx).snapshot(cx);
                        assert_eq!(
                            intent.source.position.to_point(&snapshot),
                            position.to_point(&snapshot)
                        );
                    });
                    let LspNavigationTarget::ClickedDefinition { kind, locations } = &intent.target
                    else {
                        panic!("expected clicked definition intent");
                    };
                    assert_eq!(*kind, GotoDefinitionKind::Symbol);
                    assert_eq!(locations.as_ref().map(Vec::len), cached.then_some(1));
                }
            }
        }
    }

    #[gpui::test]
    async fn test_disabled_lsp_data_does_not_query_hover_links(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorLspTestContext::new_rust(hover_link_capabilities(), cx).await;
        let requests = track_hover_link_requests(&cx);
        cache_test_document_link(
            &mut cx,
            "fn target() {}\nfn main() { tarˇget(); }\n",
            lsp::DocumentLink {
                range: lsp::Range::new(lsp::Position::new(1, 12), lsp::Position::new(1, 18)),
                target: None,
                tooltip: None,
                data: Some(serde_json::json!({"id": 1})),
            },
        )
        .await;
        cx.update_editor(|editor, _, _| editor.disable_lsp_data());

        for shift in [false, true] {
            show_test_link_definition(&mut cx, Point::new(1, 15), shift);
            cx.run_until_parked();
            cx.assert_editor_text_highlights(
                HighlightKey::HoveredLinkState,
                "fn target() {}\nfn main() { target(); }\n",
            );
            cx.update_editor(|editor, _, _| {
                assert_eq!(
                    editor
                        .hovered_link_state
                        .as_ref()
                        .map_or(0, |state| state.links.len()),
                    0,
                );
            });
            drop(cx.update_editor(|editor, _, _| {
                editor
                    .hovered_link_state
                    .as_mut()
                    .expect("negative hover cache")
                    .task
                    .take()
            }));
            for _ in 0..100 {
                show_test_link_definition(&mut cx, Point::new(1, 15), shift);
                cx.update_editor(|editor, _, _| {
                    assert!(
                        editor
                            .hovered_link_state
                            .as_ref()
                            .expect("negative hover cache")
                            .task
                            .is_none(),
                        "same-point negative hover spawned another task"
                    );
                });
            }
            let point = cx.pixel_position("fn target() {}\nfn main() { tarˇget(); }\n");
            let mut modifiers = Modifiers::secondary_key();
            modifiers.shift = shift;
            cx.simulate_click(point, modifiers);
            cx.run_until_parked();
            assert_eq!(requests.load(Ordering::SeqCst), 0);
            assert_eq!(cx.opened_url(), None);
            cx.assert_editor_state("fn target() {}\nfn main() { tarˇget(); }\n");
        }

        let point = cx.pixel_position("fn target() {}\nfn main() { tarˇget(); }\n");
        cx.simulate_mouse_move(point, None, Modifiers::secondary_key());
        let delay_ms = cx.update(|_, cx| EditorSettings::get_global(cx).hover_popover_delay.0);
        cx.background_executor
            .advance_clock(std::time::Duration::from_millis(delay_ms + 100));
        cx.run_until_parked();
        assert_eq!(requests.load(Ordering::SeqCst), 0);
    }

    #[gpui::test]
    async fn test_disabled_lsp_data_preserves_heuristic_hover_links(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});
        for cached in [false, true] {
            for (source, highlighted, file) in [
                (
                    "hˇttps://example.com/local",
                    "«https://example.com/local»",
                    false,
                ),
                ("fˇile2.rs", "«file2.rs»", true),
            ] {
                let mut cx = EditorLspTestContext::new_rust(hover_link_capabilities(), cx).await;
                let requests = track_hover_link_requests(&cx);
                let fs = cx
                    .update_workspace(|workspace, _, cx| workspace.project().read(cx).fs().clone());
                fs.as_fake()
                    .insert_file(path!("/root/dir/file2.rs"), b"fn decoy() {}".to_vec())
                    .await;
                fs.as_fake()
                    .insert_file(path!("/root/file2.rs"), b"fn expected() {}".to_vec())
                    .await;
                cx.simulate_modifiers_change(Modifiers::none());
                cx.set_state(source);
                cx.run_until_parked();
                cx.update_workspace(|workspace, _, cx| workspace.worktree_scans_complete(cx))
                    .await;
                requests.store(0, Ordering::SeqCst);
                if cached {
                    let point = cx.pixel_position(source);
                    cx.simulate_mouse_move(point, None, Modifiers::secondary_key());
                    cx.run_until_parked();
                    cx.assert_editor_text_highlights(HighlightKey::HoveredLinkState, highlighted);
                    requests.store(0, Ordering::SeqCst);
                }
                cx.update_editor(|editor, _, _| editor.disable_lsp_data());

                for _ in 0..2 {
                    show_test_link_definition(&mut cx, Point::new(0, 1), false);
                    cx.run_until_parked();
                    cx.assert_editor_text_highlights(HighlightKey::HoveredLinkState, highlighted);
                    cx.update_editor(|editor, _, _| {
                        let links = &editor
                            .hovered_link_state
                            .as_ref()
                            .expect("hovered link")
                            .links;
                        match links.as_slice() {
                            [HoverLink::File(_)] if file => {}
                            [HoverLink::Url(url)] if !file => {
                                assert_eq!(url, "https://example.com/local")
                            }
                            links => panic!("unexpected heuristic links: {links:?}"),
                        }
                    });
                    assert_eq!(requests.load(Ordering::SeqCst), 0);
                }

                let opened_url_before_click = cx.opened_url();
                let point = cx.pixel_position(source);
                cx.simulate_click(point, Modifiers::secondary_key());
                cx.run_until_parked();
                if file {
                    cx.update_workspace(|workspace, _, cx| {
                        let editor = workspace
                            .active_item_as::<Editor>(cx)
                            .expect("opened editor");
                        let buffer = editor
                            .read(cx)
                            .buffer()
                            .read(cx)
                            .as_singleton()
                            .expect("singleton buffer");
                        let file = buffer.read(cx).file().expect("opened file");
                        assert_eq!(
                            file.as_local().expect("local file").abs_path(cx),
                            PathBuf::from(path!("/root/file2.rs")),
                        );
                        assert_eq!(workspace.items(cx).count(), 2);
                    });
                    assert_eq!(cx.opened_url(), opened_url_before_click);
                } else {
                    assert_eq!(
                        cx.opened_url(),
                        Some(String::from("https://example.com/local"))
                    );
                    cx.update_workspace(|workspace, _, cx| {
                        assert_eq!(workspace.items(cx).count(), 1)
                    });
                }
                assert_eq!(requests.load(Ordering::SeqCst), 0);
            }
        }
    }

    #[gpui::test]
    async fn test_disabled_lsp_data_preserves_cached_url_provenance(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});
        for (semantic, pending) in [(false, false), (true, false), (false, true), (true, true)] {
            let mut capabilities = hover_link_capabilities();
            capabilities
                .document_link_provider
                .as_mut()
                .expect("document links")
                .resolve_provider = Some(false);
            let mut cx = EditorLspTestContext::new_rust(capabilities, cx).await;
            let requests = track_hover_link_requests(&cx);
            cache_test_document_link(
                &mut cx,
                "hˇttps://example.com/local",
                lsp::DocumentLink {
                    range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 25)),
                    target: semantic.then(|| {
                        lsp::Uri::from_str("https://example.com/semantic")
                            .expect("document link URI")
                    }),
                    tooltip: None,
                    data: None,
                },
            )
            .await;
            let pending = pending.then(|| pending_hover_response::<GotoDefinition>(&cx, None));
            show_test_link_definition(&mut cx, Point::new(0, 1), false);
            if let Some((started, respond)) = pending {
                started.await.expect("definition hover started");
                let task = cx.update_editor(|editor, _, _| {
                    editor.disable_lsp_data();
                    editor
                        .hovered_link_state
                        .as_mut()
                        .expect("pending hover")
                        .task
                        .take()
                        .expect("hover task")
                });
                requests.store(0, Ordering::SeqCst);
                respond.send(()).expect("release definition response");
                task.await;
            } else {
                cx.run_until_parked();
                cx.update_editor(|editor, _, _| {
                    match editor
                        .hovered_link_state
                        .as_ref()
                        .expect("cached hover")
                        .links
                        .as_slice()
                    {
                        [HoverLink::LspUrl(url)] if semantic => {
                            assert_eq!(url, "https://example.com/semantic")
                        }
                        [HoverLink::Url(url)] if !semantic => {
                            assert_eq!(url, "https://example.com/local")
                        }
                        links => panic!("unexpected cached links: {links:?}"),
                    }
                    editor.disable_lsp_data();
                });
                requests.store(0, Ordering::SeqCst);
                if semantic {
                    show_test_link_definition(&mut cx, Point::new(0, 1), false);
                    cx.run_until_parked();
                    cx.assert_editor_text_highlights(
                        HighlightKey::HoveredLinkState,
                        "«https://example.com/local»",
                    );
                }
            }
            cx.update(|_, cx| cx.open_url("https://example.com/before-click"));
            let point = cx.pixel_position("hˇttps://example.com/local");
            cx.simulate_click(point, Modifiers::secondary_key());
            cx.run_until_parked();
            assert_eq!(
                cx.opened_url(),
                Some(String::from("https://example.com/local"))
            );
            assert_eq!(requests.load(Ordering::SeqCst), 0);
        }
    }

    #[gpui::test]
    async fn test_pending_hover_discards_disabled_lsp_data(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorLspTestContext::new_rust(hover_link_capabilities(), cx).await;
        cx.set_state("fn target() {}\nfn main() { tarˇget(); }\n");
        for (document, shift) in [(false, false), (false, true), (true, false)] {
            cx.update_editor(|editor, _, _| editor.enable_lsp_data = true);
            let requests = track_hover_link_requests(&cx);
            let (source, point, (started, respond)) = if document {
                let mut link = lsp::DocumentLink {
                    range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 7)),
                    target: None,
                    tooltip: None,
                    data: Some(serde_json::json!({"id": 1})),
                };
                cache_test_document_link(&mut cx, "LICˇENSE", link.clone()).await;
                link.target = Some(
                    lsp::Uri::from_str("https://example.com/semantic").expect("document link URI"),
                );
                link.data = None;
                (
                    "LICENSE",
                    Point::new(0, 3),
                    pending_hover_response::<lsp::request::DocumentLinkResolve>(&cx, link),
                )
            } else {
                let target_range =
                    lsp::Range::new(lsp::Position::new(0, 3), lsp::Position::new(0, 9));
                let response = Some(lsp::GotoDefinitionResponse::Link(vec![lsp::LocationLink {
                    origin_selection_range: Some(lsp::Range::new(
                        lsp::Position::new(1, 12),
                        lsp::Position::new(1, 18),
                    )),
                    target_uri: cx.buffer_lsp_url.clone(),
                    target_range,
                    target_selection_range: target_range,
                }]));
                (
                    "fn target() {}\nfn main() { target(); }\n",
                    Point::new(1, 15),
                    if shift {
                        pending_hover_response::<GotoTypeDefinition>(&cx, response)
                    } else {
                        pending_hover_response::<GotoDefinition>(&cx, response)
                    },
                )
            };
            show_test_link_definition(&mut cx, point, shift);
            started.await.expect("hover request started");
            let task = cx.update_editor(|editor, _, _| {
                editor.disable_lsp_data();
                editor
                    .hovered_link_state
                    .as_mut()
                    .expect("pending hover")
                    .task
                    .take()
                    .expect("hover task")
            });
            respond.send(()).expect("release hover response");
            task.await;
            assert_eq!(requests.load(Ordering::SeqCst), 0);
            assert_eq!(cx.opened_url(), None);
            assert_no_test_hover_link(&mut cx, source);
        }
    }

    #[gpui::test]
    async fn test_hover_link_cache_endpoints(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});
        for (symbol, end_utf16, columns) in [
            ("target", 18, vec![11, 12, 13, 17, 18, 19]),
            ("é𐐀x", 16, vec![11, 12, 14, 18, 19, 20]),
        ] {
            for split in [false, true] {
                let mut cx = EditorLspTestContext::new_rust(hover_link_capabilities(), cx).await;
                let source = format!("fn definition() {{}}\nfn main() {{ {symbol}(); }}\n");
                cx.set_state(&format!("ˇ{source}"));
                let requests = Arc::new(AtomicUsize::new(0));
                cx.set_request_handler::<GotoDefinition, _, _>({
                    let requests = requests.clone();
                    move |url, params, _| {
                        assert_eq!(
                            params.text_document_position_params.position,
                            lsp::Position::new(1, 12)
                        );
                        requests.fetch_add(1, Ordering::SeqCst);
                        async move {
                            Ok(Some(lsp::GotoDefinitionResponse::Link(vec![
                                lsp::LocationLink {
                                    origin_selection_range: Some(lsp::Range::new(
                                        lsp::Position::new(1, 12),
                                        lsp::Position::new(1, end_utf16),
                                    )),
                                    target_uri: url,
                                    target_range: lsp::Range::new(
                                        lsp::Position::new(0, 3),
                                        lsp::Position::new(0, 13),
                                    ),
                                    target_selection_range: lsp::Range::new(
                                        lsp::Position::new(0, 3),
                                        lsp::Position::new(0, 13),
                                    ),
                                },
                            ])))
                        }
                    }
                });
                show_test_link_definition(&mut cx, Point::new(1, 12), false);
                cx.run_until_parked();
                cx.update_editor(|editor, window, cx| {
                    let snapshot = editor.snapshot(window, cx);
                    let state = editor
                        .hovered_link_state
                        .as_ref()
                        .expect("cached definition");
                    let actual = columns
                        .iter()
                        .map(|column| {
                            state.point_within_range(
                                &TriggerPoint::Text(
                                    snapshot
                                        .buffer_snapshot()
                                        .anchor_before(Point::new(1, *column)),
                                ),
                                &snapshot,
                            )
                        })
                        .collect::<Vec<_>>();
                    assert_eq!(actual, vec![false, true, true, true, false, false]);
                    let anchor = snapshot.buffer_snapshot().anchor_before(Point::new(1, 12));
                    let empty = HoveredLinkState {
                        last_trigger_point: TriggerPoint::Text(anchor),
                        preferred_kind: GotoDefinitionKind::Symbol,
                        lsp_data_enabled: true,
                        symbol_range: Some(RangeInEditor::Text(anchor..anchor)),
                        links: Vec::new(),
                        task: None,
                    };
                    assert!(!empty.point_within_range(&TriggerPoint::Text(anchor), &snapshot));
                });
                let mut modifiers = Modifiers::secondary_key();
                modifiers.alt = split;
                click_test_link(&mut cx, Point::new(1, 12), modifiers);
                cx.run_until_parked();
                assert_eq!(requests.load(Ordering::SeqCst), 1);
                cx.update_workspace(|workspace, _, cx| {
                    let editor = workspace
                        .active_item_as::<Editor>(cx)
                        .expect("target editor");
                    let editor = editor.read(cx);
                    assert_eq!(
                        editor
                            .selections
                            .newest_anchor()
                            .range()
                            .to_point(&editor.buffer.read(cx).snapshot(cx)),
                        Point::new(0, 3)..Point::new(0, 13)
                    );
                    assert_eq!(workspace.panes().len(), if split { 2 } else { 1 });
                });
            }
        }
    }

    #[gpui::test]
    async fn test_hover_link_mixed_kind_change(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});
        for (fresh_definition, fail_definition) in [(false, false), (true, false), (false, true)] {
            check_hover_link_kind_change(cx, true, fresh_definition, fail_definition, false).await;
        }
    }

    #[gpui::test]
    async fn test_hover_link_negative_kind_change(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});
        check_hover_link_kind_change(cx, false, true, false, false).await;
        let mut cx = EditorLspTestContext::new_rust(hover_link_capabilities(), cx).await;
        cx.set_state("ˇhttps://example.com/one/two\nstruct Target;\n");
        let requests = Arc::new(AtomicUsize::new(0));
        cx.set_request_handler::<GotoTypeDefinition, _, _>({
            let requests = requests.clone();
            move |uri, params, _| {
                requests.fetch_add(1, Ordering::SeqCst);
                let position = params.text_document_position_params.position;
                assert_eq!(position.line, 0);
                let definition = match position.character {
                    21 | 23 => None,
                    25 => Some(lsp::GotoDefinitionResponse::Scalar(lsp::Location {
                        uri,
                        range: lsp::Range::new(lsp::Position::new(1, 7), lsp::Position::new(1, 13)),
                    })),
                    _ => panic!("unexpected type query: {position:?}"),
                };
                async move { Ok(definition) }
            }
        });
        for column in [21, 23] {
            show_test_link_definition(&mut cx, Point::new(0, column), true);
            cx.run_until_parked();
        }
        let modifiers = Modifiers {
            shift: true,
            ..Modifiers::secondary_key()
        };
        click_test_link(&mut cx, Point::new(0, 23), modifiers);
        cx.run_until_parked();
        assert_eq!(requests.load(Ordering::SeqCst), 2);
        let opened_url = cx.opened_url();
        show_test_link_definition(&mut cx, Point::new(0, 25), true);
        cx.run_until_parked();
        assert_eq!(requests.load(Ordering::SeqCst), 3);
        click_test_link(&mut cx, Point::new(0, 25), modifiers);
        cx.run_until_parked();
        assert_eq!(requests.load(Ordering::SeqCst), 3);
        assert_eq!(cx.opened_url(), opened_url);
        cx.assert_editor_state("https://example.com/one/two\nstruct «Targetˇ»;\n");
    }

    #[gpui::test]
    async fn test_hover_link_disabled_inlay_data(cx: &mut gpui::TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                ..InlayHintSettingsContent::default()
            });
        });
        for pending_fetch in [false, true] {
            for change_mode in [false, true] {
                let mut cx = EditorLspTestContext::new_rust(
                    lsp::ServerCapabilities {
                        inlay_hint_provider: Some(lsp::OneOf::Right(
                            lsp::InlayHintServerCapabilities::Options(lsp::InlayHintOptions {
                                resolve_provider: Some(true),
                                ..lsp::InlayHintOptions::default()
                            }),
                        )),
                        ..lsp::ServerCapabilities::default()
                    },
                    cx,
                )
                .await;
                let hint = lsp::InlayHint {
                    position: lsp::Position::new(0, 21),
                    label: lsp::InlayHintLabel::String(": i32".to_owned()),
                    kind: Some(lsp::InlayHintKind::TYPE),
                    text_edits: None,
                    tooltip: None,
                    padding_left: Some(false),
                    padding_right: Some(false),
                    data: Some(serde_json::json!({"id": 1})),
                };
                let pending = if pending_fetch {
                    Some(pending_hover_response::<lsp::request::InlayHintRequest>(
                        &cx,
                        Some(vec![hint.clone()]),
                    ))
                } else {
                    cx.set_request_handler::<lsp::request::InlayHintRequest, _, _>({
                        let hint = hint.clone();
                        move |_, _, _| {
                            let hint = hint.clone();
                            async move { Ok(Some(vec![hint])) }
                        }
                    });
                    None
                };
                let resolves = Arc::new(AtomicUsize::new(0));
                cx.set_request_handler::<lsp::request::InlayHintResolveRequest, _, _>({
                    let resolves = resolves.clone();
                    move |_, hint, _| {
                        resolves.fetch_add(1, Ordering::SeqCst);
                        async move { Ok(hint) }
                    }
                });
                cx.set_state("ˇfn main() { let value = 0; }");
                let response = if let Some((started, response)) = pending {
                    started.await.expect("inlay request started");
                    Some(response)
                } else {
                    cx.run_until_parked();
                    cx.update_editor(|editor, _, cx| {
                        assert_eq!(visible_hint_labels(editor, cx), vec![": i32"])
                    });
                    None
                };
                let mode = cx.update_editor(|editor, _, _| {
                    let mode = editor.mode().clone();
                    if change_mode {
                        editor.set_mode(EditorMode::AutoHeight {
                            min_lines: 1,
                            max_lines: Some(4),
                        });
                    } else {
                        editor.disable_lsp_data();
                    }
                    mode
                });
                if let Some(response) = response {
                    response.send(()).expect("release inlay response");
                    cx.run_until_parked();
                    cx.update_editor(|editor, _, cx| {
                        assert_eq!(visible_hint_labels(editor, cx), Vec::<String>::new())
                    });
                } else {
                    cx.update_editor(|editor, window, cx| {
                        let snapshot = editor.snapshot(window, cx);
                        let mut point = snapshot
                            .buffer_snapshot()
                            .anchor_before(Point::new(0, 21))
                            .to_display_point(&snapshot.display_snapshot);
                        *point.column_mut() += 2;
                        let previous_valid = snapshot.clip_point(point, Bias::Left);
                        let next_valid = snapshot.clip_point(point, Bias::Right);
                        assert_ne!(previous_valid, next_valid);
                        editor.update_inlay_link_and_hover_points(
                            &snapshot,
                            PointForPosition {
                                previous_valid,
                                next_valid,
                                nearest_valid: previous_valid,
                                exact_unclipped: point,
                                column_overshoot_after_line_end: 0,
                            },
                            None,
                            true,
                            false,
                            window,
                            cx,
                        );
                    });
                    cx.run_until_parked();
                    assert_eq!(resolves.load(Ordering::SeqCst), 0);
                }
                cx.set_request_handler::<lsp::request::InlayHintRequest, _, _>(move |_, _, _| {
                    let hint = hint.clone();
                    async move { Ok(Some(vec![hint])) }
                });
                cx.update_editor(|editor, _, cx| {
                    editor.enable_lsp_data = true;
                    editor.set_mode(mode);
                    editor.refresh_inlay_hints(InlayHintRefreshReason::NewLinesShown, cx);
                });
                cx.run_until_parked();
                cx.update_editor(|editor, _, cx| {
                    assert_eq!(visible_hint_labels(editor, cx), vec![": i32"])
                });
            }
        }
    }

    #[gpui::test]
    async fn test_hover_link_reenabled_lsp_data(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});
        check_hover_link_kind_change(cx, false, true, false, true).await;
    }

    async fn check_hover_link_kind_change(
        cx: &mut gpui::TestAppContext,
        cached_definition: bool,
        fresh_definition: bool,
        fail_definition: bool,
        change_mode: bool,
    ) {
        for (token, document, file, click_column) in [
            ("https://example.com/local", false, false, 1),
            ("https://example.com/local", false, false, 21),
            ("LICENSE", true, false, 1),
            ("LICENSE", true, true, 1),
            ("file2.rs", false, true, 1),
            ("file2.rs", false, true, 7),
        ] {
            if change_mode && document {
                continue;
            }
            for (hover_before_click, finish_hover, split) in [
                (false, false, false),
                (true, false, false),
                (true, true, false),
                (false, false, true),
                (true, false, true),
                (true, true, true),
            ] {
                if (finish_hover || split) && !(document && file) {
                    continue;
                }
                let mut capabilities = hover_link_capabilities();
                capabilities
                    .document_link_provider
                    .as_mut()
                    .expect("document links")
                    .resolve_provider = Some(false);
                let mut cx = EditorLspTestContext::new_rust(capabilities, cx).await;
                let source = format!("{token}\nstruct Cached;\nfn fresh() {{}}\n");
                let document_target = if file {
                    url::Url::from_file_path(path!("/root/file2.rs"))
                        .expect("document file URL")
                        .to_string()
                } else {
                    "https://example.com/document".to_owned()
                };
                if document {
                    cache_test_document_link(
                        &mut cx,
                        &format!("ˇ{source}"),
                        lsp::DocumentLink {
                            range: lsp::Range::new(
                                lsp::Position::new(0, 0),
                                lsp::Position::new(0, 7),
                            ),
                            target: Some(
                                lsp::Uri::from_str(&document_target).expect("document URI"),
                            ),
                            tooltip: None,
                            data: None,
                        },
                    )
                    .await;
                } else {
                    cx.set_state(&format!("ˇ{source}"));
                }
                if file {
                    let fs = cx.update_workspace(|workspace, _, cx| {
                        workspace.project().read(cx).fs().clone()
                    });
                    fs.as_fake()
                        .insert_file(path!("/root/file2.rs"), b"expected file".to_vec())
                        .await;
                    cx.run_until_parked();
                    cx.update_workspace(|workspace, _, cx| workspace.worktree_scans_complete(cx))
                        .await;
                }
                let type_requests = Arc::new(AtomicUsize::new(0));
                let symbol_requests = Arc::new(AtomicUsize::new(0));
                cx.set_request_handler::<GotoTypeDefinition, _, _>({
                    let requests = type_requests.clone();
                    move |url, params, _| {
                        assert_eq!(
                            params.text_document_position_params.position,
                            lsp::Position::new(0, 1)
                        );
                        requests.fetch_add(1, Ordering::SeqCst);
                        async move {
                            Ok(cached_definition.then(|| {
                                lsp::GotoDefinitionResponse::Scalar(lsp::Location {
                                    uri: url,
                                    range: lsp::Range::new(
                                        lsp::Position::new(1, 7),
                                        lsp::Position::new(1, 13),
                                    ),
                                })
                            }))
                        }
                    }
                });
                let (release, response) = oneshot::channel::<()>();
                let response = response.shared();
                cx.set_request_handler::<GotoDefinition, _, _>({
                    let requests = symbol_requests.clone();
                    move |url, params, _| {
                        assert_eq!(
                            params.text_document_position_params.position,
                            lsp::Position::new(0, click_column)
                        );
                        requests.fetch_add(1, Ordering::SeqCst);
                        let response = response.clone();
                        async move {
                            response.await.expect("release symbol response");
                            if fail_definition {
                                anyhow::bail!("definition lookup failed");
                            }
                            Ok(fresh_definition.then(|| {
                                lsp::GotoDefinitionResponse::Scalar(lsp::Location {
                                    uri: url,
                                    range: lsp::Range::new(
                                        lsp::Position::new(2, 3),
                                        lsp::Position::new(2, 8),
                                    ),
                                })
                            }))
                        }
                    }
                });
                cx.update(|_, cx| cx.open_url("https://example.com/before-click"));
                if change_mode {
                    cx.update_editor(|editor, _, _| {
                        editor.set_mode(EditorMode::AutoHeight {
                            min_lines: 1,
                            max_lines: None,
                        });
                    });
                }
                show_test_link_definition(&mut cx, Point::new(0, 1), !change_mode);
                cx.run_until_parked();
                cx.update_editor(|editor, _, cx| {
                    let links = &editor
                        .hovered_link_state
                        .as_ref()
                        .expect("cached hover")
                        .links;
                    assert_eq!(
                        links.len(),
                        1 + usize::from(cached_definition),
                        "{token}, hover_before_click={hover_before_click}"
                    );
                    match &links[0] {
                        HoverLink::File(target) if file => {
                            let ResolvedPath::ProjectPath {
                                project_path,
                                is_dir,
                            } = &target.resolved_path
                            else {
                                panic!("project file")
                            };
                            assert_eq!(project_path.path.as_unix_str(), "file2.rs");
                            assert!(!is_dir);
                        }
                        HoverLink::LspLocation(location, _) if document && file => {
                            assert_eq!(location.uri.as_str(), document_target)
                        }
                        HoverLink::LspUrl(url) if document => {
                            assert_eq!(url, &document_target)
                        }
                        HoverLink::Url(url) if !file && !document => {
                            assert_eq!(url, "https://example.com/local")
                        }
                        link => panic!("unexpected independent link: {link:?}"),
                    }
                    if cached_definition {
                        let HoverLink::Text(link) = &links[1] else {
                            panic!("cached type definition")
                        };
                        assert_eq!(
                            link.target.range.to_point(link.target.buffer.read(cx)),
                            Point::new(1, 7)..Point::new(1, 13)
                        );
                    }
                });
                assert_eq!(
                    type_requests.load(Ordering::SeqCst),
                    usize::from(!change_mode)
                );
                if change_mode {
                    cx.update_editor(|editor, _, _| editor.set_mode(EditorMode::full()));
                }
                let mut release = Some(release);
                if hover_before_click {
                    show_test_link_definition(&mut cx, Point::new(0, click_column), false);
                    cx.run_until_parked();
                    if finish_hover {
                        release
                            .take()
                            .expect("pending response")
                            .send(())
                            .expect("release hover");
                        cx.run_until_parked();
                    }
                }
                click_test_link(
                    &mut cx,
                    Point::new(0, click_column),
                    Modifiers {
                        alt: split,
                        ..Modifiers::secondary_key()
                    },
                );
                cx.run_until_parked();
                assert_eq!(
                    cx.opened_url().as_deref(),
                    Some("https://example.com/before-click")
                );
                if let Some(release) = release {
                    release.send(()).expect("release definition lookup");
                }
                cx.run_until_parked();
                assert_eq!(
                    type_requests.load(Ordering::SeqCst),
                    usize::from(!change_mode)
                );
                assert_eq!(
                    symbol_requests.load(Ordering::SeqCst),
                    if hover_before_click && !finish_hover {
                        2
                    } else {
                        1
                    }
                );
                if fresh_definition && document && file {
                    cx.update_workspace(|workspace, _, cx| {
                        let editor = workspace
                            .active_item_as::<Editor>(cx)
                            .expect("target editor");
                        assert_eq!(
                            editor
                                .read(cx)
                                .buffer
                                .read(cx)
                                .all_buffers()
                                .iter()
                                .map(|buffer| buffer.read(cx).snapshot().text())
                                .sorted()
                                .collect::<Vec<_>>(),
                            [source.clone(), "expected file".to_owned()]
                                .into_iter()
                                .sorted()
                                .collect::<Vec<_>>(),
                            "document file and fresh definition must both be retained"
                        );
                        let editor = editor.read(cx);
                        let snapshot = editor.buffer.read(cx).snapshot(cx);
                        assert_eq!(
                            editor
                                .background_highlights
                                .get(&HighlightKey::Editor)
                                .expect("navigation targets")
                                .1
                                .iter()
                                .map(|range| snapshot
                                    .text_for_range(range.clone())
                                    .collect::<String>())
                                .sorted()
                                .collect::<Vec<_>>(),
                            vec!["", "fresh"],
                            "fresh definition, not cached type definition"
                        );
                    });
                } else if fresh_definition {
                    cx.assert_editor_state(&source.replace("fresh()", "«freshˇ»()"));
                } else if file {
                    cx.update_workspace(|workspace, _, cx| {
                        let editor = workspace.active_item_as::<Editor>(cx).expect("opened file");
                        assert_eq!(
                            editor.read(cx).buffer.read(cx).snapshot(cx).text(),
                            "expected file"
                        );
                    });
                }
                assert_eq!(
                    cx.opened_url().as_deref(),
                    Some(if fresh_definition || file {
                        "https://example.com/before-click"
                    } else if document {
                        "https://example.com/document"
                    } else {
                        "https://example.com/local"
                    })
                );
                cx.update_workspace(|workspace, _, cx| {
                    assert_eq!(workspace.panes().len(), if split { 2 } else { 1 });
                    assert_eq!(
                        workspace.items(cx).count(),
                        if file && (!fresh_definition || document) {
                            2
                        } else {
                            1
                        }
                    )
                });
            }
        }
    }

    fn click_test_link(cx: &mut EditorLspTestContext, position: Point, modifiers: Modifiers) {
        cx.update_editor(|editor, window, cx| {
            let snapshot = editor.snapshot(window, cx);
            let point = snapshot
                .buffer_snapshot()
                .anchor_before(position)
                .to_display_point(&snapshot.display_snapshot);
            editor.handle_click_hovered_link(
                PointForPosition {
                    previous_valid: point,
                    next_valid: point,
                    nearest_valid: point,
                    exact_unclipped: point,
                    column_overshoot_after_line_end: 0,
                },
                modifiers,
                window,
                cx,
            );
        });
    }

    fn hover_link_capabilities() -> lsp::ServerCapabilities {
        lsp::ServerCapabilities {
            definition_provider: Some(lsp::OneOf::Left(true)),
            type_definition_provider: Some(lsp::TypeDefinitionProviderCapability::Simple(true)),
            references_provider: Some(lsp::OneOf::Left(true)),
            document_link_provider: Some(lsp::DocumentLinkOptions {
                resolve_provider: Some(true),
                work_done_progress_options: lsp::WorkDoneProgressOptions::default(),
            }),
            ..lsp::ServerCapabilities::default()
        }
    }

    fn track_hover_link_requests(cx: &EditorLspTestContext) -> Arc<AtomicUsize> {
        let requests = Arc::new(AtomicUsize::new(0));
        cx.lsp.set_request_handler::<GotoDefinition, _, _>({
            let requests = requests.clone();
            let source_uri = cx.buffer_lsp_url.clone();
            move |params, _| {
                if params.text_document_position_params.text_document.uri == source_uri {
                    requests.fetch_add(1, Ordering::SeqCst);
                }
                async { Ok(None) }
            }
        });
        cx.lsp.set_request_handler::<GotoTypeDefinition, _, _>({
            let requests = requests.clone();
            let source_uri = cx.buffer_lsp_url.clone();
            move |params, _| {
                if params.text_document_position_params.text_document.uri == source_uri {
                    requests.fetch_add(1, Ordering::SeqCst);
                }
                async { Ok(None) }
            }
        });
        cx.lsp
            .set_request_handler::<lsp::request::References, _, _>({
                let requests = requests.clone();
                let source_uri = cx.buffer_lsp_url.clone();
                move |params, _| {
                    if params.text_document_position.text_document.uri == source_uri {
                        requests.fetch_add(1, Ordering::SeqCst);
                    }
                    async { Ok(None) }
                }
            });
        cx.lsp
            .set_request_handler::<lsp::request::DocumentLinkResolve, _, _>({
                let requests = requests.clone();
                move |link, _| {
                    requests.fetch_add(1, Ordering::SeqCst);
                    async move { Ok(link) }
                }
            });
        requests
    }

    async fn cache_test_document_link(
        cx: &mut EditorLspTestContext,
        source: &str,
        link: lsp::DocumentLink,
    ) {
        let mut requests = cx
            .lsp
            .set_request_handler::<lsp::request::DocumentLinkRequest, _, _>(move |_, _| {
                let link = link.clone();
                async move { Ok(Some(vec![link])) }
            });
        cx.set_state(source);
        cx.run_until_parked();
        requests.next().await.expect("document links fetched");
        cx.run_until_parked();
        cx.update_editor(|editor, _, _| {
            assert_eq!(
                editor
                    .lsp_document_links
                    .per_buffer
                    .values()
                    .flat_map(|servers| servers.values())
                    .map(|links| links.len())
                    .sum::<usize>(),
                1,
            );
        });
    }

    fn show_test_link_definition(cx: &mut EditorLspTestContext, point: Point, shift: bool) {
        cx.update_editor(|editor, window, cx| {
            let snapshot = editor.snapshot(window, cx);
            let anchor = snapshot.buffer_snapshot().anchor_before(point);
            show_link_definition(
                shift,
                editor,
                TriggerPoint::Text(anchor),
                &snapshot,
                window,
                cx,
            );
        });
    }

    fn pending_hover_response<R>(
        cx: &EditorLspTestContext,
        response: R::Result,
    ) -> (oneshot::Receiver<()>, oneshot::Sender<()>)
    where
        R: lsp::request::Request + 'static,
        R::Params: Send + 'static,
        R::Result: Send + 'static,
    {
        let (started_sender, started_receiver) = oneshot::channel();
        let (response_sender, response_receiver) = oneshot::channel();
        let mut pending = Some((started_sender, response_receiver, response));
        cx.set_request_handler::<R, _, _>(move |_, _, _| {
            let (started_sender, response_receiver, response) =
                pending.take().expect("one hover request");
            started_sender.send(()).expect("request listener");
            async move {
                response_receiver.await.expect("release hover response");
                Ok(response)
            }
        });
        (started_receiver, response_sender)
    }

    fn assert_no_test_hover_link(cx: &mut EditorLspTestContext, source: &str) {
        cx.assert_editor_text_highlights(HighlightKey::HoveredLinkState, source);
        cx.update_editor(|editor, _, _| assert!(editor.hovered_link_state.is_none()));
    }
}
