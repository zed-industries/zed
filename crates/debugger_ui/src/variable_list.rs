use crate::stack_frame_list::{StackFrameList, StackFrameListEvent};
use anyhow::Result;
use dap::{client::DebugAdapterClientId, Scope, Variable};
use editor::{
    actions::{self, SelectAll},
    Editor, EditorEvent,
};
use futures::future::try_join_all;
use gpui::{
    anchored, deferred, list, AnyElement, ClipboardItem, DismissEvent, FocusHandle, FocusableView,
    ListState, Model, MouseDownEvent, Point, Subscription, Task, View,
};
use menu::Confirm;
use project::dap_store::DapStore;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    sync::Arc,
};
use ui::{prelude::*, ContextMenu, ListItem};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableContainer {
    pub container_reference: u64,
    pub variable: Variable,
    pub depth: usize,
}

#[derive(Debug, Clone)]
pub struct SetVariableState {
    name: String,
    scope: Scope,
    value: String,
    stack_frame_id: u64,
    evaluate_name: Option<String>,
    parent_variables_reference: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum OpenEntry {
    Scope { name: String },
    Variable { name: String, depth: usize },
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
        scope: Arc<Scope>,
        variable: Arc<Variable>,
        has_children: bool,
        container_reference: u64,
    },
}

#[derive(Debug)]
struct ScopeVariableIndex {
    fetched_ids: HashSet<u64>,
    variables: Vec<VariableContainer>,
}

impl ScopeVariableIndex {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
            fetched_ids: HashSet::default(),
        }
    }

    pub fn fetched(&self, container_reference: &u64) -> bool {
        self.fetched_ids.contains(container_reference)
    }

    /// All the variables should have the same depth and the same container reference
    pub fn add_variables(&mut self, container_reference: u64, variables: Vec<VariableContainer>) {
        let position = self
            .variables
            .iter()
            .position(|v| v.variable.variables_reference == container_reference);

        self.fetched_ids.insert(container_reference);

        if let Some(position) = position {
            self.variables.splice(position + 1..=position, variables);
        } else {
            self.variables.extend(variables);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }

    pub fn variables(&self) -> &[VariableContainer] {
        &self.variables
    }
}

pub struct VariableList {
    list: ListState,
    focus_handle: FocusHandle,
    dap_store: Model<DapStore>,
    open_entries: Vec<OpenEntry>,
    client_id: DebugAdapterClientId,
    scopes: HashMap<u64, Vec<Scope>>,
    set_variable_editor: View<Editor>,
    _subscriptions: Vec<Subscription>,
    stack_frame_list: View<StackFrameList>,
    set_variable_state: Option<SetVariableState>,
    entries: HashMap<u64, Vec<VariableListEntry>>,
    fetch_variables_task: Option<Task<Result<()>>>,
    // (stack_frame_id, scope_id) -> VariableIndex
    variables: BTreeMap<(u64, u64), ScopeVariableIndex>,
    open_context_menu: Option<(View<ContextMenu>, Point<Pixels>, Subscription)>,
}

impl VariableList {
    pub fn new(
        stack_frame_list: &View<StackFrameList>,
        dap_store: Model<DapStore>,
        client_id: &DebugAdapterClientId,
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

        let _subscriptions =
            vec![cx.subscribe(stack_frame_list, Self::handle_stack_frame_list_events)];

        Self {
            list,
            dap_store,
            focus_handle,
            _subscriptions,
            set_variable_editor,
            client_id: *client_id,
            open_context_menu: None,
            set_variable_state: None,
            fetch_variables_task: None,
            scopes: Default::default(),
            entries: Default::default(),
            variables: Default::default(),
            open_entries: Default::default(),
            stack_frame_list: stack_frame_list.clone(),
        }
    }

    fn handle_stack_frame_list_events(
        &mut self,
        _: View<StackFrameList>,
        event: &StackFrameListEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            StackFrameListEvent::SelectedStackFrameChanged => {
                self.build_entries(true, true, cx);
            }
            StackFrameListEvent::StackFramesUpdated => {
                self.entries.clear();

                self.fetch_variables(cx);
            }
        }
    }

    pub fn variables(&self, cx: &mut ViewContext<Self>) -> Vec<VariableContainer> {
        let stack_frame_id = self.stack_frame_list.read(cx).current_stack_frame_id();

        self.variables
            .range((stack_frame_id, u64::MIN)..(stack_frame_id, u64::MAX))
            .flat_map(|(_, containers)| containers.variables.iter().cloned())
            .collect()
    }

    fn render_entry(&mut self, ix: usize, cx: &mut ViewContext<Self>) -> AnyElement {
        let stack_frame_id = self.stack_frame_list.read(cx).current_stack_frame_id();

        let Some(entries) = self.entries.get(&stack_frame_id) else {
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
                container_reference,
            } => self.render_variable(
                *container_reference,
                variable,
                scope,
                *depth,
                *has_children,
                cx,
            ),
        }
    }

    fn toggle_entry(&mut self, entry_id: &OpenEntry, cx: &mut ViewContext<Self>) {
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

    pub fn build_entries(
        &mut self,
        open_first_scope: bool,
        keep_open_entries: bool,
        cx: &mut ViewContext<Self>,
    ) {
        let stack_frame_id = self.stack_frame_list.read(cx).current_stack_frame_id();

        let Some(scopes) = self.scopes.get(&stack_frame_id) else {
            return;
        };

        if !keep_open_entries {
            self.open_entries.clear();
        }

        let mut entries: Vec<VariableListEntry> = Vec::default();
        for scope in scopes {
            let Some(index) = self
                .variables
                .get(&(stack_frame_id, scope.variables_reference))
            else {
                continue;
            };

            if index.is_empty() {
                continue;
            }

            let scope_open_entry_id = OpenEntry::Scope {
                name: scope.name.clone(),
            };

            if open_first_scope
                && entries.is_empty()
                && self
                    .open_entries
                    .binary_search(&scope_open_entry_id)
                    .is_err()
            {
                self.open_entries.push(scope_open_entry_id.clone());
            }
            entries.push(VariableListEntry::Scope(scope.clone()));

            if self
                .open_entries
                .binary_search(&scope_open_entry_id)
                .is_err()
            {
                continue;
            }

            let mut depth_check: Option<usize> = None;

            for variable_container in index.variables().iter() {
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
                    .binary_search(&OpenEntry::Variable {
                        name: variable.name.clone(),
                        depth,
                    })
                    .is_err()
                {
                    if depth_check.is_none() || depth_check.is_some_and(|d| d > depth) {
                        depth_check = Some(depth);
                    }
                }

                if let Some(state) = self.set_variable_state.as_ref() {
                    if state.parent_variables_reference == container_reference
                        && state.scope.variables_reference == scope.variables_reference
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
                    scope: Arc::new(scope.clone()),
                    variable: Arc::new(variable.clone()),
                    has_children: variable.variables_reference > 0,
                    container_reference,
                });
            }
        }

        let len = entries.len();
        self.entries.insert(stack_frame_id, entries);
        self.list.reset(len);

        cx.notify();
    }

    fn fetch_nested_variables(
        &self,
        variables_reference: u64,
        depth: usize,
        open_entries: &Vec<OpenEntry>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<Vec<VariableContainer>>> {
        cx.spawn({
            let open_entries = open_entries.clone();
            |this, mut cx| async move {
                let variables_task = this.update(&mut cx, |this, cx| {
                    this.dap_store.update(cx, |store, cx| {
                        store.variables(&this.client_id, variables_reference, cx)
                    })
                })?;

                let mut variables = Vec::new();

                for variable in variables_task.await? {
                    variables.push(VariableContainer {
                        variable: variable.clone(),
                        container_reference: variables_reference,
                        depth,
                    });

                    if open_entries
                        .binary_search(&&OpenEntry::Variable {
                            name: variable.name.clone(),
                            depth,
                        })
                        .is_ok()
                    {
                        let task = this.update(&mut cx, |this, cx| {
                            this.fetch_nested_variables(
                                variable.variables_reference,
                                depth + 1,
                                &open_entries,
                                cx,
                            )
                        })?;

                        variables.extend(task.await?);
                    }
                }

                anyhow::Ok(variables)
            }
        })
    }

    fn fetch_variables_for_stack_frame(
        &self,
        stack_frame_id: u64,
        open_entries: &Vec<OpenEntry>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<(Vec<Scope>, HashMap<u64, Vec<VariableContainer>>)>> {
        let scopes_task = self.dap_store.update(cx, |store, cx| {
            store.scopes(&self.client_id, stack_frame_id, cx)
        });

        cx.spawn({
            let open_entries = open_entries.clone();
            |this, mut cx| async move {
                let mut variables = HashMap::new();

                let scopes = scopes_task.await?;

                for scope in scopes.iter() {
                    let variables_task = this.update(&mut cx, |this, cx| {
                        this.fetch_nested_variables(scope.variables_reference, 1, &open_entries, cx)
                    })?;

                    variables.insert(scope.variables_reference, variables_task.await?);
                }

                Ok((scopes, variables))
            }
        })
    }

    fn fetch_variables(&mut self, cx: &mut ViewContext<Self>) {
        let stack_frames = self.stack_frame_list.read(cx).stack_frames().clone();

        self.fetch_variables_task = Some(cx.spawn(|this, mut cx| async move {
            let mut tasks = Vec::with_capacity(stack_frames.len());

            let open_entries = this.update(&mut cx, |this, _| {
                this.open_entries
                    .iter()
                    .filter(|e| matches!(e, OpenEntry::Variable { .. }))
                    .cloned()
                    .collect::<Vec<_>>()
            })?;

            for stack_frame in stack_frames.clone().into_iter() {
                let task = this.update(&mut cx, |this, cx| {
                    this.fetch_variables_for_stack_frame(stack_frame.id, &open_entries, cx)
                });

                tasks.push(
                    cx.background_executor()
                        .spawn(async move { anyhow::Ok((stack_frame.id, task?.await?)) }),
                );
            }

            let result = try_join_all(tasks).await?;

            this.update(&mut cx, |this, cx| {
                let mut new_variables = BTreeMap::new();
                let mut new_scopes = HashMap::new();

                for (stack_frame_id, (scopes, variables)) in result {
                    new_scopes.insert(stack_frame_id, scopes);

                    for (scope_id, variables) in variables.into_iter() {
                        let mut variable_index = ScopeVariableIndex::new();
                        variable_index.add_variables(scope_id, variables);

                        new_variables.insert((stack_frame_id, scope_id), variable_index);
                    }
                }

                std::mem::swap(&mut this.variables, &mut new_variables);
                std::mem::swap(&mut this.scopes, &mut new_scopes);

                this.build_entries(true, true, cx);

                this.fetch_variables_task.take();
            })
        }));
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

        let support_set_variable = self.dap_store.read_with(cx, |store, _| {
            store
                .capabilities_by_id(&self.client_id)
                .supports_set_variable
                .unwrap_or_default()
        });

        let context_menu = ContextMenu::build(cx, |menu, cx| {
            menu.entry(
                "Copy name",
                None,
                cx.handler_for(&this, {
                    let variable_name = variable.name.clone();
                    move |_, cx| {
                        cx.write_to_clipboard(ClipboardItem::new_string(variable_name.clone()))
                    }
                }),
            )
            .entry(
                "Copy value",
                None,
                cx.handler_for(&this, {
                    let variable_value = variable.value.clone();
                    move |_, cx| {
                        cx.write_to_clipboard(ClipboardItem::new_string(variable_value.clone()))
                    }
                }),
            )
            .when_some(
                variable.memory_reference.clone(),
                |menu, memory_reference| {
                    menu.entry(
                        "Copy memory reference",
                        None,
                        cx.handler_for(&this, move |_, cx| {
                            cx.write_to_clipboard(ClipboardItem::new_string(
                                memory_reference.clone(),
                            ))
                        }),
                    )
                },
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
                            stack_frame_id: this.stack_frame_list.read(cx).current_stack_frame_id(),
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
            return;
        };

        if new_variable_value == state.value
            || state.stack_frame_id != self.stack_frame_list.read(cx).current_stack_frame_id()
        {
            return cx.notify();
        }

        let client_id = self.client_id;
        let variables_reference = state.parent_variables_reference;
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

            this.update(&mut cx, |this, cx| {
                this.build_entries(false, true, cx);
                this.invalidate(cx);
            })
        })
        .detach_and_log_err(cx);
    }

    pub fn invalidate(&mut self, cx: &mut ViewContext<Self>) {
        self.stack_frame_list.update(cx, |stack_frame_list, cx| {
            stack_frame_list.invalidate(cx);
        });
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

    #[allow(clippy::too_many_arguments)]
    fn on_toggle_variable(
        &mut self,
        scope_id: u64,
        entry_id: &OpenEntry,
        variable_reference: u64,
        depth: usize,
        has_children: bool,
        disclosed: Option<bool>,
        cx: &mut ViewContext<Self>,
    ) {
        if !has_children {
            return;
        }

        let stack_frame_id = self.stack_frame_list.read(cx).current_stack_frame_id();

        let Some(index) = self.variables.get(&(stack_frame_id, scope_id)) else {
            return;
        };

        // if we already opened the variable/we already fetched it
        // we can just toggle it because we already have the nested variable
        if disclosed.unwrap_or(true) || index.fetched(&variable_reference) {
            return self.toggle_entry(&entry_id, cx);
        }

        let fetch_variables_task = self.dap_store.update(cx, |store, cx| {
            store.variables(&self.client_id, variable_reference, cx)
        });

        let entry_id = entry_id.clone();
        cx.spawn(|this, mut cx| async move {
            let new_variables = fetch_variables_task.await?;

            this.update(&mut cx, |this, cx| {
                let Some(index) = this.variables.get_mut(&(stack_frame_id, scope_id)) else {
                    return;
                };

                index.add_variables(
                    variable_reference,
                    new_variables
                        .into_iter()
                        .map(|variable| VariableContainer {
                            container_reference: variable_reference,
                            variable,
                            depth: depth + 1,
                        })
                        .collect::<Vec<_>>(),
                );

                this.toggle_entry(&entry_id, cx);
            })
        })
        .detach_and_log_err(cx);
    }

    #[allow(clippy::too_many_arguments)]
    fn render_variable(
        &self,
        parent_variables_reference: u64,
        variable: &Variable,
        scope: &Scope,
        depth: usize,
        has_children: bool,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement {
        let scope_id = scope.variables_reference;
        let variable_reference = variable.variables_reference;

        let entry_id = OpenEntry::Variable {
            name: variable.name.clone(),
            depth,
        };
        let disclosed = has_children.then(|| self.open_entries.binary_search(&entry_id).is_ok());

        div()
            .id(SharedString::from(format!(
                "variable-{}-{}-{}",
                scope.variables_reference, variable.name, depth
            )))
            .group("")
            .h_4()
            .size_full()
            .child(
                ListItem::new(SharedString::from(format!(
                    "variable-item-{}-{}-{}",
                    scope.variables_reference, variable.name, depth
                )))
                .indent_level(depth + 1)
                .indent_step_size(px(20.))
                .always_show_disclosure_icon(true)
                .toggle(disclosed)
                .on_toggle(cx.listener(move |this, _, cx| {
                    this.on_toggle_variable(
                        scope_id,
                        &entry_id,
                        variable_reference,
                        depth,
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
                                .child(variable.value.replace("\n", " ").clone()),
                        ),
                ),
            )
            .into_any()
    }

    fn render_scope(&self, scope: &Scope, cx: &mut ViewContext<Self>) -> AnyElement {
        let element_id = scope.variables_reference;

        let entry_id = OpenEntry::Scope {
            name: scope.name.clone(),
        };
        let disclosed = self.open_entries.binary_search(&entry_id).is_ok();

        div()
            .id(element_id as usize)
            .group("")
            .flex()
            .w_full()
            .h_full()
            .child(
                ListItem::new(SharedString::from(format!(
                    "scope-{}",
                    scope.variables_reference
                )))
                .indent_level(1)
                .indent_step_size(px(20.))
                .always_show_disclosure_icon(true)
                .toggle(disclosed)
                .on_toggle(cx.listener(move |this, _, cx| this.toggle_entry(&entry_id, cx)))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_initial_variables_to_index() {
        let mut index = ScopeVariableIndex::new();

        assert_eq!(index.variables(), &[]);

        let variable1 = VariableContainer {
            variable: Variable {
                name: "First variable".into(),
                value: "First variable".into(),
                type_: None,
                presentation_hint: None,
                evaluate_name: None,
                variables_reference: 0,
                named_variables: None,
                indexed_variables: None,
                memory_reference: None,
            },
            depth: 1,
            container_reference: 1,
        };

        let variable2 = VariableContainer {
            variable: Variable {
                name: "Second variable with child".into(),
                value: "Second variable with child".into(),
                type_: None,
                presentation_hint: None,
                evaluate_name: None,
                variables_reference: 2,
                named_variables: None,
                indexed_variables: None,
                memory_reference: None,
            },
            depth: 1,
            container_reference: 1,
        };

        let variable3 = VariableContainer {
            variable: Variable {
                name: "Third variable".into(),
                value: "Third variable".into(),
                type_: None,
                presentation_hint: None,
                evaluate_name: None,
                variables_reference: 0,
                named_variables: None,
                indexed_variables: None,
                memory_reference: None,
            },
            depth: 1,
            container_reference: 1,
        };

        index.add_variables(
            1,
            vec![variable1.clone(), variable2.clone(), variable3.clone()],
        );

        assert_eq!(
            index.variables(),
            &[variable1.clone(), variable2.clone(), variable3.clone()]
        );
    }

    /// This covers when you click on a variable that has a nested variable
    /// We correctly insert the variables right after the variable you clicked on
    #[test]
    fn test_add_sub_variables_to_index() {
        let mut index = ScopeVariableIndex::new();

        assert_eq!(index.variables(), &[]);

        let variable1 = VariableContainer {
            variable: Variable {
                name: "First variable".into(),
                value: "First variable".into(),
                type_: None,
                presentation_hint: None,
                evaluate_name: None,
                variables_reference: 0,
                named_variables: None,
                indexed_variables: None,
                memory_reference: None,
            },
            depth: 1,
            container_reference: 1,
        };

        let variable2 = VariableContainer {
            variable: Variable {
                name: "Second variable with child".into(),
                value: "Second variable with child".into(),
                type_: None,
                presentation_hint: None,
                evaluate_name: None,
                variables_reference: 2,
                named_variables: None,
                indexed_variables: None,
                memory_reference: None,
            },
            depth: 1,
            container_reference: 1,
        };

        let variable3 = VariableContainer {
            variable: Variable {
                name: "Third variable".into(),
                value: "Third variable".into(),
                type_: None,
                presentation_hint: None,
                evaluate_name: None,
                variables_reference: 0,
                named_variables: None,
                indexed_variables: None,
                memory_reference: None,
            },
            depth: 1,
            container_reference: 1,
        };

        index.add_variables(
            1,
            vec![variable1.clone(), variable2.clone(), variable3.clone()],
        );

        assert_eq!(
            index.variables(),
            &[variable1.clone(), variable2.clone(), variable3.clone()]
        );

        let variable4 = VariableContainer {
            variable: Variable {
                name: "Fourth variable".into(),
                value: "Fourth variable".into(),
                type_: None,
                presentation_hint: None,
                evaluate_name: None,
                variables_reference: 0,
                named_variables: None,
                indexed_variables: None,
                memory_reference: None,
            },
            depth: 1,
            container_reference: 1,
        };

        let variable5 = VariableContainer {
            variable: Variable {
                name: "Five variable".into(),
                value: "Five variable".into(),
                type_: None,
                presentation_hint: None,
                evaluate_name: None,
                variables_reference: 0,
                named_variables: None,
                indexed_variables: None,
                memory_reference: None,
            },
            depth: 1,
            container_reference: 1,
        };

        index.add_variables(2, vec![variable4.clone(), variable5.clone()]);

        assert_eq!(
            index.variables(),
            &[
                variable1.clone(),
                variable2.clone(),
                variable4.clone(),
                variable5.clone(),
                variable3.clone(),
            ]
        );
    }
}
