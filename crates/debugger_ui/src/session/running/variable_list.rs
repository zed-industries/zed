use super::stack_frame_list::{StackFrameList, StackFrameListEvent};
use dap::{StackFrameId, VariableReference};
use editor::Editor;
use gpui::{
    actions, anchored, deferred, list, AnyElement, ClickEvent, ClipboardItem, Context,
    DismissEvent, Entity, FocusHandle, Focusable, Hsla, ListOffset, ListState, MouseDownEvent,
    Point, Subscription,
};
use menu::{SelectFirst, SelectLast, SelectNext, SelectPrevious};
use project::debugger::session::{Session, SessionEvent};
use std::{collections::HashMap, sync::Arc};
use ui::{prelude::*, ContextMenu, ListItem};
use util::{debug_panic, maybe};

actions!(variable_list, [ExpandSelectedEntry, CollapseSelectedEntry]);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct VariableState {
    depth: usize,
    is_expanded: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct VariablePath {
    pub leaf_name: Option<SharedString>,
    pub indices: Arc<[VariableReference]>,
}

impl VariablePath {
    fn for_scope(scope_id: VariableReference) -> Self {
        Self {
            leaf_name: None,
            indices: Arc::new([scope_id]),
        }
    }

    fn with_name(&self, name: SharedString) -> Self {
        Self {
            leaf_name: Some(name),
            indices: self.indices.clone(),
        }
    }

    /// Create a new child of this variable path
    fn with_child(&self, variable_reference: VariableReference) -> Self {
        Self {
            leaf_name: None,
            indices: self
                .indices
                .iter()
                .cloned()
                .chain(std::iter::once(variable_reference))
                .collect(),
        }
    }

    fn parent_reference_id(&self) -> VariableReference {
        self.indices
            .last()
            .copied()
            .expect("VariablePath should have at least one variable reference")
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Variable {
    dap: dap::Variable,
    path: VariablePath,
}

impl Variable {
    fn item_id(&self) -> ElementId {
        use std::fmt::Write;
        let mut id = format!("variable-{}", self.dap.name);
        for index in self.path.indices.iter() {
            _ = write!(id, "-{}", index);
        }
        SharedString::from(id).into()
    }

    fn item_value_id(&self) -> ElementId {
        use std::fmt::Write;
        let mut id = format!("variable-{}", self.dap.name);
        for index in self.path.indices.iter() {
            _ = write!(id, "-{}", index);
        }
        _ = write!(id, "-value");
        SharedString::from(id).into()
    }
}

pub struct VariableList {
    entries: Vec<Variable>,
    variable_states: HashMap<VariablePath, VariableState>,
    selected_stack_frame_id: Option<StackFrameId>,
    list: ListState,
    session: Entity<Session>,
    selection: Option<VariablePath>,
    open_context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    focus_handle: FocusHandle,
    edited_path: Option<(VariablePath, Entity<Editor>)>,
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
        let weak_variable_list = cx.weak_entity();
        let focus_handle = cx.focus_handle();

        let list = ListState::new(
            0,
            gpui::ListAlignment::Top,
            px(1000.),
            move |ix, window, cx| {
                weak_variable_list
                    .upgrade()
                    .map(|var_list| {
                        var_list.update(cx, |this, cx| this.render_entry(ix, window, cx))
                    })
                    .unwrap_or(div().into_any())
            },
        );

        let _subscriptions = vec![
            cx.subscribe(&stack_frame_list, Self::handle_stack_frame_list_events),
            cx.subscribe(&session, |this, _, event, cx| {
                match event {
                    SessionEvent::Stopped => {
                        this.variable_states.clear();
                    }
                    _ => {}
                }
                this.build_entries(cx);
            }),
            cx.on_focus_out(&focus_handle, window, |this, _, _, cx| {
                this.edited_path.take();
                cx.notify();
            }),
        ];

        Self {
            list,
            session,
            focus_handle,
            _subscriptions,
            selected_stack_frame_id: None,
            selection: None,
            open_context_menu: None,
            disabled: false,
            edited_path: None,
            entries: Default::default(),
            variable_states: Default::default(),
        }
    }

    pub(super) fn disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        let old_disabled = std::mem::take(&mut self.disabled);
        self.disabled = disabled;
        if old_disabled != disabled {
            cx.notify();
        }
    }

    fn build_entries(&mut self, cx: &mut Context<Self>) {
        let Some(stack_frame_id) = self.selected_stack_frame_id else {
            return;
        };

        let mut entries = vec![];
        let scopes: Vec<_> = self.session.update(cx, |session, cx| {
            session.scopes(stack_frame_id, cx).iter().cloned().collect()
        });
        let mut stack = scopes
            .into_iter()
            .rev()
            .map(|scope| {
                (
                    scope.variables_reference,
                    None::<dap::Variable>,
                    VariablePath::for_scope(scope.variables_reference),
                )
            })
            .collect::<Vec<_>>();

        while let Some((variable_reference, dap, mut path)) = stack.pop() {
            if let Some(dap) = &dap {
                path = path.with_name(dap.name.clone().into());
            }
            let var_state = self
                .variable_states
                .entry(path.clone())
                .or_insert(VariableState {
                    depth: path.indices.len() + path.leaf_name.is_some() as usize,
                    is_expanded: dap.is_none(),
                });
            if let Some(dap) = dap {
                entries.push(Variable {
                    dap,
                    path: path.clone(),
                });
            }

            if var_state.is_expanded {
                let children = self
                    .session
                    .update(cx, |session, cx| session.variables(variable_reference, cx));
                stack.extend(children.into_iter().rev().map(|child| {
                    (
                        child.variables_reference,
                        Some(child),
                        path.with_child(variable_reference),
                    )
                }));
            }
        }

        let old_scroll_top = self.list.logical_scroll_top();

        if self.entries.len() != entries.len() {
            self.list.reset(entries.len());
        }

        let old_entries = &self.entries;
        if let Some(old_top_entry) = old_entries.get(old_scroll_top.item_ix) {
            let new_scroll_top = old_entries
                .iter()
                .position(|entry| entry == old_top_entry)
                .map(|item_ix| ListOffset {
                    item_ix,
                    offset_in_item: old_scroll_top.offset_in_item,
                })
                .or_else(|| {
                    let entry_after_old_top = old_entries.get(old_scroll_top.item_ix + 1)?;
                    let item_ix = entries
                        .iter()
                        .position(|entry| entry == entry_after_old_top)?;
                    Some(ListOffset {
                        item_ix,
                        offset_in_item: Pixels::ZERO,
                    })
                })
                .or_else(|| {
                    let entry_before_old_top =
                        old_entries.get(old_scroll_top.item_ix.saturating_sub(1))?;
                    let item_ix = entries
                        .iter()
                        .position(|entry| entry == entry_before_old_top)?;
                    Some(ListOffset {
                        item_ix,
                        offset_in_item: Pixels::ZERO,
                    })
                });

            self.list
                .scroll_to(new_scroll_top.unwrap_or(old_scroll_top));
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
                self.build_entries(cx);
            }
        }
    }

    // debugger(todo): This only returns visible variables will need to change it to show all variables
    // within a stack frame scope
    pub fn completion_variables(&self, _cx: &mut Context<Self>) -> Vec<dap::Variable> {
        self.entries
            .iter()
            .map(|variable| variable.dap.clone())
            .collect()
    }

    fn render_entry(
        &mut self,
        ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some((entry, state)) = self
            .entries
            .get(ix)
            .and_then(|entry| Some(entry).zip(self.variable_states.get(&entry.path)))
        else {
            debug_panic!("Trying to render entry in variable list that has an out of bounds index");
            return div().into_any_element();
        };

        self.render_variable(entry, *state, window, cx)
    }

    pub(crate) fn toggle_variable(&mut self, var_path: &VariablePath, cx: &mut Context<Self>) {
        let Some(entry) = self.variable_states.get_mut(var_path) else {
            debug_panic!("Trying to toggle variable in variable list that has an no state");
            return;
        };

        entry.is_expanded = !entry.is_expanded;
        self.build_entries(cx);
    }

    fn select_first(&mut self, _: &SelectFirst, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(variable) = self.entries.first() {
            self.selection = Some(variable.path.clone());
            cx.notify();
        }
    }

    fn select_last(&mut self, _: &SelectLast, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(variable) = self.entries.last() {
            self.selection = Some(variable.path.clone());
            cx.notify();
        }
    }

    fn select_prev(&mut self, _: &SelectPrevious, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(selection) = &self.selection {
            if let Some(var_ix) = self.entries.iter().enumerate().find_map(|(ix, var)| {
                if &var.path == selection {
                    Some(ix.saturating_sub(1))
                } else {
                    None
                }
            }) {
                if let Some(new_selection) = self.entries.get(var_ix).map(|var| var.path.clone()) {
                    self.selection = Some(new_selection);
                    cx.notify();
                } else {
                    self.select_first(&SelectFirst, window, cx);
                }
            }
        } else {
            self.select_first(&SelectFirst, window, cx);
        }
    }

    fn select_next(&mut self, _: &SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(selection) = &self.selection {
            if let Some(var_ix) = self.entries.iter().enumerate().find_map(|(ix, var)| {
                if &var.path == selection {
                    Some(ix.saturating_add(1))
                } else {
                    None
                }
            }) {
                if let Some(new_selection) = self.entries.get(var_ix).map(|var| var.path.clone()) {
                    self.selection = Some(new_selection);
                    cx.notify();
                } else {
                    self.select_first(&SelectFirst, window, cx);
                }
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
            let variables_reference = var_path.parent_reference_id();
            let name = var_path.leaf_name?;
            let value = editor.read(cx).text(cx);

            self.session.update(cx, |session, cx| {
                session.set_variable_value(variables_reference, name.into(), value, cx)
            });
            Some(())
        });

        if res.is_none() {
            log::error!("Couldn't confirm variable edit because variable doesn't have a leaf name or a parent reference id");
        }
    }

    fn collapse_selected_entry(
        &mut self,
        _: &CollapseSelectedEntry,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(ref selected_entry) = self.selection {
            let Some(entry_state) = self.variable_states.get_mut(selected_entry) else {
                debug_panic!("Trying to toggle variable in variable list that has an no state");
                return;
            };

            entry_state.is_expanded = false;
            self.build_entries(cx);
        }
    }

    fn expand_selected_entry(
        &mut self,
        _: &ExpandSelectedEntry,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(ref selected_entry) = self.selection {
            let Some(entry_state) = self.variable_states.get_mut(selected_entry) else {
                debug_panic!("Trying to toggle variable in variable list that has an no state");
                return;
            };

            entry_state.is_expanded = true;
            self.build_entries(cx);
        }
    }

    fn deploy_variable_context_menu(
        &mut self,
        variable: Variable,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let variable_value = variable.dap.value.clone();
        let this = cx.entity().clone();
        let context_menu = ContextMenu::build(window, cx, |menu, _, _| {
            menu.entry("Copy name", None, move |_, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(variable.dap.name.clone()))
            })
            .entry("Copy value", None, {
                let variable_value = variable_value.clone();
                move |_, cx| {
                    cx.write_to_clipboard(ClipboardItem::new_string(variable_value.clone()))
                }
            })
            .entry("Set value", None, move |window, cx| {
                this.update(cx, |variable_list, cx| {
                    let editor = cx.new(|cx| {
                        let mut editor = Editor::single_line(window, cx);
                        editor.set_text(variable_value.clone(), window, cx);
                        editor.select_all(&editor::actions::SelectAll, window, cx);
                        editor
                    });
                    editor.focus_handle(cx).focus(window);
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
    #[cfg(any(test, feature = "test-support"))]
    pub fn assert_visual_entries(&self, expected: Vec<&str>) {
        const INDENT: &'static str = "    ";

        let entries = &self.entries;
        let mut visual_entries = Vec::with_capacity(entries.len());
        for variable in entries {
            let state = self
                .variable_states
                .get(&variable.path)
                .expect("If there's a variable entry there has to be a state that goes with it");

            visual_entries.push(format!(
                "{}{} {}",
                INDENT.repeat(state.depth),
                if state.is_expanded { "v" } else { ">" },
                variable.dap.name,
            ));
        }

        pretty_assertions::assert_eq!(expected, visual_entries);
    }

    #[allow(clippy::too_many_arguments)]
    fn render_variable(
        &self,
        variable: &Variable,
        state: VariableState,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let var_ref = variable.dap.variables_reference;
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
                    variable.dap.name, state.depth
                )))
                .disabled(self.disabled)
                .selectable(false)
                .indent_level(state.depth as usize)
                .indent_step_size(px(20.))
                .always_show_disclosure_icon(true)
                .when(var_ref > 0, |list_item| {
                    list_item.toggle(state.is_expanded).on_toggle(cx.listener({
                        let var_path = variable.path.clone();
                        move |this, _, _window, cx| {
                            this.session.update(cx, |session, cx| {
                                session.variables(var_ref, cx);
                            });

                            this.toggle_variable(&var_path, cx);
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
                        .child(variable.dap.name.clone())
                        .when(!variable.dap.value.is_empty(), |this| {
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
                                            self.session
                                                .read(cx)
                                                .capabilities()
                                                .supports_set_variable
                                                .unwrap_or_default(),
                                            |this| {
                                                let path = variable.path.clone();
                                                let variable_value = variable.dap.value.clone();
                                                this.on_click(cx.listener(
                                                    move |this, click: &ClickEvent, window, cx| {
                                                        if click.down.click_count < 2 {
                                                            return;
                                                        }
                                                        let editor = cx.new(|cx| {
                                                            let mut editor =
                                                                Editor::single_line(window, cx);
                                                            editor.set_text(
                                                                variable_value.clone(),
                                                                window,
                                                                cx,
                                                            );
                                                            editor.select_all(
                                                                &editor::actions::SelectAll,
                                                                window,
                                                                cx,
                                                            );
                                                            editor
                                                        });
                                                        editor.focus_handle(cx).focus(window);
                                                        this.edited_path =
                                                            Some((path.clone(), editor));

                                                        cx.notify();
                                                    },
                                                ))
                                            },
                                        )
                                        .child(
                                            Label::new(&variable.dap.value)
                                                .single_line()
                                                .text_ellipsis()
                                                .size(LabelSize::XSmall)
                                                .color(Color::Muted),
                                        )
                                }
                            }))
                        }),
                ),
            )
            .into_any()
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
            .key_context("VariableList")
            .id("variable-list")
            .group("variable-list")
            .overflow_y_scroll()
            .size_full()
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::select_prev))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::expand_selected_entry))
            .on_action(cx.listener(Self::collapse_selected_entry))
            .on_action(cx.listener(Self::cancel_variable_edit))
            .on_action(cx.listener(Self::confirm_variable_edit))
            //
            .child(list(self.list.clone()).gap_1_5().size_full().flex_grow())
            .children(self.open_context_menu.as_ref().map(|(menu, position, _)| {
                deferred(
                    anchored()
                        .position(*position)
                        .anchor(gpui::Corner::TopLeft)
                        .child(menu.clone()),
                )
                .with_priority(1)
            }))
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
