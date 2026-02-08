use crate::{self as settings, settings_content::ReduceMotion};
use settings::{RegisterSetting, Settings};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, RegisterSetting)]
pub struct ReduceMotionSetting(ReduceMotion);

impl ReduceMotionSetting {
    pub fn value(&self) -> ReduceMotion {
        self.0
    }

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
        ReduceMotionSetting(settings.workspace.reduce_motion.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{SettingsStore, default_settings};
    use gpui::{App, TestAppContext, UpdateGlobal};
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

    #[test]
    fn test_reduce_motion_default_is_system() {
        assert_eq!(ReduceMotion::default(), ReduceMotion::System);
    }

    #[test]
    fn test_reduce_motion_deserialize() {
        let cases: &[(&str, ReduceMotion)] = &[
            (r#""system""#, ReduceMotion::System),
            (r#""on""#, ReduceMotion::On),
            (r#""off""#, ReduceMotion::Off),
            // serde(alias = "true") matches the JSON string "true", not the boolean true
            (r#""true""#, ReduceMotion::On),
            (r#""false""#, ReduceMotion::Off),
        ];

        for (json, expected) in cases {
            assert_eq!(
                serde_json::from_str::<ReduceMotion>(json).unwrap(),
                *expected,
                "for JSON: {json}",
            );
        }
    }

    #[test]
    fn test_reduce_motion_deserialize_rejects_invalid_values() {
        assert!(serde_json::from_str::<ReduceMotion>("true").is_err());
        assert!(serde_json::from_str::<ReduceMotion>("false").is_err());
        assert!(serde_json::from_str::<ReduceMotion>(r#""bogus""#).is_err());
        assert!(serde_json::from_str::<ReduceMotion>("42").is_err());
        assert!(serde_json::from_str::<ReduceMotion>("null").is_err());
    }

    #[test]
    fn test_reduce_motion_serialize_round_trip() {
        for variant in [ReduceMotion::System, ReduceMotion::On, ReduceMotion::Off] {
            let json = serde_json::to_string(&variant).unwrap();
            let deserialized: ReduceMotion = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, deserialized);
        }
    }

    #[gpui::test]
    fn test_should_reduce_motion_variants(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            assert!(ReduceMotionSetting(ReduceMotion::On).should_reduce_motion(cx));
            assert!(!ReduceMotionSetting(ReduceMotion::Off).should_reduce_motion(cx));
            assert_eq!(
                ReduceMotionSetting(ReduceMotion::System).should_reduce_motion(cx),
                cx.should_reduce_motion(),
            );
        });
    }

    #[gpui::test]
    fn test_from_settings_with_each_variant(cx: &mut TestAppContext) {
        init_test(cx);

        for variant in [ReduceMotion::On, ReduceMotion::Off, ReduceMotion::System] {
            set_reduce_motion(cx, variant);
            cx.update(|cx| {
                assert_eq!(ReduceMotionSetting::get_global(cx).value(), variant);
            });
        }
    }

    #[gpui::test]
    fn test_global_should_reduce_motion(cx: &mut TestAppContext) {
        init_test(cx);

        cx.update(|cx| assert!(!should_reduce_motion(cx)));

        set_reduce_motion(cx, ReduceMotion::On);
        cx.update(|cx| assert!(should_reduce_motion(cx)));

        set_reduce_motion(cx, ReduceMotion::Off);
        cx.update(|cx| assert!(!should_reduce_motion(cx)));

        set_reduce_motion(cx, ReduceMotion::System);
        cx.update(|cx| assert!(!should_reduce_motion(cx)));
    }

    #[gpui::test]
    fn test_settings_store_parses_reduce_motion_from_json(cx: &mut App) {
        let mut store = SettingsStore::new(cx, &default_settings());
        store.register_setting::<ReduceMotionSetting>();

        let cases: &[(&str, ReduceMotion)] = &[
            (r#"{ "reduce_motion": "on" }"#, ReduceMotion::On),
            (r#"{ "reduce_motion": "off" }"#, ReduceMotion::Off),
            (r#"{ "reduce_motion": "system" }"#, ReduceMotion::System),
            (r#"{ "reduce_motion": "true" }"#, ReduceMotion::On),
            (r#"{ "reduce_motion": "false" }"#, ReduceMotion::Off),
            (r#"{}"#, ReduceMotion::System),
        ];

        for (json, expected) in cases {
            store.set_user_settings(json, cx).unwrap();
            assert_eq!(store.get::<ReduceMotionSetting>(None).value(), *expected, "for JSON: {json}");
        }
    }

    #[gpui::test]
    fn test_settings_store_assign_json_before_register(cx: &mut App) {
        let mut store = SettingsStore::new(cx, &default_settings());

        store
            .set_user_settings(r#"{ "reduce_motion": "on" }"#, cx)
            .unwrap();
        store.register_setting::<ReduceMotionSetting>();

        assert_eq!(
            store.get::<ReduceMotionSetting>(None).value(),
            ReduceMotion::On,
        );
    }
}
