use std::path::Path;
use std::rc::Rc;

use assistant_context_editor::AssistantContext;
use collections::HashSet;
use editor::Editor;
use file_icons::FileIcons;
use gpui::{
    App, Bounds, ClickEvent, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    Subscription, WeakEntity,
};
use itertools::Itertools;
use language::Buffer;
use project::ProjectItem;
use ui::{PopoverMenu, PopoverMenuHandle, Tooltip, prelude::*};
use workspace::Workspace;

use crate::context::{AgentContextHandle, ContextKind};
use crate::context_picker::ContextPicker;
use crate::context_store::ContextStore;
use crate::thread::Thread;
use crate::thread_store::{TextThreadStore, ThreadStore};
use crate::ui::{AddedContext, ContextPill};
use crate::{
    AcceptSuggestedContext, AgentPanel, FocusDown, FocusLeft, FocusRight, FocusUp,
    RemoveAllContext, RemoveFocusedContext, ToggleContextPicker,
};

pub struct ContextStrip {
    context_store: Entity<ContextStore>,
    context_picker: Entity<ContextPicker>,
    context_picker_menu_handle: PopoverMenuHandle<ContextPicker>,
    focus_handle: FocusHandle,
    suggest_context_kind: SuggestContextKind,
    workspace: WeakEntity<Workspace>,
    thread_store: Option<WeakEntity<ThreadStore>>,
    _subscriptions: Vec<Subscription>,
    focused_index: Option<usize>,
    children_bounds: Option<Vec<Bounds<Pixels>>>,
}

impl ContextStrip {
    pub fn new(
        context_store: Entity<ContextStore>,
        workspace: WeakEntity<Workspace>,
        thread_store: Option<WeakEntity<ThreadStore>>,
        text_thread_store: Option<WeakEntity<TextThreadStore>>,
        context_picker_menu_handle: PopoverMenuHandle<ContextPicker>,
        suggest_context_kind: SuggestContextKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let context_picker = cx.new(|cx| {
            ContextPicker::new(
                workspace.clone(),
                thread_store.clone(),
                text_thread_store,
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
            thread_store,
            _subscriptions: subscriptions,
            focused_index: None,
            children_bounds: None,
        }
    }

    fn added_contexts(&self, cx: &App) -> Vec<AddedContext> {
        if let Some(workspace) = self.workspace.upgrade() {
            let project = workspace.read(cx).project().read(cx);
            let prompt_store = self
                .thread_store
                .as_ref()
                .and_then(|thread_store| thread_store.upgrade())
                .and_then(|thread_store| thread_store.read(cx).prompt_store().as_ref());
            self.context_store
                .read(cx)
                .context()
                .flat_map(|context| {
                    AddedContext::new_pending(context.clone(), prompt_store, project, cx)
                })
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        }
    }

    fn suggested_context(&self, cx: &Context<Self>) -> Option<SuggestedContext> {
        match self.suggest_context_kind {
            SuggestContextKind::File => self.suggested_file(cx),
            SuggestContextKind::Thread => self.suggested_thread(cx),
        }
    }

    fn suggested_file(&self, cx: &Context<Self>) -> Option<SuggestedContext> {
        let workspace = self.workspace.upgrade()?;
        let active_item = workspace.read(cx).active_item(cx)?;

        let editor = active_item.to_any().downcast::<Editor>().ok()?.read(cx);
        let active_buffer_entity = editor.buffer().read(cx).as_singleton()?;
        let active_buffer = active_buffer_entity.read(cx);
        let project_path = active_buffer.project_path(cx)?;

        if self
            .context_store
            .read(cx)
            .file_path_included(&project_path, cx)
            .is_some()
        {
            return None;
        }

        let file_name = active_buffer.file()?.file_name(cx);
        let icon_path = FileIcons::get_icon(&Path::new(&file_name), cx);
        Some(SuggestedContext::File {
            name: file_name.to_string_lossy().into_owned().into(),
            buffer: active_buffer_entity.downgrade(),
            icon_path,
        })
    }

    fn suggested_thread(&self, cx: &Context<Self>) -> Option<SuggestedContext> {
        if !self.context_picker.read(cx).allow_threads() {
            return None;
        }

        let workspace = self.workspace.upgrade()?;
        let panel = workspace.read(cx).panel::<AgentPanel>(cx)?.read(cx);

        if let Some(active_thread) = panel.active_thread() {
            let weak_active_thread = active_thread.downgrade();

            let active_thread = active_thread.read(cx);

            if self
                .context_store
                .read(cx)
                .includes_thread(active_thread.id())
            {
                return None;
            }

            Some(SuggestedContext::Thread {
                name: active_thread.summary_or_default(),
                thread: weak_active_thread,
            })
        } else if let Some(active_context_editor) = panel.active_context_editor() {
            let context = active_context_editor.read(cx).context();
            let weak_context = context.downgrade();
            let context = context.read(cx);
            let path = context.path()?;

            if self.context_store.read(cx).includes_text_thread(path) {
                return None;
            }

            Some(SuggestedContext::TextThread {
                name: context.summary_or_default(),
                context: weak_context,
            })
        } else {
            None
        }
    }

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

    fn open_context(&mut self, context: &AgentContextHandle, window: &mut Window, cx: &mut App) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        crate::active_thread::open_context(context, workspace, window, cx);
    }

    fn remove_focused_context(
        &mut self,
        _: &RemoveFocusedContext,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(index) = self.focused_index {
            let added_contexts = self.added_contexts(cx);
            let Some(context) = added_contexts.get(index) else {
                return;
            };

            self.context_store.update(cx, |this, cx| {
                this.remove_context(&context.handle, cx);
            });

            let is_now_empty = added_contexts.len() == 1;
            if is_now_empty {
                cx.emit(ContextStripEvent::BlurredEmpty);
            } else {
                self.focused_index = Some(index.saturating_sub(1));
                cx.notify();
            }
        }
    }

    fn is_suggested_focused(&self, added_contexts: &Vec<AddedContext>) -> bool {
        // We only suggest one item after the actual context
        self.focused_index == Some(added_contexts.len())
    }

    fn accept_suggested_context(
        &mut self,
        _: &AcceptSuggestedContext,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(suggested) = self.suggested_context(cx) {
            if self.is_suggested_focused(&self.added_contexts(cx)) {
                self.add_suggested_context(&suggested, cx);
            }
        }
    }

    fn add_suggested_context(&mut self, suggested: &SuggestedContext, cx: &mut Context<Self>) {
        self.context_store.update(cx, |context_store, cx| {
            context_store.add_suggested_context(&suggested, cx)
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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let context_picker = self.context_picker.clone();
        let focus_handle = self.focus_handle.clone();

        let added_contexts = self.added_contexts(cx);
        let dupe_names = added_contexts
            .iter()
            .map(|c| c.name.clone())
            .sorted()
            .tuple_windows()
            .filter(|(a, b)| a == b)
            .map(|(a, _)| a)
            .collect::<HashSet<SharedString>>();
        let no_added_context = added_contexts.is_empty();

        let suggested_context = self.suggested_context(cx).map(|suggested_context| {
            (
                suggested_context,
                self.is_suggested_focused(&added_contexts),
            )
        });

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
                    .menu({
                        let context_picker = context_picker.clone();
                        move |window, cx| {
                            context_picker.update(cx, |this, cx| {
                                this.init(window, cx);
                            });

                            Some(context_picker.clone())
                        }
                    })
                    .on_open({
                        let context_picker = context_picker.downgrade();
                        Rc::new(move |window, cx| {
                            context_picker
                                .update(cx, |context_picker, cx| {
                                    context_picker.select_first(window, cx);
                                })
                                .ok();
                        })
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
            .children(
                added_contexts
                    .into_iter()
                    .enumerate()
                    .map(|(i, added_context)| {
                        let name = added_context.name.clone();
                        let context = added_context.handle.clone();
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
            .when_some(suggested_context, |el, (suggested, focused)| {
                el.child(
                    ContextPill::suggested(
                        suggested.name().clone(),
                        suggested.icon_path(),
                        suggested.kind(),
                        focused,
                    )
                    .on_click(Rc::new(cx.listener(
                        move |this, _event, _window, cx| {
                            this.add_suggested_context(&suggested, cx);
                        },
                    ))),
                )
            })
            .when(!no_added_context, {
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
            .into_any()
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
        buffer: WeakEntity<Buffer>,
    },
    Thread {
        name: SharedString,
        thread: WeakEntity<Thread>,
    },
    TextThread {
        name: SharedString,
        context: WeakEntity<AssistantContext>,
    },
}

impl SuggestedContext {
    pub fn name(&self) -> &SharedString {
        match self {
            Self::File { name, .. } => name,
            Self::Thread { name, .. } => name,
            Self::TextThread { name, .. } => name,
        }
    }

    pub fn icon_path(&self) -> Option<SharedString> {
        match self {
            Self::File { icon_path, .. } => icon_path.clone(),
            Self::Thread { .. } => None,
            Self::TextThread { .. } => None,
        }
    }

    pub fn kind(&self) -> ContextKind {
        match self {
            Self::File { .. } => ContextKind::File,
            Self::Thread { .. } => ContextKind::Thread,
            Self::TextThread { .. } => ContextKind::TextThread,
        }
    }
}
