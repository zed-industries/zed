use std::path::Path;
use std::rc::Rc;

use collections::{HashSet, IndexSet};
use editor::Editor;
use file_icons::FileIcons;
use gpui::{
    App, Bounds, ClickEvent, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    Subscription, WeakEntity,
};
use itertools::Itertools;
use language::Buffer;
use project::ProjectItem;
use ui::{KeyBinding, PopoverMenu, PopoverMenuHandle, Tooltip, prelude::*};
use workspace::{Workspace, notifications::NotifyResultExt};

use crate::context::{AssistantContext, ContextKind};
use crate::context_picker::ContextPicker;
use crate::context_store::ContextStore;
use crate::thread::Thread;
use crate::thread_store::ThreadStore;
use crate::ui::{AddedContext, ContextPill};
use crate::{
    AcceptSuggestedContext, AssistantPanel, FocusDown, FocusLeft, FocusRight, FocusUp,
    RemoveAllContext, RemoveFocusedContext, ToggleContextPicker,
};

pub struct ContextStrip {
    context_store: Entity<ContextStore>,
    context_picker: Entity<ContextPicker>,
    context_picker_menu_handle: PopoverMenuHandle<ContextPicker>,
    focus_handle: FocusHandle,
    suggest_context_kind: SuggestContextKind,
    workspace: WeakEntity<Workspace>,
    _subscriptions: Vec<Subscription>,
    focused_index: Option<usize>,
    children_bounds: Option<Vec<Bounds<Pixels>>>,
}

impl ContextStrip {
    pub fn new(
        context_store: Entity<ContextStore>,
        workspace: WeakEntity<Workspace>,
        thread_store: Option<WeakEntity<ThreadStore>>,
        context_picker_menu_handle: PopoverMenuHandle<ContextPicker>,
        suggest_context_kind: SuggestContextKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let context_picker = cx.new(|cx| {
            ContextPicker::new(
                workspace.clone(),
                thread_store.clone(),
                context_store.downgrade(),
                window,
                cx,
            )
        });

        let focus_handle = cx.focus_handle();

        let subscriptions = vec![
            cx.observe(&context_store, |_, _, cx| cx.notify()),
            cx.subscribe_in(&context_picker, window, Self::handle_context_picker_event),
            cx.on_focus(&focus_handle, window, Self::handle_focus),
            cx.on_blur(&focus_handle, window, Self::handle_blur),
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

    fn suggested_context(&self, cx: &Context<Self>) -> Option<SuggestedContext> {
        match self.suggest_context_kind {
            SuggestContextKind::File => self.suggested_file(cx),
            // SuggestContextKind::Thread => self.suggested_thread(cx),
        }
    }

    fn suggested_file(&self, cx: &Context<Self>) -> Option<SuggestedContext> {
        let workspace = self.workspace.upgrade()?;
        let active_item = workspace.read(cx).active_item(cx)?;

        let editor = active_item.to_any().downcast::<Editor>().ok()?.read(cx);
        let active_buffer_entity = editor.buffer().read(cx).as_singleton()?;
        let active_buffer = active_buffer_entity.read(cx);

        let project_path = active_buffer.project_path(cx)?;

        /* todo!
        if self
            .context_store
            .read(cx)
            .will_include_buffer(active_buffer.remote_id(), &project_path)
            .is_some()
        {
            return None;
        }
        */

        let file_name = active_buffer.file()?.file_name(cx);

        let icon_path = FileIcons::get_icon(&Path::new(&file_name), cx);

        Some(SuggestedContext::File {
            name: file_name.to_string_lossy().into_owned().into(),
            buffer: active_buffer_entity.downgrade(),
            icon_path,
        })
    }

    /*
    fn suggested_thread(&self, cx: &Context<Self>) -> Option<SuggestedContext> {
        if !self.context_picker.read(cx).allow_threads() {
            return None;
        }

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
    */

    fn handle_context_picker_event(
        &mut self,
        _picker: &Entity<ContextPicker>,
        _event: &DismissEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.emit(ContextStripEvent::PickerDismissed);
    }

    fn handle_focus(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.focused_index = self.last_pill_index();
        cx.notify();
    }

    fn handle_blur(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.focused_index = None;
        cx.notify();
    }

    fn focus_left(&mut self, _: &FocusLeft, _window: &mut Window, cx: &mut Context<Self>) {
        self.focused_index = match self.focused_index {
            Some(index) if index > 0 => Some(index - 1),
            _ => self.last_pill_index(),
        };

        cx.notify();
    }

    fn focus_right(&mut self, _: &FocusRight, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(last_index) = self.last_pill_index() else {
            return;
        };

        self.focused_index = match self.focused_index {
            Some(index) if index < last_index => Some(index + 1),
            _ => Some(0),
        };

        cx.notify();
    }

    fn focus_up(&mut self, _: &FocusUp, _window: &mut Window, cx: &mut Context<Self>) {
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

    fn focus_down(&mut self, _: &FocusDown, _window: &mut Window, cx: &mut Context<Self>) {
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

        if pills.is_empty() { None } else { Some(pills) }
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

    fn open_context(&mut self, context: &AssistantContext, window: &mut Window, cx: &mut App) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        crate::active_thread::open_context(
            context,
            self.context_store.clone(),
            workspace,
            window,
            cx,
        );
    }

    fn remove_focused_context(
        &mut self,
        _: &RemoveFocusedContext,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(index) = self.focused_index {
            let mut is_empty = false;

            self.context_store.update(cx, |this, cx| {
                if let Some(item) = this.context().get_index(index) {
                    this.remove_context(&item.clone(), cx);
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

    fn is_suggested_focused<T>(&self, context: &IndexSet<T>) -> bool {
        // We only suggest one item after the actual context
        self.focused_index == Some(context.len())
    }

    fn accept_suggested_context(
        &mut self,
        _: &AcceptSuggestedContext,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(suggested) = self.suggested_context(cx) {
            let context_store = self.context_store.read(cx);

            if self.is_suggested_focused(context_store.context()) {
                self.add_suggested_context(&suggested, window, cx);
            }
        }
    }

    fn add_suggested_context(
        &mut self,
        suggested: &SuggestedContext,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // todo! conditional notify
        self.context_store.update(cx, |context_store, cx| {
            context_store.accept_suggested_context(&suggested, cx)
        });
        cx.notify();
    }
}

impl Focusable for ContextStrip {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ContextStrip {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let context_store = self.context_store.read(cx);
        let context = context_store.context();
        let context_picker = self.context_picker.clone();
        let focus_handle = self.focus_handle.clone();

        let suggested_context = self.suggested_context(cx);

        let added_contexts = context
            .iter()
            .flat_map(|c| AddedContext::new(c.clone(), cx))
            .collect::<Vec<_>>();
        let dupe_names = added_contexts
            .iter()
            .map(|c| c.name.clone())
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
                let entity = cx.entity().downgrade();
                move |children_bounds, _window, cx| {
                    entity
                        .update(cx, |this, _| {
                            this.children_bounds = Some(children_bounds);
                        })
                        .ok();
                }
            })
            .child(
                PopoverMenu::new("context-picker")
                    .menu(move |window, cx| {
                        context_picker.update(cx, |this, cx| {
                            this.init(window, cx);
                        });

                        Some(context_picker.clone())
                    })
                    .trigger_with_tooltip(
                        IconButton::new("add-context", IconName::Plus)
                            .icon_size(IconSize::Small)
                            .style(ui::ButtonStyle::Filled),
                        {
                            let focus_handle = focus_handle.clone();
                            move |window, cx| {
                                Tooltip::for_action_in(
                                    "Add Context",
                                    &ToggleContextPicker,
                                    &focus_handle,
                                    window,
                                    cx,
                                )
                            }
                        },
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
                                KeyBinding::for_action_in(
                                    &ToggleContextPicker,
                                    &focus_handle,
                                    window,
                                    cx,
                                )
                                .map(|binding| binding.into_any_element()),
                            ),
                    )
                }
            })
            .children(
                added_contexts
                    .into_iter()
                    .enumerate()
                    .map(|(i, added_context)| {
                        let name = added_context.name.clone();
                        let context = added_context.context.clone();
                        ContextPill::added(
                            added_context,
                            dupe_names.contains(&name),
                            self.focused_index == Some(i),
                            Some({
                                let context = context.clone();
                                let context_store = self.context_store.clone();
                                Rc::new(cx.listener(move |_this, _event, _window, cx| {
                                    context_store.update(cx, |this, cx| {
                                        this.remove_context(&context, cx);
                                    });
                                    cx.notify();
                                }))
                            }),
                        )
                        .on_click({
                            Rc::new(cx.listener(move |this, event: &ClickEvent, window, cx| {
                                if event.down.click_count > 1 {
                                    this.open_context(&context, window, cx);
                                } else {
                                    this.focused_index = Some(i);
                                }
                                cx.notify();
                            }))
                        })
                    }),
            )
            .when_some(suggested_context, |el, suggested| {
                el.child(
                    ContextPill::suggested(
                        suggested.name().clone(),
                        suggested.icon_path(),
                        suggested.kind(),
                        self.is_suggested_focused(&context),
                    )
                    .on_click(Rc::new(cx.listener(
                        move |this, _event, window, cx| {
                            this.add_suggested_context(&suggested, window, cx);
                        },
                    ))),
                )
            })
            .when(!context.is_empty(), {
                move |parent| {
                    parent.child(
                        IconButton::new("remove-all-context", IconName::Eraser)
                            .icon_size(IconSize::Small)
                            .tooltip({
                                let focus_handle = focus_handle.clone();
                                move |window, cx| {
                                    Tooltip::for_action_in(
                                        "Remove All Context",
                                        &RemoveAllContext,
                                        &focus_handle,
                                        window,
                                        cx,
                                    )
                                }
                            })
                            .on_click(cx.listener({
                                let focus_handle = focus_handle.clone();
                                move |_this, _event, window, cx| {
                                    focus_handle.dispatch_action(&RemoveAllContext, window, cx);
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
    // Thread,
}

#[derive(Clone)]
pub enum SuggestedContext {
    File {
        name: SharedString,
        icon_path: Option<SharedString>,
        buffer: WeakEntity<Buffer>,
    },
    // Thread {
    //     name: SharedString,
    //     thread: WeakEntity<Thread>,
    // },
}

impl SuggestedContext {
    pub fn name(&self) -> &SharedString {
        match self {
            Self::File { name, .. } => name,
            // Self::Thread { name, .. } => name,
        }
    }

    pub fn icon_path(&self) -> Option<SharedString> {
        match self {
            Self::File { icon_path, .. } => icon_path.clone(),
            // Self::Thread { .. } => None,
        }
    }

    pub fn kind(&self) -> ContextKind {
        match self {
            Self::File { .. } => ContextKind::File,
            // Self::Thread { .. } => ContextKind::Thread,
        }
    }
}
