// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use accesskit_atspi_common::{Action, PlatformNode};
use zbus::{fdo, interface};

pub(crate) struct ActionInterface(PlatformNode);

impl ActionInterface {
    pub fn new(node: PlatformNode) -> Self {
        Self(node)
    }

    fn map_error(&self) -> impl '_ + FnOnce(accesskit_atspi_common::Error) -> fdo::Error {
        |error| crate::util::map_error_from_node(&self.0, error)
    }
}

#[interface(name = "org.a11y.atspi.Action")]
impl ActionInterface {
    #[zbus(property)]
    fn n_actions(&self) -> fdo::Result<i32> {
        self.0.n_actions().map_err(self.map_error())
    }

    fn get_description(&self, _index: i32) -> &str {
        ""
    }

    fn get_name(&self, index: i32) -> fdo::Result<String> {
        self.0.action_name(index).map_err(self.map_error())
    }

    fn get_localized_name(&self, index: i32) -> fdo::Result<String> {
        self.0.action_name(index).map_err(self.map_error())
    }

    fn get_key_binding(&self, _index: i32) -> &str {
        ""
    }

    fn get_actions(&self) -> fdo::Result<Vec<Action>> {
        self.0.actions().map_err(self.map_error())
    }

    fn do_action(&self, index: i32) -> fdo::Result<bool> {
        self.0.do_action(index).map_err(self.map_error())
    }
}
