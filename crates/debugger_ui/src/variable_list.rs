use crate::stack_frame_list::{StackFrameId, StackFrameList, StackFrameListEvent};
use anyhow::{anyhow, Result};
use dap::{
    client::DebugAdapterClientId, proto_conversions::ProtoConversion, session::DebugSessionId,
    Scope, ScopePresentationHint, Variable,
};
use editor::{actions::SelectAll, Editor, EditorEvent};
use gpui::{
    actions, anchored, deferred, list, AnyElement, ClipboardItem, Context, DismissEvent, Entity,
    FocusHandle, Focusable, Hsla, ListOffset, ListState, MouseDownEvent, Point, Subscription, Task,
};
use menu::{Confirm, SelectFirst, SelectLast, SelectNext, SelectPrev};
use project::dap_store::DapStore;
use rpc::proto::{
    self, DebuggerScopeVariableIndex, DebuggerVariableContainer, UpdateDebugAdapter,
    VariableListScopes, VariableListVariables,
};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    sync::Arc,
};
use sum_tree::{Dimension, Item, SumTree, Summary};
use ui::{prelude::*, ContextMenu, ListItem};
use util::ResultExt;

actions!(variable_list, [ExpandSelectedEntry, CollapseSelectedEntry]);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableContainer {
    pub container_reference: u64,
    pub variable: Variable,
    pub depth: usize,
}

impl ProtoConversion for VariableContainer {
    type ProtoType = DebuggerVariableContainer;
    type Output = Result<Self>;

    fn to_proto(&self) -> Self::ProtoType {
        DebuggerVariableContainer {
            container_reference: self.container_reference,
            depth: self.depth as u64,
            variable: Some(self.variable.to_proto()),
        }
    }

    fn from_proto(payload: Self::ProtoType) -> Self::Output {
        Ok(Self {
            container_reference: payload.container_reference,
            variable: payload.variable.map(Variable::from_proto).ok_or(anyhow!(
                "DebuggerVariableContainer proto message didn't contain DapVariable variable field"
            ))?,
            depth: payload.depth as usize,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetVariableState {
    name: String,
    scope: Scope,
    value: String,
    stack_frame_id: u64,
    evaluate_name: Option<String>,
    parent_variables_reference: u64,
}

impl SetVariableState {
    fn _from_proto(payload: proto::DebuggerSetVariableState) -> Option<Self> {
        let scope = payload.scope.map(|scope| {
            let proto_hint = scope
                .presentation_hint
                .unwrap_or(proto::DapScopePresentationHint::ScopeUnknown.into());

            let presentation_hint = match proto::DapScopePresentationHint::from_i32(proto_hint) {
                Some(proto::DapScopePresentationHint::Arguments) => {
                    Some(ScopePresentationHint::Arguments)
                }
                Some(proto::DapScopePresentationHint::Locals) => {
                    Some(ScopePresentationHint::Locals)
                }
                Some(proto::DapScopePresentationHint::Registers) => {
                    Some(ScopePresentationHint::Registers)
                }
                Some(proto::DapScopePresentationHint::ReturnValue) => {
                    Some(ScopePresentationHint::ReturnValue)
                }
                _ => Some(ScopePresentationHint::Unknown),
            };

            Scope {
                name: scope.name,
                presentation_hint,
                variables_reference: scope.variables_reference,
                named_variables: scope.named_variables,
                indexed_variables: scope.indexed_variables,
                expensive: scope.expensive,
                source: None,
                line: scope.line,
                column: scope.column,
                end_line: scope.end_line,
                end_column: scope.end_column,
            }
        })?;

        Some(SetVariableState {
            name: payload.name,
            scope,
            value: payload.value,
            stack_frame_id: payload.stack_frame_id,
            evaluate_name: payload.evaluate_name.clone(),
            parent_variables_reference: payload.parent_variables_reference,
        })
    }

    fn _to_proto(&self) -> proto::DebuggerSetVariableState {
        proto::DebuggerSetVariableState {
            name: self.name.clone(),
            scope: Some(self.scope.to_proto()),
            value: self.value.clone(),
            stack_frame_id: self.stack_frame_id,
            evaluate_name: self.evaluate_name.clone(),
            parent_variables_reference: self.parent_variables_reference,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpenEntry {
    Scope {
        name: String,
    },
    Variable {
        scope_id: u64,
        name: String,
        depth: usize,
    },
}

impl OpenEntry {
    pub(crate) fn _from_proto(open_entry: &proto::VariableListOpenEntry) -> Option<Self> {
        match open_entry.entry.as_ref()? {
            proto::variable_list_open_entry::Entry::Scope(state) => Some(Self::Scope {
                name: state.name.clone(),
            }),
            proto::variable_list_open_entry::Entry::Variable(state) => Some(Self::Variable {
                name: state.name.clone(),
                depth: state.depth as usize,
                scope_id: state.scope_id,
            }),
        }
    }

    pub(crate) fn _to_proto(&self) -> proto::VariableListOpenEntry {
        let entry = match self {
            OpenEntry::Scope { name } => {
                proto::variable_list_open_entry::Entry::Scope(proto::DebuggerOpenEntryScope {
                    name: name.clone(),
                })
            }
            OpenEntry::Variable {
                name,
                depth,
                scope_id,
            } => {
                proto::variable_list_open_entry::Entry::Variable(proto::DebuggerOpenEntryVariable {
                    name: name.clone(),
                    depth: *depth as u64,
                    scope_id: *scope_id,
                })
            }
        };

        proto::VariableListOpenEntry { entry: Some(entry) }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
pub struct ScopeVariableIndex {
    fetched_ids: HashSet<u64>,
    variables: SumTree<VariableContainer>,
}

#[derive(Clone, Debug, Default)]
pub struct ScopeVariableSummary {
    count: usize,
    max_depth: usize,
    container_reference: u64,
}

impl Item for VariableContainer {
    type Summary = ScopeVariableSummary;

    fn summary(&self, _cx: &()) -> Self::Summary {
        ScopeVariableSummary {
            count: 1,
            max_depth: self.depth,
            container_reference: self.container_reference,
        }
    }
}

impl<'a> Dimension<'a, ScopeVariableSummary> for usize {
    fn zero(_cx: &()) -> Self {
        0
    }

    fn add_summary(&mut self, summary: &'a ScopeVariableSummary, _cx: &()) {
        *self += summary.count;
    }
}

impl Summary for ScopeVariableSummary {
    type Context = ();

    fn zero(_: &Self::Context) -> Self {
        Self::default()
    }

    fn add_summary(&mut self, other: &Self, _: &Self::Context) {
        self.count += other.count;
        self.max_depth = self.max_depth.max(other.max_depth);
        self.container_reference = self.container_reference.max(other.container_reference);
    }
}

impl ProtoConversion for ScopeVariableIndex {
    type ProtoType = DebuggerScopeVariableIndex;
    type Output = Self;

    fn to_proto(&self) -> Self::ProtoType {
        DebuggerScopeVariableIndex {
            fetched_ids: self.fetched_ids.iter().copied().collect(),
            variables: self.variables.iter().map(|var| var.to_proto()).collect(),
        }
    }

    fn from_proto(payload: Self::ProtoType) -> Self {
        Self {
            fetched_ids: payload.fetched_ids.iter().copied().collect(),
            variables: SumTree::from_iter(
                payload
                    .variables
                    .iter()
                    .filter_map(|var| VariableContainer::from_proto(var.clone()).log_err()),
                &(),
            ),
        }
    }
}

impl ScopeVariableIndex {
    pub fn new() -> Self {
        Self {
            variables: SumTree::default(),
            fetched_ids: HashSet::default(),
        }
    }

    pub fn fetched(&self, container_reference: &u64) -> bool {
        self.fetched_ids.contains(container_reference)
    }

    /// All the variables should have the same depth and the same container reference
    pub fn add_variables(&mut self, container_reference: u64, variables: Vec<VariableContainer>) {
        // We want to avoid adding the same variables dued to collab clients sending add variables updates
        if !self.fetched_ids.insert(container_reference) {
            return;
        }

        let mut new_variables = SumTree::new(&());
        let mut cursor = self.variables.cursor::<usize>(&());
        let mut found_insertion_point = false;

        cursor.seek(&0, editor::Bias::Left, &());
        while let Some(variable) = cursor.item() {
            if variable.variable.variables_reference == container_reference {
                found_insertion_point = true;

                let start = *cursor.start();
                new_variables.push(variable.clone(), &());
                new_variables.append(cursor.slice(&start, editor::Bias::Left, &()), &());
                new_variables.extend(variables.iter().cloned(), &());

                cursor.next(&());
                new_variables.append(cursor.suffix(&()), &());

                break;
            }
            new_variables.push(variable.clone(), &());
            cursor.next(&());
        }
        drop(cursor);

        if !found_insertion_point {
            new_variables.extend(variables.iter().cloned(), &());
        }

        self.variables = new_variables;
    }

    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }

    pub fn variables(&self) -> Vec<VariableContainer> {
        self.variables.iter().cloned().collect()
    }
}

type ScopeId = u64;

pub struct VariableList {
    list: ListState,
    focus_handle: FocusHandle,
    dap_store: Entity<DapStore>,
    session_id: DebugSessionId,
    open_entries: Vec<OpenEntry>,
    client_id: DebugAdapterClientId,
    set_variable_editor: Entity<Editor>,
    _subscriptions: Vec<Subscription>,
    selection: Option<VariableListEntry>,
    stack_frame_list: Entity<StackFrameList>,
    scopes: HashMap<StackFrameId, Vec<Scope>>,
    set_variable_state: Option<SetVariableState>,
    fetch_variables_task: Option<Task<Result<()>>>,
    entries: HashMap<StackFrameId, Vec<VariableListEntry>>,
    variables: BTreeMap<(StackFrameId, ScopeId), ScopeVariableIndex>,
    open_context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
}

impl VariableList {
    pub fn new(
        stack_frame_list: &Entity<StackFrameList>,
        dap_store: Entity<DapStore>,
        client_id: &DebugAdapterClientId,
        session_id: &DebugSessionId,
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
            selection: None,
            set_variable_editor,
            client_id: *client_id,
            session_id: *session_id,
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

    pub(crate) fn to_proto(&self) -> proto::DebuggerVariableList {
        let variables = self
            .variables
            .iter()
            .map(
                |((stack_frame_id, scope_id), scope_variable_index)| VariableListVariables {
                    scope_id: *scope_id,
                    stack_frame_id: *stack_frame_id,
                    variables: Some(scope_variable_index.to_proto()),
                },
            )
            .collect();

        let scopes = self
            .scopes
            .iter()
            .map(|(key, scopes)| VariableListScopes {
                stack_frame_id: *key,
                scopes: scopes.to_proto(),
            })
            .collect();

        proto::DebuggerVariableList {
            scopes,
            variables,
            added_variables: vec![],
        }
    }

    pub(crate) fn set_from_proto(
        &mut self,
        state: &proto::DebuggerVariableList,
        cx: &mut Context<Self>,
    ) {
        self.variables = state
            .variables
            .iter()
            .filter_map(|variable| {
                Some((
                    (variable.stack_frame_id, variable.scope_id),
                    ScopeVariableIndex::from_proto(variable.variables.clone()?),
                ))
            })
            .collect();

        self.scopes = state
            .scopes
            .iter()
            .map(|scope| {
                (
                    scope.stack_frame_id,
                    scope
                        .scopes
                        .clone()
                        .into_iter()
                        .map(Scope::from_proto)
                        .collect(),
                )
            })
            .collect();

        for variables in state.added_variables.iter() {
            self.add_variables(variables.clone());
        }

        self.build_entries(true, true, cx);
        cx.notify();
    }

    pub(crate) fn add_variables(&mut self, variables_to_add: proto::AddToVariableList) {
        let variables: Vec<Variable> = Vec::from_proto(variables_to_add.variables);
        let variable_id = variables_to_add.variable_id;
        let stack_frame_id = variables_to_add.stack_frame_id;
        let scope_id = variables_to_add.scope_id;
        let key = (stack_frame_id, scope_id);

        if let Some(depth) = self.variables.get(&key).and_then(|containers| {
            containers
                .variables
                .iter()
                .find(|container| container.variable.variables_reference == variable_id)
                .map(|container| container.depth + 1usize)
        }) {
            if let Some(index) = self.variables.get_mut(&key) {
                index.add_variables(
                    variable_id,
                    variables
                        .into_iter()
                        .map(|var| VariableContainer {
                            container_reference: variable_id,
                            variable: var,
                            depth,
                        })
                        .collect(),
                );
            }
        }
    }

    fn handle_stack_frame_list_events(
        &mut self,
        _: Entity<StackFrameList>,
        event: &StackFrameListEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            StackFrameListEvent::SelectedStackFrameChanged(stack_frame_id) => {
                self.handle_selected_stack_frame_changed(*stack_frame_id, cx);
            }
            StackFrameListEvent::StackFramesUpdated => {
                self.fetch_variables(cx);
            }
        }
    }

    fn handle_selected_stack_frame_changed(
        &mut self,
        stack_frame_id: StackFrameId,
        cx: &mut Context<Self>,
    ) {
        if self.scopes.contains_key(&stack_frame_id) {
            return self.build_entries(true, true, cx);
        }

        self.fetch_variables_task = Some(cx.spawn(|this, mut cx| async move {
            let task = this.update(&mut cx, |variable_list, cx| {
                variable_list.fetch_variables_for_stack_frame(stack_frame_id, &Vec::default(), cx)
            })?;

            let (scopes, variables) = task.await?;

            this.update(&mut cx, |variable_list, cx| {
                variable_list.scopes.insert(stack_frame_id, scopes);

                for (scope_id, variables) in variables.into_iter() {
                    let mut variable_index = ScopeVariableIndex::new();
                    variable_index.add_variables(scope_id, variables);

                    variable_list
                        .variables
                        .insert((stack_frame_id, scope_id), variable_index);
                }

                variable_list.build_entries(true, true, cx);
                variable_list.send_update_proto_message(cx);
            })
        }));
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn scopes(&self) -> &HashMap<StackFrameId, Vec<Scope>> {
        &self.scopes
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn variables(&self) -> Vec<VariableContainer> {
        self.variables
            .iter()
            .flat_map(|((_, _), scope_index)| scope_index.variables())
            .collect()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn entries(&self) -> &HashMap<StackFrameId, Vec<VariableListEntry>> {
        &self.entries
    }

    pub fn variables_by_scope(
        &self,
        stack_frame_id: StackFrameId,
        scope_id: ScopeId,
    ) -> Option<&ScopeVariableIndex> {
        self.variables.get(&(stack_frame_id, scope_id))
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn variables_by_stack_frame_id(
        &self,
        stack_frame_id: StackFrameId,
    ) -> Vec<VariableContainer> {
        self.variables
            .range((stack_frame_id, u64::MIN)..(stack_frame_id, u64::MAX))
            .flat_map(|(_, containers)| containers.variables.iter().cloned())
            .collect()
    }

    pub fn completion_variables(&self, cx: &mut Context<Self>) -> Vec<VariableContainer> {
        let stack_frame_id = self.stack_frame_list.read(cx).first_stack_frame_id();

        self.variables
            .range((stack_frame_id, u64::MIN)..(stack_frame_id, u64::MAX))
            .flat_map(|(_, containers)| containers.variables.iter().cloned())
            .collect()
    }

    fn render_entry(&mut self, ix: usize, cx: &mut Context<Self>) -> AnyElement {
        let stack_frame_id = self.stack_frame_list.read(cx).current_stack_frame_id();

        let Some(entries) = self.entries.get(&stack_frame_id) else {
            return div().into_any_element();
        };

        let entry = &entries[ix];
        match entry {
            VariableListEntry::Scope(scope) => {
                self.render_scope(scope, Some(entry) == self.selection.as_ref(), cx)
            }
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
                Some(entry) == self.selection.as_ref(),
                cx,
            ),
        }
    }

    fn toggle_variable(
        &mut self,
        scope_id: u64,
        variable: &Variable,
        depth: usize,
        cx: &mut Context<Self>,
    ) {
        let stack_frame_id = self.stack_frame_list.read(cx).current_stack_frame_id();

        let Some(variable_index) = self.variables_by_scope(stack_frame_id, scope_id) else {
            return;
        };

        let entry_id = OpenEntry::Variable {
            depth,
            scope_id,
            name: variable.name.clone(),
        };

        let has_children = variable.variables_reference > 0;
        let disclosed = has_children.then(|| self.open_entries.binary_search(&entry_id).is_ok());

        // if we already opened the variable/we already fetched it
        // we can just toggle it because we already have the nested variable
        if disclosed.unwrap_or(true) || variable_index.fetched(&variable.variables_reference) {
            return self.toggle_entry(&entry_id, cx);
        }

        let fetch_variables_task = self.dap_store.update(cx, |store, cx| {
            let thread_id = self.stack_frame_list.read(cx).thread_id();
            store.variables(
                &self.client_id,
                thread_id,
                stack_frame_id,
                scope_id,
                self.session_id,
                variable.variables_reference,
                cx,
            )
        });

        let container_reference = variable.variables_reference;
        let entry_id = entry_id.clone();

        self.fetch_variables_task = Some(cx.spawn(|this, mut cx| async move {
            let new_variables = fetch_variables_task.await?;

            this.update(&mut cx, |this, cx| {
                let Some(index) = this.variables.get_mut(&(stack_frame_id, scope_id)) else {
                    return;
                };

                index.add_variables(
                    container_reference,
                    new_variables
                        .into_iter()
                        .map(|variable| VariableContainer {
                            variable,
                            depth: depth + 1,
                            container_reference,
                        })
                        .collect::<Vec<_>>(),
                );

                this.toggle_entry(&entry_id, cx);
            })
        }))
    }

    pub fn toggle_entry(&mut self, entry_id: &OpenEntry, cx: &mut Context<Self>) {
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
        cx: &mut Context<Self>,
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
                        depth,
                        name: variable.name.clone(),
                        scope_id: scope.variables_reference,
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

        let old_entries = self.entries.get(&stack_frame_id).cloned();
        let old_scroll_top = self.list.logical_scroll_top();

        let len = entries.len();
        self.entries.insert(stack_frame_id, entries.clone());
        self.list.reset(len);

        if let Some(old_entries) = old_entries.as_ref() {
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
        }

        cx.notify();
    }

    fn fetch_nested_variables(
        &self,
        container_reference: u64,
        depth: usize,
        open_entries: &Vec<OpenEntry>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<VariableContainer>>> {
        let stack_frame_list = self.stack_frame_list.read(cx);
        let thread_id = stack_frame_list.thread_id();
        let stack_frame_id = stack_frame_list.current_stack_frame_id();
        let scope_id = container_reference;

        let variables_task = self.dap_store.update(cx, |store, cx| {
            store.variables(
                &self.client_id,
                thread_id,
                stack_frame_id,
                scope_id,
                self.session_id,
                container_reference,
                cx,
            )
        });

        cx.spawn({
            let open_entries = open_entries.clone();
            |this, mut cx| async move {
                let mut variables = Vec::new();

                for variable in variables_task.await? {
                    variables.push(VariableContainer {
                        depth,
                        container_reference,
                        variable: variable.clone(),
                    });

                    if open_entries
                        .binary_search(&OpenEntry::Variable {
                            depth,
                            scope_id: container_reference,
                            name: variable.name.clone(),
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
        cx: &mut Context<Self>,
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

    fn fetch_variables(&mut self, cx: &mut Context<Self>) {
        if self.dap_store.read(cx).upstream_client().is_some() {
            return;
        }

        let stack_frames = self.stack_frame_list.read(cx).stack_frames().clone();

        self.fetch_variables_task = Some(cx.spawn(|this, mut cx| async move {
            let open_entries = this.update(&mut cx, |this, _| {
                this.open_entries
                    .iter()
                    .filter(|e| matches!(e, OpenEntry::Variable { .. }))
                    .cloned()
                    .collect::<Vec<_>>()
            })?;

            let first_stack_frame = stack_frames
                .first()
                .ok_or(anyhow!("Expected to find a stackframe"))?;

            let (scopes, variables) = this
                .update(&mut cx, |this, cx| {
                    this.fetch_variables_for_stack_frame(first_stack_frame.id, &open_entries, cx)
                })?
                .await?;

            this.update(&mut cx, |this, cx| {
                let mut new_variables = BTreeMap::new();
                let mut new_scopes = HashMap::new();

                new_scopes.insert(first_stack_frame.id, scopes);

                for (scope_id, variables) in variables.into_iter() {
                    let mut variable_index = ScopeVariableIndex::new();
                    variable_index.add_variables(scope_id, variables);

                    new_variables.insert((first_stack_frame.id, scope_id), variable_index);
                }

                std::mem::swap(&mut this.variables, &mut new_variables);
                std::mem::swap(&mut this.scopes, &mut new_scopes);

                this.entries.clear();
                this.build_entries(true, true, cx);

                this.send_update_proto_message(cx);

                this.fetch_variables_task.take();
            })
        }));
    }

    fn send_update_proto_message(&self, cx: &mut Context<Self>) {
        if let Some((client, project_id)) = self.dap_store.read(cx).downstream_client() {
            let request = UpdateDebugAdapter {
                client_id: self.client_id.to_proto(),
                session_id: self.session_id.to_proto(),
                thread_id: Some(self.stack_frame_list.read(cx).thread_id()),
                project_id: *project_id,
                variant: Some(rpc::proto::update_debug_adapter::Variant::VariableList(
                    self.to_proto(),
                )),
            };

            client.send(request).log_err();
        };
    }

    fn deploy_variable_context_menu(
        &mut self,
        parent_variables_reference: u64,
        scope: &Scope,
        variable: &Variable,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let support_set_variable = self
            .dap_store
            .read(cx)
            .capabilities_by_id(&self.client_id)
            .supports_set_variable
            .unwrap_or_default();

        let this = cx.entity();

        let context_menu = ContextMenu::build(window, cx, |menu, window, _cx| {
            menu.entry("Copy name", None, {
                let variable_name = variable.name.clone();
                move |_window, cx| {
                    cx.write_to_clipboard(ClipboardItem::new_string(variable_name.clone()))
                }
            })
            .entry("Copy value", None, {
                let source = scope.source.clone();
                let variable_value = variable.value.clone();
                let variable_name = variable.name.clone();
                let evaluate_name = variable.evaluate_name.clone();

                window.handler_for(&this.clone(), move |this, _window, cx| {
                    this.dap_store.update(cx, |dap_store, cx| {
                        if dap_store
                            .capabilities_by_id(&this.client_id)
                            .supports_clipboard_context
                            .unwrap_or_default()
                        {
                            let task = dap_store.evaluate(
                                &this.client_id,
                                this.stack_frame_list.read(cx).current_stack_frame_id(),
                                evaluate_name.clone().unwrap_or(variable_name.clone()),
                                dap::EvaluateArgumentsContext::Clipboard,
                                source.clone(),
                                cx,
                            );

                            cx.spawn(|_, cx| async move {
                                let response = task.await?;

                                cx.update(|cx| {
                                    cx.write_to_clipboard(ClipboardItem::new_string(
                                        response.result,
                                    ))
                                })
                            })
                            .detach_and_log_err(cx);
                        } else {
                            cx.write_to_clipboard(ClipboardItem::new_string(variable_value.clone()))
                        }
                    });
                })
            })
            .when_some(
                variable.memory_reference.clone(),
                |menu, memory_reference| {
                    menu.entry(
                        "Copy memory reference",
                        None,
                        window.handler_for(&this, move |_, _window, cx| {
                            cx.write_to_clipboard(ClipboardItem::new_string(
                                memory_reference.clone(),
                            ))
                        }),
                    )
                },
            )
            .when(support_set_variable, |menu| {
                let variable = variable.clone();
                let scope = scope.clone();

                menu.entry(
                    "Set value",
                    None,
                    window.handler_for(&this, move |this, window, cx| {
                        this.set_variable_state = Some(SetVariableState {
                            parent_variables_reference,
                            name: variable.name.clone(),
                            scope: scope.clone(),
                            evaluate_name: variable.evaluate_name.clone(),
                            value: variable.value.clone(),
                            stack_frame_id: this.stack_frame_list.read(cx).current_stack_frame_id(),
                        });

                        this.set_variable_editor.update(cx, |editor, cx| {
                            editor.set_text(variable.value.clone(), window, cx);
                            editor.select_all(&SelectAll, window, cx);
                            window.focus(&editor.focus_handle(cx))
                        });

                        this.build_entries(false, true, cx);
                    }),
                )
            })
        });

        cx.focus_view(&context_menu, window);
        let subscription = cx.subscribe_in(
            &context_menu,
            window,
            |this, _entity, _event: &DismissEvent, window, cx| {
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

    fn cancel_set_variable_value(&mut self, cx: &mut Context<Self>) {
        if self.set_variable_state.take().is_none() {
            return;
        };

        self.build_entries(false, true, cx);
    }

    fn set_variable_value(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let new_variable_value = self.set_variable_editor.update(cx, |editor, cx| {
            let new_variable_value = editor.text(cx);

            editor.clear(window, cx);

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

        let set_value_task = self.dap_store.update(cx, |store, cx| {
            store.set_variable_value(
                &self.client_id,
                state.stack_frame_id,
                state.parent_variables_reference,
                state.name,
                new_variable_value,
                state.evaluate_name,
                cx,
            )
        });

        cx.spawn_in(window, |this, mut cx| async move {
            set_value_task.await?;

            this.update_in(&mut cx, |this, window, cx| {
                this.build_entries(false, true, cx);
                this.invalidate(window, cx);
            })
        })
        .detach_and_log_err(cx);
    }

    pub fn invalidate(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.stack_frame_list.update(cx, |stack_frame_list, cx| {
            stack_frame_list.invalidate(window, cx);
        });
    }

    fn select_first(&mut self, _: &SelectFirst, _window: &mut Window, cx: &mut Context<Self>) {
        let stack_frame_id = self.stack_frame_list.read(cx).current_stack_frame_id();
        if let Some(entries) = self.entries.get(&stack_frame_id) {
            self.selection = entries.first().cloned();
            cx.notify();
        };
    }

    fn select_last(&mut self, _: &SelectLast, _window: &mut Window, cx: &mut Context<Self>) {
        let stack_frame_id = self.stack_frame_list.read(cx).current_stack_frame_id();
        if let Some(entries) = self.entries.get(&stack_frame_id) {
            self.selection = entries.last().cloned();
            cx.notify();
        };
    }

    fn select_prev(&mut self, _: &SelectPrev, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(selection) = &self.selection {
            let stack_frame_id = self.stack_frame_list.read(cx).current_stack_frame_id();
            if let Some(entries) = self.entries.get(&stack_frame_id) {
                if let Some(ix) = entries.iter().position(|entry| entry == selection) {
                    self.selection = entries.get(ix.saturating_sub(1)).cloned();
                    cx.notify();
                }
            }
        } else {
            self.select_first(&SelectFirst, window, cx);
        }
    }

    fn select_next(&mut self, _: &SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(selection) = &self.selection {
            let stack_frame_id = self.stack_frame_list.read(cx).current_stack_frame_id();
            if let Some(entries) = self.entries.get(&stack_frame_id) {
                if let Some(ix) = entries.iter().position(|entry| entry == selection) {
                    self.selection = entries.get(ix + 1).cloned();
                    cx.notify();
                }
            }
        } else {
            self.select_first(&SelectFirst, window, cx);
        }
    }

    fn collapse_selected_entry(
        &mut self,
        _: &CollapseSelectedEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(selection) = &self.selection {
            match selection {
                VariableListEntry::Scope(scope) => {
                    let entry_id = &OpenEntry::Scope {
                        name: scope.name.clone(),
                    };

                    if self.open_entries.binary_search(entry_id).is_err() {
                        self.select_prev(&SelectPrev, window, cx);
                    } else {
                        self.toggle_entry(entry_id, cx);
                    }
                }
                VariableListEntry::Variable {
                    depth,
                    variable,
                    scope,
                    ..
                } => {
                    let entry_id = &OpenEntry::Variable {
                        depth: *depth,
                        name: variable.name.clone(),
                        scope_id: scope.variables_reference,
                    };

                    if self.open_entries.binary_search(entry_id).is_err() {
                        self.select_prev(&SelectPrev, window, cx);
                    } else {
                        self.toggle_variable(
                            scope.variables_reference,
                            &variable.clone(),
                            *depth,
                            cx,
                        );
                    }
                }
                VariableListEntry::SetVariableEditor { .. } => {}
            }
        }
    }

    fn expand_selected_entry(
        &mut self,
        _: &ExpandSelectedEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(selection) = &self.selection {
            match selection {
                VariableListEntry::Scope(scope) => {
                    let entry_id = &OpenEntry::Scope {
                        name: scope.name.clone(),
                    };

                    if self.open_entries.binary_search(entry_id).is_ok() {
                        self.select_next(&SelectNext, window, cx);
                    } else {
                        self.toggle_entry(entry_id, cx);
                    }
                }
                VariableListEntry::Variable {
                    depth,
                    variable,
                    scope,
                    ..
                } => {
                    let entry_id = &OpenEntry::Variable {
                        depth: *depth,
                        name: variable.name.clone(),
                        scope_id: scope.variables_reference,
                    };

                    if self.open_entries.binary_search(entry_id).is_ok() {
                        self.select_next(&SelectNext, window, cx);
                    } else {
                        self.toggle_variable(
                            scope.variables_reference,
                            &variable.clone(),
                            *depth,
                            cx,
                        );
                    }
                }
                VariableListEntry::SetVariableEditor { .. } => {}
            }
        }
    }

    fn render_set_variable_editor(
        &self,
        depth: usize,
        state: &SetVariableState,
        cx: &mut Context<Self>,
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

    #[cfg(any(test, feature = "test-support"))]
    pub fn toggle_variable_in_test(
        &mut self,
        scope_id: u64,
        variable: &Variable,
        depth: usize,
        cx: &mut Context<Self>,
    ) {
        self.toggle_variable(scope_id, variable, depth, cx);
    }

    #[track_caller]
    #[cfg(any(test, feature = "test-support"))]
    pub fn assert_visual_entries(&self, expected: Vec<&str>, cx: &Context<Self>) {
        const INDENT: &'static str = "    ";

        let stack_frame_id = self.stack_frame_list.read(cx).current_stack_frame_id();
        let entries = self.entries.get(&stack_frame_id).unwrap();

        let mut visual_entries = Vec::with_capacity(entries.len());
        for entry in entries {
            let is_selected = Some(entry) == self.selection.as_ref();

            match entry {
                VariableListEntry::Scope(scope) => {
                    let is_expanded = self
                        .open_entries
                        .binary_search(&OpenEntry::Scope {
                            name: scope.name.clone(),
                        })
                        .is_ok();

                    visual_entries.push(format!(
                        "{} {}{}",
                        if is_expanded { "v" } else { ">" },
                        scope.name,
                        if is_selected { " <=== selected" } else { "" }
                    ));
                }
                VariableListEntry::SetVariableEditor { depth, state } => {
                    visual_entries.push(format!(
                        "{}  [EDITOR: {}]{}",
                        INDENT.repeat(*depth),
                        state.name,
                        if is_selected { " <=== selected" } else { "" }
                    ));
                }
                VariableListEntry::Variable {
                    depth,
                    variable,
                    scope,
                    ..
                } => {
                    let is_expanded = self
                        .open_entries
                        .binary_search(&OpenEntry::Variable {
                            depth: *depth,
                            name: variable.name.clone(),
                            scope_id: scope.variables_reference,
                        })
                        .is_ok();

                    visual_entries.push(format!(
                        "{}{} {}{}",
                        INDENT.repeat(*depth),
                        if is_expanded { "v" } else { ">" },
                        variable.name,
                        if is_selected { " <=== selected" } else { "" }
                    ));
                }
            };
        }

        pretty_assertions::assert_eq!(expected, visual_entries);
    }

    #[allow(clippy::too_many_arguments)]
    fn render_variable(
        &self,
        container_reference: u64,
        variable: &Arc<Variable>,
        scope: &Arc<Scope>,
        depth: usize,
        has_children: bool,
        is_selected: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let scope_id = scope.variables_reference;
        let entry_id = OpenEntry::Variable {
            depth,
            scope_id,
            name: variable.name.clone(),
        };
        let disclosed = has_children.then(|| self.open_entries.binary_search(&entry_id).is_ok());

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

        div()
            .id(SharedString::from(format!(
                "variable-{}-{}-{}",
                scope.variables_reference, variable.name, depth
            )))
            .group("variable_list_entry")
            .border_1()
            .border_r_2()
            .border_color(border_color)
            .h_4()
            .size_full()
            .hover(|style| style.bg(bg_hover_color))
            .on_click(cx.listener({
                let scope = scope.clone();
                let variable = variable.clone();
                move |this, _, _window, cx| {
                    this.selection = Some(VariableListEntry::Variable {
                        depth,
                        has_children,
                        container_reference,
                        scope: scope.clone(),
                        variable: variable.clone(),
                    });
                    cx.notify();
                }
            }))
            .child(
                ListItem::new(SharedString::from(format!(
                    "variable-item-{}-{}-{}",
                    scope.variables_reference, variable.name, depth
                )))
                .selectable(false)
                .indent_level(depth + 1)
                .indent_step_size(px(20.))
                .always_show_disclosure_icon(true)
                .toggle(disclosed)
                .when(has_children, |list_item| {
                    list_item.on_toggle(cx.listener({
                        let variable = variable.clone();
                        move |this, _, _window, cx| {
                            this.toggle_variable(scope_id, &variable, depth, cx)
                        }
                    }))
                })
                .on_secondary_mouse_down(cx.listener({
                    let scope = scope.clone();
                    let variable = variable.clone();
                    move |this, event: &MouseDownEvent, window, cx| {
                        this.deploy_variable_context_menu(
                            container_reference,
                            &scope,
                            &variable,
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

    fn render_scope(&self, scope: &Scope, is_selected: bool, cx: &mut Context<Self>) -> AnyElement {
        let element_id = scope.variables_reference;

        let entry_id = OpenEntry::Scope {
            name: scope.name.clone(),
        };
        let disclosed = self.open_entries.binary_search(&entry_id).is_ok();

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

        div()
            .id(element_id as usize)
            .group("variable_list_entry")
            .border_1()
            .border_r_2()
            .border_color(border_color)
            .flex()
            .w_full()
            .h_full()
            .hover(|style| style.bg(bg_hover_color))
            .on_click(cx.listener({
                let scope = scope.clone();
                move |this, _, _window, cx| {
                    this.selection = Some(VariableListEntry::Scope(scope.clone()));
                    cx.notify();
                }
            }))
            .child(
                ListItem::new(SharedString::from(format!(
                    "scope-{}",
                    scope.variables_reference
                )))
                .selectable(false)
                .indent_level(1)
                .indent_step_size(px(20.))
                .always_show_disclosure_icon(true)
                .toggle(disclosed)
                .on_toggle(
                    cx.listener(move |this, _, _window, cx| this.toggle_entry(&entry_id, cx)),
                )
                .child(div().text_ui(cx).w_full().child(scope.name.clone())),
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
        div()
            .key_context("VariableList")
            .id("variable-list")
            .group("variable-list")
            .size_full()
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::select_prev))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::expand_selected_entry))
            .on_action(cx.listener(Self::collapse_selected_entry))
            .on_action(
                cx.listener(|this, _: &editor::actions::Cancel, _window, cx| {
                    this.cancel_set_variable_value(cx)
                }),
            )
            .child(list(self.list.clone()).gap_1_5().size_full())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_initial_variables_to_index() {
        let mut index = ScopeVariableIndex::new();

        assert_eq!(index.variables(), vec![]);
        assert_eq!(index.fetched_ids, HashSet::default());

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
            vec![variable1.clone(), variable2.clone(), variable3.clone()],
            index.variables(),
        );
        assert_eq!(HashSet::from([1]), index.fetched_ids,);
    }

    /// This covers when you click on a variable that has a nested variable
    /// We correctly insert the variables right after the variable you clicked on
    #[test]
    fn test_add_sub_variables_to_index() {
        let mut index = ScopeVariableIndex::new();

        assert_eq!(index.variables(), vec![]);

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
            vec![variable1.clone(), variable2.clone(), variable3.clone()],
            index.variables(),
        );
        assert_eq!(HashSet::from([1]), index.fetched_ids);

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
            vec![
                variable1.clone(),
                variable2.clone(),
                variable4.clone(),
                variable5.clone(),
                variable3.clone(),
            ],
            index.variables(),
        );
        assert_eq!(index.fetched_ids, HashSet::from([1, 2]));
    }

    #[test]
    fn test_can_serialize_to_and_from_proto() {
        let mut index = ScopeVariableIndex::new();

        let variable1 = VariableContainer {
            variable: Variable {
                name: "First variable".into(),
                value: "First variable".into(),
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

        let variable2 = VariableContainer {
            variable: Variable {
                name: "Second variable with child".into(),
                value: "Second variable with child".into(),
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

        index.add_variables(1, vec![variable1.clone(), variable2.clone()]);

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

        let variable4 = VariableContainer {
            variable: Variable {
                name: "Four variable".into(),
                value: "Four variable".into(),
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

        index.add_variables(2, vec![variable3.clone(), variable4.clone()]);

        assert_eq!(
            vec![
                variable1.clone(),
                variable3.clone(),
                variable4.clone(),
                variable2.clone(),
            ],
            index.variables(),
        );
        assert_eq!(HashSet::from([1, 2]), index.fetched_ids);

        let from_proto = ScopeVariableIndex::from_proto(index.to_proto());

        assert_eq!(index.variables(), from_proto.variables());
        assert_eq!(index.fetched_ids, from_proto.fetched_ids);
    }
}
