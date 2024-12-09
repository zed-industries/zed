use editor::Editor;
use gpui::{div, AppContext, IntoElement, ParentElement, Render, Subscription, View, WeakView};
use language::LanguageName;
use ui::{Button, ButtonCommon, Clickable, FluentBuilder, LabelSize, Tooltip};
use workspace::{item::ItemHandle, StatusItemView, Workspace};

use crate::{LanguageSelector, Toggle};

pub struct ActiveBufferLanguage {
    active_language: Option<Option<LanguageName>>,
    workspace: WeakModel<Workspace>,
    _observe_active_editor: Option<Subscription>,
}

impl ActiveBufferLanguage {
    pub fn new(workspace: &Workspace) -> Self {
        Self {
            active_language: None,
            workspace: workspace.weak_handle(),
            _observe_active_editor: None,
        }
    }

    fn update_language(&mut self, editor: Model<Editor>, model: &Model<Self>, cx: &mut AppContext) {
        self.active_language = Some(None);

        let editor = editor.read(cx);
        if let Some((_, buffer, _)) = editor.active_excerpt(cx) {
            if let Some(language) = buffer.read(cx).language() {
                self.active_language = Some(Some(language.name()));
            }
        }

        model.notify(cx);
    }
}

impl Render for ActiveBufferLanguage {
    fn render(
        &mut self,
        model: &Model<Self>,
        window: &mut gpui::Window,
        cx: &mut AppContext,
    ) -> impl IntoElement {
        div().when_some(self.active_language.as_ref(), |el, active_language| {
            let active_language_text = if let Some(active_language_text) = active_language {
                active_language_text.to_string()
            } else {
                "Unknown".to_string()
            };

            el.child(
                Button::new("change-language", active_language_text)
                    .label_size(LabelSize::Small)
                    .on_click(model.listener(|this, model, _, cx| {
                        if let Some(workspace) = this.workspace.upgrade() {
                            workspace.update(cx, |workspace, model, cx| {
                                LanguageSelector::toggle(workspace, model, cx)
                            });
                        }
                    }))
                    .tooltip(|window, cx| {
                        Tooltip::for_action("Select Language", &Toggle, model, cx)
                    }),
            )
        })
    }
}

impl StatusItemView for ActiveBufferLanguage {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.downcast::<Editor>()) {
            self._observe_active_editor = Some(cx.observe(&editor, Self::update_language));
            self.update_language(editor, model, cx);
        } else {
            self.active_language = None;
            self._observe_active_editor = None;
        }

        model.notify(cx);
    }
}
