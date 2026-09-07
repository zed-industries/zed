use std::ops::Range;
use std::time::Duration;

use futures::future::join_all;
use gpui::{App, AsyncWindowContext, Context, Entity, Global, SharedString, Task, Window};
use language::{Buffer, Language};
use lsp::LanguageServerId;
use multi_buffer::{
    Anchor, MultiBufferOffset, MultiBufferRow, MultiBufferSnapshot, ToOffset, ToPoint,
};
use project::Project;
use project::lsp_store::emmet_ext::{EMMET_SERVER_NAME, ExpandAbbreviation};
use snippet::Snippet;
use text::Point;
use util::ResultExt as _;

use crate::scroll::Autoscroll;
use crate::{
    Editor, InlineInputPreview, SelectionEffects, WrapWithAbbreviation, element::register_action,
};

const PREVIEW_DEBOUNCE: Duration = Duration::from_millis(150);
const MAX_HISTORY_ENTRIES: usize = 20;

const ELEMENT_NODE_KINDS: [&str; 5] = [
    "element",
    "jsx_element",
    "jsx_self_closing_element",
    "script_element",
    "style_element",
];

const TAG_NODE_KINDS: [&str; 5] = [
    "start_tag",
    "end_tag",
    "self_closing_tag",
    "jsx_opening_element",
    "jsx_closing_element",
];

const STYLESHEET_LANGUAGE_IDS: [&str; 6] = ["css", "sass", "scss", "less", "sss", "stylus"];

#[derive(Clone, Copy, Default)]
struct WrapFilters {
    trim: bool,
    comment: bool,
    bem: bool,
}

#[derive(Default)]
struct WrapAbbreviationHistory(Vec<SharedString>);

impl Global for WrapAbbreviationHistory {}

pub fn apply_related_actions(editor: &Entity<Editor>, window: &mut Window, cx: &mut App) {
    let editor_ref = editor.read(cx);
    let Some(project) = editor_ref.project() else {
        return;
    };
    let project = project.read(cx);
    if !project
        .language_server_statuses(cx)
        .any(|(_, status)| status.name == EMMET_SERVER_NAME)
    {
        return;
    }
    let supports_wrap = editor_ref
        .buffer()
        .read(cx)
        .all_buffers_iter()
        .any(|buffer| {
            buffer
                .read(cx)
                .language()
                .is_some_and(|language| language_supports_wrap(language, project))
        });
    if supports_wrap {
        register_action(editor, window, wrap_with_abbreviation);
    }
}

pub fn wrap_with_abbreviation(
    editor: &mut Editor,
    _: &WrapWithAbbreviation,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    if editor.read_only(cx) {
        return;
    }

    let selections = editor.selections.disjoint_anchors_arc();
    let snapshot = editor.buffer().read(cx).snapshot(cx);
    let mut offset_ranges = Vec::<Range<MultiBufferOffset>>::new();
    for selection in selections.iter() {
        let mut range = selection.start.to_offset(&snapshot)..selection.end.to_offset(&snapshot);
        if range.start == range.end {
            range = enclosing_element_range(range.start, &snapshot).unwrap_or_else(|| {
                let row = MultiBufferRow(selection.start.to_point(&snapshot).row);
                Point::new(row.0, 0).to_offset(&snapshot)
                    ..Point::new(row.0, snapshot.line_len(row)).to_offset(&snapshot)
            });
        } else {
            range = expand_over_partial_tags(range, &snapshot);
        }
        offset_ranges.push(trim_whitespace(range, &snapshot));
    }
    offset_ranges.sort_by_key(|range| (range.start, range.end));
    let mut merged_ranges = Vec::<Range<MultiBufferOffset>>::with_capacity(offset_ranges.len());
    for range in offset_ranges {
        match merged_ranges.last_mut() {
            Some(last) if range.start < last.end || range.start == last.start => {
                last.end = last.end.max(range.end);
            }
            _ => merged_ranges.push(range),
        }
    }
    let ranges = merged_ranges
        .into_iter()
        .map(|range| snapshot.anchor_before(range.start)..snapshot.anchor_after(range.end))
        .collect::<Vec<_>>();
    if ranges.is_empty() {
        return;
    }
    if !ranges.iter().any(|range| {
        build_wrap_target(editor, range, WrapFilters::default(), &snapshot, cx).is_some()
    }) {
        log_unavailable();
        return;
    }

    editor.change_selections(SelectionEffects::default(), window, cx, |selections| {
        selections.select_anchor_ranges(ranges.iter().cloned())
    });
    let newest = editor.selections.newest_anchor();
    let start_column = newest.start.to_point(&snapshot).column;
    let end_row = MultiBufferRow(newest.end.to_point(&snapshot).row);
    let position = snapshot.anchor_before(Point::new(
        end_row.0,
        start_column.min(snapshot.line_len(end_row)),
    ));
    let preview_language = snapshot.language_at(position).cloned();
    let history = cx
        .try_global::<WrapAbbreviationHistory>()
        .map(|history| history.0.clone())
        .unwrap_or_default();
    let confirm_ranges = ranges.clone();
    let preview_ranges = ranges;
    editor.show_inline_input(
        "Emmet abbreviation, e.g. ul>li*",
        position,
        preview_language,
        history,
        move |editor, text, window, cx| {
            wrap_targets_in_expanded_abbreviation(editor, &confirm_ranges, text, window, cx)
        },
        move |editor, text, window, cx| {
            update_expansion_preview(editor, &preview_ranges, text, window, cx)
        },
        window,
        cx,
    );
}

struct WrapTarget {
    range: Range<Anchor>,
    text: Option<Vec<String>>,
    buffer: Entity<Buffer>,
    server_id: LanguageServerId,
    language: String,
    indent: String,
    base_indent: String,
}

fn language_supports_wrap(language: &Language, project: &Project) -> bool {
    project
        .languages()
        .lsp_adapter(&language.name(), &EMMET_SERVER_NAME)
        .is_some_and(|adapter| {
            !STYLESHEET_LANGUAGE_IDS.contains(&adapter.language_id(&language.name()).as_str())
        })
}

fn build_wrap_target(
    editor: &Editor,
    range: &Range<Anchor>,
    filters: WrapFilters,
    snapshot: &MultiBufferSnapshot,
    cx: &App,
) -> Option<WrapTarget> {
    let project = editor.project.as_ref()?.read(cx);
    let multi_buffer = editor.buffer().read(cx);
    let (buffer, start) = multi_buffer.text_anchor_for_position(range.start, cx)?;
    let (end_buffer, _) = multi_buffer.text_anchor_for_position(range.end, cx)?;
    if buffer != end_buffer {
        return None;
    }
    let server_id = project.language_server_id_for_name(buffer.read(cx), &EMMET_SERVER_NAME, cx)?;
    let language = buffer
        .read(cx)
        .language_at(start)
        .or_else(|| buffer.read(cx).language().cloned())?;
    let adapter = project
        .lsp_store()
        .read(cx)
        .language_server_adapter_for_id(server_id)?;
    let language = adapter.language_id(&language.name());
    if STYLESHEET_LANGUAGE_IDS.contains(&language.as_str()) {
        return None;
    }
    let text = snapshot.text_for_range(range.clone()).collect::<String>();
    let settings = snapshot.language_settings_at(range.start, cx);
    let indent = if settings.hard_tabs {
        "\t".to_string()
    } else {
        " ".repeat(settings.tab_size.get() as usize)
    };
    let base_indent = base_indent_for(range.start, snapshot);
    Some(WrapTarget {
        text: selection_lines(&text, &base_indent, filters.trim),
        range: range.clone(),
        buffer,
        server_id,
        language,
        indent,
        base_indent,
    })
}

fn selection_lines(text: &str, base_indent: &str, trim_markers: bool) -> Option<Vec<String>> {
    if text.is_empty() {
        return None;
    }
    let lines = text
        .split('\n')
        .enumerate()
        .map(|(ix, line)| {
            let line = line.trim_end();
            let line = if ix == 0 {
                line
            } else {
                line.strip_prefix(base_indent).unwrap_or(line)
            };
            let line = if trim_markers {
                trim_list_marker(line)
            } else {
                line
            };
            line.replace('\\', "\\\\").replace('$', "\\$")
        })
        .collect::<Vec<_>>();
    Some(lines)
}

fn trim_list_marker(line: &str) -> &str {
    let trimmed = line.trim_start_matches([' ', '\t', '\u{a0}']);
    let marker_len = trimmed
        .chars()
        .take_while(|ch| ch.is_ascii_digit() || matches!(ch, '#' | '-' | '*' | '•'))
        .map(char::len_utf8)
        .sum::<usize>();
    if marker_len == 0 {
        return line.trim();
    }
    let rest = &trimmed[marker_len..];
    let rest = rest.strip_prefix(['.', ')']).unwrap_or(rest);
    rest.trim()
}

fn parse_abbreviation(input: &str) -> Option<(String, WrapFilters)> {
    let mut abbreviation = input.trim();
    let mut filters = WrapFilters::default();
    loop {
        if let Some(stripped) = abbreviation.strip_suffix("|t") {
            filters.trim = true;
            abbreviation = stripped.trim_end();
        } else if let Some(stripped) = abbreviation.strip_suffix("|c") {
            filters.comment = true;
            abbreviation = stripped.trim_end();
        } else if let Some(stripped) = abbreviation.strip_suffix("|bem") {
            filters.bem = true;
            abbreviation = stripped.trim_end();
        } else {
            break;
        }
    }
    if abbreviation.is_empty() {
        return None;
    }
    Some((abbreviation.to_string(), filters))
}

fn wrap_targets_in_expanded_abbreviation(
    editor: &mut Editor,
    ranges: &[Range<Anchor>],
    text: String,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> Option<Task<()>> {
    let input = text.trim().to_string();
    let (abbreviation, filters) = parse_abbreviation(&input)?;
    let project = editor.project.clone()?;

    let snapshot = editor.buffer().read(cx).snapshot(cx);
    let targets = ranges
        .iter()
        .filter_map(|range| build_wrap_target(editor, range, filters, &snapshot, cx))
        .collect::<Vec<_>>();
    if targets.is_empty() {
        log_unavailable();
        editor.take_inline_input(window, cx);
        return None;
    }

    Some(cx.spawn_in(window, async move |editor, cx| {
        let requests = targets
            .into_iter()
            .map(|target| {
                let range = target.range.clone();
                let request =
                    request_expansion(&project, abbreviation.clone(), filters, target, cx);
                async move { anyhow::Ok((range, request.await?)) }
            })
            .collect::<Vec<_>>();
        let mut expansions = Vec::new();
        let mut errors = Vec::new();
        for result in join_all(requests).await {
            match result {
                Ok((range, Some(expansion))) => expansions.push((range, expansion)),
                Ok((_, None)) => {}
                Err(error) => errors.push(error),
            }
        }

        editor
            .update_in(cx, |editor, window, cx| {
                for error in &errors {
                    log::error!("{error:#}");
                }
                if expansions.is_empty() {
                    let message = match errors.first() {
                        Some(error) => error.root_cause().to_string(),
                        None => no_expansion_message(&abbreviation),
                    };
                    editor.set_inline_input_preview(Some(InlineInputPreview::Error(message)), cx);
                    return anyhow::Ok(());
                }
                remember_abbreviation(SharedString::from(input), cx);
                if let Some(mut state) = editor.take_inline_input(window, cx)
                    && let Some(confirm_task) = state.take_confirm_task()
                {
                    confirm_task.detach();
                }
                let snapshot = editor.buffer().read(cx).snapshot(cx);
                if let [(range, expansion)] = expansions.as_slice() {
                    if let Ok(snippet) = Snippet::parse(expansion) {
                        let range =
                            range.start.to_offset(&snapshot)..range.end.to_offset(&snapshot);
                        editor.insert_snippet_with_autoindent(
                            &[range],
                            snippet,
                            None,
                            window,
                            cx,
                        )?;
                        return anyhow::Ok(());
                    }
                }
                let edits = expansions
                    .into_iter()
                    .map(|(range, expansion)| (range, expansion_text(&expansion)))
                    .collect::<Vec<_>>();
                editor.transact(window, cx, |editor, _, cx| {
                    editor.buffer().update(cx, |buffer, cx| {
                        buffer.edit(edits, None, cx);
                    });
                });
                editor.request_autoscroll(Autoscroll::fit(), cx);
                anyhow::Ok(())
            })
            .and_then(|result| result)
            .log_err();
    }))
}

fn update_expansion_preview(
    editor: &mut Editor,
    ranges: &[Range<Anchor>],
    text: String,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> Option<Task<()>> {
    let Some((abbreviation, filters)) = parse_abbreviation(&text) else {
        editor.set_inline_input_preview(None, cx);
        return None;
    };
    let Some(project) = editor.project.clone() else {
        editor.set_inline_input_preview(None, cx);
        return None;
    };
    let snapshot = editor.buffer().read(cx).snapshot(cx);
    let Some(mut target) = ranges
        .iter()
        .find_map(|range| build_wrap_target(editor, range, filters, &snapshot, cx))
    else {
        editor.set_inline_input_preview(None, cx);
        return None;
    };
    target.base_indent.clear();

    Some(cx.spawn_in(window, async move |editor, cx| {
        cx.background_executor().timer(PREVIEW_DEBOUNCE).await;
        let result = request_expansion(&project, abbreviation.clone(), filters, target, cx).await;
        editor
            .update(cx, |editor, cx| {
                let preview = match result {
                    Ok(Some(expansion)) => InlineInputPreview::Text(expansion_text(&expansion)),
                    Ok(None) => InlineInputPreview::Error(no_expansion_message(&abbreviation)),
                    Err(error) => InlineInputPreview::Error(error.root_cause().to_string()),
                };
                editor.set_inline_input_preview(Some(preview), cx);
            })
            .ok();
    }))
}

fn request_expansion(
    project: &Entity<Project>,
    abbreviation: String,
    filters: WrapFilters,
    target: WrapTarget,
    cx: &mut AsyncWindowContext,
) -> Task<anyhow::Result<Option<String>>> {
    project.update(cx, |project, cx| {
        project.request_lsp(
            target.buffer,
            project::LanguageServerToQuery::Other(target.server_id),
            ExpandAbbreviation {
                abbreviation,
                text: target.text,
                language: target.language,
                server_id: target.server_id,
                indent: target.indent,
                base_indent: target.base_indent,
                comment_filter: filters.comment,
                bem_filter: filters.bem,
            },
            cx,
        )
    })
}

fn expansion_text(expansion: &str) -> String {
    Snippet::parse(expansion)
        .map(|snippet| snippet.text)
        .unwrap_or_else(|_| expansion.to_string())
}

fn no_expansion_message(abbreviation: &str) -> String {
    format!("No Emmet expansion for {abbreviation:?}")
}

fn remember_abbreviation(abbreviation: SharedString, cx: &mut App) {
    let history = &mut cx.default_global::<WrapAbbreviationHistory>().0;
    history.retain(|entry| *entry != abbreviation);
    history.insert(0, abbreviation);
    history.truncate(MAX_HISTORY_ENTRIES);
}

fn log_unavailable() {
    log::info!("Wrapping with an Emmet abbreviation is not available for the selected text");
}

fn base_indent_for(range_start: Anchor, snapshot: &MultiBufferSnapshot) -> String {
    let row = MultiBufferRow(range_start.to_point(snapshot).row);
    snapshot.indent_size_for_line(row).chars().collect()
}

fn enclosing_element_range(
    position: MultiBufferOffset,
    snapshot: &MultiBufferSnapshot,
) -> Option<Range<MultiBufferOffset>> {
    let mut range = position..position;
    while let Some((node, node_range)) = snapshot.syntax_ancestor(range.clone()) {
        if ELEMENT_NODE_KINDS.contains(&node.kind()) {
            return Some(node_range);
        }
        if node_range == range {
            return None;
        }
        range = node_range;
    }
    None
}

fn expand_over_partial_tags(
    range: Range<MultiBufferOffset>,
    snapshot: &MultiBufferSnapshot,
) -> Range<MultiBufferOffset> {
    let mut expanded = range.clone();
    for position in [range.start, range.end] {
        if let Some(element_range) = element_range_for_partial_tag(position, snapshot) {
            expanded.start = expanded.start.min(element_range.start);
            expanded.end = expanded.end.max(element_range.end);
        }
    }
    expanded
}

fn element_range_for_partial_tag(
    position: MultiBufferOffset,
    snapshot: &MultiBufferSnapshot,
) -> Option<Range<MultiBufferOffset>> {
    let mut range = position..position;
    let mut in_tag = false;
    while let Some((node, node_range)) = snapshot.syntax_ancestor(range.clone()) {
        if TAG_NODE_KINDS.contains(&node.kind()) {
            if node_range.start < position && position < node_range.end {
                in_tag = true;
            }
        } else if ELEMENT_NODE_KINDS.contains(&node.kind()) {
            return in_tag.then_some(node_range);
        }
        if node_range == range {
            return None;
        }
        range = node_range;
    }
    None
}

fn trim_whitespace(
    mut range: Range<MultiBufferOffset>,
    snapshot: &MultiBufferSnapshot,
) -> Range<MultiBufferOffset> {
    for ch in snapshot.chars_at(range.start) {
        if range.start == range.end || !ch.is_whitespace() {
            break;
        }
        range.start += ch.len_utf8();
    }
    for ch in snapshot.reversed_chars_at(range.end) {
        if range.end == range.start || !ch.is_whitespace() {
            break;
        }
        range.end -= ch.len_utf8();
    }
    range
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use futures::StreamExt as _;
    use gpui::{AppContext as _, Focusable as _, TestAppContext, VisualTestContext};
    use language::{Capability, FakeLspAdapter, Language, LanguageConfig, LanguageMatcher};
    use multi_buffer::{MultiBuffer, PathKey};
    use project::lsp_store::emmet_ext::{EmmetOutputOptions, LspExpandAbbreviation};
    use project::{FakeFs, Project};
    use util::path;

    use crate::editor_tests::init_test;
    use crate::test::build_editor_with_project;

    use super::*;

    #[gpui::test]
    async fn test_wrap_selection_in_emmet_abbreviation(cx: &mut TestAppContext) {
        let (editor, mut fake_servers, cx) = setup("<p>hello</p>", cx).await;
        let fake_server = fake_servers.next().await.unwrap();
        cx.executor().run_until_parked();

        select_and_wrap(&editor, [Point::new(0, 3)..Point::new(0, 8)], cx);

        let mut requests = fake_server.set_request_handler::<LspExpandAbbreviation, _, _>(
            |params, _| async move {
                assert_eq!(params.abbreviation, "div.wrap");
                assert_eq!(params.language, "html");
                assert_eq!(params.options.text, Some(vec!["hello".to_string()]));
                assert_eq!(
                    params.options.options,
                    EmmetOutputOptions {
                        indent: "    ".to_string(),
                        base_indent: String::new(),
                        ..EmmetOutputOptions::default()
                    },
                    "single-line wraps should not force inline breaks"
                );
                Ok(Some("<div class=\"wrap\">${1:hello}</div>$0".to_string()))
            },
        );

        editor.update_in(cx, |editor, window, cx| {
            let input = pending_input(editor);
            assert!(input.focus_handle(cx).is_focused(window));
            input.update(cx, |input, cx| input.set_text("div.wrap", window, cx));
        });
        cx.update(|_, cx| {
            cx.bind_keys([gpui::KeyBinding::new(
                "enter",
                menu::Confirm,
                Some("Editor && inline_input"),
            )]);
        });
        cx.simulate_keystrokes("enter");
        requests.next().await.unwrap();
        cx.executor().run_until_parked();

        editor.update(cx, |editor, cx| {
            assert_eq!(editor.text(cx), "<p><div class=\"wrap\">hello</div></p>");
            assert!(editor.pending_inline_input.is_none());
        });
    }

    #[gpui::test]
    async fn test_input_stays_open_on_empty_expansion(cx: &mut TestAppContext) {
        let (editor, mut fake_servers, cx) = setup("<p>hello</p>", cx).await;
        let fake_server = fake_servers.next().await.unwrap();
        cx.executor().run_until_parked();

        select_and_wrap(&editor, [Point::new(0, 3)..Point::new(0, 8)], cx);

        let mut requests = fake_server
            .set_request_handler::<LspExpandAbbreviation, _, _>(|_, _| async move { Ok(None) });

        confirm_abbreviation(&editor, "@invalid@", cx);
        requests.next().await.unwrap();
        cx.executor().run_until_parked();

        editor.update(cx, |editor, cx| {
            assert_eq!(editor.text(cx), "<p>hello</p>");
            assert!(
                editor.pending_inline_input.is_some(),
                "the input should stay open so the abbreviation can be corrected"
            );
            assert_eq!(
                editor
                    .pending_inline_input
                    .as_ref()
                    .and_then(|state| state.preview.clone()),
                Some(InlineInputPreview::Error(
                    "No Emmet expansion for \"@invalid@\"".to_string()
                )),
                "the failure should show up next to the input"
            );
        });
    }

    #[gpui::test]
    async fn test_wrap_line_on_empty_selection(cx: &mut TestAppContext) {
        let (editor, mut fake_servers, cx) = setup("  hello world\n", cx).await;
        let fake_server = fake_servers.next().await.unwrap();
        cx.executor().run_until_parked();

        select_and_wrap(&editor, [Point::new(0, 4)..Point::new(0, 4)], cx);

        let mut requests = fake_server.set_request_handler::<LspExpandAbbreviation, _, _>(
            |params, _| async move {
                assert_eq!(params.abbreviation, "strong");
                assert_eq!(params.options.text, Some(vec!["hello world".to_string()]));
                Ok(Some("<strong>hello world</strong>".to_string()))
            },
        );

        confirm_abbreviation(&editor, "strong", cx);
        requests.next().await.unwrap();
        cx.executor().run_until_parked();

        editor.update(cx, |editor, cx| {
            assert_eq!(editor.text(cx), "  <strong>hello world</strong>\n");
        });
    }

    #[gpui::test]
    async fn test_wrap_enclosing_element_on_empty_selection(cx: &mut TestAppContext) {
        let (editor, mut fake_servers, cx) = setup("<div>\n  <p>hello</p>\n</div>", cx).await;
        let fake_server = fake_servers.next().await.unwrap();
        cx.executor().run_until_parked();

        select_and_wrap(&editor, [Point::new(1, 3)..Point::new(1, 3)], cx);
        editor.update(cx, |editor, cx| {
            let snapshot = editor.display_snapshot(cx);
            assert_eq!(
                editor.selections.newest::<Point>(&snapshot).range(),
                Point::new(1, 2)..Point::new(1, 14),
                "the element about to be wrapped should be selected so it is visible"
            );
        });

        let mut requests = fake_server.set_request_handler::<LspExpandAbbreviation, _, _>(
            |params, _| async move {
                assert_eq!(params.abbreviation, "div.x");
                assert_eq!(
                    params.options.text,
                    Some(vec!["<p>hello</p>".to_string()]),
                    "the enclosing element should be wrapped, not the line"
                );
                Ok(Some("<div class=\"x\"><p>hello</p></div>".to_string()))
            },
        );

        confirm_abbreviation(&editor, "div.x", cx);
        requests.next().await.unwrap();
        cx.executor().run_until_parked();

        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "<div>\n  <div class=\"x\"><p>hello</p></div>\n</div>"
            );
        });
    }

    #[gpui::test]
    async fn test_multiline_expansion_preserves_surrounding_indentation(cx: &mut TestAppContext) {
        let (editor, mut fake_servers, cx) =
            setup("<section>\n  <p>a</p>\n  <p>b</p>\n</section>", cx).await;
        let fake_server = fake_servers.next().await.unwrap();
        cx.executor().run_until_parked();

        select_and_wrap(&editor, [Point::new(1, 2)..Point::new(2, 10)], cx);

        let mut requests = fake_server.set_request_handler::<LspExpandAbbreviation, _, _>(
            |params, _| async move {
                assert_eq!(params.abbreviation, "div");
                assert_eq!(
                    params.options.text,
                    Some(vec!["<p>a</p>".to_string(), "<p>b</p>".to_string()]),
                    "lines should be dedented by the base indentation of the first line"
                );
                assert_eq!(
                    params.options.options.base_indent, "  ",
                    "the wrapped line's indentation should be sent as the base indent"
                );
                Ok(Some(format_expansion(
                    "<div>\n\t<p>a</p>\n\t<p>b</p>\n</div>",
                    &params.options.options,
                )))
            },
        );

        confirm_abbreviation(&editor, "div", cx);
        requests.next().await.unwrap();
        cx.executor().run_until_parked();

        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "<section>\n  <div>\n      <p>a</p>\n      <p>b</p>\n  </div>\n</section>",
                "inserted lines should be indented relative to the wrapped line, \
                 and surrounding indentation should not change"
            );
        });
    }

    #[gpui::test]
    async fn test_escape_dismisses_emmet_wrap_input(cx: &mut TestAppContext) {
        let (editor, mut fake_servers, cx) = setup("<p>hello</p>", cx).await;
        fake_servers.next().await.unwrap();
        cx.executor().run_until_parked();

        select_and_wrap(&editor, [Point::new(0, 3)..Point::new(0, 8)], cx);
        editor.update_in(cx, |editor, window, cx| {
            assert!(editor.pending_inline_input.is_some());
            editor.cancel(&crate::actions::Cancel, window, cx);
            assert!(editor.pending_inline_input.is_none());
            assert_eq!(editor.text(cx), "<p>hello</p>");
        });
    }

    #[gpui::test]
    async fn test_multiple_cursors_on_blank_line_wrap_once(cx: &mut TestAppContext) {
        let (editor, mut fake_servers, cx) = setup("   \n", cx).await;
        let fake_server = fake_servers.next().await.unwrap();
        cx.executor().run_until_parked();

        select_and_wrap(
            &editor,
            [
                Point::new(0, 0)..Point::new(0, 0),
                Point::new(0, 2)..Point::new(0, 2),
            ],
            cx,
        );

        let mut requests = fake_server.set_request_handler::<LspExpandAbbreviation, _, _>(
            |params, _| async move {
                assert_eq!(params.abbreviation, "div");
                assert_eq!(params.options.text, None);
                Ok(Some("<div></div>".to_string()))
            },
        );

        confirm_abbreviation(&editor, "div", cx);
        requests.next().await.unwrap();
        cx.executor().run_until_parked();

        editor.update(cx, |editor, cx| {
            assert_eq!(editor.text(cx), "   <div></div>\n");
            assert!(editor.pending_inline_input.is_none());
        });
    }

    #[gpui::test]
    async fn test_stale_confirmation_is_ignored(cx: &mut TestAppContext) {
        let (editor, mut fake_servers, cx) = setup("<p>hello</p>", cx).await;
        let fake_server = fake_servers.next().await.unwrap();
        cx.executor().run_until_parked();

        select_and_wrap(&editor, [Point::new(0, 3)..Point::new(0, 8)], cx);

        let (release_fresh, fresh_gate) = futures::channel::oneshot::channel::<()>();
        let fresh_gate = Arc::new(std::sync::Mutex::new(Some(fresh_gate)));
        fake_server.set_request_handler::<LspExpandAbbreviation, _, _>({
            move |params, _| {
                let fresh_gate = fresh_gate.clone();
                async move {
                    if params.abbreviation == "span" {
                        let gate = fresh_gate.lock().unwrap().take();
                        if let Some(gate) = gate {
                            gate.await.ok();
                        }
                        Ok(Some("<span>hello</span>".to_string()))
                    } else {
                        assert_eq!(params.abbreviation, "div");
                        Ok(Some("<div>hello</div>".to_string()))
                    }
                }
            }
        });

        confirm_abbreviation(&editor, "div", cx);
        confirm_abbreviation(&editor, "span", cx);
        cx.executor().run_until_parked();

        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "<p>hello</p>",
                "the stale expansion should be dropped while the fresh one is in flight"
            );
            assert!(editor.pending_inline_input.is_some());
        });

        release_fresh.send(()).ok();
        cx.executor().run_until_parked();

        editor.update(cx, |editor, cx| {
            assert_eq!(editor.text(cx), "<p><span>hello</span></p>");
            assert!(editor.pending_inline_input.is_none());
        });
    }

    #[gpui::test]
    async fn test_preview_updates_as_you_type(cx: &mut TestAppContext) {
        let (editor, mut fake_servers, cx) = setup("<p>hello</p>", cx).await;
        let fake_server = fake_servers.next().await.unwrap();
        cx.executor().run_until_parked();

        select_and_wrap(&editor, [Point::new(0, 3)..Point::new(0, 8)], cx);

        let mut requests = fake_server.set_request_handler::<LspExpandAbbreviation, _, _>(
            |params, _| async move {
                assert_eq!(params.abbreviation, "div.wrap");
                assert_eq!(params.options.text, Some(vec!["hello".to_string()]));
                Ok(Some("<div class=\"wrap\">${1:hello}</div>$0".to_string()))
            },
        );

        editor.update_in(cx, |editor, window, cx| {
            let input = pending_input(editor);
            input.update(cx, |input, cx| input.set_text("div.wrap", window, cx));
        });
        cx.executor().advance_clock(PREVIEW_DEBOUNCE * 2);
        requests.next().await.unwrap();
        cx.executor().run_until_parked();

        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor
                    .pending_inline_input
                    .as_ref()
                    .and_then(|state| state.preview.clone()),
                Some(InlineInputPreview::Text(
                    "<div class=\"wrap\">hello</div>".to_string()
                )),
                "the pending expansion should be previewed under the input"
            );
            assert_eq!(
                editor.text(cx),
                "<p>hello</p>",
                "previewing should not edit the buffer"
            );
        });

        editor.update_in(cx, |editor, window, cx| {
            let input = pending_input(editor);
            input.update(cx, |input, cx| input.set_text("", window, cx));
        });
        cx.executor().run_until_parked();
        editor.update(cx, |editor, _| {
            assert_eq!(
                editor
                    .pending_inline_input
                    .as_ref()
                    .and_then(|state| state.preview.clone()),
                None,
                "clearing the input should clear the preview"
            );
        });
    }

    #[gpui::test]
    async fn test_preview_does_not_embed_base_indentation(cx: &mut TestAppContext) {
        let (editor, mut fake_servers, cx) =
            setup("<section>\n  <p>a</p>\n  <p>b</p>\n</section>", cx).await;
        let fake_server = fake_servers.next().await.unwrap();
        cx.executor().run_until_parked();

        select_and_wrap(&editor, [Point::new(1, 2)..Point::new(2, 10)], cx);

        let mut requests = fake_server.set_request_handler::<LspExpandAbbreviation, _, _>(
            |params, _| async move {
                assert_eq!(params.abbreviation, "div");
                assert_eq!(
                    params.options.options.base_indent, "",
                    "previews render at the anchor column already, \
                     so they must request no base indent"
                );
                Ok(Some(format_expansion(
                    "<div>\n\t<p>a</p>\n\t<p>b</p>\n</div>",
                    &params.options.options,
                )))
            },
        );

        editor.update_in(cx, |editor, window, cx| {
            let input = pending_input(editor);
            input.update(cx, |input, cx| input.set_text("div", window, cx));
        });
        cx.executor().advance_clock(PREVIEW_DEBOUNCE * 2);
        requests.next().await.unwrap();
        cx.executor().run_until_parked();

        editor.update(cx, |editor, _| {
            assert_eq!(
                editor
                    .pending_inline_input
                    .as_ref()
                    .and_then(|state| state.preview.clone()),
                Some(InlineInputPreview::Text(
                    "<div>\n    <p>a</p>\n    <p>b</p>\n</div>".to_string()
                )),
                "the preview block already renders at the anchor column, \
                 so its lines must not repeat the wrapped line's base indentation"
            );
        });
    }

    #[gpui::test]
    async fn test_last_abbreviation_is_prefilled(cx: &mut TestAppContext) {
        let (editor, mut fake_servers, cx) = setup("<p>hello</p>", cx).await;
        let fake_server = fake_servers.next().await.unwrap();
        cx.executor().run_until_parked();

        select_and_wrap(&editor, [Point::new(0, 3)..Point::new(0, 8)], cx);

        let mut requests =
            fake_server.set_request_handler::<LspExpandAbbreviation, _, _>(|_, _| async move {
                Ok(Some("<div class=\"wrap\">hello</div>".to_string()))
            });

        confirm_abbreviation(&editor, "div.wrap", cx);
        requests.next().await.unwrap();
        cx.executor().run_until_parked();

        editor.update(cx, |editor, cx| {
            assert_eq!(editor.text(cx), "<p><div class=\"wrap\">hello</div></p>");
        });

        select_and_wrap(&editor, [Point::new(0, 3)..Point::new(0, 8)], cx);
        editor.update(cx, |editor, cx| {
            let input = pending_input(editor);
            input.update(cx, |input, cx| {
                assert_eq!(
                    input.text(cx),
                    "div.wrap",
                    "the last used abbreviation should be prefilled"
                );
                let snapshot = input.display_snapshot(cx);
                let selected = input.selections.newest::<Point>(&snapshot);
                assert_eq!(
                    selected.range(),
                    Point::new(0, 0)..Point::new(0, "div.wrap".len() as u32),
                    "the prefilled abbreviation should be selected so typing replaces it"
                );
            });
        });
    }

    #[gpui::test]
    async fn test_up_and_down_cycle_abbreviation_history(cx: &mut TestAppContext) {
        let (editor, mut fake_servers, cx) = setup("<p>hello</p>", cx).await;
        let fake_server = fake_servers.next().await.unwrap();
        cx.executor().run_until_parked();
        cx.update(|_, cx| {
            cx.bind_keys([
                gpui::KeyBinding::new("up", crate::MoveUp, Some("Editor")),
                gpui::KeyBinding::new("down", crate::MoveDown, Some("Editor")),
            ]);
        });
        let mut requests = fake_server.set_request_handler::<LspExpandAbbreviation, _, _>(
            |params, _| async move { Ok(Some(format!("<{0}>hello</{0}>", params.abbreviation))) },
        );

        for abbreviation in ["em", "strong", "span"] {
            let hello = editor.update(cx, |editor, cx| {
                let text = editor.text(cx);
                let start = text.find("hello").expect("hello should still be there") as u32;
                Point::new(0, start)..Point::new(0, start + "hello".len() as u32)
            });
            select_and_wrap(&editor, [hello], cx);
            confirm_abbreviation(&editor, abbreviation, cx);
            requests.next().await.unwrap();
            cx.executor().run_until_parked();
        }
        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "<p><em><strong><span>hello</span></strong></em></p>"
            );
        });

        select_and_wrap(&editor, [Point::new(0, 3)..Point::new(0, 8)], cx);
        assert_eq!(input_text(&editor, cx), "span", "newest entry is prefilled");

        cx.simulate_keystrokes("up");
        assert_eq!(
            input_text(&editor, cx),
            "strong",
            "up moves to the older entry"
        );
        cx.simulate_keystrokes("up");
        assert_eq!(input_text(&editor, cx), "em");
        cx.simulate_keystrokes("up");
        assert_eq!(
            input_text(&editor, cx),
            "em",
            "up stops at the oldest entry"
        );

        cx.simulate_keystrokes("down down down");
        assert_eq!(
            input_text(&editor, cx),
            "",
            "down past the newest entry returns to the empty draft"
        );
        cx.simulate_keystrokes("down");
        assert_eq!(input_text(&editor, cx), "", "down stops at the draft");

        editor.update_in(cx, |editor, window, cx| {
            let input = pending_input(editor);
            input.update(cx, |input, cx| input.set_text("div.wip", window, cx));
        });
        cx.executor().run_until_parked();
        cx.simulate_keystrokes("up");
        assert_eq!(
            input_text(&editor, cx),
            "span",
            "up from a draft starts at the newest entry"
        );
        cx.simulate_keystrokes("down");
        assert_eq!(
            input_text(&editor, cx),
            "div.wip",
            "down restores the draft that was being typed"
        );

        editor.update(cx, |editor, cx| {
            assert!(
                editor.pending_inline_input.is_some(),
                "cycling history must not dismiss the input"
            );
            assert_eq!(
                editor.text(cx),
                "<p><em><strong><span>hello</span></strong></em></p>"
            );
        });
    }

    #[gpui::test]
    async fn test_wrap_individual_lines(cx: &mut TestAppContext) {
        let (editor, mut fake_servers, cx) =
            setup("<div>\n    About\n    News\n    Products\n</div>", cx).await;
        let fake_server = fake_servers.next().await.unwrap();
        cx.executor().run_until_parked();

        select_and_wrap(&editor, [Point::new(1, 4)..Point::new(3, 12)], cx);

        let mut requests = fake_server.set_request_handler::<LspExpandAbbreviation, _, _>(
            |params, _| async move {
                assert_eq!(params.abbreviation, "ul>li*");
                assert_eq!(
                    params.options.text,
                    Some(vec![
                        "About".to_string(),
                        "News".to_string(),
                        "Products".to_string(),
                    ]),
                    "each selected line should be sent separately so `*` repeats per line"
                );
                assert_eq!(
                    params.options.options,
                    EmmetOutputOptions {
                        indent: "    ".to_string(),
                        base_indent: "    ".to_string(),
                        inline_break: Some(1),
                        ..EmmetOutputOptions::default()
                    },
                    "multi-line wraps should break inline elements per line \
                     and carry the buffer's indentation to the server"
                );
                Ok(Some(format_expansion(
                    "<ul>\n\t<li>About</li>\n\t<li>News</li>\n\t<li>Products</li>\n</ul>",
                    &params.options.options,
                )))
            },
        );

        confirm_abbreviation(&editor, "ul>li*", cx);
        requests.next().await.unwrap();
        cx.executor().run_until_parked();

        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "<div>\n    <ul>\n        <li>About</li>\n        <li>News</li>\n        <li>Products</li>\n    </ul>\n</div>"
            );
        });
    }

    #[gpui::test]
    async fn test_trim_filter_strips_list_markers(cx: &mut TestAppContext) {
        let (editor, mut fake_servers, cx) = setup("  * one\n  2. two\n", cx).await;
        let fake_server = fake_servers.next().await.unwrap();
        cx.executor().run_until_parked();

        select_and_wrap(&editor, [Point::new(0, 0)..Point::new(1, 8)], cx);

        let mut requests = fake_server.set_request_handler::<LspExpandAbbreviation, _, _>(
            |params, _| async move {
                assert_eq!(
                    params.abbreviation, "ul>li*",
                    "the `|t` filter should be stripped before querying the server"
                );
                assert_eq!(
                    params.options.text,
                    Some(vec!["one".to_string(), "two".to_string()]),
                    "list markers should be removed from the wrapped lines"
                );
                Ok(Some(format_expansion(
                    "<ul>\n\t<li>one</li>\n\t<li>two</li>\n</ul>",
                    &params.options.options,
                )))
            },
        );

        confirm_abbreviation(&editor, "ul>li*|t", cx);
        requests.next().await.unwrap();
        cx.executor().run_until_parked();

        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "  <ul>\n      <li>one</li>\n      <li>two</li>\n  </ul>\n"
            );
        });
    }

    #[gpui::test]
    async fn test_partial_tag_selection_expands_to_element(cx: &mut TestAppContext) {
        let (editor, mut fake_servers, cx) = setup("<p>hello</p>", cx).await;
        let fake_server = fake_servers.next().await.unwrap();
        cx.executor().run_until_parked();

        select_and_wrap(&editor, [Point::new(0, 1)..Point::new(0, 6)], cx);

        let mut requests = fake_server.set_request_handler::<LspExpandAbbreviation, _, _>(
            |params, _| async move {
                assert_eq!(params.abbreviation, "div");
                assert_eq!(
                    params.options.text,
                    Some(vec!["<p>hello</p>".to_string()]),
                    "a selection endpoint inside a tag should expand to the whole element"
                );
                Ok(Some("<div><p>hello</p></div>".to_string()))
            },
        );

        confirm_abbreviation(&editor, "div", cx);
        requests.next().await.unwrap();
        cx.executor().run_until_parked();

        editor.update(cx, |editor, cx| {
            assert_eq!(editor.text(cx), "<div><p>hello</p></div>");
        });
    }

    #[gpui::test]
    async fn test_dollar_signs_in_wrapped_text_are_escaped(cx: &mut TestAppContext) {
        let (editor, mut fake_servers, cx) = setup("<p>costs $100</p>", cx).await;
        let fake_server = fake_servers.next().await.unwrap();
        cx.executor().run_until_parked();

        select_and_wrap(&editor, [Point::new(0, 3)..Point::new(0, 13)], cx);

        let mut requests = fake_server.set_request_handler::<LspExpandAbbreviation, _, _>(
            |params, _| async move {
                assert_eq!(
                    params.options.text,
                    Some(vec!["costs \\$100".to_string()]),
                    "dollar signs should be escaped so snippet insertion cannot eat them"
                );
                Ok(Some("<div>costs \\$100</div>".to_string()))
            },
        );

        confirm_abbreviation(&editor, "div", cx);
        requests.next().await.unwrap();
        cx.executor().run_until_parked();

        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "<p><div>costs $100</div></p>",
                "the escaped dollar sign should round-trip back to a literal"
            );
        });
    }

    #[gpui::test]
    async fn test_comment_and_bem_filters_are_forwarded(cx: &mut TestAppContext) {
        let (editor, mut fake_servers, cx) = setup("<p>hello</p>", cx).await;
        let fake_server = fake_servers.next().await.unwrap();
        cx.executor().run_until_parked();

        select_and_wrap(&editor, [Point::new(0, 3)..Point::new(0, 8)], cx);

        let mut requests = fake_server.set_request_handler::<LspExpandAbbreviation, _, _>(
            |params, _| async move {
                assert_eq!(
                    params.abbreviation, "div#page",
                    "filters should be stripped from the abbreviation"
                );
                assert_eq!(
                    params.options.options,
                    EmmetOutputOptions {
                        indent: "    ".to_string(),
                        base_indent: String::new(),
                        comment_enabled: Some(true),
                        bem_enabled: Some(true),
                        inline_break: None,
                    },
                    "the |c and |bem filters should map to emmet output options"
                );
                Ok(Some(
                    "<div id=\"page\">hello</div>\n<!-- /#page -->".to_string(),
                ))
            },
        );

        confirm_abbreviation(&editor, "div#page|c|bem", cx);
        requests.next().await.unwrap();
        cx.executor().run_until_parked();

        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "<p><div id=\"page\">hello</div>\n<!-- /#page --></p>"
            );
        });
    }

    #[gpui::test]
    async fn test_wrap_is_unavailable_in_stylesheets(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_file(path!("/file.css"), ".a { color: red; }".into())
            .await;

        let project = Project::test(fs, [path!("/file.css").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(Language::new(
            LanguageConfig {
                name: "CSS".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["css".into()],
                    ..LanguageMatcher::default()
                }
                .into(),
                ..LanguageConfig::default()
            },
            None,
        )));
        let mut fake_servers = language_registry.register_fake_lsp(
            "CSS",
            FakeLspAdapter {
                name: "emmet-language-server",
                ..FakeLspAdapter::default()
            },
        );

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/file.css"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let (editor, cx) = cx.add_window_view(|window, cx| {
            build_editor_with_project(project, multi_buffer, window, cx)
        });
        fake_servers.next().await.unwrap();
        cx.executor().run_until_parked();

        select_and_wrap(&editor, [Point::new(0, 0)..Point::new(0, 18)], cx);

        editor.update(cx, |editor, cx| {
            assert!(
                editor.pending_inline_input.is_none(),
                "wrapping is a markup action and should refuse stylesheets"
            );
            assert_eq!(editor.text(cx), ".a { color: red; }");
        });
    }

    #[gpui::test]
    async fn test_interleaved_overlapping_selections_are_merged(cx: &mut TestAppContext) {
        let (editor, mut fake_servers, cx) = setup("<div><p>a</p><p>b</p>xy</div>", cx).await;
        let fake_server = fake_servers.next().await.unwrap();
        cx.executor().run_until_parked();

        select_and_wrap(
            &editor,
            [
                Point::new(0, 8)..Point::new(0, 9),
                Point::new(0, 16)..Point::new(0, 17),
                Point::new(0, 22)..Point::new(0, 22),
            ],
            cx,
        );

        let mut requests = fake_server.set_request_handler::<LspExpandAbbreviation, _, _>(
            |params, _| async move {
                assert_eq!(params.abbreviation, "section");
                assert_eq!(
                    params.options.text,
                    Some(vec!["<div><p>a</p><p>b</p>xy</div>".to_string()]),
                    "the element expanded from the trailing caret should be merged \
                     with the earlier selections into a single wrap target"
                );
                Ok(Some(
                    "<section><div><p>a</p><p>b</p>xy</div></section>".to_string(),
                ))
            },
        );

        confirm_abbreviation(&editor, "section", cx);
        requests.next().await.unwrap();
        cx.executor().run_until_parked();

        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "<section><div><p>a</p><p>b</p>xy</div></section>"
            );
            assert!(editor.pending_inline_input.is_none());
        });
    }

    #[gpui::test]
    async fn test_enter_confirms_when_parent_editor_is_focused(cx: &mut TestAppContext) {
        let (editor, mut fake_servers, cx) = setup("<p>hello</p>", cx).await;
        let fake_server = fake_servers.next().await.unwrap();
        cx.executor().run_until_parked();

        select_and_wrap(&editor, [Point::new(0, 3)..Point::new(0, 8)], cx);

        let mut requests = fake_server.set_request_handler::<LspExpandAbbreviation, _, _>(
            |params, _| async move {
                assert_eq!(params.abbreviation, "div.wrap");
                Ok(Some("<div class=\"wrap\">hello</div>".to_string()))
            },
        );

        editor.update_in(cx, |editor, window, cx| {
            let input = pending_input(editor);
            input.update(cx, |input, cx| input.set_text("div.wrap", window, cx));
        });
        cx.update(|_, cx| {
            cx.bind_keys([
                gpui::KeyBinding::new(
                    "enter",
                    crate::actions::Newline,
                    Some("Editor && mode == full"),
                ),
                gpui::KeyBinding::new("enter", menu::Confirm, Some("Editor && inline_input")),
            ]);
        });
        editor.update_in(cx, |editor, window, cx| {
            let focus_handle = editor.focus_handle(cx);
            window.focus(&focus_handle, cx);
        });
        cx.simulate_keystrokes("enter");
        requests.next().await.unwrap();
        cx.executor().run_until_parked();

        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "<p><div class=\"wrap\">hello</div></p>",
                "enter should confirm the wrap instead of inserting a newline"
            );
            assert!(editor.pending_inline_input.is_none());
        });
    }

    #[gpui::test]
    async fn test_wrap_in_multibuffer_excerpts(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            serde_json::json!({
                "a.html": "<p>one</p>",
                "b.html": "<p>two</p>",
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let mut fake_servers = register_html_with_emmet(&project, cx);

        let buffer_a = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/a.html"), cx)
            })
            .await
            .unwrap();
        let buffer_b = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/b.html"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| {
            let mut multi_buffer = MultiBuffer::new(Capability::ReadWrite);
            multi_buffer.set_excerpts_for_path(
                PathKey::sorted(0),
                buffer_a.clone(),
                [Point::new(0, 0)..Point::new(0, 10)],
                0,
                cx,
            );
            multi_buffer.set_excerpts_for_path(
                PathKey::sorted(1),
                buffer_b.clone(),
                [Point::new(0, 0)..Point::new(0, 10)],
                0,
                cx,
            );
            multi_buffer
        });
        let (editor, cx) = cx.add_window_view(|window, cx| {
            build_editor_with_project(project, multi_buffer, window, cx)
        });
        let fake_server = fake_servers.next().await.unwrap();
        cx.executor().run_until_parked();

        select_and_wrap(
            &editor,
            [
                Point::new(0, 3)..Point::new(0, 6),
                Point::new(1, 3)..Point::new(1, 6),
            ],
            cx,
        );

        let mut requests = fake_server.set_request_handler::<LspExpandAbbreviation, _, _>(
            |params, _| async move {
                assert_eq!(params.abbreviation, "div");
                let wrapped = match params.options.text.as_deref() {
                    Some([line]) if line == "one" => "<div>one</div>",
                    Some([line]) if line == "two" => "<div>two</div>",
                    other => panic!("unexpected wrap text: {other:?}"),
                };
                Ok(Some(wrapped.to_string()))
            },
        );

        confirm_abbreviation(&editor, "div", cx);
        requests.next().await.unwrap();
        requests.next().await.unwrap();
        cx.executor().run_until_parked();

        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "<p><div>one</div></p>\n<p><div>two</div></p>",
                "each excerpt should be wrapped in its own buffer"
            );
            assert!(editor.pending_inline_input.is_none());
        });
        cx.update(|_, cx| {
            assert_eq!(buffer_a.read(cx).text(), "<p><div>one</div></p>");
            assert_eq!(buffer_b.read(cx).text(), "<p><div>two</div></p>");
        });
    }

    fn select_and_wrap(
        editor: &Entity<Editor>,
        ranges: impl IntoIterator<Item = Range<Point>>,
        cx: &mut VisualTestContext,
    ) {
        editor.update_in(cx, |editor, window, cx| {
            editor.change_selections(SelectionEffects::default(), window, cx, |s| {
                s.select_ranges(ranges)
            });
            wrap_with_abbreviation(editor, &WrapWithAbbreviation, window, cx);
        });
    }

    fn confirm_abbreviation(
        editor: &Entity<Editor>,
        abbreviation: &str,
        cx: &mut VisualTestContext,
    ) {
        editor.update_in(cx, |editor, window, cx| {
            let input = pending_input(editor);
            input.update(cx, |input, cx| input.set_text(abbreviation, window, cx));
            editor.confirm_inline_input(window, cx);
        });
    }

    fn format_expansion(tab_indented: &str, options: &EmmetOutputOptions) -> String {
        tab_indented
            .split('\n')
            .enumerate()
            .map(|(ix, line)| {
                let levels = line.chars().take_while(|ch| *ch == '\t').count();
                let base = if ix == 0 { "" } else { &options.base_indent };
                format!("{base}{}{}", options.indent.repeat(levels), &line[levels..])
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn input_text(editor: &Entity<Editor>, cx: &mut VisualTestContext) -> String {
        editor.update(cx, |editor, cx| pending_input(editor).read(cx).text(cx))
    }

    fn pending_input(editor: &Editor) -> Entity<Editor> {
        editor
            .pending_inline_input
            .as_ref()
            .expect("emmet wrap input should be pending")
            .editor
            .clone()
    }

    async fn setup<'a>(
        text: &str,
        cx: &'a mut TestAppContext,
    ) -> (
        Entity<Editor>,
        futures::channel::mpsc::UnboundedReceiver<lsp::FakeLanguageServer>,
        &'a mut VisualTestContext,
    ) {
        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_file(path!("/file.html"), text.into()).await;

        let project = Project::test(fs, [path!("/file.html").as_ref()], cx).await;
        let fake_servers = register_html_with_emmet(&project, cx);

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/file.html"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let (editor, cx) = cx.add_window_view(|window, cx| {
            build_editor_with_project(project, multi_buffer, window, cx)
        });
        (editor, fake_servers, cx)
    }

    fn register_html_with_emmet(
        project: &Entity<Project>,
        cx: &mut TestAppContext,
    ) -> futures::channel::mpsc::UnboundedReceiver<lsp::FakeLanguageServer> {
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(Language::new(
            LanguageConfig {
                name: "HTML".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["html".into()],
                    ..LanguageMatcher::default()
                }
                .into(),
                ..LanguageConfig::default()
            },
            Some(tree_sitter_html::LANGUAGE.into()),
        )));
        language_registry.register_fake_lsp(
            "HTML",
            FakeLspAdapter {
                name: "emmet-language-server",
                ..FakeLspAdapter::default()
            },
        )
    }
}
