use gpui_shared_string::SharedString;

/// A reference to an environment variable that reads its value from the process
/// environment on each access.
///
/// This avoids caching stale values when the process environment is updated
/// after construction (e.g., when shell environment variables are loaded
/// asynchronously at startup).
#[derive(Clone)]
pub struct EnvVar {
    pub name: SharedString,
}

impl EnvVar {
    pub fn new(name: SharedString) -> Self {
        Self { name }
    }

    /// Read the current value of the environment variable.
    /// Returns `None` if the variable is unset or empty.
    pub fn value(&self) -> Option<String> {
        let value = std::env::var(self.name.as_str()).ok();
        if value.as_ref().is_some_and(|v| v.is_empty()) {
            None
        } else {
            value
        }
    }

    pub fn or(self, other: EnvVar) -> EnvVar {
        if self.value().is_some() { self } else { other }
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
///
/// Note: Unlike `env_var!`, this captures the value once at first access and does not re-read.
#[macro_export]
macro_rules! bool_env_var {
    ($name:expr) => {
        ::std::sync::LazyLock::new(|| $crate::EnvVar::new(($name).into()).value().is_some())
    };
}
