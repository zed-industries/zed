use std::any::TypeId;

use collections::HashMap;
use db::kvp::KeyValueStore;
use gpui::{App, AppContext as _, BorrowAppContext};
use util::ResultExt as _;

use crate::{FeatureFlag, FeatureFlagValue, ZED_DISABLE_STAFF};

const OVERRIDES_NAMESPACE: &str = "feature-flag-overrides";

pub struct FeatureFlagDescriptor {
    pub name: &'static str,
    pub is_presence: fn() -> bool,
    pub variants: fn() -> Vec<FeatureFlagVariant>,
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
                is_presence: <<$flag as $crate::FeatureFlag>::Value as $crate::FeatureFlagValue>::is_presence,
                variants: || {
                    <<$flag as $crate::FeatureFlag>::Value as $crate::FeatureFlagValue>::all_variants()
                        .iter()
                        .map(|v| $crate::FeatureFlagVariant {
                            override_key: <<$flag as $crate::FeatureFlag>::Value as $crate::FeatureFlagValue>::override_key(v),
                            label: <<$flag as $crate::FeatureFlag>::Value as $crate::FeatureFlagValue>::label(v),
                        })
                        .collect()
                },
                enabled_for_all: <$flag as $crate::FeatureFlag>::enabled_for_all,
                enabled_for_staff: <$flag as $crate::FeatureFlag>::enabled_for_staff,
                type_id: || std::any::TypeId::of::<$flag>(),
            }
        }
    };
}

pub type FlagOverride = Option<String>;

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
                let Some(raw) = scoped.read(name).log_err().flatten() else {
                    continue;
                };
                let Some(parsed) = decode_override(&raw) else {
                    continue;
                };
                overrides.insert(name.to_owned(), parsed);
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

    pub fn override_for(&self, flag_name: &str) -> Option<&FlagOverride> {
        self.overrides.get(flag_name)
    }

    pub fn set_override(&mut self, flag_name: &str, value: FlagOverride, cx: &mut App) {
        self.overrides.insert(flag_name.to_owned(), value.clone());
        self.persist_row(flag_name, Some(value), cx);
    }

    pub fn clear_override(&mut self, flag_name: &str, cx: &mut App) {
        if self.overrides.remove(flag_name).is_some() {
            self.persist_row(flag_name, None, cx);
        }
    }

    fn persist_row(&self, flag_name: &str, value: Option<FlagOverride>, cx: &mut App) {
        let Some(kvp) = self.kvp.clone() else {
            return;
        };
        let flag_name = flag_name.to_owned();
        db::write_and_log(cx, move || async move {
            let scoped = kvp.scoped(OVERRIDES_NAMESPACE);
            match value {
                Some(override_value) => {
                    scoped
                        .write(flag_name, encode_override(&override_value))
                        .await
                }
                None => scoped.delete(flag_name).await,
            }
        });
    }

    /// The resolved value of the flag for the current user, taking overrides,
    /// `enabled_for_all`, staff rules, and server flags into account in that
    /// order of precedence.
    pub fn flag_value<T: FeatureFlag>(&self) -> Option<T::Value> {
        // `enabled_for_all` always wins, including over user overrides.
        if T::enabled_for_all() {
            return Self::default_on_value::<T>();
        }

        if let Some(override_value) = self.overrides.get(T::NAME) {
            return match override_value {
                None => None,
                Some(key) => variant_from_key::<T::Value>(key),
            };
        }

        // Staff default: behave as if the "on" variant were set.
        if (cfg!(debug_assertions) || self.staff) && !*ZED_DISABLE_STAFF && T::enabled_for_staff() {
            return Self::default_on_value::<T>();
        }

        // Server-delivered flag.
        if let Some(wire) = self.server_flags.get(T::NAME) {
            return T::Value::from_wire(wire);
        }

        None
    }

    pub fn has_flag<T: FeatureFlag>(&self) -> bool {
        self.flag_value::<T>().is_some()
    }

    /// The override key the UI should show as "selected" for a flag whose
    /// concrete type isn't known (e.g. when rendering a generated list of
    /// descriptors in the configuration UI).
    ///
    /// Returns `None` if the flag resolves to "off" — either because the user
    /// explicitly disabled it or because nothing has enabled it.
    pub fn resolved_key(&self, descriptor: &FeatureFlagDescriptor) -> Option<&'static str> {
        let first_variant_key = || {
            let variants = (descriptor.variants)();
            variants.first().map(|v| v.override_key)
        };

        if (descriptor.enabled_for_all)() {
            return first_variant_key();
        }

        if let Some(override_value) = self.overrides.get(descriptor.name) {
            return match override_value {
                None => None,
                Some(requested) => (descriptor.variants)()
                    .into_iter()
                    .find(|v| v.override_key == requested.as_str())
                    .map(|v| v.override_key),
            };
        }

        if (cfg!(debug_assertions) || self.staff)
            && !*ZED_DISABLE_STAFF
            && (descriptor.enabled_for_staff)()
        {
            return first_variant_key();
        }

        if self.server_flags.contains_key(descriptor.name) {
            return first_variant_key();
        }

        None
    }

    /// Whether this flag is forced on by `enabled_for_all` and therefore not
    /// user-overridable. The UI uses this to render the row as disabled.
    pub fn is_forced_on(descriptor: &FeatureFlagDescriptor) -> bool {
        (descriptor.enabled_for_all)()
    }

    pub fn has_flag_default<T: FeatureFlag>() -> bool {
        if T::enabled_for_all() {
            return true;
        }
        cfg!(debug_assertions) && T::enabled_for_staff() && !*ZED_DISABLE_STAFF
    }

    fn default_on_value<T: FeatureFlag>() -> Option<T::Value> {
        // The "on" value is the first variant, by convention. For presence
        // flags this is PresenceFlag; for enums this is the first declared
        // variant — typically the one the flag used to gate before it became
        // multi-valued.
        T::Value::all_variants().first().cloned()
    }
}

fn variant_from_key<V: FeatureFlagValue>(key: &str) -> Option<V> {
    V::all_variants()
        .iter()
        .find(|v| v.override_key() == key)
        .cloned()
}

fn encode_override(value: &FlagOverride) -> String {
    value.clone().unwrap_or_default()
}

fn decode_override(raw: &str) -> Option<FlagOverride> {
    if raw.is_empty() {
        Some(None)
    } else {
        Some(Some(raw.to_owned()))
    }
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

    /// Example of an enum-valued flag. `Intensity::Low` is the default
    /// variant — staff / server / `enabled_for_all` all resolve to it, and
    /// `from_wire` also returns it. Users can override to any other variant
    /// via the configuration UI.
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
    fn disabled_override_beats_server_flag() {
        let mut store = FeatureFlagStore::default();
        store.update_server_flags(false, vec!["demo".to_string()]);
        store.overrides.insert(DemoFlag::NAME.to_string(), None);
        assert!(!store.has_flag::<DemoFlag>());
    }

    #[test]
    fn enabled_for_all_wins_over_override() {
        let mut store = FeatureFlagStore::default();
        store
            .overrides
            .insert(IntensityFlag::NAME.to_string(), None);
        assert_eq!(store.flag_value::<IntensityFlag>(), Some(Intensity::Low));
    }

    #[test]
    fn enum_override_selects_specific_variant() {
        let mut store = FeatureFlagStore::default();
        // Staff path would normally resolve to `Low`; the override pushes
        // us to `High` instead.
        store
            .overrides
            .insert("enum-demo".to_string(), Some("high".to_string()));

        struct EnumDemo;
        impl FeatureFlag for EnumDemo {
            const NAME: &'static str = "enum-demo";
            type Value = Intensity;
        }

        assert_eq!(store.flag_value::<EnumDemo>(), Some(Intensity::High));
    }

    #[test]
    fn unknown_variant_key_resolves_to_none() {
        let mut store = FeatureFlagStore::default();
        store
            .overrides
            .insert("enum-demo".to_string(), Some("nonsense".to_string()));

        struct EnumDemo;
        impl FeatureFlag for EnumDemo {
            const NAME: &'static str = "enum-demo";
            type Value = Intensity;
        }

        assert_eq!(store.flag_value::<EnumDemo>(), None);
    }

    #[test]
    fn override_enables_without_server_or_staff() {
        let mut store = FeatureFlagStore::default();
        store
            .overrides
            .insert(DemoFlag::NAME.to_string(), Some("on".to_string()));
        assert!(store.has_flag::<DemoFlag>());
    }

    #[test]
    fn encode_decode_roundtrip() {
        assert_eq!(decode_override(&encode_override(&None)), Some(None));
        assert_eq!(
            decode_override(&encode_override(&Some("on".to_string()))),
            Some(Some("on".to_string()))
        );
    }
}
