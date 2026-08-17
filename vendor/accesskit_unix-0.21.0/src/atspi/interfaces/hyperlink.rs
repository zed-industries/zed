// Copyright 2026 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use accesskit_atspi_common::PlatformNode;
use zbus::{fdo, interface, names::OwnedUniqueName};

use crate::atspi::{ObjectId, OwnedObjectAddress};

pub(crate) struct HyperlinkInterface {
    bus_name: OwnedUniqueName,
    node: PlatformNode,
}

impl HyperlinkInterface {
    pub fn new(bus_name: OwnedUniqueName, node: PlatformNode) -> Self {
        Self { bus_name, node }
    }

    fn map_error(&self) -> impl '_ + FnOnce(accesskit_atspi_common::Error) -> fdo::Error {
        |error| crate::util::map_error_from_node(&self.node, error)
    }
}

#[interface(name = "org.a11y.atspi.Hyperlink")]
impl HyperlinkInterface {
    #[zbus(property)]
    fn n_anchors(&self) -> fdo::Result<i32> {
        self.node.n_anchors().map_err(self.map_error())
    }

    #[zbus(property)]
    fn start_index(&self) -> fdo::Result<i32> {
        self.node.hyperlink_start_index().map_err(self.map_error())
    }

    #[zbus(property)]
    fn end_index(&self) -> fdo::Result<i32> {
        self.node.hyperlink_end_index().map_err(self.map_error())
    }

    fn get_object(&self, index: i32) -> fdo::Result<(OwnedObjectAddress,)> {
        let object = self
            .node
            .hyperlink_object(index)
            .map_err(self.map_error())?
            .map(|node| ObjectId::Node {
                adapter: self.node.adapter_id(),
                node,
            });
        Ok(super::optional_object_address(&self.bus_name, object))
    }

    #[zbus(name = "GetURI")]
    fn get_uri(&self, index: i32) -> fdo::Result<String> {
        self.node.uri(index).map_err(self.map_error())
    }

    fn is_valid(&self) -> fdo::Result<bool> {
        self.node.hyperlink_is_valid().map_err(self.map_error())
    }
}
