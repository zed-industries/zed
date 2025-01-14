use std::path::PathBuf;

use collections::HashMap;
use gpui::SharedString;

use crate::Appearance;

/// A family of icon themes.
pub struct IconThemeFamily {
    /// The unique ID for the icon theme family.
    pub id: String,
    /// The name of the icon theme family.
    pub name: SharedString,
    /// The author of the icon theme family.
    pub author: SharedString,
    /// The list of icon themes in the family.
    pub themes: Vec<IconTheme>,
}

/// An icon theme.
pub struct IconTheme {
    /// The unique ID for the icon theme.
    pub id: String,
    /// The name of the icon theme.
    pub name: SharedString,
    /// The appearance of the icon theme (e.g., light or dark).
    pub appearance: Appearance,
    /// The mapping of file types to icon definitions.
    pub file_icons: HashMap<String, IconDefinition>,
}

/// An icon definition.
pub struct IconDefinition {
    /// The path to the icon file.
    pub path: PathBuf,
}
