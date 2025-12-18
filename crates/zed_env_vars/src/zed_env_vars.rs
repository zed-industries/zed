use gpui::SharedString;
use std::sync::LazyLock;

/// Whether Zed is running in stateless mode.
/// When true, Zed will use in-memory databases instead of persistent storage.
pub static ZED_STATELESS: LazyLock<bool> = bool_env_var!("ZED_STATELESS");

#[derive(Clone)]
pub struct EnvVar {
    pub name: SharedString,
    /// Value of the environment variable. Also `None` when set to an empty string.
    pub value: Option<String>,
}

impl EnvVar {
    pub fn new(name: SharedString) -> Self {
        let value = std::env::var(name.as_str()).ok();
        if value.as_ref().is_some_and(|v| v.is_empty()) {
            Self { name, value: None }
        } else {
            Self { name, value }
        }
    }

    pub fn or(self, other: EnvVar) -> EnvVar {
        if self.value.is_some() { self } else { other }
    }
}

/// Creates a `LazyLock<EnvVar>` expression for use in a `static` declaration.
#[macro_export]
macro_rules! env_var {
    ($name:expr) => {
        ::std::sync::LazyLock::new(|| $crate::EnvVar::new(($name).into()))
    };
}

/// Generates a `LazyLock<bool>` expression for use in a `static` declaration. Checks if the
/// environment variable exists and is non-empty.
#[macro_export]
macro_rules! bool_env_var {
    ($name:expr) => {
        ::std::sync::LazyLock::new(|| $crate::EnvVar::new(($name).into()).value.is_some())
    };
}
