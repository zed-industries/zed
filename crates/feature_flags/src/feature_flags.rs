mod flags;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::LazyLock;

use gpui::{App, Context, Global, Subscription, Window};

pub use flags::*;

#[derive(Default)]
struct FeatureFlags {
    flags: Vec<String>,
    staff: bool,
}

pub static ZED_DISABLE_STAFF: LazyLock<bool> = LazyLock::new(|| {
    std::env::var("ZED_DISABLE_STAFF").is_ok_and(|value| !value.is_empty() && value != "0")
});

impl FeatureFlags {
    fn has_flag<T: FeatureFlag>(&self) -> bool {
        if T::enabled_for_all() {
            return true;
        }

        if (cfg!(debug_assertions) || self.staff) && !*ZED_DISABLE_STAFF && T::enabled_for_staff() {
            return true;
        }

        self.flags.iter().any(|f| f.as_str() == T::NAME)
    }
}

impl Global for FeatureFlags {}

/// To create a feature flag, implement this trait on a trivial type and use it as
/// a generic parameter when called [`FeatureFlagAppExt::has_flag`].
///
/// Feature flags are enabled for members of Zed staff by default. To disable this behavior
/// so you can test flags being disabled, set ZED_DISABLE_STAFF=1 in your environment,
/// which will force Zed to treat the current user as non-staff.
pub trait FeatureFlag {
    const NAME: &'static str;

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
        self.observe_global_in::<FeatureFlags>(window, move |v, window, cx| {
            let feature_flags = cx.global::<FeatureFlags>();
            callback(feature_flags.has_flag::<T>(), v, window, cx);
        })
    }

    fn when_flag_enabled<T: FeatureFlag>(
        &mut self,
        window: &mut Window,
        callback: impl Fn(&mut V, &mut Window, &mut Context<V>) + Send + Sync + 'static,
    ) {
        if self
            .try_global::<FeatureFlags>()
            .is_some_and(|f| f.has_flag::<T>())
        {
            self.defer_in(window, move |view, window, cx| {
                callback(view, window, cx);
            });
            return;
        }
        let subscription = Rc::new(RefCell::new(None));
        let inner = self.observe_global_in::<FeatureFlags>(window, {
            let subscription = subscription.clone();
            move |v, window, cx| {
                let feature_flags = cx.global::<FeatureFlags>();
                if feature_flags.has_flag::<T>() {
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
        let feature_flags = self.default_global::<FeatureFlags>();
        feature_flags.staff = staff;
        feature_flags.flags = flags;
    }

    fn set_staff(&mut self, staff: bool) {
        let feature_flags = self.default_global::<FeatureFlags>();
        feature_flags.staff = staff;
    }

    fn has_flag<T: FeatureFlag>(&self) -> bool {
        self.try_global::<FeatureFlags>()
            .map(|flags| flags.has_flag::<T>())
            .unwrap_or_else(|| {
                (cfg!(debug_assertions) && T::enabled_for_staff() && !*ZED_DISABLE_STAFF)
                    || T::enabled_for_all()
            })
    }

    fn is_staff(&self) -> bool {
        self.try_global::<FeatureFlags>()
            .map(|flags| flags.staff)
            .unwrap_or(false)
    }

    fn on_flags_ready<F>(&mut self, mut callback: F) -> Subscription
    where
        F: FnMut(OnFlagsReady, &mut App) + 'static,
    {
        self.observe_global::<FeatureFlags>(move |cx| {
            let feature_flags = cx.global::<FeatureFlags>();
            callback(
                OnFlagsReady {
                    is_staff: feature_flags.staff,
                },
                cx,
            );
        })
    }

    fn observe_flag<T: FeatureFlag, F>(&mut self, mut callback: F) -> Subscription
    where
        F: FnMut(bool, &mut App) + 'static,
    {
        self.observe_global::<FeatureFlags>(move |cx| {
            let feature_flags = cx.global::<FeatureFlags>();
            callback(feature_flags.has_flag::<T>(), cx);
        })
    }
}
