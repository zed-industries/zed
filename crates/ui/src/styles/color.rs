use crate::{Label, LabelCommon, component_prelude::*, v_flex};
use documented::{DocumentedFields, DocumentedVariants};
use gpui::{App, Hsla, IntoElement, ParentElement, Styled};
use theme::ActiveTheme;

/// Sets a color that has a consistent meaning across all themes.
#[derive(
    Debug,
    Default,
    Eq,
    PartialEq,
    Copy,
    Clone,
    RegisterComponent,
    Documented,
    DocumentedFields,
    DocumentedVariants,
)]
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
    /// A color used for all debugger UI elements.
    Debugger,
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
    /// A version control color used to indicate a newly added file or content in version control.
    VersionControlAdded,
    /// A version control color used to indicate conflicting changes that need resolution.
    VersionControlConflict,
    /// A version control color used to indicate a file or content that has been deleted in version control.
    VersionControlDeleted,
    /// A version control color used to indicate files or content that is being ignored by version control.
    VersionControlIgnored,
    /// A version control color used to indicate modified files or content in version control.
    VersionControlModified,
    /// A color used to indicate a warning condition.
    Warning,
}

impl Color {
    /// Returns the Color's HSLA value.
    pub fn color(&self, cx: &App) -> Hsla {
        match self {
            Color::Default => cx.theme().colors().text,
            Color::Muted => cx.theme().colors().text_muted,
            Color::Created => cx.theme().status().created,
            Color::Modified => cx.theme().status().modified,
            Color::Conflict => cx.theme().status().conflict,
            Color::Ignored => cx.theme().status().ignored,
            Color::Debugger => cx.theme().colors().debugger_accent,
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
            Color::VersionControlAdded => cx.theme().colors().version_control_added,
            Color::VersionControlConflict => cx.theme().colors().version_control_conflict,
            Color::VersionControlDeleted => cx.theme().colors().version_control_deleted,
            Color::VersionControlIgnored => cx.theme().colors().version_control_ignored,
            Color::VersionControlModified => cx.theme().colors().version_control_modified,
            Color::Warning => cx.theme().status().warning,
            Color::Custom(color) => *color,
        }
    }
}

impl From<Hsla> for Color {
    fn from(color: Hsla) -> Self {
        Color::Custom(color)
    }
}

impl Component for Color {
    fn scope() -> ComponentScope {
        ComponentScope::Utilities
    }

    fn description() -> Option<&'static str> {
        Some(Color::DOCS)
    }

    fn preview(_window: &mut gpui::Window, _cx: &mut App) -> Option<gpui::AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    example_group_with_title(
                        "Text Colors",
                        vec![
                            single_example(
                                "Default",
                                Label::new("Default text color")
                                    .color(Color::Default)
                                    .into_any_element(),
                            )
                            .description(Color::Default.get_variant_docs()),
                            single_example(
                                "Muted",
                                Label::new("Muted text color")
                                    .color(Color::Muted)
                                    .into_any_element(),
                            )
                            .description(Color::Muted.get_variant_docs()),
                            single_example(
                                "Accent",
                                Label::new("Accent text color")
                                    .color(Color::Accent)
                                    .into_any_element(),
                            )
                            .description(Color::Accent.get_variant_docs()),
                            single_example(
                                "Disabled",
                                Label::new("Disabled text color")
                                    .color(Color::Disabled)
                                    .into_any_element(),
                            )
                            .description(Color::Disabled.get_variant_docs()),
                        ],
                    ),
                    example_group_with_title(
                        "Status Colors",
                        vec![
                            single_example(
                                "Success",
                                Label::new("Success status")
                                    .color(Color::Success)
                                    .into_any_element(),
                            )
                            .description(Color::Success.get_variant_docs()),
                            single_example(
                                "Warning",
                                Label::new("Warning status")
                                    .color(Color::Warning)
                                    .into_any_element(),
                            )
                            .description(Color::Warning.get_variant_docs()),
                            single_example(
                                "Error",
                                Label::new("Error status")
                                    .color(Color::Error)
                                    .into_any_element(),
                            )
                            .description(Color::Error.get_variant_docs()),
                            single_example(
                                "Info",
                                Label::new("Info status")
                                    .color(Color::Info)
                                    .into_any_element(),
                            )
                            .description(Color::Info.get_variant_docs()),
                        ],
                    ),
                    example_group_with_title(
                        "Version Control Colors",
                        vec![
                            single_example(
                                "Created",
                                Label::new("Created item")
                                    .color(Color::Created)
                                    .into_any_element(),
                            )
                            .description(Color::Created.get_variant_docs()),
                            single_example(
                                "Modified",
                                Label::new("Modified item")
                                    .color(Color::Modified)
                                    .into_any_element(),
                            )
                            .description(Color::Modified.get_variant_docs()),
                            single_example(
                                "Deleted",
                                Label::new("Deleted item")
                                    .color(Color::Deleted)
                                    .into_any_element(),
                            )
                            .description(Color::Deleted.get_variant_docs()),
                            single_example(
                                "Conflict",
                                Label::new("Conflict item")
                                    .color(Color::Conflict)
                                    .into_any_element(),
                            )
                            .description(Color::Conflict.get_variant_docs()),
                        ],
                    ),
                ])
                .into_any_element(),
        )
    }
}
