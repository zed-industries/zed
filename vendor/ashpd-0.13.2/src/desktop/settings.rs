//! ```rust,no_run
//! use ashpd::desktop::settings::Settings;
//! use futures_util::StreamExt;
//!
//! async fn run() -> ashpd::Result<()> {
//!     let proxy = Settings::new().await?;
//!
//!     let clock_format = proxy
//!         .read::<String>("org.gnome.desktop.interface", "clock-format")
//!         .await?;
//!     println!("{:#?}", clock_format);
//!
//!     let settings = proxy.read_all(&["org.gnome.desktop.interface"]).await?;
//!     println!("{:#?}", settings);
//!
//!     let setting = proxy
//!         .receive_setting_changed()
//!         .await?
//!         .next()
//!         .await
//!         .expect("Stream exhausted");
//!     println!("{}", setting.namespace());
//!     println!("{}", setting.key());
//!     println!("{:#?}", setting.value());
//!
//!     Ok(())
//! }
//! ```

use std::{collections::HashMap, convert::TryFrom, fmt::Debug, future::ready};

use futures_util::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use zbus::zvariant::{OwnedValue, Type, Value};

use crate::{Error, desktop::Color, proxy::Proxy};

/// A HashMap of the <key, value> settings found on a specific namespace.
pub type Namespace = HashMap<String, OwnedValue>;

#[derive(Deserialize, Type)]
/// A specific `namespace.key = value` setting.
pub struct Setting(String, String, OwnedValue);

impl Setting {
    /// The setting namespace.
    pub fn namespace(&self) -> &str {
        &self.0
    }

    /// The setting key.
    pub fn key(&self) -> &str {
        &self.1
    }

    /// The setting value.
    pub fn value(&self) -> &OwnedValue {
        &self.2
    }
}

impl std::fmt::Debug for Setting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Setting")
            .field("namespace", &self.namespace())
            .field("key", &self.key())
            .field("value", self.value())
            .finish()
    }
}

/// The system's preferred color scheme
#[cfg_attr(feature = "glib", derive(glib::Enum))]
#[cfg_attr(feature = "glib", enum_type(name = "AshpdColorScheme"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum ColorScheme {
    /// No preference
    #[default]
    NoPreference,
    /// Prefers dark appearance
    PreferDark,
    /// Prefers light appearance
    PreferLight,
}

impl From<ColorScheme> for OwnedValue {
    fn from(value: ColorScheme) -> Self {
        match value {
            ColorScheme::PreferDark => 1,
            ColorScheme::PreferLight => 2,
            _ => 0,
        }
        .into()
    }
}

impl TryFrom<OwnedValue> for ColorScheme {
    type Error = Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        TryFrom::<Value>::try_from(value.into())
    }
}

impl TryFrom<Value<'_>> for ColorScheme {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Ok(match u32::try_from(value)? {
            1 => Self::PreferDark,
            2 => Self::PreferLight,
            _ => Self::NoPreference,
        })
    }
}

/// The system's preferred contrast level
#[cfg_attr(feature = "glib", derive(glib::Enum))]
#[cfg_attr(feature = "glib", enum_type(name = "AshpdContrast"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Contrast {
    /// No preference
    #[default]
    NoPreference,
    /// Higher contrast
    High,
}

impl From<Contrast> for OwnedValue {
    fn from(value: Contrast) -> Self {
        match value {
            Contrast::High => 1,
            _ => 0,
        }
        .into()
    }
}

impl TryFrom<OwnedValue> for Contrast {
    type Error = Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        TryFrom::<Value>::try_from(value.into())
    }
}

impl TryFrom<Value<'_>> for Contrast {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Ok(match u32::try_from(value)? {
            1 => Self::High,
            _ => Self::NoPreference,
        })
    }
}

/// Appearance namespace
pub const APPEARANCE_NAMESPACE: &str = "org.freedesktop.appearance";
/// Color scheme key
pub const COLOR_SCHEME_KEY: &str = "color-scheme";
/// Accent color key
pub const ACCENT_COLOR_SCHEME_KEY: &str = "accent-color";
/// Contrast key
pub const CONTRAST_KEY: &str = "contrast";

/// The interface provides read-only access to a small number of host settings
/// required for toolkits similar to XSettings. It is not for general purpose
/// settings.
///
/// Wrapper of the DBus interface: [`org.freedesktop.portal.Settings`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Settings.html).
#[derive(Debug)]
#[doc(alias = "org.freedesktop.portal.Settings")]
pub struct Settings(Proxy<'static>);

impl Settings {
    /// Create a new instance of [`Settings`].
    pub async fn new() -> Result<Settings, Error> {
        let proxy = Proxy::new_desktop("org.freedesktop.portal.Settings").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`Settings`].
    pub async fn with_connection(connection: zbus::Connection) -> Result<Settings, Error> {
        let proxy =
            Proxy::new_desktop_with_connection(connection, "org.freedesktop.portal.Settings")
                .await?;
        Ok(Self(proxy))
    }

    /// Returns the version of the portal interface.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    /// Reads a single value. Returns an error on any unknown namespace or key.
    ///
    /// # Arguments
    ///
    /// * `namespaces` - List of namespaces to filter results by.
    ///
    /// If `namespaces` is an empty array or contains an empty string it matches
    /// all. Globing is supported but only for trailing sections, e.g.
    /// `org.example.*`.
    ///
    /// # Returns
    ///
    /// A `HashMap` of namespaces to its keys and values.
    ///
    /// # Specifications
    ///
    /// See also [`ReadAll`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Settings.html#org-freedesktop-portal-settings-readall).
    #[doc(alias = "ReadAll")]
    pub async fn read_all(
        &self,
        namespaces: &[impl AsRef<str> + Type + Serialize + Debug],
    ) -> Result<HashMap<String, Namespace>, Error> {
        self.0.call("ReadAll", &(namespaces)).await
    }

    /// Reads a single value. Returns an error on any unknown namespace or key.
    ///
    /// # Arguments
    ///
    /// * `namespace` - Namespace to look up key in.
    /// * `key` - The key to get.
    ///
    /// # Returns
    ///
    /// The value for `key` as a `zvariant::OwnedValue`.
    ///
    /// # Specifications
    ///
    /// See also [`Read`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Settings.html#org-freedesktop-portal-settings-read).
    #[doc(alias = "Read")]
    #[doc(alias = "ReadOne")]
    pub async fn read<T>(&self, namespace: &str, key: &str) -> Result<T, Error>
    where
        T: TryFrom<OwnedValue>,
        Error: From<<T as TryFrom<OwnedValue>>::Error>,
    {
        let value = self.0.call::<OwnedValue>("Read", &(namespace, key)).await?;
        if let Ok(v) = value.downcast_ref::<Value>() {
            T::try_from(v.try_to_owned()?).map_err(From::from)
        } else {
            T::try_from(value).map_err(From::from)
        }
    }

    /// Retrieves the system's preferred accent color
    pub async fn accent_color(&self) -> Result<Color, Error> {
        self.read::<(f64, f64, f64)>(APPEARANCE_NAMESPACE, ACCENT_COLOR_SCHEME_KEY)
            .await
            .map(Color::from)
    }

    /// Retrieves the system's preferred color scheme
    pub async fn color_scheme(&self) -> Result<ColorScheme, Error> {
        self.read::<ColorScheme>(APPEARANCE_NAMESPACE, COLOR_SCHEME_KEY)
            .await
    }

    /// Retrieves the system's preferred contrast level
    pub async fn contrast(&self) -> Result<Contrast, Error> {
        self.read::<Contrast>(APPEARANCE_NAMESPACE, CONTRAST_KEY)
            .await
    }

    /// Listen to changes of the system's preferred color scheme
    pub async fn receive_color_scheme_changed(
        &self,
    ) -> Result<impl Stream<Item = ColorScheme>, Error> {
        Ok(self
            .receive_setting_changed_with_args(APPEARANCE_NAMESPACE, COLOR_SCHEME_KEY)
            .await?
            .filter_map(|t| ready(t.ok())))
    }

    /// Listen to changes of the system's accent color
    pub async fn receive_accent_color_changed(&self) -> Result<impl Stream<Item = Color>, Error> {
        Ok(self
            .receive_setting_changed_with_args::<(f64, f64, f64)>(
                APPEARANCE_NAMESPACE,
                ACCENT_COLOR_SCHEME_KEY,
            )
            .await?
            .filter_map(|t| ready(t.ok().map(Color::from))))
    }

    /// Listen to changes of the system's contrast level
    pub async fn receive_contrast_changed(&self) -> Result<impl Stream<Item = Contrast>, Error> {
        Ok(self
            .receive_setting_changed_with_args(APPEARANCE_NAMESPACE, CONTRAST_KEY)
            .await?
            .filter_map(|t| ready(t.ok())))
    }

    /// Signal emitted when a setting changes.
    ///
    /// # Specifications
    ///
    /// See also [`SettingChanged`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Settings.html#org-freedesktop-portal-settings-settingchanged).
    #[doc(alias = "SettingChanged")]
    pub async fn receive_setting_changed(&self) -> Result<impl Stream<Item = Setting>, Error> {
        self.0.signal("SettingChanged").await
    }

    /// Similar to [Self::receive_setting_changed]
    /// but allows you to filter specific settings.
    ///
    /// # Example
    /// ```rust,no_run
    /// use ashpd::desktop::settings::{ColorScheme, Settings};
    /// use futures_util::StreamExt;
    ///
    /// # async fn run() -> ashpd::Result<()> {
    /// let settings = Settings::new().await?;
    /// while let Some(Ok(scheme)) = settings
    ///     .receive_setting_changed_with_args::<ColorScheme>(
    ///         "org.freedesktop.appearance",
    ///         "color-scheme",
    ///     )
    ///     .await?
    ///     .next()
    ///     .await
    /// {
    ///     println!("{:#?}", scheme);
    /// }
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn receive_setting_changed_with_args<T>(
        &self,
        namespace: &str,
        key: &str,
    ) -> Result<impl Stream<Item = Result<T, Error>> + use<T>, Error>
    where
        T: TryFrom<OwnedValue>,
        Error: From<<T as TryFrom<OwnedValue>>::Error>,
    {
        Ok(self
            .0
            .signal_with_args::<Setting>("SettingChanged", &[(0, namespace), (1, key)])
            .await?
            .map(|x| T::try_from(x.2).map_err(From::from)))
    }
}

impl std::ops::Deref for Settings {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
