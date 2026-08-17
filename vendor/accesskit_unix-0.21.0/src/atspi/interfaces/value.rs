// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use accesskit_atspi_common::PlatformNode;
use zbus::{fdo, interface};

pub(crate) struct ValueInterface {
    node: PlatformNode,
}

impl ValueInterface {
    pub fn new(node: PlatformNode) -> Self {
        Self { node }
    }

    fn map_error(&self) -> impl '_ + FnOnce(accesskit_atspi_common::Error) -> fdo::Error {
        |error| crate::util::map_error_from_node(&self.node, error)
    }
}

#[interface(name = "org.a11y.atspi.Value")]
impl ValueInterface {
    #[zbus(property)]
    fn minimum_value(&self) -> fdo::Result<f64> {
        self.node.minimum_value().map_err(self.map_error())
    }

    #[zbus(property)]
    fn maximum_value(&self) -> fdo::Result<f64> {
        self.node.maximum_value().map_err(self.map_error())
    }

    #[zbus(property)]
    fn minimum_increment(&self) -> fdo::Result<f64> {
        self.node.minimum_increment().map_err(self.map_error())
    }

    #[zbus(property)]
    fn current_value(&self) -> fdo::Result<f64> {
        self.node.current_value().map_err(self.map_error())
    }

    #[zbus(property)]
    fn set_current_value(&mut self, value: f64) -> fdo::Result<()> {
        self.node.set_current_value(value).map_err(self.map_error())
    }
}
