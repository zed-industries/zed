use crate::{IndentSizeSelector, Toggle};
use editor::Editor;
use gpui::{Entity, FocusHandle, Subscription, WeakEntity};
use language::language_settings::language_settings;
use std::num::NonZeroU32;
use ui::{
    div, Button, ButtonCommon, Clickable, Context, FluentBuilder, IntoElement, LabelSize,
    ParentElement, Render, Tooltip, Window,
};
use workspace::{ItemHandle, StatusItemView, Workspace};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct IndentationSettings {
    pub tab_size: NonZeroU32,
    pub hard_tabs: bool,
}

pub struct Indentation {
    indentation: Option<IndentationSettings>,
    context: Option<FocusHandle>,
    workspace: WeakEntity<Workspace>,
    _observe_active_editor: Option<Subscription>,
}

impl Indentation {
    pub fn new(workspace: &Workspace) -> Self {
        Self {
            indentation: None,
            context: None,
            workspace: workspace.weak_handle(),
            _observe_active_editor: None,
        }
    }

    fn update_indentation(
        &mut self,
        editor: Entity<Editor>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let editor = editor.read(cx);
        if let Some((_, buffer, _)) = editor.active_excerpt(cx) {
            let buffer = buffer.read(cx);
            if let Some(language) = buffer.language() {
                let file = buffer.file();
                let settings = language_settings(Some(language.name()), file, cx);

                self.indentation = Some(IndentationSettings {
                    tab_size: settings.tab_size,
                    hard_tabs: settings.hard_tabs,
                });
            }
        }

        cx.notify();
    }
}

impl Render for Indentation {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().when_some(self.indentation, |el, indentation| {
            let mode = if indentation.hard_tabs {
                "Tabs"
            } else {
                "Spaces"
            };
            let text = format!("{}:{}", mode, indentation.tab_size);
            let context = self.context.clone();
            el.child(
                Button::new("tab-size", text)
                    .label_size(LabelSize::Small)
                    .on_click(cx.listener(|this, _, window, cx| {
                        if let Some(workspace) = this.workspace.upgrade() {
                            workspace.update(cx, |workspace, cx| {
                                IndentSizeSelector::toggle(workspace, window, cx);
                            })
                        }
                    }))
                    .tooltip(|window, cx| {
                        Tooltip::for_action("Update Indentation", &Toggle, window, cx)
                    }),
            )
        })
    }
}

impl StatusItemView for Indentation {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.downcast::<Editor>()) {
            self._observe_active_editor =
                Some(cx.observe_in(&editor, window, Self::update_indentation));
            self.update_indentation(editor, window, cx);
        } else {
            self.indentation = None;
            self._observe_active_editor = None;
        }

        cx.notify();
    }
}
