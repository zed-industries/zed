// Copyright 2017-2022 allenbenz <allenbenz@users.noreply.github.com>
// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! An incomplete wrapper over the WinRT toast api
//!
//! Tested in Windows 10 and 8.1. Untested in Windows 8, might work.
//!
//! Todo:
//!
//! * Add support for Adaptive Content
//!
//! Known Issues:
//!
//! * Will not work for Windows 7.
//!
//! Limitations:
//!
//! * Windows 8.1 only supports a single image, the last image (icon, hero, image) will be the one on the toast
//!
//! for xml schema details check out:
//!
//! * <https://docs.microsoft.com/en-us/uwp/schemas/tiles/toastschema/root-elements>
//! * <https://docs.microsoft.com/en-us/windows/uwp/controls-and-patterns/tiles-and-notifications-toast-xml-schema>
//! * <https://docs.microsoft.com/en-us/windows/uwp/controls-and-patterns/tiles-and-notifications-adaptive-interactive-toasts>
//! * <https://msdn.microsoft.com/library/14a07fce-d631-4bad-ab99-305b703713e6#Sending_toast_notifications_from_desktop_apps>
//!
//! For Windows 7 and older support look into Shell_NotifyIcon
//! * <https://msdn.microsoft.com/en-us/library/windows/desktop/ee330740(v=vs.85).aspx>
//! * <https://softwareengineering.stackexchange.com/questions/222339/using-the-system-tray-notification-area-app-in-windows-7>
//!
//! For actions look at <https://docs.microsoft.com/en-us/dotnet/api/microsoft.toolkit.uwp.notifications.toastactionscustom?view=win-comm-toolkit-dotnet-7.0>

mod xml_escape;

use windows::{
    core::{IInspectable, Interface},
    Data::Xml::Dom::XmlDocument,
    Foundation::{Collections::StringMap, TypedEventHandler},
    UI::Notifications::{
        NotificationData, ToastActivatedEventArgs, ToastDismissedEventArgs,
        ToastNotificationManager,
    },
};

use std::fmt::Display;
use std::fmt::Write;
use std::path::Path;
use std::str::FromStr;

pub use windows::core::HSTRING;
pub use windows::UI::Notifications::NotificationUpdateResult;
pub use windows::UI::Notifications::ToastNotification;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Windows API error: {0}")]
    Os(#[from] windows::core::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

/// `ToastDismissalReason` is a struct representing the reason a toast notification was dismissed.
///
/// Variants:
/// - `UserCanceled`: The user explicitly dismissed the toast notification.
/// - `ApplicationHidden`: The application hid the toast notification programmatically.
/// - `TimedOut`: The toast notification was dismissed because it timed out.
pub use windows::UI::Notifications::ToastDismissalReason;

pub struct Toast {
    duration: String,
    title: String,
    line1: String,
    line2: String,
    images: String,
    audio: String,
    app_id: String,
    progress: Option<Progress>,
    scenario: String,
    on_activated: Option<TypedEventHandler<ToastNotification, IInspectable>>,
    on_dismissed: Option<TypedEventHandler<ToastNotification, ToastDismissedEventArgs>>,
    buttons: Vec<Button>,
}

#[derive(Clone, Copy)]
pub enum Duration {
    /// 7 seconds
    Short,

    /// 25 seconds
    Long,
}

#[derive(Debug, Clone, Copy)]
pub enum Sound {
    Default,
    IM,
    Mail,
    Reminder,
    SMS,
    /// Play the loopable sound only once
    Single(LoopableSound),
    /// Loop the loopable sound for the entire duration of the toast
    Loop(LoopableSound),
}

impl TryFrom<&str> for Sound {
    type Error = SoundParsingError;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl FromStr for Sound {
    type Err = SoundParsingError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "Default" => Sound::Default,
            "IM" => Sound::IM,
            "Mail" => Sound::Mail,
            "Reminder" => Sound::Reminder,
            "SMS" => Sound::SMS,
            _ => Sound::Single(LoopableSound::from_str(s)?),
        })
    }
}

impl Display for Sound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Sound::Default => "Default",
                Sound::IM => "IM",
                Sound::Mail => "Mail",
                Sound::Reminder => "Reminder",
                Sound::SMS => "SMS",
                Sound::Single(s) | Sound::Loop(s) => return write!(f, "{s}"),
            }
        )
    }
}

struct Button {
    content: String,
    action: String,
}

/// Sounds suitable for Looping
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum LoopableSound {
    Alarm,
    Alarm2,
    Alarm3,
    Alarm4,
    Alarm5,
    Alarm6,
    Alarm7,
    Alarm8,
    Alarm9,
    Alarm10,
    Call,
    Call2,
    Call3,
    Call4,
    Call5,
    Call6,
    Call7,
    Call8,
    Call9,
    Call10,
}

#[derive(Debug)]
pub struct SoundParsingError;
impl Display for SoundParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "couldn't parse string as a valid sound")
    }
}
impl std::error::Error for SoundParsingError {}

impl TryFrom<&str> for LoopableSound {
    type Error = SoundParsingError;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl FromStr for LoopableSound {
    type Err = SoundParsingError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "Alarm" => LoopableSound::Alarm,
            "Alarm2" => LoopableSound::Alarm2,
            "Alarm3" => LoopableSound::Alarm3,
            "Alarm4" => LoopableSound::Alarm4,
            "Alarm5" => LoopableSound::Alarm5,
            "Alarm6" => LoopableSound::Alarm6,
            "Alarm7" => LoopableSound::Alarm7,
            "Alarm8" => LoopableSound::Alarm8,
            "Alarm9" => LoopableSound::Alarm9,
            "Alarm10" => LoopableSound::Alarm10,
            "Call" => LoopableSound::Call,
            "Call2" => LoopableSound::Call2,
            "Call3" => LoopableSound::Call3,
            "Call4" => LoopableSound::Call4,
            "Call5" => LoopableSound::Call5,
            "Call6" => LoopableSound::Call6,
            "Call7" => LoopableSound::Call7,
            "Call8" => LoopableSound::Call8,
            "Call9" => LoopableSound::Call9,
            "Call10" => LoopableSound::Call10,
            _ => return Err(SoundParsingError),
        })
    }
}

impl Display for LoopableSound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LoopableSound::Alarm => "Alarm",
                LoopableSound::Alarm2 => "Alarm2",
                LoopableSound::Alarm3 => "Alarm3",
                LoopableSound::Alarm4 => "Alarm4",
                LoopableSound::Alarm5 => "Alarm5",
                LoopableSound::Alarm6 => "Alarm6",
                LoopableSound::Alarm7 => "Alarm7",
                LoopableSound::Alarm8 => "Alarm8",
                LoopableSound::Alarm9 => "Alarm9",
                LoopableSound::Alarm10 => "Alarm10",
                LoopableSound::Call => "Call",
                LoopableSound::Call2 => "Call2",
                LoopableSound::Call3 => "Call3",
                LoopableSound::Call4 => "Call4",
                LoopableSound::Call5 => "Call5",
                LoopableSound::Call6 => "Call6",
                LoopableSound::Call7 => "Call7",
                LoopableSound::Call8 => "Call8",
                LoopableSound::Call9 => "Call9",
                LoopableSound::Call10 => "Call10",
            }
        )
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum IconCrop {
    Square,
    Circular,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Scenario {
    /// The normal toast behavior.
    Default,
    /// This will be displayed pre-expanded and stay on the user's screen till dismissed. Audio will loop by default and will use alarm audio.
    Alarm,
    /// This will be displayed pre-expanded and stay on the user's screen till dismissed.
    Reminder,
    /// This will be displayed pre-expanded in a special call format and stay on the user's screen till dismissed. Audio will loop by default and will use ringtone audio.
    IncomingCall,
}

#[derive(Clone)]
pub struct Progress {
    /// Define a tag to uniquely identify the notification, in order update the notification data later.
    pub tag: String,
    /// Gets or sets an optional title string. Supports data binding.
    pub title: String,
    /// Gets or sets a status string (required), which is displayed underneath the progress bar on the left. This string should reflect the status of the operation, like "Downloading..." or "Installing..."
    pub status: String,
    /// Gets or sets the value of the progress bar. Supports data binding. Defaults to 0. Can either be a double between 0.0 and 1.0,
    pub value: f32,
    /// Gets or sets an optional string to be displayed instead of the default percentage string.
    pub value_string: String,
}

impl Progress {
    fn xml() -> &'static str {
        r#"<progress
                title="{progressTitle}"
                value="{progressValue}"
                valueStringOverride="{progressValueString}"
                status="{progressStatus}"/>"#
    }

    fn tag(&self) -> HSTRING {
        HSTRING::from(&self.tag)
    }

    fn title(&self) -> HSTRING {
        HSTRING::from(&self.title)
    }

    fn status(&self) -> HSTRING {
        HSTRING::from(&self.status)
    }

    fn value(&self) -> HSTRING {
        HSTRING::from(&self.value.to_string())
    }

    fn value_string(&self) -> HSTRING {
        HSTRING::from(&self.value_string)
    }
}

impl Toast {
    /// This can be used if you do not have a AppUserModelID.
    ///
    /// However, the toast will erroneously report its origin as powershell.
    pub const POWERSHELL_APP_ID: &'static str = "{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\
                                                 \\WindowsPowerShell\\v1.0\\powershell.exe";
    /// Constructor for the toast builder.
    ///
    /// app_id is the running application's [AppUserModelID][1].
    ///
    /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/dd378459(v=vs.85).aspx
    ///
    /// If the program you are using this in was not installed, use Toast::POWERSHELL_APP_ID for now
    #[allow(dead_code)]
    pub fn new(app_id: &str) -> Toast {
        Toast {
            duration: String::new(),
            title: String::new(),
            line1: String::new(),
            line2: String::new(),
            images: String::new(),
            audio: String::new(),
            app_id: app_id.to_string(),
            progress: None,
            scenario: String::new(),
            on_activated: None,
            on_dismissed: None,
            buttons: Vec::new(),
        }
    }

    /// Sets the title of the toast.
    ///
    /// Will be white.
    /// Supports Unicode ✓
    pub fn title(mut self, content: &str) -> Toast {
        self.title = format!(r#"<text id="1">{}</text>"#, xml_escape::escape(content));
        self
    }

    /// Add/Sets the first line of text below title.
    ///
    /// Will be grey.
    /// Supports Unicode ✓
    pub fn text1(mut self, content: &str) -> Toast {
        self.line1 = format!(r#"<text id="2">{}</text>"#, xml_escape::escape(content));
        self
    }

    /// Add/Sets the second line of text below title.
    ///
    /// Will be grey.
    /// Supports Unicode ✓
    pub fn text2(mut self, content: &str) -> Toast {
        self.line2 = format!(r#"<text id="3">{}</text>"#, xml_escape::escape(content));
        self
    }

    /// Set the length of time to show the toast
    pub fn duration(mut self, duration: Duration) -> Toast {
        self.duration = match duration {
            Duration::Long => "duration=\"long\"",
            Duration::Short => "duration=\"short\"",
        }
        .to_string();
        self
    }

    /// Set the scenario of the toast
    ///
    /// The system keeps the notification on screen until the user acts upon/dismisses it.
    /// The system also plays the suitable notification sound as well.
    pub fn scenario(mut self, scenario: Scenario) -> Toast {
        self.scenario = match scenario {
            Scenario::Default => "",
            Scenario::Alarm => "scenario=\"alarm\"",
            Scenario::Reminder => "scenario=\"reminder\"",
            Scenario::IncomingCall => "scenario=\"incomingCall\"",
        }
        .to_string();
        self
    }

    /// Set the icon shown in the upper left of the toast
    ///
    /// The default is determined by your app id.
    /// If you are using the powershell workaround, it will be the powershell icon
    pub fn icon(mut self, source: &Path, crop: IconCrop, alt_text: &str) -> Toast {
        if is_newer_than_windows81() {
            let crop_type_attr = match crop {
                IconCrop::Square => "".to_string(),
                IconCrop::Circular => "hint-crop=\"circle\"".to_string(),
            };

            self.images = format!(
                r#"{}<image placement="appLogoOverride" {} src="file:///{}" alt="{}" />"#,
                self.images,
                crop_type_attr,
                xml_escape::escape(source.display().to_string()),
                xml_escape::escape(alt_text)
            );
            self
        } else {
            // Win81 rejects the above xml, so we fall back to a simpler call
            self.image(source, alt_text)
        }
    }

    /// Add/Set a Hero image for the toast.
    ///
    /// This will be above the toast text and the icon.
    pub fn hero(mut self, source: &Path, alt_text: &str) -> Toast {
        if is_newer_than_windows81() {
            self.images = format!(
                r#"{}<image placement="Hero" src="file:///{}" alt="{}" />"#,
                self.images,
                xml_escape::escape(source.display().to_string()),
                xml_escape::escape(alt_text)
            );
            self
        } else {
            // win81 rejects the above xml, so we fall back to a simpler call
            self.image(source, alt_text)
        }
    }

    /// Add an image to the toast
    ///
    /// May be done many times.
    /// Will appear below text.
    pub fn image(mut self, source: &Path, alt_text: &str) -> Toast {
        if !is_newer_than_windows81() {
            // win81 cannot have more than 1 image and shows nothing if there is more than that
            self.images = String::new();
        }
        self.images = format!(
            r#"{}<image id="1" src="file:///{}" alt="{}" />"#,
            self.images,
            xml_escape::escape(source.display().to_string()),
            xml_escape::escape(alt_text)
        );
        self
    }

    /// Set the sound for the toast or silence it
    ///
    /// Default is [Sound::IM](enum.Sound.html)
    pub fn sound(mut self, src: Option<Sound>) -> Toast {
        self.audio = match src {
            None => "<audio silent=\"true\" />".to_owned(),
            Some(Sound::Default) => "".to_owned(),
            Some(Sound::Loop(sound)) => format!(
                r#"<audio loop="true" src="ms-winsoundevent:Notification.Looping.{sound}" />"#
            ),
            Some(Sound::Single(sound)) => {
                format!(r#"<audio src="ms-winsoundevent:Notification.Looping.{sound}" />"#)
            }
            Some(sound) => format!(r#"<audio src="ms-winsoundevent:Notification.{sound}" />"#),
        };

        self
    }

    /// Adds a button to the notification
    /// `content` is the text of the button.
    /// `action` will be sent as an argument [on_activated](Self::on_activated) when the button is clicked.
    pub fn add_button(mut self, content: &str, action: &str) -> Toast {
        self.buttons.push(Button {
            content: content.to_owned(),
            action: action.to_owned(),
        });
        self
    }

    /// Set the progress for the toast
    pub fn progress(mut self, progress: &Progress) -> Toast {
        self.progress = Some(progress.clone());
        self
    }

    // HACK: f is static so that we know the function is valid to call.
    //       this would be nice to remove at some point
    pub fn on_activated<F>(mut self, mut f: F) -> Self
    where
        F: FnMut(Option<String>) -> Result<()> + Send + 'static,
    {
        self.on_activated = Some(TypedEventHandler::new(move |_, insp| {
            let _ = f(Self::get_activated_action(&insp));
            Ok(())
        }));
        self
    }

    fn get_activated_action(insp: &Option<IInspectable>) -> Option<String> {
        if let Some(insp) = insp {
            if let Ok(args) = insp.cast::<ToastActivatedEventArgs>() {
                if let Ok(arguments) = args.Arguments() {
                    if !arguments.is_empty() {
                        return Some(arguments.to_string());
                    }
                }
            }
        }
        None
    }

    /// Set the function to be called when the toast is dismissed
    /// `f` will be called with the reason the toast was dismissed.
    /// If the toast was dismissed by the user, the reason will be `ToastDismissalReason::UserCanceled`.
    /// If the toast was dismissed by the application, the reason will be `ToastDismissalReason::ApplicationHidden`.
    /// If the toast was dismissed because it timed out, the reason will be `ToastDismissalReason::TimedOut`.
    /// If the reason is unknown, the reason will be `None`.
    ///
    /// # Example
    /// ```rust
    /// use tauri_winrt_notification::{Toast, ToastDismissalReason};
    ///
    /// let toast = Toast::new(Toast::POWERSHELL_APP_ID);
    /// toast.on_dismissed(|reason| {
    ///     match reason {
    ///         Some(ToastDismissalReason::UserCanceled) => println!("UserCanceled"),
    ///         Some(ToastDismissalReason::ApplicationHidden) => println!("ApplicationHidden"),
    ///         Some(ToastDismissalReason::TimedOut) => println!("TimedOut"),
    ///         _ => println!("Unknown"),
    ///     }
    ///     Ok(())
    /// }).show().expect("notification failed");
    /// ```
    pub fn on_dismissed<F>(mut self, f: F) -> Self
    where
        F: Fn(Option<ToastDismissalReason>) -> Result<()> + Send + 'static,
    {
        self.on_dismissed = Some(TypedEventHandler::new(move |_, args| {
            let _ = f(Self::get_dismissed_reason(&args));
            Ok(())
        }));
        self
    }

    fn get_dismissed_reason(
        args: &Option<ToastDismissedEventArgs>,
    ) -> Option<ToastDismissalReason> {
        if let Some(args) = args {
            if let Ok(reason) = args.Reason() {
                return Some(reason);
            }
        }
        None
    }

    fn create_template(&self) -> Result<ToastNotification> {
        //using this to get an instance of XmlDocument
        let toast_xml = XmlDocument::new()?;

        let template_binding = if is_newer_than_windows81() {
            "ToastGeneric"
        } else {
            // Need to do this or an empty placeholder will be shown if no image is set
            if self.images.is_empty() {
                "ToastText04"
            } else {
                "ToastImageAndText04"
            }
        };

        let progress = match self.progress {
            Some(_) => Progress::xml(),
            None => "",
        };

        let mut actions = String::new();
        if !self.buttons.is_empty() {
            let _ = write!(actions, "<actions>");
            for b in &self.buttons {
                let _ = write!(
                    actions,
                    "<action content='{}' arguments='{}'/>",
                    b.content, b.action
                );
            }
            let _ = write!(actions, "</actions>");
        }

        toast_xml.LoadXml(&HSTRING::from(format!(
            r#"<toast {} {}>
                <visual>
                    <binding template="{}">
                        {}
                        {}{}{}
                        {}
                    </binding>
                </visual>
                {}
                {}
            </toast>"#,
            self.duration,
            self.scenario,
            template_binding,
            self.images,
            self.title,
            self.line1,
            self.line2,
            progress,
            self.audio,
            actions
        )))?;

        // Create the toast
        ToastNotification::CreateToastNotification(&toast_xml).map_err(Into::into)
    }

    /// Update progress bar title, status, progress value, progress value string
    /// If the notification update is successful, the reason will be `NotificationUpdateResult::Succeeded`.
    /// If the update notification fails, the reason will be `NotificationUpdateResult::Failed`.
    /// If no notification is found, the reason will be `NotificationUpdateResult::NotificationNotFound`.
    ///
    /// # Example
    /// ```rust
    /// use std::{thread::sleep, time::Duration as StdDuration};
    /// use tauri_winrt_notification::{Toast, Progress};
    ///
    /// let mut progress = Progress {
    ///     tag: "my_tag".to_string(),
    ///     title: "video.mp4".to_string(),
    ///     status: "Transferring files...".to_string(),
    ///     value: 0.0,
    ///     value_string: "0/1000 MB".to_string(),
    /// };
    ///
    /// let toast = Toast::new(Toast::POWERSHELL_APP_ID).progress(&progress);
    /// toast.show().expect("notification failed");
    ///
    /// for i in 1..=10 {
    ///     sleep(StdDuration::from_secs(1));
    ///
    ///     progress.value = i as f32 / 10.0;
    ///     progress.value_string = format!("{}/1000 MB", i * 100);
    ///
    ///     if i == 10 {
    ///         progress.status = String::from("Completed!");
    ///     };
    ///
    ///     toast.set_progress(&progress).expect("failed to set notification progress");
    /// }
    /// ```
    pub fn set_progress(&self, progress: &Progress) -> Result<NotificationUpdateResult> {
        let map = StringMap::new()?;
        map.Insert(&HSTRING::from("progressTitle"), &progress.title())?;
        map.Insert(&HSTRING::from("progressStatus"), &progress.status())?;
        map.Insert(&HSTRING::from("progressValue"), &progress.value())?;
        map.Insert(
            &HSTRING::from("progressValueString"),
            &progress.value_string(),
        )?;

        let data = NotificationData::CreateNotificationDataWithValuesAndSequenceNumber(&map, 2)?;

        let toast_notifier =
            ToastNotificationManager::CreateToastNotifierWithId(&HSTRING::from(&self.app_id))?;

        toast_notifier
            .UpdateWithTag(&data, &progress.tag())
            .map_err(Into::into)
    }

    /// Display the toast on the screen
    pub fn show(&self) -> Result<()> {
        let toast_template = self.create_template()?;
        if let Some(handler) = &self.on_activated {
            toast_template.Activated(handler)?;
        }

        if let Some(handler) = &self.on_dismissed {
            toast_template.Dismissed(handler)?;
        }

        let toast_notifier =
            ToastNotificationManager::CreateToastNotifierWithId(&HSTRING::from(&self.app_id))?;

        if let Some(progress) = &self.progress {
            toast_template.SetTag(&progress.tag())?;

            let map = StringMap::new()?;
            map.Insert(&HSTRING::from("progressTitle"), &progress.title())?;
            map.Insert(&HSTRING::from("progressStatus"), &progress.status())?;
            map.Insert(&HSTRING::from("progressValue"), &progress.value())?;
            map.Insert(
                &HSTRING::from("progressValueString"),
                &progress.value_string(),
            )?;

            let data =
                NotificationData::CreateNotificationDataWithValuesAndSequenceNumber(&map, 1)?;
            toast_template.SetData(&data)?;
        }

        // Show the toast.
        let result = toast_notifier.Show(&toast_template).map_err(Into::into);
        std::thread::sleep(std::time::Duration::from_millis(10));
        result
    }
}

fn is_newer_than_windows81() -> bool {
    let os = windows_version::OsVersion::current();
    os.major > 6
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_toast() {
        let toast = Toast::new(Toast::POWERSHELL_APP_ID);
        toast
            .hero(
                &Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/flower.jpeg"),
                "flower",
            )
            .icon(
                &Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/chick.jpeg"),
                IconCrop::Circular,
                "chicken",
            )
            .title("title")
            .text1("line1")
            .text2("line2")
            .duration(Duration::Short)
            //.sound(Some(Sound::Loop(LoopableSound::Call)))
            //.sound(Some(Sound::SMS))
            .sound(None)
            .show()
            // silently consume errors
            .expect("notification failed");
    }
}
