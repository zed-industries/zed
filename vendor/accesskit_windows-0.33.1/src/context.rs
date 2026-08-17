// Copyright 2023 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use accesskit::{ActionHandler, ActionRequest, Point};
use accesskit_consumer::Tree;
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard, atomic::AtomicBool};

use crate::{util::*, window_handle::WindowHandle};

pub(crate) trait ActionHandlerNoMut {
    fn do_action(&self, request: ActionRequest);
}

pub(crate) struct ActionHandlerWrapper<H: ActionHandler + Send>(Mutex<H>);

impl<H: 'static + ActionHandler + Send> ActionHandlerWrapper<H> {
    pub(crate) fn new(inner: H) -> Self {
        Self(Mutex::new(inner))
    }
}

impl<H: ActionHandler + Send> ActionHandlerNoMut for ActionHandlerWrapper<H> {
    fn do_action(&self, request: ActionRequest) {
        self.0.lock().unwrap().do_action(request)
    }
}

pub(crate) struct Context {
    pub(crate) hwnd: WindowHandle,
    pub(crate) tree: RwLock<Tree>,
    pub(crate) action_handler: Arc<dyn ActionHandlerNoMut + Send + Sync>,
    pub(crate) is_placeholder: AtomicBool,
}

impl Debug for Context {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("hwnd", &self.hwnd)
            .field("tree", &self.tree)
            .field("action_handler", &"ActionHandler")
            .field("is_placeholder", &self.is_placeholder)
            .finish()
    }
}

impl Context {
    pub(crate) fn new(
        hwnd: WindowHandle,
        tree: Tree,
        action_handler: Arc<dyn ActionHandlerNoMut + Send + Sync>,
        is_placeholder: bool,
    ) -> Arc<Self> {
        Arc::new(Self {
            hwnd,
            tree: RwLock::new(tree),
            action_handler,
            is_placeholder: AtomicBool::new(is_placeholder),
        })
    }

    pub(crate) fn read_tree(&self) -> RwLockReadGuard<'_, Tree> {
        self.tree.read().unwrap()
    }

    pub(crate) fn client_top_left(&self) -> Point {
        client_top_left(self.hwnd)
    }

    pub(crate) fn do_action(&self, request: ActionRequest) {
        self.action_handler.do_action(request);
    }
}
