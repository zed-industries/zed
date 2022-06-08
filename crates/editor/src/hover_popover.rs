use std::{
    ops::Range,
    time::{Duration, Instant},
};

use gpui::{
    actions,
    elements::{Flex, MouseEventHandler, Padding, Text},
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
pub struct HoverAt {
    pub point: Option<DisplayPoint>,
}

actions!(editor, [Hover]);
impl_internal_actions!(editor, [HoverAt]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(hover);
    cx.add_action(hover_at);
}

/// Bindable action which uses the most recent selection head to trigger a hover
fn hover(editor: &mut Editor, _: &Hover, cx: &mut ViewContext<Editor>) {
    let head = editor.selections.newest_display(cx).head();
    show_hover(editor, head, true, cx);
}

/// The internal hover action dispatches between `show_hover` or `hide_hover`
/// depending on whether a point to hover over is provided.
fn hover_at(editor: &mut Editor, action: &HoverAt, cx: &mut ViewContext<Editor>) {
    if let Some(point) = action.point {
        show_hover(editor, point, false, cx);
    } else {
        hide_hover(editor, cx);
    }
}

/// Hides the type information popup.
/// Triggered by the `Hover` action when the cursor is not over a symbol or when the
/// selections changed.
pub fn hide_hover(editor: &mut Editor, cx: &mut ViewContext<Editor>) -> bool {
    let mut did_hide = false;

    // only notify the context once
    if editor.hover_state.popover.is_some() {
        editor.hover_state.popover = None;
        editor.hover_state.hidden_at = Some(Instant::now());
        editor.hover_state.symbol_range = None;
        did_hide = true;
        cx.notify();
    }

    editor.clear_background_highlights::<HoverState>(cx);

    editor.hover_state.task = None;

    did_hide
}

/// Queries the LSP and shows type info and documentation
/// about the symbol the mouse is currently hovering over.
/// Triggered by the `Hover` action when the cursor may be over a symbol.
fn show_hover(
    editor: &mut Editor,
    point: DisplayPoint,
    ignore_timeout: bool,
    cx: &mut ViewContext<Editor>,
) {
    if editor.pending_rename.is_some() {
        return;
    }

    let snapshot = editor.snapshot(cx);
    let multibuffer_offset = point.to_offset(&snapshot.display_snapshot, Bias::Left);

    if let Some(range) = &editor.hover_state.symbol_range {
        if range
            .to_offset(&snapshot.buffer_snapshot)
            .contains(&multibuffer_offset)
        {
            // Hover triggered from same location as last time. Don't show again.
            return;
        }
    }

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

    // query the LSP for hover info
    let hover_request = project.update(cx, |project, cx| {
        project.hover(&buffer, buffer_position.clone(), cx)
    });

    // We should only delay if the hover popover isn't visible, it wasn't recently hidden, and
    // the hover wasn't triggered from the keyboard
    let should_delay = editor.hover_state.popover.is_none() // Hover visible currently
        && editor
            .hover_state
            .hidden_at
            .map(|hidden| hidden.elapsed().as_millis() > 200)
            .unwrap_or(true) // Hover was visible recently enough
        && !ignore_timeout; // Hover triggered from keyboard

    // Get input anchor
    let anchor = snapshot
        .buffer_snapshot
        .anchor_at(multibuffer_offset, Bias::Left);

    let task = cx.spawn_weak(|this, mut cx| {
        async move {
            let delay = if should_delay {
                Some(cx.background().timer(Duration::from_millis(500)))
            } else {
                None
            };

            // Construct new hover popover from hover request
            let hover_popover = hover_request.await.ok().flatten().and_then(|hover_result| {
                if hover_result.contents.is_empty() {
                    return None;
                }

                let range = if let Some(range) = hover_result.range {
                    let start = snapshot
                        .buffer_snapshot
                        .anchor_in_excerpt(excerpt_id.clone(), range.start);
                    let end = snapshot
                        .buffer_snapshot
                        .anchor_in_excerpt(excerpt_id.clone(), range.end);

                    start..end
                } else {
                    anchor.clone()..anchor.clone()
                };

                if let Some(this) = this.upgrade(&cx) {
                    this.update(&mut cx, |this, _| {
                        this.hover_state.symbol_range = Some(range.clone());
                    });
                }

                Some(HoverPopover {
                    project: project.clone(),
                    anchor: range.start.clone(),
                    contents: hover_result.contents,
                })
            });

            if let Some(delay) = delay {
                delay.await;
            }

            if let Some(this) = this.upgrade(&cx) {
                this.update(&mut cx, |this, cx| {
                    if this.hover_state.popover.is_some() || hover_popover.is_some() {
                        // Highlight the selected symbol using a background highlight
                        if let Some(range) = this.hover_state.symbol_range.clone() {
                            this.highlight_background::<HoverState>(
                                vec![range],
                                |theme| theme.editor.hover_popover.highlight,
                                cx,
                            );
                        }

                        this.hover_state.popover = hover_popover;

                        if this.hover_state.popover.is_none() {
                            this.hover_state.hidden_at = Some(Instant::now());
                        }

                        cx.notify();
                    }
                });
            }
            Ok::<_, anyhow::Error>(())
        }
        .log_err()
    });

    editor.hover_state.task = Some(task);
}

#[derive(Default)]
pub struct HoverState {
    pub popover: Option<HoverPopover>,
    pub hidden_at: Option<Instant>,
    pub symbol_range: Option<Range<Anchor>>,
    pub task: Option<Task<Option<()>>>,
}

#[derive(Clone)]
pub struct HoverPopover {
    pub project: ModelHandle<Project>,
    pub anchor: Anchor,
    pub contents: Vec<HoverBlock>,
}

impl HoverPopover {
    pub fn render(
        &self,
        snapshot: &EditorSnapshot,
        style: EditorStyle,
        cx: &mut RenderContext<Editor>,
    ) -> (DisplayPoint, ElementBox) {
        let element = MouseEventHandler::new::<HoverPopover, _, _>(0, cx, |_, cx| {
            let mut flex = Flex::new(Axis::Vertical).scrollable::<HoverBlock, _>(1, None, cx);
            flex.extend(self.contents.iter().map(|content| {
                let project = self.project.read(cx);
                if let Some(language) = content
                    .language
                    .clone()
                    .and_then(|language| project.languages().get_language(&language))
                {
                    let runs = language
                        .highlight_text(&content.text.as_str().into(), 0..content.text.len());

                    Text::new(content.text.clone(), style.text.clone())
                        .with_soft_wrap(true)
                        .with_highlights(
                            runs.iter()
                                .filter_map(|(range, id)| {
                                    id.style(style.theme.syntax.as_ref())
                                        .map(|style| (range.clone(), style))
                                })
                                .collect(),
                        )
                        .boxed()
                } else {
                    let mut text_style = style.hover_popover.prose.clone();
                    text_style.font_size = style.text.font_size;

                    Text::new(content.text.clone(), text_style)
                        .with_soft_wrap(true)
                        .contained()
                        .with_style(style.hover_popover.block_style)
                        .boxed()
                }
            }));
            flex.contained()
                .with_style(style.hover_popover.container)
                .boxed()
        })
        .with_cursor_style(CursorStyle::Arrow)
        .with_padding(Padding {
            bottom: 5.,
            top: 5.,
            ..Default::default()
        })
        .boxed();

        let display_point = self.anchor.to_display_point(&snapshot.display_snapshot);
        (display_point, element)
    }
}
