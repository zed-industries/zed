// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use crate::{
    atspi::{interfaces::*, ObjectId},
    context::get_or_init_app_context,
    executor::{Executor, Task},
};
use accesskit_atspi_common::{
    NodeId, NodeIdOrRoot, ObjectEvent, PlatformNode, PlatformRoot, Property, WindowEvent,
};
use atspi::{
    events::EventBodyBorrowed,
    proxy::{bus::BusProxy, socket::SocketProxy},
    Interface, InterfaceSet,
};
use std::{env::var, io};
use zbus::{
    connection::Builder,
    names::{BusName, InterfaceName, MemberName, OwnedUniqueName},
    zvariant::{Str, Value},
    Address, Connection, Result,
};

pub(crate) struct Bus {
    conn: Connection,
    _task: Task<()>,
    socket_proxy: SocketProxy<'static>,
}

impl Bus {
    pub(crate) async fn new(
        session_bus: &Connection,
        executor: &Executor<'_>,
    ) -> zbus::Result<Self> {
        let address = match var("AT_SPI_BUS_ADDRESS") {
            Ok(address) if !address.is_empty() => address,
            _ => BusProxy::new(session_bus).await?.get_address().await?,
        };
        let address: Address = address.as_str().try_into()?;
        let conn = Builder::address(address)?
            .internal_executor(false)
            .build()
            .await?;
        let conn_copy = conn.clone();
        let _task = executor.spawn(
            async move {
                loop {
                    conn_copy.executor().tick().await;
                }
            },
            "accesskit_atspi_bus_task",
        );
        let socket_proxy = SocketProxy::new(&conn).await?;
        let mut bus = Bus {
            conn,
            _task,
            socket_proxy,
        };
        bus.register_root_node().await?;
        Ok(bus)
    }

    fn unique_name(&self) -> &OwnedUniqueName {
        self.conn.unique_name().unwrap()
    }

    async fn register_root_node(&mut self) -> Result<()> {
        let node = PlatformRoot::new(get_or_init_app_context());
        let path = ObjectId::Root.path();

        if self
            .conn
            .object_server()
            .at(path.clone(), ApplicationInterface(node.clone()))
            .await?
        {
            self.socket_proxy
                .embed(&(self.unique_name().as_str(), ObjectId::Root.path().into()))
                .await?;

            self.conn
                .object_server()
                .at(
                    path,
                    RootAccessibleInterface::new(self.unique_name().to_owned(), node),
                )
                .await?;
        }

        Ok(())
    }

    pub(crate) async fn register_interfaces(
        &self,
        node: PlatformNode,
        new_interfaces: InterfaceSet,
    ) -> zbus::Result<()> {
        let path = ObjectId::from(&node).path();
        let bus_name = self.unique_name().to_owned();
        if new_interfaces.contains(Interface::Accessible) {
            self.register_interface(
                &path,
                NodeAccessibleInterface::new(bus_name.clone(), node.clone()),
            )
            .await?;
        }
        if new_interfaces.contains(Interface::Action) {
            self.register_interface(&path, ActionInterface::new(node.clone()))
                .await?;
        }
        if new_interfaces.contains(Interface::Component) {
            self.register_interface(
                &path,
                ComponentInterface::new(bus_name.clone(), node.clone()),
            )
            .await?;
        }
        if new_interfaces.contains(Interface::Hyperlink) {
            self.register_interface(
                &path,
                HyperlinkInterface::new(bus_name.clone(), node.clone()),
            )
            .await?;
        }
        if new_interfaces.contains(Interface::Selection) {
            self.register_interface(
                &path,
                SelectionInterface::new(bus_name.clone(), node.clone()),
            )
            .await?;
        }
        if new_interfaces.contains(Interface::Text) {
            self.register_interface(&path, TextInterface::new(node.clone()))
                .await?;
        }
        if new_interfaces.contains(Interface::Value) {
            self.register_interface(&path, ValueInterface::new(node.clone()))
                .await?;
        }

        Ok(())
    }

    async fn register_interface<T>(&self, path: &str, interface: T) -> Result<bool>
    where
        T: zbus::object_server::Interface,
    {
        map_or_ignoring_broken_pipe(
            self.conn.object_server().at(path, interface).await,
            false,
            |result| result,
        )
    }

    pub(crate) async fn unregister_interfaces(
        &self,
        adapter_id: usize,
        node_id: NodeId,
        old_interfaces: InterfaceSet,
    ) -> zbus::Result<()> {
        let path = ObjectId::Node {
            adapter: adapter_id,
            node: node_id,
        }
        .path();
        if old_interfaces.contains(Interface::Accessible) {
            self.unregister_interface::<NodeAccessibleInterface>(&path)
                .await?;
        }
        if old_interfaces.contains(Interface::Action) {
            self.unregister_interface::<ActionInterface>(&path).await?;
        }
        if old_interfaces.contains(Interface::Component) {
            self.unregister_interface::<ComponentInterface>(&path)
                .await?;
        }
        if old_interfaces.contains(Interface::Hyperlink) {
            self.unregister_interface::<HyperlinkInterface>(&path)
                .await?;
        }
        if old_interfaces.contains(Interface::Selection) {
            self.unregister_interface::<SelectionInterface>(&path)
                .await?;
        }
        if old_interfaces.contains(Interface::Text) {
            self.unregister_interface::<TextInterface>(&path).await?;
        }
        if old_interfaces.contains(Interface::Value) {
            self.unregister_interface::<ValueInterface>(&path).await?;
        }

        Ok(())
    }

    async fn unregister_interface<T>(&self, path: &str) -> Result<bool>
    where
        T: zbus::object_server::Interface,
    {
        map_or_ignoring_broken_pipe(
            self.conn.object_server().remove::<T, _>(path).await,
            false,
            |result| result,
        )
    }

    pub(crate) async fn emit_object_event(
        &self,
        adapter_id: usize,
        target: NodeIdOrRoot,
        event: ObjectEvent,
    ) -> Result<()> {
        let target = match target {
            NodeIdOrRoot::Node(node) => ObjectId::Node {
                adapter: adapter_id,
                node,
            },
            NodeIdOrRoot::Root => ObjectId::Root,
        };
        let interface = "org.a11y.atspi.Event.Object";
        let signal = match event {
            ObjectEvent::ActiveDescendantChanged(_) => "ActiveDescendantChanged",
            ObjectEvent::Announcement(_, _) => "Announcement",
            ObjectEvent::BoundsChanged(_) => "BoundsChanged",
            ObjectEvent::CaretMoved(_) => "TextCaretMoved",
            ObjectEvent::ChildAdded(_, _) | ObjectEvent::ChildRemoved(_) => "ChildrenChanged",
            ObjectEvent::PropertyChanged(_) => "PropertyChange",
            ObjectEvent::SelectionChanged => "SelectionChanged",
            ObjectEvent::StateChanged(_, _) => "StateChanged",
            ObjectEvent::TextInserted { .. } | ObjectEvent::TextRemoved { .. } => "TextChanged",
            ObjectEvent::TextSelectionChanged => "TextSelectionChanged",
        };
        match event {
            ObjectEvent::ActiveDescendantChanged(child) => {
                let child = ObjectId::Node {
                    adapter: adapter_id,
                    node: child,
                };
                let mut body = EventBodyBorrowed::default();
                body.any_data = child.to_address(self.unique_name().inner()).into();
                self.emit_event(target, interface, signal, body).await
            }
            ObjectEvent::Announcement(message, politeness) => {
                let mut body = EventBodyBorrowed::default();
                body.detail1 = politeness as i32;
                body.any_data = message.into();
                self.emit_event(target, interface, signal, body).await
            }
            ObjectEvent::BoundsChanged(bounds) => {
                let mut body = EventBodyBorrowed::default();
                body.any_data = Value::from(bounds);
                self.emit_event(target, interface, signal, body).await
            }
            ObjectEvent::CaretMoved(offset) => {
                let mut body = EventBodyBorrowed::default();
                body.detail1 = offset;
                self.emit_event(target, interface, signal, body).await
            }
            ObjectEvent::ChildAdded(index, child) => {
                let child = ObjectId::Node {
                    adapter: adapter_id,
                    node: child,
                };
                let mut body = EventBodyBorrowed::default();
                body.kind = "add";
                body.detail1 = index as i32;
                body.any_data = child.to_address(self.unique_name().inner()).into();
                self.emit_event(target, interface, signal, body).await
            }
            ObjectEvent::ChildRemoved(child) => {
                let child = ObjectId::Node {
                    adapter: adapter_id,
                    node: child,
                };
                let mut body = EventBodyBorrowed::default();
                body.kind = "remove";
                body.detail1 = -1;
                body.any_data = child.to_address(self.unique_name().inner()).into();
                self.emit_event(target, interface, signal, body).await
            }
            ObjectEvent::PropertyChanged(property) => {
                let mut body = EventBodyBorrowed::default();
                body.kind = match property {
                    Property::Name(_) => "accessible-name",
                    Property::Description(_) => "accessible-description",
                    Property::Parent(_) => "accessible-parent",
                    Property::Role(_) => "accessible-role",
                    Property::Value(_) => "accessible-value",
                };
                body.any_data = match property {
                    Property::Name(value) => Str::from(value).into(),
                    Property::Description(value) => Str::from(value).into(),
                    Property::Parent(parent) => {
                        let parent = match parent {
                            NodeIdOrRoot::Node(node) => ObjectId::Node {
                                adapter: adapter_id,
                                node,
                            },
                            NodeIdOrRoot::Root => ObjectId::Root,
                        };
                        parent.to_address(self.unique_name().inner()).into()
                    }
                    Property::Role(value) => Value::U32(value as u32),
                    Property::Value(value) => Value::F64(value),
                };
                self.emit_event(target, interface, signal, body).await
            }
            ObjectEvent::SelectionChanged => {
                self.emit_event(target, interface, signal, EventBodyBorrowed::default())
                    .await
            }
            ObjectEvent::StateChanged(state, value) => {
                let mut body = EventBodyBorrowed::default();
                body.kind = state.to_static_str();
                body.detail1 = value as i32;
                self.emit_event(target, interface, signal, body).await
            }
            ObjectEvent::TextInserted {
                start_index,
                length,
                content,
            } => {
                let mut body = EventBodyBorrowed::default();
                body.kind = "insert";
                body.detail1 = start_index;
                body.detail2 = length;
                body.any_data = content.into();
                self.emit_event(target, interface, signal, body).await
            }
            ObjectEvent::TextRemoved {
                start_index,
                length,
                content,
            } => {
                let mut body = EventBodyBorrowed::default();
                body.kind = "delete";
                body.detail1 = start_index;
                body.detail2 = length;
                body.any_data = content.into();
                self.emit_event(target, interface, signal, body).await
            }
            ObjectEvent::TextSelectionChanged => {
                self.emit_event(target, interface, signal, EventBodyBorrowed::default())
                    .await
            }
        }
    }

    pub(crate) async fn emit_window_event(
        &self,
        adapter_id: usize,
        target: NodeId,
        window_name: String,
        event: WindowEvent,
    ) -> Result<()> {
        let target = ObjectId::Node {
            adapter: adapter_id,
            node: target,
        };
        let signal = match event {
            WindowEvent::Activated => "Activate",
            WindowEvent::Deactivated => "Deactivate",
        };
        let mut body = EventBodyBorrowed::default();
        body.any_data = window_name.into();
        self.emit_event(target, "org.a11y.atspi.Event.Window", signal, body)
            .await
    }

    async fn emit_event(
        &self,
        target: ObjectId,
        interface: &str,
        signal_name: &str,
        body: EventBodyBorrowed<'_>,
    ) -> Result<()> {
        map_or_ignoring_broken_pipe(
            self.conn
                .emit_signal(
                    Option::<BusName>::None,
                    target.path(),
                    InterfaceName::from_str_unchecked(interface),
                    MemberName::from_str_unchecked(signal_name),
                    &body,
                )
                .await,
            (),
            |_| (),
        )
    }
}

pub(crate) fn map_or_ignoring_broken_pipe<T, U, F>(
    result: zbus::Result<T>,
    default: U,
    f: F,
) -> zbus::Result<U>
where
    F: FnOnce(T) -> U,
{
    match result {
        Ok(result) => Ok(f(result)),
        Err(zbus::Error::InputOutput(error)) if error.kind() == io::ErrorKind::BrokenPipe => {
            Ok(default)
        }
        Err(error) => Err(error),
    }
}
