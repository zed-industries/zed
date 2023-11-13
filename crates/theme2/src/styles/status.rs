use gpui::Hsla;

use crate::{blue, grass, neutral, red, yellow, StatusColors};

impl Default for StatusColors {
    /// Don't use this!
    /// We have to have a default for StatusColors to be `[refineable::Refinable]`.
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
