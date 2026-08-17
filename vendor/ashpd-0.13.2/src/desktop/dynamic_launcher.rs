//! Install launchers like Web Application from your browser or Steam.
//!
//! # Examples
//!
//! ```rust,no_run
//! use std::io::Read;
//! use ashpd::{
//!     desktop::{
//!         dynamic_launcher::DynamicLauncherProxy,
//!         Icon,
//!     },
//!     WindowIdentifier,
//! };
//!
//! async fn run() -> ashpd::Result<()> {
//!     let proxy = DynamicLauncherProxy::new().await?;
//!
//!     let filename = "/home/bilalelmoussaoui/Projects/ashpd/ashpd-demo/data/icons/com.belmoussaoui.ashpd.demo.svg";
//!     let mut f = std::fs::File::open(&filename).expect("no file found");
//!     let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
//!     let mut buffer = vec![0; metadata.len() as usize];
//!     f.read(&mut buffer).expect("buffer overflow");
//!
//!     let icon = Icon::Bytes(buffer);
//!     let response = proxy
//!         .prepare_install(
//!             None,
//!             "SomeApp",
//!             icon,
//!             Default::default()
//!         )
//!         .await?
//!         .response()?;
//!     let token = response.token();
//!
//!
//!     // Name and Icon will be overwritten from what we provided above
//!     // Exec will be overridden to call `flatpak run our-app` if the application is sandboxed
//!     let desktop_entry = r#"
//!         [Desktop Entry]
//!         Comment=My Web App
//!         Type=Application
//!     "#;
//!     proxy
//!         .install(&token, "some_file.desktop", desktop_entry, Default::default())
//!         .await?;
//!
//!     proxy.uninstall("some_file.desktop", Default::default()).await?;
//!     Ok(())
//! }
//! ```

use enumflags2::{BitFlags, bitflags};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use zbus::zvariant::{
    self, Optional, OwnedValue, Type, Value,
    as_value::{self, optional},
};

use super::{HandleToken, Icon, Request};
use crate::{ActivationToken, Error, WindowIdentifier, proxy::Proxy};

#[bitflags]
#[derive(Default, Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug, Copy, Clone, Type)]
#[repr(u32)]
#[doc(alias = "XdpLauncherType")]
/// The type of the launcher.
pub enum LauncherType {
    #[doc(alias = "XDP_LAUNCHER_APPLICATION")]
    #[default]
    /// A launcher that represents an application
    Application,
    #[doc(alias = "XDP_LAUNCHER_WEBAPP")]
    /// A launcher that represents a web application
    WebApplication,
}

#[cfg_attr(feature = "glib", derive(glib::Enum))]
#[cfg_attr(feature = "glib", enum_type(name = "AshpdIconType"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Type)]
#[zvariant(signature = "s")]
#[serde(rename_all = "lowercase")]
/// The icon format.
pub enum IconType {
    /// PNG.
    Png,
    /// JPEG.
    Jpeg,
    /// SVG.
    Svg,
}

#[derive(Debug, Deserialize, Type)]
#[zvariant(signature = "(vsu)")]
/// The icon of the launcher.
pub struct LauncherIcon(zvariant::OwnedValue, IconType, u32);

impl LauncherIcon {
    /// The actual icon.
    pub fn icon(&self) -> Icon {
        Icon::try_from(&self.0).unwrap()
    }

    /// The icon type.
    pub fn type_(&self) -> IconType {
        self.1
    }

    /// The icon size.
    pub fn size(&self) -> u32 {
        self.2
    }
}

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Options to pass to [`DynamicLauncherProxy::prepare_install`]
pub struct PrepareInstallOptions {
    #[serde(with = "as_value")]
    handle_token: HandleToken,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    modal: Option<bool>,
    #[serde(with = "as_value")]
    launcher_type: LauncherType,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    target: Option<String>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    editable_name: Option<bool>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    editable_icon: Option<bool>,
}

impl PrepareInstallOptions {
    /// Sets whether the dialog should be a modal.
    pub fn set_modal(mut self, modal: impl Into<Option<bool>>) -> Self {
        self.modal = modal.into();
        self
    }

    /// Sets the launcher type.
    pub fn set_launcher_type(mut self, launcher_type: LauncherType) -> Self {
        self.launcher_type = launcher_type;
        self
    }

    /// The URL for a [`LauncherType::WebApplication`] otherwise it is not
    /// needed.
    pub fn set_target<'a>(mut self, target: impl Into<Option<&'a str>>) -> Self {
        self.target = target.into().map(ToOwned::to_owned);
        self
    }

    /// Sets whether the name should be editable.
    pub fn set_editable_name(mut self, editable_name: impl Into<Option<bool>>) -> Self {
        self.editable_name = editable_name.into();
        self
    }

    /// Sets whether the icon should be editable.
    pub fn set_editable_icon(mut self, editable_icon: impl Into<Option<bool>>) -> Self {
        self.editable_icon = editable_icon.into();
        self
    }
}

#[derive(Deserialize, Type)]
#[zvariant(signature = "dict")]
/// A response of [`DynamicLauncherProxy::prepare_install`]
pub struct PrepareInstallResponse {
    #[serde(with = "as_value")]
    name: String,
    #[serde(with = "as_value")]
    icon: OwnedValue,
    #[serde(with = "as_value")]
    token: String,
}

impl PrepareInstallResponse {
    /// The user defined name or a predefined one
    pub fn name(&self) -> &str {
        &self.name
    }

    /// A token to pass to [`DynamicLauncherProxy::install`]
    pub fn token(&self) -> &str {
        &self.token
    }

    /// The user selected icon or a predefined one
    pub fn icon(&self) -> Icon {
        let inner = self.icon.downcast_ref::<Value>().unwrap();
        Icon::try_from(inner).unwrap()
    }
}

impl std::fmt::Debug for PrepareInstallResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrepareInstallResponse")
            .field("name", &self.name())
            .field("icon", &self.icon())
            .field("token", &self.token())
            .finish()
    }
}

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Options to pass to [`DynamicLauncherProxy::launch`]
pub struct LaunchOptions {
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    activation_token: Option<ActivationToken>,
}

impl LaunchOptions {
    /// Sets the token that can be used to activate the chosen application.
    #[must_use]
    pub fn activation_token(
        mut self,
        activation_token: impl Into<Option<ActivationToken>>,
    ) -> Self {
        self.activation_token = activation_token.into();
        self
    }
}

#[derive(Serialize, Type, Debug, Default)]
/// Specified options for a [`DynamicLauncherProxy::install`] request.
#[zvariant(signature = "dict")]
pub struct InstallOptions {}

#[derive(Serialize, Type, Debug, Default)]
/// Specified options for a [`DynamicLauncherProxy::uninstall`] request.
#[zvariant(signature = "dict")]
pub struct UninstallOptions {}

#[derive(Serialize, Type, Debug, Default)]
/// Specified options for a [`DynamicLauncherProxy::request_install_token`]
/// request.
#[zvariant(signature = "dict")]
pub struct RequestInstallTokenOptions {}

#[derive(Debug)]
/// Wrong type of [`crate::desktop::Icon`] was used.
pub struct UnexpectedIconError;

impl std::error::Error for UnexpectedIconError {}
impl std::fmt::Display for UnexpectedIconError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Unexpected icon type. Only Icon::Bytes is supported")
    }
}

/// The interface lets sandboxed applications install launchers like Web
/// Application from your browser or Steam.
///
/// Wrapper of the DBus interface: [`org.freedesktop.portal.DynamicLauncher`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.DynamicLauncher.html).
#[derive(Debug)]
#[doc(alias = "org.freedesktop.portal.DynamicLauncher")]
pub struct DynamicLauncherProxy(Proxy<'static>);

impl DynamicLauncherProxy {
    /// Create a new instance of [`DynamicLauncherProxy`].
    pub async fn new() -> Result<Self, Error> {
        let proxy = Proxy::new_desktop("org.freedesktop.portal.DynamicLauncher").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`DynamicLauncherProxy`].
    pub async fn with_connection(connection: zbus::Connection) -> Result<Self, Error> {
        let proxy = Proxy::new_desktop_with_connection(
            connection,
            "org.freedesktop.portal.DynamicLauncher",
        )
        .await?;
        Ok(Self(proxy))
    }

    /// Returns the version of the portal interface.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    /// *Note* Only `Icon::Bytes` is accepted.
    ///
    ///  # Specifications
    ///
    /// See also [`PrepareInstall`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.DynamicLauncher.html#org-freedesktop-portal-dynamiclauncher-prepareinstall).
    #[doc(alias = "PrepareInstall")]
    #[doc(alias = "xdp_portal_dynamic_launcher_prepare_install")]
    #[doc(alias = "xdp_portal_dynamic_launcher_prepare_install_finish")]
    pub async fn prepare_install(
        &self,
        identifier: Option<&WindowIdentifier>,
        name: &str,
        icon: Icon,
        options: PrepareInstallOptions,
    ) -> Result<Request<PrepareInstallResponse>, Error> {
        if !icon.is_bytes() {
            return Err(UnexpectedIconError {}.into());
        }
        let identifier = Optional::from(identifier);
        self.0
            .request(
                &options.handle_token,
                "PrepareInstall",
                &(identifier, name, icon.as_value(), &options),
            )
            .await
    }

    /// *Note* Only `Icon::Bytes` is accepted.
    ///
    /// # Specifications
    ///
    /// See also [`RequestInstallToken`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.DynamicLauncher.html#org-freedesktop-portal-dynamiclauncher-requestinstalltoken).
    #[doc(alias = "RequestInstallToken")]
    #[doc(alias = "xdp_portal_dynamic_launcher_request_install_token")]
    pub async fn request_install_token(
        &self,
        name: &str,
        icon: Icon,
        options: RequestInstallTokenOptions,
    ) -> Result<String, Error> {
        if !icon.is_bytes() {
            return Err(UnexpectedIconError {}.into());
        }

        self.0
            .call::<String>("RequestInstallToken", &(name, icon.as_value(), options))
            .await
    }

    /// # Specifications
    ///
    /// See also [`Install`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.DynamicLauncher.html#org-freedesktop-portal-dynamiclauncher-install).
    #[doc(alias = "Install")]
    #[doc(alias = "xdp_portal_dynamic_launcher_install")]
    pub async fn install(
        &self,
        token: &str,
        desktop_file_id: &str,
        desktop_entry: &str,
        options: InstallOptions,
    ) -> Result<(), Error> {
        self.0
            .call::<()>("Install", &(token, desktop_file_id, desktop_entry, options))
            .await
    }

    /// # Specifications
    ///
    /// See also [`Uninstall`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.DynamicLauncher.html#org-freedesktop-portal-dynamiclauncher-uninstall).
    #[doc(alias = "Uninstall")]
    #[doc(alias = "xdp_portal_dynamic_launcher_uninstall")]
    pub async fn uninstall(
        &self,
        desktop_file_id: &str,
        options: UninstallOptions,
    ) -> Result<(), Error> {
        self.0
            .call::<()>("Uninstall", &(desktop_file_id, options))
            .await
    }

    /// # Specifications
    ///
    /// See also [`GetDesktopEntry`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.DynamicLauncher.html#org-freedesktop-portal-dynamiclauncher-getdesktopentry).
    #[doc(alias = "GetDesktopEntry")]
    #[doc(alias = "xdp_portal_dynamic_launcher_get_desktop_entry")]
    pub async fn desktop_entry(&self, desktop_file_id: &str) -> Result<String, Error> {
        self.0.call("GetDesktopEntry", &(desktop_file_id)).await
    }

    /// # Specifications
    ///
    /// See also [`GetIcon`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.DynamicLauncher.html#org-freedesktop-portal-dynamiclauncher-geticon).
    #[doc(alias = "GetIcon")]
    #[doc(alias = "xdp_portal_dynamic_launcher_get_icon")]
    pub async fn icon(&self, desktop_file_id: &str) -> Result<LauncherIcon, Error> {
        self.0.call("GetIcon", &(desktop_file_id)).await
    }

    /// # Specifications
    ///
    /// See also [`Launch`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.DynamicLauncher.html#org-freedesktop-portal-dynamiclauncher-launch).
    #[doc(alias = "Launch")]
    #[doc(alias = "xdp_portal_dynamic_launcher_launch")]
    pub async fn launch(&self, desktop_file_id: &str, options: LaunchOptions) -> Result<(), Error> {
        self.0.call("Launch", &(desktop_file_id, &options)).await
    }

    /// # Specifications
    ///
    /// See also [`SupportedLauncherTypes`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.DynamicLauncher.html#org-freedesktop-portal-dynamiclauncher-supportedlaunchertypes).
    #[doc(alias = "SupportedLauncherTypes")]
    pub async fn supported_launcher_types(&self) -> Result<BitFlags<LauncherType>, Error> {
        self.0
            .property::<BitFlags<LauncherType>>("SupportedLauncherTypes")
            .await
    }
}

impl std::ops::Deref for DynamicLauncherProxy {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_icon_signature() {
        assert_eq!(LauncherIcon::SIGNATURE, "(vsu)");

        let icon = vec![IconType::Png];
        assert_eq!(serde_json::to_string(&icon).unwrap(), "[\"png\"]");
    }
}
