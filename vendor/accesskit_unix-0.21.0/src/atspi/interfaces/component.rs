// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use accesskit_atspi_common::{PlatformNode, Rect};
use atspi::{CoordType, Layer, ScrollType};
use zbus::{fdo, interface, names::OwnedUniqueName};

use crate::atspi::{ObjectId, OwnedObjectAddress};

pub(crate) struct ComponentInterface {
    bus_name: OwnedUniqueName,
    node: PlatformNode,
}

impl ComponentInterface {
    pub fn new(bus_name: OwnedUniqueName, node: PlatformNode) -> Self {
        Self { bus_name, node }
    }

    fn map_error(&self) -> impl '_ + FnOnce(accesskit_atspi_common::Error) -> fdo::Error {
        |error| crate::util::map_error_from_node(&self.node, error)
    }
}

#[interface(name = "org.a11y.atspi.Component")]
impl ComponentInterface {
    fn contains(&self, x: i32, y: i32, coord_type: CoordType) -> fdo::Result<bool> {
        self.node
            .contains(x, y, coord_type)
            .map_err(self.map_error())
    }

    fn get_accessible_at_point(
        &self,
        x: i32,
        y: i32,
        coord_type: CoordType,
    ) -> fdo::Result<(OwnedObjectAddress,)> {
        let accessible = self
            .node
            .accessible_at_point(x, y, coord_type)
            .map_err(self.map_error())?
            .map(|node| ObjectId::Node {
                adapter: self.node.adapter_id(),
                node,
            });
        Ok(super::optional_object_address(&self.bus_name, accessible))
    }

    fn get_extents(&self, coord_type: CoordType) -> fdo::Result<(Rect,)> {
        self.node
            .extents(coord_type)
            .map(|rect| (rect,))
            .map_err(self.map_error())
    }

    fn get_layer(&self) -> fdo::Result<Layer> {
        self.node.layer().map_err(self.map_error())
    }

    fn grab_focus(&self) -> fdo::Result<bool> {
        self.node.grab_focus().map_err(self.map_error())
    }

    fn scroll_to(&self, scroll_type: ScrollType) -> fdo::Result<bool> {
        self.node.scroll_to(scroll_type).map_err(self.map_error())
    }

    fn scroll_to_point(&self, coord_type: CoordType, x: i32, y: i32) -> fdo::Result<bool> {
        self.node
            .scroll_to_point(coord_type, x, y)
            .map_err(self.map_error())
    }
}
