use collections::HashMap;
use gpui::{Context, Window};
use language::{BufferSnapshot, ParseStatus};
use multi_buffer::MultiBufferOffset;
use std::{ops::Range, sync::Arc};

use crate::{Editor, SelectionEffects};

/// Languages in which typing `${` inside a plain quoted string should turn
/// that string into a template literal.
const CONVERTIBLE_LANGUAGES: &[&str] = &["JavaScript", "TypeScript", "TSX"];

pub(crate) type InitialBufferVersionsMap = HashMap<language::BufferId, clock::Global>;

/// Collects, for each buffer that just had a `{` inserted, the version it
/// was at right before the edit. `handle_from` uses this once the buffer has
/// finished re-parsing to find the specific edit made by this input event.
pub(crate) fn construct_initial_buffer_versions_map<D: multi_buffer::ToOffset + Copy>(
    editor: &Editor,
    edits: &[(Range<D>, Arc<str>)],
    cx: &Context<Editor>,
) -> InitialBufferVersionsMap {
    let mut initial_buffer_versions = InitialBufferVersionsMap::default();
    if !edits.iter().any(|(_, text)| text.starts_with('{')) {
        return initial_buffer_versions;
    }

    let multibuffer = editor.buffer.read(cx);
    let snapshot = multibuffer.snapshot(cx);
    for (edit_range, text) in edits {
        // `{` may be inserted alone, or together with an auto-closed `}` as
        // a single "{}" edit; either way the `{` is the first character.
        if !text.starts_with('{') {
            continue;
        }
        let anchor = snapshot.anchor_before(edit_range.end);
        let Some((text_anchor, _)) = snapshot.anchor_to_buffer_anchor(anchor) else {
            continue;
        };
        let Some(buffer) = multibuffer.buffer(text_anchor.buffer_id) else {
            continue;
        };
        let (buffer_id, buffer_version) =
            buffer.read_with(cx, |buffer, _| (buffer.remote_id(), buffer.version.clone()));
        initial_buffer_versions.insert(buffer_id, buffer_version);
    }
    initial_buffer_versions
}

pub(crate) fn handle_from(
    editor: &Editor,
    initial_buffer_versions: InitialBufferVersionsMap,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    for (buffer_id, buffer_version_initial) in initial_buffer_versions {
        let Some(buffer) = editor.buffer.read(cx).buffer(buffer_id) else {
            continue;
        };

        // `buffer_version_initial` (from the caller's map) is the version
        // from right *before* the `{` was inserted; it's what we diff
        // against below to find that edit. Separately, `buffer_version_now`
        // is the version *after* that edit, captured here so we can detect
        // whether the user typed anything else while we were waiting for
        // the buffer to finish parsing.
        let (buffer_version_now, mut parse_status_rx) =
            buffer.read_with(cx, |buffer, _| (buffer.version(), buffer.parse_status()));

        cx.spawn_in(window, async move |this, cx| {
            let Some(status) = parse_status_rx.recv().await.ok() else {
                return Some(());
            };
            if status == ParseStatus::Parsing {
                let Some(ParseStatus::Idle) = parse_status_rx.recv().await.ok() else {
                    return Some(());
                };
            }

            let has_new_edits = this
                .read_with(cx, |this, cx| {
                    this.buffer.read(cx).buffer(buffer_id).is_none_or(|buffer| {
                        buffer.read(cx).has_edits_since(&buffer_version_now)
                    })
                })
                .ok()?;
            if has_new_edits {
                // Something else changed the buffer while we were waiting;
                // the position we'd edit at is no longer trustworthy, so
                // bail out rather than risk editing the wrong text.
                return Some(());
            }

            let edited_ranges: Vec<Range<usize>> = buffer.read_with(cx, |buffer, _| {
                buffer
                    .edits_since::<usize>(&buffer_version_initial)
                    .map(|edit| edit.new)
                    .collect::<Vec<_>>()
            });

            let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());
            let edits = compute_template_literal_edits(&snapshot, &edited_ranges);
            if edits.is_empty() {
                return Some(());
            }

            // These are same-length replacements (one quote character for
            // another), so the overall document length is unchanged. But a
            // cursor sitting exactly at a replaced quote's position would
            // otherwise get shifted by the edit's anchor resolution, so
            // capture plain offsets beforehand and restore them verbatim
            // afterwards rather than relying on anchors across this edit.
            let selection_offsets: Vec<Range<MultiBufferOffset>> = this
                .update(cx, |editor, cx| {
                    let display_snapshot = editor.display_snapshot(cx);
                    editor
                        .selections
                        .all::<MultiBufferOffset>(&display_snapshot)
                        .into_iter()
                        .map(|s| s.start..s.end)
                        .collect()
                })
                .ok()?;

            buffer.update(cx, |buffer, cx| {
                buffer.edit(edits, None, cx);
            });

            this.update_in(cx, |editor, window, cx| {
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_ranges(selection_offsets);
                });
            })
            .ok();

            Some(())
        })
        .detach();
    }
}

fn compute_template_literal_edits(
    snapshot: &BufferSnapshot,
    edited_ranges: &[Range<usize>],
) -> Vec<(Range<usize>, &'static str)> {
    let Some(language) = snapshot.language() else {
        return Vec::new();
    };
    if !CONVERTIBLE_LANGUAGES.contains(&language.name().as_ref()) {
        return Vec::new();
    }

    let mut edits = Vec::new();
    let mut converted_string_starts = Vec::new();
    for edited_range in edited_ranges {
        // The edit inserts `{` at its start, possibly followed by an
        // auto-closed `}` (as a single "{}" edit), so the position right
        // after the `{` itself is always `start + 1`, regardless of whether
        // a closing bracket was appended.
        let brace_offset = edited_range.start + 1;
        if brace_offset < 2 {
            continue;
        }
        let dollar_offset = brace_offset - 2;
        let mut preceding_chars = snapshot
            .text_for_range(dollar_offset..brace_offset)
            .flat_map(str::chars);
        if !matches!(
            (preceding_chars.next(), preceding_chars.next()),
            (Some('$'), Some('{'))
        ) {
            continue;
        }

        let Some(node_range) = enclosing_string_range(snapshot, dollar_offset) else {
            continue;
        };
        if converted_string_starts.contains(&node_range.start) {
            continue;
        }
        if node_range.end - node_range.start < 2 {
            continue;
        }

        let open_quote = snapshot.chars_at(node_range.start).next();
        let close_quote = snapshot.chars_at(node_range.end - 1).next();
        let (Some(open_quote), Some(close_quote)) = (open_quote, close_quote) else {
            continue;
        };
        if open_quote != close_quote || (open_quote != '"' && open_quote != '\'') {
            continue;
        }

        // A well-formed single- or double-quoted JS/TS string can't contain a
        // literal newline; bail out defensively if the syntax tree is stale
        // and this doesn't hold.
        let contains_newline = snapshot
            .text_for_range(node_range.clone())
            .flat_map(str::chars)
            .any(|c| c == '\n');
        if contains_newline {
            continue;
        }

        edits.push((node_range.start..node_range.start + 1, "`"));
        edits.push((node_range.end - 1..node_range.end, "`"));
        converted_string_starts.push(node_range.start);
    }
    edits
}

/// Walks up from the smallest node at `offset` looking for the enclosing
/// plain string literal, stopping early if it finds a template string first.
fn enclosing_string_range(snapshot: &BufferSnapshot, offset: usize) -> Option<Range<usize>> {
    // Query the span of the `$` character itself rather than an empty range
    // at its boundary, so the lookup isn't ambiguous about which side of the
    // boundary it's asking about.
    let query_range = offset..offset + 1;
    let layer = snapshot.smallest_syntax_layer_containing(query_range.clone())?;
    let mut node = layer
        .node()
        .named_descendant_for_byte_range(query_range.start, query_range.end)?;
    for _ in 0..8 {
        match node.grammar_name() {
            "string" => return Some(node.byte_range()),
            "template_string" => return None,
            _ => {}
        }
        node = node.parent()?;
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::{editor_tests::init_test, test::editor_test_context::EditorTestContext};
    use gpui::{AppContext as _, TestAppContext};
    use languages::language;

    #[gpui::test]
    async fn test_compute_edits_directly(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let buffer = cx.new(|cx| {
            let mut buffer = language::Buffer::local(r#"const s = "hello${";"#, cx);
            buffer.set_language(
                Some(language(
                    "TypeScript",
                    tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
                )),
                cx,
            );
            buffer
        });
        cx.run_until_parked();

        let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());
        // `const s = "hello${";` - "$" is at offset 16, "{" at offset 17.
        let edited_ranges = vec![17..18];
        let edits = super::compute_template_literal_edits(&snapshot, &edited_ranges);
        eprintln!("edits computed: {edits:?}");
        assert!(!edits.is_empty(), "expected quote-to-backtick edits");
    }

    async fn test_setup(cx: &mut TestAppContext) -> EditorTestContext {
        init_test(cx, |_| {});

        let mut cx = EditorTestContext::new(cx).await;
        cx.update_buffer(|buffer, cx| {
            let language = language("TypeScript", tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into());
            buffer.set_language(Some(language), cx)
        });

        cx
    }

    macro_rules! check {
        ($name:ident, $initial:literal + $input:literal => $expected:expr) => {
            #[gpui::test]
            async fn $name(cx: &mut TestAppContext) {
                let mut cx = test_setup(cx).await;
                cx.set_state($initial);
                cx.run_until_parked();

                cx.update_editor(|editor, window, cx| {
                    editor.handle_input($input, window, cx);
                });
                cx.run_until_parked();
                cx.assert_editor_state($expected);
            }
        };
    }

    check!(
        test_dollar_brace_converts_double_quotes,
        r#"const s = "hello$ˇ";"#
        + "{" =>
        r#"const s = `hello${ˇ`;"#
    );

    check!(
        test_dollar_brace_converts_single_quotes,
        r#"const s = 'hello$ˇ';"#
        + "{" =>
        r#"const s = `hello${ˇ`;"#
    );

    check!(
        test_dollar_brace_does_not_convert_already_template_literal,
        r#"const s = `hello$ˇ`;"#
        + "{" =>
        r#"const s = `hello${ˇ`;"#
    );

    check!(
        test_brace_without_preceding_dollar_does_not_convert,
        r#"const s = "helloˇ";"#
        + "{" =>
        r#"const s = "hello{ˇ";"#
    );

    #[gpui::test]
    async fn test_dollar_brace_outside_string_does_not_convert(cx: &mut TestAppContext) {
        let mut cx = test_setup(cx).await;
        cx.set_state(r#"const s = 1; $ˇ"#);
        cx.run_until_parked();

        cx.update_editor(|editor, window, cx| {
            editor.handle_input("{", window, cx);
        });
        cx.run_until_parked();

        // `$` isn't inside a string here, so nothing should be converted to
        // a template literal, regardless of whether `{` auto-closed.
        let text = cx.update_editor(|editor, _, cx| editor.text(cx));
        assert!(!text.contains('`'), "unexpected backtick in {text:?}");
    }

    check!(
        test_dollar_brace_in_second_string_only_converts_that_string,
        r#"const a = "one"; const b = "two$ˇ";"#
        + "{" =>
        r#"const a = "one"; const b = `two${ˇ`;"#
    );
}
