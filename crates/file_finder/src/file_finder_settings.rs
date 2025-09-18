use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use util::MergeFrom;

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct FileFinderSettings {
    pub file_icons: bool,
    pub modal_max_width: FileFinderWidth,
    pub skip_focus_for_active_in_search: bool,
    pub include_ignored: Option<bool>,
}

impl Settings for FileFinderSettings {
    fn from_defaults(content: &settings::SettingsContent, _cx: &mut ui::App) -> Self {
        let file_finder = content.file_finder.as_ref().unwrap();

        Self {
            file_icons: file_finder.file_icons.unwrap(),
            modal_max_width: file_finder.modal_max_width.unwrap().into(),
            skip_focus_for_active_in_search: file_finder.skip_focus_for_active_in_search.unwrap(),
            include_ignored: file_finder.include_ignored.flatten(),
        }
    }

    fn refine(&mut self, content: &settings::SettingsContent, _cx: &mut ui::App) {
        let Some(file_finder) = content.file_finder.as_ref() else {
            return;
        };

        self.file_icons.merge_from(&file_finder.file_icons);
        self.modal_max_width
            .merge_from(&file_finder.modal_max_width.map(Into::into));
        self.skip_focus_for_active_in_search
            .merge_from(&file_finder.skip_focus_for_active_in_search);
        self.include_ignored
            .merge_from(&file_finder.include_ignored);
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
