use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    AppID, WindowIdentifierType,
    backend::{
        Result,
        request::{Request, RequestImpl},
    },
    desktop::{HandleToken, Icon, request::Response},
    zvariant::{
        self, Optional, OwnedObjectPath,
        as_value::{self, optional},
    },
};

#[derive(Deserialize, zvariant::Type)]
#[zvariant(signature = "dict")]
pub struct AccessOptions {
    #[serde(default, with = "optional")]
    modal: Option<bool>,
    #[serde(default, with = "optional")]
    deny_label: Option<String>,
    #[serde(default, with = "optional")]
    grant_label: Option<String>,
    #[serde(default, with = "optional")]
    icon: Option<String>,
    #[serde(default, with = "as_value")]
    choices: Vec<Choice>,
}

#[derive(Clone, Deserialize, zvariant::Type, Debug)]
pub struct Choice(String, String, Vec<(String, String)>, String);

impl Choice {
    /// The choice's unique id
    pub fn id(&self) -> &str {
        &self.0
    }

    /// The user visible label of the choice.
    pub fn label(&self) -> &str {
        &self.1
    }

    /// Pairs of choices.
    pub fn pairs(&self) -> Vec<(&str, &str)> {
        self.2
            .iter()
            .map(|(x, y)| (x.as_str(), y.as_str()))
            .collect::<Vec<_>>()
    }

    /// The initially selected value.
    pub fn initial_selection(&self) -> &str {
        &self.3
    }
}

impl AccessOptions {
    pub fn is_modal(&self) -> Option<bool> {
        self.modal
    }

    pub fn deny_label(&self) -> Option<&str> {
        self.deny_label.as_deref()
    }

    pub fn grant_label(&self) -> Option<&str> {
        self.grant_label.as_deref()
    }

    pub fn icon(&self) -> Option<Icon> {
        self.icon.as_ref().map(|i| Icon::with_names([i]))
    }

    pub fn choices(&self) -> &[Choice] {
        &self.choices
    }
}

#[derive(Serialize, Debug, zvariant::Type, Default)]
#[zvariant(signature = "dict")]
pub struct AccessResponse {
    #[serde(with = "as_value", skip_serializing_if = "Vec::is_empty")]
    choices: Vec<(String, String)>,
}

impl AccessResponse {
    /// Adds a selected choice (key, value).
    #[must_use]
    pub fn choice(mut self, key: &str, value: &str) -> Self {
        self.choices.push((key.to_owned(), value.to_owned()));
        self
    }
}

#[async_trait]
pub trait AccessImpl: RequestImpl {
    #[allow(clippy::too_many_arguments)]
    #[doc(alias = "AccessDialog")]
    async fn access_dialog(
        &self,
        token: HandleToken,
        app_id: Option<AppID>,
        window_identifier: Option<WindowIdentifierType>,
        title: String,
        subtitle: String,
        body: String,
        options: AccessOptions,
    ) -> Result<AccessResponse>;
}

pub(crate) struct AccessInterface {
    imp: Arc<dyn AccessImpl>,
    spawn: Arc<dyn futures_util::task::Spawn + Send + Sync>,
    cnx: zbus::Connection,
}

impl AccessInterface {
    pub fn new(
        imp: Arc<dyn AccessImpl>,
        cnx: zbus::Connection,
        spawn: Arc<dyn futures_util::task::Spawn + Send + Sync>,
    ) -> Self {
        Self { imp, cnx, spawn }
    }
}

#[zbus::interface(name = "org.freedesktop.impl.portal.Access")]
impl AccessInterface {
    #[zbus(property(emits_changed_signal = "const"), name = "version")]
    fn version(&self) -> u32 {
        1 // TODO: Is this correct?
    }

    #[allow(clippy::too_many_arguments)]
    #[zbus(out_args("response", "results"))]
    async fn access_dialog(
        &self,
        handle: OwnedObjectPath,
        app_id: Optional<AppID>,
        window_identifier: Optional<WindowIdentifierType>,
        title: String,
        subtitle: String,
        body: String,
        options: AccessOptions,
    ) -> Result<Response<AccessResponse>> {
        let imp = Arc::clone(&self.imp);

        Request::spawn(
            "Access::AccessDialog",
            &self.cnx,
            handle.clone(),
            Arc::clone(&self.imp),
            Arc::clone(&self.spawn),
            async move {
                imp.access_dialog(
                    HandleToken::try_from(&handle).unwrap(),
                    app_id.into(),
                    window_identifier.into(),
                    title,
                    subtitle,
                    body,
                    options,
                )
                .await
            },
        )
        .await
    }
}
