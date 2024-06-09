use gpui::{AppContext, Global, Subscription, ViewContext};

#[derive(Default)]
struct FeatureFlags {
    flags: Vec<String>,
    staff: bool,
}

impl FeatureFlags {
    fn has_flag(&self, flag: &str) -> bool {
        self.staff || self.flags.iter().any(|f| f.as_str() == flag)
    }
}

impl Global for FeatureFlags {}

/// To create a feature flag, implement this trait on a trivial type and use it as
/// a generic parameter when called [`FeatureFlagAppExt::has_flag`].
///
/// Feature flags are always enabled for members of Zed staff. To disable this behavior
/// so you can test flags being disabled, set ZED_DISABLE_STAFF=1 in your environment,
/// which will force Zed to treat the current user as non-staff.
pub trait FeatureFlag {
    const NAME: &'static str;
}

pub struct Remoting {}
impl FeatureFlag for Remoting {
    const NAME: &'static str = "remoting";
}

pub trait FeatureFlagViewExt<V: 'static> {
    fn observe_flag<T: FeatureFlag, F>(&mut self, callback: F) -> Subscription
    where
        F: Fn(bool, &mut V, &mut ViewContext<V>) + Send + Sync + 'static;
}

impl<V> FeatureFlagViewExt<V> for ViewContext<'_, V>
where
    V: 'static,
{
    fn observe_flag<T: FeatureFlag, F>(&mut self, callback: F) -> Subscription
    where
        F: Fn(bool, &mut V, &mut ViewContext<V>) + 'static,
    {
        self.observe_global::<FeatureFlags>(move |v, cx| {
            let feature_flags = cx.global::<FeatureFlags>();
            callback(feature_flags.has_flag(<T as FeatureFlag>::NAME), v, cx);
        })
    }
}

pub trait FeatureFlagAppExt {
    fn update_flags(&mut self, staff: bool, flags: Vec<String>);
    fn set_staff(&mut self, staff: bool);
    fn has_flag<T: FeatureFlag>(&self) -> bool;
    fn is_staff(&self) -> bool;
}

impl FeatureFlagAppExt for AppContext {
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
            .map(|flags| flags.has_flag(T::NAME))
            .unwrap_or(false)
    }

    fn is_staff(&self) -> bool {
        self.try_global::<FeatureFlags>()
            .map(|flags| flags.staff)
            .unwrap_or(false)
    }
}
