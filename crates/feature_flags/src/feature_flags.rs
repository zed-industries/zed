// Makes the derive macro's reference to `::feature_flags::FeatureFlagValue`
// resolve when the macro is invoked inside this crate itself.
extern crate self as feature_flags;

mod flags;
mod store;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::LazyLock;

use gpui::{App, Context, Global, Subscription, Window};

pub use feature_flags_macros::EnumFeatureFlag;
pub use flags::*;
pub use store::*;

pub static ZED_DISABLE_STAFF: LazyLock<bool> = LazyLock::new(|| {
    std::env::var("ZED_DISABLE_STAFF").is_ok_and(|value| !value.is_empty() && value != "0")
});

impl Global for FeatureFlagStore {}

pub trait FeatureFlagValue: Sized + Clone + Eq + std::fmt::Debug + Send + Sync + 'static {
    /// Every possible value for this flag, in the order the UI should display them.
    fn all_variants() -> &'static [Self];

    /// A stable identifier for this variant used when persisting overrides.
    fn override_key(&self) -> &'static str;

    fn from_wire(wire: &str) -> Option<Self>;

    /// Human-readable label for use in the configuration UI.
    fn label(&self) -> &'static str {
        self.override_key()
    }

    /// Whether this value type represents a simple on/off flag.
    ///
    /// The default is `false` because most `FeatureFlagValue` impls are enums.
    /// The [`PresenceFlag`] impl overrides this to `true`, which is what the UI
    /// uses to decide between a checkbox and a radio group.
    fn is_presence() -> bool {
        false
    }
}

/// Value type for simple presence/absence feature flags.
///
/// A flag whose `type Value = PresenceFlag` behaves identically to the
/// pre-existing `bool` feature flags: `cx.has_flag::<T>()` returns `true` iff
/// the server included the flag name in its response (or staff / `enabled_for_all`
/// rules apply).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PresenceFlag;

impl FeatureFlagValue for PresenceFlag {
    fn all_variants() -> &'static [Self] {
        &[PresenceFlag]
    }

    fn override_key(&self) -> &'static str {
        "on"
    }

    fn from_wire(_: &str) -> Option<Self> {
        Some(PresenceFlag)
    }

    fn is_presence() -> bool {
        true
    }
}

/// To create a feature flag, implement this trait on a trivial type and use it as
/// a generic parameter when called [`FeatureFlagAppExt::has_flag`].
///
/// Feature flags are enabled for members of Zed staff by default. To disable this behavior
/// so you can test flags being disabled, set ZED_DISABLE_STAFF=1 in your environment,
/// which will force Zed to treat the current user as non-staff.
pub trait FeatureFlag {
    const NAME: &'static str;

    /// The type of value this flag can hold. Use [`PresenceFlag`] for simple
    /// on/off flags.
    type Value: FeatureFlagValue;

    /// Returns whether this feature flag is enabled for Zed staff.
    fn enabled_for_staff() -> bool {
        true
    }

    /// Returns whether this feature flag is enabled for everyone.
    ///
    /// This is generally done on the server, but we provide this as a way to entirely enable a feature flag client-side
    /// without needing to remove all of the call sites.
    fn enabled_for_all() -> bool {
        false
    }

    /// Subscribes the current view to changes in the feature flag store, so
    /// that any mutation of flags or overrides will trigger a re-render.
    ///
    /// The returned subscription is immediately detached; use [`observe_flag`]
    /// directly if you need to hold onto the subscription.
    fn watch<V: 'static>(cx: &mut Context<V>) {
        cx.observe_global::<FeatureFlagStore>(|_, cx| cx.notify())
            .detach();
    }
}

pub trait FeatureFlagViewExt<V: 'static> {
    fn observe_flag<T: FeatureFlag, F>(&mut self, window: &Window, callback: F) -> Subscription
    where
        F: Fn(bool, &mut V, &mut Window, &mut Context<V>) + Send + Sync + 'static;

    fn when_flag_enabled<T: FeatureFlag>(
        &mut self,
        window: &mut Window,
        callback: impl Fn(&mut V, &mut Window, &mut Context<V>) + Send + Sync + 'static,
    );
}

impl<V> FeatureFlagViewExt<V> for Context<'_, V>
where
    V: 'static,
{
    fn observe_flag<T: FeatureFlag, F>(&mut self, window: &Window, callback: F) -> Subscription
    where
        F: Fn(bool, &mut V, &mut Window, &mut Context<V>) + 'static,
    {
        self.observe_global_in::<FeatureFlagStore>(window, move |v, window, cx| {
            let store = cx.global::<FeatureFlagStore>();
            callback(store.has_flag::<T>(), v, window, cx);
        })
    }

    fn when_flag_enabled<T: FeatureFlag>(
        &mut self,
        window: &mut Window,
        callback: impl Fn(&mut V, &mut Window, &mut Context<V>) + Send + Sync + 'static,
    ) {
        if self
            .try_global::<FeatureFlagStore>()
            .is_some_and(|f| f.has_flag::<T>())
        {
            self.defer_in(window, move |view, window, cx| {
                callback(view, window, cx);
            });
            return;
        }
        let subscription = Rc::new(RefCell::new(None));
        let inner = self.observe_global_in::<FeatureFlagStore>(window, {
            let subscription = subscription.clone();
            move |v, window, cx| {
                let store = cx.global::<FeatureFlagStore>();
                if store.has_flag::<T>() {
                    callback(v, window, cx);
                    subscription.take();
                }
            }
        });
        subscription.borrow_mut().replace(inner);
    }
}

#[derive(Debug)]
pub struct OnFlagsReady {
    pub is_staff: bool,
}

pub trait FeatureFlagAppExt {
    fn update_flags(&mut self, staff: bool, flags: Vec<String>);
    fn set_staff(&mut self, staff: bool);
    fn has_flag<T: FeatureFlag>(&self) -> bool;
    fn flag_value<T: FeatureFlag>(&self) -> Option<T::Value>;
    fn is_staff(&self) -> bool;

    fn on_flags_ready<F>(&mut self, callback: F) -> Subscription
    where
        F: FnMut(OnFlagsReady, &mut App) + 'static;

    fn observe_flag<T: FeatureFlag, F>(&mut self, callback: F) -> Subscription
    where
        F: FnMut(bool, &mut App) + 'static;
}

impl FeatureFlagAppExt for App {
    fn update_flags(&mut self, staff: bool, flags: Vec<String>) {
        let store = self.default_global::<FeatureFlagStore>();
        store.update_server_flags(staff, flags);
    }

    fn set_staff(&mut self, staff: bool) {
        let store = self.default_global::<FeatureFlagStore>();
        store.set_staff(staff);
    }

    fn has_flag<T: FeatureFlag>(&self) -> bool {
        self.try_global::<FeatureFlagStore>()
            .map(|store| store.has_flag::<T>())
            .unwrap_or_else(|| FeatureFlagStore::has_flag_default::<T>())
    }

    fn flag_value<T: FeatureFlag>(&self) -> Option<T::Value> {
        self.try_global::<FeatureFlagStore>()
            .and_then(|store| store.flag_value::<T>())
    }

    fn is_staff(&self) -> bool {
        self.try_global::<FeatureFlagStore>()
            .map(|store| store.is_staff())
            .unwrap_or(false)
    }

    fn on_flags_ready<F>(&mut self, mut callback: F) -> Subscription
    where
        F: FnMut(OnFlagsReady, &mut App) + 'static,
    {
        self.observe_global::<FeatureFlagStore>(move |cx| {
            let store = cx.global::<FeatureFlagStore>();
            callback(
                OnFlagsReady {
                    is_staff: store.is_staff(),
                },
                cx,
            );
        })
    }

    fn observe_flag<T: FeatureFlag, F>(&mut self, mut callback: F) -> Subscription
    where
        F: FnMut(bool, &mut App) + 'static,
    {
        self.observe_global::<FeatureFlagStore>(move |cx| {
            let store = cx.global::<FeatureFlagStore>();
            callback(store.has_flag::<T>(), cx);
        })
    }
}
