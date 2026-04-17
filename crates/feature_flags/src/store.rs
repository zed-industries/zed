use std::any::TypeId;

use collections::HashMap;
use db::kvp::KeyValueStore;
use gpui::{App, AppContext as _, BorrowAppContext};
use util::ResultExt as _;

use crate::{FeatureFlag, FeatureFlagValue, ZED_DISABLE_STAFF};

const OVERRIDES_NAMESPACE: &str = "feature-flag-overrides";

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

pub type FlagOverride = String;

#[derive(Default)]
pub struct FeatureFlagStore {
    staff: bool,
    server_flags: HashMap<String, String>,
    overrides: HashMap<String, FlagOverride>,
    kvp: Option<KeyValueStore>,
}

impl FeatureFlagStore {
    pub fn init(cx: &mut App) {
        let kvp = KeyValueStore::global(cx);
        cx.update_default_global::<FeatureFlagStore, _>(|store, _| {
            store.kvp = Some(kvp.clone());
        });

        let known: Vec<&'static str> = Self::known_flags().map(|d| d.name).collect();
        let load_kvp = kvp.clone();
        let loader = cx.background_spawn(async move {
            let scoped = load_kvp.scoped(OVERRIDES_NAMESPACE);
            let mut overrides = HashMap::default();
            for name in known {
                let Some(value) = scoped.read(name).log_err().flatten() else {
                    continue;
                };
                overrides.insert(name.to_owned(), value);
            }
            overrides
        });
        cx.spawn(async move |cx| {
            let overrides = loader.await;
            cx.update(|cx| {
                cx.update_default_global::<FeatureFlagStore, _>(|store, _| {
                    store.overrides = overrides;
                });
            });
        })
        .detach();
    }

    pub fn known_flags() -> impl Iterator<Item = &'static FeatureFlagDescriptor> {
        let mut seen = collections::HashSet::default();
        inventory::iter::<FeatureFlagDescriptor>().filter(move |d| seen.insert((d.type_id)()))
    }

    pub fn is_staff(&self) -> bool {
        self.staff
    }

    pub fn set_staff(&mut self, staff: bool) {
        self.staff = staff;
    }

    pub fn update_server_flags(&mut self, staff: bool, flags: Vec<String>) {
        self.staff = staff;
        self.server_flags.clear();
        for flag in flags {
            self.server_flags.insert(flag.clone(), flag);
        }
    }

    pub fn override_for(&self, flag_name: &str) -> Option<&str> {
        self.overrides.get(flag_name).map(String::as_str)
    }

    pub fn set_override(&mut self, flag_name: &str, override_key: String, cx: &mut App) {
        self.overrides
            .insert(flag_name.to_owned(), override_key.clone());
        self.persist_row(flag_name, Some(override_key), cx);
    }

    /// Removes any override for the given flag and persists the change.
    pub fn clear_override(&mut self, flag_name: &str, cx: &mut App) {
        if self.overrides.remove(flag_name).is_some() {
            self.persist_row(flag_name, None, cx);
        }
    }

    fn persist_row(&self, flag_name: &str, value: Option<String>, cx: &mut App) {
        let Some(kvp) = self.kvp.clone() else {
            return;
        };
        let flag_name = flag_name.to_owned();
        db::write_and_log(cx, move || async move {
            let scoped = kvp.scoped(OVERRIDES_NAMESPACE);
            match value {
                Some(value) => scoped.write(flag_name, value).await,
                None => scoped.delete(flag_name).await,
            }
        });
    }


    pub fn try_flag_value<T: FeatureFlag>(&self) -> Option<T::Value> {
        // `enabled_for_all` always wins, including over user overrides.
        if T::enabled_for_all() {
            return Some(T::Value::on_variant());
        }

        if let Some(override_key) = self.overrides.get(T::NAME) {
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
    /// [`FeatureFlagAppExt::flag_value`].
    pub fn has_flag<T: FeatureFlag>(&self) -> bool {
        self.try_flag_value::<T>()
            .is_some_and(|v| v == T::Value::on_variant())
    }

    /// Mirrors the resolution order of [`Self::try_flag_value`], but falls
    /// back to the [`Default`] variant when no rule applies so the UI always
    /// shows *something* selected — matching what
    /// [`crate::FeatureFlagAppExt::flag_value`] would return.
    pub fn resolved_key(&self, descriptor: &FeatureFlagDescriptor) -> &'static str {
        let on_variant_key = (descriptor.on_variant_key)();

        if (descriptor.enabled_for_all)() {
            return on_variant_key;
        }

        if let Some(requested) = self.overrides.get(descriptor.name) {
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

    #[test]
    fn server_flag_enables_presence() {
        let mut store = FeatureFlagStore::default();
        assert!(!store.has_flag::<DemoFlag>());
        store.update_server_flags(false, vec!["demo".to_string()]);
        assert!(store.has_flag::<DemoFlag>());
    }

    #[test]
    fn off_override_beats_server_flag() {
        let mut store = FeatureFlagStore::default();
        store.update_server_flags(false, vec!["demo".to_string()]);
        store
            .overrides
            .insert(DemoFlag::NAME.to_string(), "off".to_string());
        assert!(!store.has_flag::<DemoFlag>());
        assert_eq!(
            store.try_flag_value::<DemoFlag>(),
            Some(PresenceFlag::Off)
        );
    }

    #[test]
    fn enabled_for_all_wins_over_override() {
        let mut store = FeatureFlagStore::default();
        store
            .overrides
            .insert(IntensityFlag::NAME.to_string(), "high".to_string());
        assert_eq!(store.try_flag_value::<IntensityFlag>(), Some(Intensity::Low));
    }

    #[test]
    fn enum_override_selects_specific_variant() {
        let mut store = FeatureFlagStore::default();
        // Staff path would normally resolve to `Low`; the override pushes
        // us to `High` instead.
        store
            .overrides
            .insert("enum-demo".to_string(), "high".to_string());

        struct EnumDemo;
        impl FeatureFlag for EnumDemo {
            const NAME: &'static str = "enum-demo";
            type Value = Intensity;
        }

        assert_eq!(store.try_flag_value::<EnumDemo>(), Some(Intensity::High));
    }

    #[test]
    fn unknown_variant_key_resolves_to_none() {
        let mut store = FeatureFlagStore::default();
        store
            .overrides
            .insert("enum-demo".to_string(), "nonsense".to_string());

        struct EnumDemo;
        impl FeatureFlag for EnumDemo {
            const NAME: &'static str = "enum-demo";
            type Value = Intensity;
        }

        assert_eq!(store.try_flag_value::<EnumDemo>(), None);
    }

    #[test]
    fn on_override_enables_without_server_or_staff() {
        let mut store = FeatureFlagStore::default();
        store
            .overrides
            .insert(DemoFlag::NAME.to_string(), "on".to_string());
        assert!(store.has_flag::<DemoFlag>());
    }

    /// No rule applies, so the store's `try_flag_value` returns `None`. The
    /// `FeatureFlagAppExt::flag_value` path (used by most callers) falls
    /// back to [`Default`], which for `PresenceFlag` is `Off`.
    #[test]
    fn presence_flag_defaults_to_off() {
        let store = FeatureFlagStore::default();
        assert_eq!(store.try_flag_value::<DemoFlag>(), None);
        assert_eq!(PresenceFlag::default(), PresenceFlag::Off);
    }
}
