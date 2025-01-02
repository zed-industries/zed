use std::rc::Rc;

use editor::Editor;
use gpui::{AppContext, FocusHandle, Model, View, WeakModel, WeakView};
use language::Buffer;
use project::ProjectEntryId;
use ui::{prelude::*, PopoverMenu, PopoverMenuHandle, Tooltip};
use workspace::Workspace;

use crate::context::ContextKind;
use crate::context_picker::{ConfirmBehavior, ContextPicker};
use crate::context_store::ContextStore;
use crate::thread::{Thread, ThreadId};
use crate::thread_store::ThreadStore;
use crate::ui::ContextPill;
use crate::{AssistantPanel, ToggleContextPicker};
use settings::Settings;

pub struct ContextStrip {
    context_store: Model<ContextStore>,
    context_picker: View<ContextPicker>,
    context_picker_menu_handle: PopoverMenuHandle<ContextPicker>,
    focus_handle: FocusHandle,
    suggest_context_kind: SuggestContextKind,
    workspace: WeakView<Workspace>,
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
            suggest_context_kind,
            workspace,
        }
    }

    fn suggested_context(&self, cx: &ViewContext<Self>) -> Option<SuggestedContext> {
        match self.suggest_context_kind {
            SuggestContextKind::File => self.suggested_file(cx),
            SuggestContextKind::Thread => self.suggested_thread(cx),
        }
    }

    fn suggested_file(&self, cx: &ViewContext<Self>) -> Option<SuggestedContext> {
        let workspace = self.workspace.upgrade()?;
        let active_item = workspace.read(cx).active_item(cx)?;
        let entry_id = *active_item.project_entry_ids(cx).first()?;

        if self.context_store.read(cx).contains_project_entry(entry_id) {
            return None;
        }

        let editor = active_item.to_any().downcast::<Editor>().ok()?.read(cx);
        let active_buffer = editor.buffer().read(cx).as_singleton()?;

        let file = active_buffer.read(cx).file()?;
        let title = file.path().to_string_lossy().into_owned().into();

        Some(SuggestedContext::File {
            entry_id,
            title,
            buffer: active_buffer.downgrade(),
        })
    }

    fn suggested_thread(&self, cx: &ViewContext<Self>) -> Option<SuggestedContext> {
        let workspace = self.workspace.upgrade()?;
        let active_thread = workspace
            .read(cx)
            .panel::<AssistantPanel>(cx)?
            .read(cx)
            .active_thread(cx);
        let weak_active_thread = active_thread.downgrade();

        let active_thread = active_thread.read(cx);

        if self
            .context_store
            .read(cx)
            .contains_thread(active_thread.id())
        {
            return None;
        }

        Some(SuggestedContext::Thread {
            id: active_thread.id().clone(),
            title: active_thread.summary().unwrap_or("Active Thread".into()),
            thread: weak_active_thread,
        })
    }
}

impl Render for ContextStrip {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let context_store = self.context_store.read(cx);
        let context = context_store.context().clone();
        let context_picker = self.context_picker.clone();
        let focus_handle = self.focus_handle.clone();

        let suggested_context = self.suggested_context(cx);

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
            .when(context.is_empty() && suggested_context.is_none(), {
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
                    Button::new("add-suggested-context", suggested.title().clone())
                        .on_click({
                            let context_store = self.context_store.clone();

                            cx.listener(move |_this, _event, cx| {
                                context_store.update(cx, |context_store, cx| {
                                    suggested.accept(context_store, cx);
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

pub enum SuggestContextKind {
    File,
    Thread,
}

#[derive(Clone)]
pub enum SuggestedContext {
    File {
        entry_id: ProjectEntryId,
        title: SharedString,
        buffer: WeakModel<Buffer>,
    },
    Thread {
        id: ThreadId,
        title: SharedString,
        thread: WeakModel<Thread>,
    },
}

impl SuggestedContext {
    pub fn title(&self) -> &SharedString {
        match self {
            Self::File { title, .. } => title,
            Self::Thread { title, .. } => title,
        }
    }

    pub fn accept(&self, context_store: &mut ContextStore, cx: &mut AppContext) {
        match self {
            Self::File {
                entry_id,
                title,
                buffer,
            } => {
                let Some(buffer) = buffer.upgrade() else {
                    return;
                };
                let text = buffer.read(cx).text();

                context_store.insert_context(
                    ContextKind::File(*entry_id),
                    title.clone(),
                    text.clone(),
                );
            }
            Self::Thread { id, title, thread } => {
                let Some(thread) = thread.upgrade() else {
                    return;
                };

                context_store.insert_context(
                    ContextKind::Thread(id.clone()),
                    title.clone(),
                    thread.read(cx).text(),
                );
            }
        }
    }
}
