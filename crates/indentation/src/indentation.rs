use editor::Editor;
use gpui::{Entity, Focusable, FocusHandle, Subscription, Task, WeakEntity};
use settings::{LocalSettingsKind, SettingsStore};
use std::{num::NonZeroU32, time::Duration};
use text::Point;
use ui::{
    div, BorrowAppContext, Button, ButtonCommon, Clickable, Context, FluentBuilder, IntoElement, LabelSize, ParentElement, Render, Tooltip, Window
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
    update_indentation: Task<()>,
    _observe_active_editor: Option<Subscription>,
}

impl Indentation {
    pub fn new(workspace: &Workspace) -> Self {
        Self {
            indentation: None,
            context: None,
            workspace: workspace.weak_handle(),
            update_indentation: Task::ready(()),
            _observe_active_editor: None,
        }
    }

    fn update_indentation(
        &mut self,
        editor: Entity<Editor>,
        debounce: Option<Duration>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let editor = editor.downgrade();
        self.update_indentation = cx.spawn_in(window, |indentation, mut cx| async move {
            let is_singleton = editor
                .update(&mut cx, |editor, cx| {
                    editor.buffer().read(cx).is_singleton()
                })
                .ok()
                .unwrap_or(true);

            if !is_singleton {
                if let Some(debounce) = debounce {
                    cx.background_executor().timer(debounce).await;
                }
            }

            editor
                .update(&mut cx, |editor, cx| {
                    indentation.update(cx, |indentation, cx| {
                        if editor.mode() == editor::EditorMode::Full {
                            let settings = editor.buffer().read(cx).settings_at(Point::new(0, 0), cx);
                            indentation.indentation = Some(IndentationSettings {
                                tab_size: settings.tab_size,
                                hard_tabs: settings.hard_tabs,
                            });
                            indentation.context = Some(editor.focus_handle(cx));
                        } else {
                            indentation.indentation = None;
                            indentation.context = None;
                        }
                    })
                })
                .ok()
                .transpose()
                .ok()
                .flatten();
        });
    }
}

impl Render for Indentation {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().when_some(self.indentation, |el, indentation| {
            let context = self.context.clone();
            el.child(
                Button::new("tab-size", indentation.tab_size.to_string())
                    .label_size(LabelSize::Small)
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Some(workspace) = this.workspace.upgrade() {
                            workspace.update(cx, |workspace, cx| {
                                if let Some(editor) = workspace
                                    .active_item(cx)
                                    .and_then(|item| item.act_as::<Editor>(cx))
                                {
                                    let editor = editor.downgrade();
                                    let _ = editor.update(cx, |editor, cx| {
                                        if let Some(file) = editor.file_at(Point::zero(), cx) {
                                            let _ = cx.update_global(|store: &mut SettingsStore, cx| {
                                                let worktree_id = file.worktree_id(cx);
                                                let path = file.path().clone();
                                                let _ = store.set_local_settings(
                                                    worktree_id,
                                                    path,
                                                    LocalSettingsKind::Editorconfig,
                                                    Some("[/**]\nindent_size = 3\nindent_style = space\ntab_width=3"),
                                                    cx
                                                ).inspect_err(|e| log::error!("set_indent failed: {e}"));
                                            });
                                        }
                                    });
                                }
                            })
                        }
                    }))
                    .tooltip(move |window, cx| match context.as_ref() {
                        Some(context) => Tooltip::for_action_in(
                            "Go to Line/Column",
                            &editor::actions::ToggleGoToLine,
                            context,
                            window,
                            cx,
                        ),
                        None => Tooltip::for_action(
                            "Go to Line/Column",
                            &editor::actions::ToggleGoToLine,
                            window,
                            cx,
                        ),
                    }),
            )
        })
    }
}

const UPDATE_DEBOUNCE: Duration = Duration::from_millis(50);

impl StatusItemView for Indentation {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.act_as::<Editor>(cx)) {
            self._observe_active_editor = Some(cx.observe_in(
                &editor,
                window,
                |indentation, editor, window, cx| {
                    Self::update_indentation(indentation, editor, Some(UPDATE_DEBOUNCE), window, cx)
                },
            ));
            self.update_indentation(editor, None, window, cx);
        } else {
            self.indentation = None;
            self._observe_active_editor = None;
        }

        cx.notify();
    }
}
