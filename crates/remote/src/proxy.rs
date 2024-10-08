use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProxyLaunchError {
    #[error("Attempted reconnect. Server is already running.")]
    ServerNotRunning,
}

impl ProxyLaunchError {
    pub fn to_exit_code(&self) -> i32 {
        match self {
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
