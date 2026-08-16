use editor::Editor;
use gpui::{App, Entity, Subscription, WeakEntity};
use language::language_settings::LanguageSettings;
use language::Buffer;
use ui::{Tooltip, prelude::*};
use workspace::{
    HideStatusItem, StatusBarSettings, StatusItemView, item::ItemHandle, item::Settings,
};

use crate::{IndentationSelector, Toggle};

#[derive(Default)]
pub struct IndentationIndicator {
    label: Option<SharedString>,
    active_editor: Option<WeakEntity<Editor>>,
    _observe_active_editor: Option<Subscription>,
    _observe_active_buffer: Option<Subscription>,
}

impl IndentationIndicator {
    fn update(&mut self, editor: Entity<Editor>, window: &mut Window, cx: &mut Context<Self>) {
        self.label = None;
        self.active_editor = None;
        self._observe_active_buffer = None;

        if let Some(buffer) = editor.read(cx).active_buffer(cx) {
            self.refresh_label(&buffer, cx);
            self.active_editor = Some(editor.downgrade());
            self._observe_active_buffer =
                Some(cx.observe_in(&buffer, window, |this, buffer, _window, cx| {
                    this.refresh_label(&buffer, cx);
                    cx.notify();
                }));
        }
        cx.notify();
    }

    fn refresh_label(&mut self, buffer: &Entity<Buffer>, cx: &App) {
        let settings = LanguageSettings::for_buffer(buffer.read(cx), cx);
        self.label = Some(if settings.hard_tabs {
            format!("Tab Size: {}", settings.tab_size).into()
        } else {
            format!("Spaces: {}", settings.tab_size).into()
        });
    }
}

impl Render for IndentationIndicator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !StatusBarSettings::get_global(cx).indentation_button {
            return div();
        }

        div().when_some(self.label.clone(), |el, label| {
            el.child(
                Button::new("change-indentation", label)
                    .label_size(LabelSize::Small)
                    .tab_index(0isize)
                    .on_click(cx.listener(|this, _, window, cx| {
                        if let Some(editor) = this.active_editor.as_ref() {
                            IndentationSelector::toggle(editor, window, cx);
                        }
                    }))
                    .tooltip(|_window, cx| {
                        Tooltip::for_action("Change Indentation", &Toggle, cx)
                    }),
            )
        })
    }
}

impl StatusItemView for IndentationIndicator {
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
            self.label = None;
            self._observe_active_editor = None;
            self._observe_active_buffer = None;
        }
        cx.notify();
    }

    fn hide_setting(&self, _: &App) -> Option<HideStatusItem> {
        Some(HideStatusItem::new(|settings| {
            settings
                .status_bar
                .get_or_insert_default()
                .indentation_button = Some(false);
        }))
    }
}
