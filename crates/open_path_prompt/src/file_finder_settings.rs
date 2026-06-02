use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{RegisterSetting, Settings};

/// Hard upper bound on the recency horizon, to keep timestamp arithmetic
/// well-behaved even if a user (or a buggy settings file) requests a huge
/// value. Matches the maximum documented in the settings schema.
const MAX_RECENCY_HORIZON_DAYS: u32 = 90;

#[derive(Debug, Clone, PartialEq, RegisterSetting)]
pub struct FileFinderSettings {
    pub file_icons: bool,
    pub modal_max_width: FileFinderWidth,
    pub skip_focus_for_active_in_search: bool,
    pub include_ignored: Option<bool>,
    pub include_channels: bool,
    /// Weight (clamped to [0.0, 1.0]) for the recency boost. 0.0 disables
    /// it entirely, restoring the post-PR-#12103 ranking.
    pub recency_boost: f32,
    /// Additive boost (clamped to [0.0, 1.0]) granted to files currently
    /// open in any pane.
    pub open_tab_boost: f32,
    /// How the recency boost decays with the age of the last visit.
    pub recency_decay: RecencyDecay,
    /// Number of days a file remains eligible for the recency boost since
    /// its last visit. Clamped to [1, MAX_RECENCY_HORIZON_DAYS].
    pub recency_horizon_days: u32,
    /// Validated path prefixes (each ends with `/`) that earn a positive
    /// directory boost.
    pub directory_priority: Vec<String>,
    /// Validated path prefixes (each ends with `/`) that earn a negative
    /// directory boost.
    pub directory_deprioritize: Vec<String>,
    /// Validated glob patterns whose matches earn a large additive boost.
    pub pinned_files: Vec<String>,
}

impl Settings for FileFinderSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let file_finder = content.file_finder.as_ref().unwrap();

        Self {
            file_icons: file_finder.file_icons.unwrap(),
            modal_max_width: file_finder.modal_max_width.unwrap().into(),
            skip_focus_for_active_in_search: file_finder.skip_focus_for_active_in_search.unwrap(),
            include_ignored: match file_finder.include_ignored.unwrap() {
                settings::IncludeIgnoredContent::All => Some(true),
                settings::IncludeIgnoredContent::Indexed => Some(false),
                settings::IncludeIgnoredContent::Smart => None,
            },
            include_channels: file_finder.include_channels.unwrap(),
            recency_boost: file_finder.recency_boost.unwrap_or(0.0).clamp(0.0, 1.0),
            open_tab_boost: file_finder.open_tab_boost.unwrap_or(0.0).clamp(0.0, 1.0),
            recency_decay: file_finder
                .recency_decay
                .map(RecencyDecay::from)
                .unwrap_or_default(),
            recency_horizon_days: file_finder
                .recency_horizon_days
                .unwrap_or(7)
                .clamp(1, MAX_RECENCY_HORIZON_DAYS),
            directory_priority: file_finder
                .directory_priority
                .clone()
                .map(filter_valid_prefixes)
                .unwrap_or_default(),
            directory_deprioritize: file_finder
                .directory_deprioritize
                .clone()
                .map(filter_valid_prefixes)
                .unwrap_or_default(),
            pinned_files: file_finder.pinned_files.clone().unwrap_or_default(),
        }
    }
}

/// Drop prefixes that do not end with `/` (with a one-time warning per
/// entry) so the rest of the ranking pipeline can assume valid input.
fn filter_valid_prefixes(prefixes: Vec<String>) -> Vec<String> {
    prefixes
        .into_iter()
        .filter(|p| {
            if p.ends_with('/') {
                true
            } else {
                log::warn!(
                    "file_finder: directory priority entry {p:?} does not end with `/`; ignoring",
                );
                false
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RecencyDecay {
    /// Boost falls off proportionally to elapsed time.
    #[default]
    Linear,
    /// Heavily favours very recent visits over the long tail.
    Exponential,
    /// Full boost inside the recency horizon, zero outside it.
    Step,
}

impl From<settings::RecencyDecayContent> for RecencyDecay {
    fn from(content: settings::RecencyDecayContent) -> Self {
        match content {
            settings::RecencyDecayContent::Linear => RecencyDecay::Linear,
            settings::RecencyDecayContent::Exponential => RecencyDecay::Exponential,
            settings::RecencyDecayContent::Step => RecencyDecay::Step,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum FileFinderWidth {
    #[default]
    Small,
    Medium,
    Large,
    XLarge,
    Full,
}

impl From<settings::FileFinderWidthContent> for FileFinderWidth {
    fn from(content: settings::FileFinderWidthContent) -> Self {
        match content {
            settings::FileFinderWidthContent::Small => FileFinderWidth::Small,
            settings::FileFinderWidthContent::Medium => FileFinderWidth::Medium,
            settings::FileFinderWidthContent::Large => FileFinderWidth::Large,
            settings::FileFinderWidthContent::XLarge => FileFinderWidth::XLarge,
            settings::FileFinderWidthContent::Full => FileFinderWidth::Full,
        }
    }
}
