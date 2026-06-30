//! The static (no-backend) state catalog.
//!
//! These two states need no changes to Zed and no configuration. The dynamic
//! impersonation states are sourced from `config::AppConfig` instead.

/// A launchable state that requires only a fresh local profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SimState {
    /// Signed out, pristine first-run onboarding.
    NewUser,
    /// A fresh profile that goes through the real sign-in flow.
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
}
