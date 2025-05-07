use super::stack_frame_list::{StackFrameList, StackFrameListEvent};
use dap::{ScopePresentationHint, StackFrameId, VariablePresentationHintKind, VariableReference};
use editor::Editor;
use gpui::{
    AnyElement, ClickEvent, ClipboardItem, Context, DismissEvent, Entity, FocusHandle, Focusable,
    Hsla, MouseButton, MouseDownEvent, Point, Stateful, Subscription, TextStyleRefinement,
    UniformListScrollHandle, actions, anchored, deferred, uniform_list,
};
use menu::{SelectFirst, SelectLast, SelectNext, SelectPrevious};
use project::debugger::session::{Session, SessionEvent};
use std::{collections::HashMap, ops::Range, sync::Arc};
use ui::{ContextMenu, ListItem, Scrollbar, ScrollbarState, prelude::*};
use util::{debug_panic, maybe};

actions!(variable_list, [ExpandSelectedEntry, CollapseSelectedEntry]);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct EntryState {
    depth: usize,
    is_expanded: bool,
    has_children: bool,
    parent_reference: VariableReference,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct EntryPath {
    pub leaf_name: Option<SharedString>,
    pub indices: Arc<[SharedString]>,
}

impl EntryPath {
    fn for_scope(scope_name: impl Into<SharedString>) -> Self {
        Self {
            leaf_name: Some(scope_name.into()),
            indices: Arc::new([]),
        }
    }

    fn with_name(&self, name: SharedString) -> Self {
        Self {
            leaf_name: Some(name),
            indices: self.indices.clone(),
        }
    }

    /// Create a new child of this variable path
    fn with_child(&self, name: SharedString) -> Self {
        Self {
            leaf_name: None,
            indices: self
                .indices
                .iter()
                .cloned()
                .chain(std::iter::once(name))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum EntryKind {
    Variable(dap::Variable),
    Scope(dap::Scope),
}

impl EntryKind {
    fn as_variable(&self) -> Option<&dap::Variable> {
        match self {
            EntryKind::Variable(dap) => Some(dap),
            _ => None,
        }
    }

    fn as_scope(&self) -> Option<&dap::Scope> {
        match self {
            EntryKind::Scope(dap) => Some(dap),
            _ => None,
        }
    }

    #[allow(dead_code)]
    fn name(&self) -> &str {
        match self {
            EntryKind::Variable(dap) => &dap.name,
            EntryKind::Scope(dap) => &dap.name,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ListEntry {
    dap_kind: EntryKind,
    path: EntryPath,
}

impl ListEntry {
    fn as_variable(&self) -> Option<&dap::Variable> {
        self.dap_kind.as_variable()
    }

    fn as_scope(&self) -> Option<&dap::Scope> {
        self.dap_kind.as_scope()
    }

    fn item_id(&self) -> ElementId {
        use std::fmt::Write;
        let mut id = match &self.dap_kind {
            EntryKind::Variable(dap) => format!("variable-{}", dap.name),
            EntryKind::Scope(dap) => format!("scope-{}", dap.name),
        };
        for name in self.path.indices.iter() {
            _ = write!(id, "-{}", name);
        }
        SharedString::from(id).into()
    }

    fn item_value_id(&self) -> ElementId {
        use std::fmt::Write;
        let mut id = match &self.dap_kind {
            EntryKind::Variable(dap) => format!("variable-{}", dap.name),
            EntryKind::Scope(dap) => format!("scope-{}", dap.name),
        };
        for name in self.path.indices.iter() {
            _ = write!(id, "-{}", name);
        }
        _ = write!(id, "-value");
        SharedString::from(id).into()
    }
}

pub struct VariableList {
    entries: Vec<ListEntry>,
    entry_states: HashMap<EntryPath, EntryState>,
    selected_stack_frame_id: Option<StackFrameId>,
    list_handle: UniformListScrollHandle,
    scrollbar_state: ScrollbarState,
    session: Entity<Session>,
    selection: Option<EntryPath>,
    open_context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    focus_handle: FocusHandle,
    edited_path: Option<(EntryPath, Entity<Editor>)>,
    disabled: bool,
    _subscriptions: Vec<Subscription>,
}

impl VariableList {
    pub fn new(
        session: Entity<Session>,
        stack_frame_list: Entity<StackFrameList>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        let _subscriptions = vec![
            cx.subscribe(&stack_frame_list, Self::handle_stack_frame_list_events),
            cx.subscribe(&session, |this, _, event, _| match event {
                SessionEvent::Stopped(_) => {
                    this.selection.take();
                    this.edited_path.take();
                    this.selected_stack_frame_id.take();
                }
                _ => {}
            }),
            cx.on_focus_out(&focus_handle, window, |this, _, _, cx| {
                this.edited_path.take();
                cx.notify();
            }),
        ];

        let list_state = UniformListScrollHandle::default();

        Self {
            scrollbar_state: ScrollbarState::new(list_state.clone()),
            list_handle: list_state,
            session,
            focus_handle,
            _subscriptions,
            selected_stack_frame_id: None,
            selection: None,
            open_context_menu: None,
            disabled: false,
            edited_path: None,
            entries: Default::default(),
            entry_states: Default::default(),
        }
    }

    pub(super) fn disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        let old_disabled = std::mem::take(&mut self.disabled);
        self.disabled = disabled;
        if old_disabled != disabled {
            cx.notify();
        }
    }

    pub(super) fn has_open_context_menu(&self) -> bool {
        self.open_context_menu.is_some()
    }

    fn build_entries(&mut self, cx: &mut Context<Self>) {
        let Some(stack_frame_id) = self.selected_stack_frame_id else {
            return;
        };

        let mut entries = vec![];
        let scopes: Vec<_> = self.session.update(cx, |session, cx| {
            session.scopes(stack_frame_id, cx).iter().cloned().collect()
        });

        let mut contains_local_scope = false;

        let mut stack = scopes
            .into_iter()
            .rev()
            .filter(|scope| {
                if scope
                    .presentation_hint
                    .as_ref()
                    .map(|hint| *hint == ScopePresentationHint::Locals)
                    .unwrap_or(scope.name.to_lowercase().starts_with("local"))
                {
                    contains_local_scope = true;
                }

                self.session.update(cx, |session, cx| {
                    session.variables(scope.variables_reference, cx).len() > 0
                })
            })
            .map(|scope| {
                (
                    scope.variables_reference,
                    scope.variables_reference,
                    EntryPath::for_scope(&scope.name),
                    EntryKind::Scope(scope),
                )
            })
            .collect::<Vec<_>>();

        let scopes_count = stack.len();

        while let Some((container_reference, variables_reference, mut path, dap_kind)) = stack.pop()
        {
            match &dap_kind {
                EntryKind::Variable(dap) => path = path.with_name(dap.name.clone().into()),
                EntryKind::Scope(dap) => path = path.with_child(dap.name.clone().into()),
            }

            let var_state = self
                .entry_states
                .entry(path.clone())
                .and_modify(|state| {
                    state.parent_reference = container_reference;
                    state.has_children = variables_reference != 0;
                })
                .or_insert(EntryState {
                    depth: path.indices.len(),
                    is_expanded: dap_kind.as_scope().is_some_and(|scope| {
                        (scopes_count == 1 && !contains_local_scope)
                            || scope
                                .presentation_hint
                                .as_ref()
                                .map(|hint| *hint == ScopePresentationHint::Locals)
                                .unwrap_or(scope.name.to_lowercase().starts_with("local"))
                    }),
                    parent_reference: container_reference,
                    has_children: variables_reference != 0,
                });

            entries.push(ListEntry {
                dap_kind,
                path: path.clone(),
            });

            if var_state.is_expanded {
                let children = self
                    .session
                    .update(cx, |session, cx| session.variables(variables_reference, cx));
                stack.extend(children.into_iter().rev().map(|child| {
                    (
                        variables_reference,
                        child.variables_reference,
                        path.with_child(child.name.clone().into()),
                        EntryKind::Variable(child),
                    )
                }));
            }
        }

        self.entries = entries;
        cx.notify();
    }

    fn handle_stack_frame_list_events(
        &mut self,
        _: Entity<StackFrameList>,
        event: &StackFrameListEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            StackFrameListEvent::SelectedStackFrameChanged(stack_frame_id) => {
                self.selected_stack_frame_id = Some(*stack_frame_id);
                cx.notify();
            }
        }
    }

    pub fn completion_variables(&self, _cx: &mut Context<Self>) -> Vec<dap::Variable> {
        self.entries
            .iter()
            .filter_map(|entry| match &entry.dap_kind {
                EntryKind::Variable(dap) => Some(dap.clone()),
                EntryKind::Scope(_) => None,
            })
            .collect()
    }

    fn render_entries(
        &mut self,
        ix: Range<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        ix.into_iter()
            .filter_map(|ix| {
                let (entry, state) = self
                    .entries
                    .get(ix)
                    .and_then(|entry| Some(entry).zip(self.entry_states.get(&entry.path)))?;

                match &entry.dap_kind {
                    EntryKind::Variable(_) => Some(self.render_variable(entry, *state, window, cx)),
                    EntryKind::Scope(_) => Some(self.render_scope(entry, *state, cx)),
                }
            })
            .collect()
    }

    pub(crate) fn toggle_entry(&mut self, var_path: &EntryPath, cx: &mut Context<Self>) {
        let Some(entry) = self.entry_states.get_mut(var_path) else {
            log::error!("Could not find variable list entry state to toggle");
            return;
        };

        entry.is_expanded = !entry.is_expanded;
        cx.notify();
    }

    fn select_first(&mut self, _: &SelectFirst, window: &mut Window, cx: &mut Context<Self>) {
        self.cancel_variable_edit(&Default::default(), window, cx);
        if let Some(variable) = self.entries.first() {
            self.selection = Some(variable.path.clone());
            cx.notify();
        }
    }

    fn select_last(&mut self, _: &SelectLast, window: &mut Window, cx: &mut Context<Self>) {
        self.cancel_variable_edit(&Default::default(), window, cx);
        if let Some(variable) = self.entries.last() {
            self.selection = Some(variable.path.clone());
            cx.notify();
        }
    }

    fn select_prev(&mut self, _: &SelectPrevious, window: &mut Window, cx: &mut Context<Self>) {
        self.cancel_variable_edit(&Default::default(), window, cx);
        if let Some(selection) = &self.selection {
            let index = self.entries.iter().enumerate().find_map(|(ix, var)| {
                if &var.path == selection && ix > 0 {
                    Some(ix.saturating_sub(1))
                } else {
                    None
                }
            });

            if let Some(new_selection) =
                index.and_then(|ix| self.entries.get(ix).map(|var| var.path.clone()))
            {
                self.selection = Some(new_selection);
                cx.notify();
            } else {
                self.select_last(&SelectLast, window, cx);
            }
        } else {
            self.select_last(&SelectLast, window, cx);
        }
    }

    fn select_next(&mut self, _: &SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        self.cancel_variable_edit(&Default::default(), window, cx);
        if let Some(selection) = &self.selection {
            let index = self.entries.iter().enumerate().find_map(|(ix, var)| {
                if &var.path == selection {
                    Some(ix.saturating_add(1))
                } else {
                    None
                }
            });

            if let Some(new_selection) =
                index.and_then(|ix| self.entries.get(ix).map(|var| var.path.clone()))
            {
                self.selection = Some(new_selection);
                cx.notify();
            } else {
                self.select_first(&SelectFirst, window, cx);
            }
        } else {
            self.select_first(&SelectFirst, window, cx);
        }
    }

    fn cancel_variable_edit(
        &mut self,
        _: &menu::Cancel,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.edited_path.take();
        self.focus_handle.focus(window);
        cx.notify();
    }

    fn confirm_variable_edit(
        &mut self,
        _: &menu::Confirm,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let res = maybe!({
            let (var_path, editor) = self.edited_path.take()?;
            let state = self.entry_states.get(&var_path)?;
            let variables_reference = state.parent_reference;
            let name = var_path.leaf_name?;
            let value = editor.read(cx).text(cx);

            self.session.update(cx, |session, cx| {
                session.set_variable_value(variables_reference, name.into(), value, cx)
            });
            Some(())
        });

        if res.is_none() {
            log::error!(
                "Couldn't confirm variable edit because variable doesn't have a leaf name or a parent reference id"
            );
        }
    }

    fn collapse_selected_entry(
        &mut self,
        _: &CollapseSelectedEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(ref selected_entry) = self.selection {
            let Some(entry_state) = self.entry_states.get_mut(selected_entry) else {
                debug_panic!("Trying to toggle variable in variable list that has an no state");
                return;
            };

            if !entry_state.is_expanded || !entry_state.has_children {
                self.select_prev(&SelectPrevious, window, cx);
            } else {
                entry_state.is_expanded = false;
                cx.notify();
            }
        }
    }

    fn expand_selected_entry(
        &mut self,
        _: &ExpandSelectedEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(selected_entry) = &self.selection {
            let Some(entry_state) = self.entry_states.get_mut(selected_entry) else {
                debug_panic!("Trying to toggle variable in variable list that has an no state");
                return;
            };

            if entry_state.is_expanded || !entry_state.has_children {
                self.select_next(&SelectNext, window, cx);
            } else {
                entry_state.is_expanded = true;
                cx.notify();
            }
        }
    }

    fn deploy_variable_context_menu(
        &mut self,
        variable: ListEntry,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(dap_var) = variable.as_variable() else {
            debug_panic!("Trying to open variable context menu on a scope");
            return;
        };

        let variable_value = dap_var.value.clone();
        let variable_name = dap_var.name.clone();
        let this = cx.entity().clone();

        let context_menu = ContextMenu::build(window, cx, |menu, _, _| {
            menu.entry("Copy name", None, move |_, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(variable_name.clone()))
            })
            .entry("Copy value", None, {
                let variable_value = variable_value.clone();
                move |_, cx| {
                    cx.write_to_clipboard(ClipboardItem::new_string(variable_value.clone()))
                }
            })
            .entry("Set value", None, move |window, cx| {
                this.update(cx, |variable_list, cx| {
                    let editor = Self::create_variable_editor(&variable_value, window, cx);
                    variable_list.edited_path = Some((variable.path.clone(), editor));

                    cx.notify();
                });
            })
        });

        cx.focus_view(&context_menu, window);
        let subscription = cx.subscribe_in(
            &context_menu,
            window,
            |this, _, _: &DismissEvent, window, cx| {
                if this.open_context_menu.as_ref().is_some_and(|context_menu| {
                    context_menu.0.focus_handle(cx).contains_focused(window, cx)
                }) {
                    cx.focus_self(window);
                }
                this.open_context_menu.take();
                cx.notify();
            },
        );

        self.open_context_menu = Some((context_menu, position, subscription));
    }

    #[track_caller]
    #[cfg(test)]
    pub(crate) fn assert_visual_entries(&self, expected: Vec<&str>) {
        const INDENT: &'static str = "    ";

        let entries = &self.entries;
        let mut visual_entries = Vec::with_capacity(entries.len());
        for entry in entries {
            let state = self
                .entry_states
                .get(&entry.path)
                .expect("If there's a variable entry there has to be a state that goes with it");

            visual_entries.push(format!(
                "{}{} {}{}",
                INDENT.repeat(state.depth - 1),
                if state.is_expanded { "v" } else { ">" },
                entry.dap_kind.name(),
                if self.selection.as_ref() == Some(&entry.path) {
                    " <=== selected"
                } else {
                    ""
                }
            ));
        }

        pretty_assertions::assert_eq!(expected, visual_entries);
    }

    #[track_caller]
    #[cfg(test)]
    pub(crate) fn scopes(&self) -> Vec<dap::Scope> {
        self.entries
            .iter()
            .filter_map(|entry| match &entry.dap_kind {
                EntryKind::Scope(scope) => Some(scope),
                _ => None,
            })
            .cloned()
            .collect()
    }

    #[track_caller]
    #[cfg(test)]
    pub(crate) fn variables_per_scope(&self) -> Vec<(dap::Scope, Vec<dap::Variable>)> {
        let mut scopes: Vec<(dap::Scope, Vec<_>)> = Vec::new();
        let mut idx = 0;

        for entry in self.entries.iter() {
            match &entry.dap_kind {
                EntryKind::Variable(dap) => scopes[idx].1.push(dap.clone()),
                EntryKind::Scope(scope) => {
                    if scopes.len() > 0 {
                        idx += 1;
                    }

                    scopes.push((scope.clone(), Vec::new()));
                }
            }
        }

        scopes
    }

    #[track_caller]
    #[cfg(test)]
    pub(crate) fn variables(&self) -> Vec<dap::Variable> {
        self.entries
            .iter()
            .filter_map(|entry| match &entry.dap_kind {
                EntryKind::Variable(variable) => Some(variable),
                _ => None,
            })
            .cloned()
            .collect()
    }

    fn create_variable_editor(default: &str, window: &mut Window, cx: &mut App) -> Entity<Editor> {
        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);

            let refinement = TextStyleRefinement {
                font_size: Some(
                    TextSize::XSmall
                        .rems(cx)
                        .to_pixels(window.rem_size())
                        .into(),
                ),
                ..Default::default()
            };
            editor.set_text_style_refinement(refinement);
            editor.set_text(default, window, cx);
            editor.select_all(&editor::actions::SelectAll, window, cx);
            editor
        });
        editor.focus_handle(cx).focus(window);
        editor
    }

    fn render_scope(
        &self,
        entry: &ListEntry,
        state: EntryState,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(scope) = entry.as_scope() else {
            debug_panic!("Called render scope on non scope variable list entry variant");
            return div().into_any_element();
        };

        let var_ref = scope.variables_reference;
        let is_selected = self
            .selection
            .as_ref()
            .is_some_and(|selection| selection == &entry.path);

        let colors = get_entry_color(cx);
        let bg_hover_color = if !is_selected {
            colors.hover
        } else {
            colors.default
        };
        let border_color = if is_selected {
            colors.marked_active
        } else {
            colors.default
        };
        let path = entry.path.clone();

        div()
            .id(var_ref as usize)
            .group("variable_list_entry")
            .border_1()
            .border_r_2()
            .border_color(border_color)
            .flex()
            .w_full()
            .h_full()
            .hover(|style| style.bg(bg_hover_color))
            .on_click(cx.listener({
                move |this, _, _window, cx| {
                    this.selection = Some(path.clone());
                    cx.notify();
                }
            }))
            .child(
                ListItem::new(SharedString::from(format!("scope-{}", var_ref)))
                    .selectable(false)
                    .disabled(self.disabled)
                    .indent_level(state.depth)
                    .indent_step_size(px(10.))
                    .always_show_disclosure_icon(true)
                    .toggle(state.is_expanded)
                    .on_toggle({
                        let var_path = entry.path.clone();
                        cx.listener(move |this, _, _, cx| this.toggle_entry(&var_path, cx))
                    })
                    .child(
                        div()
                            .text_ui(cx)
                            .w_full()
                            .when(self.disabled, |this| {
                                this.text_color(Color::Disabled.color(cx))
                            })
                            .child(scope.name.clone()),
                    ),
            )
            .into_any()
    }

    fn render_variable(
        &self,
        variable: &ListEntry,
        state: EntryState,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let dap = match &variable.dap_kind {
            EntryKind::Variable(dap) => dap,
            EntryKind::Scope(_) => {
                debug_panic!("Called render variable on variable list entry kind scope");
                return div().into_any_element();
            }
        };

        let syntax_color_for = |name| cx.theme().syntax().get(name).color;
        let variable_name_color = if self.disabled {
            Some(Color::Disabled.color(cx))
        } else {
            match &dap
                .presentation_hint
                .as_ref()
                .and_then(|hint| hint.kind.as_ref())
                .unwrap_or(&VariablePresentationHintKind::Unknown)
            {
                VariablePresentationHintKind::Class
                | VariablePresentationHintKind::BaseClass
                | VariablePresentationHintKind::InnerClass
                | VariablePresentationHintKind::MostDerivedClass => syntax_color_for("type"),
                VariablePresentationHintKind::Data => syntax_color_for("variable"),
                VariablePresentationHintKind::Unknown | _ => syntax_color_for("variable"),
            }
        };
        let variable_color = self
            .disabled
            .then(|| Color::Disabled.color(cx))
            .or_else(|| syntax_color_for("variable.special"));

        let var_ref = dap.variables_reference;
        let colors = get_entry_color(cx);
        let is_selected = self
            .selection
            .as_ref()
            .is_some_and(|selected_path| *selected_path == variable.path);

        let bg_hover_color = if !is_selected {
            colors.hover
        } else {
            colors.default
        };
        let border_color = if is_selected && self.focus_handle.contains_focused(window, cx) {
            colors.marked_active
        } else {
            colors.default
        };
        let path = variable.path.clone();
        div()
            .id(variable.item_id())
            .group("variable_list_entry")
            .border_1()
            .border_r_2()
            .border_color(border_color)
            .h_4()
            .size_full()
            .hover(|style| style.bg(bg_hover_color))
            .on_click(cx.listener({
                move |this, _, _window, cx| {
                    this.selection = Some(path.clone());
                    cx.notify();
                }
            }))
            .child(
                ListItem::new(SharedString::from(format!(
                    "variable-item-{}-{}",
                    dap.name, state.depth
                )))
                .disabled(self.disabled)
                .selectable(false)
                .indent_level(state.depth)
                .indent_step_size(px(10.))
                .always_show_disclosure_icon(true)
                .when(var_ref > 0, |list_item| {
                    list_item.toggle(state.is_expanded).on_toggle(cx.listener({
                        let var_path = variable.path.clone();
                        move |this, _, _, cx| {
                            this.session.update(cx, |session, cx| {
                                session.variables(var_ref, cx);
                            });

                            this.toggle_entry(&var_path, cx);
                        }
                    }))
                })
                .on_secondary_mouse_down(cx.listener({
                    let variable = variable.clone();
                    move |this, event: &MouseDownEvent, window, cx| {
                        this.deploy_variable_context_menu(
                            variable.clone(),
                            event.position,
                            window,
                            cx,
                        )
                    }
                }))
                .child(
                    h_flex()
                        .gap_1()
                        .text_ui_sm(cx)
                        .w_full()
                        .child(
                            Label::new(&dap.name).when_some(variable_name_color, |this, color| {
                                this.color(Color::from(color))
                            }),
                        )
                        .when(!dap.value.is_empty(), |this| {
                            this.child(div().w_full().id(variable.item_value_id()).map(|this| {
                                if let Some((_, editor)) = self
                                    .edited_path
                                    .as_ref()
                                    .filter(|(path, _)| path == &variable.path)
                                {
                                    this.child(div().size_full().px_2().child(editor.clone()))
                                } else {
                                    this.text_color(cx.theme().colors().text_muted)
                                        .when(
                                            !self.disabled
                                                && self
                                                    .session
                                                    .read(cx)
                                                    .capabilities()
                                                    .supports_set_variable
                                                    .unwrap_or_default(),
                                            |this| {
                                                let path = variable.path.clone();
                                                let variable_value = dap.value.clone();
                                                this.on_click(cx.listener(
                                                    move |this, click: &ClickEvent, window, cx| {
                                                        if click.down.click_count < 2 {
                                                            return;
                                                        }
                                                        let editor = Self::create_variable_editor(
                                                            &variable_value,
                                                            window,
                                                            cx,
                                                        );
                                                        this.edited_path =
                                                            Some((path.clone(), editor));

                                                        cx.notify();
                                                    },
                                                ))
                                            },
                                        )
                                        .child(
                                            Label::new(format!("=  {}", &dap.value))
                                                .single_line()
                                                .truncate()
                                                .size(LabelSize::Small)
                                                .color(Color::Muted)
                                                .when_some(variable_color, |this, color| {
                                                    this.color(Color::from(color))
                                                }),
                                        )
                                }
                            }))
                        }),
                ),
            )
            .into_any()
    }

    fn render_vertical_scrollbar(&self, cx: &mut Context<Self>) -> Stateful<Div> {
        div()
            .occlude()
            .id("variable-list-vertical-scrollbar")
            .on_mouse_move(cx.listener(|_, _, _, cx| {
                cx.notify();
                cx.stop_propagation()
            }))
            .on_hover(|_, _, cx| {
                cx.stop_propagation();
            })
            .on_any_mouse_down(|_, _, cx| {
                cx.stop_propagation();
            })
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|_, _, _, cx| {
                    cx.stop_propagation();
                }),
            )
            .on_scroll_wheel(cx.listener(|_, _, _, cx| {
                cx.notify();
            }))
            .h_full()
            .absolute()
            .right_1()
            .top_1()
            .bottom_0()
            .w(px(12.))
            .cursor_default()
            .children(Scrollbar::vertical(self.scrollbar_state.clone()))
    }
}

impl Focusable for VariableList {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for VariableList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.build_entries(cx);

        v_flex()
            .track_focus(&self.focus_handle)
            .key_context("VariableList")
            .id("variable-list")
            .group("variable-list")
            .overflow_y_scroll()
            .size_full()
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::select_prev))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::expand_selected_entry))
            .on_action(cx.listener(Self::collapse_selected_entry))
            .on_action(cx.listener(Self::cancel_variable_edit))
            .on_action(cx.listener(Self::confirm_variable_edit))
            //
            .child(
                uniform_list(
                    cx.entity().clone(),
                    "variable-list",
                    self.entries.len(),
                    move |this, range, window, cx| this.render_entries(range, window, cx),
                )
                .track_scroll(self.list_handle.clone())
                .gap_1_5()
                .size_full()
                .flex_grow(),
            )
            .children(self.open_context_menu.as_ref().map(|(menu, position, _)| {
                deferred(
                    anchored()
                        .position(*position)
                        .anchor(gpui::Corner::TopLeft)
                        .child(menu.clone()),
                )
                .with_priority(1)
            }))
            .child(self.render_vertical_scrollbar(cx))
    }
}

struct EntryColors {
    default: Hsla,
    hover: Hsla,
    marked_active: Hsla,
}

fn get_entry_color(cx: &Context<VariableList>) -> EntryColors {
    let colors = cx.theme().colors();

    EntryColors {
        default: colors.panel_background,
        hover: colors.ghost_element_hover,
        marked_active: colors.ghost_element_selected,
    }
}
