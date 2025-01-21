use std::rc::Rc;

use collections::HashSet;
use editor::Editor;
use file_icons::FileIcons;
use gpui::{
    AppContext, Bounds, DismissEvent, EventEmitter, FocusHandle, FocusableView, Model,
    Subscription, View, WeakModel, WeakView,
};
use itertools::Itertools;
use language::Buffer;
use ui::{prelude::*, KeyBinding, PopoverMenu, PopoverMenuHandle, Tooltip};
use workspace::{notifications::NotifyResultExt, Workspace};

use crate::context::ContextKind;
use crate::context_picker::{ConfirmBehavior, ContextPicker};
use crate::context_store::ContextStore;
use crate::thread::Thread;
use crate::thread_store::ThreadStore;
use crate::ui::ContextPill;
use crate::{
    AcceptSuggestedContext, AssistantPanel, FocusDown, FocusLeft, FocusRight, FocusUp,
    RemoveAllContext, RemoveFocusedContext, ToggleContextPicker,
};

pub struct ContextStrip {
    context_store: Model<ContextStore>,
    pub context_picker: View<ContextPicker>,
    context_picker_menu_handle: PopoverMenuHandle<ContextPicker>,
    focus_handle: FocusHandle,
    suggest_context_kind: SuggestContextKind,
    workspace: WeakView<Workspace>,
    _subscriptions: Vec<Subscription>,
    focused_index: Option<usize>,
    children_bounds: Option<Vec<Bounds<Pixels>>>,
}

impl ContextStrip {
    pub fn new(
        context_store: Model<ContextStore>,
        workspace: WeakView<Workspace>,
        editor: WeakView<Editor>,
        thread_store: Option<WeakModel<ThreadStore>>,
        context_picker_menu_handle: PopoverMenuHandle<ContextPicker>,
        suggest_context_kind: SuggestContextKind,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let context_picker = cx.new_view(|cx| {
            ContextPicker::new(
                workspace.clone(),
                thread_store.clone(),
                context_store.downgrade(),
                editor.clone(),
                ConfirmBehavior::KeepOpen,
                cx,
            )
        });

        let focus_handle = cx.focus_handle();

        let subscriptions = vec![
            cx.subscribe(&context_picker, Self::handle_context_picker_event),
            cx.on_focus(&focus_handle, Self::handle_focus),
            cx.on_blur(&focus_handle, Self::handle_blur),
        ];

        Self {
            context_store: context_store.clone(),
            context_picker,
            context_picker_menu_handle,
            focus_handle,
            suggest_context_kind,
            workspace,
            _subscriptions: subscriptions,
            focused_index: None,
            children_bounds: None,
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

    fn handle_focus(&mut self, cx: &mut ViewContext<Self>) {
        self.focused_index = self.last_pill_index();
        cx.notify();
    }

    fn handle_blur(&mut self, cx: &mut ViewContext<Self>) {
        self.focused_index = None;
        cx.notify();
    }

    fn focus_left(&mut self, _: &FocusLeft, cx: &mut ViewContext<Self>) {
        self.focused_index = match self.focused_index {
            Some(index) if index > 0 => Some(index - 1),
            _ => self.last_pill_index(),
        };

        cx.notify();
    }

    fn focus_right(&mut self, _: &FocusRight, cx: &mut ViewContext<Self>) {
        let Some(last_index) = self.last_pill_index() else {
            return;
        };

        self.focused_index = match self.focused_index {
            Some(index) if index < last_index => Some(index + 1),
            _ => Some(0),
        };

        cx.notify();
    }

    fn focus_up(&mut self, _: &FocusUp, cx: &mut ViewContext<Self>) {
        let Some(focused_index) = self.focused_index else {
            return;
        };

        if focused_index == 0 {
            return cx.emit(ContextStripEvent::BlurredUp);
        }

        let Some((focused, pills)) = self.focused_bounds(focused_index) else {
            return;
        };

        let iter = pills[..focused_index].iter().enumerate().rev();
        self.focused_index = Self::find_best_horizontal_match(focused, iter).or(Some(0));
        cx.notify();
    }

    fn focus_down(&mut self, _: &FocusDown, cx: &mut ViewContext<Self>) {
        let Some(focused_index) = self.focused_index else {
            return;
        };

        let last_index = self.last_pill_index();

        if self.focused_index == last_index {
            return cx.emit(ContextStripEvent::BlurredDown);
        }

        let Some((focused, pills)) = self.focused_bounds(focused_index) else {
            return;
        };

        let iter = pills.iter().enumerate().skip(focused_index + 1);
        self.focused_index = Self::find_best_horizontal_match(focused, iter).or(last_index);
        cx.notify();
    }

    fn focused_bounds(&self, focused: usize) -> Option<(&Bounds<Pixels>, &[Bounds<Pixels>])> {
        let pill_bounds = self.pill_bounds()?;
        let focused = pill_bounds.get(focused)?;

        Some((focused, pill_bounds))
    }

    fn pill_bounds(&self) -> Option<&[Bounds<Pixels>]> {
        let bounds = self.children_bounds.as_ref()?;
        let eraser = if bounds.len() < 3 { 0 } else { 1 };
        let pills = &bounds[1..bounds.len() - eraser];

        if pills.is_empty() {
            None
        } else {
            Some(pills)
        }
    }

    fn last_pill_index(&self) -> Option<usize> {
        Some(self.pill_bounds()?.len() - 1)
    }

    fn find_best_horizontal_match<'a>(
        focused: &'a Bounds<Pixels>,
        iter: impl Iterator<Item = (usize, &'a Bounds<Pixels>)>,
    ) -> Option<usize> {
        let mut best = None;

        let focused_left = focused.left();
        let focused_right = focused.right();

        for (index, probe) in iter {
            if probe.origin.y == focused.origin.y {
                continue;
            }

            let overlap = probe.right().min(focused_right) - probe.left().max(focused_left);

            best = match best {
                Some((_, prev_overlap, y)) if probe.origin.y != y || prev_overlap > overlap => {
                    break;
                }
                Some(_) | None => Some((index, overlap, probe.origin.y)),
            };
        }

        best.map(|(index, _, _)| index)
    }

    fn remove_focused_context(&mut self, _: &RemoveFocusedContext, cx: &mut ViewContext<Self>) {
        if let Some(index) = self.focused_index {
            let mut is_empty = false;

            self.context_store.update(cx, |this, _cx| {
                if let Some(item) = this.context().get(index) {
                    this.remove_context(item.id());
                }

                is_empty = this.context().is_empty();
            });

            if is_empty {
                cx.emit(ContextStripEvent::BlurredEmpty);
            } else {
                self.focused_index = Some(index.saturating_sub(1));
                cx.notify();
            }
        }
    }

    fn is_suggested_focused<T>(&self, context: &Vec<T>) -> bool {
        // We only suggest one item after the actual context
        self.focused_index == Some(context.len())
    }

    fn accept_suggested_context(&mut self, _: &AcceptSuggestedContext, cx: &mut ViewContext<Self>) {
        if let Some(suggested) = self.suggested_context(cx) {
            let context_store = self.context_store.read(cx);

            if self.is_suggested_focused(context_store.context()) {
                self.add_suggested_context(&suggested, cx);
            }
        }
    }

    fn add_suggested_context(&mut self, suggested: &SuggestedContext, cx: &mut ViewContext<Self>) {
        let task = self.context_store.update(cx, |context_store, cx| {
            context_store.accept_suggested_context(&suggested, cx)
        });

        cx.spawn(|this, mut cx| async move {
            match task.await.notify_async_err(&mut cx) {
                None => {}
                Some(()) => {
                    if let Some(this) = this.upgrade() {
                        this.update(&mut cx, |_, cx| cx.notify())?;
                    }
                }
            }
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        cx.notify();
    }
}

impl FocusableView for ContextStrip {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
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
            .track_focus(&focus_handle)
            .key_context("ContextStrip")
            .on_action(cx.listener(Self::focus_up))
            .on_action(cx.listener(Self::focus_right))
            .on_action(cx.listener(Self::focus_down))
            .on_action(cx.listener(Self::focus_left))
            .on_action(cx.listener(Self::remove_focused_context))
            .on_action(cx.listener(Self::accept_suggested_context))
            .on_children_prepainted({
                let view = cx.view().downgrade();
                move |children_bounds, cx| {
                    view.update(cx, |this, _| {
                        this.children_bounds = Some(children_bounds);
                    })
                    .ok();
                }
            })
            .child(
                PopoverMenu::new("context-picker")
                    .menu(move |cx| {
                        context_picker.update(cx, |this, cx| {
                            this.init(cx);
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
            .children(context.iter().enumerate().map(|(i, context)| {
                ContextPill::new_added(
                    context.clone(),
                    dupe_names.contains(&context.name),
                    self.focused_index == Some(i),
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
                .on_click(Rc::new(cx.listener(move |this, _, cx| {
                    this.focused_index = Some(i);
                    cx.notify();
                })))
            }))
            .when_some(suggested_context, |el, suggested| {
                el.child(
                    ContextPill::new_suggested(
                        suggested.name().clone(),
                        suggested.icon_path(),
                        suggested.kind(),
                        self.is_suggested_focused(&context),
                    )
                    .on_click(Rc::new(cx.listener(move |this, _event, cx| {
                        this.add_suggested_context(&suggested, cx);
                    }))),
                )
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
    BlurredEmpty,
    BlurredDown,
    BlurredUp,
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
