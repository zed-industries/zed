use std::fmt::Debug;

use zbus::export::futures_util::{Stream, StreamExt};
use zbus::export::serde::Deserialize;
use zbus::proxy::SignalStream;
use zbus::zvariant::Type;

pub struct StatusNotifierWatcher<'a>(zbus::Proxy<'a>);

const STATUS_NOTIFIER_WATCHER_INTERFACE: &str = "org.kde.StatusNotifierWatcher";
const STATUS_NOTIFIER_WATCHER_PATH: &str = "/StatusNotifierWatcher";
const STATUS_NOTIFIER_WATCHER_DESTINATION: &str = "org.kde.StatusNotifierWatcher";

#[allow(dead_code)]
impl<'a> StatusNotifierWatcher<'a> {
    pub async fn new() -> zbus::Result<Self> {
        let conn = zbus::Connection::session().await?;
        let proxy: zbus::Proxy = zbus::ProxyBuilder::new(&conn)
            .interface(STATUS_NOTIFIER_WATCHER_INTERFACE)?
            .path(STATUS_NOTIFIER_WATCHER_PATH)?
            .destination(STATUS_NOTIFIER_WATCHER_DESTINATION)?
            .build()
            .await?;
        Ok(Self(proxy))
    }

    pub async fn register_status_notifier_item(
        &self,
        service: impl Into<String>,
    ) -> zbus::Result<()> {
        self.0
            .connection()
            .call_method(
                Some(STATUS_NOTIFIER_WATCHER_DESTINATION),
                STATUS_NOTIFIER_WATCHER_PATH,
                Some(STATUS_NOTIFIER_WATCHER_INTERFACE),
                "RegisterStatusNotifierItem",
                &(service.into()),
            )
            .await?;
        Ok(())
    }

    pub async fn register_status_notifier_host(
        &self,
        service: impl Into<String>,
    ) -> zbus::Result<()> {
        self.0
            .connection()
            .call_method(
                Some(STATUS_NOTIFIER_WATCHER_DESTINATION),
                STATUS_NOTIFIER_WATCHER_PATH,
                Some(STATUS_NOTIFIER_WATCHER_INTERFACE),
                "RegisterStatusNotifierHost",
                &(service.into()),
            )
            .await?;
        Ok(())
    }

    pub async fn registered_status_notifier_items(&self) -> zbus::Result<Vec<String>> {
        Ok(self.0.get_property("RegisteredStatusNotifierItems").await?)
    }

    pub async fn is_status_notifier_host_registered(&self) -> zbus::Result<bool> {
        Ok(self
            .0
            .get_property("IsStatusNotifierHostRegistered")
            .await?)
    }

    pub async fn protocol_version(&self) -> zbus::Result<bool> {
        Ok(self.0.get_property("ProtocolVersion").await?)
    }

    pub async fn receive_all_signals(&self) -> zbus::Result<SignalStream<'static>> {
        self.0.receive_all_signals().await
    }

    pub async fn receive_status_notifier_item_registered(
        &self,
    ) -> zbus::Result<impl Stream<Item = bool>> {
        self.receive_signal("StatusNotifierItemRegistered").await
    }

    pub async fn receive_status_notifier_item_unregistered(
        &self,
    ) -> zbus::Result<impl Stream<Item = bool>> {
        self.receive_signal("StatusNotifierItemUnregistered").await
    }

    pub async fn receive_status_notifier_host_registered(
        &self,
    ) -> zbus::Result<impl Stream<Item = bool>> {
        self.receive_signal("StatusNotifierHostRegistered").await
    }

    async fn receive_signal<R>(&self, name: &'static str) -> zbus::Result<impl Stream<Item = R>>
    where
        R: for<'de> Deserialize<'de> + Type + Debug,
    {
        let stream = self.0.receive_signal(name).await?;
        Ok(stream.filter_map(move |msg| core::future::ready(msg.body().deserialize().ok())))
    }
}
