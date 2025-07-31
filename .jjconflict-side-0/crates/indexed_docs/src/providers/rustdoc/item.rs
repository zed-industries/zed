use std::sync::Arc;

use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Serialize, Deserialize, EnumIter,
)]
#[serde(rename_all = "snake_case")]
pub enum RustdocItemKind {
    Mod,
    Macro,
    Struct,
    Enum,
    Constant,
    Trait,
    Function,
    TypeAlias,
    AttributeMacro,
    DeriveMacro,
}

impl RustdocItemKind {
    pub(crate) const fn class(&self) -> &'static str {
        match self {
            Self::Mod => "mod",
            Self::Macro => "macro",
            Self::Struct => "struct",
            Self::Enum => "enum",
            Self::Constant => "constant",
            Self::Trait => "trait",
            Self::Function => "fn",
            Self::TypeAlias => "type",
            Self::AttributeMacro => "attr",
            Self::DeriveMacro => "derive",
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct RustdocItem {
    pub kind: RustdocItemKind,
    /// The item path, up until the name of the item.
    pub path: Vec<Arc<str>>,
    /// The name of the item.
    pub name: Arc<str>,
}

impl RustdocItem {
    pub fn display(&self) -> String {
        let mut path_segments = self.path.clone();
        path_segments.push(self.name.clone());

        path_segments.join("::")
    }

    pub fn url_path(&self) -> String {
        let name = &self.name;
        let mut path_components = self.path.clone();

        match self.kind {
            RustdocItemKind::Mod => {
                path_components.push(name.clone());
                path_components.push("index.html".into());
            }
            RustdocItemKind::Macro
            | RustdocItemKind::Struct
            | RustdocItemKind::Enum
            | RustdocItemKind::Constant
            | RustdocItemKind::Trait
            | RustdocItemKind::Function
            | RustdocItemKind::TypeAlias
            | RustdocItemKind::AttributeMacro
            | RustdocItemKind::DeriveMacro => {
                path_components
                    .push(format!("{kind}.{name}.html", kind = self.kind.class()).into());
            }
        }

        path_components.join("/")
    }
}
