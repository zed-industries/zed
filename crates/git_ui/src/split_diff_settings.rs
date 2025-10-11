use gpui::{Pixels, px};
use serde::{Deserialize, Serialize};
use settings::{
    GitIntralineMode, GitSplitDiffViewMode, GitWhitespaceMode, Settings, SettingsContent,
};

// Re-export types from settings crate for convenience
pub use settings::{
    GitIntralineMode as IntralineMode, GitSplitDiffSettingsContent as SplitDiffSettingsContent,
    GitSplitDiffViewMode as SplitDiffViewMode, GitWhitespaceMode as WhitespaceMode,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SplitDiffSettings {
    pub default_view: SplitDiffViewMode,
    pub context_lines: u32,
    pub ignore_whitespace: WhitespaceMode,
    pub sync_scroll: bool,
    pub intraline: IntralineMode,
    pub word_wrap: bool,
    pub default_width: Pixels,
    pub default_height: Pixels,
}

impl Default for SplitDiffSettings {
    fn default() -> Self {
        Self {
            default_view: SplitDiffViewMode::Split,
            context_lines: 3,
            ignore_whitespace: WhitespaceMode::None,
            sync_scroll: true,
            intraline: IntralineMode::Word,
            word_wrap: false,
            default_width: px(800.0),
            default_height: px(600.0),
        }
    }
}

impl Settings for SplitDiffSettings {
    fn from_settings(content: &SettingsContent) -> Self {
        let git_split_diff = content.git_split_diff.as_ref();

        Self {
            default_view: git_split_diff
                .and_then(|c| c.default_view.clone())
                .unwrap_or(GitSplitDiffViewMode::Split),
            context_lines: git_split_diff.and_then(|c| c.context_lines).unwrap_or(3),
            ignore_whitespace: git_split_diff
                .and_then(|c| c.ignore_whitespace.clone())
                .unwrap_or(GitWhitespaceMode::None),
            sync_scroll: git_split_diff.and_then(|c| c.sync_scroll).unwrap_or(true),
            intraline: git_split_diff
                .and_then(|c| c.intraline.clone())
                .unwrap_or(GitIntralineMode::Word),
            word_wrap: git_split_diff.and_then(|c| c.word_wrap).unwrap_or(false),
            default_width: git_split_diff
                .and_then(|c| c.default_width)
                .map(px)
                .unwrap_or(px(800.0)),
            default_height: git_split_diff
                .and_then(|c| c.default_height)
                .map(px)
                .unwrap_or(px(600.0)),
        }
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut SettingsContent) {}
}

// TODO: Implement SettingsUi once the API is stable
// impl settings::SettingsUi for SplitDiffSettings { ... }
