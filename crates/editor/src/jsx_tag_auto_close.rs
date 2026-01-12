use anyhow::{Context as _, Result, anyhow};
use collections::HashMap;
use gpui::{Context, Entity, Window};
use multi_buffer::{BufferOffset, MultiBuffer, ToOffset};
use std::ops::Range;
use util::ResultExt as _;

use language::{BufferSnapshot, JsxTagAutoCloseConfig, Node};
use text::{Anchor, OffsetRangeExt as _};

use crate::{Editor, SelectionEffects};

pub struct JsxTagCompletionState {
    edit_index: usize,
    open_tag_range: Range<usize>,
}

/// Index of the named child within an open or close tag
/// that corresponds to the tag name
/// Note that this is not configurable, i.e. we assume the first
/// named child of a tag node is the tag name
const TS_NODE_TAG_NAME_CHILD_INDEX: u32 = 0;

/// Maximum number of parent elements to walk back when checking if an open tag
/// is already closed.
///
/// See the comment in `generate_auto_close_edits` for more details
const ALREADY_CLOSED_PARENT_ELEMENT_WALK_BACK_LIMIT: usize = 2;

pub(crate) fn should_auto_close(
    buffer: &BufferSnapshot,
    edited_ranges: &[Range<Anchor>],
    config: &JsxTagAutoCloseConfig,
) -> Option<Vec<JsxTagCompletionState>> {
    let mut to_auto_edit = vec![];
    for (index, edited_range) in edited_ranges.iter().enumerate() {
        let text = buffer
            .text_for_range(edited_range.clone())
            .collect::<String>();
        let edited_range = edited_range.to_offset(buffer);
        if !text.ends_with(">") {
            continue;
        }
        let Some(layer) = buffer.smallest_syntax_layer_containing(edited_range.clone()) else {
            continue;
        };
        let Some(node) = layer
            .node()
            .named_descendant_for_byte_range(edited_range.start, edited_range.end)
        else {
            continue;
        };
        let mut jsx_open_tag_node = node;
        if node.grammar_name() != config.open_tag_node_name
            && let Some(parent) = node.parent()
            && parent.grammar_name() == config.open_tag_node_name
        {
            jsx_open_tag_node = parent;
        }
        if jsx_open_tag_node.grammar_name() != config.open_tag_node_name {
            continue;
        }

        let first_two_chars: Option<[char; 2]> = {
            let mut chars = buffer
                .text_for_range(jsx_open_tag_node.byte_range())
                .flat_map(|chunk| chunk.chars());
            if let (Some(c1), Some(c2)) = (chars.next(), chars.next()) {
                Some([c1, c2])
            } else {
                None
            }
        };
        if let Some(chars) = first_two_chars {
            if chars[0] != '<' {
                continue;
            }
            if chars[1] == '!' || chars[1] == '/' {
                continue;
            }
        }

        to_auto_edit.push(JsxTagCompletionState {
            edit_index: index,
            open_tag_range: jsx_open_tag_node.byte_range(),
        });
    }
    if to_auto_edit.is_empty() {
        None
    } else {
        Some(to_auto_edit)
    }
}

pub(crate) fn generate_auto_close_edits(
    buffer: &BufferSnapshot,
    ranges: &[Range<Anchor>],
    config: &JsxTagAutoCloseConfig,
    state: Vec<JsxTagCompletionState>,
) -> Result<Vec<(Range<Anchor>, String)>> {
    let mut edits = Vec::with_capacity(state.len());
    for auto_edit in state {
        let edited_range = ranges[auto_edit.edit_index].clone();
        let Some(layer) = buffer.smallest_syntax_layer_containing(edited_range.clone()) else {
            continue;
        };
        let layer_root_node = layer.node();
        let Some(open_tag) = layer_root_node.descendant_for_byte_range(
            auto_edit.open_tag_range.start,
            auto_edit.open_tag_range.end,
        ) else {
            continue;
        };
        assert!(open_tag.kind() == config.open_tag_node_name);
        let tag_name = open_tag
            .named_child(TS_NODE_TAG_NAME_CHILD_INDEX)
            .filter(|node| {
                node.kind() == config.tag_name_node_name
                    || config
                        .tag_name_node_name_alternates
                        .iter()
                        .any(|alternate| alternate == node.kind())
            })
            .map_or("".to_string(), |node| {
                buffer.text_for_range(node.byte_range()).collect::<String>()
            });

        /*
         * Naive check to see if the tag is already closed
         * Essentially all we do is count the number of open and close tags
         * with the same tag name as the open tag just entered by the user
         * The search is limited to some scope determined by
         * `ALREADY_CLOSED_PARENT_ELEMENT_WALK_BACK_LIMIT`
         *
         * The limit is preferable to walking up the tree until we find a non-tag node,
         * and then checking the entire tree, as this is unnecessarily expensive, and
         * risks false positives
         * eg. a `</div>` tag without a corresponding opening tag exists 25 lines away
         *     and the user typed in `<div>`, intuitively we still want to auto-close it because
         *     the other `</div>` tag is almost certainly not supposed to be the closing tag for the
         *     current element
         *
         * We have to walk up the tree some amount because tree-sitters error correction is not
         * designed to handle this case, and usually does not represent the tree structure
         * in the way we might expect,
         *
         * We half to walk up the tree until we hit an element with a different open tag name (`doing_deep_search == true`)
         * because tree-sitter may pair the new open tag with the root of the tree's closing tag leaving the
         * root's opening tag unclosed.
         * e.g
         *      ```
         *      <div>
         *          <div>|cursor here|
         *      </div>
         *      ```
         *     in Astro/vue/svelte tree-sitter represented the tree as
         *      (
         *          (jsx_element
         *              (jsx_opening_element
         *                  "<div>")
         *          )
         *          (jsx_element
         *              (jsx_opening_element
         *                  "<div>") // <- cursor is here
         *              (jsx_closing_element
         *                  "</div>")
         *          )
         *      )
         *     so if we only walked to the first `jsx_element` node,
         *     we would mistakenly identify the div entered by the
         *     user as already being closed, despite this clearly
         *     being false
         *
         * The errors with the tree-sitter tree caused by error correction,
         * are also why the naive algorithm was chosen, as the alternative
         * approach would be to maintain or construct a full parse tree (like tree-sitter)
         * that better represents errors in a way that we can simply check
         * the enclosing scope of the entered tag for a closing tag
         * This is far more complex and expensive, and was deemed impractical
         * given that the naive algorithm is sufficient in the majority of cases.
         */
        {
            let tag_node_name_equals = |node: &Node, name: &str| {
                let is_empty = name.is_empty();
                if let Some(node_name) = node.named_child(TS_NODE_TAG_NAME_CHILD_INDEX) {
                    let range = node_name.byte_range();
                    return buffer.text_for_range(range).equals_str(name);
                }
                is_empty
            };

            let tree_root_node = {
                let mut ancestors = Vec::with_capacity(
                    // estimate of max, not based on any data,
                    // but trying to avoid excessive reallocation
                    16,
                );
                ancestors.push(layer_root_node);
                let mut cur = layer_root_node;
                // walk down the tree until we hit the open tag
                // note: this is what node.parent() does internally
                while let Some(descendant) = cur.child_with_descendant(open_tag) {
                    if descendant == open_tag {
                        break;
                    }
                    ancestors.push(descendant);
                    cur = descendant;
                }

                assert!(!ancestors.is_empty());

                let mut tree_root_node = open_tag;

                let mut parent_element_node_count = 0;
                let mut doing_deep_search = false;

                for &ancestor in ancestors.iter().rev() {
                    tree_root_node = ancestor;
                    let is_element = ancestor.kind() == config.jsx_element_node_name;
                    let is_error = ancestor.is_error();
                    if is_error || !is_element {
                        break;
                    }
                    if is_element {
                        let is_first = parent_element_node_count == 0;
                        if !is_first {
                            let has_open_tag_with_same_tag_name = ancestor
                                .named_child(0)
                                .filter(|n| n.kind() == config.open_tag_node_name)
                                .is_some_and(|element_open_tag_node| {
                                    tag_node_name_equals(&element_open_tag_node, &tag_name)
                                });
                            if has_open_tag_with_same_tag_name {
                                doing_deep_search = true;
                            } else if doing_deep_search {
                                break;
                            }
                        }
                        parent_element_node_count += 1;
                        if !doing_deep_search
                            && parent_element_node_count
                                >= ALREADY_CLOSED_PARENT_ELEMENT_WALK_BACK_LIMIT
                        {
                            break;
                        }
                    }
                }
                tree_root_node
            };

            let mut unclosed_open_tag_count: i32 = 0;

            let mut cursor = layer_root_node.walk();

            let mut stack = Vec::with_capacity(tree_root_node.descendant_count());
            stack.extend(tree_root_node.children(&mut cursor));

            let mut has_erroneous_close_tag = false;
            let mut erroneous_close_tag_node_name = "";
            if let Some(name) = config.erroneous_close_tag_node_name.as_deref() {
                has_erroneous_close_tag = true;
                erroneous_close_tag_node_name = name;
            }

            let is_after_open_tag = |node: &Node| {
                node.start_byte() < open_tag.start_byte() && node.end_byte() < open_tag.start_byte()
            };

            // perf: use cursor for more efficient traversal
            // if child -> go to child
            // else if next sibling -> go to next sibling
            // else -> go to parent
            // if parent == tree_root_node -> break
            while let Some(node) = stack.pop() {
                let kind = node.kind();
                if kind == config.open_tag_node_name {
                    if tag_node_name_equals(&node, &tag_name) {
                        unclosed_open_tag_count += 1;
                    }
                } else if kind == config.close_tag_node_name {
                    if tag_node_name_equals(&node, &tag_name) {
                        unclosed_open_tag_count -= 1;
                    }
                } else if has_erroneous_close_tag && kind == erroneous_close_tag_node_name {
                    if tag_node_name_equals(&node, &tag_name) && !is_after_open_tag(&node) {
                        unclosed_open_tag_count -= 1;
                    }
                } else if kind == config.jsx_element_node_name {
                    // perf: filter only open,close,element,erroneous nodes
                    stack.extend(node.children(&mut cursor));
                }
            }

            if unclosed_open_tag_count <= 0 {
                // skip if already closed
                continue;
            }
        }
        let edit_anchor = buffer.anchor_after(edited_range.end);
        let edit_range = edit_anchor..edit_anchor;
        edits.push((edit_range, format!("</{}>", tag_name)));
    }
    Ok(edits)
}

pub(crate) fn refresh_enabled_in_any_buffer(
    editor: &mut Editor,
    multi_buffer: &Entity<MultiBuffer>,
    cx: &Context<Editor>,
) {
    editor.jsx_tag_auto_close_enabled_in_any_buffer = {
        let multi_buffer = multi_buffer.read(cx);
        let mut found_enabled = false;
        multi_buffer.for_each_buffer(|buffer| {
            if found_enabled {
                return;
            }

            let buffer = buffer.read(cx);
            let snapshot = buffer.snapshot();
            for syntax_layer in snapshot.syntax_layers() {
                let language = syntax_layer.language;
                if language.config().jsx_tag_auto_close.is_none() {
                    continue;
                }
                let language_settings = language::language_settings::language_settings(
                    Some(language.name()),
                    snapshot.file(),
                    cx,
                );
                if language_settings.jsx_tag_auto_close {
                    found_enabled = true;
                }
            }
        });

        found_enabled
    };
}

pub(crate) type InitialBufferVersionsMap = HashMap<language::BufferId, clock::Global>;

pub(crate) fn construct_initial_buffer_versions_map<
    D: ToOffset + Copy,
    _S: Into<std::sync::Arc<str>>,
>(
    editor: &Editor,
    edits: &[(Range<D>, _S)],
    cx: &Context<Editor>,
) -> InitialBufferVersionsMap {
    let mut initial_buffer_versions = InitialBufferVersionsMap::default();

    if !editor.jsx_tag_auto_close_enabled_in_any_buffer {
        return initial_buffer_versions;
    }

    for (edit_range, _) in edits {
        let edit_range_buffer = editor
            .buffer()
            .read(cx)
            .excerpt_containing(edit_range.end, cx)
            .map(|e| e.1);
        if let Some(buffer) = edit_range_buffer {
            let (buffer_id, buffer_version) =
                buffer.read_with(cx, |buffer, _| (buffer.remote_id(), buffer.version.clone()));
            initial_buffer_versions.insert(buffer_id, buffer_version);
        }
    }
    initial_buffer_versions
}

pub(crate) fn handle_from(
    editor: &Editor,
    initial_buffer_versions: InitialBufferVersionsMap,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    if !editor.jsx_tag_auto_close_enabled_in_any_buffer {
        return;
    }

    struct JsxAutoCloseEditContext {
        buffer: Entity<language::Buffer>,
        config: language::JsxTagAutoCloseConfig,
        edits: Vec<Range<Anchor>>,
    }

    let mut edit_contexts =
        HashMap::<(language::BufferId, language::LanguageId), JsxAutoCloseEditContext>::default();

    for (buffer_id, buffer_version_initial) in initial_buffer_versions {
        let Some(buffer) = editor.buffer.read(cx).buffer(buffer_id) else {
            continue;
        };
        let snapshot = buffer.read(cx).snapshot();
        for (edit, range) in buffer
            .read(cx)
            .anchored_edits_since::<usize>(&buffer_version_initial)
        {
            let Some(language) = snapshot.language_at(edit.new.end) else {
                continue;
            };

            let Some(config) = language.config().jsx_tag_auto_close.as_ref() else {
                continue;
            };

            let language_settings = snapshot.settings_at(edit.new.end, cx);
            if !language_settings.jsx_tag_auto_close {
                continue;
            }

            edit_contexts
                .entry((snapshot.remote_id(), language.id()))
                .or_insert_with(|| JsxAutoCloseEditContext {
                    buffer: buffer.clone(),
                    config: config.clone(),
                    edits: vec![],
                })
                .edits
                .push(range);
        }
    }

    for ((buffer_id, _), auto_close_context) in edit_contexts {
        let JsxAutoCloseEditContext {
            buffer,
            config: jsx_tag_auto_close_config,
            edits: edited_ranges,
        } = auto_close_context;

        let (buffer_version_initial, mut buffer_parse_status_rx) =
            buffer.read_with(cx, |buffer, _| (buffer.version(), buffer.parse_status()));

        cx.spawn_in(window, async move |this, cx| {
            let Some(buffer_parse_status) = buffer_parse_status_rx.recv().await.ok() else {
                return Some(());
            };
            if buffer_parse_status == language::ParseStatus::Parsing {
                let Some(language::ParseStatus::Idle) = buffer_parse_status_rx.recv().await.ok()
                else {
                    return Some(());
                };
            }

            let buffer_snapshot = buffer.read_with(cx, |buf, _| buf.snapshot());

            let Some(edit_behavior_state) =
                should_auto_close(&buffer_snapshot, &edited_ranges, &jsx_tag_auto_close_config)
            else {
                return Some(());
            };

            let ensure_no_edits_since_start = || -> Option<()> {
                let has_edits_since_start = this
                    .read_with(cx, |this, cx| {
                        this.buffer.read(cx).buffer(buffer_id).is_none_or(|buffer| {
                            buffer.read(cx).has_edits_since(&buffer_version_initial)
                        })
                    })
                    .ok()?;

                if has_edits_since_start {
                    Err(anyhow!(
                        "Auto-close Operation Failed - Buffer has edits since start"
                    ))
                    .log_err()?;
                }

                Some(())
            };

            ensure_no_edits_since_start()?;

            let edits = cx
                .background_executor()
                .spawn({
                    let buffer_snapshot = buffer_snapshot.clone();
                    async move {
                        generate_auto_close_edits(
                            &buffer_snapshot,
                            &edited_ranges,
                            &jsx_tag_auto_close_config,
                            edit_behavior_state,
                        )
                    }
                })
                .await;

            let edits = edits
                .context("Auto-close Operation Failed - Failed to compute edits")
                .log_err()?;

            if edits.is_empty() {
                return Some(());
            }

            // check again after awaiting background task before applying edits
            ensure_no_edits_since_start()?;

            let multi_buffer_snapshot = this
                .read_with(cx, |this, cx| this.buffer.read(cx).snapshot(cx))
                .ok()?;

            let mut base_selections = Vec::new();
            let mut buffer_selection_map = HashMap::default();

            {
                let selections = this
                    .read_with(cx, |this, _| this.selections.disjoint_anchors_arc())
                    .ok()?;
                for selection in selections.iter() {
                    let Some(selection_buffer_offset_head) =
                        multi_buffer_snapshot.point_to_buffer_offset(selection.head())
                    else {
                        base_selections.push(selection.clone());
                        continue;
                    };
                    let Some(selection_buffer_offset_tail) =
                        multi_buffer_snapshot.point_to_buffer_offset(selection.tail())
                    else {
                        base_selections.push(selection.clone());
                        continue;
                    };

                    let is_entirely_in_buffer = selection_buffer_offset_head.0.remote_id()
                        == buffer_id
                        && selection_buffer_offset_tail.0.remote_id() == buffer_id;
                    if !is_entirely_in_buffer {
                        base_selections.push(selection.clone());
                        continue;
                    }

                    let selection_buffer_offset_head = selection_buffer_offset_head.1;
                    let selection_buffer_offset_tail = selection_buffer_offset_tail.1;
                    buffer_selection_map.insert(
                        (selection_buffer_offset_head, selection_buffer_offset_tail),
                        (selection.clone(), None),
                    );
                }
            }

            let mut any_selections_need_update = false;
            for edit in &edits {
                let edit_range_offset = edit.0.to_offset(&buffer_snapshot);
                if edit_range_offset.start != edit_range_offset.end {
                    continue;
                }
                if let Some(selection) = buffer_selection_map.get_mut(&(
                    BufferOffset(edit_range_offset.start),
                    BufferOffset(edit_range_offset.end),
                )) {
                    if selection.0.head().bias() != text::Bias::Right
                        || selection.0.tail().bias() != text::Bias::Right
                    {
                        continue;
                    }
                    if selection.1.is_none() {
                        any_selections_need_update = true;
                        selection.1 = Some(
                            selection
                                .0
                                .clone()
                                .map(|anchor| multi_buffer_snapshot.anchor_before(anchor)),
                        );
                    }
                }
            }

            buffer.update(cx, |buffer, cx| {
                buffer.edit(edits, None, cx);
            });

            if any_selections_need_update {
                let multi_buffer_snapshot = this
                    .read_with(cx, |this, cx| {
                        this.buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx))
                    })
                    .ok()?;

                base_selections.extend(buffer_selection_map.values().map(|selection| {
                    match &selection.1 {
                        Some(left_biased_selection) => left_biased_selection.clone(),
                        None => selection.0.clone(),
                    }
                }));

                let base_selections = base_selections
                    .into_iter()
                    .map(|selection| {
                        selection.map(|anchor| anchor.to_offset(&multi_buffer_snapshot))
                    })
                    .collect::<Vec<_>>();
                this.update_in(cx, |this, window, cx| {
                    this.change_selections(
                        SelectionEffects::no_scroll().completions(false),
                        window,
                        cx,
                        |s| {
                            s.select(base_selections);
                        },
                    );
                })
                .ok()?;
            }

            Some(())
        })
        .detach();
    }
}

#[cfg(test)]
mod jsx_tag_autoclose_tests {
    use crate::{
        editor_tests::init_test,
        test::{build_editor, editor_test_context::EditorTestContext},
    };

    use super::*;
    use gpui::{AppContext as _, TestAppContext};
    use languages::language;
    use multi_buffer::{ExcerptRange, MultiBufferOffset};
    use text::Selection;

    async fn test_setup(cx: &mut TestAppContext) -> EditorTestContext {
        init_test(cx, |settings| {
            settings
                .defaults
                .jsx_tag_auto_close
                .get_or_insert_default()
                .enabled = Some(true);
        });

        let mut cx = EditorTestContext::new(cx).await;
        cx.update_buffer(|buffer, cx| {
            let language = language("tsx", tree_sitter_typescript::LANGUAGE_TSX.into());

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
        test_basic,
        "<divˇ" + ">" => "<div>ˇ</div>"
    );

    check!(
        test_basic_nested,
        "<div><divˇ</div>" + ">" => "<div><div>ˇ</div></div>"
    );

    check!(
        test_basic_ignore_already_closed,
        "<div><divˇ</div></div>" + ">" => "<div><div>ˇ</div></div>"
    );

    check!(
        test_doesnt_autoclose_closing_tag,
        "</divˇ" + ">" => "</div>ˇ"
    );

    check!(
        test_jsx_attr,
        "<div attr={</div>}ˇ" + ">" => "<div attr={</div>}>ˇ</div>"
    );

    check!(
        test_ignores_closing_tags_in_expr_block,
        "<div><divˇ{</div>}</div>" + ">" => "<div><div>ˇ</div>{</div>}</div>"
    );

    check!(
        test_doesnt_autoclose_on_gt_in_expr,
        "<div attr={1 ˇ" + ">" => "<div attr={1 >ˇ"
    );

    check!(
        test_ignores_closing_tags_with_different_tag_names,
        "<div><divˇ</div></span>" + ">" => "<div><div>ˇ</div></div></span>"
    );

    check!(
        test_autocloses_in_jsx_expression,
        "<div>{<divˇ}</div>" + ">" => "<div>{<div>ˇ</div>}</div>"
    );

    check!(
        test_doesnt_autoclose_already_closed_in_jsx_expression,
        "<div>{<divˇ</div>}</div>" + ">" => "<div>{<div>ˇ</div>}</div>"
    );

    check!(
        test_autocloses_fragment,
        "<ˇ" + ">" => "<>ˇ</>"
    );

    check!(
        test_does_not_include_type_argument_in_autoclose_tag_name,
        "<Component<T> attr={boolean_value}ˇ" + ">" => "<Component<T> attr={boolean_value}>ˇ</Component>"
    );

    check!(
        test_does_not_autoclose_doctype,
        "<!DOCTYPE htmlˇ" + ">" => "<!DOCTYPE html>ˇ"
    );

    check!(
        test_does_not_autoclose_comment,
        "<!-- comment --ˇ" + ">" => "<!-- comment -->ˇ"
    );

    check!(
        test_autocloses_dot_separated_component,
        "<Component.Fooˇ" + ">" => "<Component.Foo>ˇ</Component.Foo>"
    );

    check!(
        test_multi_cursor_autoclose_same_tag,
        r#"
        <divˇ
        <divˇ
        "#
        + ">" =>
        r#"
        <div>ˇ</div>
        <div>ˇ</div>
        "#
    );

    check!(
        test_multi_cursor_autoclose_different_tags,
        r#"
        <divˇ
        <spanˇ
        "#
        + ">" =>
        r#"
        <div>ˇ</div>
        <span>ˇ</span>
        "#
    );

    check!(
        test_multi_cursor_autoclose_some_dont_autoclose_others,
        r#"
        <divˇ
        <div /ˇ
        <spanˇ</span>
        <!DOCTYPE htmlˇ
        </headˇ
        <Component<T>ˇ
        ˇ
        "#
        + ">" =>
        r#"
        <div>ˇ</div>
        <div />ˇ
        <span>ˇ</span>
        <!DOCTYPE html>ˇ
        </head>ˇ
        <Component<T>>ˇ</Component>
        >ˇ
        "#
    );

    check!(
        test_doesnt_mess_up_trailing_text,
        "<divˇfoobar" + ">" => "<div>ˇ</div>foobar"
    );

    #[gpui::test]
    async fn test_multibuffer(cx: &mut TestAppContext) {
        init_test(cx, |settings| {
            settings
                .defaults
                .jsx_tag_auto_close
                .get_or_insert_default()
                .enabled = Some(true);
        });

        let buffer_a = cx.new(|cx| {
            let mut buf = language::Buffer::local("<div", cx);
            buf.set_language(
                Some(language("tsx", tree_sitter_typescript::LANGUAGE_TSX.into())),
                cx,
            );
            buf
        });
        let buffer_b = cx.new(|cx| {
            let mut buf = language::Buffer::local("<pre", cx);
            buf.set_language(
                Some(language("tsx", tree_sitter_typescript::LANGUAGE_TSX.into())),
                cx,
            );
            buf
        });
        let buffer_c = cx.new(|cx| language::Buffer::local("<span", cx));
        let buffer = cx.new(|cx| {
            let mut buf = MultiBuffer::new(language::Capability::ReadWrite);
            buf.push_excerpts(
                buffer_a,
                [ExcerptRange::new(text::Anchor::MIN..text::Anchor::MAX)],
                cx,
            );
            buf.push_excerpts(
                buffer_b,
                [ExcerptRange::new(text::Anchor::MIN..text::Anchor::MAX)],
                cx,
            );
            buf.push_excerpts(
                buffer_c,
                [ExcerptRange::new(text::Anchor::MIN..text::Anchor::MAX)],
                cx,
            );
            buf
        });
        let editor = cx.add_window(|window, cx| build_editor(buffer.clone(), window, cx));

        let mut cx = EditorTestContext::for_editor(editor, cx).await;

        cx.update_editor(|editor, window, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select(vec![
                    Selection::from_offset(MultiBufferOffset(4)),
                    Selection::from_offset(MultiBufferOffset(9)),
                    Selection::from_offset(MultiBufferOffset(15)),
                ])
            })
        });
        cx.run_until_parked();

        cx.update_editor(|editor, window, cx| {
            editor.handle_input(">", window, cx);
        });
        cx.run_until_parked();

        cx.assert_editor_state("<div>ˇ</div>\n<pre>ˇ</pre>\n<span>ˇ");
    }
}
