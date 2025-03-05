use super::stack_frame_list::{StackFrameList, StackFrameListEvent};
use dap::{StackFrameId, VariableReference};
use editor::{Editor, EditorEvent};
use gpui::{
    actions, anchored, deferred, list, AnyElement, Context, Entity, FocusHandle, Focusable, Hsla,
    ListState, MouseDownEvent, Point, Subscription,
};
use menu::{SelectFirst, SelectLast, SelectNext, SelectPrevious};
use project::debugger::session::{Session, SessionEvent};
use std::{collections::HashMap, sync::Arc};
use ui::{prelude::*, ContextMenu, ListItem};
use util::debug_panic;

actions!(variable_list, [ExpandSelectedEntry, CollapseSelectedEntry]);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct VariableState {
    depth: usize,
    is_expanded: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct VariablePath {
    pub indices: Arc<[VariableReference]>,
}

#[derive(Debug)]
struct Variable {
    dap: dap::Variable,
    path: VariablePath,
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
            move |ix, _window, cx| {
                weak_variable_list
                    .upgrade()
                    .map(|var_list| var_list.update(cx, |this, cx| this.render_entry(ix, cx)))
                    .unwrap_or(div().into_any())
            },
        );

        let set_variable_editor = cx.new(|cx| Editor::single_line(window, cx));

        cx.subscribe(
            &set_variable_editor,
            |_this: &mut Self, _, event: &EditorEvent, _cx| {
                if *event == EditorEvent::Blurred {
                    // this.cancel_set_variable_value(cx);
                }
            },
        )
        .detach();

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
                    None,
                    vec![scope.variables_reference],
                )
            })
            .collect::<Vec<_>>();

        while let Some((variable_reference, dap, indices)) = stack.pop() {
            let path = VariablePath {
                indices: indices.clone().into(),
            };

            let var_state = self
                .variable_states
                .entry(path.clone())
                .or_insert(VariableState {
                    depth: path.indices.len(),
                    is_expanded: dap.is_none(),
                });

            if let Some(dap) = dap {
                entries.push(Variable { dap, path });
            }

            if var_state.is_expanded {
                let children = self
                    .session
                    .update(cx, |session, cx| session.variables(variable_reference, cx));
                stack.extend(children.into_iter().rev().map(|child| {
                    let mut indices = indices.clone();
                    indices.push(child.variables_reference);
                    (child.variables_reference, Some(child), indices)
                }));
            }
        }
        self.entries = entries;
        self.list.reset(self.entries.len());
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

    fn render_entry(&mut self, ix: usize, cx: &mut Context<Self>) -> AnyElement {
        let Some((entry, state)) = self
            .entries
            .get(ix)
            .and_then(|entry| Some(entry).zip(self.variable_states.get(&entry.path)))
        else {
            debug_panic!("Trying to render entry in variable list that has an out of bounds index");
            return div().into_any_element();
        };

        // todo(debugger) pass a valid value for is selected

        self.render_variable(entry, *state, cx)
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

    #[track_caller]
    #[cfg(any(test, feature = "test-support"))]
    pub fn assert_visual_entries(&self, expected: Vec<&str>) {
        // TODO(debugger): Implement this method
        // const INDENT: &'static str = "    ";

        // let entries = &self.entries;
        // let mut visual_entries = Vec::with_capacity(entries.len());
        // for entry in entries {
        //     match entry {
        //         VariableListEntry::Scope((scope, state)) => {
        //             visual_entries.push(format!(
        //                 "{} {}",
        //                 if state.is_expanded { "v" } else { ">" },
        //                 scope.name,
        //             ));
        //         }
        //         // TODO(debugger): make this work again
        //         // VariableListEntry::SetVariableEditor { depth, state } => {
        //         //     visual_entries.push(format!(
        //         //         "{}  [EDITOR: {}]{}",
        //         //         INDENT.repeat(*depth),
        //         //         state.name,
        //         //         if is_selected { " <=== selected" } else { "" }
        //         //     ));
        //         // }
        //         VariableListEntry::Variable((variable, _, state)) => {
        //             visual_entries.push(format!(
        //                 "{}{} {}",
        //                 INDENT.repeat(state.depth),
        //                 if state.is_expanded { "v" } else { ">" },
        //                 variable.name,
        //             ));
        //         }
        //     };
        // }

        // pretty_assertions::assert_eq!(expected, visual_entries);
    }

    #[allow(clippy::too_many_arguments)]
    fn render_variable(
        &self,
        variable: &Variable,
        state: VariableState,
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
        let border_color = if is_selected {
            colors.marked_active
        } else {
            colors.default
        };

        div()
            .id(SharedString::from(format!(
                "variable-{}-{}",
                variable.dap.name, state.depth
            )))
            .group("variable_list_entry")
            .border_1()
            .border_r_2()
            .border_color(border_color)
            .h_4()
            .size_full()
            .hover(|style| style.bg(bg_hover_color))
            .on_click(cx.listener({
                // let scope = scope.clone();
                // let variable = variable.clone();
                move |_this, _, _window, _cx| {
                    // this.selection = Some(VariableListEntry::Variable {
                    //     depth,
                    //     has_children,
                    //     container_reference,
                    //     scope: scope.clone(),
                    //     variable: variable.clone(),
                    // });
                    // cx.notify();
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
                    // let scope = scope.clone();
                    // let variable = variable.clone();
                    move |_this, _event: &MouseDownEvent, _window, _cx| {

                        // todo(debugger): Get this working
                        // this.deploy_variable_context_menu(
                        //     container_reference,
                        //     &scope,
                        //     &variable,
                        //     event.position,
                        //     window,
                        //     cx,
                        // )
                    }
                }))
                .child(
                    h_flex()
                        .gap_1()
                        .text_ui_sm(cx)
                        .child(variable.dap.name.clone())
                        .child(
                            div()
                                .text_ui_xs(cx)
                                .text_color(cx.theme().colors().text_muted)
                                .child(variable.dap.value.replace("\n", " ").clone()),
                        ),
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
            .size_full()
            .overflow_y_scroll()
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::select_prev))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::expand_selected_entry))
            .on_action(cx.listener(Self::collapse_selected_entry))
            //
            .on_action(
                cx.listener(|_this, _: &editor::actions::Cancel, _window, _cx| {

                    // this.cancel_set_variable_value(cx)
                }),
            )
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
