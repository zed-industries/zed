use gpui::StyledText;
use language::HighlightId;

use super::*;

pub struct InlineInputState {
    pub editor: Entity<Editor>,
    block_id: CustomBlockId,
    position: Anchor,
    confirm_task: Option<Task<()>>,
    preview_task: Option<Task<()>>,
    pub(crate) preview: Option<InlineInputPreview>,
    preview_language: Option<Arc<Language>>,
    block_height: u32,
    history: Vec<SharedString>,
    history_ix: Option<usize>,
    draft: String,
    _subscription: Subscription,
    on_confirm:
        Rc<dyn Fn(&mut Editor, String, &mut Window, &mut Context<Editor>) -> Option<Task<()>>>,
}

impl InlineInputState {
    pub(crate) fn take_confirm_task(&mut self) -> Option<Task<()>> {
        self.confirm_task.take()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum InlineInputPreview {
    Text(String),
    Error(String),
}

#[derive(Clone, Copy)]
pub(crate) enum InlineInputHistoryDirection {
    Older,
    Newer,
}

const INLINE_INPUT_PREVIEW_MIN_LINES: usize = 8;

impl InlineInputPreview {
    fn display_text(&self, max_lines: usize) -> (String, u32) {
        let text = match self {
            InlineInputPreview::Text(text) => text,
            InlineInputPreview::Error(message) => message,
        };
        let line_count = text.lines().count().max(1);
        if line_count <= max_lines {
            return (text.to_string(), line_count as u32);
        }
        let hidden_lines = line_count - max_lines;
        let noun = if hidden_lines == 1 { "line" } else { "lines" };
        let shown = text.lines().take(max_lines).collect::<Vec<_>>().join("\n");
        (
            format!("{shown}\n… +{hidden_lines} more {noun}"),
            max_lines as u32 + 1,
        )
    }

    fn is_error(&self) -> bool {
        matches!(self, InlineInputPreview::Error(_))
    }
}

struct RenderedPreview {
    text: SharedString,
    highlights: Vec<(Range<usize>, HighlightId)>,
    height_in_lines: u32,
    is_error: bool,
}

impl RenderedPreview {
    fn new(
        preview: &InlineInputPreview,
        language: Option<&Arc<Language>>,
        max_lines: usize,
    ) -> Self {
        let (text, height_in_lines) = preview.display_text(max_lines);
        let highlights = match (preview, language) {
            (InlineInputPreview::Text(_), Some(language)) => {
                language.highlight_text(&Rope::from(text.as_str()), 0..text.len())
            }
            _ => Vec::new(),
        };
        Self {
            text: SharedString::from(text),
            highlights,
            height_in_lines,
            is_error: preview.is_error(),
        }
    }
}

impl Editor {
    pub fn pending_inline_input(&self) -> Option<&InlineInputState> {
        self.pending_inline_input.as_ref()
    }

    pub fn confirm_inline_input(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(state) = self.pending_inline_input.as_ref() else {
            return;
        };
        let text = state.editor.read(cx).text(cx);
        let on_confirm = state.on_confirm.clone();
        if let Some(task) = on_confirm(self, text, window, cx)
            && let Some(state) = self.pending_inline_input.as_mut()
        {
            state.confirm_task = Some(task);
        }
    }

    pub(crate) fn show_inline_input(
        &mut self,
        placeholder: &str,
        position: Anchor,
        preview_language: Option<Arc<Language>>,
        history: Vec<SharedString>,
        on_confirm: impl Fn(&mut Editor, String, &mut Window, &mut Context<Editor>) -> Option<Task<()>>
        + 'static,
        on_change: impl Fn(&mut Editor, String, &mut Window, &mut Context<Editor>) -> Option<Task<()>>
        + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.take_inline_input(window, cx);

        let input = cx.new(|cx| {
            let mut input = Editor::single_line(window, cx);
            input.set_placeholder_text(placeholder, window, cx);
            if let Some(newest) = history.first() {
                input.set_text(newest.clone(), window, cx);
                input.select_all(&SelectAll, window, cx);
            }
            input
        });
        let subscription = cx.subscribe_in(
            &input,
            window,
            move |editor, input, event: &EditorEvent, window, cx| match event {
                EditorEvent::Focused => cx.emit(EditorEvent::FocusedIn),
                EditorEvent::BufferEdited => {
                    let Some(state) = editor.pending_inline_input.as_mut() else {
                        return;
                    };
                    let text = input.read(cx).text(cx);
                    if state.history_ix.is_some_and(|ix| {
                        state.history.get(ix).map(SharedString::as_ref) != Some(text.as_str())
                    }) {
                        state.history_ix = None;
                    }
                    let task = on_change(editor, text, window, cx);
                    if let Some(state) = editor.pending_inline_input.as_mut() {
                        state.preview_task = task;
                    }
                }
                _ => {}
            },
        );
        let block_ids = self.insert_blocks(
            [BlockProperties {
                style: BlockStyle::Flex,
                placement: BlockPlacement::Below(position),
                height: Some(1),
                render: render_inline_input_block(input.clone(), None),
                priority: 0,
            }],
            Some(reveal_block_below(
                position,
                &self.buffer().read(cx).snapshot(cx),
            )),
            cx,
        );
        let Some(&block_id) = block_ids.first() else {
            return;
        };

        let input_focus_handle = input.focus_handle(cx);
        window.focus(&input_focus_handle, cx);
        self.pending_inline_input = Some(InlineInputState {
            editor: input,
            block_id,
            position,
            confirm_task: None,
            preview_task: None,
            preview: None,
            preview_language,
            block_height: 1,
            history_ix: (!history.is_empty()).then_some(0),
            history,
            draft: String::new(),
            _subscription: subscription,
            on_confirm: Rc::new(on_confirm),
        });
    }

    pub(crate) fn set_inline_input_preview(
        &mut self,
        preview: Option<InlineInputPreview>,
        cx: &mut Context<Self>,
    ) {
        let max_lines = self.inline_input_preview_max_lines();
        let Some(state) = self.pending_inline_input.as_mut() else {
            return;
        };
        let block_id = state.block_id;
        let position = state.position;
        let input = state.editor.clone();
        let rendered = preview.as_ref().map(|preview| {
            RenderedPreview::new(preview, state.preview_language.as_ref(), max_lines)
        });
        let height = 1 + rendered
            .as_ref()
            .map_or(0, |rendered| rendered.height_in_lines);
        state.preview = preview;
        let resized = height != state.block_height;
        state.block_height = height;
        let autoscroll =
            resized.then(|| reveal_block_below(position, &self.buffer().read(cx).snapshot(cx)));
        self.replace_blocks(
            HashMap::from_iter([(block_id, render_inline_input_block(input, rendered))]),
            None,
            cx,
        );
        self.resize_blocks(HashMap::from_iter([(block_id, height)]), autoscroll, cx);
    }

    pub(crate) fn cycle_inline_input_history(
        &mut self,
        direction: InlineInputHistoryDirection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(state) = self.pending_inline_input.as_mut() else {
            return false;
        };
        let next_ix = match (direction, state.history_ix) {
            (InlineInputHistoryDirection::Older, None) => {
                if state.history.is_empty() {
                    return true;
                }
                state.draft = state.editor.read(cx).text(cx);
                Some(0)
            }
            (InlineInputHistoryDirection::Older, Some(ix)) => {
                if ix + 1 >= state.history.len() {
                    return true;
                }
                Some(ix + 1)
            }
            (InlineInputHistoryDirection::Newer, None) => return true,
            (InlineInputHistoryDirection::Newer, Some(0)) => None,
            (InlineInputHistoryDirection::Newer, Some(ix)) => Some(ix - 1),
        };
        state.history_ix = next_ix;
        let text = match next_ix {
            Some(ix) => state.history.get(ix).cloned().unwrap_or_default(),
            None => SharedString::from(mem::take(&mut state.draft)),
        };
        let input = state.editor.clone();
        input.update(cx, |input, cx| {
            input.set_text(text, window, cx);
            input.select_all(&SelectAll, window, cx);
        });
        true
    }

    pub(crate) fn take_inline_input(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<InlineInputState> {
        let state = self.pending_inline_input.take()?;
        if state.editor.focus_handle(cx).is_focused(window) {
            window.focus(&self.focus_handle, cx);
        }
        self.remove_blocks(
            HashSet::from_iter([state.block_id]),
            Some(Autoscroll::fit()),
            cx,
        );
        Some(state)
    }

    fn inline_input_preview_max_lines(&self) -> usize {
        let half_viewport = (self.visible_line_count().unwrap_or(0.) / 2.).floor() as usize;
        half_viewport.max(INLINE_INPUT_PREVIEW_MIN_LINES)
    }
}

fn reveal_block_below(position: Anchor, snapshot: &MultiBufferSnapshot) -> Autoscroll {
    let row_below = position.to_point(snapshot).row + 1;
    if row_below > snapshot.max_point().row {
        return Autoscroll::center().for_anchor(position);
    }
    Autoscroll::fit().for_anchor(snapshot.anchor_before(Point::new(row_below, 0)))
}

fn render_inline_input_block(
    input: Entity<Editor>,
    preview: Option<RenderedPreview>,
) -> RenderBlock {
    Arc::new(move |cx: &mut BlockContext| {
        v_flex()
            .block_mouse_except_scroll()
            .w_full()
            .bg(cx.theme().colors().elevated_surface_background)
            .pl(cx.anchor_x)
            .child(EditorElement::new(
                &input,
                EditorStyle {
                    background: cx.theme().system().transparent,
                    local_player: cx.editor_style.local_player,
                    text: cx.editor_style.text.clone(),
                    scrollbar_width: cx.editor_style.scrollbar_width,
                    syntax: cx.editor_style.syntax.clone(),
                    status: cx.editor_style.status.clone(),
                    ..EditorStyle::default()
                },
            ))
            .when_some(preview.as_ref(), |this, preview| {
                let mut text_style = cx.editor_style.text.clone();
                text_style.color = if preview.is_error {
                    cx.theme().status().error
                } else {
                    cx.theme().colors().text_muted
                };
                let syntax = cx.theme().syntax();
                let highlights = preview
                    .highlights
                    .iter()
                    .filter_map(|(range, highlight_id)| {
                        Some((range.clone(), *syntax.get(*highlight_id)?))
                    });
                this.child(
                    StyledText::new(preview.text.clone())
                        .with_default_highlights(&text_style, highlights),
                )
            })
            .into_any_element()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preview_truncation_reports_hidden_lines() {
        let text = (1..=12)
            .map(|ix| format!("line{ix}"))
            .collect::<Vec<_>>()
            .join("\n");
        let preview = InlineInputPreview::Text(text.clone());
        let (shown, height) = preview.display_text(8);
        assert_eq!(height, 9);
        assert_eq!(
            shown,
            format!(
                "{}\n… +4 more lines",
                text.lines().take(8).collect::<Vec<_>>().join("\n")
            )
        );
        let (shown, height) = preview.display_text(11);
        assert_eq!(height, 12);
        assert_eq!(shown.lines().last(), Some("… +1 more line"));
        assert_eq!(preview.display_text(12), (text, 12));
    }
}
