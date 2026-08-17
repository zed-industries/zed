use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;

use crate::{
    AppID, PortalError,
    documents::{DocumentID, Permission},
    zbus::object_server::SignalEmitter,
    zvariant::{OwnedValue, Value},
};

#[async_trait]
pub trait PermissionStoreSignalEmitter: Send + Sync {
    async fn emit_document_changed(
        &self,
        table: &str,
        id: DocumentID,
        deleted: bool,
        data: Value<'_>,
        permissions: HashMap<AppID, Vec<Permission>>,
    ) -> zbus::Result<()>;
}

#[async_trait]
pub trait PermissionStoreImpl: Send + Sync {
    #[doc(alias = "Lookup")]
    async fn lookup(
        &self,
        table: &str,
        id: DocumentID,
    ) -> Result<(HashMap<AppID, Vec<Permission>>, OwnedValue), PortalError>;

    #[doc(alias = "Set")]
    async fn set(
        &self,
        table: &str,
        create: bool,
        id: DocumentID,
        app_permissions: HashMap<AppID, Vec<Permission>>,
        data: Value<'_>,
    ) -> Result<(), PortalError>;

    #[doc(alias = "Delete")]
    async fn delete(&self, table: &str, id: DocumentID) -> Result<(), PortalError>;

    #[doc(alias = "SetValue")]
    async fn set_value(
        &self,
        table: &str,
        create: bool,
        id: DocumentID,
        data: Value<'_>,
    ) -> Result<(), PortalError>;

    #[doc(alias = "List")]
    async fn list(&self, table: &str) -> Result<Vec<DocumentID>, PortalError>;

    #[doc(alias = "GetPermission")]
    async fn get_permission(
        &self,
        table: &str,
        id: DocumentID,
        app: AppID,
    ) -> Result<Vec<Permission>, PortalError>;

    #[doc(alias = "SetPermission")]
    async fn set_permission(
        &self,
        table: &str,
        create: bool,
        id: DocumentID,
        app: AppID,
        permissions: Vec<Permission>,
    ) -> Result<(), PortalError>;

    #[doc(alias = "DeletePermission")]
    async fn delete_permission(
        &self,
        table: &str,
        id: DocumentID,
        app: AppID,
    ) -> Result<(), PortalError>;

    // Set the signal emitter, allowing to notify of changes.
    fn set_signal_emitter(&mut self, signal_emitter: Arc<dyn PermissionStoreSignalEmitter>);
}

pub(crate) struct PermissionStoreInterface {
    imp: Arc<dyn PermissionStoreImpl>,
    #[allow(dead_code)]
    cnx: zbus::Connection,
    #[allow(dead_code)]
    spawn: Arc<dyn futures_util::task::Spawn + Send + Sync>,
}

impl PermissionStoreInterface {
    pub fn new(
        imp: Arc<dyn PermissionStoreImpl>,
        cnx: zbus::Connection,
        spawn: Arc<dyn futures_util::task::Spawn + Send + Sync>,
    ) -> Self {
        Self { imp, cnx, spawn }
    }

    pub async fn document_changed(
        &self,
        table: &str,
        id: DocumentID,
        deleted: bool,
        data: Value<'_>,
        permissions: HashMap<AppID, Vec<Permission>>,
    ) -> zbus::Result<()> {
        let object_server = self.cnx.object_server();
        let iface_ref = object_server
            .interface::<_, Self>(crate::proxy::DESKTOP_PATH)
            .await?;
        Self::changed(
            iface_ref.signal_emitter(),
            table,
            id,
            deleted,
            data,
            permissions,
        )
        .await
    }
}

#[async_trait]
impl PermissionStoreSignalEmitter for PermissionStoreInterface {
    #[doc(alias = "Changed")]
    async fn emit_document_changed(
        &self,
        table: &str,
        id: DocumentID,
        deleted: bool,
        data: Value<'_>,
        permissions: HashMap<AppID, Vec<Permission>>,
    ) -> zbus::Result<()> {
        self.document_changed(table, id, deleted, data, permissions)
            .await
    }
}

#[zbus::interface(name = "org.freedesktop.impl.portal.PermissionStore")]
impl PermissionStoreInterface {
    #[zbus(property(emits_changed_signal = "const"), name = "version")]
    fn version(&self) -> u32 {
        2
    }

    #[zbus(out_args("permissions", "data"))]
    async fn lookup(
        &self,
        table: &str,
        id: DocumentID,
    ) -> Result<(HashMap<AppID, Vec<Permission>>, OwnedValue), PortalError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("PermissionStore::Lookup");

        let response = self.imp.lookup(table, id).await;

        #[cfg(feature = "tracing")]
        tracing::debug!("PermissionStore::Lookup returned {:#?}", response);
        response
    }

    async fn set(
        &self,
        table: &str,
        create: bool,
        id: DocumentID,
        app_permissions: HashMap<AppID, Vec<Permission>>,
        data: Value<'_>,
    ) -> Result<(), PortalError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("PermissionStore::Set");

        let response = self.imp.set(table, create, id, app_permissions, data).await;

        #[cfg(feature = "tracing")]
        tracing::debug!("PermissionStore::Set returned {:#?}", response);
        response
    }

    async fn set_value(
        &self,
        table: &str,
        create: bool,
        id: DocumentID,
        data: Value<'_>,
    ) -> Result<(), PortalError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("PermissionStore::SetValue");

        let response = self.imp.set_value(table, create, id, data).await;

        #[cfg(feature = "tracing")]
        tracing::debug!("PermissionStore::SetValue returned {:#?}", response);
        response
    }

    #[zbus(out_args("ids"))]
    async fn list(&self, table: &str) -> Result<Vec<DocumentID>, PortalError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("PermissionStore::List");

        let response = self.imp.list(table).await;

        #[cfg(feature = "tracing")]
        tracing::debug!("PermissionStore::List returned {:#?}", response);
        response
    }

    #[zbus(out_args("permissions"))]
    async fn get_permission(
        &self,
        table: &str,
        id: DocumentID,
        app: AppID,
    ) -> Result<Vec<Permission>, PortalError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("PermissionStore::GetPermission");

        let response = self.imp.get_permission(table, id, app).await;

        #[cfg(feature = "tracing")]
        tracing::debug!("PermissionStore::GetPermission returned {:#?}", response);
        response
    }

    async fn set_permission(
        &self,
        table: &str,
        create: bool,
        id: DocumentID,
        app: AppID,
        permissions: Vec<Permission>,
    ) -> Result<(), PortalError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("PermissionStore::SetPermission");

        let response = self
            .imp
            .set_permission(table, create, id, app, permissions)
            .await;

        #[cfg(feature = "tracing")]
        tracing::debug!("PermissionStore::SetPermission returned {:#?}", response);
        response
    }

    async fn delete_permission(
        &self,
        table: &str,
        id: DocumentID,
        app: AppID,
    ) -> Result<(), PortalError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("PermissionStore::DeletePermission");

        let response = self.imp.delete_permission(table, id, app).await;

        #[cfg(feature = "tracing")]
        tracing::debug!("PermissionStore::DeletePermission returned {:#?}", response);
        response
    }

    async fn delete(&self, table: &str, id: DocumentID) -> Result<(), PortalError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("PermissionStore::Delete");

        let response = self.imp.delete(table, id).await;

        #[cfg(feature = "tracing")]
        tracing::debug!("PermissionStore::Delete returned {:#?}", response);
        response
    }

    #[zbus(signal)]
    async fn changed(
        signal_ctxt: &SignalEmitter<'_>,
        table: &str,
        id: DocumentID,
        deleted: bool,
        data: Value<'_>,
        permissions: HashMap<AppID, Vec<Permission>>,
    ) -> zbus::Result<()>;
}
