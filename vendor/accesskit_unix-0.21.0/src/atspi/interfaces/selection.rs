// Copyright 2024 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use accesskit_atspi_common::PlatformNode;
use zbus::{fdo, interface, names::OwnedUniqueName};

use crate::atspi::{ObjectId, OwnedObjectAddress};

pub(crate) struct SelectionInterface {
    bus_name: OwnedUniqueName,
    node: PlatformNode,
}

impl SelectionInterface {
    pub fn new(bus_name: OwnedUniqueName, node: PlatformNode) -> Self {
        Self { bus_name, node }
    }

    fn map_error(&self) -> impl '_ + FnOnce(accesskit_atspi_common::Error) -> fdo::Error {
        |error| crate::util::map_error_from_node(&self.node, error)
    }
}

#[interface(name = "org.a11y.atspi.Selection")]
impl SelectionInterface {
    #[zbus(property)]
    fn n_selected_children(&self) -> fdo::Result<i32> {
        self.node.n_selected_children().map_err(self.map_error())
    }

    fn get_selected_child(&self, selected_child_index: i32) -> fdo::Result<(OwnedObjectAddress,)> {
        let child = self
            .node
            .selected_child(map_child_index(selected_child_index)?)
            .map_err(self.map_error())?
            .map(|child| ObjectId::Node {
                adapter: self.node.adapter_id(),
                node: child,
            });
        Ok(super::optional_object_address(&self.bus_name, child))
    }

    fn select_child(&self, child_index: i32) -> fdo::Result<bool> {
        self.node
            .select_child(map_child_index(child_index)?)
            .map_err(self.map_error())
    }

    fn deselect_selected_child(&self, selected_child_index: i32) -> fdo::Result<bool> {
        self.node
            .deselect_selected_child(map_child_index(selected_child_index)?)
            .map_err(self.map_error())
    }

    fn is_child_selected(&self, child_index: i32) -> fdo::Result<bool> {
        self.node
            .is_child_selected(map_child_index(child_index)?)
            .map_err(self.map_error())
    }

    fn select_all(&self) -> fdo::Result<bool> {
        self.node.select_all().map_err(self.map_error())
    }

    fn clear_selection(&self) -> fdo::Result<bool> {
        self.node.clear_selection().map_err(self.map_error())
    }

    fn deselect_child(&self, child_index: i32) -> fdo::Result<bool> {
        self.node
            .deselect_child(map_child_index(child_index)?)
            .map_err(self.map_error())
    }
}

fn map_child_index(index: i32) -> fdo::Result<usize> {
    index
        .try_into()
        .map_err(|_| fdo::Error::InvalidArgs("Index can't be negative.".into()))
}
