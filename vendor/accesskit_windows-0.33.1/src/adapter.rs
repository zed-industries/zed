// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use accesskit::{
    ActionHandler, ActivationHandler, Live, Node as NodeProvider, NodeId as LocalNodeId, Role,
    Tree as TreeData, TreeId, TreeUpdate,
};
use accesskit_consumer::{FilterResult, Node, NodeId, Tree, TreeChangeHandler};
use hashbrown::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, atomic::Ordering};
use windows::Win32::{
    Foundation::*,
    UI::{Accessibility::*, WindowsAndMessaging::*},
};

use crate::{
    context::{ActionHandlerNoMut, ActionHandlerWrapper, Context},
    filters::filter,
    node::{NodeWrapper, PlatformNode},
    util::QueuedEvent,
    window_handle::WindowHandle,
};

fn focus_event(context: &Arc<Context>, node_id: NodeId) -> QueuedEvent {
    let platform_node = PlatformNode::new(context, node_id);
    let element: IRawElementProviderSimple = platform_node.into();
    QueuedEvent::Simple {
        element,
        event_id: UIA_AutomationFocusChangedEventId,
    }
}

struct AdapterChangeHandler<'a> {
    context: &'a Arc<Context>,
    queue: Vec<QueuedEvent>,
    text_changed: HashSet<NodeId>,
    selection_changed: HashMap<NodeId, SelectionChanges>,
}

impl<'a> AdapterChangeHandler<'a> {
    fn new(context: &'a Arc<Context>) -> Self {
        Self {
            context,
            queue: Vec::new(),
            text_changed: HashSet::new(),
            selection_changed: HashMap::new(),
        }
    }
}

impl AdapterChangeHandler<'_> {
    fn insert_text_change_if_needed_parent(&mut self, node: Node) {
        if !node.supports_text_ranges() {
            return;
        }
        let id = node.id();
        if self.text_changed.contains(&id) {
            return;
        }
        let platform_node = PlatformNode::new(self.context, node.id());
        let element: IRawElementProviderSimple = platform_node.into();
        // Text change events must come before selection change
        // events. It doesn't matter if text change events come
        // before other events.
        self.queue.insert(
            0,
            QueuedEvent::Simple {
                element,
                event_id: UIA_Text_TextChangedEventId,
            },
        );
        self.text_changed.insert(id);
    }

    fn insert_text_change_if_needed(&mut self, node: &Node) {
        if node.role() != Role::TextRun {
            return;
        }
        if let Some(node) = node.filtered_parent(&filter) {
            self.insert_text_change_if_needed_parent(node);
        }
    }

    fn handle_selection_state_change(&mut self, node: &Node, is_selected: bool) {
        // If `node` belongs to a selection container, then map the events with the
        // selection container as the key because |FinalizeSelectionEvents| needs to
        // determine whether or not there is only one element selected in order to
        // optimize what platform events are sent.
        let key = if let Some(container) = node.selection_container(&filter) {
            container.id()
        } else {
            node.id()
        };

        let changes = self
            .selection_changed
            .entry(key)
            .or_insert_with(|| SelectionChanges {
                added_items: HashSet::new(),
                removed_items: HashSet::new(),
            });
        if is_selected {
            changes.added_items.insert(node.id());
        } else {
            changes.removed_items.insert(node.id());
        }
    }

    fn enqueue_selection_changes(&mut self, tree: &Tree) {
        let tree_state = tree.state();
        for (id, changes) in self.selection_changed.iter() {
            let Some(node) = tree_state.node_by_id(*id) else {
                continue;
            };
            // Determine if `node` is a selection container with one selected child in
            // order to optimize what platform events are sent.
            let mut container = None;
            let mut only_selected_child = None;
            if node.is_container_with_selectable_children() {
                container = Some(node);
                for child in node.filtered_children(filter) {
                    if let Some(true) = child.is_selected() {
                        if only_selected_child.is_none() {
                            only_selected_child = Some(child);
                        } else {
                            only_selected_child = None;
                            break;
                        }
                    }
                }
            }

            if let Some(only_selected_child) = only_selected_child {
                self.queue.push(QueuedEvent::Simple {
                    element: PlatformNode::new(self.context, only_selected_child.id()).into(),
                    event_id: UIA_SelectionItem_ElementSelectedEventId,
                });
                self.queue.push(QueuedEvent::PropertyChanged {
                    element: PlatformNode::new(self.context, only_selected_child.id()).into(),
                    property_id: UIA_SelectionItemIsSelectedPropertyId,
                    old_value: false.into(),
                    new_value: true.into(),
                });
                for child_id in changes.removed_items.iter() {
                    let platform_node = PlatformNode::new(self.context, *child_id);
                    self.queue.push(QueuedEvent::PropertyChanged {
                        element: platform_node.into(),
                        property_id: UIA_SelectionItemIsSelectedPropertyId,
                        old_value: true.into(),
                        new_value: false.into(),
                    });
                }
            } else {
                // Per UIA documentation, beyond the "invalidate limit" we're supposed to
                // fire a 'SelectionInvalidated' event.  The exact value isn't specified,
                // but System.Windows.Automation.Provider uses a value of 20.
                const INVALIDATE_LIMIT: usize = 20;
                if let Some(container) = container.filter(|_| {
                    changes.added_items.len() + changes.removed_items.len() > INVALIDATE_LIMIT
                }) {
                    let platform_node = PlatformNode::new(self.context, container.id());
                    self.queue.push(QueuedEvent::Simple {
                        element: platform_node.into(),
                        event_id: UIA_Selection_InvalidatedEventId,
                    });
                } else {
                    let container_is_multiselectable =
                        container.is_some_and(|c| c.is_multiselectable());
                    for added_id in changes.added_items.iter() {
                        self.queue.push(QueuedEvent::Simple {
                            element: PlatformNode::new(self.context, *added_id).into(),
                            event_id: match container_is_multiselectable {
                                true => UIA_SelectionItem_ElementAddedToSelectionEventId,
                                false => UIA_SelectionItem_ElementSelectedEventId,
                            },
                        });
                        self.queue.push(QueuedEvent::PropertyChanged {
                            element: PlatformNode::new(self.context, *added_id).into(),
                            property_id: UIA_SelectionItemIsSelectedPropertyId,
                            old_value: false.into(),
                            new_value: true.into(),
                        });
                    }
                    for removed_id in changes.removed_items.iter() {
                        self.queue.push(QueuedEvent::Simple {
                            element: PlatformNode::new(self.context, *removed_id).into(),
                            event_id: UIA_SelectionItem_ElementRemovedFromSelectionEventId,
                        });
                        self.queue.push(QueuedEvent::PropertyChanged {
                            element: PlatformNode::new(self.context, *removed_id).into(),
                            property_id: UIA_SelectionItemIsSelectedPropertyId,
                            old_value: true.into(),
                            new_value: false.into(),
                        });
                    }
                }
            }
        }
    }
}

struct SelectionChanges {
    added_items: HashSet<NodeId>,
    removed_items: HashSet<NodeId>,
}

impl TreeChangeHandler for AdapterChangeHandler<'_> {
    fn node_added(&mut self, node: &Node) {
        self.insert_text_change_if_needed(node);
        if filter(node) != FilterResult::Include {
            return;
        }
        let wrapper = NodeWrapper(node);
        if node.is_dialog() {
            let platform_node = PlatformNode::new(self.context, node.id());
            let element: IRawElementProviderSimple = platform_node.into();
            self.queue.push(QueuedEvent::Simple {
                element,
                event_id: UIA_Window_WindowOpenedEventId,
            });
        }
        if wrapper.name().is_some() && node.live() != Live::Off {
            let platform_node = PlatformNode::new(self.context, node.id());
            let element: IRawElementProviderSimple = platform_node.into();
            self.queue.push(QueuedEvent::Simple {
                element,
                event_id: UIA_LiveRegionChangedEventId,
            });
        }
        if wrapper.is_selection_item_pattern_supported() && wrapper.is_selected() {
            self.handle_selection_state_change(node, true);
        }
    }

    fn node_updated(&mut self, old_node: &Node, new_node: &Node) {
        if old_node.raw_value() != new_node.raw_value() {
            self.insert_text_change_if_needed(new_node);
        }
        let old_node_was_filtered_out = filter(old_node) != FilterResult::Include;
        if filter(new_node) != FilterResult::Include {
            if !old_node_was_filtered_out {
                if old_node.is_dialog() {
                    let platform_node = PlatformNode::new(self.context, old_node.id());
                    let element: IRawElementProviderSimple = platform_node.into();
                    self.queue.push(QueuedEvent::Simple {
                        element,
                        event_id: UIA_Window_WindowClosedEventId,
                    });
                }
                let old_wrapper = NodeWrapper(old_node);
                if old_wrapper.is_selection_item_pattern_supported() && old_wrapper.is_selected() {
                    self.handle_selection_state_change(old_node, false);
                }
            }
            return;
        }
        let platform_node = PlatformNode::new(self.context, new_node.id());
        let element: IRawElementProviderSimple = platform_node.into();
        let old_wrapper = NodeWrapper(old_node);
        let new_wrapper = NodeWrapper(new_node);
        new_wrapper.enqueue_property_changes(
            &mut self.queue,
            &PlatformNode::new(self.context, new_node.id()),
            &element,
            &old_wrapper,
        );
        let new_name = new_wrapper.name();
        if new_name.is_some()
            && new_node.live() != Live::Off
            && (new_node.live() != old_node.live()
                || old_node_was_filtered_out
                || new_name != old_wrapper.name())
        {
            self.queue.push(QueuedEvent::Simple {
                element,
                event_id: UIA_LiveRegionChangedEventId,
            });
        }
        if old_node_was_filtered_out && new_node.is_dialog() {
            let platform_node = PlatformNode::new(self.context, new_node.id());
            let element: IRawElementProviderSimple = platform_node.into();
            self.queue.push(QueuedEvent::Simple {
                element,
                event_id: UIA_Window_WindowOpenedEventId,
            });
        }
        if new_wrapper.is_selection_item_pattern_supported()
            && (new_wrapper.is_selected() != old_wrapper.is_selected()
                || (old_node_was_filtered_out && new_wrapper.is_selected()))
        {
            self.handle_selection_state_change(new_node, new_wrapper.is_selected());
        }
    }

    fn focus_moved(&mut self, _old_node: Option<&Node>, new_node: Option<&Node>) {
        if let Some(new_node) = new_node {
            self.queue.push(focus_event(self.context, new_node.id()));
        }
    }

    fn node_removed(&mut self, node: &Node) {
        self.insert_text_change_if_needed(node);
        if filter(node) != FilterResult::Include {
            return;
        }
        if node.is_dialog() {
            let platform_node = PlatformNode::new(self.context, node.id());
            let element: IRawElementProviderSimple = platform_node.into();
            self.queue.push(QueuedEvent::Simple {
                element,
                event_id: UIA_Window_WindowClosedEventId,
            });
        }
        let wrapper = NodeWrapper(node);
        if wrapper.is_selection_item_pattern_supported() {
            self.handle_selection_state_change(node, false);
        }
    }

    // TODO: handle other events (#20)
}

const PLACEHOLDER_ROOT_ID: LocalNodeId = LocalNodeId(0);

enum State {
    Inactive {
        hwnd: WindowHandle,
        is_window_focused: bool,
        action_handler: Arc<dyn ActionHandlerNoMut + Send + Sync>,
    },
    Placeholder(Arc<Context>),
    Active(Arc<Context>),
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Inactive {
                hwnd,
                is_window_focused,
                action_handler: _,
            } => f
                .debug_struct("Inactive")
                .field("hwnd", hwnd)
                .field("is_window_focused", is_window_focused)
                .field("action_handler", &"ActionHandler")
                .finish(),
            State::Placeholder(context) => f.debug_tuple("Placeholder").field(context).finish(),
            State::Active(context) => f.debug_tuple("Active").field(context).finish(),
        }
    }
}

#[derive(Debug)]
pub struct Adapter {
    state: State,
}

impl Adapter {
    /// Creates a new Windows platform adapter.
    ///
    /// The action handler may or may not be called on the thread that owns
    /// the window.
    ///
    /// This must not be called while handling the `WM_GETOBJECT` message,
    /// because this function must initialize UI Automation before
    /// that message is handled. This is necessary to prevent a race condition
    /// that leads to nested `WM_GETOBJECT` messages and, in some cases,
    /// assistive technologies not realizing that the window natively implements.
    /// UIA. See [AccessKit issue #37](https://github.com/AccessKit/accesskit/issues/37)
    /// for more details.
    pub fn new(
        hwnd: HWND,
        is_window_focused: bool,
        action_handler: impl 'static + ActionHandler + Send,
    ) -> Self {
        Self::with_wrapped_action_handler(
            hwnd,
            is_window_focused,
            Arc::new(ActionHandlerWrapper::new(action_handler)),
        )
    }

    // Currently required by the test infrastructure
    pub(crate) fn with_wrapped_action_handler(
        hwnd: HWND,
        is_window_focused: bool,
        action_handler: Arc<dyn ActionHandlerNoMut + Send + Sync>,
    ) -> Self {
        init_uia();

        let state = State::Inactive {
            hwnd: hwnd.into(),
            is_window_focused,
            action_handler,
        };
        Self { state }
    }

    /// If and only if the tree has been initialized, call the provided function
    /// and apply the resulting update. Note: If the caller's implementation of
    /// [`ActivationHandler::request_initial_tree`] initially returned `None`,
    /// the [`TreeUpdate`] returned by the provided function must contain
    /// a full tree.
    ///
    /// If a [`QueuedEvents`] instance is returned, the caller must call
    /// [`QueuedEvents::raise`] on it.
    ///
    /// This method may be safely called on any thread, but refer to
    /// [`QueuedEvents::raise`] for restrictions on the context in which
    /// it should be called.
    pub fn update_if_active(
        &mut self,
        update_factory: impl FnOnce() -> TreeUpdate,
    ) -> Option<QueuedEvents> {
        match &self.state {
            State::Inactive { .. } => None,
            State::Placeholder(context) => {
                let is_window_focused = context.read_tree().state().is_host_focused();
                let tree = Tree::new(update_factory(), is_window_focused);
                *context.tree.write().unwrap() = tree;
                context.is_placeholder.store(false, Ordering::SeqCst);
                let result = context
                    .read_tree()
                    .state()
                    .focus()
                    .map(|node| QueuedEvents(vec![focus_event(context, node.id())]));
                self.state = State::Active(Arc::clone(context));
                result
            }
            State::Active(context) => {
                let mut handler = AdapterChangeHandler::new(context);
                let mut tree = context.tree.write().unwrap();
                tree.update_and_process_changes(update_factory(), &mut handler);
                handler.enqueue_selection_changes(&tree);
                Some(QueuedEvents(handler.queue))
            }
        }
    }

    /// Update the tree state based on whether the window is focused.
    ///
    /// If a [`QueuedEvents`] instance is returned, the caller must call
    /// [`QueuedEvents::raise`] on it.
    ///
    /// This method may be safely called on any thread, but refer to
    /// [`QueuedEvents::raise`] for restrictions on the context in which
    /// it should be called.
    pub fn update_window_focus_state(&mut self, is_focused: bool) -> Option<QueuedEvents> {
        match &mut self.state {
            State::Inactive {
                is_window_focused, ..
            } => {
                *is_window_focused = is_focused;
                None
            }
            State::Placeholder(context) => {
                let mut handler = AdapterChangeHandler::new(context);
                let mut tree = context.tree.write().unwrap();
                tree.update_host_focus_state_and_process_changes(is_focused, &mut handler);
                Some(QueuedEvents(handler.queue))
            }
            State::Active(context) => {
                let mut handler = AdapterChangeHandler::new(context);
                let mut tree = context.tree.write().unwrap();
                tree.update_host_focus_state_and_process_changes(is_focused, &mut handler);
                Some(QueuedEvents(handler.queue))
            }
        }
    }

    /// Handle the `WM_GETOBJECT` window message. The accessibility tree
    /// is lazily initialized if necessary using the provided
    /// [`ActivationHandler`] implementation.
    ///
    /// This returns an `Option` so the caller can pass the message
    /// to `DefWindowProc` if AccessKit decides not to handle it.
    /// The optional value is an `Into<LRESULT>` rather than simply an `LRESULT`
    /// so the necessary call to UIA, which may lead to a nested `WM_GETOBJECT`
    /// message, can be done outside of any lock that the caller might hold
    /// on the `Adapter` or window state, while still abstracting away
    /// the details of that call to UIA.
    pub fn handle_wm_getobject<H: ActivationHandler + ?Sized>(
        &mut self,
        wparam: WPARAM,
        lparam: LPARAM,
        activation_handler: &mut H,
    ) -> Option<impl Into<LRESULT> + use<H>> {
        // Don't bother with MSAA object IDs that are asking for something other
        // than the client area of the window. DefWindowProc can handle those.
        // First, cast the lparam to i32, to handle inconsistent conversion
        // behavior in senders.
        let objid = normalize_objid(lparam);
        if objid < 0 && objid != UiaRootObjectId && objid != OBJID_CLIENT.0 {
            return None;
        }

        let (hwnd, platform_node) = match &self.state {
            State::Inactive {
                hwnd,
                is_window_focused,
                action_handler,
            } => match activation_handler.request_initial_tree() {
                Some(initial_state) => {
                    let hwnd = *hwnd;
                    let tree = Tree::new(initial_state, *is_window_focused);
                    let context = Context::new(hwnd, tree, Arc::clone(action_handler), false);
                    let node_id = context.read_tree().state().root_id();
                    let platform_node = PlatformNode::new(&context, node_id);
                    self.state = State::Active(context);
                    (hwnd, platform_node)
                }
                None => {
                    let hwnd = *hwnd;
                    let placeholder_update = TreeUpdate {
                        nodes: vec![(PLACEHOLDER_ROOT_ID, NodeProvider::new(Role::Window))],
                        tree: Some(TreeData::new(PLACEHOLDER_ROOT_ID)),
                        tree_id: TreeId::ROOT,
                        focus: PLACEHOLDER_ROOT_ID,
                    };
                    let placeholder_tree = Tree::new(placeholder_update, *is_window_focused);
                    let context =
                        Context::new(hwnd, placeholder_tree, Arc::clone(action_handler), true);
                    let platform_node = PlatformNode::unspecified_root(&context);
                    self.state = State::Placeholder(context);
                    (hwnd, platform_node)
                }
            },
            State::Placeholder(context) => (context.hwnd, PlatformNode::unspecified_root(context)),
            State::Active(context) => {
                let node_id = context.read_tree().state().root_id();
                (context.hwnd, PlatformNode::new(context, node_id))
            }
        };
        let el: IRawElementProviderSimple = platform_node.into();
        Some(WmGetObjectResult {
            hwnd,
            wparam,
            lparam,
            el,
        })
    }
}

fn init_uia() {
    // `UiaLookupId` is a cheap way of forcing UIA to initialize itself.
    unsafe {
        UiaLookupId(
            AutomationIdentifierType_Property,
            &ControlType_Property_GUID,
        )
    };
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
fn normalize_objid(lparam: LPARAM) -> i32 {
    (lparam.0 & 0xFFFFFFFF) as _
}
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
fn normalize_objid(lparam: LPARAM) -> i32 {
    lparam.0 as _
}

struct WmGetObjectResult {
    hwnd: WindowHandle,
    wparam: WPARAM,
    lparam: LPARAM,
    el: IRawElementProviderSimple,
}

impl From<WmGetObjectResult> for LRESULT {
    fn from(this: WmGetObjectResult) -> Self {
        unsafe { UiaReturnRawElementProvider(this.hwnd.0, this.wparam, this.lparam, &this.el) }
    }
}

/// Events generated by a tree update.
#[must_use = "events must be explicitly raised"]
pub struct QueuedEvents(Vec<QueuedEvent>);

impl QueuedEvents {
    /// Raise all queued events synchronously.
    ///
    /// The window may receive `WM_GETOBJECT` messages during this call.
    /// This means that any locks required by the `WM_GETOBJECT` handler
    /// must not be held when this method is called.
    ///
    /// This method should be called on the thread that owns the window.
    /// It's not clear whether this is a strict requirement of UIA itself,
    /// but based on the known behavior of UIA, MSAA, and some ATs,
    /// it's strongly recommended.
    pub fn raise(self) {
        for event in self.0 {
            match event {
                QueuedEvent::Simple { element, event_id } => {
                    unsafe { UiaRaiseAutomationEvent(&element, event_id) }.unwrap();
                }
                QueuedEvent::PropertyChanged {
                    element,
                    property_id,
                    old_value,
                    new_value,
                } => {
                    unsafe {
                        UiaRaiseAutomationPropertyChangedEvent(
                            &element,
                            property_id,
                            &old_value,
                            &new_value,
                        )
                    }
                    .unwrap();
                }
            }
        }
    }
}

// We explicitly want to allow the queued events to be sent to the UI thread,
// so implement Send even though windows-rs doesn't implement it for all
// contained types. This is safe because we're not using COM threading.
unsafe impl Send for QueuedEvents {}
