use crate::error::ErrorKind;

/// Levels of Urgency.
///
/// # Specification
/// > Developers must use their own judgement when deciding the urgency of a notification. Typically, if the majority of programs are using the same level for a specific type of urgency, other applications should follow them.
/// >
/// > For low and normal urgencies, server implementations may display the notifications how they choose. They should, however, have a sane expiration timeout dependent on the urgency level.
/// >
/// > **Critical notifications should not automatically expire**, as they are things that the user will most likely want to know about. They should only be closed when the user dismisses them, for example, by clicking on the notification.
///
/// <cite> — see [Galago](http://www.galago-project.org/specs/notification/0.9/x320.html) or [Gnome](https://developer.gnome.org/notification-spec/#urgency-levels) specification.</cite>
///
/// # Example
/// ```no_run
/// # use notify_rust::*;
/// # fn _doc() -> Result<(), Box<dyn std::error::Error>> {
/// Notification::new()
///     .summary("oh no")
///     .icon("dialog-warning")
///     .urgency(Urgency::Critical)
///     .show()?;
/// # Ok(())
/// # }
/// ```
///
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Urgency {
    /// The behavior for `Low` urgency depends on the notification server.
    Low = 0,
    /// The behavior for `Normal` urgency depends on the notification server.
    Normal = 1,
    /// A critical notification will not time out.
    Critical = 2,
}

impl TryFrom<&str> for Urgency {
    type Error = crate::error::Error;

    #[rustfmt::skip]
    fn try_from(string: &str) -> Result<Urgency, Self::Error> {
        match string.to_lowercase().as_ref() {
            "low"      |
            "lo"       => Ok(Urgency::Low),
            "normal"   |
            "medium"   => Ok(Urgency::Normal),
            "critical" |
            "high"     |
            "hi"       => Ok(Urgency::Critical),
            _ => Err(ErrorKind::Conversion(format!("invalid input {string:?}")).into())
        }
    }
}

impl From<Option<u64>> for Urgency {
    fn from(maybe_int: Option<u64>) -> Urgency {
        match maybe_int {
            Some(0) => Urgency::Low,
            Some(x) if x >= 2 => Urgency::Critical,
            _ => Urgency::Normal,
        }
    }
}

// TODO: remove this in v5.0
// #[cfg(not(feature = "server"))]
impl From<u64> for Urgency {
    fn from(int: u64) -> Urgency {
        match int {
            0 => Urgency::Low,
            1 => Urgency::Normal,
            2..=u64::MAX => Urgency::Critical,
        }
    }
}

// // TODO: make this the default in v5.0
// #[cfg(feature = "server")]
// impl From<u8> for Urgency {
//     fn from(int: u8) -> Urgency {
//         match int {
//             0 => Urgency::Low,
//             1 => Urgency::Normal,
//             2..=std::u8::MAX => Urgency::Critical,
//         }
//     }
// }
