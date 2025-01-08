use std::rc::Rc;

use editor::Editor;
use gpui::{
    AppContext, DismissEvent, EventEmitter, FocusHandle, Model, Subscription, View, WeakModel,
    WeakView,
};
use language::Buffer;
use ui::{prelude::*, KeyBinding, PopoverMenu, PopoverMenuHandle, Tooltip};
use workspace::Workspace;

use crate::context::ContextKind;
use crate::context_picker::{ConfirmBehavior, ContextPicker};
use crate::context_store::ContextStore;
use crate::thread::Thread;
use crate::thread_store::ThreadStore;
use crate::ui::ContextPill;
use crate::{AssistantPanel, ToggleContextPicker};

pub struct ContextStrip {
    context_store: Model<ContextStore>,
    context_picker: View<ContextPicker>,
    context_picker_menu_handle: PopoverMenuHandle<ContextPicker>,
    focus_handle: FocusHandle,
    suggest_context_kind: SuggestContextKind,
    workspace: WeakView<Workspace>,
    _context_picker_subscription: Subscription,
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
        let context_picker = cx.new_view(|cx| {
            ContextPicker::new(
                workspace.clone(),
                thread_store.clone(),
                context_store.downgrade(),
                ConfirmBehavior::KeepOpen,
                cx,
            )
        });

        let context_picker_subscription =
            cx.subscribe(&context_picker, Self::handle_context_picker_event);

        Self {
            context_store: context_store.clone(),
            context_picker,
            context_picker_menu_handle,
            focus_handle,
            suggest_context_kind,
            workspace,
            _context_picker_subscription: context_picker_subscription,
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

        let editor = active_item.to_any().downcast::<Editor>().ok()?.read(cx);
        let active_buffer = editor.buffer().read(cx).as_singleton()?;

        let path = active_buffer.read(cx).file()?.path();

        if self.context_store.read(cx).included_file(path).is_some() {
            return None;
        }

        let name = match path.file_name() {
            Some(name) => name.to_string_lossy().into_owned().into(),
            None => path.to_string_lossy().into_owned().into(),
        };

        Some(SuggestedContext::File {
            name,
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
            .included_thread(active_thread.id())
            .is_some()
        {
            return None;
        }

        Some(SuggestedContext::Thread {
            name: active_thread.summary().unwrap_or("New Thread".into()),
            thread: weak_active_thread,
        })
    }

    fn handle_context_picker_event(
        &mut self,
        _picker: View<ContextPicker>,
        _event: &DismissEvent,
        cx: &mut ViewContext<Self>,
    ) {
        cx.emit(ContextStripEvent::PickerDismissed);
    }
}

impl Render for ContextStrip {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let context_store = self.context_store.read(cx);
        let context = context_store.context().clone();
        let context_picker = self.context_picker.clone();
        let focus_handle = self.focus_handle.clone();

        let suggested_context = self.suggested_context(cx);

        let dupe_names = context_store.duplicated_names();

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
                            .ml_1p5()
                            .gap_2()
                            .child(
                                Label::new("Add Context")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .opacity(0.5)
                            .children(
                                KeyBinding::for_action_in(&ToggleContextPicker, &focus_handle, cx)
                                    .map(|binding| binding.into_any_element()),
                            ),
                    )
                }
            })
            .children(context.iter().map(|context| {
                ContextPill::new_added(
                    context.clone(),
                    dupe_names.contains(&context.name),
                    Some({
                        let context = context.clone();
                        let context_store = self.context_store.clone();
                        Rc::new(cx.listener(move |_this, _event, cx| {
                            context_store.update(cx, |this, _cx| {
                                this.remove_context(&context.id);
                            });
                            cx.notify();
                        }))
                    }),
                )
            }))
            .when_some(suggested_context, |el, suggested| {
                el.child(ContextPill::new_suggested(
                    suggested.name().clone(),
                    suggested.kind(),
                    {
                        let context_store = self.context_store.clone();
                        Rc::new(cx.listener(move |_this, _event, cx| {
                            context_store.update(cx, |context_store, cx| {
                                suggested.accept(context_store, cx);
                            });

                            cx.notify();
                        }))
                    },
                ))
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

pub enum ContextStripEvent {
    PickerDismissed,
}

impl EventEmitter<ContextStripEvent> for ContextStrip {}

pub enum SuggestContextKind {
    File,
    Thread,
}

#[derive(Clone)]
pub enum SuggestedContext {
    File {
        name: SharedString,
        buffer: WeakModel<Buffer>,
    },
    Thread {
        name: SharedString,
        thread: WeakModel<Thread>,
    },
}

impl SuggestedContext {
    pub fn name(&self) -> &SharedString {
        match self {
            Self::File { name, .. } => name,
            Self::Thread { name, .. } => name,
        }
    }

    pub fn accept(&self, context_store: &mut ContextStore, cx: &mut AppContext) {
        match self {
            Self::File { buffer, name: _ } => {
                if let Some(buffer) = buffer.upgrade() {
                    context_store.insert_file(buffer.read(cx));
                };
            }
            Self::Thread { thread, name: _ } => {
                if let Some(thread) = thread.upgrade() {
                    context_store.insert_thread(thread.read(cx));
                };
            }
        }
    }

    pub fn kind(&self) -> ContextKind {
        match self {
            Self::File { .. } => ContextKind::File,
            Self::Thread { .. } => ContextKind::Thread,
        }
    }
}
