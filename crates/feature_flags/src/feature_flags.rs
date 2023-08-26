use gpui::{AppContext, Subscription, ViewContext};

#[derive(Default)]
struct FeatureFlags {
    flags: Vec<String>,
    staff: bool,
}

impl FeatureFlags {
    fn has_flag(&self, flag: &str) -> bool {
        self.staff || self.flags.iter().find(|f| f.as_str() == flag).is_some()
    }
}

pub trait FeatureFlag {
    const NAME: &'static str;
}

pub enum ChannelsAlpha {}

impl FeatureFlag for ChannelsAlpha {
    const NAME: &'static str = "channels_alpha";
}

pub trait FeatureFlagViewExt<V: 'static> {
    fn observe_flag<T: FeatureFlag, F>(&mut self, callback: F) -> Subscription
    where
        F: Fn(bool, &mut V, &mut ViewContext<V>) + 'static;
}

impl<V: 'static> FeatureFlagViewExt<V> for ViewContext<'_, '_, V> {
    fn observe_flag<T: FeatureFlag, F>(&mut self, callback: F) -> Subscription
    where
        F: Fn(bool, &mut V, &mut ViewContext<V>) + 'static,
    {
        self.observe_global::<FeatureFlags, _>(move |v, cx| {
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
        self.update_default_global::<FeatureFlags, _, _>(|feature_flags, _| {
            feature_flags.staff = staff;
            feature_flags.flags = flags;
        })
    }

    fn set_staff(&mut self, staff: bool) {
        self.update_default_global::<FeatureFlags, _, _>(|feature_flags, _| {
            feature_flags.staff = staff;
        })
    }

    fn has_flag<T: FeatureFlag>(&self) -> bool {
        if self.has_global::<FeatureFlags>() {
            self.global::<FeatureFlags>().has_flag(T::NAME)
        } else {
            false
        }
    }

    fn is_staff(&self) -> bool {
        if self.has_global::<FeatureFlags>() {
            return self.global::<FeatureFlags>().staff;
        } else {
            false
        }
    }
}
