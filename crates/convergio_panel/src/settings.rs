//! Settings for the Convergio Panel

use gpui::{App, Pixels};
use ui::px;
use workspace::dock::DockPosition;

/// Convergio Edition determines the available feature set
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ConvergioEdition {
    /// Base edition with all 54 agents
    #[default]
    Base,
    /// Education edition with accessibility focus
    Education,
    /// Enterprise edition with custom agent sets
    Enterprise,
}

impl ConvergioEdition {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Base => "Convergio Studio",
            Self::Education => "Convergio Studio Edu",
            Self::Enterprise => "Convergio Studio Enterprise",
        }
    }

    /// Returns the default agent pack for this edition
    pub fn default_pack(&self) -> &'static str {
        match self {
            Self::Base => "enterprise",     // All agents
            Self::Education => "education", // Accessibility-focused
            Self::Enterprise => "custom",   // Configurable
        }
    }
}

/// Settings for the Convergio Panel
pub struct ConvergioPanelSettings {
    pub button: bool,
    pub dock: DockPosition,
    pub default_width: Pixels,
    pub edition: ConvergioEdition,
}

impl ConvergioPanelSettings {
    pub fn get_global(_cx: &App) -> Self {
        Self::default()
    }

    /// Get the current edition from compile-time feature flags
    pub fn detect_edition() -> ConvergioEdition {
        #[cfg(feature = "convergio-education")]
        return ConvergioEdition::Education;

        #[cfg(feature = "convergio-enterprise")]
        return ConvergioEdition::Enterprise;

        #[cfg(not(any(feature = "convergio-education", feature = "convergio-enterprise")))]
        ConvergioEdition::Base
    }
}

impl Default for ConvergioPanelSettings {
    fn default() -> Self {
        Self {
            button: true,
            dock: DockPosition::Left,
            default_width: px(260.),
            edition: Self::detect_edition(),
        }
    }
}

pub fn init(_cx: &mut App) {
    // Settings registration will be added later
}
