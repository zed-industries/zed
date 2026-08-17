use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    AppID, WindowIdentifierType,
    backend::{
        Result,
        request::{Request, RequestImpl},
    },
    desktop::{
        HandleToken,
        account::{UserInformation, UserInformationOptions},
        request::Response,
    },
    zvariant::{Optional, OwnedObjectPath},
};

#[async_trait]
pub trait AccountImpl: RequestImpl {
    #[doc(alias = "GetUserInformation")]
    async fn get_user_information(
        &self,
        token: HandleToken,
        app_id: Option<AppID>,
        window_identifier: Option<WindowIdentifierType>,
        options: UserInformationOptions,
    ) -> Result<UserInformation>;
}

pub(crate) struct AccountInterface {
    imp: Arc<dyn AccountImpl>,
    spawn: Arc<dyn futures_util::task::Spawn + Send + Sync>,
    cnx: zbus::Connection,
}

impl AccountInterface {
    pub fn new(
        imp: Arc<dyn AccountImpl>,
        cnx: zbus::Connection,
        spawn: Arc<dyn futures_util::task::Spawn + Send + Sync>,
    ) -> Self {
        Self { imp, cnx, spawn }
    }
}

#[zbus::interface(name = "org.freedesktop.impl.portal.Account")]
impl AccountInterface {
    #[zbus(property(emits_changed_signal = "const"), name = "version")]
    fn version(&self) -> u32 {
        1
    }

    #[zbus(name = "GetUserInformation")]
    #[zbus(out_args("response", "results"))]
    async fn get_user_information(
        &self,
        handle: OwnedObjectPath,
        app_id: Optional<AppID>,
        window_identifier: Optional<WindowIdentifierType>,
        options: UserInformationOptions,
    ) -> Result<Response<UserInformation>> {
        let imp = Arc::clone(&self.imp);

        Request::spawn(
            "Account::GetUserInformation",
            &self.cnx,
            handle.clone(),
            Arc::clone(&self.imp),
            Arc::clone(&self.spawn),
            async move {
                imp.get_user_information(
                    HandleToken::try_from(&handle).unwrap(),
                    app_id.into(),
                    window_identifier.into(),
                    options,
                )
                .await
            },
        )
        .await
    }
}
