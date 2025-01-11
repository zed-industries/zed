use std::rc::Rc;

use collections::HashSet;
use editor::Editor;
use file_icons::FileIcons;
use gpui::{
    DismissEvent, EventEmitter, FocusHandle, Model, Subscription, View, WeakModel, WeakView,
};
use itertools::Itertools;
use language::Buffer;
use ui::{prelude::*, KeyBinding, PopoverMenu, PopoverMenuHandle, Tooltip};
use workspace::Workspace;

use crate::context::ContextKind;
use crate::context_picker::{ConfirmBehavior, ContextPicker};
use crate::context_store::ContextStore;
use crate::thread::Thread;
use crate::thread_store::ThreadStore;
use crate::ui::ContextPill;
use crate::{AssistantPanel, RemoveAllContext, ToggleContextPicker};

pub struct ContextStrip {
    context_store: Model<ContextStore>,
    pub context_picker: View<ContextPicker>,
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
        let active_buffer_model = editor.buffer().read(cx).as_singleton()?;
        let active_buffer = active_buffer_model.read(cx);

        let path = active_buffer.file()?.path();

        if self
            .context_store
            .read(cx)
            .will_include_buffer(active_buffer.remote_id(), path)
            .is_some()
        {
            return None;
        }

        let name = match path.file_name() {
            Some(name) => name.to_string_lossy().into_owned().into(),
            None => path.to_string_lossy().into_owned().into(),
        };

        let icon_path = FileIcons::get_icon(path, cx);

        Some(SuggestedContext::File {
            name,
            buffer: active_buffer_model.downgrade(),
            icon_path,
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
            .includes_thread(active_thread.id())
            .is_some()
        {
            return None;
        }

        Some(SuggestedContext::Thread {
            name: active_thread.summary_or_default(),
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
        let context = context_store
            .context()
            .iter()
            .flat_map(|context| context.snapshot(cx))
            .collect::<Vec<_>>();
        let context_picker = self.context_picker.clone();
        let focus_handle = self.focus_handle.clone();

        let suggested_context = self.suggested_context(cx);

        let dupe_names = context
            .iter()
            .map(|context| context.name.clone())
            .sorted()
            .tuple_windows()
            .filter(|(a, b)| a == b)
            .map(|(a, _)| a)
            .collect::<HashSet<SharedString>>();

        h_flex()
            .flex_wrap()
            .gap_1()
            .child(
                PopoverMenu::new("context-picker")
                    .menu(move |cx| {
                        context_picker.update(cx, |this, cx| {
                            this.reset_mode(cx);
                        });

                        Some(context_picker.clone())
                    })
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
                        y: px(-2.0),
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
                        let id = context.id;
                        let context_store = self.context_store.clone();
                        Rc::new(cx.listener(move |_this, _event, cx| {
                            context_store.update(cx, |this, _cx| {
                                this.remove_context(id);
                            });
                            cx.notify();
                        }))
                    }),
                )
            }))
            .when_some(suggested_context, |el, suggested| {
                el.child(ContextPill::new_suggested(
                    suggested.name().clone(),
                    suggested.icon_path(),
                    suggested.kind(),
                    {
                        let context_store = self.context_store.clone();
                        Rc::new(cx.listener(move |this, _event, cx| {
                            let task = context_store.update(cx, |context_store, cx| {
                                context_store.accept_suggested_context(&suggested, cx)
                            });

                            let workspace = this.workspace.clone();
                            cx.spawn(|this, mut cx| async move {
                                match task.await {
                                    Ok(()) => {
                                        if let Some(this) = this.upgrade() {
                                            this.update(&mut cx, |_, cx| cx.notify())?;
                                        }
                                    }
                                    Err(err) => {
                                        let Some(workspace) = workspace.upgrade() else {
                                            return anyhow::Ok(());
                                        };

                                        workspace.update(&mut cx, |workspace, cx| {
                                            workspace.show_error(&err, cx);
                                        })?;
                                    }
                                }
                                anyhow::Ok(())
                            })
                            .detach_and_log_err(cx);
                        }))
                    },
                ))
            })
            .when(!context.is_empty(), {
                move |parent| {
                    parent.child(
                        IconButton::new("remove-all-context", IconName::Eraser)
                            .icon_size(IconSize::Small)
                            .tooltip({
                                let focus_handle = focus_handle.clone();
                                move |cx| {
                                    Tooltip::for_action_in(
                                        "Remove All Context",
                                        &RemoveAllContext,
                                        &focus_handle,
                                        cx,
                                    )
                                }
                            })
                            .on_click(cx.listener({
                                let focus_handle = focus_handle.clone();
                                move |_this, _event, cx| {
                                    focus_handle.dispatch_action(&RemoveAllContext, cx);
                                }
                            })),
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
        icon_path: Option<SharedString>,
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

    pub fn icon_path(&self) -> Option<SharedString> {
        match self {
            Self::File { icon_path, .. } => icon_path.clone(),
            Self::Thread { .. } => None,
        }
    }

    pub fn kind(&self) -> ContextKind {
        match self {
            Self::File { .. } => ContextKind::File,
            Self::Thread { .. } => ContextKind::Thread,
        }
    }
}
