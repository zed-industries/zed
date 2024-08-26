use crate::debugger_panel_item::{DebugPanelItem, DebugPanelItemEvent, ThreadEntry};
use dap::{client::ThreadState, Scope, Variable};

use gpui::{list, AnyElement, ListState, Model, Subscription};
use ui::{prelude::*, ListItem};

use std::{collections::HashMap, sync::Arc};

pub struct VariableList {
    pub list: ListState,
    debug_panel_item: Model<DebugPanelItem>,
    open_entries: Vec<SharedString>,
    stack_frame_entries: HashMap<u64, Vec<ThreadEntry>>,
    _subscriptions: Vec<Subscription>,
}

impl VariableList {
    pub fn new(debug_panel_item: Model<DebugPanelItem>, cx: &mut ViewContext<Self>) -> Self {
        let weakview = cx.view().downgrade();

        let list = ListState::new(0, gpui::ListAlignment::Top, px(1000.), move |ix, cx| {
            weakview
                .upgrade()
                .map(|view| view.update(cx, |this, cx| this.render_entry(ix, cx)))
                .unwrap_or(div().into_any())
        });

        let _subscriptions = vec![cx.subscribe(&debug_panel_item, Self::handle_events)];

        Self {
            list,
            debug_panel_item,
            open_entries: Default::default(),
            stack_frame_entries: Default::default(),
            _subscriptions,
        }
    }

    fn render_entry(&mut self, ix: usize, cx: &mut ViewContext<Self>) -> AnyElement {
        let debug_item = self.debug_panel_item.read(cx);
        let Some(entries) = self
            .stack_frame_entries
            .get(&debug_item.current_thread_state().current_stack_frame_id)
        else {
            return div().into_any_element();
        };

        match &entries[ix] {
            ThreadEntry::Scope(scope) => self.render_scope(scope, cx),
            ThreadEntry::Variable {
                depth,
                scope,
                variable,
                has_children,
                ..
            } => self.render_variable(ix, variable, scope, *depth, *has_children, cx),
        }
    }

    fn handle_events(
        &mut self,
        _debug_panel_item: Model<DebugPanelItem>,
        _event: &DebugPanelItemEvent,
        _cx: &mut ViewContext<Self>,
    ) {
    }

    pub fn toggle_entry_collapsed(&mut self, entry_id: &SharedString, cx: &mut ViewContext<Self>) {
        match self.open_entries.binary_search(&entry_id) {
            Ok(ix) => {
                self.open_entries.remove(ix);
            }
            Err(ix) => {
                self.open_entries.insert(ix, entry_id.clone());
            }
        };

        let thread_state = self
            .debug_panel_item
            .read_with(cx, |panel, _cx| panel.current_thread_state());

        self.build_entries(thread_state, false, cx);
        cx.notify();
    }

    pub fn build_entries(
        &mut self,
        thread_state: ThreadState,
        open_first_scope: bool,
        _cx: &mut ViewContext<Self>,
    ) {
        let stack_frame_id = thread_state.current_stack_frame_id;
        let Some(scopes_and_vars) = thread_state.variables.get(&stack_frame_id) else {
            return;
        };

        let mut entries: Vec<ThreadEntry> = Vec::default();
        for (scope, variables) in scopes_and_vars {
            if variables.is_empty() {
                continue;
            }

            if open_first_scope && self.open_entries.is_empty() {
                self.open_entries.push(scope_entry_id(scope));
            }

            entries.push(ThreadEntry::Scope(scope.clone()));

            if self
                .open_entries
                .binary_search(&scope_entry_id(scope))
                .is_err()
            {
                continue;
            }

            let mut depth_check: Option<usize> = None;

            for (depth, variable) in variables {
                if depth_check.is_some_and(|d| *depth > d) {
                    continue;
                }

                if depth_check.is_some_and(|d| d >= *depth) {
                    depth_check = None;
                }

                let has_children = variable.variables_reference > 0;

                if self
                    .open_entries
                    .binary_search(&variable_entry_id(&variable, &scope, *depth))
                    .is_err()
                {
                    if depth_check.is_none() || depth_check.is_some_and(|d| d > *depth) {
                        depth_check = Some(*depth);
                    }
                }

                entries.push(ThreadEntry::Variable {
                    has_children,
                    depth: *depth,
                    scope: scope.clone(),
                    variable: Arc::new(variable.clone()),
                });
            }
        }

        let len = entries.len();
        self.stack_frame_entries.insert(stack_frame_id, entries);
        self.list.reset(len);
    }

    pub fn render_variable(
        &self,
        ix: usize,
        variable: &Variable,
        scope: &Scope,
        depth: usize,
        has_children: bool,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement {
        let variable_reference = variable.variables_reference;
        let variable_id = variable_entry_id(variable, scope, depth);

        let disclosed = has_children.then(|| self.open_entries.binary_search(&variable_id).is_ok());

        div()
            .id(variable_id.clone())
            .group("")
            .h_4()
            .size_full()
            .child(
                ListItem::new(variable_id.clone())
                    .indent_level(depth + 1)
                    .indent_step_size(px(20.))
                    .always_show_disclosure_icon(true)
                    .toggle(disclosed)
                    .on_toggle(cx.listener(move |this, _, cx| {
                        if !has_children {
                            return;
                        }

                        let debug_item = this.debug_panel_item.read(cx);

                        // if we already opend the variable/we already fetched it
                        // we can just toggle it because we already have the nested variable
                        if disclosed.unwrap_or(true)
                            || debug_item
                                .current_thread_state()
                                .vars
                                .contains_key(&variable_reference)
                        {
                            return this.toggle_entry_collapsed(&variable_id, cx);
                        }

                        let Some(entries) = this
                            .stack_frame_entries
                            .get(&debug_item.current_thread_state().current_stack_frame_id)
                        else {
                            return;
                        };

                        let Some(entry) = entries.get(ix) else {
                            return;
                        };

                        if let ThreadEntry::Variable { scope, depth, .. } = entry {
                            let variable_id = variable_id.clone();
                            let client = debug_item.client();
                            let scope = scope.clone();
                            let depth = *depth;

                            cx.spawn(|this, mut cx| async move {
                                let variables = client.variables(variable_reference).await?;

                                this.update(&mut cx, |this, cx| {
                                    let client = client.clone();
                                    let mut thread_states = client.thread_states();
                                    let Some(thread_state) = thread_states
                                        .get_mut(&this.debug_panel_item.read(cx).thread_id())
                                    else {
                                        return;
                                    };

                                    if let Some(state) = thread_state
                                        .variables
                                        .get_mut(&thread_state.current_stack_frame_id)
                                        .and_then(|s| s.get_mut(&scope))
                                    {
                                        let position = state.iter().position(|(d, v)| {
                                            variable_entry_id(v, &scope, *d) == variable_id
                                        });

                                        if let Some(position) = position {
                                            state.splice(
                                                position + 1..position + 1,
                                                variables
                                                    .clone()
                                                    .into_iter()
                                                    .map(|v| (depth + 1, v)),
                                            );
                                        }

                                        thread_state.vars.insert(variable_reference, variables);
                                    }

                                    drop(thread_states);
                                    this.toggle_entry_collapsed(&variable_id, cx);
                                })
                            })
                            .detach_and_log_err(cx);
                        }
                    }))
                    .child(
                        h_flex()
                            .gap_1()
                            .text_ui_sm(cx)
                            .child(variable.name.clone())
                            .child(
                                div()
                                    .text_ui_xs(cx)
                                    .text_color(cx.theme().colors().text_muted)
                                    .child(variable.value.clone()),
                            ),
                    ),
            )
            .into_any()
    }

    fn render_scope(&self, scope: &Scope, cx: &mut ViewContext<Self>) -> AnyElement {
        let element_id = scope.variables_reference;

        let scope_id = scope_entry_id(scope);
        let disclosed = self.open_entries.binary_search(&scope_id).is_ok();

        div()
            .id(element_id as usize)
            .group("")
            .flex()
            .w_full()
            .h_full()
            .child(
                ListItem::new(scope_id.clone())
                    .indent_level(1)
                    .indent_step_size(px(20.))
                    .always_show_disclosure_icon(true)
                    .toggle(disclosed)
                    .on_toggle(
                        cx.listener(move |this, _, cx| this.toggle_entry_collapsed(&scope_id, cx)),
                    )
                    .child(div().text_ui(cx).w_full().child(scope.name.clone())),
            )
            .into_any()
    }
}

impl Render for VariableList {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        list(self.list.clone()).gap_1_5().size_full()
    }
}

pub fn variable_entry_id(variable: &Variable, scope: &Scope, depth: usize) -> SharedString {
    SharedString::from(format!(
        "variable-{}-{}-{}",
        depth, scope.variables_reference, variable.name
    ))
}

fn scope_entry_id(scope: &Scope) -> SharedString {
    SharedString::from(format!("scope-{}", scope.variables_reference))
}
