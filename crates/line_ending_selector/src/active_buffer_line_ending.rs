use editor::Editor;
use gpui::{Entity, Subscription, WeakEntity};
use language::LineEnding;
use ui::{Tooltip, prelude::*};
use workspace::{ItemHandle, StatusItemView, Workspace};

use crate::{LineEndingSelector, Toggle};

pub struct ActiveBufferLineEnding {
    active_line_ending: Option<LineEnding>,
    workspace: WeakEntity<Workspace>,
    _observe_active_editor: Option<Subscription>,
}

impl ActiveBufferLineEnding {
    pub fn new(workspace: &Workspace) -> Self {
        Self {
            active_line_ending: None,
            workspace: workspace.weak_handle(),
            _observe_active_editor: None,
        }
    }

    fn update_line_ending(
        &mut self,
        editor: Entity<Editor>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let editor = editor.read(cx);
        if let Some((_, buffer, _)) = editor.active_excerpt(cx) {
            let line_ending = buffer.read(cx).line_ending();
            self.active_line_ending = Some(line_ending);
        }

        cx.notify();
    }
}

impl Render for ActiveBufferLineEnding {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().when_some(
            self.active_line_ending.as_ref(),
            |el, active_line_ending| {
                let active_line_ending_text = match active_line_ending {
                    LineEnding::Unix => "LF",
                    LineEnding::Windows => "CRLF",
                };

                el.child(
                    Button::new("change-line-ending", active_line_ending_text)
                        .label_size(LabelSize::Small)
                        .on_click(cx.listener(|this, _, window, cx| {
                            if let Some(workspace) = this.workspace.upgrade() {
                                workspace.update(cx, |workspace, cx| {
                                    LineEndingSelector::toggle(workspace, window, cx)
                                });
                            }
                        }))
                        .tooltip(|window, cx| {
                            Tooltip::for_action("Select Line Ending", &Toggle, window, cx)
                        }),
                )
            },
        )
    }
}

impl StatusItemView for ActiveBufferLineEnding {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.downcast::<Editor>()) {
            self._observe_active_editor =
                Some(cx.observe_in(&editor, window, Self::update_line_ending));
            self.update_line_ending(editor, window, cx);
        } else {
            self.active_line_ending = None;
            self._observe_active_editor = None;
        }

        cx.notify();
    }
}
