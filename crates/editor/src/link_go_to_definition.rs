use std::{
    ops::Range,
    time::{Duration, Instant},
};

use gpui::{
    actions,
    color::Color,
    elements::{Flex, MouseEventHandler, Padding, Text},
    fonts::{HighlightStyle, Underline},
    impl_internal_actions,
    platform::CursorStyle,
    Axis, Element, ElementBox, ModelHandle, MutableAppContext, RenderContext, Task, ViewContext,
};
use language::Bias;
use project::{HoverBlock, Project};
use util::TryFutureExt;

use crate::{
    display_map::ToDisplayPoint, Anchor, AnchorRangeExt, DisplayPoint, Editor, EditorSnapshot,
    EditorStyle,
};

#[derive(Clone, PartialEq)]
pub struct FetchDefinition {
    pub point: Option<DisplayPoint>,
}

#[derive(Clone, PartialEq)]
pub struct GoToFetchedDefinition {
    pub point: Option<DisplayPoint>,
}

impl_internal_actions!(edtior, [FetchDefinition, GoToFetchedDefinition]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(fetch_definition);
    cx.add_action(go_to_fetched_definition);
}

#[derive(Default)]
pub struct LinkGoToDefinitionState {
    pub triggered_from: Option<Anchor>,
    pub symbol_range: Option<Range<Anchor>>,
    pub task: Option<Task<Option<()>>>,
}

pub fn fetch_definition(
    editor: &mut Editor,
    &FetchDefinition { point }: &FetchDefinition,
    cx: &mut ViewContext<Editor>,
) {
    if let Some(point) = point {
        show_link_definition(editor, point, cx);
    } else {
        //TODO: Also needs to be dispatched when cmd modifier is released
        hide_link_definition(editor, cx);
    }
}

pub fn show_link_definition(
    editor: &mut Editor,
    point: DisplayPoint,
    cx: &mut ViewContext<Editor>,
) {
    if editor.pending_rename.is_some() {
        return;
    }

    let snapshot = editor.snapshot(cx);
    let multibuffer_offset = point.to_offset(&snapshot.display_snapshot, Bias::Left);

    let (buffer, buffer_position) = if let Some(output) = editor
        .buffer
        .read(cx)
        .text_anchor_for_position(multibuffer_offset, cx)
    {
        output
    } else {
        return;
    };

    let excerpt_id = if let Some((excerpt_id, _, _)) = editor
        .buffer()
        .read(cx)
        .excerpt_containing(multibuffer_offset, cx)
    {
        excerpt_id
    } else {
        return;
    };

    let project = if let Some(project) = editor.project.clone() {
        project
    } else {
        return;
    };

    // Get input anchor
    let anchor = snapshot
        .buffer_snapshot
        .anchor_at(multibuffer_offset, Bias::Left);

    // Don't request again if the location is the same as the previous request
    if let Some(triggered_from) = &editor.link_go_to_definition_state.triggered_from {
        if triggered_from
            .cmp(&anchor, &snapshot.buffer_snapshot)
            .is_eq()
        {
            return;
        }
    }

    let task = cx.spawn_weak(|this, mut cx| {
        async move {
            // query the LSP for definition info
            let definition_request = cx.update(|cx| {
                project.update(cx, |project, cx| {
                    project.definition(&buffer, buffer_position.clone(), cx)
                })
            });

            let origin_range = definition_request.await.ok().and_then(|definition_result| {
                definition_result
                    .into_iter()
                    .filter_map(|link| {
                        link.origin.map(|origin| {
                            let start = snapshot
                                .buffer_snapshot
                                .anchor_in_excerpt(excerpt_id.clone(), origin.range.start);
                            let end = snapshot
                                .buffer_snapshot
                                .anchor_in_excerpt(excerpt_id.clone(), origin.range.end);

                            start..end
                        })
                    })
                    .next()
            });

            if let Some(this) = this.upgrade(&cx) {
                this.update(&mut cx, |this, cx| {
                    if let Some(origin_range) = origin_range {
                        this.highlight_text::<LinkGoToDefinitionState>(
                            vec![origin_range],
                            HighlightStyle {
                                underline: Some(Underline {
                                    color: Some(Color::red()),
                                    thickness: 1.0.into(),
                                    squiggly: false,
                                }),
                                ..Default::default()
                            },
                            cx,
                        )
                    }
                })
            }

            Ok::<_, anyhow::Error>(())
        }
        .log_err()
    });

    editor.link_go_to_definition_state.task = Some(task);
}

pub fn hide_link_definition(editor: &mut Editor, cx: &mut ViewContext<Editor>) {
    // only notify the context once
    if editor.link_go_to_definition_state.symbol_range.is_some() {
        editor.link_go_to_definition_state.symbol_range.take();
        cx.notify();
    }

    editor.link_go_to_definition_state.task = None;
    editor.link_go_to_definition_state.triggered_from = None;

    editor.clear_text_highlights::<LinkGoToDefinitionState>(cx);
}

pub fn go_to_fetched_definition(
    editor: &mut Editor,
    GoToFetchedDefinition { point }: &GoToFetchedDefinition,
    cx: &mut ViewContext<Editor>,
) {
}
