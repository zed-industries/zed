use gpui::{Hsla, WindowContext};
use refineable::Refineable;

use crate::{blue, color_alpha, grass, neutral, red, yellow, ActiveTheme};

#[derive(Clone, Debug)]
pub enum StatusColor {
    Conflict,
    Created,
    Deleted,
    Error,
    Hidden,
    Hint,
    Ignored,
    Info,
    Modified,
    Predictive,
    Renamed,
    Success,
    Unreachable,
    Warning,
}

impl StatusColor {
    pub fn fg(self, cx: &mut WindowContext) -> Hsla {
        match self {
            Self::Conflict => cx.theme().status().conflict,
            Self::Created => cx.theme().status().created,
            Self::Deleted => cx.theme().status().deleted,
            Self::Error => cx.theme().status().error,
            Self::Hidden => cx.theme().status().hidden,
            Self::Hint => cx.theme().status().hint,
            Self::Ignored => cx.theme().status().ignored,
            Self::Info => cx.theme().status().info,
            Self::Modified => cx.theme().status().modified,
            Self::Predictive => cx.theme().status().predictive,
            Self::Renamed => cx.theme().status().renamed,
            Self::Success => cx.theme().status().success,
            Self::Unreachable => cx.theme().status().unreachable,
            Self::Warning => cx.theme().status().warning,
        }
    }

    pub fn bg(self, cx: &mut WindowContext) -> Hsla {
        match self {
            Self::Conflict => color_alpha(cx.theme().status().conflict, 0.12),
            Self::Created => color_alpha(cx.theme().status().created, 0.12),
            Self::Deleted => color_alpha(cx.theme().status().deleted, 0.12),
            Self::Error => color_alpha(cx.theme().status().error, 0.12),
            Self::Hidden => color_alpha(cx.theme().status().hidden, 0.12),
            Self::Hint => color_alpha(cx.theme().status().hint, 0.12),
            Self::Ignored => color_alpha(cx.theme().status().ignored, 0.12),
            Self::Info => color_alpha(cx.theme().status().info, 0.12),
            Self::Modified => color_alpha(cx.theme().status().modified, 0.12),
            Self::Predictive => color_alpha(cx.theme().status().predictive, 0.12),
            Self::Renamed => color_alpha(cx.theme().status().renamed, 0.12),
            Self::Success => color_alpha(cx.theme().status().success, 0.12),
            Self::Unreachable => color_alpha(cx.theme().status().unreachable, 0.12),
            Self::Warning => color_alpha(cx.theme().status().warning, 0.12),
        }
    }
}

#[derive(Refineable, Clone, Debug)]
#[refineable(Debug, serde::Deserialize)]
pub struct StatusColors {
    /// Indicates some kind of conflict, like a file changed on disk while it was open, or
    /// merge conflicts in a Git repository.
    pub conflict: Hsla,

    /// Indicates something new, like a new file added to a Git repository.
    pub created: Hsla,

    /// Indicates that something no longer exists, like a deleted file.
    pub deleted: Hsla,

    /// Indicates a system error, a failed operation or a diagnostic error.
    pub error: Hsla,

    /// Represents a hidden status, such as a file being hidden in a file tree.
    pub hidden: Hsla,

    /// Indicates a hint or some kind of additional information.
    pub hint: Hsla,

    /// Indicates that something is deliberately ignored, such as a file or operation ignored by Git.
    pub ignored: Hsla,

    /// Represents informational status updates or messages.
    pub info: Hsla,

    /// Indicates a changed or altered status, like a file that has been edited.
    pub modified: Hsla,

    /// Indicates something that is predicted, like automatic code completion, or generated code.
    pub predictive: Hsla,

    /// Represents a renamed status, such as a file that has been renamed.
    pub renamed: Hsla,

    /// Indicates a successful operation or task completion.
    pub success: Hsla,

    /// Indicates some kind of unreachable status, like a block of code that can never be reached.
    pub unreachable: Hsla,

    /// Represents a warning status, like an operation that is about to fail.
    pub warning: Hsla,
}

impl Default for StatusColors {
    /// Don't use this!
    /// We have to have a default to be `[refineable::Refinable]`.
    /// todo!("Find a way to not need this for Refinable")
    fn default() -> Self {
        Self::dark()
    }
}

pub struct DiagnosticColors {
    pub error: Hsla,
    pub warning: Hsla,
    pub info: Hsla,
}

pub struct GitStatusColors {
    pub created: Hsla,
    pub deleted: Hsla,
    pub modified: Hsla,
    pub renamed: Hsla,
    pub conflict: Hsla,
    pub ignored: Hsla,
}

impl StatusColors {
    pub fn dark() -> Self {
        Self {
            conflict: red().dark().step_9(),
            created: grass().dark().step_9(),
            deleted: red().dark().step_9(),
            error: red().dark().step_9(),
            hidden: neutral().dark().step_9(),
            hint: blue().dark().step_9(),
            ignored: neutral().dark().step_9(),
            info: blue().dark().step_9(),
            modified: yellow().dark().step_9(),
            predictive: neutral().dark_alpha().step_9(),
            renamed: blue().dark().step_9(),
            success: grass().dark().step_9(),
            unreachable: neutral().dark().step_10(),
            warning: yellow().dark().step_9(),
        }
    }

    pub fn light() -> Self {
        Self {
            conflict: red().light().step_9(),
            created: grass().light().step_9(),
            deleted: red().light().step_9(),
            error: red().light().step_9(),
            hidden: neutral().light().step_9(),
            hint: blue().light().step_9(),
            ignored: neutral().light().step_9(),
            info: blue().light().step_9(),
            modified: yellow().light().step_9(),
            predictive: neutral().light_alpha().step_9(),
            renamed: blue().light().step_9(),
            success: grass().light().step_9(),
            unreachable: neutral().light().step_10(),
            warning: yellow().light().step_9(),
        }
    }

    pub fn diagnostic(&self) -> DiagnosticColors {
        DiagnosticColors {
            error: self.error,
            warning: self.warning,
            info: self.info,
        }
    }

    pub fn git(&self) -> GitStatusColors {
        GitStatusColors {
            created: self.created,
            deleted: self.deleted,
            modified: self.modified,
            renamed: self.renamed,
            conflict: self.conflict,
            ignored: self.ignored,
        }
    }
}
