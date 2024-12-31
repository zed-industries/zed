use std::rc::Rc;

use editor::Editor;
use gpui::{EntityId, FocusHandle, Model, Subscription, View, WeakModel, WeakView};
use language::Buffer;
use project::ProjectEntryId;
use ui::{prelude::*, PopoverMenu, PopoverMenuHandle, Tooltip};
use workspace::{ItemHandle, Workspace};

use crate::context::ContextKind;
use crate::context_picker::{ConfirmBehavior, ContextPicker};
use crate::context_store::ContextStore;
use crate::thread_store::ThreadStore;
use crate::ui::ContextPill;
use crate::ToggleContextPicker;
use settings::Settings;

pub struct ContextStrip {
    context_store: Model<ContextStore>,
    context_picker: View<ContextPicker>,
    context_picker_menu_handle: PopoverMenuHandle<ContextPicker>,
    focus_handle: FocusHandle,
    workspace_active_pane_id: Option<EntityId>,
    suggested_context: Option<SuggestedContext>,
    _subscription: Option<Subscription>,
}

pub enum SuggestContextKind {
    File,
    Thread,
}

#[derive(Clone)]
pub struct SuggestedContext {
    entry_id: ProjectEntryId,
    title: SharedString,
    buffer: WeakModel<Buffer>,
}

impl ContextStrip {
    pub fn new(
        context_store: Model<ContextStore>,
        workspace: WeakView<Workspace>,
        thread_store: Option<WeakModel<ThreadStore>>,
        focus_handle: FocusHandle,
        context_picker_menu_handle: PopoverMenuHandle<ContextPicker>,
        suggest_context_kind: SuggestContextKind,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let subscription = match suggest_context_kind {
            SuggestContextKind::File => {
                if let Some(workspace) = workspace.upgrade() {
                    Some(cx.subscribe(&workspace, Self::handle_workspace_event))
                } else {
                    None
                }
            }
            SuggestContextKind::Thread => {
                // TODO: Suggest current thread
                None
            }
        };

        Self {
            context_store: context_store.clone(),
            context_picker: cx.new_view(|cx| {
                ContextPicker::new(
                    workspace.clone(),
                    thread_store.clone(),
                    context_store.downgrade(),
                    ConfirmBehavior::KeepOpen,
                    cx,
                )
            }),
            context_picker_menu_handle,
            focus_handle,
            workspace_active_pane_id: None,
            suggested_context: None,
            _subscription: subscription,
        }
    }

    fn handle_workspace_event(
        &mut self,
        workspace: View<Workspace>,
        event: &workspace::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            workspace::Event::WorkspaceCreated(_) | workspace::Event::ActiveItemChanged => {
                let workspace = workspace.read(cx);

                if let Some(active_item) = workspace.active_item(cx) {
                    let new_active_item_id = Some(active_item.item_id());

                    if self.workspace_active_pane_id != new_active_item_id {
                        self.suggested_context = Self::suggested_file(active_item, cx);
                        self.workspace_active_pane_id = new_active_item_id;
                    }
                } else {
                    self.suggested_context = None;
                    self.workspace_active_pane_id = None;
                }
            }
            _ => {}
        }
    }

    fn suggested_file(
        active_item: Box<dyn ItemHandle>,
        cx: &WindowContext,
    ) -> Option<SuggestedContext> {
        let entry_id = *active_item.project_entry_ids(cx).first()?;

        let editor = active_item.to_any().downcast::<Editor>().ok()?.read(cx);
        let active_buffer = editor.buffer().read(cx).as_singleton()?;

        let file = active_buffer.read(cx).file()?;
        let title = file.path().to_string_lossy().into_owned().into();

        Some(SuggestedContext {
            entry_id,
            title,
            buffer: active_buffer.downgrade(),
        })
    }
}

impl Render for ContextStrip {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let context_store = self.context_store.read(cx);
        let context = context_store.context().clone();
        let context_picker = self.context_picker.clone();
        let focus_handle = self.focus_handle.clone();

        let suggested_context = self.suggested_context.as_ref().and_then(|suggested| {
            if context_store.contains_project_entry(suggested.entry_id) {
                None
            } else {
                Some(suggested.clone())
            }
        });

        h_flex()
            .flex_wrap()
            .gap_1()
            .child(
                PopoverMenu::new("context-picker")
                    .menu(move |_cx| Some(context_picker.clone()))
                    .trigger(
                        IconButton::new("add-context", IconName::Plus)
                            .icon_size(IconSize::Small)
                            .style(ui::ButtonStyle::Filled)
                            .tooltip({
                                let focus_handle = focus_handle.clone();

                                move |cx| {
                                    Tooltip::for_action_in(
                                        "Add Context",
                                        &ToggleContextPicker,
                                        &focus_handle,
                                        cx,
                                    )
                                }
                            }),
                    )
                    .attach(gpui::Corner::TopLeft)
                    .anchor(gpui::Corner::BottomLeft)
                    .offset(gpui::Point {
                        x: px(0.0),
                        y: px(-16.0),
                    })
                    .with_handle(self.context_picker_menu_handle.clone()),
            )
            .when(context.is_empty() && self.suggested_context.is_none(), {
                |parent| {
                    parent.child(
                        h_flex()
                            .id("no-content-info")
                            .ml_1p5()
                            .gap_2()
                            .font(theme::ThemeSettings::get_global(cx).buffer_font.clone())
                            .text_size(TextSize::Small.rems(cx))
                            .text_color(cx.theme().colors().text_muted)
                            .child("Add Context")
                            .children(
                                ui::KeyBinding::for_action_in(
                                    &ToggleContextPicker,
                                    &focus_handle,
                                    cx,
                                )
                                .map(|binding| binding.into_any_element()),
                            )
                            .opacity(0.5),
                    )
                }
            })
            .children(context.iter().map(|context| {
                ContextPill::new(context.clone()).on_remove({
                    let context = context.clone();
                    let context_store = self.context_store.clone();
                    Rc::new(cx.listener(move |_this, _event, cx| {
                        context_store.update(cx, |this, _cx| {
                            this.remove_context(&context.id);
                        });
                        cx.notify();
                    }))
                })
            }))
            .when_some(suggested_context, |el, suggested| {
                el.child(
                    Button::new("add-suggested-context", suggested.title.clone())
                        .on_click({
                            let context_store = self.context_store.clone();

                            cx.listener(move |_this, _event, cx| {
                                let Some(buffer) = suggested.buffer.upgrade() else {
                                    return;
                                };

                                let title = suggested.title.clone();
                                let text = buffer.read(cx).text();

                                context_store.update(cx, move |context_store, _cx| {
                                    context_store.insert_context(
                                        ContextKind::File(suggested.entry_id),
                                        title,
                                        text,
                                    );
                                });
                                cx.notify();
                            })
                        })
                        .icon(IconName::Plus)
                        .icon_position(IconPosition::Start)
                        .icon_size(IconSize::XSmall)
                        .icon_color(Color::Muted)
                        .label_size(LabelSize::Small)
                        .style(ButtonStyle::Filled)
                        .tooltip(|cx| {
                            Tooltip::with_meta("Suggested Context", None, "Click to add it", cx)
                        }),
                )
            })
            .when(!context.is_empty(), {
                move |parent| {
                    parent.child(
                        IconButton::new("remove-all-context", IconName::Eraser)
                            .icon_size(IconSize::Small)
                            .tooltip(move |cx| Tooltip::text("Remove All Context", cx))
                            .on_click({
                                let context_store = self.context_store.clone();
                                cx.listener(move |_this, _event, cx| {
                                    context_store.update(cx, |this, _cx| this.clear());
                                    cx.notify();
                                })
                            }),
                    )
                }
            })
    }
}
