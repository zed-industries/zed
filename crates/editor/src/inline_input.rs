use super::*;

pub struct InlineInputState {
    pub editor: Entity<Editor>,
    block_id: CustomBlockId,
    confirm_task: Option<Task<()>>,
    preview_task: Option<Task<()>>,
    pub(crate) preview: Option<InlineInputPreview>,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InlineInputHistoryDirection {
    Older,
    Newer,
}

const INLINE_INPUT_PREVIEW_MIN_LINES: usize = 8;

impl InlineInputPreview {
    fn display_lines(&self, max_lines: usize) -> Vec<SharedString> {
        let text = match self {
            InlineInputPreview::Text(text) => text,
            InlineInputPreview::Error(message) => message,
        };
        let mut lines = text
            .lines()
            .map(|line| {
                if line.is_empty() {
                    SharedString::from(" ")
                } else {
                    SharedString::from(line.to_string())
                }
            })
            .collect::<Vec<_>>();
        if lines.is_empty() {
            lines.push(SharedString::from(" "));
        }
        if lines.len() > max_lines {
            let hidden_lines = lines.len() - max_lines;
            lines.truncate(max_lines);
            let noun = if hidden_lines == 1 { "line" } else { "lines" };
            lines.push(SharedString::from(format!("… +{hidden_lines} more {noun}")));
        }
        lines
    }

    fn is_error(&self) -> bool {
        matches!(self, InlineInputPreview::Error(_))
    }
}

struct RenderedPreview {
    lines: Vec<SharedString>,
    is_error: bool,
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
            Some(Autoscroll::fit()),
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
            confirm_task: None,
            preview_task: None,
            preview: None,
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
        let input = state.editor.clone();
        let rendered = preview.as_ref().map(|preview| RenderedPreview {
            lines: preview.display_lines(max_lines),
            is_error: preview.is_error(),
        });
        let height = 1 + rendered
            .as_ref()
            .map_or(0, |rendered| rendered.lines.len() as u32);
        state.preview = preview;
        let autoscroll = (height != state.block_height).then_some(Autoscroll::fit());
        state.block_height = height;
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

fn render_inline_input_block(
    input: Entity<Editor>,
    preview: Option<RenderedPreview>,
) -> RenderBlock {
    Arc::new(move |cx: &mut BlockContext| {
        v_flex()
            .block_mouse_except_scroll()
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
                let text_style = cx.editor_style.text.clone();
                let color = if preview.is_error {
                    cx.theme().status().error
                } else {
                    cx.theme().colors().text_muted
                };
                this.children(preview.lines.iter().cloned().map(|line| {
                    div()
                        .font_family(text_style.font().family)
                        .text_size(text_style.font_size)
                        .text_color(color)
                        .child(line)
                }))
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
        let preview = InlineInputPreview::Text(text);
        let lines = preview.display_lines(8);
        assert_eq!(lines.len(), 9);
        assert_eq!(lines[8].as_ref(), "… +4 more lines");
        assert_eq!(preview.display_lines(11).len(), 12);
        assert_eq!(preview.display_lines(11)[11].as_ref(), "… +1 more line");
        assert_eq!(preview.display_lines(12).len(), 12);
    }
}
