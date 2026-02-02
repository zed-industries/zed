use crate::{self as settings, settings_content::ReduceMotion};
use settings::{RegisterSetting, Settings};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, RegisterSetting)]
pub struct ReduceMotionSetting(pub ReduceMotion);

impl ReduceMotionSetting {
    pub fn should_reduce_motion(&self, cx: &gpui::App) -> bool {
        match self.0 {
            ReduceMotion::System => cx.should_reduce_motion(),
            ReduceMotion::On => true,
            ReduceMotion::Off => false,
        }
    }
}

pub fn should_reduce_motion(cx: &gpui::App) -> bool {
    ReduceMotionSetting::get_global(cx).should_reduce_motion(cx)
}

impl Settings for ReduceMotionSetting {
    fn from_settings(settings: &crate::settings_content::SettingsContent) -> Self {
        ReduceMotionSetting(settings.workspace.reduce_motion.unwrap_or_default())
    }
}
