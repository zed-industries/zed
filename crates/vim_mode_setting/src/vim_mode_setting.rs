//! Contains the [`VimModeSetting`] and [`HelixModeSetting`] used to enable/disable Vim and Helix modes,
//! and the [`HelixNextFeatureFlag`] that picks the Helix mode implementation.
//!
//! This is in its own crate as we want other crates to be able to enable or
//! disable Vim/Helix modes without having to depend on the `vim` crate in its
//! entirety.

use std::{cell::RefCell, rc::Rc};

use feature_flags::{FeatureFlag, FeatureFlagAppExt as _, FeatureFlagStore, PresenceFlag};
use gpui::{App, Context, Subscription, Window};
use settings::{RegisterSetting, Settings, SettingsContent, SettingsStore};

/// Makes Helix mode use the `helix` crate instead of the Helix modes of the
/// `vim` crate.
pub struct HelixNextFeatureFlag;

impl FeatureFlag for HelixNextFeatureFlag {
    const NAME: &'static str = "helix-next";
    type Value = PresenceFlag;

    fn enabled_for_staff() -> bool {
        false
    }
}
feature_flags::register_feature_flag!(HelixNextFeatureFlag);

#[derive(RegisterSetting)]
pub struct VimModeSetting(pub bool);

impl Settings for VimModeSetting {
    fn from_settings(content: &SettingsContent) -> Self {
        Self(content.vim_mode.unwrap())
    }
}

impl VimModeSetting {
    pub fn is_enabled(cx: &App) -> bool {
        Self::try_get(cx)
            .map(|vim_mode| vim_mode.0)
            .unwrap_or(false)
    }
}

#[derive(RegisterSetting)]
pub struct HelixModeSetting(pub bool);

impl HelixModeSetting {
    pub fn is_enabled(cx: &App) -> bool {
        Self::try_get(cx)
            .map(|helix_mode| helix_mode.0)
            .unwrap_or(false)
    }

    /// Whether Helix mode is enabled and provided by the `helix` crate rather
    /// than by the `vim` crate.
    pub fn uses_helix_crate(cx: &App) -> bool {
        Self::is_enabled(cx) && cx.has_flag::<HelixNextFeatureFlag>()
    }
}

/// Calls `callback` whenever the settings or feature flags change, as together
/// they decide which modal editing implementation is active.
pub fn observe_modal_editing_changes(
    cx: &mut App,
    callback: impl FnMut(&mut App) + 'static,
) -> Subscription {
    let callback = Rc::new(RefCell::new(callback));
    Subscription::join(
        cx.observe_global::<SettingsStore>({
            let callback = callback.clone();
            move |cx| (callback.borrow_mut())(cx)
        }),
        cx.observe_global::<FeatureFlagStore>(move |cx| (callback.borrow_mut())(cx)),
    )
}

/// Like [`observe_modal_editing_changes`], for an entity in a window.
pub fn observe_modal_editing_changes_in<T: 'static>(
    window: &Window,
    cx: &mut Context<T>,
    callback: impl FnMut(&mut T, &mut Window, &mut Context<T>) + 'static,
) -> Subscription {
    let callback = Rc::new(RefCell::new(callback));
    Subscription::join(
        cx.observe_global_in::<SettingsStore>(window, {
            let callback = callback.clone();
            move |this, window, cx| (callback.borrow_mut())(this, window, cx)
        }),
        cx.observe_global_in::<FeatureFlagStore>(window, move |this, window, cx| {
            (callback.borrow_mut())(this, window, cx)
        }),
    )
}

impl Settings for HelixModeSetting {
    fn from_settings(content: &SettingsContent) -> Self {
        Self(content.helix_mode.unwrap())
    }
}
