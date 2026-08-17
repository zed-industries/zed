use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use zbus::zvariant::{self, OwnedValue};

use crate::{
    AppID,
    backend::{
        Result,
        request::{Request, RequestImpl},
    },
    desktop::{HandleToken, Response},
};

#[async_trait]
pub trait SecretImpl: RequestImpl {
    #[doc(alias = "RetrieveSecret")]
    async fn retrieve(
        &self,
        token: HandleToken,
        app_id: AppID,
        fd: std::os::fd::OwnedFd,
    ) -> Result<HashMap<String, OwnedValue>>;
}

pub(crate) struct SecretInterface {
    imp: Arc<dyn SecretImpl>,
    spawn: Arc<dyn futures_util::task::Spawn + Send + Sync>,
    cnx: zbus::Connection,
}

impl SecretInterface {
    pub fn new(
        imp: Arc<dyn SecretImpl>,
        cnx: zbus::Connection,
        spawn: Arc<dyn futures_util::task::Spawn + Send + Sync>,
    ) -> Self {
        Self { imp, cnx, spawn }
    }
}

#[zbus::interface(name = "org.freedesktop.impl.portal.Secret")]
impl SecretInterface {
    #[zbus(property(emits_changed_signal = "const"), name = "version")]
    fn version(&self) -> u32 {
        1
    }

    #[zbus(out_args("response", "results"))]
    async fn retrieve_secret(
        &self,
        handle: zvariant::OwnedObjectPath,
        app_id: AppID,
        fd: zvariant::OwnedFd,
        _options: HashMap<String, OwnedValue>,
    ) -> Result<Response<HashMap<String, OwnedValue>>> {
        let imp = Arc::clone(&self.imp);

        Request::spawn(
            "Secret::RetrieveSecret",
            &self.cnx,
            handle.clone(),
            Arc::clone(&self.imp),
            Arc::clone(&self.spawn),
            async move {
                imp.retrieve(
                    HandleToken::try_from(&handle).unwrap(),
                    app_id,
                    std::os::fd::OwnedFd::from(fd),
                )
                .await
            },
        )
        .await
    }
}
