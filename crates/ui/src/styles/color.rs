use gpui::{App, Hsla};
use theme::ActiveTheme;

/// Sets a color that has a consistent meaning across all themes.
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
pub enum Color {
    #[default]
    /// The default text color. Might be known as "foreground" or "primary" in
    /// some theme systems.
    ///
    /// For less emphasis, consider using [`Color::Muted`] or [`Color::Hidden`].
    Default,
    /// A text color used for accents, such as links or highlights.
    Accent,
    /// A color used to indicate a conflict, such as a version control merge conflict, or a conflict between a file in the editor and the file system.
    Conflict,
    /// A color used to indicate a newly created item, such as a new file in
    /// version control, or a new file on disk.
    Created,
    /// It is highly, HIGHLY recommended not to use this! Using this color
    /// means detaching it from any semantic meaning across themes.
    ///
    /// A custom color specified by an HSLA value.
    Custom(Hsla),
    /// A color used to indicate a deleted item, such as a file removed from version control.
    Deleted,
    /// A color used for disabled UI elements or text, like a disabled button or menu item.
    Disabled,
    /// A color used to indicate an error condition, or something the user
    /// cannot do. In very rare cases, it might be used to indicate dangerous or
    /// destructive action.
    Error,
    /// A color used for elements that represent something that is hidden, like
    /// a hidden file, or an element that should be visually de-emphasized.
    Hidden,
    /// A color used for hint or suggestion text, often a blue color. Use this
    /// color to represent helpful, or semantically neutral information.
    Hint,
    /// A color used for items that are intentionally ignored, such as files ignored by version control.
    Ignored,
    /// A color used for informational messages or status indicators, often a blue color.
    Info,
    /// A color used to indicate a modified item, such as an edited file, or a modified entry in version control.
    Modified,
    /// A color used for text or UI elements that should be visually muted or de-emphasized.
    ///
    /// For more emphasis, consider using [`Color::Default`].
    ///
    /// For less emphasis, consider using [`Color::Hidden`].
    Muted,
    /// A color used for placeholder text in input fields.
    Placeholder,
    /// A color associated with a specific player number.
    Player(u32),
    /// A color used to indicate selected text or UI elements.
    Selected,
    /// A color used to indicate a successful operation or status.
    Success,
    /// A color used to indicate a warning condition.
    Warning,
    VersionControlAdded,
    VersionControlModified,
    VersionControlDeleted,
}

impl Color {
    /// Returns the Color's HSLA value based on the current theme.
    ///
    /// Using this method over directly accessing it from the theme
    /// allows us to dynamically adjust colors for accessibility and such.
    pub fn color(&self, cx: &App) -> Hsla {
        let high_contrast_enabled = true;

        let mut color = match self {
            Color::Default => cx.theme().colors().text,
            Color::Muted => cx.theme().colors().text_muted,
            Color::Created => cx.theme().status().created,
            Color::Modified => cx.theme().status().modified,
            Color::Conflict => cx.theme().status().conflict,
            Color::Ignored => cx.theme().status().ignored,
            Color::Deleted => cx.theme().status().deleted,
            Color::Disabled => cx.theme().colors().text_disabled,
            Color::Hidden => cx.theme().status().hidden,
            Color::Hint => cx.theme().status().hint,
            Color::Info => cx.theme().status().info,
            Color::Placeholder => cx.theme().colors().text_placeholder,
            Color::Accent => cx.theme().colors().text_accent,
            Color::Player(i) => cx.theme().styles.player.color_for_participant(*i).cursor,
            Color::Error => cx.theme().status().error,
            Color::Selected => cx.theme().colors().text_accent,
            Color::Success => cx.theme().status().success,
            Color::Warning => cx.theme().status().warning,
            Color::Custom(color) => *color,
            Color::VersionControlAdded => cx.theme().colors().version_control_added,
            Color::VersionControlModified => cx.theme().colors().version_control_modified,
            Color::VersionControlDeleted => cx.theme().colors().version_control_deleted,
        };

        if high_contrast_enabled {
            let start_green_hue = 70.0;
            let end_green_hue = 150.0;
            let start_blue_hue = 210.0;
            let end_blue_hue = 270.0;
            let hue = color.h * 360.0;

            if hue > start_green_hue && hue < end_green_hue {
                let ratio = (hue - start_green_hue) / (end_green_hue - start_green_hue);
                let new_hue = start_blue_hue + ratio * (end_blue_hue - start_blue_hue);
                color.h = new_hue / 360.0;
            }
        }

        color
    }
}

impl From<Hsla> for Color {
    fn from(color: Hsla) -> Self {
        Color::Custom(color)
    }
}
