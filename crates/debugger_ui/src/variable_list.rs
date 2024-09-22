use crate::debugger_panel::{ThreadState, VariableContainer};
use dap::{client::DebugAdapterClientId, Capabilities, Scope, Variable};
use editor::{
    actions::{self, SelectAll},
    Editor, EditorEvent,
};
use futures::future::try_join_all;
use gpui::{
    anchored, deferred, list, AnyElement, ClipboardItem, DismissEvent, FocusHandle, FocusableView,
    ListState, Model, MouseDownEvent, Point, Subscription, View,
};
use menu::Confirm;
use project::dap_store::DapStore;
use std::{collections::HashMap, sync::Arc};
use ui::{prelude::*, ContextMenu, ListItem};

#[derive(Debug, Clone)]
pub struct SetVariableState {
    name: String,
    scope: Scope,
    value: String,
    stack_frame_id: u64,
    evaluate_name: Option<String>,
    parent_variables_reference: u64,
}

#[derive(Debug, Clone)]
pub enum VariableListEntry {
    Scope(Scope),
    SetVariableEditor {
        depth: usize,
        state: SetVariableState,
    },
    Variable {
        depth: usize,
        scope: Scope,
        variable: Arc<Variable>,
        has_children: bool,
        container_reference: u64,
    },
}

pub struct VariableList {
    list: ListState,
    stack_frame_id: u64,
    dap_store: Model<DapStore>,
    focus_handle: FocusHandle,
    capabilities: Capabilities,
    client_id: DebugAdapterClientId,
    open_entries: Vec<SharedString>,
    thread_state: Model<ThreadState>,
    set_variable_editor: View<Editor>,
    set_variable_state: Option<SetVariableState>,
    entries: HashMap<u64, Vec<VariableListEntry>>,
    open_context_menu: Option<(View<ContextMenu>, Point<Pixels>, Subscription)>,
}

impl VariableList {
    pub fn new(
        dap_store: Model<DapStore>,
        client_id: &DebugAdapterClientId,
        thread_state: &Model<ThreadState>,
        capabilities: &Capabilities,
        stack_frame_id: u64,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let weakview = cx.view().downgrade();
        let focus_handle = cx.focus_handle();

        let list = ListState::new(0, gpui::ListAlignment::Top, px(1000.), move |ix, cx| {
            weakview
                .upgrade()
                .map(|view| view.update(cx, |this, cx| this.render_entry(ix, cx)))
                .unwrap_or(div().into_any())
        });

        let set_variable_editor = cx.new_view(Editor::single_line);

        cx.subscribe(
            &set_variable_editor,
            |this: &mut Self, _, event: &EditorEvent, cx| {
                if *event == EditorEvent::Blurred {
                    this.cancel_set_variable_value(cx);
                }
            },
        )
        .detach();

        Self {
            list,
            dap_store,
            focus_handle,
            stack_frame_id,
            set_variable_editor,
            client_id: *client_id,
            open_context_menu: None,
            set_variable_state: None,
            entries: Default::default(),
            open_entries: Default::default(),
            thread_state: thread_state.clone(),
            capabilities: capabilities.clone(),
        }
    }

    fn render_entry(&mut self, ix: usize, cx: &mut ViewContext<Self>) -> AnyElement {
        let Some(entries) = self.entries.get(&self.stack_frame_id) else {
            return div().into_any_element();
        };

        match &entries[ix] {
            VariableListEntry::Scope(scope) => self.render_scope(scope, cx),
            VariableListEntry::SetVariableEditor { depth, state } => {
                self.render_set_variable_editor(*depth, state, cx)
            }
            VariableListEntry::Variable {
                depth,
                scope,
                variable,
                has_children,
                container_reference: parent_variables_reference,
            } => self.render_variable(
                ix,
                *parent_variables_reference,
                variable,
                scope,
                *depth,
                *has_children,
                cx,
            ),
        }
    }

    fn toggle_entry_collapsed(&mut self, entry_id: &SharedString, cx: &mut ViewContext<Self>) {
        match self.open_entries.binary_search(&entry_id) {
            Ok(ix) => {
                self.open_entries.remove(ix);
            }
            Err(ix) => {
                self.open_entries.insert(ix, entry_id.clone());
            }
        };

        self.build_entries(false, true, cx);
    }

    pub fn update_stack_frame_id(&mut self, stack_frame_id: u64, cx: &mut ViewContext<Self>) {
        self.stack_frame_id = stack_frame_id;

        cx.notify();
    }

    pub fn build_entries(
        &mut self,
        open_first_scope: bool,
        keep_open_entries: bool,
        cx: &mut ViewContext<Self>,
    ) {
        let thread_state = self.thread_state.read(cx);

        let Some(scopes) = thread_state.scopes.get(&self.stack_frame_id) else {
            return;
        };

        if !keep_open_entries {
            self.open_entries.clear();
        }

        let mut entries: Vec<VariableListEntry> = Vec::default();
        for scope in scopes {
            let Some(variables) = thread_state.variables.get(&scope.variables_reference) else {
                continue;
            };

            if variables.is_empty() {
                continue;
            }

            if open_first_scope && entries.is_empty() {
                self.open_entries.push(scope_entry_id(scope));
            }
            entries.push(VariableListEntry::Scope(scope.clone()));

            if self
                .open_entries
                .binary_search(&scope_entry_id(scope))
                .is_err()
            {
                continue;
            }

            let mut depth_check: Option<usize> = None;

            for variable_container in variables {
                let depth = variable_container.depth;
                let variable = &variable_container.variable;
                let container_reference = variable_container.container_reference;

                if depth_check.is_some_and(|d| depth > d) {
                    continue;
                }

                if depth_check.is_some_and(|d| d >= depth) {
                    depth_check = None;
                }

                if self
                    .open_entries
                    .binary_search(&variable_entry_id(scope, variable, depth))
                    .is_err()
                {
                    if depth_check.is_none() || depth_check.is_some_and(|d| d > depth) {
                        depth_check = Some(depth);
                    }
                }

                if let Some(state) = self.set_variable_state.as_ref() {
                    if state.parent_variables_reference == container_reference
                        && state.name == variable.name
                    {
                        entries.push(VariableListEntry::SetVariableEditor {
                            depth,
                            state: state.clone(),
                        });
                    }
                }

                entries.push(VariableListEntry::Variable {
                    depth,
                    scope: scope.clone(),
                    variable: Arc::new(variable.clone()),
                    has_children: variable.variables_reference > 0,
                    container_reference,
                });
            }
        }

        let len = entries.len();
        self.entries.insert(self.stack_frame_id, entries);
        self.list.reset(len);

        cx.notify();
    }

    fn deploy_variable_context_menu(
        &mut self,
        parent_variables_reference: u64,
        scope: &Scope,
        variable: &Variable,
        position: Point<Pixels>,
        cx: &mut ViewContext<Self>,
    ) {
        let this = cx.view().clone();

        let stack_frame_id = self.stack_frame_id;
        let support_set_variable = self.capabilities.supports_set_variable.unwrap_or_default();

        let context_menu = ContextMenu::build(cx, |menu, cx| {
            menu.entry(
                "Copy name",
                None,
                cx.handler_for(&this, {
                    let variable = variable.clone();
                    move |_, cx| {
                        cx.write_to_clipboard(ClipboardItem::new_string(variable.name.clone()))
                    }
                }),
            )
            .entry(
                "Copy value",
                None,
                cx.handler_for(&this, {
                    let variable = variable.clone();
                    move |_, cx| {
                        cx.write_to_clipboard(ClipboardItem::new_string(variable.value.clone()))
                    }
                }),
            )
            .when(support_set_variable, move |menu| {
                let variable = variable.clone();
                let scope = scope.clone();

                menu.entry(
                    "Set value",
                    None,
                    cx.handler_for(&this, move |this, cx| {
                        this.set_variable_state = Some(SetVariableState {
                            parent_variables_reference,
                            name: variable.name.clone(),
                            scope: scope.clone(),
                            evaluate_name: variable.evaluate_name.clone(),
                            value: variable.value.clone(),
                            stack_frame_id,
                        });

                        this.set_variable_editor.update(cx, |editor, cx| {
                            editor.set_text(variable.value.clone(), cx);
                            editor.select_all(&SelectAll, cx);
                            editor.focus(cx);
                        });

                        this.build_entries(false, true, cx);
                    }),
                )
            })
        });

        cx.focus_view(&context_menu);
        let subscription =
            cx.subscribe(&context_menu, |this, _, _: &DismissEvent, cx| {
                if this.open_context_menu.as_ref().is_some_and(|context_menu| {
                    context_menu.0.focus_handle(cx).contains_focused(cx)
                }) {
                    cx.focus_self();
                }
                this.open_context_menu.take();
                cx.notify();
            });

        self.open_context_menu = Some((context_menu, position, subscription));
    }

    fn cancel_set_variable_value(&mut self, cx: &mut ViewContext<Self>) {
        if self.set_variable_state.take().is_none() {
            return;
        };

        self.build_entries(false, true, cx);
    }

    fn set_variable_value(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        let new_variable_value = self.set_variable_editor.update(cx, |editor, cx| {
            let new_variable_value = editor.text(cx);

            editor.clear(cx);

            new_variable_value
        });

        let Some(state) = self.set_variable_state.take() else {
            return cx.notify();
        };

        if new_variable_value == state.value || state.stack_frame_id != self.stack_frame_id {
            return cx.notify();
        }

        let client_id = self.client_id;
        let variables_reference = state.parent_variables_reference;
        let scope = state.scope;
        let name = state.name;
        let evaluate_name = state.evaluate_name;
        let stack_frame_id = state.stack_frame_id;

        cx.spawn(|this, mut cx| async move {
            let set_value_task = this.update(&mut cx, |this, cx| {
                this.dap_store.update(cx, |store, cx| {
                    store.set_variable_value(
                        &client_id,
                        stack_frame_id,
                        variables_reference,
                        name,
                        new_variable_value,
                        evaluate_name,
                        cx,
                    )
                })
            });

            set_value_task?.await?;

            let Some(scope_variables) = this.update(&mut cx, |this, cx| {
                this.thread_state.update(cx, |thread_state, _| {
                    thread_state.variables.remove(&scope.variables_reference)
                })
            })?
            else {
                return Ok(());
            };

            let tasks = this.update(&mut cx, |this, cx| {
                let mut tasks = Vec::new();

                for variable_container in scope_variables {
                    let fetch_variables_task = this.dap_store.update(cx, |store, cx| {
                        store.variables(&client_id, variable_container.container_reference, cx)
                    });

                    tasks.push(async move {
                        let depth = variable_container.depth;
                        let container_reference = variable_container.container_reference;

                        anyhow::Ok(
                            fetch_variables_task
                                .await?
                                .into_iter()
                                .map(move |variable| VariableContainer {
                                    container_reference,
                                    variable,
                                    depth,
                                })
                                .collect::<Vec<_>>(),
                        )
                    });
                }

                tasks
            })?;

            let updated_variables = try_join_all(tasks).await?;

            this.update(&mut cx, |this, cx| {
                this.thread_state.update(cx, |thread_state, cx| {
                    for variables in updated_variables {
                        thread_state
                            .variables
                            .insert(scope.variables_reference, variables);
                    }

                    cx.notify();
                });

                this.build_entries(false, true, cx);
            })
        })
        .detach_and_log_err(cx);
    }

    fn render_set_variable_editor(
        &self,
        depth: usize,
        state: &SetVariableState,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement {
        div()
            .h_4()
            .size_full()
            .on_action(cx.listener(Self::set_variable_value))
            .child(
                ListItem::new(SharedString::from(state.name.clone()))
                    .indent_level(depth + 1)
                    .indent_step_size(px(20.))
                    .child(self.set_variable_editor.clone()),
            )
            .into_any_element()
    }

    fn on_toggle_variable(
        &mut self,
        ix: usize,
        variable_id: &SharedString,
        variable_reference: u64,
        has_children: bool,
        disclosed: Option<bool>,
        cx: &mut ViewContext<Self>,
    ) {
        if !has_children {
            return;
        }

        // if we already opened the variable/we already fetched it
        // we can just toggle it because we already have the nested variable
        if disclosed.unwrap_or(true)
            || self
                .thread_state
                .read(cx)
                .fetched_variable_ids
                .contains(&variable_reference)
        {
            return self.toggle_entry_collapsed(&variable_id, cx);
        }

        let Some(entries) = self.entries.get(&self.stack_frame_id) else {
            return;
        };

        let Some(entry) = entries.get(ix) else {
            return;
        };

        if let VariableListEntry::Variable { scope, depth, .. } = entry {
            let variable_id = variable_id.clone();
            let scope = scope.clone();
            let depth = *depth;

            let fetch_variables_task = self.dap_store.update(cx, |store, cx| {
                store.variables(&self.client_id, variable_reference, cx)
            });

            cx.spawn(|this, mut cx| async move {
                let new_variables = fetch_variables_task.await?;

                this.update(&mut cx, |this, cx| {
                    this.thread_state.update(cx, |thread_state, cx| {
                        let Some(variables) =
                            thread_state.variables.get_mut(&scope.variables_reference)
                        else {
                            return;
                        };

                        let position = variables.iter().position(|v| {
                            variable_entry_id(&scope, &v.variable, v.depth) == variable_id
                        });

                        if let Some(position) = position {
                            variables.splice(
                                position + 1..position + 1,
                                new_variables.clone().into_iter().map(|variable| {
                                    VariableContainer {
                                        container_reference: variable_reference,
                                        variable,
                                        depth: depth + 1,
                                    }
                                }),
                            );

                            thread_state.fetched_variable_ids.insert(variable_reference);
                        }

                        cx.notify();
                    });

                    this.toggle_entry_collapsed(&variable_id, cx);
                })
            })
            .detach_and_log_err(cx);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_variable(
        &self,
        ix: usize,
        parent_variables_reference: u64,
        variable: &Variable,
        scope: &Scope,
        depth: usize,
        has_children: bool,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement {
        let variable_reference = variable.variables_reference;
        let variable_id = variable_entry_id(scope, variable, depth);

        let disclosed = has_children.then(|| {
            self.open_entries
                .binary_search(&variable_entry_id(scope, variable, depth))
                .is_ok()
        });

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
                        this.on_toggle_variable(
                            ix,
                            &variable_id,
                            variable_reference,
                            has_children,
                            disclosed,
                            cx,
                        )
                    }))
                    .on_secondary_mouse_down(cx.listener({
                        let scope = scope.clone();
                        let variable = variable.clone();
                        move |this, event: &MouseDownEvent, cx| {
                            this.deploy_variable_context_menu(
                                parent_variables_reference,
                                &scope,
                                &variable,
                                event.position,
                                cx,
                            )
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

impl FocusableView for VariableList {
    fn focus_handle(&self, _: &gpui::AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for VariableList {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .on_action(
                cx.listener(|this, _: &actions::Cancel, cx| this.cancel_set_variable_value(cx)),
            )
            .child(list(self.list.clone()).gap_1_5().size_full())
            .children(self.open_context_menu.as_ref().map(|(menu, position, _)| {
                deferred(
                    anchored()
                        .position(*position)
                        .anchor(gpui::AnchorCorner::TopLeft)
                        .child(menu.clone()),
                )
                .with_priority(1)
            }))
    }
}

pub fn variable_entry_id(scope: &Scope, variable: &Variable, depth: usize) -> SharedString {
    SharedString::from(format!(
        "variable-{}-{}-{}",
        scope.variables_reference, variable.name, depth
    ))
}

fn scope_entry_id(scope: &Scope) -> SharedString {
    SharedString::from(format!("scope-{}", scope.variables_reference))
}
