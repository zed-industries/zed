//! The object server API.

use std::{
    collections::{BTreeMap, HashMap, btree_map, hash_map},
    fmt::Write,
};

use zbus_names::InterfaceName;
use zvariant::{ObjectPath, OwnedObjectPath, OwnedValue};

use crate::{
    Connection, ObjectServer,
    fdo::{self, Introspectable, ManagedObjects, ObjectManager, Peer, Properties},
    object_server::SignalEmitter,
};

use super::{ArcInterface, Interface};

#[derive(Default, Debug)]
pub(crate) struct Node {
    path: OwnedObjectPath,
    children: HashMap<String, Node>,
    interfaces: BTreeMap<InterfaceName<'static>, ArcInterface>,
}

impl Node {
    pub(crate) fn new(path: OwnedObjectPath) -> Self {
        let mut node = Self {
            path,
            ..Default::default()
        };
        assert!(node.add_interface(Peer));
        assert!(node.add_interface(Introspectable));
        assert!(node.add_interface(Properties));

        node
    }

    // Get the child Node at path.
    pub(crate) fn get_child(&self, path: &ObjectPath<'_>) -> Option<&Node> {
        let mut node = self;

        for i in path.split('/').skip(1) {
            if i.is_empty() {
                continue;
            }
            match node.children.get(i) {
                Some(n) => node = n,
                None => return None,
            }
        }

        Some(node)
    }

    /// Get the child Node at path. Optionally create one if it doesn't exist.
    ///
    /// This also returns the path of the parent node that implements ObjectManager (if any). If
    /// multiple parents implement it (they shouldn't), then the closest one is returned.
    pub(super) fn get_child_mut(
        &mut self,
        path: &ObjectPath<'_>,
        create: bool,
    ) -> (Option<&mut Node>, Option<ObjectPath<'_>>) {
        let mut node = self;
        let mut node_path = String::new();
        let mut obj_manager_path = None;

        for i in path.split('/').skip(1) {
            if i.is_empty() {
                continue;
            }

            if node.interfaces.contains_key(&ObjectManager::name()) {
                obj_manager_path = Some((*node.path).clone());
            }

            write!(&mut node_path, "/{i}").unwrap();
            match node.children.entry(i.into()) {
                hash_map::Entry::Vacant(e) => {
                    if create {
                        let path = node_path.as_str().try_into().expect("Invalid Object Path");
                        node = e.insert(Node::new(path));
                    } else {
                        return (None, obj_manager_path);
                    }
                }
                hash_map::Entry::Occupied(e) => node = e.into_mut(),
            }
        }

        (Some(node), obj_manager_path)
    }

    pub(crate) fn interface_lock(&self, interface_name: InterfaceName<'_>) -> Option<ArcInterface> {
        self.interfaces.get(&interface_name).cloned()
    }

    pub(super) fn remove_interface(&mut self, interface_name: InterfaceName<'static>) -> bool {
        self.interfaces.remove(&interface_name).is_some()
    }

    pub(super) fn is_empty(&self) -> bool {
        !self.interfaces.keys().any(|k| {
            *k != Peer::name()
                && *k != Introspectable::name()
                && *k != Properties::name()
                && *k != ObjectManager::name()
        })
    }

    pub(super) fn remove_node(&mut self, node: &str) -> bool {
        self.children.remove(node).is_some()
    }

    pub(super) fn add_arc_interface(
        &mut self,
        name: InterfaceName<'static>,
        arc_iface: ArcInterface,
    ) -> bool {
        match self.interfaces.entry(name) {
            btree_map::Entry::Vacant(e) => {
                e.insert(arc_iface);
                true
            }
            btree_map::Entry::Occupied(_) => false,
        }
    }

    fn add_interface<I>(&mut self, iface: I) -> bool
    where
        I: Interface,
    {
        self.add_arc_interface(I::name(), ArcInterface::new(iface))
    }

    async fn introspect_to_writer<W: Write + Send>(&self, writer: &mut W) {
        enum Fragment<'a> {
            /// Represent an unclosed node tree, could be further splitted into sub-`Fragment`s.
            Node {
                name: &'a str,
                node: &'a Node,
                level: usize,
            },
            /// Represent a closing `</node>`.
            End { level: usize },
        }

        let mut stack = Vec::new();
        stack.push(Fragment::Node {
            name: "",
            node: self,
            level: 0,
        });

        // This can be seen as traversing the fragment tree in pre-order DFS with formatted XML
        // fragment, splitted `Fragment::Node`s and `Fragment::End` being current node, left
        // subtree and right leaf respectively.
        while let Some(fragment) = stack.pop() {
            match fragment {
                Fragment::Node { name, node, level } => {
                    stack.push(Fragment::End { level });

                    for (name, node) in &node.children {
                        stack.push(Fragment::Node {
                            name,
                            node,
                            level: level + 2,
                        })
                    }

                    if level == 0 {
                        writeln!(
                            writer,
                            r#"
<!DOCTYPE node PUBLIC "-//freedesktop//DTD D-BUS Object Introspection 1.0//EN"
 "http://www.freedesktop.org/standards/dbus/1.0/introspect.dtd">
<node>"#
                        )
                        .unwrap();
                    } else {
                        writeln!(
                            writer,
                            "{:indent$}<node name=\"{}\">",
                            "",
                            name,
                            indent = level
                        )
                        .unwrap();
                    }

                    for iface in node.interfaces.values() {
                        iface
                            .instance
                            .read()
                            .await
                            .introspect_to_writer(writer, level + 2);
                    }
                }
                Fragment::End { level } => {
                    writeln!(writer, "{:indent$}</node>", "", indent = level).unwrap();
                }
            }
        }
    }

    pub(crate) async fn introspect(&self) -> String {
        let mut xml = String::with_capacity(1024);

        self.introspect_to_writer(&mut xml).await;

        xml
    }

    pub(crate) async fn get_managed_objects(
        &self,
        object_server: &ObjectServer,
        connection: &Connection,
    ) -> fdo::Result<ManagedObjects> {
        let mut managed_objects = ManagedObjects::new();

        // Recursively get all properties of all interfaces of descendants.
        let mut node_list: Vec<_> = self.children.values().collect();
        while let Some(node) = node_list.pop() {
            let mut interfaces = HashMap::new();
            for iface_name in node.interfaces.keys().filter(|n| {
                // Filter standard interfaces.
                *n != &Peer::name()
                    && *n != &Introspectable::name()
                    && *n != &Properties::name()
                    && *n != &ObjectManager::name()
            }) {
                let props = node
                    .get_properties(object_server, connection, iface_name.clone())
                    .await?;
                interfaces.insert(iface_name.clone().into(), props);
            }
            managed_objects.insert(node.path.clone(), interfaces);
            node_list.extend(node.children.values());
        }

        Ok(managed_objects)
    }

    pub(super) async fn get_properties(
        &self,
        object_server: &ObjectServer,
        connection: &Connection,
        interface_name: InterfaceName<'_>,
    ) -> fdo::Result<HashMap<String, OwnedValue>> {
        let emitter = SignalEmitter::new(connection, self.path.clone())?;
        self.interface_lock(interface_name)
            .expect("Interface was added but not found")
            .instance
            .read()
            .await
            .get_all(object_server, connection, None, &emitter)
            .await
    }
}
