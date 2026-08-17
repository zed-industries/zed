use crate::{error::*, notification::Notification, xdg};
use futures_lite::stream::StreamExt;
use zbus::MatchRule;

use super::bus::NotificationBus;
use crate::response::{CloseReason, NotificationResponse, ResponseHandler};

pub mod bus {

    use crate::xdg::NOTIFICATION_DEFAULT_BUS;

    fn skip_first_slash(s: &str) -> &str {
        if let Some('/') = s.chars().next() {
            &s[1..]
        } else {
            s
        }
    }

    use std::path::PathBuf;

    type BusNameType = zbus::names::WellKnownName<'static>;

    #[derive(Clone, Debug)]
    pub struct NotificationBus(BusNameType);

    impl Default for NotificationBus {
        #[cfg(feature = "zbus")]
        fn default() -> Self {
            Self(zbus::names::WellKnownName::from_static_str(NOTIFICATION_DEFAULT_BUS).unwrap())
        }
    }

    impl NotificationBus {
        fn namespaced_custom(custom_path: &str) -> Option<String> {
            // abusing path for semantic join
            skip_first_slash(
                PathBuf::from("/de/hoodie/Notification")
                    .join(custom_path)
                    .to_str()?,
            )
            .replace('/', ".")
            .into()
        }

        pub fn custom(custom_path: &str) -> Option<Self> {
            let name =
                zbus::names::WellKnownName::try_from(Self::namespaced_custom(custom_path)?).ok()?;
            Some(Self(name))
        }

        pub fn into_name(self) -> BusNameType {
            self.0
        }
    }
}

/// A handle to a shown notification.
///
/// This keeps a connection alive to ensure actions work on certain desktops.
#[derive(Debug)]
pub struct ZbusNotificationHandle {
    pub(crate) id: u32,
    pub(crate) connection: zbus::Connection,
    pub(crate) notification: Notification,
}

impl ZbusNotificationHandle {
    pub(crate) fn new(
        id: u32,
        connection: zbus::Connection,
        notification: Notification,
    ) -> ZbusNotificationHandle {
        ZbusNotificationHandle {
            id,
            connection,
            notification,
        }
    }

    pub async fn wait_for_action(&self, invocation_closure: impl ResponseHandler) {
        wait_for_action_signal(&self.connection, self.id, invocation_closure).await;
    }

    pub async fn close_fallible(&self) -> Result<()> {
        self.connection
            .call_method(
                Some(self.notification.bus.clone().into_name()),
                xdg::NOTIFICATION_OBJECTPATH,
                Some(xdg::NOTIFICATION_INTERFACE),
                "CloseNotification",
                &(self.id),
            )
            .await?;
        Ok(())
    }

    pub async fn close(&self) {
        let _ = self.close_fallible().await;
    }

    pub fn on_close<F>(self, closure: F)
    where
        F: FnOnce(CloseReason),
    {
        zbus::block_on(self.wait_for_action(|action: &NotificationResponse| {
            if let NotificationResponse::Closed(reason) = action {
                closure(*reason);
            }
        }));
    }

    pub fn update_fallible(&mut self) -> Result<()> {
        self.id = zbus::block_on(send_notification_via_connection(
            &self.notification,
            self.id,
            &self.connection,
        ))?;
        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        self.update_fallible()
    }
}

async fn send_notification_via_connection(
    notification: &Notification,
    id: u32,
    connection: &zbus::Connection,
) -> Result<u32> {
    send_notification_via_connection_at_bus(notification, id, connection, Default::default()).await
}

async fn send_notification_via_connection_at_bus(
    notification: &Notification,
    id: u32,
    connection: &zbus::Connection,
    bus: NotificationBus,
) -> Result<u32> {
    let reply: u32 = connection
        .call_method(
            Some(bus.into_name()),
            xdg::NOTIFICATION_OBJECTPATH,
            Some(xdg::NOTIFICATION_INTERFACE),
            "Notify",
            &(
                &notification.appname,
                id,
                &notification.icon,
                &notification.summary,
                &notification.body,
                &notification.actions,
                crate::hints::hints_to_map(notification),
                i32::from(notification.timeout),
            ),
        )
        .await?
        .body()
        .deserialize()?;
    Ok(reply)
}

pub async fn connect_and_send_notification(
    notification: &Notification,
) -> Result<ZbusNotificationHandle> {
    let bus = notification.bus.clone();
    connect_and_send_notification_at_bus(notification, bus).await
}

pub(crate) async fn connect_and_send_notification_at_bus(
    notification: &Notification,
    bus: NotificationBus,
) -> Result<ZbusNotificationHandle> {
    let connection = zbus::Connection::session().await?;
    let inner_id = notification.id.unwrap_or(0);
    let id =
        send_notification_via_connection_at_bus(notification, inner_id, &connection, bus).await?;

    Ok(ZbusNotificationHandle::new(
        id,
        connection,
        notification.clone(),
    ))
}

pub async fn get_capabilities_at_bus(bus: NotificationBus) -> Result<Vec<String>> {
    let connection = zbus::Connection::session().await?;
    let info: Vec<String> = connection
        .call_method(
            Some(bus.into_name()),
            xdg::NOTIFICATION_OBJECTPATH,
            Some(xdg::NOTIFICATION_INTERFACE),
            "GetCapabilities",
            &(),
        )
        .await?
        .body()
        .deserialize()?;
    Ok(info)
}

pub async fn get_capabilities() -> Result<Vec<String>> {
    get_capabilities_at_bus(Default::default()).await
}

pub async fn get_server_information_at_bus(bus: NotificationBus) -> Result<xdg::ServerInformation> {
    let connection = zbus::Connection::session().await?;
    let info: xdg::ServerInformation = connection
        .call_method(
            Some(bus.into_name()),
            xdg::NOTIFICATION_OBJECTPATH,
            Some(xdg::NOTIFICATION_INTERFACE),
            "GetServerInformation",
            &(),
        )
        .await?
        .body()
        .deserialize()?;

    Ok(info)
}

pub async fn get_server_information() -> Result<xdg::ServerInformation> {
    get_server_information_at_bus(Default::default()).await
}

/// Listens for the `ActionInvoked(UInt32, String)` Signal.
///
/// No need to use this, check out `Notification::show_and_wait_for_action(FnOnce(action:&str))`
pub async fn handle_action(id: u32, func: impl ResponseHandler) {
    let connection = zbus::Connection::session().await.unwrap();
    wait_for_action_signal(&connection, id, func).await;
}

async fn wait_for_action_signal(
    connection: &zbus::Connection,
    id: u32,
    handler: impl ResponseHandler,
) {
    let action_signal_rule = MatchRule::builder()
        .msg_type(zbus::message::Type::Signal)
        .interface(xdg::NOTIFICATION_INTERFACE)
        .unwrap()
        .member("ActionInvoked")
        .unwrap()
        .build();

    let proxy = zbus::fdo::DBusProxy::new(connection).await.unwrap();
    proxy.add_match_rule(action_signal_rule).await.unwrap();

    let close_signal_rule = MatchRule::builder()
        .msg_type(zbus::message::Type::Signal)
        .interface(xdg::NOTIFICATION_INTERFACE)
        .unwrap()
        .member("NotificationClosed")
        .unwrap()
        .build();
    proxy.add_match_rule(close_signal_rule).await.unwrap();

    while let Ok(Some(msg)) = zbus::MessageStream::from(connection).try_next().await {
        let header = msg.header();
        if let zbus::message::Type::Signal = header.message_type() {
            match header.member() {
                Some(name) if name == "ActionInvoked" => {
                    match msg.body().deserialize::<(u32, String)>() {
                        Ok((nid, action)) if nid == id => {
                            let response = if action == "default" {
                                NotificationResponse::Default
                            } else {
                                NotificationResponse::Action(action)
                            };
                            handler.call(&response);
                            break;
                        }
                        _ => {}
                    }
                }
                Some(name) if name == "NotificationClosed" => {
                    match msg.body().deserialize::<(u32, u32)>() {
                        Ok((nid, reason)) if nid == id => {
                            handler.call(&NotificationResponse::Closed(reason.into()));
                            break;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}
