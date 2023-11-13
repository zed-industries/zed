use crate::StatusColors;

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
}
