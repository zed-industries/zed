use editor::Editor;
use gpui::{Entity, Subscription, WeakEntity};
use language::LineEnding;
use ui::{Tooltip, prelude::*};
use workspace::{StatusBarSettings, StatusItemView, item::ItemHandle, item::Settings};

use crate::{LineEndingSelector, Toggle};

#[derive(Default)]
pub struct LineEndingIndicator {
    line_ending: Option<LineEnding>,
    active_editor: Option<WeakEntity<Editor>>,
    _observe_active_editor: Option<Subscription>,
}

impl LineEndingIndicator {
    fn update(&mut self, editor: Entity<Editor>, _: &mut Window, cx: &mut Context<Self>) {
        self.line_ending = None;
        self.active_editor = None;

        if let Some((_, buffer, _)) = editor.read(cx).active_excerpt(cx) {
            let line_ending = buffer.read(cx).line_ending();
            self.line_ending = Some(line_ending);
            self.active_editor = Some(editor.downgrade());
        }

        cx.notify();
    }
}

impl Render for LineEndingIndicator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !StatusBarSettings::get_global(cx).line_endings_button {
            return div();
        }

        div().when_some(self.line_ending.as_ref(), |el, line_ending| {
            el.child(
                Button::new("change-line-ending", line_ending.label())
                    .label_size(LabelSize::Small)
                    .on_click(cx.listener(|this, _, window, cx| {
                        if let Some(editor) = this.active_editor.as_ref() {
                            LineEndingSelector::toggle(editor, window, cx);
                        }
                    }))
                    .tooltip(|_window, cx| Tooltip::for_action("Select Line Ending", &Toggle, cx)),
            )
        })
    }
}

impl StatusItemView for LineEndingIndicator {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.downcast::<Editor>()) {
            self._observe_active_editor = Some(cx.observe_in(&editor, window, Self::update));
            self.update(editor, window, cx);
        } else {
            self.line_ending = None;
            self._observe_active_editor = None;
        }
        cx.notify();
    }
}
