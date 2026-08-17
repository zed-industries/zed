// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use accesskit::{ActionHandler, ActivationHandler, DeactivationHandler, Rect, TreeUpdate};
use accesskit_atspi_common::{
    next_adapter_id, ActionHandlerNoMut, ActionHandlerWrapper, Adapter as AdapterImpl,
    AdapterCallback, Event, NodeId, PlatformNode, WindowBounds,
};
#[cfg(not(feature = "tokio"))]
use async_channel::Sender;
use atspi::InterfaceSet;
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};
#[cfg(feature = "tokio")]
use tokio::sync::mpsc::UnboundedSender as Sender;

use crate::context::{get_or_init_app_context, get_or_init_messages};

pub(crate) struct Callback {
    messages: Sender<Message>,
}

impl Callback {
    pub(crate) fn new() -> Self {
        let messages = get_or_init_messages();
        Self { messages }
    }

    fn send_message(&self, message: Message) {
        #[cfg(not(feature = "tokio"))]
        let _ = self.messages.try_send(message);
        #[cfg(feature = "tokio")]
        let _ = self.messages.send(message);
    }
}

impl AdapterCallback for Callback {
    fn register_interfaces(&self, adapter: &AdapterImpl, id: NodeId, interfaces: InterfaceSet) {
        let node = adapter.platform_node(id);
        self.send_message(Message::RegisterInterfaces { node, interfaces });
    }

    fn unregister_interfaces(&self, adapter: &AdapterImpl, id: NodeId, interfaces: InterfaceSet) {
        self.send_message(Message::UnregisterInterfaces {
            adapter_id: adapter.id(),
            node_id: id,
            interfaces,
        })
    }

    fn emit_event(&self, adapter: &AdapterImpl, event: Event) {
        self.send_message(Message::EmitEvent {
            adapter_id: adapter.id(),
            event,
        });
    }
}

pub(crate) enum AdapterState {
    Inactive {
        is_window_focused: bool,
        root_window_bounds: WindowBounds,
        action_handler: Arc<dyn ActionHandlerNoMut + Send + Sync>,
    },
    Pending {
        is_window_focused: bool,
        root_window_bounds: WindowBounds,
        action_handler: Arc<dyn ActionHandlerNoMut + Send + Sync>,
    },
    Active(AdapterImpl),
}

impl Debug for AdapterState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AdapterState::Inactive {
                is_window_focused,
                root_window_bounds,
                action_handler: _,
            } => f
                .debug_struct("Inactive")
                .field("is_window_focused", is_window_focused)
                .field("root_window_bounds", root_window_bounds)
                .field("action_handler", &"ActionHandler")
                .finish(),
            AdapterState::Pending {
                is_window_focused,
                root_window_bounds,
                action_handler: _,
            } => f
                .debug_struct("Pending")
                .field("is_window_focused", is_window_focused)
                .field("root_window_bounds", root_window_bounds)
                .field("action_handler", &"ActionHandler")
                .finish(),
            AdapterState::Active(r#impl) => f.debug_tuple("Active").field(r#impl).finish(),
        }
    }
}

#[derive(Debug)]
pub struct Adapter {
    messages: Sender<Message>,
    id: usize,
    state: Arc<Mutex<AdapterState>>,
}

impl Adapter {
    /// Create a new Unix adapter.
    ///
    /// All of the handlers will always be called from another thread.
    pub fn new(
        activation_handler: impl 'static + ActivationHandler + Send,
        action_handler: impl 'static + ActionHandler + Send,
        deactivation_handler: impl 'static + DeactivationHandler + Send,
    ) -> Self {
        let id = next_adapter_id();
        let messages = get_or_init_messages();
        let state = Arc::new(Mutex::new(AdapterState::Inactive {
            is_window_focused: false,
            root_window_bounds: Default::default(),
            action_handler: Arc::new(ActionHandlerWrapper::new(action_handler)),
        }));
        let adapter = Self {
            id,
            messages,
            state: Arc::clone(&state),
        };
        adapter.send_message(Message::AddAdapter {
            id,
            activation_handler: Box::new(activation_handler),
            deactivation_handler: Box::new(deactivation_handler),
            state,
        });
        adapter
    }

    pub(crate) fn send_message(&self, message: Message) {
        #[cfg(not(feature = "tokio"))]
        let _ = self.messages.try_send(message);
        #[cfg(feature = "tokio")]
        let _ = self.messages.send(message);
    }

    /// Set the bounds of the top-level window. The outer bounds contain any
    /// window decoration and borders.
    ///
    /// # Caveats
    ///
    /// Since an application can not get the position of its window under
    /// Wayland, calling this method only makes sense under X11.
    pub fn set_root_window_bounds(&mut self, outer: Rect, inner: Rect) {
        let new_bounds = WindowBounds::new(outer, inner);
        let mut state = self.state.lock().unwrap();
        match &mut *state {
            AdapterState::Inactive {
                root_window_bounds, ..
            } => {
                *root_window_bounds = new_bounds;
            }
            AdapterState::Pending {
                root_window_bounds, ..
            } => {
                *root_window_bounds = new_bounds;
            }
            AdapterState::Active(r#impl) => r#impl.set_root_window_bounds(new_bounds),
        }
    }

    /// If and only if the tree has been initialized, call the provided function
    /// and apply the resulting update. Note: If the caller's implementation of
    /// [`ActivationHandler::request_initial_tree`] initially returned `None`,
    /// the [`TreeUpdate`] returned by the provided function must contain
    /// a full tree.
    pub fn update_if_active(&mut self, update_factory: impl FnOnce() -> TreeUpdate) {
        let mut state = self.state.lock().unwrap();
        match &mut *state {
            AdapterState::Inactive { .. } => (),
            AdapterState::Pending {
                is_window_focused,
                root_window_bounds,
                action_handler,
            } => {
                let initial_state = update_factory();
                let r#impl = AdapterImpl::with_wrapped_action_handler(
                    self.id,
                    get_or_init_app_context(),
                    Callback::new(),
                    initial_state,
                    *is_window_focused,
                    *root_window_bounds,
                    Arc::clone(action_handler),
                );
                *state = AdapterState::Active(r#impl);
            }
            AdapterState::Active(r#impl) => r#impl.update(update_factory()),
        }
    }

    /// Update the tree state based on whether the window is focused.
    pub fn update_window_focus_state(&mut self, is_focused: bool) {
        let mut state = self.state.lock().unwrap();
        match &mut *state {
            AdapterState::Inactive {
                is_window_focused, ..
            } => {
                *is_window_focused = is_focused;
            }
            AdapterState::Pending {
                is_window_focused, ..
            } => {
                *is_window_focused = is_focused;
            }
            AdapterState::Active(r#impl) => r#impl.update_window_focus_state(is_focused),
        }
    }
}

impl Drop for Adapter {
    fn drop(&mut self) {
        self.send_message(Message::RemoveAdapter { id: self.id });
    }
}

pub(crate) enum Message {
    AddAdapter {
        id: usize,
        activation_handler: Box<dyn ActivationHandler + Send>,
        deactivation_handler: Box<dyn DeactivationHandler + Send>,
        state: Arc<Mutex<AdapterState>>,
    },
    RemoveAdapter {
        id: usize,
    },
    RegisterInterfaces {
        node: PlatformNode,
        interfaces: InterfaceSet,
    },
    UnregisterInterfaces {
        adapter_id: usize,
        node_id: NodeId,
        interfaces: InterfaceSet,
    },
    EmitEvent {
        adapter_id: usize,
        event: Event,
    },
}
