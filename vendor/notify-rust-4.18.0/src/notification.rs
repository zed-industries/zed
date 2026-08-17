#[cfg(all(target_os = "macos", feature = "preview-macos-un"))]
use mac_usernotifications::InterruptionLevel;

#[cfg(all(unix, not(target_os = "macos")))]
use crate::{
    hints::{CustomHintType, Hint},
    urgency::Urgency,
    xdg,
};

#[cfg(all(unix, not(target_os = "macos"), feature = "images_no_default_features"))]
use crate::image::Image;

#[cfg(all(unix, target_os = "macos"))]
use crate::macos;
#[cfg(target_os = "windows")]
use crate::{windows, Urgency};

use crate::{error::*, timeout::Timeout};

#[cfg(all(unix, not(target_os = "macos")))]
use std::collections::{HashMap, HashSet};

// Returns the name of the current executable, used as a default for `Notification.appname`.
fn exe_name() -> String {
    std::env::current_exe()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
}

/// Desktop notification.
///
/// A desktop notification is configured via builder pattern, before it is launched with `show()`.
///
/// # Example
/// ``` no_run
/// # use notify_rust::*;
/// # fn _doc() -> Result<(), Box<dyn std::error::Error>> {
///     Notification::new()
///         .summary("☝️ A notification")
///         .show()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Notification {
    /// Filled by default with the executable name.
    pub appname: String,

    /// Single line to summarize the content.
    pub summary: String,

    /// Subtitle for macOS.
    pub subtitle: Option<String>,

    /// Multiple lines possible, may support simple markup.
    /// Check out [`get_capabilities()`](crate::get_capabilities) -> `body-markup` and `body-hyperlinks`.
    pub body: String,

    /// Use a `file://` URI or a name in an icon theme, must be compliant with freedesktop.org.
    pub icon: String,

    /// Check out [`Hint`].
    ///
    /// # Warning
    /// This does not hold all hints. [`Hint::Custom`] and [`Hint::CustomInt`] are held elsewhere.
    // /// please access hints via [`Notification::get_hints`].
    #[cfg(all(unix, not(target_os = "macos")))]
    pub hints: HashSet<Hint>,

    #[cfg(all(unix, not(target_os = "macos")))]
    pub(crate) hints_unique: HashMap<(String, CustomHintType), Hint>,

    /// See [`Notification::actions()`] and [`Notification::action()`].
    pub actions: Vec<String>,

    #[cfg(target_os = "macos")]
    pub(crate) sound_name: Option<String>,

    #[cfg(target_os = "windows")]
    pub(crate) sound_name: Option<String>,

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    pub(crate) path_to_image: Option<String>,

    #[cfg(target_os = "windows")]
    pub(crate) app_id: Option<String>,

    #[cfg(target_os = "windows")]
    pub(crate) urgency: Option<Urgency>,

    #[cfg(all(unix, not(target_os = "macos")))]
    pub(crate) bus: xdg::NotificationBus,

    /// Lifetime of the notification in ms. Often not respected by the server.
    pub timeout: Timeout, // both gnome and galago want allow for -1

    /// Interruption level (macOS only; has effect with the `preview-macos-un` feature).
    #[cfg(all(target_os = "macos", feature = "preview-macos-un"))]
    pub(crate) interruption_level: Option<InterruptionLevel>,

    /// Only to be used on the receive end. Use [`NotificationHandle`](crate::NotificationHandle) for updating.
    #[cfg(not(all(target_os = "macos", feature = "preview-macos-un")))]
    pub(crate) id: Option<u32>,

    /// Notification identifier for the macOS UN backend.
    #[cfg(all(target_os = "macos", feature = "preview-macos-un"))]
    pub(crate) id: Option<crate::notification_id::NotificationId>,
}

impl Notification {
    /// Constructs a new Notification.
    ///
    /// Most fields are empty by default, only `appname` is initialized with the name of the current
    /// executable.
    ///
    /// The `appname` is used by some desktop environments to group notifications.
    pub fn new() -> Notification {
        Notification::default()
    }

    /// This is for testing purposes only and will not work with actual implementations.
    #[cfg(all(unix, not(target_os = "macos")))]
    #[doc(hidden)]
    #[deprecated(note = "this is a test only feature")]
    pub fn at_bus(sub_bus: &str) -> Notification {
        let bus = xdg::NotificationBus::custom(sub_bus)
            .ok_or("invalid subpath")
            .unwrap();
        Notification {
            bus,
            ..Notification::default()
        }
    }

    /// Overwrite the `appname` field used for the notification.
    ///
    /// # Platform Support
    /// This method has no effect on macOS. There you can only set the application via [`set_application()`](fn.set_application.html).
    pub fn appname(&mut self, appname: &str) -> &mut Notification {
        appname.clone_into(&mut self.appname);
        self
    }

    /// Set the `summary`.
    ///
    /// Often acts as the title of the notification. For more elaborate content use the `body` field.
    pub fn summary(&mut self, summary: &str) -> &mut Notification {
        summary.clone_into(&mut self.summary);
        self
    }

    /// Set the `subtitle`.
    ///
    /// Only useful on macOS. Not part of the XDG specification.
    pub fn subtitle(&mut self, subtitle: &str) -> &mut Notification {
        self.subtitle = Some(subtitle.to_owned());
        self
    }

    /// Manual wrapper for [`Hint::ImageData`].
    #[cfg(all(feature = "images_no_default_features", unix, not(target_os = "macos")))]
    pub fn image_data(&mut self, image: Image) -> &mut Notification {
        self.hint(Hint::ImageData(image));
        self
    }

    /// Sets the image path for the notification.
    ///
    /// The path is passed to the platform's native notification API directly — no additional
    /// dependencies or crate features are required.
    ///
    /// Platform behaviour:
    /// - **Linux/BSD (XDG):** maps to the `image-path` hint in the D-Bus notification spec.
    /// - **macOS:** maps to `content_image` in `mac-notification-sys`, displayed on the right
    ///   side of the notification banner.
    /// - **Windows:** passed directly to `winrt-notification` as the notification image.
    pub fn image_path(&mut self, path: &str) -> &mut Notification {
        #[cfg(all(unix, not(target_os = "macos")))]
        {
            self.hint(Hint::ImagePath(path.to_string()));
        }
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        {
            self.path_to_image = Some(path.to_string());
        }
        self
    }

    /// Sets the app's `System.AppUserModel.ID`.
    #[cfg(target_os = "windows")]
    pub fn app_id(&mut self, app_id: &str) -> &mut Notification {
        self.app_id = Some(app_id.to_string());
        self
    }

    /// Wrapper for [`Hint::ImageData`].
    #[cfg(all(feature = "images_no_default_features", unix, not(target_os = "macos")))]
    pub fn image<T: AsRef<std::path::Path> + Sized>(
        &mut self,
        path: T,
    ) -> Result<&mut Notification> {
        let img = Image::open(&path)?;
        self.hint(Hint::ImageData(img));
        Ok(self)
    }

    /// Wrapper for [`Hint::SoundName`].
    #[cfg(all(unix, not(target_os = "macos")))]
    pub fn sound_name(&mut self, name: &str) -> &mut Notification {
        self.hint(Hint::SoundName(name.to_owned()));
        self
    }

    /// Set the `sound_name` for the `NSUserNotification`.
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    pub fn sound_name(&mut self, name: &str) -> &mut Notification {
        self.sound_name = Some(name.to_owned());
        self
    }

    /// Set the interruption level (macOS only; has effect with the `preview-macos-un` feature).
    ///
    /// Controls whether the notification breaks through Focus modes on macOS 12+.
    ///
    /// # Platform support
    ///
    /// This method is only available on macOS when the `preview-macos-un` feature is enabled.
    /// For a more cross-platform alternative, use `.urgency()`, which is automatically converted to the appropriate `InterruptionLevel` on macOS.
    #[cfg(all(target_os = "macos", feature = "preview-macos-un"))]
    pub fn interruption_level(&mut self, level: InterruptionLevel) -> &mut Notification {
        self.interruption_level = Some(level);
        self
    }

    /// Set the content of the `body` field.
    ///
    /// Multiline textual content of the notification.
    /// Each line should be treated as a paragraph.
    /// Simple html markup should be supported, depending on the server implementation.
    pub fn body(&mut self, body: &str) -> &mut Notification {
        body.clone_into(&mut self.body);
        self
    }

    /// Set the `icon` field.
    ///
    /// You can use common icon names here, usually those in `/usr/share/icons`
    /// can all be used.
    /// You can also use an absolute path to file.
    ///
    /// # Platform support
    /// macOS does not have support manually setting the icon. However you can pretend to be another app using [`set_application()`](fn.set_application.html)
    pub fn icon(&mut self, icon: &str) -> &mut Notification {
        icon.clone_into(&mut self.icon);
        self
    }

    /// Set the `icon` field automatically.
    ///
    /// This looks at your binary's name and uses it to set the icon.
    ///
    /// # Platform support
    /// macOS does not support manually setting the icon. However you can pretend to be another app using [`set_application()`](fn.set_application.html)
    pub fn auto_icon(&mut self) -> &mut Notification {
        self.icon = exe_name();
        self
    }

    /// Adds a hint.
    ///
    /// This method will add a hint to the internal hint [`HashSet`].
    /// Hints must be of type [`Hint`].
    ///
    /// Many of these are wrapped by more convenient functions such as:
    ///
    /// * [`sound_name()`](Self::sound_name)
    /// * [`urgency()`](Self::urgency)
    /// * [`image(...)`](#method.image) or
    ///   * [`image_data(...)`](#method.image_data)
    ///   * [`image_path(...)`](#method.image_path)
    ///
    /// ```no_run
    /// # use notify_rust::Notification;
    /// # use notify_rust::Hint;
    /// Notification::new().summary("Category:email")
    ///                    .body("This should not go away until you acknowledge it.")
    ///                    .icon("thunderbird")
    ///                    .appname("thunderbird")
    ///                    .hint(Hint::Category("email".to_owned()))
    ///                    .hint(Hint::Resident(true))
    ///                    .show();
    /// ```
    ///
    /// # Platform support
    /// Most of these hints don't even have an effect on the big XDG Desktops, they are completely tossed on macOS.
    #[cfg(all(unix, not(target_os = "macos")))]
    pub fn hint(&mut self, hint: Hint) -> &mut Notification {
        match hint {
            Hint::CustomInt(k, v) => {
                self.hints_unique
                    .insert((k.clone(), CustomHintType::Int), Hint::CustomInt(k, v));
            }
            Hint::Custom(k, v) => {
                self.hints_unique
                    .insert((k.clone(), CustomHintType::String), Hint::Custom(k, v));
            }
            _ => {
                self.hints.insert(hint);
            }
        }
        self
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    pub(crate) fn get_hints(&self) -> impl Iterator<Item = &Hint> {
        self.hints.iter().chain(self.hints_unique.values())
    }

    /// Set the `timeout`.
    ///
    /// Accepts multiple types that implement `Into<Timeout>`.
    ///
    /// ## `i32`
    ///
    /// This sets the time (in milliseconds) from the time the notification is displayed until it is
    /// closed again by the notification server.
    /// According to [specification](https://developer.gnome.org/notification-spec/)
    /// -1 will leave the timeout to be set by the server and
    /// 0 will cause the notification never to expire.
    /// ## [Duration](`std::time::Duration`)
    ///
    /// When passing a [`Duration`](`std::time::Duration`) we will try convert it into milliseconds.
    ///
    ///
    /// ```
    /// # use std::time::Duration;
    /// # use notify_rust::Timeout;
    /// assert_eq!(Timeout::from(Duration::from_millis(2000)), Timeout::Milliseconds(2000));
    /// ```
    /// ### Caveats!
    ///
    /// 1. If the duration is zero milliseconds then the original behavior will apply and the notification will **Never** timeout.
    /// 2. Should the number of milliseconds not fit within an [`i32`] then we will fall back to the default timeout.
    /// ```
    /// # use std::time::Duration;
    /// # use notify_rust::Timeout;
    /// assert_eq!(Timeout::from(Duration::from_millis(0)), Timeout::Never);
    /// assert_eq!(Timeout::from(Duration::from_millis(u64::MAX)), Timeout::Default);
    /// ```
    ///
    /// # Platform support
    /// This only works on XDG Desktops, macOS does not support manually setting the timeout.
    ///
    /// TODO: this will become available in 5.0 using `mac-usernotifications` using the new `.response()` api
    pub fn timeout<T: Into<Timeout>>(&mut self, timeout: T) -> &mut Notification {
        self.timeout = timeout.into();
        self
    }

    /// Set the `urgency`.
    ///
    /// Pick between Low, Normal, and Critical.
    ///
    /// # Platform support
    ///
    /// ## Linux/BSD (XDG)
    /// Urgency is sent as a hint to the notification server. Most desktops are fairly relaxed
    /// about urgency and may not change behavior significantly. Critical notifications are
    /// intended to not timeout automatically.
    ///
    /// ## Windows
    /// Urgency is mapped to toast scenarios:
    /// - `Low` and `Normal` → Default scenario (standard toast behavior)
    /// - `Critical` → Reminder scenario (stays on screen until user dismisses)
    ///
    /// ## macOS
    /// Mapped to [`InterruptionLevel`](`mac_usernotifications::InterruptionLevel`): `Low` → `Passive`, `Normal` → `Active`,
    /// `Critical` → `TimeSensitive`. Use `interruption_level`
    /// directly for finer control (e.g. `Critical` level that bypasses mute).
    #[cfg(all(unix, not(target_os = "macos")))]
    pub fn urgency(&mut self, urgency: Urgency) -> &mut Notification {
        self.hint(Hint::Urgency(urgency)); // TODO impl as T where T: Into<Urgency>
        self
    }

    /// Set the `urgency`.
    ///
    /// Pick between Low, Normal, and Critical.
    ///
    /// # Platform support
    ///
    /// ## Windows
    /// Urgency is mapped to toast scenarios:
    /// - `Low` and `Normal` → Default scenario (standard toast behavior)
    /// - `Critical` → Reminder scenario (stays on screen until user dismisses)
    ///
    /// ## Linux/BSD (XDG)
    /// See the Unix implementation documentation.
    ///
    /// ## macOS
    /// Mapped to [`InterruptionLevel`]: `Low` → `Passive`, `Normal` → `Active`,
    /// `Critical` → `TimeSensitive`. Use [`interruption_level`](Self::interruption_level)
    /// directly for finer control (e.g. `Critical` level that bypasses mute).
    #[cfg(target_os = "windows")]
    pub fn urgency(&mut self, urgency: Urgency) -> &mut Notification {
        self.urgency = Some(urgency);
        self
    }

    /// Set the `urgency` (macOS).
    ///
    /// Maps `Urgency` to the platform-native [`InterruptionLevel`]:
    /// - `Low` → [`Passive`](InterruptionLevel::Passive)
    /// - `Normal` → [`Active`](InterruptionLevel::Active)
    /// - `Critical` → [`TimeSensitive`](InterruptionLevel::TimeSensitive)
    ///
    /// For finer control (e.g. the `Critical` interruption level that bypasses
    /// mute and Do Not Disturb) use [`interruption_level`](Self::interruption_level)
    /// directly.
    #[cfg(all(target_os = "macos", feature = "preview-macos-un"))]
    pub fn urgency(&mut self, urgency: impl Into<InterruptionLevel>) -> &mut Notification {
        self.interruption_level.replace(urgency.into());
        self
    }

    /// Set `actions`.
    ///
    /// To quote <http://www.galago-project.org/specs/notification/0.9/x408.html#command-notify>
    ///
    /// >  Actions are sent over as a list of pairs.
    /// >  Each even element in the list (starting at index 0) represents the identifier for the action.
    /// >  Each odd element in the list is the localized string that will be displayed to the user.y
    ///
    /// There is nothing fancy going on here yet.
    /// **Careful! This replaces the internal list of actions!**
    #[deprecated(note = "please use .action() only")]
    pub fn actions(&mut self, actions: Vec<String>) -> &mut Notification {
        self.actions = actions;
        self
    }

    /// Add an action.
    ///
    /// This adds a single action to the internal list of actions.
    pub fn action(&mut self, identifier: &str, label: &str) -> &mut Notification {
        self.actions.push(identifier.to_owned());
        self.actions.push(label.to_owned());
        self
    }

    /// Set an id ahead of time.
    ///
    /// Setting the id ahead of time allows overriding a known other notification.
    /// If you want to update a notification, it is easier to use the `update()` method of
    /// the `NotificationHandle` object that `show()` returns.
    ///
    /// (XDG, Windows, and legacy macOS)
    #[cfg(not(all(target_os = "macos", feature = "preview-macos-un")))]
    pub fn id(&mut self, id: u32) -> &mut Notification {
        self.id = Some(id);
        self
    }

    /// Set a notification identifier (macOS `preview-macos-un` path).
    ///
    /// Re-posting with the same identifier replaces the existing notification.
    #[cfg(all(target_os = "macos", feature = "preview-macos-un"))]
    pub fn id(
        &mut self,
        id: impl Into<crate::notification_id::NotificationId>,
    ) -> &mut Notification {
        self.id = Some(id.into());
        self
    }

    /// Finalizes a notification.
    ///
    /// Part of the builder pattern, returns a complete copy of the built notification.
    pub fn finalize(&self) -> Notification {
        self.clone()
    }

    /// Schedules a notification to be sent at the specified date.
    #[cfg(all(target_os = "macos", feature = "chrono"))]
    pub fn schedule<T: chrono::TimeZone>(
        &self,
        delivery_date: chrono::DateTime<T>,
    ) -> Result<macos::NotificationHandle> {
        macos::schedule_notification(self, delivery_date.timestamp() as f64)
    }

    /// Schedules a notification to be sent at the specified timestamp.
    ///
    /// This is a raw `f64`. If you prefer a typed date, activate the `"chrono"` feature
    /// and use [`Notification::schedule()`] instead, which accepts a `chrono::DateTime<T>`.
    #[cfg(target_os = "macos")]
    pub fn schedule_raw(&self, timestamp: f64) -> Result<macos::NotificationHandle> {
        macos::schedule_notification(self, timestamp)
    }

    /// Sends the notification to D-Bus.
    ///
    /// Returns a handle to the notification.
    #[cfg(all(unix, not(target_os = "macos")))]
    pub fn show(&self) -> Result<xdg::NotificationHandle> {
        xdg::show_notification(self)
    }

    /// Sends the notification to D-Bus asynchronously.
    ///
    /// Returns a handle to the notification.
    #[cfg(all(unix, not(target_os = "macos")))]
    #[cfg(feature = "zbus")]
    pub async fn show_async(&self) -> Result<xdg::NotificationHandle> {
        xdg::show_notification_async(self).await
    }

    /// Sends the notification to D-Bus at the given sub-bus path.
    ///
    /// Returns a handle to the notification.
    #[cfg(all(unix, not(target_os = "macos")))]
    #[cfg(feature = "zbus")]
    // #[cfg(test)]
    pub async fn show_async_at_bus(&self, sub_bus: &str) -> Result<xdg::NotificationHandle> {
        let bus = xdg::NotificationBus::custom(sub_bus).ok_or("invalid subpath")?;
        xdg::show_notification_async_at_bus(self, bus).await
    }

    /// Sends Notification to `NSUserNotificationCenter` (default) or
    /// `UNUserNotificationCenter` (with `preview-macos-un` feature).
    #[cfg(target_os = "macos")]
    pub fn show(&self) -> Result<macos::NotificationHandle> {
        macos::show_notification(self)
    }

    /// Sends notification asynchronously via `UNUserNotificationCenter`.
    ///
    /// Only available with the `preview-macos-un` feature.
    #[cfg(all(target_os = "macos", feature = "preview-macos-un"))]
    pub async fn show_async(&self) -> Result<macos::NotificationHandle> {
        macos::show_notification_async(self).await
    }

    /// Sends Notification as a toast notification.
    #[cfg(target_os = "windows")]
    pub fn show(&self) -> Result<windows::NotificationHandle> {
        windows::show_notification(self)
    }

    /// Wraps [`Notification::show()`] but prints the notification to stdout.
    #[cfg(all(unix, not(target_os = "macos")))]
    #[deprecated = "this was never meant to be public API"]
    pub fn show_debug(&mut self) -> Result<xdg::NotificationHandle> {
        println!(
            "Notification:\n{appname}: ({icon}) {summary:?} {body:?}\nhints: [{hints:?}]\n",
            appname = self.appname,
            summary = self.summary,
            body = self.body,
            hints = self.hints,
            icon = self.icon,
        );
        self.show()
    }
}

impl Default for Notification {
    #[cfg(all(unix, not(target_os = "macos")))]
    fn default() -> Notification {
        Notification {
            appname: exe_name(),
            summary: String::new(),
            subtitle: None,
            body: String::new(),
            icon: String::new(),
            hints: HashSet::new(),
            hints_unique: HashMap::new(),
            actions: Vec::new(),
            timeout: Timeout::Default,
            bus: Default::default(),
            id: None,
        }
    }

    #[cfg(target_os = "macos")]
    fn default() -> Notification {
        Notification {
            appname: exe_name(),
            summary: String::new(),
            subtitle: None,
            body: String::new(),
            icon: String::new(),
            actions: Vec::new(),
            timeout: Timeout::Default,
            sound_name: Default::default(),
            path_to_image: None,
            #[cfg(all(target_os = "macos", feature = "preview-macos-un"))]
            interruption_level: None,
            id: None,
        }
    }

    #[cfg(target_os = "windows")]
    fn default() -> Notification {
        Notification {
            appname: exe_name(),
            summary: String::new(),
            subtitle: None,
            body: String::new(),
            icon: String::new(),
            actions: Vec::new(),
            timeout: Timeout::Default,
            sound_name: Default::default(),
            id: None,
            path_to_image: None,
            app_id: None,
            urgency: None,
        }
    }
}
