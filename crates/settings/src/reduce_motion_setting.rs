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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SettingsStore;
    use gpui::{TestAppContext, UpdateGlobal};
    use settings_content::ReduceMotion;

    fn init_test(cx: &mut TestAppContext) {
        let store = cx.update(|cx| SettingsStore::test(cx));
        cx.update(|cx| cx.set_global(store));
    }

    fn set_reduce_motion(cx: &mut TestAppContext, value: ReduceMotion) {
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.workspace.reduce_motion = Some(value);
                });
            });
        });
    }

    #[gpui::test]
    fn test_reduce_motion_on(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            assert!(ReduceMotionSetting(ReduceMotion::On).should_reduce_motion(cx));
        });
    }

    #[gpui::test]
    fn test_reduce_motion_off(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            assert!(!ReduceMotionSetting(ReduceMotion::Off).should_reduce_motion(cx));
        });
    }

    #[gpui::test]
    fn test_reduce_motion_system_delegates_to_platform(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            assert!(!ReduceMotionSetting(ReduceMotion::System).should_reduce_motion(cx));
        });
    }

    #[gpui::test]
    fn test_default_is_system(_cx: &mut TestAppContext) {
        assert_eq!(ReduceMotion::default(), ReduceMotion::System);
        assert_eq!(ReduceMotionSetting::default().0, ReduceMotion::System);
    }

    #[gpui::test]
    fn test_from_settings_reads_workspace(cx: &mut TestAppContext) {
        init_test(cx);

        set_reduce_motion(cx, ReduceMotion::On);
        cx.update(|cx| {
            assert_eq!(ReduceMotionSetting::get_global(cx).0, ReduceMotion::On);
        });

        set_reduce_motion(cx, ReduceMotion::Off);
        cx.update(|cx| {
            assert_eq!(ReduceMotionSetting::get_global(cx).0, ReduceMotion::Off);
        });
    }

    #[gpui::test]
    fn test_global_should_reduce_motion(cx: &mut TestAppContext) {
        init_test(cx);

        cx.update(|cx| assert!(!should_reduce_motion(cx)));

        set_reduce_motion(cx, ReduceMotion::On);
        cx.update(|cx| assert!(should_reduce_motion(cx)));

        set_reduce_motion(cx, ReduceMotion::Off);
        cx.update(|cx| assert!(!should_reduce_motion(cx)));
    }
}
