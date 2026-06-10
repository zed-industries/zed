//! The catalog of states Zed Sim can launch.
//!
//! Phase 1 (MVP) ships two states that need no changes to Zed itself. The
//! fabricated plan states (Pro, Trial, etc.) are listed in the UI as
//! "coming soon" and land in Phase 2 via an injection build.

/// A launchable simulation state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SimState {
    /// Signed out, pristine first-run onboarding.
    NewUser,
    /// Fresh profile that goes through the real sign-in flow.
    SignedIn,
}

impl SimState {
    /// Resolves a state from the id used by the control UI.
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "new_user" => Some(Self::NewUser),
            "signed_in" => Some(Self::SignedIn),
            _ => None,
        }
    }

    pub fn launch_config(self) -> LaunchConfig {
        match self {
            // Both MVP states isolate credentials: a unique keychain key means a
            // disposable session can never read or overwrite the user's real
            // saved login, even if they choose to sign in while exploring.
            SimState::NewUser => LaunchConfig {
                isolate_credentials: true,
                server_url: None,
            },
            SimState::SignedIn => LaunchConfig {
                isolate_credentials: true,
                server_url: None,
            },
        }
    }
}

/// How a given state shapes the launched Zed process and its profile.
pub struct LaunchConfig {
    /// Write a unique `credentials_url` into the profile so the disposable
    /// session never touches the user's real saved login in the OS keychain.
    pub isolate_credentials: bool,
    /// Override the backend via the `server_url` setting. `None` uses Zed's
    /// default (production). Reserved for the optional preview path.
    pub server_url: Option<String>,
}
