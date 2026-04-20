use std::any::TypeId;
use std::sync::Arc;

use collections::HashMap;
use fs::Fs;
use gpui::{App, BorrowAppContext, Subscription};
use settings::{Settings, SettingsStore, update_settings_file};

use crate::{FeatureFlag, FeatureFlagValue, FeatureFlagsSettings, ZED_DISABLE_STAFF};

pub struct FeatureFlagDescriptor {
    pub name: &'static str,
    pub variants: fn() -> Vec<FeatureFlagVariant>,
    pub on_variant_key: fn() -> &'static str,
    pub default_variant_key: fn() -> &'static str,
    pub enabled_for_all: fn() -> bool,
    pub enabled_for_staff: fn() -> bool,
    pub type_id: fn() -> TypeId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FeatureFlagVariant {
    pub override_key: &'static str,
    pub label: &'static str,
}

inventory::collect!(FeatureFlagDescriptor);

#[doc(hidden)]
pub mod __private {
    pub use inventory;
}

/// Submits a [`FeatureFlagDescriptor`] for this flag so it shows up in the
/// configuration UI and in `FeatureFlagStore::known_flags()`.
#[macro_export]
macro_rules! register_feature_flag {
    ($flag:ty) => {
        $crate::__private::inventory::submit! {
            $crate::FeatureFlagDescriptor {
                name: <$flag as $crate::FeatureFlag>::NAME,
                variants: || {
                    <<$flag as $crate::FeatureFlag>::Value as $crate::FeatureFlagValue>::all_variants()
                        .iter()
                        .map(|v| $crate::FeatureFlagVariant {
                            override_key: <<$flag as $crate::FeatureFlag>::Value as $crate::FeatureFlagValue>::override_key(v),
                            label: <<$flag as $crate::FeatureFlag>::Value as $crate::FeatureFlagValue>::label(v),
                        })
                        .collect()
                },
                on_variant_key: || {
                    <<$flag as $crate::FeatureFlag>::Value as $crate::FeatureFlagValue>::override_key(
                        &<<$flag as $crate::FeatureFlag>::Value as $crate::FeatureFlagValue>::on_variant(),
                    )
                },
                default_variant_key: || {
                    <<$flag as $crate::FeatureFlag>::Value as $crate::FeatureFlagValue>::override_key(
                        &<<$flag as $crate::FeatureFlag>::Value as ::std::default::Default>::default(),
                    )
                },
                enabled_for_all: <$flag as $crate::FeatureFlag>::enabled_for_all,
                enabled_for_staff: <$flag as $crate::FeatureFlag>::enabled_for_staff,
                type_id: || std::any::TypeId::of::<$flag>(),
            }
        }
    };
}

#[derive(Default)]
pub struct FeatureFlagStore {
    staff: bool,
    server_flags: HashMap<String, String>,
    server_flags_received: bool,

    _settings_subscription: Option<Subscription>,
}

impl FeatureFlagStore {
    pub fn init(cx: &mut App) {
        let subscription = cx.observe_global::<SettingsStore>(|cx| {
            // Touch the global so anything observing `FeatureFlagStore` re-runs
            cx.update_default_global::<FeatureFlagStore, _>(|_, _| {});
        });

        cx.update_default_global::<FeatureFlagStore, _>(|store, _| {
            store._settings_subscription = Some(subscription);
        });
    }

    pub fn known_flags() -> impl Iterator<Item = &'static FeatureFlagDescriptor> {
        let mut seen = collections::HashSet::default();
        inventory::iter::<FeatureFlagDescriptor>().filter(move |d| seen.insert((d.type_id)()))
    }

    pub fn is_staff(&self) -> bool {
        self.staff
    }

    pub fn server_flags_received(&self) -> bool {
        self.server_flags_received
    }

    pub fn set_staff(&mut self, staff: bool) {
        self.staff = staff;
    }

    pub fn update_server_flags(&mut self, staff: bool, flags: Vec<String>) {
        self.staff = staff;
        self.server_flags_received = true;
        self.server_flags.clear();
        for flag in flags {
            self.server_flags.insert(flag.clone(), flag);
        }
    }

    /// The user's override key for this flag, read directly from
    /// [`FeatureFlagsSettings`].
    pub fn override_for<'a>(flag_name: &str, cx: &'a App) -> Option<&'a str> {
        FeatureFlagsSettings::get_global(cx)
            .overrides
            .get(flag_name)
            .map(String::as_str)
    }

    /// Applies an override by writing to `settings.json`. The store's own
    /// `overrides` field will be updated when the settings-store observer
    /// fires. Pass the [`FeatureFlagValue::override_key`] of the variant
    /// you want forced.
    pub fn set_override(flag_name: &str, override_key: String, fs: Arc<dyn Fs>, cx: &App) {
        let flag_name = flag_name.to_owned();
        update_settings_file(fs, cx, move |content, _| {
            content
                .feature_flags
                .get_or_insert_default()
                .insert(flag_name, override_key);
        });
    }

    /// Removes any override for the given flag from `settings.json`. Leaves
    /// an empty `"feature_flags"` object rather than removing the key
    /// entirely so the user can see it's still a meaningful settings surface.
    pub fn clear_override(flag_name: &str, fs: Arc<dyn Fs>, cx: &App) {
        let flag_name = flag_name.to_owned();
        update_settings_file(fs, cx, move |content, _| {
            if let Some(map) = content.feature_flags.as_mut() {
                map.remove(&flag_name);
            }
        });
    }

    /// The resolved value of the flag for the current user, taking overrides,
    /// `enabled_for_all`, staff rules, and server flags into account in that
    /// order of precedence. Overrides are read directly from
    /// [`FeatureFlagsSettings`].
    pub fn try_flag_value<T: FeatureFlag>(&self, cx: &App) -> Option<T::Value> {
        // `enabled_for_all` always wins, including over user overrides.
        if T::enabled_for_all() {
            return Some(T::Value::on_variant());
        }

        if let Some(override_key) = FeatureFlagsSettings::get_global(cx).overrides.get(T::NAME) {
            return variant_from_key::<T::Value>(override_key);
        }

        // Staff default: resolve to the enabled variant.
        if (cfg!(debug_assertions) || self.staff) && !*ZED_DISABLE_STAFF && T::enabled_for_staff() {
            return Some(T::Value::on_variant());
        }

        // Server-delivered flag.
        if let Some(wire) = self.server_flags.get(T::NAME) {
            return T::Value::from_wire(wire);
        }

        None
    }

    /// Whether the flag resolves to its "on" value. Best for presence-style
    /// flags. For enum flags with meaningful non-default variants, prefer
    /// [`crate::FeatureFlagAppExt::flag_value`].
    pub fn has_flag<T: FeatureFlag>(&self, cx: &App) -> bool {
        self.try_flag_value::<T>(cx)
            .is_some_and(|v| v == T::Value::on_variant())
    }

    /// Mirrors the resolution order of [`Self::try_flag_value`], but falls
    /// back to the [`Default`] variant when no rule applies so the UI always
    /// shows *something* selected — matching what
    /// [`crate::FeatureFlagAppExt::flag_value`] would return.
    pub fn resolved_key(&self, descriptor: &FeatureFlagDescriptor, cx: &App) -> &'static str {
        let on_variant_key = (descriptor.on_variant_key)();

        if (descriptor.enabled_for_all)() {
            return on_variant_key;
        }

        if let Some(requested) = FeatureFlagsSettings::get_global(cx)
            .overrides
            .get(descriptor.name)
        {
            if let Some(variant) = (descriptor.variants)()
                .into_iter()
                .find(|v| v.override_key == requested.as_str())
            {
                return variant.override_key;
            }
        }

        if (cfg!(debug_assertions) || self.staff)
            && !*ZED_DISABLE_STAFF
            && (descriptor.enabled_for_staff)()
        {
            return on_variant_key;
        }

        if self.server_flags.contains_key(descriptor.name) {
            return on_variant_key;
        }

        (descriptor.default_variant_key)()
    }

    /// Whether this flag is forced on by `enabled_for_all` and therefore not
    /// user-overridable. The UI uses this to render the row as disabled.
    pub fn is_forced_on(descriptor: &FeatureFlagDescriptor) -> bool {
        (descriptor.enabled_for_all)()
    }

    /// Fallback used when the store isn't installed as a global yet (e.g. very
    /// early in startup). Matches the pre-existing default behavior.
    pub fn has_flag_default<T: FeatureFlag>() -> bool {
        if T::enabled_for_all() {
            return true;
        }
        cfg!(debug_assertions) && T::enabled_for_staff() && !*ZED_DISABLE_STAFF
    }
}

fn variant_from_key<V: FeatureFlagValue>(key: &str) -> Option<V> {
    V::all_variants()
        .iter()
        .find(|v| v.override_key() == key)
        .cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EnumFeatureFlag, FeatureFlag, PresenceFlag};
    use gpui::UpdateGlobal;
    use settings::SettingsStore;

    struct DemoFlag;
    impl FeatureFlag for DemoFlag {
        const NAME: &'static str = "demo";
        type Value = PresenceFlag;
        fn enabled_for_staff() -> bool {
            false
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug, EnumFeatureFlag)]
    enum Intensity {
        #[default]
        Low,
        High,
    }

    struct IntensityFlag;
    impl FeatureFlag for IntensityFlag {
        const NAME: &'static str = "intensity";
        type Value = Intensity;
        fn enabled_for_all() -> bool {
            true
        }
    }

    fn init_settings_store(cx: &mut App) {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        SettingsStore::update_global(cx, |store, _| {
            store.register_setting::<FeatureFlagsSettings>();
        });
    }

    fn set_override(name: &str, value: &str, cx: &mut App) {
        SettingsStore::update_global(cx, |store: &mut SettingsStore, cx| {
            store.update_user_settings(cx, |content| {
                content
                    .feature_flags
                    .get_or_insert_default()
                    .insert(name.to_string(), value.to_string());
            });
        });
    }

    #[gpui::test]
    fn server_flag_enables_presence(cx: &mut App) {
        init_settings_store(cx);
        let mut store = FeatureFlagStore::default();
        assert!(!store.has_flag::<DemoFlag>(cx));
        store.update_server_flags(false, vec!["demo".to_string()]);
        assert!(store.has_flag::<DemoFlag>(cx));
    }

    #[gpui::test]
    fn off_override_beats_server_flag(cx: &mut App) {
        init_settings_store(cx);
        let mut store = FeatureFlagStore::default();
        store.update_server_flags(false, vec!["demo".to_string()]);
        set_override(DemoFlag::NAME, "off", cx);
        assert!(!store.has_flag::<DemoFlag>(cx));
        assert_eq!(
            store.try_flag_value::<DemoFlag>(cx),
            Some(PresenceFlag::Off)
        );
    }

    #[gpui::test]
    fn enabled_for_all_wins_over_override(cx: &mut App) {
        init_settings_store(cx);
        let store = FeatureFlagStore::default();
        set_override(IntensityFlag::NAME, "high", cx);
        assert_eq!(
            store.try_flag_value::<IntensityFlag>(cx),
            Some(Intensity::Low)
        );
    }

    #[gpui::test]
    fn enum_override_selects_specific_variant(cx: &mut App) {
        init_settings_store(cx);
        let store = FeatureFlagStore::default();
        // Staff path would normally resolve to `Low`; the override pushes
        // us to `High` instead.
        set_override("enum-demo", "high", cx);

        struct EnumDemo;
        impl FeatureFlag for EnumDemo {
            const NAME: &'static str = "enum-demo";
            type Value = Intensity;
        }

        assert_eq!(store.try_flag_value::<EnumDemo>(cx), Some(Intensity::High));
    }

    #[gpui::test]
    fn unknown_variant_key_resolves_to_none(cx: &mut App) {
        init_settings_store(cx);
        let store = FeatureFlagStore::default();
        set_override("enum-demo", "nonsense", cx);

        struct EnumDemo;
        impl FeatureFlag for EnumDemo {
            const NAME: &'static str = "enum-demo";
            type Value = Intensity;
        }

        assert_eq!(store.try_flag_value::<EnumDemo>(cx), None);
    }

    #[gpui::test]
    fn on_override_enables_without_server_or_staff(cx: &mut App) {
        init_settings_store(cx);
        let store = FeatureFlagStore::default();
        set_override(DemoFlag::NAME, "on", cx);
        assert!(store.has_flag::<DemoFlag>(cx));
    }

    /// No rule applies, so the store's `try_flag_value` returns `None`. The
    /// `FeatureFlagAppExt::flag_value` path (used by most callers) falls
    /// back to [`Default`], which for `PresenceFlag` is `Off`.
    #[gpui::test]
    fn presence_flag_defaults_to_off(cx: &mut App) {
        init_settings_store(cx);
        let store = FeatureFlagStore::default();
        assert_eq!(store.try_flag_value::<DemoFlag>(cx), None);
        assert_eq!(PresenceFlag::default(), PresenceFlag::Off);
    }

    #[gpui::test]
    fn on_flags_ready_waits_for_server_flags(cx: &mut gpui::TestAppContext) {
        use crate::FeatureFlagAppExt;
        use std::cell::Cell;
        use std::rc::Rc;

        cx.update(|cx| {
            init_settings_store(cx);
            FeatureFlagStore::init(cx);
        });

        let fired = Rc::new(Cell::new(false));
        cx.update({
            let fired = fired.clone();
            |cx| cx.on_flags_ready(move |_, _| fired.set(true)).detach()
        });

        // Settings-triggered no-op touch must not fire on_flags_ready.
        cx.update(|cx| cx.update_default_global::<FeatureFlagStore, _>(|_, _| {}));
        cx.run_until_parked();
        assert!(!fired.get());

        // Server flags arrive — now it should fire.
        cx.update(|cx| cx.update_flags(true, vec![]));
        cx.run_until_parked();
        assert!(fired.get());
    }
}
