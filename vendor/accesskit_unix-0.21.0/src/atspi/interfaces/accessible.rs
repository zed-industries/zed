// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use std::collections::HashMap;

use accesskit_atspi_common::{NodeIdOrRoot, PlatformNode, PlatformRoot};
use atspi::{Interface, InterfaceSet, RelationType, Role, StateSet};
use zbus::{fdo, interface, names::OwnedUniqueName};

use super::map_root_error;
use crate::atspi::{ObjectId, OwnedObjectAddress};

pub(crate) struct NodeAccessibleInterface {
    bus_name: OwnedUniqueName,
    node: PlatformNode,
}

impl NodeAccessibleInterface {
    pub fn new(bus_name: OwnedUniqueName, node: PlatformNode) -> Self {
        Self { bus_name, node }
    }

    fn map_error(&self) -> impl '_ + FnOnce(accesskit_atspi_common::Error) -> fdo::Error {
        |error| crate::util::map_error_from_node(&self.node, error)
    }
}

#[interface(name = "org.a11y.atspi.Accessible")]
impl NodeAccessibleInterface {
    #[zbus(property)]
    fn name(&self) -> fdo::Result<String> {
        self.node.name().map_err(self.map_error())
    }

    #[zbus(property)]
    fn description(&self) -> fdo::Result<String> {
        self.node.description().map_err(self.map_error())
    }

    #[zbus(property)]
    fn parent(&self) -> fdo::Result<OwnedObjectAddress> {
        self.node.parent().map_err(self.map_error()).map(|parent| {
            match parent {
                NodeIdOrRoot::Node(node) => ObjectId::Node {
                    adapter: self.node.adapter_id(),
                    node,
                },
                NodeIdOrRoot::Root => ObjectId::Root,
            }
            .to_address(self.bus_name.inner())
        })
    }

    #[zbus(property)]
    fn child_count(&self) -> fdo::Result<i32> {
        self.node.child_count().map_err(self.map_error())
    }

    #[zbus(property)]
    fn locale(&self) -> &str {
        ""
    }

    #[zbus(property)]
    fn accessible_id(&self) -> fdo::Result<String> {
        self.node.accessible_id().map_err(self.map_error())
    }

    fn get_child_at_index(&self, index: i32) -> fdo::Result<(OwnedObjectAddress,)> {
        let index = index
            .try_into()
            .map_err(|_| fdo::Error::InvalidArgs("Index can't be negative.".into()))?;
        let child = self
            .node
            .child_at_index(index)
            .map_err(self.map_error())?
            .map(|child| ObjectId::Node {
                adapter: self.node.adapter_id(),
                node: child,
            });
        Ok(super::optional_object_address(&self.bus_name, child))
    }

    fn get_children(&self) -> fdo::Result<Vec<OwnedObjectAddress>> {
        self.node
            .map_children(|child| {
                ObjectId::Node {
                    adapter: self.node.adapter_id(),
                    node: child,
                }
                .to_address(self.bus_name.inner())
            })
            .map_err(self.map_error())
    }

    fn get_index_in_parent(&self) -> fdo::Result<i32> {
        self.node.index_in_parent().map_err(self.map_error())
    }

    fn get_relation_set(&self) -> fdo::Result<Vec<(RelationType, Vec<OwnedObjectAddress>)>> {
        self.node
            .relation_set(|relation| {
                ObjectId::Node {
                    adapter: self.node.adapter_id(),
                    node: relation,
                }
                .to_address(self.bus_name.inner())
            })
            .map(|set| {
                set.into_iter()
                    .collect::<Vec<(RelationType, Vec<OwnedObjectAddress>)>>()
            })
            .map_err(self.map_error())
    }

    fn get_role(&self) -> fdo::Result<Role> {
        self.node.role().map_err(self.map_error())
    }

    fn get_localized_role_name(&self) -> fdo::Result<String> {
        self.node.localized_role_name().map_err(self.map_error())
    }

    fn get_state(&self) -> StateSet {
        self.node.state()
    }

    fn get_attributes(&self) -> fdo::Result<HashMap<&str, String>> {
        self.node.attributes().map_err(self.map_error())
    }

    fn get_application(&self) -> (OwnedObjectAddress,) {
        (ObjectId::Root.to_address(self.bus_name.inner()),)
    }

    fn get_interfaces(&self) -> fdo::Result<InterfaceSet> {
        self.node.interfaces().map_err(self.map_error())
    }
}

pub(crate) struct RootAccessibleInterface {
    bus_name: OwnedUniqueName,
    root: PlatformRoot,
}

impl RootAccessibleInterface {
    pub fn new(bus_name: OwnedUniqueName, root: PlatformRoot) -> Self {
        Self { bus_name, root }
    }
}

#[interface(name = "org.a11y.atspi.Accessible")]
impl RootAccessibleInterface {
    #[zbus(property)]
    fn name(&self) -> fdo::Result<String> {
        self.root.name().map_err(map_root_error)
    }

    #[zbus(property)]
    fn description(&self) -> &str {
        ""
    }

    #[zbus(property)]
    fn parent(&self) -> OwnedObjectAddress {
        OwnedObjectAddress::null()
    }

    #[zbus(property)]
    fn child_count(&self) -> fdo::Result<i32> {
        self.root.child_count().map_err(map_root_error)
    }

    #[zbus(property)]
    fn locale(&self) -> &str {
        ""
    }

    #[zbus(property)]
    fn accessible_id(&self) -> &str {
        ""
    }

    fn get_child_at_index(&self, index: i32) -> fdo::Result<(OwnedObjectAddress,)> {
        let index = index
            .try_into()
            .map_err(|_| fdo::Error::InvalidArgs("Index can't be negative.".into()))?;
        let child = self
            .root
            .child_id_at_index(index)
            .map_err(map_root_error)?
            .map(|(adapter, node)| ObjectId::Node { adapter, node });
        Ok(super::optional_object_address(&self.bus_name, child))
    }

    fn get_children(&self) -> fdo::Result<Vec<OwnedObjectAddress>> {
        self.root
            .map_child_ids(|(adapter, node)| {
                ObjectId::Node { adapter, node }.to_address(self.bus_name.inner())
            })
            .map_err(map_root_error)
    }

    fn get_index_in_parent(&self) -> i32 {
        -1
    }

    fn get_relation_set(&self) -> Vec<(RelationType, Vec<OwnedObjectAddress>)> {
        Vec::new()
    }

    fn get_role(&self) -> Role {
        Role::Application
    }

    fn get_state(&self) -> StateSet {
        StateSet::empty()
    }

    fn get_application(&self) -> (OwnedObjectAddress,) {
        (ObjectId::Root.to_address(self.bus_name.inner()),)
    }

    fn get_interfaces(&self) -> InterfaceSet {
        InterfaceSet::new(Interface::Accessible | Interface::Application)
    }
}
