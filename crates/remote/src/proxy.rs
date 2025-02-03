use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProxyLaunchError {
    #[error("Attempted reconnect, but server not running.")]
    ServerNotRunning,
}

impl ProxyLaunchError {
    pub fn to_exit_code(&self) -> i32 {
        match self {
            // We're using 90 as the exit code, because 0-78 are often taken
            // by shells and other conventions and >128 also has certain meanings
            // in certain contexts.
            Self::ServerNotRunning => 90,
        }
    }

    pub fn from_exit_code(exit_code: i32) -> Option<Self> {
        match exit_code {
            90 => Some(Self::ServerNotRunning),
            _ => None,
        }
    }
}
