use crate::{
    Anchor, Editor, EditorSettings, EditorSnapshot, FindAllReferences, GoToDefinition,
    GoToDefinitionSplit, GoToTypeDefinition, GoToTypeDefinitionSplit, GotoDefinitionKind,
    Navigated, PointForPosition, SelectPhase, editor_settings::GoToDefinitionFallback,
    scroll::ScrollAmount,
};
use gpui::{App, AsyncWindowContext, Context, Entity, Modifiers, Task, Window, px};
use language::{Bias, ToOffset};
use linkify::{LinkFinder, LinkKind};
use lsp::LanguageServerId;
use project::{InlayId, LocationLink, Project, ResolvedPath};
use settings::Settings;
use std::ops::Range;
use theme::ActiveTheme as _;
use util::{ResultExt, TryFutureExt as _, maybe};

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
    File(ResolvedPath),
    Text(LocationLink),
    InlayHint(lsp::Location, LanguageServerId),
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
        snapshot: &EditorSnapshot,
        modifiers: Modifiers,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let hovered_link_modifier = Editor::multi_cursor_modifier(false, &modifiers, cx);
        if !hovered_link_modifier || self.has_pending_selection() {
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
        self.clear_highlights::<HoveredLinkState>(cx);
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
                            editor.find_all_references(&FindAllReferences, window, cx)
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
                    window.focus(&self.focus_handle);
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
                let navigate_task =
                    self.navigate_to_hover_links(None, links, modifiers.alt, window, cx);
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
            match (modifiers.shift, modifiers.alt) {
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

    let trigger_anchor = trigger_point.anchor();
    let anchor = snapshot.buffer_snapshot().anchor_before(*trigger_anchor);
    let Some(buffer) = editor.buffer().read(cx).buffer_for_anchor(anchor, cx) else {
        return;
    };
    let Anchor {
        excerpt_id,
        text_anchor,
        ..
    } = anchor;
    let same_kind = hovered_link_state.preferred_kind == preferred_kind
        || hovered_link_state
            .links
            .first()
            .is_some_and(|d| matches!(d, HoverLink::Url(_)));

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

    let snapshot = snapshot.buffer_snapshot().clone();
    hovered_link_state.task = Some(cx.spawn_in(window, async move |this, cx| {
        async move {
            let result = match &trigger_point {
                TriggerPoint::Text(_) => {
                    if let Some((url_range, url)) = find_url(&buffer, text_anchor, cx.clone()) {
                        this.read_with(cx, |_, _| {
                            let range = maybe!({
                                let range =
                                    snapshot.anchor_range_in_excerpt(excerpt_id, url_range)?;
                                Some(RangeInEditor::Text(range))
                            });
                            (range, vec![HoverLink::Url(url)])
                        })
                        .ok()
                    } else if let Some((filename_range, filename)) =
                        find_file(&buffer, project.clone(), text_anchor, cx).await
                    {
                        let range = maybe!({
                            let range =
                                snapshot.anchor_range_in_excerpt(excerpt_id, filename_range)?;
                            Some(RangeInEditor::Text(range))
                        });

                        Some((range, vec![HoverLink::File(filename)]))
                    } else if let Some(provider) = provider {
                        let task = cx.update(|_, cx| {
                            provider.definitions(&buffer, text_anchor, preferred_kind, cx)
                        })?;
                        if let Some(task) = task {
                            task.await.ok().flatten().map(|definition_result| {
                                (
                                    definition_result.iter().find_map(|link| {
                                        link.origin.as_ref().and_then(|origin| {
                                            let range = snapshot.anchor_range_in_excerpt(
                                                excerpt_id,
                                                origin.range.clone(),
                                            )?;
                                            Some(RangeInEditor::Text(range))
                                        })
                                    }),
                                    definition_result.into_iter().map(HoverLink::Text).collect(),
                                )
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                TriggerPoint::InlayHint(highlight, lsp_location, server_id) => Some((
                    Some(RangeInEditor::Inlay(highlight.clone())),
                    vec![HoverLink::InlayHint(lsp_location.clone(), *server_id)],
                )),
            };

            this.update(cx, |editor, cx| {
                // Clear any existing highlights
                editor.clear_highlights::<HoveredLinkState>(cx);
                let Some(hovered_link_state) = editor.hovered_link_state.as_mut() else {
                    editor.hide_hovered_link(cx);
                    return;
                };
                hovered_link_state.preferred_kind = preferred_kind;
                hovered_link_state.symbol_range = result
                    .as_ref()
                    .and_then(|(symbol_range, _)| symbol_range.clone());

                if let Some((symbol_range, definitions)) = result {
                    hovered_link_state.links = definitions;

                    let underline_hovered_link = !hovered_link_state.links.is_empty()
                        || hovered_link_state.symbol_range.is_some();

                    if underline_hovered_link {
                        let style = gpui::HighlightStyle {
                            underline: Some(gpui::UnderlineStyle {
                                thickness: px(1.),
                                ..Default::default()
                            }),
                            color: Some(cx.theme().colors().link_text_hover),
                            ..Default::default()
                        };
                        let highlight_range =
                            symbol_range.unwrap_or_else(|| match &trigger_point {
                                TriggerPoint::Text(trigger_anchor) => {
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
                            RangeInEditor::Text(text_range) => editor
                                .highlight_text::<HoveredLinkState>(vec![text_range], style, cx),
                            RangeInEditor::Inlay(highlight) => editor
                                .highlight_inlays::<HoveredLinkState>(vec![highlight], style, cx),
                        }
                    }
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
    cx: AsyncWindowContext,
) -> Option<(Range<text::Anchor>, String)> {
    const LIMIT: usize = 2048;

    let snapshot = buffer.read_with(&cx, |buffer, _| buffer.snapshot()).ok()?;

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
    cx: AsyncWindowContext,
) -> Option<String> {
    const LIMIT: usize = 2048;

    let Ok(snapshot) = buffer.read_with(&cx, |buffer, _| buffer.snapshot()) else {
        return None;
    };

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

pub(crate) async fn find_file(
    buffer: &Entity<language::Buffer>,
    project: Option<Entity<Project>>,
    position: text::Anchor,
    cx: &mut AsyncWindowContext,
) -> Option<(Range<text::Anchor>, ResolvedPath)> {
    let project = project?;
    let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot()).ok()?;
    let scope = snapshot.language_scope_at(position);
    let (range, candidate_file_path) = surrounding_filename(snapshot, position)?;

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
            .ok()?
            .await
            .filter(|s| s.is_file())
    }

    if let Some(existing_path) = check_path(&candidate_file_path, &project, buffer, cx).await {
        return Some((range, existing_path));
    }

    if let Some(scope) = scope {
        for suffix in scope.path_suffixes() {
            if candidate_file_path.ends_with(format!(".{suffix}").as_str()) {
                continue;
            }

            let suffixed_candidate = format!("{candidate_file_path}.{suffix}");
            if let Some(existing_path) = check_path(&suffixed_candidate, &project, buffer, cx).await
            {
                return Some((range, existing_path));
            }
        }
    }

    None
}

fn surrounding_filename(
    snapshot: language::BufferSnapshot,
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
    use gpui::Modifiers;
    use indoc::indoc;
    use lsp::request::{GotoDefinition, GotoTypeDefinition};
    use settings::InlayHintSettingsContent;
    use util::{assert_set_eq, path};
    use workspace::item::Item;

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
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
            struct A;
            let «variable» = A;
        "});

        cx.simulate_modifiers_change(Modifiers::secondary_key());
        cx.run_until_parked();
        // Assert no link highlights
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
            struct A;
            let variable = A;
        "});

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
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
                fn test() { «do_work»(); }
                fn do_work() { test(); }
            "});

        // Unpress cmd causes highlight to go away
        cx.simulate_modifiers_change(Modifiers::none());
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
                fn test() { do_work(); }
                fn do_work() { test(); }
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
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
                fn test() { «do_work»(); }
                fn do_work() { test(); }
            "});

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
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
                fn test() { do_work(); }
                fn do_work() { test(); }
            "});

        // // Move mouse without cmd and then pressing cmd triggers highlight
        let hover_point = cx.pixel_position(indoc! {"
                fn test() { do_work(); }
                fn do_work() { teˇst(); }
            "});
        cx.simulate_mouse_move(hover_point, None, Modifiers::none());

        // Assert no link highlights
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
                fn test() { do_work(); }
                fn do_work() { test(); }
            "});

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

        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
                fn test() { do_work(); }
                fn do_work() { «test»(); }
            "});

        cx.deactivate_window();
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
                fn test() { do_work(); }
                fn do_work() { test(); }
            "});

        cx.simulate_mouse_move(hover_point, None, Modifiers::secondary_key());
        cx.background_executor.run_until_parked();
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
                fn test() { do_work(); }
                fn do_work() { «test»(); }
            "});

        // Moving again within the same symbol range doesn't re-request
        let hover_point = cx.pixel_position(indoc! {"
                fn test() { do_work(); }
                fn do_work() { tesˇt(); }
            "});
        cx.simulate_mouse_move(hover_point, None, Modifiers::secondary_key());
        cx.background_executor.run_until_parked();
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
                fn test() { do_work(); }
                fn do_work() { «test»(); }
            "});

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
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
                fn test() { do_work(); }
                fn do_work() { test(); }
            "});

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
            let anchor_range = snapshot.anchor_before(selection_range.start)
                ..snapshot.anchor_after(selection_range.end);
            editor.change_selections(Default::default(), window, cx, |s| {
                s.set_pending_anchor_range(anchor_range, crate::SelectMode::Character)
            });
        });
        cx.simulate_mouse_move(hover_point, None, Modifiers::secondary_key());
        cx.background_executor.run_until_parked();
        assert!(requests.try_next().is_err());
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
                fn test() { do_work(); }
                fn do_work() { test(); }
            "});
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
        let hint_position = cx.to_lsp(hint_start_offset);
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
            let previous_valid = inlay_range.start.to_display_point(&snapshot);
            let next_valid = inlay_range.end.to_display_point(&snapshot);
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
                .inlay_highlights::<HoveredLinkState>()
                .into_iter()
                .flat_map(|highlights| highlights.values().map(|(_, highlight)| highlight))
                .collect::<Vec<_>>();

            let buffer_snapshot = editor.buffer().update(cx, |buffer, cx| buffer.snapshot(cx));
            let expected_highlight = InlayHighlight {
                inlay: InlayId::Hint(0),
                inlay_position: buffer_snapshot.anchor_after(inlay_range.start),
                range: 0..hint_label.len(),
            };
            assert_set_eq!(actual_highlights, vec![&expected_highlight]);
        });

        cx.simulate_mouse_move(hover_point, None, Modifiers::none());
        // Assert no link highlights
        cx.update_editor(|editor, window, cx| {
                let snapshot = editor.snapshot(window, cx);
                let actual_ranges = snapshot
                    .text_highlight_ranges::<HoveredLinkState>()
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
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
            Let's test a [complex](«https://zed.dev/channel/had-(oops)ˇ») case.
        "});

        cx.simulate_click(screen_coord, Modifiers::secondary_key());
        assert_eq!(
            cx.opened_url(),
            Some("https://zed.dev/channel/had-(oops)".into())
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
        cx.assert_editor_text_highlights::<HoveredLinkState>(
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
        cx.assert_editor_text_highlights::<HoveredLinkState>(
            indoc! {"A cool webpage is «https://zed.dev/releasesˇ»"},
        );

        cx.simulate_click(screen_coord, Modifiers::secondary_key());
        assert_eq!(cx.opened_url(), Some("https://zed.dev/releases".into()));
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
        ];

        for (input, expected) in test_cases {
            cx.set_state(input);

            let (position, snapshot) = cx.editor(|editor, _, cx| {
                let positions = editor.selections.newest_anchor().head().text_anchor;
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

            let result = surrounding_filename(snapshot, position);

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

        #[cfg(not(target_os = "windows"))]
        cx.set_state(indoc! {"
            You can't go to a file that does_not_exist.txt.
            Go to file2.rs if you want.
            Or go to ../dir/file2.rs if you want.
            Or go to /root/dir/file2.rs if project is local.
            Or go to /root/dir/file2 if this is a Rust file.ˇ
            "});
        #[cfg(target_os = "windows")]
        cx.set_state(indoc! {"
            You can't go to a file that does_not_exist.txt.
            Go to file2.rs if you want.
            Or go to ../dir/file2.rs if you want.
            Or go to C:/root/dir/file2.rs if project is local.
            Or go to C:/root/dir/file2 if this is a Rust file.ˇ
        "});

        // File does not exist
        #[cfg(not(target_os = "windows"))]
        let screen_coord = cx.pixel_position(indoc! {"
            You can't go to a file that dˇoes_not_exist.txt.
            Go to file2.rs if you want.
            Or go to ../dir/file2.rs if you want.
            Or go to /root/dir/file2.rs if project is local.
            Or go to /root/dir/file2 if this is a Rust file.
        "});
        #[cfg(target_os = "windows")]
        let screen_coord = cx.pixel_position(indoc! {"
            You can't go to a file that dˇoes_not_exist.txt.
            Go to file2.rs if you want.
            Or go to ../dir/file2.rs if you want.
            Or go to C:/root/dir/file2.rs if project is local.
            Or go to C:/root/dir/file2 if this is a Rust file.
        "});
        cx.simulate_mouse_move(screen_coord, None, Modifiers::secondary_key());
        // No highlight
        cx.update_editor(|editor, window, cx| {
            assert!(
                editor
                    .snapshot(window, cx)
                    .text_highlight_ranges::<HoveredLinkState>()
                    .unwrap_or_default()
                    .1
                    .is_empty()
            );
        });

        // Moving the mouse over a file that does exist should highlight it.
        #[cfg(not(target_os = "windows"))]
        let screen_coord = cx.pixel_position(indoc! {"
            You can't go to a file that does_not_exist.txt.
            Go to fˇile2.rs if you want.
            Or go to ../dir/file2.rs if you want.
            Or go to /root/dir/file2.rs if project is local.
            Or go to /root/dir/file2 if this is a Rust file.
        "});
        #[cfg(target_os = "windows")]
        let screen_coord = cx.pixel_position(indoc! {"
            You can't go to a file that does_not_exist.txt.
            Go to fˇile2.rs if you want.
            Or go to ../dir/file2.rs if you want.
            Or go to C:/root/dir/file2.rs if project is local.
            Or go to C:/root/dir/file2 if this is a Rust file.
        "});

        cx.simulate_mouse_move(screen_coord, None, Modifiers::secondary_key());
        #[cfg(not(target_os = "windows"))]
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
            You can't go to a file that does_not_exist.txt.
            Go to «file2.rsˇ» if you want.
            Or go to ../dir/file2.rs if you want.
            Or go to /root/dir/file2.rs if project is local.
            Or go to /root/dir/file2 if this is a Rust file.
        "});
        #[cfg(target_os = "windows")]
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
            You can't go to a file that does_not_exist.txt.
            Go to «file2.rsˇ» if you want.
            Or go to ../dir/file2.rs if you want.
            Or go to C:/root/dir/file2.rs if project is local.
            Or go to C:/root/dir/file2 if this is a Rust file.
        "});

        // Moving the mouse over a relative path that does exist should highlight it
        #[cfg(not(target_os = "windows"))]
        let screen_coord = cx.pixel_position(indoc! {"
            You can't go to a file that does_not_exist.txt.
            Go to file2.rs if you want.
            Or go to ../dir/fˇile2.rs if you want.
            Or go to /root/dir/file2.rs if project is local.
            Or go to /root/dir/file2 if this is a Rust file.
        "});
        #[cfg(target_os = "windows")]
        let screen_coord = cx.pixel_position(indoc! {"
            You can't go to a file that does_not_exist.txt.
            Go to file2.rs if you want.
            Or go to ../dir/fˇile2.rs if you want.
            Or go to C:/root/dir/file2.rs if project is local.
            Or go to C:/root/dir/file2 if this is a Rust file.
        "});

        cx.simulate_mouse_move(screen_coord, None, Modifiers::secondary_key());
        #[cfg(not(target_os = "windows"))]
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
            You can't go to a file that does_not_exist.txt.
            Go to file2.rs if you want.
            Or go to «../dir/file2.rsˇ» if you want.
            Or go to /root/dir/file2.rs if project is local.
            Or go to /root/dir/file2 if this is a Rust file.
        "});
        #[cfg(target_os = "windows")]
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
            You can't go to a file that does_not_exist.txt.
            Go to file2.rs if you want.
            Or go to «../dir/file2.rsˇ» if you want.
            Or go to C:/root/dir/file2.rs if project is local.
            Or go to C:/root/dir/file2 if this is a Rust file.
        "});

        // Moving the mouse over an absolute path that does exist should highlight it
        #[cfg(not(target_os = "windows"))]
        let screen_coord = cx.pixel_position(indoc! {"
            You can't go to a file that does_not_exist.txt.
            Go to file2.rs if you want.
            Or go to ../dir/file2.rs if you want.
            Or go to /root/diˇr/file2.rs if project is local.
            Or go to /root/dir/file2 if this is a Rust file.
        "});

        #[cfg(target_os = "windows")]
        let screen_coord = cx.pixel_position(indoc! {"
            You can't go to a file that does_not_exist.txt.
            Go to file2.rs if you want.
            Or go to ../dir/file2.rs if you want.
            Or go to C:/root/diˇr/file2.rs if project is local.
            Or go to C:/root/dir/file2 if this is a Rust file.
        "});

        cx.simulate_mouse_move(screen_coord, None, Modifiers::secondary_key());
        #[cfg(not(target_os = "windows"))]
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
            You can't go to a file that does_not_exist.txt.
            Go to file2.rs if you want.
            Or go to ../dir/file2.rs if you want.
            Or go to «/root/dir/file2.rsˇ» if project is local.
            Or go to /root/dir/file2 if this is a Rust file.
        "});
        #[cfg(target_os = "windows")]
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
            You can't go to a file that does_not_exist.txt.
            Go to file2.rs if you want.
            Or go to ../dir/file2.rs if you want.
            Or go to «C:/root/dir/file2.rsˇ» if project is local.
            Or go to C:/root/dir/file2 if this is a Rust file.
        "});

        // Moving the mouse over a path that exists, if we add the language-specific suffix, it should highlight it
        #[cfg(not(target_os = "windows"))]
        let screen_coord = cx.pixel_position(indoc! {"
            You can't go to a file that does_not_exist.txt.
            Go to file2.rs if you want.
            Or go to ../dir/file2.rs if you want.
            Or go to /root/dir/file2.rs if project is local.
            Or go to /root/diˇr/file2 if this is a Rust file.
        "});
        #[cfg(target_os = "windows")]
        let screen_coord = cx.pixel_position(indoc! {"
            You can't go to a file that does_not_exist.txt.
            Go to file2.rs if you want.
            Or go to ../dir/file2.rs if you want.
            Or go to C:/root/dir/file2.rs if project is local.
            Or go to C:/root/diˇr/file2 if this is a Rust file.
        "});

        cx.simulate_mouse_move(screen_coord, None, Modifiers::secondary_key());
        #[cfg(not(target_os = "windows"))]
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
            You can't go to a file that does_not_exist.txt.
            Go to file2.rs if you want.
            Or go to ../dir/file2.rs if you want.
            Or go to /root/dir/file2.rs if project is local.
            Or go to «/root/dir/file2ˇ» if this is a Rust file.
        "});
        #[cfg(target_os = "windows")]
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
            You can't go to a file that does_not_exist.txt.
            Go to file2.rs if you want.
            Or go to ../dir/file2.rs if you want.
            Or go to C:/root/dir/file2.rs if project is local.
            Or go to «C:/root/dir/file2ˇ» if this is a Rust file.
        "});

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
                    .text_highlight_ranges::<HoveredLinkState>()
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
                    .text_highlight_ranges::<HoveredLinkState>()
                    .unwrap_or_default()
                    .1
                    .is_empty()
            );
        });

        // Does not open the directory
        cx.simulate_click(screen_coord, Modifiers::secondary_key());
        cx.update_workspace(|workspace, _, cx| assert_eq!(workspace.items(cx).count(), 1));
    }
}
