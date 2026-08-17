// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use crate::node::PlatformNode;
use accesskit::{ActionHandler, ActionRequest};
use accesskit_consumer::{NodeId, Tree};
use hashbrown::HashMap;
use objc2::rc::{Id, WeakId};
use objc2_app_kit::*;
use objc2_foundation::MainThreadMarker;
use std::fmt::Debug;
use std::{cell::RefCell, rc::Rc};

pub(crate) trait ActionHandlerNoMut {
    fn do_action(&self, request: ActionRequest);
}

pub(crate) struct ActionHandlerWrapper<H: ActionHandler>(RefCell<H>);

impl<H: 'static + ActionHandler> ActionHandlerWrapper<H> {
    pub(crate) fn new(inner: H) -> Self {
        Self(RefCell::new(inner))
    }
}

impl<H: ActionHandler> ActionHandlerNoMut for ActionHandlerWrapper<H> {
    fn do_action(&self, request: ActionRequest) {
        self.0.borrow_mut().do_action(request)
    }
}

pub(crate) struct Context {
    pub(crate) view: WeakId<NSView>,
    pub(crate) tree: RefCell<Tree>,
    pub(crate) action_handler: Rc<dyn ActionHandlerNoMut>,
    platform_nodes: RefCell<HashMap<NodeId, Id<PlatformNode>>>,
    pub(crate) mtm: MainThreadMarker,
}

impl Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("view", &self.view)
            .field("tree", &self.tree)
            .field("action_handler", &"ActionHandler")
            .field("platform_nodes", &self.platform_nodes)
            .field("mtm", &self.mtm)
            .finish()
    }
}

impl Context {
    pub(crate) fn new(
        view: WeakId<NSView>,
        tree: Tree,
        action_handler: Rc<dyn ActionHandlerNoMut>,
        mtm: MainThreadMarker,
    ) -> Rc<Self> {
        Rc::new(Self {
            view,
            tree: RefCell::new(tree),
            action_handler,
            platform_nodes: RefCell::new(HashMap::new()),
            mtm,
        })
    }

    pub(crate) fn get_or_create_platform_node(self: &Rc<Self>, id: NodeId) -> Id<PlatformNode> {
        let mut platform_nodes = self.platform_nodes.borrow_mut();
        if let Some(result) = platform_nodes.get(&id) {
            return result.clone();
        }

        let result = PlatformNode::new(Rc::downgrade(self), id);
        platform_nodes.insert(id, result.clone());
        result
    }

    pub(crate) fn remove_platform_node(&self, id: NodeId) -> Option<Id<PlatformNode>> {
        let mut platform_nodes = self.platform_nodes.borrow_mut();
        platform_nodes.remove(&id)
    }

    pub(crate) fn do_action(&self, request: ActionRequest) {
        self.action_handler.do_action(request);
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        let platform_nodes = self.platform_nodes.borrow();
        for platform_node in platform_nodes.values() {
            unsafe {
                NSAccessibilityPostNotification(
                    platform_node,
                    NSAccessibilityUIElementDestroyedNotification,
                )
            };
        }
    }
}
