use crate::{
    Anchor, Editor, EditorSettings, EditorSnapshot, FindAllReferences, GoToDefinition,
    GoToDefinitionSplit, GoToTypeDefinition, GoToTypeDefinitionSplit, GotoDefinitionKind,
    HighlightKey, Navigated, PointForPosition, SelectPhase,
    editor_settings::GoToDefinitionFallback, scroll::ScrollAmount,
};
use gpui::{
    App, AsyncWindowContext, Context, Entity, HighlightStyle, Modifiers, Pixels, Task,
    UnderlineStyle, Window, px,
};
use language::{Bias, ToOffset};
use linkify::{LinkFinder, LinkKind};
use lsp::LanguageServerId;
use project::{InlayId, LocationLink, Project, ResolvedPath};
use regex::Regex;
use settings::Settings;
use std::{ops::Range, str::FromStr as _, sync::LazyLock};
use text::OffsetRangeExt;
use theme::ActiveTheme as _;
use util::{ResultExt, TryFutureExt as _, paths::PathWithPosition};

#[derive(Debug)]
pub struct HoveredLinkState {
    pub last_trigger_point: TriggerPoint,
    pub preferred_kind: GotoDefinitionKind,
    pub symbol_range: Option<RangeInEditor>,
    pub links: Vec<HoverLink>,
    pub task: Option<Task<Option<()>>>,
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
                let point_after_start = range.start.cmp(point, &snapshot.buffer_snapshot()).is_le();
                point_after_start && range.end.cmp(point, &snapshot.buffer_snapshot()).is_ge()
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
}

#[derive(Debug, Clone)]
pub enum HoverLink {
    Url(String),
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
            .and_then(parse_uri_fragment_position)
            .unwrap_or_default();
        return HoverLink::LspLocation(
            lsp::Location {
                uri,
                range: lsp::Range::new(position, position),
            },
            server_id,
        );
    }
    HoverLink::Url(target.to_string())
}

/// Parse a URI fragment such as `9,16`, `9:16`, `L9`, or `L9:16` into an
/// LSP position (1-based input, 0-based output). Servers like the JSON
/// language server attach this fragment to `file://` document link
/// targets to point at a specific row/column inside the file.
fn parse_uri_fragment_position(fragment: &str) -> Option<lsp::Position> {
    let stripped = fragment.strip_prefix('L').unwrap_or(fragment);
    let (line_str, column_str) = match stripped.split_once([',', ':']) {
        Some((line, column)) => (line, Some(column)),
        None => (stripped, None),
    };
    let line = line_str.parse::<u32>().ok()?.checked_sub(1)?;
    let character = column_str
        .and_then(|column| column.parse::<u32>().ok())
        .and_then(|column| column.checked_sub(1))
        .unwrap_or(0);
    Some(lsp::Position { line, character })
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
        let reveal_task = self.cmd_click_reveal_task(point, modifiers, window, cx);
        cx.spawn_in(window, async move |editor, cx| {
            let definition_revealed = reveal_task.await.log_err().unwrap_or(Navigated::No);
            let find_references = editor
                .update_in(cx, |editor, window, cx| {
                    if definition_revealed == Navigated::Yes {
                        return None;
                    }
                    match EditorSettings::get_global(cx).go_to_definition_fallback {
                        GoToDefinitionFallback::None => None,
                        GoToDefinitionFallback::FindAllReferences => {
                            editor.find_all_references(&FindAllReferences::default(), window, cx)
                        }
                    }
                })
                .ok()
                .flatten();
            if let Some(find_references) = find_references {
                find_references.await.log_err();
            }
        })
        .detach();
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
        if let Some(hovered_link_state) = self.hovered_link_state.take() {
            self.hide_hovered_link(cx);
            if !hovered_link_state.links.is_empty() {
                if !self.focus_handle.is_focused(window) {
                    window.focus(&self.focus_handle, cx);
                }

                // exclude links pointing back to the current anchor
                let current_position = point
                    .next_valid
                    .to_point(&self.snapshot(window, cx).display_snapshot);
                let Some((buffer, anchor)) = self
                    .buffer()
                    .read(cx)
                    .text_anchor_for_position(current_position, cx)
                else {
                    return Task::ready(Ok(Navigated::No));
                };
                let Some(multi_buffer_anchor) = self
                    .buffer()
                    .read(cx)
                    .snapshot(cx)
                    .anchor_in_excerpt(anchor)
                else {
                    return Task::ready(Ok(Navigated::No));
                };
                let links = hovered_link_state
                    .links
                    .into_iter()
                    .filter(|link| {
                        if let HoverLink::Text(location) = link {
                            exclude_link_to_position(&buffer, &anchor, location, cx)
                        } else {
                            true
                        }
                    })
                    .collect();
                let nav_entry = self.navigation_entry(multi_buffer_anchor, cx);
                let split = Self::is_alt_pressed(&modifiers, cx);
                let navigate_task =
                    self.navigate_to_hover_links(None, links, nav_entry, split, window, cx);
                self.select(SelectPhase::End, window, cx);
                return navigate_task;
            }
        }

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

        let navigate_task = if point.as_valid().is_some() {
            let split = Self::is_alt_pressed(&modifiers, cx);
            match (modifiers.shift, split) {
                (true, true) => {
                    self.go_to_type_definition_split(&GoToTypeDefinitionSplit, window, cx)
                }
                (true, false) => self.go_to_type_definition(&GoToTypeDefinition, window, cx),
                (false, true) => self.go_to_definition_split(&GoToDefinitionSplit, window, cx),
                (false, false) => self.go_to_definition(&GoToDefinition, window, cx),
            }
        } else {
            Task::ready(Ok(Navigated::No))
        };
        self.select(SelectPhase::End, window, cx);
        navigate_task
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

    let (mut hovered_link_state, is_cached) =
        if let Some(existing) = editor.hovered_link_state.take() {
            (existing, true)
        } else {
            (
                HoveredLinkState {
                    last_trigger_point: trigger_point.clone(),
                    symbol_range: None,
                    preferred_kind,
                    links: vec![],
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
        || hovered_link_state
            .links
            .first()
            .is_some_and(|d| matches!(d, HoverLink::Url(_) | HoverLink::LspLocation(_, _)));

    if same_kind {
        if is_cached && (hovered_link_state.last_trigger_point == trigger_point)
            || hovered_link_state
                .symbol_range
                .as_ref()
                .is_some_and(|symbol_range| {
                    symbol_range.point_within_range(&trigger_point, snapshot)
                })
        {
            editor.hovered_link_state = Some(hovered_link_state);
            return;
        }
    } else {
        editor.hide_hovered_link(cx)
    }
    let project = editor.project.clone();
    let provider = editor.semantics_provider.clone();

    hovered_link_state.task = Some(cx.spawn_in(window, async move |this, cx| {
        async move {
            // LSP document links take priority: the server explicitly
            // declares which ranges are clickable, so they are more
            // accurate than the heuristic-based URL/file detection.
            //
            // Resolution is deduplicated by `LspStore`; awaiting here only
            // blocks until either the cached resolved entry is returned or
            // the in-flight `Shared` task completes.
            let resolved_document_links = this
                .update(cx, |editor, cx| {
                    editor.document_links_at(buffer.clone(), anchor, cx)
                })
                .ok()
                .flatten();
            let resolved_document_links = match resolved_document_links {
                Some(task) => task.await,
                None => Vec::new(),
            };
            let snapshot = this.read_with(cx, |editor, cx| editor.buffer.read(cx).snapshot(cx))?;
            let detected_document_link =
                resolved_document_links
                    .into_iter()
                    .find_map(|(server_id, link)| {
                        let multi_buffer_range =
                            snapshot.buffer_anchor_range_to_anchor_range(link.range.clone())?;
                        Some((link.range, multi_buffer_range, link.target, server_id))
                    });
            drop(snapshot);

            let result = match &trigger_point {
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
                        let snapshot =
                            this.read_with(cx, |editor, cx| editor.buffer.read(cx).snapshot(cx))?;
                        if let Some(range) = snapshot.buffer_anchor_range_to_anchor_range(url_range)
                        {
                            symbol_range = Some(RangeInEditor::Text(range));
                        }
                        links.push(HoverLink::Url(url));
                    } else if let Some((filename_range, file_target)) =
                        find_file(&buffer, project.clone(), anchor, cx).await
                    {
                        let snapshot =
                            this.read_with(cx, |editor, cx| editor.buffer.read(cx).snapshot(cx))?;
                        if let Some(range) =
                            snapshot.buffer_anchor_range_to_anchor_range(filename_range)
                        {
                            symbol_range = Some(RangeInEditor::Text(range));
                        }
                        links.push(HoverLink::File(file_target));
                    }

                    // Always also collect LSP definitions so that cmd-click
                    // reveals every applicable target (e.g. a position that
                    // carries both a document link and a definition).
                    if let Some(provider) = provider {
                        let task = cx.update(|_, cx| {
                            provider.definitions(&buffer, anchor, preferred_kind, cx)
                        })?;
                        if let Some(task) = task
                            && let Some(definition_result) = task.await.ok().flatten()
                        {
                            if symbol_range.is_none() {
                                let snapshot = this.read_with(cx, |editor, cx| {
                                    editor.buffer.read(cx).snapshot(cx)
                                })?;
                                symbol_range = definition_result.iter().find_map(|link| {
                                    link.origin.as_ref().and_then(|origin| {
                                        let range = snapshot.buffer_anchor_range_to_anchor_range(
                                            origin.range.clone(),
                                        )?;
                                        Some(RangeInEditor::Text(range))
                                    })
                                });
                            }
                            links.extend(definition_result.into_iter().map(HoverLink::Text));
                        }
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

            this.update(cx, |editor, cx| {
                // Clear any existing highlights
                editor.clear_highlights(HighlightKey::HoveredLinkState, cx);
                let Some(hovered_link_state) = editor.hovered_link_state.as_mut() else {
                    editor.hide_hovered_link(cx);
                    return;
                };
                hovered_link_state.preferred_kind = preferred_kind;
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
                    editor.hide_hovered_link(cx);
                }
            })?;

            anyhow::Ok(())
        }
        .log_err()
        .await
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
        DisplayPoint,
        display_map::ToDisplayPoint,
        editor_tests::init_test,
        inlays::inlay_hints::tests::{cached_hint_labels, visible_hint_labels},
        test::editor_lsp_test_context::EditorLspTestContext,
    };
    use futures::StreamExt;
    use gpui::{Modifiers, MousePressureEvent, PressureStage};
    use indoc::indoc;
    use lsp::request::{GotoDefinition, GotoTypeDefinition};
    use multi_buffer::MultiBufferOffset;
    use settings::InlayHintSettingsContent;
    use std::str::FromStr;
    use util::{assert_set_eq, path};
    use workspace::item::Item;

    #[test]
    fn test_parse_uri_fragment_position() {
        // json-language-server style: 1-based `line,column`.
        assert_eq!(
            parse_uri_fragment_position("9,16"),
            Some(lsp::Position {
                line: 8,
                character: 15,
            })
        );
        assert_eq!(
            parse_uri_fragment_position("33,33"),
            Some(lsp::Position {
                line: 32,
                character: 32,
            })
        );

        // GitHub-style `L<line>` and `L<line>:<col>`.
        assert_eq!(
            parse_uri_fragment_position("L42"),
            Some(lsp::Position {
                line: 41,
                character: 0,
            })
        );
        assert_eq!(
            parse_uri_fragment_position("L42:7"),
            Some(lsp::Position {
                line: 41,
                character: 6,
            })
        );

        // Bare line number, no column.
        assert_eq!(
            parse_uri_fragment_position("5"),
            Some(lsp::Position {
                line: 4,
                character: 0,
            })
        );

        // Garbage / unparseable / 0-based fragments are rejected.
        assert_eq!(parse_uri_fragment_position(""), None);
        assert_eq!(parse_uri_fragment_position("section-name"), None);
        assert_eq!(parse_uri_fragment_position("0,0"), None);
    }

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
            HoverLink::Url(url) => assert_eq!(url, target),
            other => panic!("expected Url variant, got {other:?}"),
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
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                ..Default::default()
            },
            cx,
        )
        .await;

        cx.set_state(indoc! {"A cool ˇwebpage is https://zed.dev/releases"});

        let screen_coord =
            cx.pixel_position(indoc! {"A cool webpage is https://zed.dev/releˇases"});

        cx.simulate_mouse_move(screen_coord, None, Modifiers::secondary_key());
        cx.assert_editor_text_highlights(
            HighlightKey::HoveredLinkState,
            indoc! {"A cool webpage is «https://zed.dev/releasesˇ»"},
        );

        cx.simulate_click(screen_coord, Modifiers::secondary_key());
        assert_eq!(cx.opened_url(), Some("https://zed.dev/releases".into()));
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
        // VSCode behavior: when a position carries multiple link sources
        // (LSP document link, go-to-definition, ...), cmd-click should reveal
        // every applicable target. We assert this by inspecting the
        // aggregated `hovered_link_state.links` after a cmd-hover.
        init_test(cx, |_| {});

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                document_link_provider: Some(lsp::DocumentLinkOptions {
                    resolve_provider: Some(false),
                    work_done_progress_options: lsp::WorkDoneProgressOptions::default(),
                }),
                definition_provider: Some(lsp::OneOf::Left(true)),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        cx.set_state(indoc! {"
            // See LICENSE for details
            fn definition() {}ˇ
        "});

        let link_range = cx.lsp_range(indoc! {"
            // See «LICENSE» for details
            fn definition() {}
        "});
        let definition_target_range = cx.lsp_range(indoc! {"
            // See LICENSE for details
            fn «definition»() {}
        "});

        let mut document_link_requests = cx
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

        let mut definition_requests =
            cx.set_request_handler::<GotoDefinition, _, _>(move |url, _, _| async move {
                Ok(Some(lsp::GotoDefinitionResponse::Link(vec![
                    lsp::LocationLink {
                        origin_selection_range: Some(link_range),
                        target_uri: url.clone(),
                        target_range: definition_target_range,
                        target_selection_range: definition_target_range,
                    },
                ])))
            });

        cx.run_until_parked();
        document_link_requests.next().await;
        cx.run_until_parked();

        let screen_coord = cx.pixel_position(indoc! {"
            // See LICˇENSE for details
            fn definition() {}
        "});
        cx.simulate_mouse_move(screen_coord, None, Modifiers::secondary_key());
        definition_requests.next().await;
        cx.run_until_parked();

        cx.update_editor(|editor, _, _| {
            let links = &editor
                .hovered_link_state
                .as_ref()
                .expect("cmd-hover should populate `hovered_link_state`")
                .links;
            let url_count = links
                .iter()
                .filter(|link| matches!(link, HoverLink::Url(_)))
                .count();
            let text_count = links
                .iter()
                .filter(|link| matches!(link, HoverLink::Text(_)))
                .count();
            assert_eq!(
                url_count, 1,
                "document link should contribute exactly one Url hover link, got {links:?}"
            );
            assert_eq!(
                text_count, 1,
                "go-to-definition should contribute exactly one Text hover link, got {links:?}"
            );
        });

        // Cmd-click resolves the in-buffer location (definition) since the
        // mixed Url + Text case lets `navigate_to_hover_links` prefer the
        // location target over the external URL.
        cx.simulate_click(screen_coord, Modifiers::secondary_key());
        cx.run_until_parked();
        cx.assert_editor_state(indoc! {"
            // See LICENSE for details
            fn «definitionˇ»() {}
        "});
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
}
