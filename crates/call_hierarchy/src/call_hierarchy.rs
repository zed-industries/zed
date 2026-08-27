use std::{
    ops::Range,
    path::PathBuf,
    sync::{Arc, atomic::AtomicBool},
};

use editor::{
    Editor, RowHighlightOptions, SelectionEffects,
    scroll::{Autoscroll, ScrollOffset},
};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    AbsoluteLength, Action, AnyView, App, AsyncWindowContext, Context, DismissEvent, Entity,
    EventEmitter, FocusHandle, Focusable, HighlightStyle, ParentElement, Point, Render, Styled,
    StyledText, Subscription, Task, TextRun, TextStyle, WeakEntity, Window,
};
use language::{
    Buffer, CodeLabel, File as _, Language, Location, Rope, ToOffset, ToPoint, lsp_to_symbol_kind,
};
use picker::{Picker, PickerDelegate};
use project::{CallHierarchyItem, LspStoreEvent, Project};
use settings::Settings;
use theme::SyntaxTheme;
use theme_settings::ThemeSettings;
use ui::{KeyBinding, ListItem, ListItemSpacing, prelude::*, tooltip_container};
use util::{ResultExt, paths::PathExt};
use workspace::{DismissDecision, ModalView, Workspace};
pub use zed_actions::{ShowIncomingCalls, ShowOutgoingCalls, ToggleDirection};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CallHierarchyMode {
    #[default]
    Incoming,
    Outgoing,
}

impl CallHierarchyMode {
    fn opposite(self) -> Self {
        match self {
            CallHierarchyMode::Incoming => CallHierarchyMode::Outgoing,
            CallHierarchyMode::Outgoing => CallHierarchyMode::Incoming,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Call {
    pub item: CallHierarchyItem,
    pub target: Location,
    pub site_count: usize,
    pub label: Option<Arc<CodeLabel>>,
    pub display: CallDisplay,
}

#[derive(Debug, Clone, Default)]
pub struct CallDisplay {
    pub name: SharedString,
    pub detail: Option<SharedString>,
    pub path: Option<SharedString>,
    pub full_signature: SharedString,
    pub label_text: Option<SharedString>,
    pub needs_tooltip: bool,
}

const BASE_MODAL_WIDTH: Rems = Rems(42.0);

#[derive(Debug, Clone, Copy, PartialEq, settings::RegisterSetting)]
pub struct CallHierarchySettings {
    pub modal_max_width: settings::ModalWidthContent,
}

impl Settings for CallHierarchySettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let call_hierarchy = content.call_hierarchy.as_ref().unwrap();
        Self {
            modal_max_width: call_hierarchy.modal_max_width.unwrap(),
        }
    }
}

pub async fn fetch_calls(
    item: &CallHierarchyItem,
    project: &Entity<Project>,
    mode: CallHierarchyMode,
    cx: &mut AsyncWindowContext,
) -> Vec<Call> {
    let raw_calls = match mode {
        CallHierarchyMode::Incoming => {
            let task = project.update(cx, |project, cx| project.incoming_calls(item.clone(), cx));
            task.await
                .log_err()
                .flatten()
                .unwrap_or_default()
                .into_iter()
                .map(|call| (call.from, call.from_ranges))
                .collect::<Vec<_>>()
        }
        CallHierarchyMode::Outgoing => {
            let task = project.update(cx, |project, cx| project.outgoing_calls(item.clone(), cx));
            task.await
                .log_err()
                .flatten()
                .unwrap_or_default()
                .into_iter()
                .map(|call| (call.to, call.from_ranges))
                .collect::<Vec<_>>()
        }
    };

    let mut calls = cx
        .update(|_, cx| {
            raw_calls
                .into_iter()
                .map(|(item, mut sites)| {
                    sites.sort_by_key(|site| {
                        let buffer = site.buffer.read(cx);
                        site.range.start.to_offset(buffer)
                    });
                    let site_count = sites.len().max(1);
                    let target = sites
                        .into_iter()
                        .next()
                        .unwrap_or_else(|| item_location(&item));
                    Call {
                        item,
                        target,
                        site_count,
                        label: None,
                        display: CallDisplay::default(),
                    }
                })
                .collect::<Vec<_>>()
        })
        .ok()
        .unwrap_or_default();

    attach_labels(&mut calls, project, cx).await;
    cx.update(|_, cx| {
        for call in calls.iter_mut() {
            call.display = compute_call_display(call, cx);
        }
        calls.sort_by_cached_key(|call| {
            let offset = call
                .target
                .range
                .start
                .to_offset(call.target.buffer.read(cx));
            (call.display.path.clone(), offset)
        });
    })
    .ok();
    calls
}

pub fn init(cx: &mut App) {
    cx.observe_new(CallHierarchyView::register).detach();
}

pub struct CallHierarchyView {
    picker: Entity<Picker<CallHierarchyDelegate>>,
    mode: CallHierarchyMode,
    _subscriptions: [Subscription; 2],
}

impl CallHierarchyView {
    fn register(editor: &mut Editor, _: Option<&mut Window>, cx: &mut Context<Editor>) {
        if editor.mode().is_full() {
            let handle = cx.entity().downgrade();
            editor
                .register_action({
                    let handle = handle.clone();
                    move |_: &ShowIncomingCalls, window, cx| {
                        if let Some(editor) = handle.upgrade() {
                            toggle_call_hierarchy(editor, CallHierarchyMode::Incoming, window, cx);
                        }
                    }
                })
                .detach();
            editor
                .register_action(move |_: &ShowOutgoingCalls, window, cx| {
                    if let Some(editor) = handle.upgrade() {
                        toggle_call_hierarchy(editor, CallHierarchyMode::Outgoing, window, cx);
                    }
                })
                .detach();
        }
    }

    fn new(
        editor: Entity<Editor>,
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        mode: CallHierarchyMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> CallHierarchyView {
        let prev_scroll_position = editor.update(cx, |editor, cx| Some(editor.scroll_position(cx)));
        let project_subscription =
            cx.subscribe_in(&project, window, |view, _, event, window, cx| match event {
                project::Event::LanguageServerBufferRegistered { buffer_id, .. } => {
                    view.retry_empty_hierarchy(
                        |delegate, cx| {
                            delegate
                                .queried_buffer
                                .as_ref()
                                .is_some_and(|buffer| buffer.read(cx).remote_id() == *buffer_id)
                        },
                        window,
                        cx,
                    );
                }
                _ => {}
            });
        let lsp_store_subscription = cx.subscribe_in(
            &project.read(cx).lsp_store(),
            window,
            |view, _, event, window, cx| match event {
                LspStoreEvent::LanguageServerUpdate {
                    message: proto::update_language_server::Variant::WorkEnd(_),
                    ..
                } => {
                    view.retry_empty_hierarchy(
                        |delegate, _| delegate.queried_buffer.is_some(),
                        window,
                        cx,
                    );
                }
                _ => {}
            },
        );
        let delegate = CallHierarchyDelegate::new(
            cx.entity().downgrade(),
            editor,
            project,
            workspace,
            mode,
            prev_scroll_position,
            cx.focus_handle(),
        );
        let modal_width = CallHierarchySettings::get_global(cx)
            .modal_max_width
            .to_pixels(
                BASE_MODAL_WIDTH.to_pixels(window.rem_size()),
                window.viewport_size().width,
            );
        let picker = cx.new(|cx| {
            Picker::uniform_list(delegate, window, cx)
                .initial_width(Rems::from_pixels(modal_width, window))
                .show_scrollbar(true)
                .max_height(Rems::from_pixels(
                    window.viewport_size().height * 0.75,
                    window,
                ))
        });
        let picker_focus_handle = picker.focus_handle(cx);
        picker.update(cx, |picker, cx| {
            picker.delegate.modal_width = modal_width;
            picker.delegate.focus_handle = picker_focus_handle;
            picker.delegate.fetch_root(window, cx);
        });

        CallHierarchyView {
            picker,
            mode,
            _subscriptions: [project_subscription, lsp_store_subscription],
        }
    }

    fn retry_empty_hierarchy(
        &mut self,
        should_retry: impl FnOnce(&CallHierarchyDelegate, &App) -> bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            if picker.delegate.root_item.is_none() && should_retry(&picker.delegate, cx) {
                picker.delegate.fetch_root(window, cx);
            }
        });
    }

    fn set_mode(&mut self, mode: CallHierarchyMode, window: &mut Window, cx: &mut Context<Self>) {
        if self.mode == mode {
            cx.emit(DismissEvent);
            return;
        }
        self.mode = mode;
        self.picker.update(cx, |picker, cx| {
            picker.delegate.mode = mode;
            if let Some(root_item) = picker.delegate.root_item.clone() {
                picker.delegate.load_calls(root_item, None, window, cx);
            }
            picker.set_query("", window, cx);
            picker.refresh_placeholder(window, cx);
        });
        cx.notify();
    }
}

impl Focusable for CallHierarchyView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for CallHierarchyView {}

impl ModalView for CallHierarchyView {
    fn on_before_dismiss(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> DismissDecision {
        self.picker.update(cx, |picker, cx| {
            picker.delegate.restore_editor(window, cx);
        });
        DismissDecision::Dismiss(true)
    }
}

impl Render for CallHierarchyView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("CallHierarchyPicker")
            .on_action(cx.listener(|view, _: &ShowIncomingCalls, window, cx| {
                view.set_mode(CallHierarchyMode::Incoming, window, cx);
            }))
            .on_action(cx.listener(|view, _: &ShowOutgoingCalls, window, cx| {
                view.set_mode(CallHierarchyMode::Outgoing, window, cx);
            }))
            .on_action(cx.listener(|view, _: &ToggleDirection, window, cx| {
                view.set_mode(view.mode.opposite(), window, cx);
            }))
            .child(self.picker.clone())
    }
}

pub struct CallHierarchyDelegate {
    view: WeakEntity<CallHierarchyView>,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    editor: Entity<Editor>,
    mode: CallHierarchyMode,
    state: FetchState,
    fetch_task: Option<Task<()>>,
    root_item: Option<CallHierarchyItem>,
    root_stack: Vec<CallHierarchyItem>,
    prev_scroll_position: Option<Point<ScrollOffset>>,
    focus_handle: FocusHandle,
    queried_buffer: Option<Entity<Buffer>>,
    modal_width: Pixels,
    calls: Vec<Call>,
    candidates: Arc<Vec<StringMatchCandidate>>,
    matches: Vec<StringMatch>,
    selected_index: usize,
}

impl CallHierarchyDelegate {
    fn new(
        view: WeakEntity<CallHierarchyView>,
        editor: Entity<Editor>,
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        mode: CallHierarchyMode,
        prev_scroll_position: Option<Point<ScrollOffset>>,
        focus_handle: FocusHandle,
    ) -> Self {
        Self {
            view,
            workspace,
            project,
            editor,
            mode,
            state: FetchState::Loading,
            fetch_task: None,
            root_item: None,
            root_stack: Vec::new(),
            prev_scroll_position,
            focus_handle,
            queried_buffer: None,
            modal_width: Pixels::ZERO,
            calls: Vec::new(),
            candidates: Arc::new(Vec::new()),
            matches: Vec::new(),
            selected_index: 0,
        }
    }

    fn fetch_root(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let editor_anchor = self.editor.update(cx, |editor, cx| {
            let selection = editor.selections.newest_anchor().head();
            editor
                .buffer()
                .read(cx)
                .text_anchor_for_position(selection, cx)
        });
        let Some((buffer, position)) = editor_anchor else {
            self.state = FetchState::NoSymbol;
            cx.notify();
            return;
        };
        self.queried_buffer = Some(buffer.clone());
        self.state = FetchState::Loading;
        cx.notify();

        let prepare_task = self.project.update(cx, |project, cx| {
            project.prepare_call_hierarchy(&buffer, position, cx)
        });

        self.fetch_task = Some(cx.spawn_in(window, async move |picker, mut cx| {
            let root_item = match prepare_task.await {
                Ok(items) => items.unwrap_or_default().into_iter().next(),
                Err(error) => {
                    log::error!("failed to prepare call hierarchy: {error:#}");
                    picker
                        .update_in(cx, |picker, window, cx| {
                            picker
                                .delegate
                                .workspace
                                .update(cx, |workspace, cx| workspace.show_error(error, cx))
                                .ok();
                            picker.delegate.dismissed(window, cx);
                        })
                        .ok();
                    return;
                }
            };
            let Some(root_item) = root_item else {
                picker
                    .update_in(cx, |picker, _, cx| {
                        picker.delegate.state = FetchState::NoSymbol;
                        cx.notify();
                    })
                    .ok();
                return;
            };
            picker
                .update_in(cx, |picker, window, cx| {
                    picker.delegate.root_item = Some(root_item.clone());
                    picker.refresh_placeholder(window, cx);
                })
                .ok();
            load_and_apply_calls(picker, root_item, None, &mut cx).await;
        }));
    }

    fn load_calls(
        &mut self,
        root_item: CallHierarchyItem,
        select_item: Option<CallHierarchyItem>,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        self.state = FetchState::Loading;
        self.calls.clear();
        self.matches.clear();
        self.selected_index = 0;
        cx.notify();

        self.fetch_task = Some(cx.spawn_in(window, async move |picker, mut cx| {
            load_and_apply_calls(picker, root_item, select_item, &mut cx).await;
        }));
    }

    fn selected_call(&self) -> Option<&Call> {
        let mat = self.matches.get(self.selected_index)?;
        self.calls.get(mat.candidate_id)
    }

    fn expand_selected(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) -> bool {
        let Some(new_root) = self.selected_call().map(|call| call.item.clone()) else {
            return false;
        };
        if let Some(previous_root) = self.root_item.replace(new_root.clone()) {
            self.root_stack.push(previous_root);
        }
        self.load_calls(new_root, None, window, cx);
        true
    }

    fn collapse_to_parent(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) -> bool {
        let Some(parent) = self.root_stack.pop() else {
            return false;
        };
        let child = self.root_item.replace(parent.clone());
        self.load_calls(parent, child, window, cx);
        true
    }

    fn preview_selected(&self, cx: &mut Context<Picker<Self>>) {
        let call = self.selected_call();
        self.editor.update(cx, |editor, cx| {
            editor.clear_row_highlights::<CallHierarchyPreview>();
            let Some(call) = call else {
                return;
            };
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let Some(buffer_snapshot) = snapshot.as_singleton() else {
                return;
            };
            if buffer_snapshot.remote_id() != call.target.buffer.read(cx).remote_id() {
                return;
            }
            let Some(start) = snapshot.anchor_in_buffer(call.target.range.start) else {
                return;
            };
            let Some(end) = snapshot.anchor_in_buffer(call.target.range.end) else {
                return;
            };
            editor.highlight_rows::<CallHierarchyPreview>(
                start..end,
                |cx| cx.theme().colors().editor_highlighted_line_background,
                RowHighlightOptions {
                    autoscroll: true,
                    ..RowHighlightOptions::default()
                },
                cx,
            );
            editor.request_autoscroll(Autoscroll::center(), cx);
        });
    }

    fn restore_editor(&self, window: &mut Window, cx: &mut App) {
        let scroll_position = self.prev_scroll_position;
        self.editor.update(cx, |editor, cx| {
            editor.clear_row_highlights::<CallHierarchyPreview>();
            if let Some(scroll_position) = scroll_position {
                editor.set_scroll_position(scroll_position, window, cx);
            }
        });
    }
}

impl PickerDelegate for CallHierarchyDelegate {
    type ListItem = ListItem;

    fn name() -> &'static str {
        "call hierarchy"
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        match (&self.root_item, self.mode) {
            (Some(root), CallHierarchyMode::Incoming) => {
                Arc::from(format!("Search calls to `{}`...", root.name))
            }
            (Some(root), CallHierarchyMode::Outgoing) => {
                Arc::from(format!("Search calls from `{}`...", root.name))
            }
            (None, CallHierarchyMode::Incoming) => Arc::from("Search incoming calls..."),
            (None, CallHierarchyMode::Outgoing) => Arc::from("Search outgoing calls..."),
        }
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some(SharedString::new_static(match self.state {
            FetchState::Loading => "Fetching call hierarchy…",
            FetchState::NoSymbol => "No callable symbol under the cursor",
            FetchState::Loaded => {
                if self.calls.is_empty() {
                    match self.mode {
                        CallHierarchyMode::Incoming => "No incoming calls found",
                        CallHierarchyMode::Outgoing => "No outgoing calls found",
                    }
                } else {
                    "No matches"
                }
            }
        }))
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
        self.preview_selected(cx);
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        if query.is_empty() {
            self.matches = identity_matches(self.calls.len());
            self.selected_index = 0;
            return Task::ready(());
        }

        let candidates = self.candidates.clone();
        let executor = cx.background_executor().clone();
        cx.spawn(async move |picker, cx| {
            let matches = fuzzy::match_strings(
                candidates.as_slice(),
                &query,
                true,
                true,
                100,
                &AtomicBool::new(false),
                executor,
            )
            .await;

            picker
                .update(cx, |picker, cx| {
                    picker.delegate.matches = matches;
                    picker.delegate.selected_index = 0;
                    cx.notify();
                })
                .ok();
        })
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(call) = self.selected_call() else {
            return;
        };

        let buffer = call.target.buffer.clone();
        let target = call.target.range.start;
        self.prev_scroll_position = None;

        self.workspace
            .update(cx, |workspace, cx| {
                let position = target.to_point(&buffer.read(cx).snapshot());
                let pane = if secondary {
                    workspace.adjacent_pane(window, cx)
                } else {
                    workspace.active_pane().clone()
                };

                let editor = workspace
                    .open_project_item::<Editor>(pane, buffer, true, true, true, true, window, cx);

                editor.update(cx, |editor, cx| {
                    editor.change_selections(
                        SelectionEffects::scroll(Autoscroll::center()),
                        window,
                        cx,
                        |s| s.select_ranges([position..position]),
                    );
                });
            })
            .ok();

        self.dismissed(window, cx);
    }

    fn dismissed(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.restore_editor(window, cx);
        self.view.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
    }

    fn select_child(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<String> {
        self.expand_selected(window, cx).then(String::new)
    }

    fn select_parent(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<String> {
        self.collapse_to_parent(window, cx).then(String::new)
    }

    fn render_footer(
        &self,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<gpui::AnyElement> {
        if self.calls.is_empty() && self.root_stack.is_empty() {
            return None;
        }
        let focus_handle = self.focus_handle.clone();
        let expand_label = match self.mode {
            CallHierarchyMode::Incoming => "Show Callers",
            CallHierarchyMode::Outgoing => "Show Callees",
        };
        Some(
            h_flex()
                .w_full()
                .p_1p5()
                .gap_0p5()
                .justify_end()
                .flex_wrap()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .when(!self.root_stack.is_empty(), |this| {
                    this.child(
                        Button::new("collapse-call", "Back")
                            .key_binding(
                                KeyBinding::for_action_in(&menu::SelectParent, &focus_handle, cx)
                                    .map(|key_binding| key_binding.size(rems_from_px(12_f32))),
                            )
                            .on_click(|_, window, cx| {
                                window.dispatch_action(menu::SelectParent.boxed_clone(), cx);
                            }),
                    )
                })
                .when(!self.calls.is_empty(), |this| {
                    this.child(
                        Button::new("expand-call", expand_label)
                            .key_binding(
                                KeyBinding::for_action_in(&menu::SelectChild, &focus_handle, cx)
                                    .map(|key_binding| key_binding.size(rems_from_px(12_f32))),
                            )
                            .on_click(|_, window, cx| {
                                window.dispatch_action(menu::SelectChild.boxed_clone(), cx);
                            }),
                    )
                })
                .child(
                    Button::new("toggle-direction", "Switch Direction")
                        .key_binding(
                            KeyBinding::for_action_in(&ToggleDirection, &focus_handle, cx)
                                .map(|key_binding| key_binding.size(rems_from_px(12_f32))),
                        )
                        .on_click(|_, window, cx| {
                            window.dispatch_action(ToggleDirection.boxed_clone(), cx);
                        }),
                )
                .into_any_element(),
        )
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = self.matches.get(ix)?;
        let call = self.calls.get(mat.candidate_id)?;

        let (name_styled, detail_styled) = render_item(call, mat.ranges(), cx);
        let tooltip_label = call.label.clone().zip(call.display.label_text.clone());
        let tooltip_max_width = self.modal_width;
        let full_signature = call.display.full_signature.clone();
        let full_path = call.display.path.clone();

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    v_flex()
                        .text_ui(cx)
                        .child(
                            h_flex()
                                .overflow_x_hidden()
                                .text_size(ThemeSettings::get_global(cx).buffer_font_size(cx))
                                .child(name_styled)
                                .children(detail_styled),
                        )
                        .children(call.display.path.clone().map(|path| {
                            Label::new(path)
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                                .truncate()
                        })),
                )
                .when(call.display.needs_tooltip, |this| {
                    this.tooltip(move |_window, cx| {
                        AnyView::from(cx.new(|_| CallSignatureTooltip {
                            label: tooltip_label.clone(),
                            full_signature: full_signature.clone(),
                            path: full_path.clone(),
                            max_width: tooltip_max_width,
                        }))
                    })
                }),
        )
    }
}

struct CallSignatureTooltip {
    label: Option<(Arc<CodeLabel>, SharedString)>,
    full_signature: SharedString,
    path: Option<SharedString>,
    max_width: Pixels,
}

impl Render for CallSignatureTooltip {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut signature_style = buffer_text_style(cx);
        signature_style.line_height =
            relative(ThemeSettings::get_global(cx).buffer_line_height.value());
        let signature = match &self.label {
            Some((label, label_text)) => StyledText::new(label_text.clone())
                .with_default_highlights(
                    &signature_style,
                    label_syntax_runs(label, cx.theme().syntax()),
                )
                .into_any_element(),
            None => div().child(self.full_signature.clone()).into_any_element(),
        };
        let buffer_font_size = ThemeSettings::get_global(cx).buffer_font_size(cx);
        let max_width = self.max_width;
        let path = self.path.clone();
        tooltip_container(cx, move |el, _| {
            el.max_w(max_width)
                .gap_0p5()
                .child(div().text_size(buffer_font_size).child(signature))
                .children(
                    path.map(|path| Label::new(path).size(LabelSize::Small).color(Color::Muted)),
                )
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FetchState {
    Loading,
    NoSymbol,
    Loaded,
}

struct CallHierarchyPreview;

fn toggle_call_hierarchy(
    editor: Entity<Editor>,
    mode: CallHierarchyMode,
    window: &mut Window,
    cx: &mut App,
) {
    let Some(workspace) = editor.read(cx).workspace() else {
        log::error!("call hierarchy: editor has no workspace");
        return;
    };

    let project = workspace.read(cx).project().clone();
    let workspace_weak = workspace.downgrade();

    workspace.update(cx, |workspace, cx| {
        workspace.toggle_modal(window, cx, |window, cx| {
            CallHierarchyView::new(editor, project, workspace_weak, mode, window, cx)
        });
    });
}

fn item_location(item: &CallHierarchyItem) -> Location {
    Location {
        buffer: item.buffer.clone(),
        range: item.selection_range.clone(),
    }
}

async fn attach_labels(calls: &mut [Call], project: &Entity<Project>, cx: &mut AsyncWindowContext) {
    let mut fallback_groups = Vec::<(Arc<Language>, Vec<usize>)>::new();
    for (index, call) in calls.iter_mut().enumerate() {
        let Some(language) = call
            .item
            .buffer
            .read_with(cx, |buffer, _| buffer.language().cloned())
        else {
            continue;
        };
        if let Some(label) = signature_label(&call.item, &language) {
            call.label = Some(Arc::new(label));
            continue;
        }
        match fallback_groups
            .iter_mut()
            .find(|(group_language, _)| group_language.name() == language.name())
        {
            Some((_, indices)) => indices.push(index),
            None => fallback_groups.push((language, vec![index])),
        }
    }
    if fallback_groups.is_empty() {
        return;
    }
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    for (language, indices) in fallback_groups {
        let Some(lsp_adapter) = language_registry
            .lsp_adapters(&language.name())
            .first()
            .cloned()
        else {
            continue;
        };
        let symbols = indices
            .iter()
            .map(|&index| language::Symbol {
                name: calls[index].item.name.clone(),
                kind: lsp_to_symbol_kind(calls[index].item.kind),
                container_name: None,
            })
            .collect::<Vec<_>>();
        let Some(labels) = lsp_adapter
            .labels_for_symbols(&symbols, &language)
            .await
            .log_err()
        else {
            continue;
        };
        for (index, label) in indices.into_iter().zip(labels) {
            calls[index].label = label.map(Arc::new);
        }
    }
}

async fn load_and_apply_calls(
    picker: WeakEntity<Picker<CallHierarchyDelegate>>,
    root_item: CallHierarchyItem,
    select_item: Option<CallHierarchyItem>,
    cx: &mut AsyncWindowContext,
) {
    let Ok((project, mode)) = picker.read_with(cx, |picker, _| {
        (picker.delegate.project.clone(), picker.delegate.mode)
    }) else {
        return;
    };
    let mut calls = fetch_calls(&root_item, &project, mode, cx).await;
    picker
        .update_in(cx, |picker, window, cx| {
            let row_insets = (DynamicSpacing::Base04.rems(cx).to_pixels(window.rem_size())
                + DynamicSpacing::Base06.rems(cx).to_pixels(window.rem_size()))
                * 2.;
            let available_width = picker.delegate.modal_width - row_insets;
            let settings = ThemeSettings::get_global(cx);
            let buffer_font_size = settings.buffer_font_size(cx);
            let path_font_size = TextSize::Small.rems(cx).to_pixels(window.rem_size());
            for call in calls.iter_mut() {
                let signature = call
                    .display
                    .label_text
                    .clone()
                    .unwrap_or_else(|| call.display.full_signature.clone());
                let signature_width = shaped_width(
                    signature,
                    settings.buffer_font.clone(),
                    buffer_font_size,
                    window,
                );
                let path_width = call
                    .display
                    .path
                    .clone()
                    .map(|path| {
                        shaped_width(path, settings.ui_font.clone(), path_font_size, window)
                    })
                    .unwrap_or_default();
                call.display.needs_tooltip = signature_width.max(path_width) > available_width;
            }
            picker.delegate.state = FetchState::Loaded;
            picker.delegate.calls = calls;
            picker.delegate.candidates = Arc::new(
                picker
                    .delegate
                    .calls
                    .iter()
                    .enumerate()
                    .map(|(id, call)| StringMatchCandidate::new(id, &call.item.name))
                    .collect(),
            );
            picker.delegate.matches = identity_matches(picker.delegate.calls.len());
            picker.delegate.selected_index = 0;
            let restored_index = select_item.and_then(|target| {
                picker.delegate.calls.iter().position(|call| {
                    call.item.buffer == target.buffer
                        && call.item.selection_range == target.selection_range
                        && call.item.name == target.name
                })
            });
            if let Some(restored_index) = restored_index {
                picker.set_selected_index(restored_index, None, true, window, cx);
            }
            picker.refresh_placeholder(window, cx);
            cx.notify();
        })
        .ok();
}

fn call_display_path(buffer: &Buffer, cx: &App) -> Option<PathBuf> {
    let file = buffer.file()?;
    Some(
        if let Some(project_file) = project::File::from_dyn(Some(file))
            && project_file.worktree.read(cx).is_visible()
        {
            project_file.full_path(cx)
        } else if let Some(local_file) = file.as_local() {
            local_file.abs_path(cx).compact()
        } else {
            file.full_path(cx)
        },
    )
}

fn signature_label(item: &CallHierarchyItem, language: &Arc<Language>) -> Option<CodeLabel> {
    let detail = item
        .detail
        .as_deref()
        .map(str::trim)
        .filter(|detail| !detail.is_empty())?;
    let signature = collapse_whitespace(detail);
    let name_start = find_symbol_name(&signature, &item.name)?;
    let filter_range = name_start..name_start + item.name.len();
    let source = Rope::from_iter([signature.as_str(), " {}"]);
    let runs = language.highlight_text(&source, 0..signature.len());
    Some(CodeLabel::new(signature, filter_range, runs))
}

fn collapse_whitespace(text: &str) -> String {
    fn is_word(ch: char) -> bool {
        ch.is_alphanumeric() || ch == '_'
    }

    let mut result = String::new();
    let mut previous_indent = 0;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let indent = line.chars().take_while(|ch| ch.is_whitespace()).count();
        let fragment = trimmed.split_whitespace().collect::<Vec<_>>().join(" ");
        if result.is_empty() {
            result = fragment;
        } else {
            let opens_block = indent > previous_indent
                && result.chars().last().is_some_and(|last| !is_word(last));
            let closes_block = indent < previous_indent
                && fragment.chars().next().is_some_and(|first| !is_word(first));
            if closes_block && result.ends_with(',') {
                result.pop();
            }
            if !opens_block && !closes_block {
                result.push(' ');
            }
            result.push_str(&fragment);
        }
        previous_indent = indent;
    }
    if result.ends_with(',') {
        result.pop();
    }
    result
}

fn find_symbol_name(text: &str, symbol_name: &str) -> Option<usize> {
    if symbol_name.is_empty() {
        return None;
    }
    text.match_indices(symbol_name)
        .map(|(start, _)| start)
        .find(|&start| {
            let boundary_before = text[..start]
                .chars()
                .next_back()
                .is_none_or(|c| !c.is_alphanumeric() && c != '_');
            let boundary_after = text[start + symbol_name.len()..]
                .chars()
                .next()
                .is_none_or(|c| !c.is_alphanumeric() && c != '_');
            boundary_before && boundary_after
        })
}

fn identity_matches(count: usize) -> Vec<StringMatch> {
    (0..count)
        .map(|index| StringMatch {
            candidate_id: index,
            score: 0.0,
            positions: Vec::new(),
            string: String::new(),
        })
        .collect()
}

/// Extracts display information from a `Call` for rendering in UI.
fn compute_call_display(call: &Call, cx: &App) -> CallDisplay {
    let buffer = call.target.buffer.read(cx);
    let line_number = call.target.range.start.to_point(&buffer.snapshot()).row + 1;
    let path = call_display_path(buffer, cx).map(|path| {
        let path = path.to_string_lossy();
        SharedString::from(if call.site_count > 1 {
            format!("{path}:{line_number} ({} calls)", call.site_count)
        } else {
            format!("{path}:{line_number}")
        })
    });

    let detail = extract_call_detail(call).map(SharedString::from);
    let full_signature = SharedString::from(match &detail {
        Some(detail) => format!("{}{detail}", call.item.name),
        None => call.item.name.clone(),
    });
    let label_text = call
        .label
        .as_ref()
        .map(|label| SharedString::from(label.text.clone()));

    CallDisplay {
        name: SharedString::from(call.item.name.clone()),
        detail,
        path,
        full_signature,
        label_text,
        needs_tooltip: false,
    }
}

fn extract_call_detail(call: &Call) -> Option<String> {
    let detail = if let Some(detail) = call.label.as_deref().and_then(detail_from_label_suffix) {
        Some(detail)
    } else {
        call.item
            .detail
            .as_deref()
            .map(str::trim)
            .filter(|detail| !detail.is_empty())
            .and_then(|detail| {
                trim_detail_after_symbol_name(detail, &call.item.name)
                    .or_else(|| Some(detail.to_owned()))
            })
    };
    detail.map(|detail| collapse_whitespace(&detail))
}

fn detail_from_label_suffix(label: &CodeLabel) -> Option<String> {
    label
        .text
        .get(label.filter_range.end..)
        .map(str::trim)
        .filter(|suffix| !suffix.is_empty())
        .map(ToOwned::to_owned)
}

fn trim_detail_after_symbol_name(detail: &str, symbol_name: &str) -> Option<String> {
    let name_start = find_symbol_name(detail, symbol_name)?;
    let suffix = detail.get(name_start + symbol_name.len()..)?.trim();
    (!suffix.is_empty()).then(|| suffix.to_owned())
}

fn shaped_width(
    text: SharedString,
    font: gpui::Font,
    font_size: Pixels,
    window: &Window,
) -> Pixels {
    let run = TextRun {
        len: text.len(),
        font,
        color: gpui::Hsla::default(),
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    window
        .text_system()
        .shape_line(text, font_size, &[run], None)
        .width
}

fn buffer_text_style(cx: &App) -> TextStyle {
    let settings = ThemeSettings::get_global(cx);
    TextStyle {
        color: cx.theme().colors().text,
        font_family: settings.buffer_font.family.clone(),
        font_features: settings.buffer_font.features.clone(),
        font_fallbacks: settings.buffer_font.fallbacks.clone(),
        font_size: AbsoluteLength::from(settings.buffer_font_size(cx)),
        font_weight: settings.buffer_font.weight,
        line_height: relative(1.),
        ..TextStyle::default()
    }
}

fn label_syntax_runs<'a>(
    label: &'a CodeLabel,
    syntax_theme: &'a SyntaxTheme,
) -> impl Iterator<Item = (Range<usize>, HighlightStyle)> + 'a {
    label.runs.iter().filter_map(|(range, highlight_id)| {
        Some((range.clone(), *syntax_theme.get(*highlight_id)?))
    })
}

fn render_item(
    call_item: &Call,
    match_ranges: impl IntoIterator<Item = Range<usize>>,
    cx: &App,
) -> (StyledText, Option<StyledText>) {
    let mut base_text_style = buffer_text_style(cx);
    base_text_style.text_overflow = Some(gpui::TextOverflow::Truncate(SharedString::from("…")));

    let highlight_style = HighlightStyle {
        background_color: Some(cx.theme().colors().text_accent.alpha(0.3)),
        ..HighlightStyle::default()
    };

    let label_carries_detail = call_item
        .label
        .as_ref()
        .is_some_and(|label| detail_from_label_suffix(label).is_some());
    let detail_styled = if label_carries_detail {
        None
    } else {
        call_item.display.detail.clone().map(|detail| {
            let mut detail_style = base_text_style.clone();
            detail_style.color = cx.theme().colors().text_muted;
            StyledText::new(detail).with_default_highlights(&detail_style, std::iter::empty())
        })
    };

    let name_styled = if let Some((label, label_text)) = call_item
        .label
        .as_ref()
        .zip(call_item.display.label_text.clone())
    {
        let syntax_runs = label_syntax_runs(label, cx.theme().syntax());
        let custom_highlights = match_ranges.into_iter().map(|range| {
            let start = label.filter_range.start + range.start;
            let end = label.filter_range.start + range.end;
            (start..end, highlight_style)
        });

        StyledText::new(label_text).with_default_highlights(
            &base_text_style,
            gpui::combine_highlights(custom_highlights, syntax_runs),
        )
    } else {
        StyledText::new(call_item.display.name.clone()).with_default_highlights(
            &base_text_style,
            match_ranges.into_iter().map(|r| (r, highlight_style)),
        )
    };

    (name_styled, detail_styled)
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt as _;
    use gpui::{Hsla, TestAppContext, VisualTestContext};
    use language::{FakeLspAdapter, LanguageConfig, LanguageMatcher, LanguageName};
    use project::{CallHierarchyItem, FakeFs};
    use serde_json::json;
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };
    use theme::SyntaxTheme;
    use util::{path, rel_path::rel_path};
    use workspace::{AppState, MultiWorkspace};

    #[gpui::test]
    async fn test_call_display_basic(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/test"), json!({"src": {"main.rs": source()}}))
            .await;
        let project = Project::test(fs, [path!("/test").as_ref()], cx).await;

        let call = make_call(
            "my_function",
            path!("/test/src/main.rs").as_ref(),
            10,
            None,
            &project,
            cx,
        )
        .await;
        let display = cx.update(|cx| compute_call_display(&call, cx));

        #[cfg(not(windows))]
        let expected_path = "test/src/main.rs:11";
        #[cfg(windows)]
        let expected_path = "test\\src\\main.rs:11";

        assert_eq!(display.name, "my_function");
        assert_eq!(display.detail, None);
        assert_eq!(display.path.as_deref(), Some(expected_path));
    }

    #[gpui::test]
    async fn test_call_display_external_path_compacted(cx: &mut TestAppContext) {
        init_test(cx);

        let home_dir = util::paths::home_dir();
        let external_path = home_dir
            .join("projects")
            .join("app")
            .join("src")
            .join("main.rs");

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/test"), json!({"main.rs": source()}))
            .await;
        fs.insert_tree(
            &home_dir,
            json!({"projects": {"app": {"src": {"main.rs": source()}}}}),
        )
        .await;
        let project = Project::test(fs, [path!("/test").as_ref()], cx).await;

        let call = make_call("my_function", &external_path, 10, None, &project, cx).await;
        let display = cx.update(|cx| compute_call_display(&call, cx));

        #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "macos"))]
        let expected_path = "~/projects/app/src/main.rs:11".to_string();
        #[cfg(windows)]
        let expected_path = format!("{}:11", external_path.to_string_lossy());

        assert_eq!(display.name, "my_function");
        assert_eq!(display.detail, None);
        assert_eq!(display.path.as_deref(), Some(expected_path.as_str()));
    }

    #[gpui::test]
    async fn test_call_display_with_detail(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/test"), json!({"src": {"lib.rs": source()}}))
            .await;
        let project = Project::test(fs, [path!("/test").as_ref()], cx).await;

        let call = make_call(
            "helper",
            path!("/test/src/lib.rs").as_ref(),
            5,
            Some("fn helper() -> i32".to_string()),
            &project,
            cx,
        )
        .await;
        let display = cx.update(|cx| compute_call_display(&call, cx));

        #[cfg(not(windows))]
        let expected_path = "test/src/lib.rs:6";
        #[cfg(windows)]
        let expected_path = "test\\src\\lib.rs:6";

        assert_eq!(display.name, "helper");
        assert_eq!(display.detail.as_deref(), Some("() -> i32"));
        assert_eq!(display.path.as_deref(), Some(expected_path));
    }

    #[gpui::test]
    async fn test_call_display_prefers_label_suffix(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/test"), json!({"src": {"lib.rs": source()}}))
            .await;
        let project = Project::test(fs, [path!("/test").as_ref()], cx).await;

        let mut call = make_call(
            "helper",
            path!("/test/src/lib.rs").as_ref(),
            5,
            Some("fn helper(&self) -> i32".to_string()),
            &project,
            cx,
        )
        .await;
        call.label = Some(Arc::new(CodeLabel::new(
            "fn helper(&self) -> i32".to_string(),
            3..9,
            Vec::new(),
        )));

        let display = cx.update(|cx| compute_call_display(&call, cx));
        assert_eq!(display.detail.as_deref(), Some("(&self) -> i32"));
    }

    #[gpui::test]
    async fn test_call_display_trims_raw_detail_after_symbol_name(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/test"), json!({"src": {"lib.rs": source()}}))
            .await;
        let project = Project::test(fs, [path!("/test").as_ref()], cx).await;

        let call = make_call(
            "helper",
            path!("/test/src/lib.rs").as_ref(),
            5,
            Some("fn helper(&self, arg: i32) -> i32".to_string()),
            &project,
            cx,
        )
        .await;

        let display = cx.update(|cx| compute_call_display(&call, cx));
        assert_eq!(display.detail.as_deref(), Some("(&self, arg: i32) -> i32"));
    }

    #[gpui::test]
    async fn test_call_display_multiline_detail(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/test"), json!({"main.rs": source()}))
            .await;
        let project = Project::test(fs, [path!("/test").as_ref()], cx).await;

        let call = make_call(
            "foo",
            path!("/test/main.rs").as_ref(),
            0,
            Some("line1\nline2\nline3".to_string()),
            &project,
            cx,
        )
        .await;
        let display = cx.update(|cx| compute_call_display(&call, cx));

        assert_eq!(display.name, "foo");
        assert_eq!(display.detail.as_deref(), Some("line1 line2 line3"));
    }

    #[gpui::test]
    async fn test_call_display_multiline_where_clause(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/test"), json!({"main.rs": source()}))
            .await;
        let project = Project::test(fs, [path!("/test").as_ref()], cx).await;

        let call = make_call(
            "contains",
            path!("/test/main.rs").as_ref(),
            0,
            Some(
                "fn contains<Q>(&self, value: &Q) -> bool\nwhere\n    Q: ?Sized,\n    T: Borrow<Q>,"
                    .to_string(),
            ),
            &project,
            cx,
        )
        .await;
        let display = cx.update(|cx| compute_call_display(&call, cx));

        assert_eq!(
            display.detail.as_deref(),
            Some("<Q>(&self, value: &Q) -> bool where Q: ?Sized, T: Borrow<Q>")
        );
    }

    #[gpui::test]
    async fn test_call_display_multiline_params(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/test"), json!({"main.rs": source()}))
            .await;
        let project = Project::test(fs, [path!("/test").as_ref()], cx).await;

        let call = make_call(
            "hints",
            path!("/test/main.rs").as_ref(),
            0,
            Some(
                "fn hints(\n    hints: &mut Vec<InlayHint>,\n    ctx: &mut InlayHintCtx,\n)"
                    .to_string(),
            ),
            &project,
            cx,
        )
        .await;
        let display = cx.update(|cx| compute_call_display(&call, cx));

        assert_eq!(
            display.detail.as_deref(),
            Some("(hints: &mut Vec<InlayHint>, ctx: &mut InlayHintCtx)")
        );
    }

    #[gpui::test]
    async fn test_call_display_single_letter_name(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/test"), json!({"main.rs": source()}))
            .await;
        let project = Project::test(fs, [path!("/test").as_ref()], cx).await;

        let call = make_call(
            "f",
            path!("/test/main.rs").as_ref(),
            0,
            Some("fn f()".to_string()),
            &project,
            cx,
        )
        .await;
        let display = cx.update(|cx| compute_call_display(&call, cx));

        assert_eq!(display.detail.as_deref(), Some("()"));
    }

    #[gpui::test]
    async fn test_signature_label_highlights(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/test"), json!({"main.rs": source()}))
            .await;
        let project = Project::test(fs, [path!("/test").as_ref()], cx).await;

        let call = make_call(
            "handle_event",
            path!("/test/main.rs").as_ref(),
            0,
            Some("fn handle_event(\n    ctx: &mut InlayHintCtx<'_>,\n) -> bool".to_string()),
            &project,
            cx,
        )
        .await;

        let language = rust_lang_with_highlights();
        let label = signature_label(&call.item, &language).expect("signature label");
        assert_eq!(
            label.text,
            "fn handle_event(ctx: &mut InlayHintCtx<'_>) -> bool"
        );
        assert_eq!(label.filter_range, 3..15);
        assert_eq!(&label.text[label.filter_range.clone()], "handle_event");

        let grammar = language.grammar().expect("grammar");
        let keyword = grammar.highlight_id_for_name("keyword").expect("keyword");
        let function = grammar.highlight_id_for_name("function").expect("function");
        let type_id = grammar.highlight_id_for_name("type").expect("type");
        let lifetime = grammar.highlight_id_for_name("lifetime").expect("lifetime");
        assert_eq!(
            label.runs,
            vec![
                (0..2, keyword),
                (3..15, function),
                (22..25, keyword),
                (26..38, type_id),
                (39..41, lifetime),
            ]
        );
    }

    #[test]
    fn test_find_symbol_name() {
        assert_eq!(find_symbol_name("fn f()", "f"), Some(3));
        assert_eq!(find_symbol_name("static void a(int a)", "a"), Some(12));
        assert_eq!(
            find_symbol_name("fn contains<Q>(q: &Q)", "contains"),
            Some(3)
        );
        assert_eq!(find_symbol_name("fn foo_bar(foo: Foo)", "foo"), Some(11));
        assert_eq!(find_symbol_name("namespace::Class", "foo"), None);
        assert_eq!(find_symbol_name("fn foo()", ""), None);
    }

    #[gpui::test]
    async fn test_call_hierarchy_modal_basic(cx: &mut TestAppContext) {
        init_test(cx);
        let (workspace, fake_server, test_uri, cx) =
            setup_modal_test("fn main() { helper(); }\nfn helper() {}\n", cx).await;

        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = test_uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move { Ok(Some(vec![make_lsp_call_hierarchy_item("main", uri, 0)])) }
            }
        });

        fake_server.set_request_handler::<lsp::request::CallHierarchyOutgoingCalls, _, _>({
            move |_, _| {
                let uri = test_uri.clone();
                async move {
                    Ok(Some(vec![lsp::CallHierarchyOutgoingCall {
                        to: make_lsp_call_hierarchy_item("helper", uri, 1),
                        from_ranges: Vec::new(),
                    }]))
                }
            }
        });

        cx.dispatch_action(ShowOutgoingCalls);
        cx.executor().run_until_parked();

        let modal = workspace
            .update(cx, |workspace, cx| {
                workspace.active_modal::<CallHierarchyView>(cx)
            })
            .expect("Call hierarchy modal should be open");
        modal.read_with(cx, |view, cx| {
            let delegate = &view.picker.read(cx).delegate;
            assert_eq!(delegate.state, FetchState::Loaded);
            assert_eq!(
                delegate
                    .root_item
                    .as_ref()
                    .map(|root_item| root_item.name.as_str()),
                Some("main")
            );
            assert_eq!(delegate.calls.len(), 1);
            assert_eq!(delegate.calls[0].item.name, "helper");
            assert_eq!(delegate.calls[0].site_count, 1);
        });
    }

    #[gpui::test]
    async fn test_call_hierarchy_modal_incoming_mode(cx: &mut TestAppContext) {
        init_test(cx);
        let (workspace, fake_server, test_uri, cx) =
            setup_modal_test("fn main() { helper(); }\nfn helper() {}\n", cx).await;

        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = test_uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move { Ok(Some(vec![make_lsp_call_hierarchy_item("helper", uri, 1)])) }
            }
        });

        fake_server.set_request_handler::<lsp::request::CallHierarchyIncomingCalls, _, _>({
            move |_, _| {
                let uri = test_uri.clone();
                async move {
                    Ok(Some(vec![lsp::CallHierarchyIncomingCall {
                        from: make_lsp_call_hierarchy_item("main", uri, 0),
                        from_ranges: vec![lsp::Range {
                            start: lsp::Position {
                                line: 0,
                                character: 12,
                            },
                            end: lsp::Position {
                                line: 0,
                                character: 18,
                            },
                        }],
                    }]))
                }
            }
        });

        cx.dispatch_action(ShowIncomingCalls);
        cx.executor().run_until_parked();

        let modal = workspace
            .update(cx, |workspace, cx| {
                workspace.active_modal::<CallHierarchyView>(cx)
            })
            .expect("Call hierarchy modal should be open in incoming mode");
        modal.read_with(cx, |view, cx| {
            let delegate = &view.picker.read(cx).delegate;
            assert_eq!(delegate.mode, CallHierarchyMode::Incoming);
            assert_eq!(delegate.calls.len(), 1);
            assert_eq!(delegate.calls[0].item.name, "main");
            let target_start = delegate.calls[0]
                .target
                .range
                .start
                .to_point(&delegate.calls[0].target.buffer.read(cx).snapshot());
            assert_eq!(target_start, language::Point::new(0, 12));
        });

        let editor = workspace
            .update(cx, |workspace, cx| workspace.active_item_as::<Editor>(cx))
            .expect("editor should be open");
        cx.executor()
            .advance_clock(std::time::Duration::from_millis(200));
        cx.dispatch_action(menu::SelectNext);
        let highlighted_rows = highlighted_display_rows(&editor, cx);
        assert_eq!(
            highlighted_rows,
            vec![0],
            "selecting a call should preview its call site in the editor"
        );

        cx.dispatch_action(menu::Cancel);
        let highlighted_rows = highlighted_display_rows(&editor, cx);
        assert_eq!(
            highlighted_rows,
            Vec::<u32>::new(),
            "dismissing the modal should clear the preview highlights"
        );
    }

    #[gpui::test]
    async fn test_call_hierarchy_modal_filtering(cx: &mut TestAppContext) {
        init_test(cx);
        let (workspace, fake_server, test_uri, cx) = setup_modal_test(
            "fn main() { foo(); bar(); baz(); }\nfn foo() {}\nfn bar() {}\nfn baz() {}\n",
            cx,
        )
        .await;

        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = test_uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move { Ok(Some(vec![make_lsp_call_hierarchy_item("main", uri, 0)])) }
            }
        });

        fake_server.set_request_handler::<lsp::request::CallHierarchyOutgoingCalls, _, _>({
            let uri = test_uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move {
                    Ok(Some(vec![
                        lsp::CallHierarchyOutgoingCall {
                            to: make_lsp_call_hierarchy_item("foo", uri.clone(), 1),
                            from_ranges: Vec::new(),
                        },
                        lsp::CallHierarchyOutgoingCall {
                            to: make_lsp_call_hierarchy_item("bar_1", uri.clone(), 2),
                            from_ranges: Vec::new(),
                        },
                        lsp::CallHierarchyOutgoingCall {
                            to: make_lsp_call_hierarchy_item("bar_2", uri.clone(), 2),
                            from_ranges: Vec::new(),
                        },
                        lsp::CallHierarchyOutgoingCall {
                            to: make_lsp_call_hierarchy_item("baz", uri, 3),
                            from_ranges: Vec::new(),
                        },
                    ]))
                }
            }
        });

        cx.dispatch_action(ShowOutgoingCalls);
        cx.executor().run_until_parked();

        let modal = workspace.update(cx, |workspace, cx| {
            workspace.active_modal::<CallHierarchyView>(cx)
        });
        assert!(modal.is_some(), "Modal should be open");

        let modal = modal.unwrap();
        let match_count_before =
            modal.read_with(cx, |view, cx| view.picker.read(cx).delegate.matches.len());
        assert_eq!(match_count_before, 4, "Should have 4 matches initially");

        let task = modal.update_in(cx, |view, window, cx| {
            view.picker.update(cx, |picker, cx| {
                picker
                    .delegate
                    .update_matches("bar".to_string(), window, cx)
            })
        });
        task.await;
        cx.executor().run_until_parked();

        let match_count_after =
            modal.read_with(cx, |view, cx| view.picker.read(cx).delegate.matches.len());
        assert_eq!(
            match_count_after, 2,
            "Filter 'bar' should match 2 entries (bar, baz)"
        );
    }

    #[gpui::test]
    async fn test_call_hierarchy_modal_direction_toggle(cx: &mut TestAppContext) {
        init_test(cx);
        let (workspace, fake_server, test_uri, cx) =
            setup_modal_test("fn main() { helper(); }\nfn helper() {}\n", cx).await;

        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = test_uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move { Ok(Some(vec![make_lsp_call_hierarchy_item("helper", uri, 1)])) }
            }
        });
        fake_server.set_request_handler::<lsp::request::CallHierarchyOutgoingCalls, _, _>({
            let uri = test_uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move {
                    Ok(Some(vec![lsp::CallHierarchyOutgoingCall {
                        to: make_lsp_call_hierarchy_item("callee", uri, 1),
                        from_ranges: Vec::new(),
                    }]))
                }
            }
        });
        fake_server.set_request_handler::<lsp::request::CallHierarchyIncomingCalls, _, _>({
            move |_, _| {
                let uri = test_uri.clone();
                async move {
                    Ok(Some(vec![lsp::CallHierarchyIncomingCall {
                        from: make_lsp_call_hierarchy_item("caller", uri, 0),
                        from_ranges: Vec::new(),
                    }]))
                }
            }
        });

        cx.dispatch_action(ShowOutgoingCalls);
        cx.executor()
            .advance_clock(std::time::Duration::from_millis(200));

        let modal = workspace
            .update(cx, |workspace, cx| {
                workspace.active_modal::<CallHierarchyView>(cx)
            })
            .expect("modal should be open");
        modal.read_with(cx, |view, cx| {
            let delegate = &view.picker.read(cx).delegate;
            assert_eq!(delegate.mode, CallHierarchyMode::Outgoing);
            assert_eq!(delegate.calls[0].item.name, "callee");
        });

        let picker_focused = cx.update(|window, cx| {
            modal
                .read(cx)
                .picker
                .focus_handle(cx)
                .contains_focused(window, cx)
        });
        assert!(picker_focused, "opening the modal should focus the picker");

        cx.dispatch_action(ToggleDirection);
        cx.executor()
            .advance_clock(std::time::Duration::from_millis(200));
        modal.read_with(cx, |view, cx| {
            let delegate = &view.picker.read(cx).delegate;
            assert_eq!(delegate.mode, CallHierarchyMode::Incoming);
            assert_eq!(
                delegate
                    .root_item
                    .as_ref()
                    .map(|root_item| root_item.name.as_str()),
                Some("helper"),
                "toggling direction should keep the same root"
            );
            assert_eq!(delegate.calls[0].item.name, "caller");
        });

        cx.dispatch_action(ShowOutgoingCalls);
        cx.executor()
            .advance_clock(std::time::Duration::from_millis(200));
        modal.read_with(cx, |view, cx| {
            let delegate = &view.picker.read(cx).delegate;
            assert_eq!(delegate.mode, CallHierarchyMode::Outgoing);
            assert_eq!(delegate.calls[0].item.name, "callee");
        });

        cx.dispatch_action(ShowOutgoingCalls);
        cx.executor()
            .advance_clock(std::time::Duration::from_millis(200));
        let has_modal = workspace.update(cx, |workspace, cx| {
            workspace.active_modal::<CallHierarchyView>(cx).is_some()
        });
        assert!(
            !has_modal,
            "dispatching the current mode's action should dismiss the modal"
        );
    }

    #[gpui::test]
    async fn test_call_hierarchy_modal_expand_and_back(cx: &mut TestAppContext) {
        init_test(cx);
        let (workspace, fake_server, test_uri, cx) = setup_modal_test(
            "fn main() { alpha(); helper(); zeta(); }\nfn alpha() {}\nfn helper() { leaf(); }\nfn zeta() {}\nfn leaf() {}\n",
            cx,
        )
        .await;

        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = test_uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move { Ok(Some(vec![make_lsp_call_hierarchy_item("main", uri, 0)])) }
            }
        });
        fake_server.set_request_handler::<lsp::request::CallHierarchyOutgoingCalls, _, _>({
            move |params, _| {
                let uri = test_uri.clone();
                async move {
                    let callees = match params.item.name.as_str() {
                        "main" => vec![
                            make_lsp_call_hierarchy_item("alpha", uri.clone(), 1),
                            make_lsp_call_hierarchy_item("helper", uri.clone(), 2),
                            make_lsp_call_hierarchy_item("zeta", uri, 3),
                        ],
                        "helper" => vec![make_lsp_call_hierarchy_item("leaf", uri, 4)],
                        _ => Vec::new(),
                    };
                    Ok(Some(
                        callees
                            .into_iter()
                            .map(|to| lsp::CallHierarchyOutgoingCall {
                                to,
                                from_ranges: Vec::new(),
                            })
                            .collect(),
                    ))
                }
            }
        });

        cx.dispatch_action(ShowOutgoingCalls);
        cx.executor().run_until_parked();

        let modal = workspace
            .update(cx, |workspace, cx| {
                workspace.active_modal::<CallHierarchyView>(cx)
            })
            .expect("modal should be open");
        let picker = modal.read_with(cx, |view, _| view.picker.clone());
        modal.read_with(cx, |view, cx| {
            let delegate = &view.picker.read(cx).delegate;
            assert_eq!(
                delegate
                    .calls
                    .iter()
                    .map(|call| call.item.name.as_str())
                    .collect::<Vec<_>>(),
                vec!["alpha", "helper", "zeta"]
            );
        });

        cx.update(|_, cx| {
            cx.bind_keys([
                gpui::KeyBinding::new("right", editor::actions::MoveRight, Some("Editor")),
                gpui::KeyBinding::new("left", editor::actions::MoveLeft, Some("Editor")),
                gpui::KeyBinding::new("down", menu::SelectNext, Some("Picker > Editor")),
                gpui::KeyBinding::new(
                    "cmd-k right",
                    menu::SelectChild,
                    Some("CallHierarchyPicker > Picker > Editor"),
                ),
                gpui::KeyBinding::new(
                    "cmd-k left",
                    menu::SelectParent,
                    Some("CallHierarchyPicker > Picker > Editor"),
                ),
            ]);
        });

        cx.simulate_keystrokes("down cmd-k right");
        cx.executor().run_until_parked();
        modal.read_with(cx, |view, cx| {
            let delegate = &view.picker.read(cx).delegate;
            assert_eq!(
                delegate
                    .root_item
                    .as_ref()
                    .map(|root_item| root_item.name.as_str()),
                Some("helper"),
                "the expand chord should re-root the hierarchy at the selected call"
            );
            assert_eq!(
                delegate
                    .root_stack
                    .iter()
                    .map(|item| item.name.as_str())
                    .collect::<Vec<_>>(),
                vec!["main"]
            );
            assert_eq!(delegate.calls[0].item.name, "leaf");
            assert_eq!(view.picker.read(cx).query(cx), "");
        });

        cx.simulate_keystrokes("cmd-k right");
        cx.executor().run_until_parked();
        modal.read_with(cx, |view, cx| {
            let delegate = &view.picker.read(cx).delegate;
            assert_eq!(
                delegate
                    .root_item
                    .as_ref()
                    .map(|root_item| root_item.name.as_str()),
                Some("leaf")
            );
            assert_eq!(delegate.state, FetchState::Loaded);
            assert_eq!(
                delegate.calls.len(),
                0,
                "leaf function should have no callees"
            );
        });

        cx.simulate_keystrokes("cmd-k left");
        cx.executor().run_until_parked();
        modal.read_with(cx, |view, cx| {
            let delegate = &view.picker.read(cx).delegate;
            assert_eq!(
                delegate
                    .root_item
                    .as_ref()
                    .map(|root_item| root_item.name.as_str()),
                Some("helper"),
                "the back chord should step back up from the empty leaf state"
            );
            assert_eq!(delegate.selected_index, 0);
            assert_eq!(
                delegate.selected_call().map(|call| call.item.name.as_str()),
                Some("leaf"),
                "stepping back should select the item we drilled into"
            );
        });

        cx.simulate_keystrokes("cmd-k left");
        cx.executor().run_until_parked();
        modal.read_with(cx, |view, cx| {
            let delegate = &view.picker.read(cx).delegate;
            assert_eq!(
                delegate
                    .root_item
                    .as_ref()
                    .map(|root_item| root_item.name.as_str()),
                Some("main")
            );
            assert_eq!(delegate.root_stack.len(), 0);
            assert_eq!(
                delegate.selected_index, 1,
                "stepping back should reselect the item we drilled into, not the first row"
            );
            assert_eq!(
                delegate.selected_call().map(|call| call.item.name.as_str()),
                Some("helper")
            );
        });

        let collapsed_again = picker.update_in(cx, |picker, window, cx| {
            picker.delegate.select_parent(window, cx)
        });
        assert_eq!(collapsed_again, None);

        cx.simulate_input("he");
        cx.simulate_keystrokes("right");
        cx.executor().run_until_parked();
        modal.read_with(cx, |view, cx| {
            let delegate = &view.picker.read(cx).delegate;
            assert_eq!(
                delegate
                    .root_item
                    .as_ref()
                    .map(|root_item| root_item.name.as_str()),
                Some("main"),
                "a bare arrow must not expand the hierarchy"
            );
            assert_eq!(view.picker.read(cx).query(cx), "he");
        });

        cx.simulate_keystrokes("cmd-k right");
        cx.executor().run_until_parked();
        modal.read_with(cx, |view, cx| {
            let delegate = &view.picker.read(cx).delegate;
            assert_eq!(
                delegate
                    .root_item
                    .as_ref()
                    .map(|root_item| root_item.name.as_str()),
                Some("helper"),
                "expanding a filtered selection should drill into it"
            );
            assert_eq!(view.picker.read(cx).query(cx), "");
        });
    }

    #[gpui::test]
    async fn test_call_hierarchy_modal_refreshes_on_buffer_registration(cx: &mut TestAppContext) {
        init_test(cx);
        let (workspace, fake_server, test_uri, cx) =
            setup_modal_test("fn main() { helper(); }\nfn helper() {}\n", cx).await;

        let server_ready = Arc::new(AtomicBool::new(false));
        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = test_uri.clone();
            let server_ready = server_ready.clone();
            move |_, _| {
                let uri = uri.clone();
                let server_ready = server_ready.clone();
                async move {
                    if server_ready.load(Ordering::Acquire) {
                        Ok(Some(vec![make_lsp_call_hierarchy_item("helper", uri, 1)]))
                    } else {
                        Ok(Some(Vec::new()))
                    }
                }
            }
        });
        fake_server.set_request_handler::<lsp::request::CallHierarchyIncomingCalls, _, _>({
            let uri = test_uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move {
                    Ok(Some(vec![lsp::CallHierarchyIncomingCall {
                        from: make_lsp_call_hierarchy_item("main", uri, 0),
                        from_ranges: Vec::new(),
                    }]))
                }
            }
        });

        cx.dispatch_action(ShowIncomingCalls);
        cx.executor().run_until_parked();

        let modal = workspace
            .update(cx, |workspace, cx| {
                workspace.active_modal::<CallHierarchyView>(cx)
            })
            .expect("modal should be open");
        let (project, buffer_id) = modal.read_with(cx, |view, cx| {
            let delegate = &view.picker.read(cx).delegate;
            assert_eq!(
                delegate.state,
                FetchState::NoSymbol,
                "prepare against a not-yet-ready server should find nothing"
            );
            assert_eq!(
                delegate.root_item.as_ref().map(|item| item.name.as_str()),
                None
            );
            let buffer = delegate
                .queried_buffer
                .clone()
                .expect("queried buffer should be recorded");
            (delegate.project.clone(), buffer.read(cx).remote_id())
        });

        let emit_registered = |cx: &mut VisualTestContext| {
            project.update(cx, |_, cx| {
                cx.emit(project::Event::LanguageServerBufferRegistered {
                    server_id: lsp::LanguageServerId(0),
                    buffer_id,
                    buffer_abs_path: std::path::PathBuf::from(path!("/test/src/main.rs")),
                    name: None,
                });
            });
        };

        server_ready.store(true, Ordering::Release);
        emit_registered(cx);
        cx.executor().run_until_parked();
        modal.read_with(cx, |view, cx| {
            let delegate = &view.picker.read(cx).delegate;
            assert_eq!(
                delegate.root_item.as_ref().map(|item| item.name.as_str()),
                Some("helper"),
                "buffer registration should retry the preparation"
            );
            assert_eq!(delegate.calls[0].item.name, "main");
        });

        fake_server.set_request_handler::<lsp::request::CallHierarchyIncomingCalls, _, _>({
            move |_, _| {
                let uri = test_uri.clone();
                async move {
                    Ok(Some(vec![lsp::CallHierarchyIncomingCall {
                        from: make_lsp_call_hierarchy_item("other", uri, 0),
                        from_ranges: Vec::new(),
                    }]))
                }
            }
        });
        emit_registered(cx);
        emit_work_end(&project, cx);
        cx.executor().run_until_parked();
        modal.read_with(cx, |view, cx| {
            let delegate = &view.picker.read(cx).delegate;
            assert_eq!(
                delegate.calls[0].item.name, "main",
                "server events after results arrived should not refetch"
            );
        });
    }

    #[gpui::test]
    async fn test_call_hierarchy_modal_refreshes_on_lsp_work_end(cx: &mut TestAppContext) {
        init_test(cx);
        let (workspace, fake_server, test_uri, cx) =
            setup_modal_test("fn main() { helper(); }\nfn helper() {}\n", cx).await;

        let server_ready = Arc::new(AtomicBool::new(false));
        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = test_uri.clone();
            let server_ready = server_ready.clone();
            move |_, _| {
                let uri = uri.clone();
                let server_ready = server_ready.clone();
                async move {
                    if server_ready.load(Ordering::Acquire) {
                        Ok(Some(vec![make_lsp_call_hierarchy_item("helper", uri, 1)]))
                    } else {
                        Ok(Some(Vec::new()))
                    }
                }
            }
        });
        fake_server.set_request_handler::<lsp::request::CallHierarchyIncomingCalls, _, _>({
            move |_, _| {
                let uri = test_uri.clone();
                async move {
                    Ok(Some(vec![lsp::CallHierarchyIncomingCall {
                        from: make_lsp_call_hierarchy_item("main", uri, 0),
                        from_ranges: Vec::new(),
                    }]))
                }
            }
        });

        cx.dispatch_action(ShowIncomingCalls);
        cx.executor().run_until_parked();

        let modal = workspace
            .update(cx, |workspace, cx| {
                workspace.active_modal::<CallHierarchyView>(cx)
            })
            .expect("modal should be open");
        let project = modal.read_with(cx, |view, cx| {
            let delegate = &view.picker.read(cx).delegate;
            assert_eq!(delegate.state, FetchState::NoSymbol);
            delegate.project.clone()
        });

        emit_work_end(&project, cx);
        cx.executor().run_until_parked();
        modal.read_with(cx, |view, cx| {
            assert_eq!(
                view.picker.read(cx).delegate.state,
                FetchState::NoSymbol,
                "a retry against a still-empty server should stay empty"
            );
        });

        server_ready.store(true, Ordering::Release);
        emit_work_end(&project, cx);
        cx.executor().run_until_parked();
        modal.read_with(cx, |view, cx| {
            let delegate = &view.picker.read(cx).delegate;
            assert_eq!(
                delegate.root_item.as_ref().map(|item| item.name.as_str()),
                Some("helper"),
                "finished language server work should retry the preparation"
            );
            assert_eq!(delegate.calls[0].item.name, "main");
        });
    }

    #[gpui::test]
    async fn test_call_hierarchy_modal_no_symbol(cx: &mut TestAppContext) {
        init_test(cx);
        let (workspace, fake_server, _test_uri, cx) = setup_modal_test("fn main() {}\n", cx).await;

        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>(
            move |_, _| async move { Ok(Some(Vec::new())) },
        );

        cx.dispatch_action(ShowIncomingCalls);
        cx.executor().run_until_parked();

        let modal = workspace
            .update(cx, |workspace, cx| {
                workspace.active_modal::<CallHierarchyView>(cx)
            })
            .expect("modal should be open");
        modal.read_with(cx, |view, cx| {
            let delegate = &view.picker.read(cx).delegate;
            assert_eq!(delegate.state, FetchState::NoSymbol);
            assert_eq!(delegate.calls.len(), 0);
        });
    }

    #[gpui::test]
    async fn test_call_hierarchy_modal_groups_call_sites(cx: &mut TestAppContext) {
        init_test(cx);
        let (workspace, fake_server, test_uri, cx) = setup_modal_test(
            "fn main() { helper(); helper(); helper(); }\nfn helper() {}\n",
            cx,
        )
        .await;

        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = test_uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move { Ok(Some(vec![make_lsp_call_hierarchy_item("helper", uri, 1)])) }
            }
        });
        fake_server.set_request_handler::<lsp::request::CallHierarchyIncomingCalls, _, _>({
            move |_, _| {
                let uri = test_uri.clone();
                let site = |start: u32, end: u32| lsp::Range {
                    start: lsp::Position {
                        line: 0,
                        character: start,
                    },
                    end: lsp::Position {
                        line: 0,
                        character: end,
                    },
                };
                async move {
                    Ok(Some(vec![lsp::CallHierarchyIncomingCall {
                        from: make_lsp_call_hierarchy_item("main", uri, 0),
                        from_ranges: vec![site(22, 28), site(12, 18), site(32, 38)],
                    }]))
                }
            }
        });

        cx.dispatch_action(ShowIncomingCalls);
        cx.executor().run_until_parked();

        let modal = workspace
            .update(cx, |workspace, cx| {
                workspace.active_modal::<CallHierarchyView>(cx)
            })
            .expect("modal should be open");
        modal.read_with(cx, |view, cx| {
            let delegate = &view.picker.read(cx).delegate;
            assert_eq!(
                delegate.calls.len(),
                1,
                "call sites should be grouped per caller"
            );
            let call = &delegate.calls[0];
            assert_eq!(call.item.name, "main");
            assert_eq!(call.site_count, 3);
            let target_start = call
                .target
                .range
                .start
                .to_point(&call.target.buffer.read(cx).snapshot());
            assert_eq!(
                target_start,
                language::Point::new(0, 12),
                "target should be the earliest call site"
            );
            let path = compute_call_display(call, cx)
                .path
                .expect("path should be present");
            assert!(
                path.ends_with(":1 (3 calls)"),
                "path should mention the call site count, got {path:?}"
            );
        });
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let _state = AppState::test(cx);
            init(cx);
            editor::init(cx);
        });
    }

    fn rust_lang() -> Arc<Language> {
        Arc::new(Language::new(
            LanguageConfig {
                name: LanguageName::new("Rust"),
                matcher: Arc::new(LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..LanguageMatcher::default()
                }),
                ..LanguageConfig::default()
            },
            Some(tree_sitter_rust::LANGUAGE.into()),
        ))
    }

    fn rust_lang_with_highlights() -> Arc<Language> {
        let language = Language::new(
            LanguageConfig {
                name: LanguageName::new("Rust"),
                matcher: Arc::new(LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..LanguageMatcher::default()
                }),
                ..LanguageConfig::default()
            },
            Some(tree_sitter_rust::LANGUAGE.into()),
        )
        .with_highlights_query(
            r#"
            (function_item name: (identifier) @function)
            (function_signature_item name: (identifier) @function)
            (type_identifier) @type
            (lifetime) @lifetime
            "fn" @keyword
            (mutable_specifier) @keyword
            "#,
        )
        .unwrap();
        language.set_theme(&SyntaxTheme::new_test([
            ("function", Hsla::default()),
            ("keyword", Hsla::default()),
            ("type", Hsla::default()),
            ("lifetime", Hsla::default()),
        ]));
        Arc::new(language)
    }

    fn make_lsp_call_hierarchy_item(
        name: &str,
        uri: lsp::Uri,
        line: u32,
    ) -> lsp::CallHierarchyItem {
        lsp::CallHierarchyItem {
            name: name.to_string(),
            kind: lsp::SymbolKind::FUNCTION,
            tags: None,
            detail: Some(format!("fn {name}()")),
            uri,
            range: lsp::Range {
                start: lsp::Position { line, character: 0 },
                end: lsp::Position {
                    line,
                    character: 10,
                },
            },
            selection_range: lsp::Range {
                start: lsp::Position { line, character: 3 },
                end: lsp::Position {
                    line,
                    character: 3 + name.len() as u32,
                },
            },
            data: None,
        }
    }

    fn source() -> String {
        format!("{}\n", "x".repeat(30)).repeat(20)
    }

    async fn make_call(
        name: &str,
        abs_path: &std::path::Path,
        line: u32,
        detail: Option<String>,
        project: &Entity<Project>,
        cx: &mut TestAppContext,
    ) -> Call {
        let buffer = project
            .update(cx, |project, cx| project.open_local_buffer(abs_path, cx))
            .await
            .unwrap();
        let (range, selection_range) = buffer.read_with(cx, |buffer, _| {
            (
                buffer.anchor_after(language::Point::new(line, 0))
                    ..buffer.anchor_before(language::Point::new(line, 10)),
                buffer.anchor_after(language::Point::new(line, 3))
                    ..buffer.anchor_before(language::Point::new(line, 3 + name.len() as u32)),
            )
        });
        let target = Location {
            buffer: buffer.clone(),
            range: selection_range.clone(),
        };
        Call {
            item: CallHierarchyItem {
                buffer,
                server_id: lsp::LanguageServerId(0),
                name: name.to_string(),
                kind: lsp::SymbolKind::FUNCTION,
                detail,
                range,
                selection_range,
                data: None,
            },
            target,
            site_count: 1,
            label: None,
            display: CallDisplay::default(),
        }
    }

    fn emit_work_end(project: &Entity<Project>, cx: &mut VisualTestContext) {
        let lsp_store = project.read_with(cx, |project, _| project.lsp_store());
        lsp_store.update(cx, |_, cx| {
            cx.emit(LspStoreEvent::LanguageServerUpdate {
                language_server_id: lsp::LanguageServerId(0),
                name: None,
                message: proto::update_language_server::Variant::WorkEnd(proto::LspWorkEnd {
                    token: None,
                }),
            });
        });
    }

    fn highlighted_display_rows(editor: &Entity<Editor>, cx: &mut VisualTestContext) -> Vec<u32> {
        editor.update_in(cx, |editor, window, cx| {
            editor
                .highlighted_display_rows(window, cx)
                .into_keys()
                .map(|row| row.0)
                .collect()
        })
    }

    async fn setup_modal_test<'a>(
        source_text: &str,
        cx: &'a mut TestAppContext,
    ) -> (
        Entity<Workspace>,
        lsp::FakeLanguageServer,
        lsp::Uri,
        &'a mut VisualTestContext,
    ) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/test"), json!({"src": {"main.rs": source_text}}))
            .await;

        let project = Project::test(fs, [path!("/test").as_ref()], cx).await;
        project.read_with(cx, |project, _| project.languages().add(rust_lang()));

        let mut fake_servers = project.read_with(cx, |project, _| {
            project.languages().register_fake_lsp(
                "Rust",
                FakeLspAdapter {
                    capabilities: lsp::ServerCapabilities {
                        call_hierarchy_provider: Some(lsp::CallHierarchyServerCapability::Simple(
                            true,
                        )),
                        ..lsp::ServerCapabilities::default()
                    },
                    ..FakeLspAdapter::default()
                },
            )
        });

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace =
            multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());

        let worktree_id = workspace.update(cx, |workspace, cx| {
            workspace.project().update(cx, |project, cx| {
                project.worktrees(cx).next().unwrap().read(cx).id()
            })
        });

        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path(
                    (worktree_id, rel_path("src/main.rs")),
                    None,
                    true,
                    window,
                    cx,
                )
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        let fake_server = fake_servers.next().await.unwrap();
        cx.executor().run_until_parked();
        let test_uri = lsp::Uri::from_file_path(path!("/test/src/main.rs")).unwrap();
        (workspace, fake_server, test_uri, cx)
    }
}
