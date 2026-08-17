//! # Examples
//!
//! ```rust,no_run
//! use std::{thread, time};
//!
//! use ashpd::desktop::{
//!     Icon,
//!     notification::{Action, Button, Notification, NotificationProxy, Priority},
//! };
//! use futures_util::StreamExt;
//! use zbus::zvariant::Value;
//!
//! async fn run() -> ashpd::Result<()> {
//!     let proxy = NotificationProxy::new().await?;
//!
//!     let notification_id = "org.gnome.design.Contrast";
//!     proxy
//!         .add_notification(
//!             notification_id,
//!             Notification::new("Contrast")
//!                 .default_action("open")
//!                 .default_action_target(100)
//!                 .body("color copied to clipboard")
//!                 .priority(Priority::High)
//!                 .icon(Icon::with_names(&["dialog-question-symbolic"]))
//!                 .button(Button::new("Copy", "copy").target(32))
//!                 .button(Button::new("Delete", "delete").target(40)),
//!         )
//!         .await?;
//!
//!     let action = proxy
//!         .receive_action_invoked()
//!         .await?
//!         .next()
//!         .await
//!         .expect("Stream exhausted");
//!     match action.name() {
//!         "copy" => (),   // Copy something to clipboard
//!         "delete" => (), // Delete the file
//!         _ => (),
//!     };
//!     println!("{:#?}", action.id());
//!     println!(
//!         "{:#?}",
//!         action.parameter().get(0).unwrap().downcast_ref::<u32>()
//!     );
//!
//!     proxy.remove_notification(notification_id).await?;
//!     Ok(())
//! }
//! ```

use std::{fmt, os::fd::AsFd, str::FromStr};

use futures_util::Stream;
use serde::{self, Deserialize, Serialize};
use zbus::zvariant::{
    OwnedValue, Type, Value,
    as_value::{self, optional},
};

use super::Icon;
use crate::{Error, proxy::Proxy};

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[zvariant(signature = "s")]
/// The content of a notification.
pub enum Category {
    /// Instant messaging apps message.
    #[doc(alias = "im.message")]
    ImMessage,
    /// Ringing alarm.
    #[doc(alias = "alarm.ringing")]
    AlarmRinging,
    /// Incoming call.
    #[doc(alias = "call.incoming")]
    IncomingCall,
    /// Ongoing call.
    #[doc(alias = "call.ongoing")]
    OngoingCall,
    /// Missed call.
    #[doc(alias = "call.missed")]
    MissedCall,
    /// Extreme weather warning.
    #[doc(alias = "weather.warning.extreme")]
    ExtremeWeather,
    /// Extreme danger broadcast.
    #[doc(alias = "cellbroadcast.danger.extreme")]
    CellNetworkExtremeDanger,
    /// Severe danger broadcast.
    #[doc(alias = "cellbroadcast.danger.severe")]
    CellNetworkSevereDanger,
    /// Amber alert broadcast.
    #[doc(alias = "cellbroadcast.amber-alert")]
    CellNetworkAmberAlert,
    /// Test broadcast.
    #[doc(alias = "cellbroadcast.test")]
    CellNetworkBroadcastTest,
    /// Low battery.
    #[doc(alias = "os.battery.low")]
    LowBattery,
    /// Browser websites notifications.
    #[doc(alias = "browser.web-notification")]
    WebNotification,
    /// Vendor specific.
    Other(String),
}

impl Serialize for Category {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let category_str = match self {
            Self::ImMessage => "im.message",
            Self::AlarmRinging => "alarm.ringing",
            Self::IncomingCall => "call.incoming",
            Self::OngoingCall => "call.ongoing",
            Self::MissedCall => "call.missed",
            Self::ExtremeWeather => "weather.warning.extreme",
            Self::CellNetworkExtremeDanger => "cellbroadcast.danger.extreme",
            Self::CellNetworkSevereDanger => "cellbroadcast.danger.severe",
            Self::CellNetworkAmberAlert => "cellbroadcast.amber-alert",
            Self::CellNetworkBroadcastTest => "cellbroadcast.test",
            Self::LowBattery => "os.battery.low",
            Self::WebNotification => "browser.web-notification",
            Self::Other(other) => other.as_str(),
        };
        serializer.serialize_str(category_str)
    }
}

impl FromStr for Category {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "im.message" => Ok(Self::ImMessage),
            "alarm.ringing" => Ok(Self::AlarmRinging),
            "call.incoming" => Ok(Self::IncomingCall),
            "call.ongoing" => Ok(Self::OngoingCall),
            "call.missed" => Ok(Self::MissedCall),
            "weather.warning.extreme" => Ok(Self::ExtremeWeather),
            "cellbroadcast.danger.extreme" => Ok(Self::CellNetworkExtremeDanger),
            "cellbroadcast.danger.severe" => Ok(Self::CellNetworkSevereDanger),
            "cellbroadcast.amber-alert" => Ok(Self::CellNetworkAmberAlert),
            "cellbroadcast.test" => Ok(Self::CellNetworkBroadcastTest),
            "os.battery.low" => Ok(Self::LowBattery),
            "browser.web-notification" => Ok(Self::WebNotification),
            _ => Ok(Self::Other(s.to_owned())),
        }
    }
}

impl<'de> Deserialize<'de> for Category {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let category = String::deserialize(deserializer)?;
        category
            .parse::<Self>()
            .map_err(|_e| serde::de::Error::custom("Failed to parse category"))
    }
}

#[cfg_attr(feature = "glib", derive(glib::Enum))]
#[cfg_attr(feature = "glib", enum_type(name = "AshpdPriority"))]
#[derive(Debug, Copy, Clone, Serialize, PartialEq, Eq, Type)]
#[zvariant(signature = "s")]
#[serde(rename_all = "lowercase")]
/// The notification priority
pub enum Priority {
    /// Low.
    Low,
    /// Normal.
    Normal,
    /// High.
    High,
    /// Urgent.
    Urgent,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Low => write!(f, "Low"),
            Self::Normal => write!(f, "Normal"),
            Self::High => write!(f, "High"),
            Self::Urgent => write!(f, "Urgent"),
        }
    }
}

impl AsRef<str> for Priority {
    fn as_ref(&self) -> &str {
        match self {
            Self::Low => "Low",
            Self::Normal => "Normal",
            Self::High => "High",
            Self::Urgent => "Urgent",
        }
    }
}

impl From<Priority> for &'static str {
    fn from(d: Priority) -> Self {
        match d {
            Priority::Low => "Low",
            Priority::Normal => "Normal",
            Priority::High => "High",
            Priority::Urgent => "Urgent",
        }
    }
}

impl FromStr for Priority {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Low" | "low" => Ok(Priority::Low),
            "Normal" | "normal" => Ok(Priority::Normal),
            "High" | "high" => Ok(Priority::High),
            "Urgent" | "urgent" => Ok(Priority::Urgent),
            _ => Err(Error::ParseError("Failed to parse priority, invalid value")),
        }
    }
}

#[cfg_attr(feature = "glib", derive(glib::Enum))]
#[cfg_attr(feature = "glib", enum_type(name = "AshpdNotificationDisplayHint"))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Type, Serialize)]
#[zvariant(signature = "s")]
#[serde(rename_all = "kebab-case")]
/// Ways to display a notification.
pub enum DisplayHint {
    /// Transient.
    #[doc(alias = "transient")]
    Transient,
    /// Tray.
    #[doc(alias = "tray")]
    Tray,
    /// Persistent.
    #[doc(alias = "persistent")]
    Persistent,
    /// Hide on lockscreen.
    #[doc(alias = "hide-on-lockscreen")]
    HideOnLockScreen,
    /// Enable speakerphone.
    #[doc(alias = "hide-content-on-lockscreen")]
    HideContentOnLockScreen,
    /// Show as new.
    #[doc(alias = "show-as-new")]
    ShowAsNew,
}

#[derive(Serialize, Type, Debug)]
/// A notification
#[zvariant(signature = "dict")]
#[serde(rename_all = "kebab-case")]
pub struct Notification {
    /// User-visible string to display as the title.
    #[serde(with = "as_value")]
    title: String,
    /// User-visible string to display as the body.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    body: Option<String>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    markup_body: Option<String>,
    /// Serialized icon (e.g using gio::Icon::serialize).
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    icon: Option<Icon>,
    /// The priority for the notification.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    priority: Option<Priority>,
    /// Name of an action that is exported by the application.
    /// This action will be activated when the user clicks on the notification.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    default_action: Option<String>,
    /// Target parameter to send along when activating the default action.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    default_action_target: Option<OwnedValue>,
    /// Array of buttons to add to the notification.
    #[serde(with = "as_value", skip_serializing_if = "Vec::is_empty")]
    buttons: Vec<Button>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    category: Option<Category>,
    #[serde(with = "as_value", skip_serializing_if = "Vec::is_empty")]
    display_hint: Vec<DisplayHint>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    sound: Option<OwnedValue>,
}

impl Notification {
    /// Create a new notification.
    ///
    /// # Arguments
    ///
    /// * `title` - the notification title.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_owned(),
            body: None,
            markup_body: None,
            priority: None,
            icon: None,
            default_action: None,
            default_action_target: None,
            buttons: Vec::new(),
            category: None,
            display_hint: Vec::new(),
            sound: None,
        }
    }

    /// Sets the notification body.
    #[must_use]
    pub fn body<'a>(mut self, body: impl Into<Option<&'a str>>) -> Self {
        self.body = body.into().map(ToOwned::to_owned);
        self
    }

    /// Same as [`Notification::body`] but supports markup formatting.
    #[must_use]
    pub fn markup_body<'a>(mut self, markup_body: impl Into<Option<&'a str>>) -> Self {
        self.markup_body = markup_body.into().map(ToOwned::to_owned);
        self
    }

    /// Sets an icon to the notification.
    #[must_use]
    pub fn icon(mut self, icon: impl Into<Option<Icon>>) -> Self {
        self.icon = icon.into();
        self
    }

    /// Sets the notification sound.
    #[must_use]
    pub fn sound<S>(mut self, sound: impl Into<Option<S>>) -> Self
    where
        S: AsFd,
    {
        self.sound = sound.into().map(|s| {
            zbus::zvariant::Value::from(zbus::zvariant::Fd::from(s.as_fd()))
                .try_to_owned()
                .unwrap()
        });
        self
    }

    /// Sets the notification category.
    #[must_use]
    pub fn category(mut self, category: impl Into<Option<Category>>) -> Self {
        self.category = category.into();
        self
    }

    #[must_use]
    /// Sets the notification display hints.
    pub fn display_hint(mut self, hints: impl IntoIterator<Item = DisplayHint>) -> Self {
        self.display_hint = hints.into_iter().collect();
        self
    }

    /// Sets the notification priority.
    #[must_use]
    pub fn priority(mut self, priority: impl Into<Option<Priority>>) -> Self {
        self.priority = priority.into();
        self
    }

    /// Sets the default action when the user clicks on the notification.
    #[must_use]
    pub fn default_action<'a>(mut self, default_action: impl Into<Option<&'a str>>) -> Self {
        self.default_action = default_action.into().map(ToOwned::to_owned);
        self
    }

    /// Sets a value to be sent in the `action_invoked` signal.
    #[must_use]
    pub fn default_action_target<'a, T: Into<Value<'a>>>(
        mut self,
        default_action_target: impl Into<Option<T>>,
    ) -> Self {
        self.default_action_target = default_action_target
            .into()
            .map(|t| t.into().try_to_owned().unwrap());
        self
    }

    /// Adds a new button to the notification.
    #[must_use]
    pub fn button(mut self, button: Button) -> Self {
        self.buttons.push(button);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[zvariant(signature = "s")]
/// The purpose of a button.
pub enum ButtonPurpose {
    /// Instant messaging reply with text.
    #[doc(alias = "im.reply-with-text")]
    ImReplyWithText,
    /// Accept call.
    #[doc(alias = "call.accept")]
    CallAccept,
    /// Decline call.
    #[doc(alias = "call.decline")]
    CallDecline,
    /// Hangup call.
    #[doc(alias = "call.hang-up")]
    CallHangup,
    /// Enable speakerphone.
    #[doc(alias = "call.enable-speakerphone")]
    CallEnableSpeakerphone,
    /// Disable speakerphone.
    #[doc(alias = "call.disable-speakerphone")]
    CallDisableSpeakerphone,
    /// System custom alert.
    #[doc(alias = "system.custom-alert")]
    SystemCustomAlert,
    /// Vendor specific.
    Other(String),
}

impl Serialize for ButtonPurpose {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let purpose = match self {
            Self::ImReplyWithText => "im.reply-with-text",
            Self::CallAccept => "call.accept",
            Self::CallDecline => "call.decline",
            Self::CallHangup => "call.hang-up",
            Self::CallEnableSpeakerphone => "call.enable-speakerphone",
            Self::CallDisableSpeakerphone => "call.disable-speakerphone",
            Self::SystemCustomAlert => "system.custom-alert",
            Self::Other(other) => other.as_str(),
        };
        serializer.serialize_str(purpose)
    }
}

impl FromStr for ButtonPurpose {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "im.reply-with-text" => Ok(Self::ImReplyWithText),
            "call.accept" => Ok(Self::CallAccept),
            "call.decline" => Ok(Self::CallDecline),
            "call.hang-up" => Ok(Self::CallHangup),
            "call.enable-speakerphone" => Ok(Self::CallEnableSpeakerphone),
            "call.disable-speakerphone" => Ok(Self::CallDisableSpeakerphone),
            "system.custom-alert" => Ok(Self::SystemCustomAlert),
            _ => Ok(Self::Other(s.to_owned())),
        }
    }
}

impl<'de> Deserialize<'de> for ButtonPurpose {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let purpose = String::deserialize(deserializer)?;
        purpose
            .parse::<Self>()
            .map_err(|_e| serde::de::Error::custom("Failed to parse purpose"))
    }
}

#[derive(Serialize, Type, Debug)]
/// A notification button
#[zvariant(signature = "dict")]
pub struct Button {
    /// User-visible label for the button. Mandatory.
    #[serde(with = "as_value")]
    label: String,
    /// Name of an action that is exported by the application. The action will
    /// be activated when the user clicks on the button.
    #[serde(with = "as_value")]
    action: String,
    /// Target parameter to send along when activating the action.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    target: Option<OwnedValue>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    purpose: Option<ButtonPurpose>,
}

impl Button {
    /// Create a new notification button.
    ///
    /// # Arguments
    ///
    /// * `label` - the user visible label of the button.
    /// * `action` - the action name to be invoked when the user clicks on the
    ///   button.
    pub fn new(label: &str, action: &str) -> Self {
        Self {
            label: label.to_owned(),
            action: action.to_owned(),
            target: None,
            purpose: None,
        }
    }

    /// The value to send with the action name when the button is clicked.
    #[must_use]
    pub fn target<'a, T: Into<Value<'a>>>(mut self, target: impl Into<Option<T>>) -> Self {
        self.target = target.into().map(|t| t.into().try_to_owned().unwrap());
        self
    }

    /// Sets the button purpose.
    #[must_use]
    pub fn purpose(mut self, purpose: impl Into<Option<ButtonPurpose>>) -> Self {
        self.purpose = purpose.into();
        self
    }
}

#[derive(Debug, Deserialize, Type)]
/// An invoked action.
pub struct Action(String, String, Vec<OwnedValue>);

impl Action {
    /// Notification ID.
    pub fn id(&self) -> &str {
        &self.0
    }

    /// Action name.
    pub fn name(&self) -> &str {
        &self.1
    }

    /// The parameters passed to the action.
    pub fn parameter(&self) -> &[OwnedValue] {
        &self.2
    }
}

#[derive(Deserialize, Type, Debug, OwnedValue)]
#[zvariant(signature = "dict")]
#[serde(rename_all = "kebab-case")]
// TODO: figure out why this can't use the enums
struct SupportedOptions {
    #[serde(default, with = "as_value")]
    category: Vec<String>,
    #[serde(default, with = "as_value")]
    button_purpose: Vec<String>,
}

/// The interface lets sandboxed applications send and withdraw notifications.
///
/// It is not possible for the application to learn if the notification was
/// actually presented to the user. Not a portal in the strict sense, since
/// there is no user interaction.
///
/// **Note** in contrast to most other portal requests, notifications are
/// expected to outlast the running application. If a user clicks on a
/// notification after the application has exited, it will get activated again.
///
/// Notifications can specify actions that can be activated by the user.
/// Actions whose name starts with 'app.' are assumed to be exported and will be
/// activated via the ActivateAction() method in the org.freedesktop.Application
/// interface. Other actions are activated by sending the
///  `#org.freedeskop.portal.Notification::ActionInvoked` signal to the
/// application.
///
/// Wrapper of the DBus interface: [`org.freedesktop.portal.Notification`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Notification.html).
#[derive(Debug)]
#[doc(alias = "org.freedesktop.portal.Notification")]
pub struct NotificationProxy(Proxy<'static>);

impl NotificationProxy {
    /// Create a new instance of [`NotificationProxy`].
    pub async fn new() -> Result<Self, Error> {
        let proxy = Proxy::new_desktop("org.freedesktop.portal.Notification").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`NotificationProxy`].
    pub async fn with_connection(connection: zbus::Connection) -> Result<Self, Error> {
        let proxy =
            Proxy::new_desktop_with_connection(connection, "org.freedesktop.portal.Notification")
                .await?;
        Ok(Self(proxy))
    }

    /// Returns the version of the portal interface.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    /// Signal emitted when a particular action is invoked.
    ///
    /// # Specifications
    ///
    /// See also [`ActionInvoked`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Notification.html#org-freedesktop-portal-notification-actioninvoked).
    #[doc(alias = "ActionInvoked")]
    #[doc(alias = "XdpPortal::notification-action-invoked")]
    pub async fn receive_action_invoked(&self) -> Result<impl Stream<Item = Action>, Error> {
        self.0.signal("ActionInvoked").await
    }

    /// Sends a notification.
    ///
    /// The ID can be used to later withdraw the notification.
    /// If the application reuses the same ID without withdrawing, the
    /// notification is replaced by the new one.
    ///
    /// # Arguments
    ///
    /// * `id` - Application-provided ID for this notification.
    /// * `notification` - The notification.
    ///
    /// # Specifications
    ///
    /// See also [`AddNotification`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Notification.html#org-freedesktop-portal-notification-addnotification).
    #[doc(alias = "AddNotification")]
    #[doc(alias = "xdp_portal_add_notification")]
    pub async fn add_notification(
        &self,
        id: &str,
        notification: Notification,
    ) -> Result<(), Error> {
        self.0.call("AddNotification", &(id, notification)).await
    }

    /// Withdraws a notification.
    ///
    /// # Arguments
    ///
    /// * `id` - Application-provided ID for this notification.
    ///
    /// # Specifications
    ///
    /// See also [`RemoveNotification`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Notification.html#org-freedesktop-portal-notification-removenotification).
    #[doc(alias = "RemoveNotification")]
    #[doc(alias = "xdp_portal_remove_notification")]
    pub async fn remove_notification(&self, id: &str) -> Result<(), Error> {
        self.0.call("RemoveNotification", &(id)).await
    }

    /// Supported options by the notifications server.
    ///
    /// # Required version
    ///
    /// The method requires the 2nd version implementation of the portal and
    /// would fail with [`Error::RequiresVersion`] otherwise.
    ///
    /// # Specifications
    ///
    /// See also [`SupportedOptions`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Notification.html#org-freedesktop-portal-notification-supportedoptions).
    #[doc(alias = "SupportedOptions")]
    pub async fn supported_options(&self) -> Result<(Vec<Category>, Vec<ButtonPurpose>), Error> {
        let options = self
            .0
            .property_versioned::<SupportedOptions>("SupportedOptions", 2)
            .await?;
        let categories = options
            .category
            .into_iter()
            .map(|c| Category::from_str(&c).unwrap())
            .collect();
        let purposes = options
            .button_purpose
            .into_iter()
            .map(|c| ButtonPurpose::from_str(&c).unwrap())
            .collect();
        Ok((categories, purposes))
    }
}

impl std::ops::Deref for NotificationProxy {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
