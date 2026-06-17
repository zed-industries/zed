extern crate self as feature_flags;

use std::sync::LazyLock;

use collections::HashMap;
use gpui::{App, BorrowAppContext, Context, Global, Subscription, Window};

pub use feature_flags_macros::EnumFeatureFlag;

pub static ZED_DISABLE_STAFF: LazyLock<bool> = LazyLock::new(|| {
    std::env::var("ZED_DISABLE_STAFF").is_ok_and(|value| !value.is_empty() && value != "0")
});

pub trait FeatureFlagValue: Sized + Clone + Eq + Default + std::fmt::Debug + Send + Sync + 'static
{
    fn all_variants() -> &'static [Self];
    fn override_key(&self) -> &'static str;
    fn from_wire(wire: &str) -> Option<Self>;
    fn label(&self) -> &'static str {
        self.override_key()
    }
    fn on_variant() -> Self {
        Self::default()
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub enum PresenceFlag {
    On,
    #[default]
    Off,
}

impl std::ops::Deref for PresenceFlag {
    type Target = bool;
    fn deref(&self) -> &bool {
        match self {
            PresenceFlag::On => &true,
            PresenceFlag::Off => &false,
        }
    }
}

impl FeatureFlagValue for PresenceFlag {
    fn all_variants() -> &'static [Self] {
        &[PresenceFlag::On, PresenceFlag::Off]
    }
    fn override_key(&self) -> &'static str {
        match self {
            PresenceFlag::On => "on",
            PresenceFlag::Off => "off",
        }
    }
    fn label(&self) -> &'static str {
        match self {
            PresenceFlag::On => "On",
            PresenceFlag::Off => "Off",
        }
    }
    fn from_wire(_: &str) -> Option<Self> {
        Some(PresenceFlag::On)
    }
    fn on_variant() -> Self {
        PresenceFlag::On
    }
}

pub trait FeatureFlag {
    const NAME: &'static str;
    type Value: FeatureFlagValue;
    fn enabled_for_staff() -> bool {
        true
    }
    fn enabled_for_all() -> bool {
        false
    }
    fn watch<V: 'static>(cx: &mut Context<V>) {
        cx.observe_global::<FeatureFlagStore>(|_, cx| cx.notify()).detach();
    }
}

pub trait FeatureFlagViewExt<V: 'static> {
    fn observe_flag<T: FeatureFlag, F>(&mut self, window: &Window, callback: F) -> Subscription
    where
        F: Fn(T::Value, &mut V, &mut Window, &mut Context<V>) + Send + Sync + 'static;
}

impl<V> FeatureFlagViewExt<V> for Context<'_, V>
where
    V: 'static,
{
    fn observe_flag<T: FeatureFlag, F>(&mut self, window: &Window, callback: F) -> Subscription
    where
        F: Fn(T::Value, &mut V, &mut Window, &mut Context<V>) + 'static,
    {
        let value = T::Value::on_variant();
        self.defer_in(window, move |view, window, cx| {
            callback(value.clone(), view, window, cx);
        });
        self.observe_global_in::<FeatureFlagStore>(window, |_, _, _| {})
    }
}

pub struct OnFlagsReady {
    pub is_staff: bool,
}

pub trait FeatureFlagAppExt {
    fn update_flags(&mut self, staff: bool, flags: Vec<String>);
    fn set_staff(&mut self, staff: bool);
    fn has_flag<T: FeatureFlag>(&self) -> bool;
    fn flag_value<T: FeatureFlag>(&self) -> T::Value;
    fn is_staff(&self) -> bool;
    fn feature_flag_overrides_enabled(&self) -> bool;
    fn on_flags_ready<F>(&mut self, callback: F) -> Subscription
    where
        F: FnMut(OnFlagsReady, &mut App) + 'static;
    fn observe_flag<T: FeatureFlag, F>(&mut self, callback: F) -> Subscription
    where
        F: FnMut(T::Value, &mut App) + 'static;
}

impl FeatureFlagAppExt for App {
    fn update_flags(&mut self, _staff: bool, _flags: Vec<String>) {}
    fn set_staff(&mut self, _staff: bool) {}
    fn has_flag<T: FeatureFlag>(&self) -> bool {
        T::enabled_for_all() || (T::enabled_for_staff() && !*ZED_DISABLE_STAFF)
    }
    fn flag_value<T: FeatureFlag>(&self) -> T::Value {
        T::Value::on_variant()
    }
    fn is_staff(&self) -> bool {
        !*ZED_DISABLE_STAFF
    }
    fn feature_flag_overrides_enabled(&self) -> bool {
        !*ZED_DISABLE_STAFF
    }
    fn on_flags_ready<F>(&mut self, mut callback: F) -> Subscription
    where
        F: FnMut(OnFlagsReady, &mut App) + 'static,
    {
        let is_staff = !*ZED_DISABLE_STAFF;
        self.defer(move |cx| {
            callback(OnFlagsReady { is_staff }, cx);
        });
        self.observe_global::<FeatureFlagStore>(move |_| {})
    }
    fn observe_flag<T: FeatureFlag, F>(&mut self, mut callback: F) -> Subscription
    where
        F: FnMut(T::Value, &mut App) + 'static,
    {
        let value = T::Value::on_variant();
        self.defer(move |cx| {
            callback(value, cx);
        });
        self.observe_global::<FeatureFlagStore>(move |_| {})
    }
}

pub struct FeatureFlagDescriptor {
    pub name: &'static str,
    pub variants: fn() -> Vec<FeatureFlagVariant>,
    pub on_variant_key: fn() -> &'static str,
    pub default_variant_key: fn() -> &'static str,
    pub enabled_for_all: fn() -> bool,
    pub enabled_for_staff: fn() -> bool,
    pub type_id: fn() -> std::any::TypeId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FeatureFlagVariant {
    pub override_key: &'static str,
    pub label: &'static str,
}

#[macro_export]
macro_rules! register_feature_flag {
    ($flag:ty) => {};
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
        cx.update_default_global::<FeatureFlagStore, _>(|_, _| {});
    }

    pub fn known_flags() -> impl Iterator<Item = &'static FeatureFlagDescriptor> {
        [].iter()
    }

    pub fn is_staff(&self) -> bool {
        self.staff
    }

    pub fn overrides_enabled(&self) -> bool {
        !*ZED_DISABLE_STAFF
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

    pub fn override_for<'a>(_flag_name: &str, _cx: &'a App) -> Option<&'a str> {
        None
    }

    pub fn set_override(
        _flag_name: &str,
        _override_key: String,
        _fs: std::sync::Arc<dyn fs::Fs>,
        _cx: &App,
    ) {
    }

    pub fn clear_override(_flag_name: &str, _fs: std::sync::Arc<dyn fs::Fs>, _cx: &App) {}

    pub fn try_flag_value<T: FeatureFlag>(&self, _cx: &App) -> Option<T::Value> {
        if T::enabled_for_all() {
            return Some(T::Value::on_variant());
        }
        if !*ZED_DISABLE_STAFF && T::enabled_for_staff() {
            return Some(T::Value::on_variant());
        }
        None
    }

    pub fn has_flag<T: FeatureFlag>(&self, _cx: &App) -> bool {
        T::enabled_for_all() || (T::enabled_for_staff() && !*ZED_DISABLE_STAFF)
    }

    pub fn resolved_key(
        &self,
        descriptor: &FeatureFlagDescriptor,
        _cx: &App,
    ) -> &'static str {
        (descriptor.default_variant_key)()
    }

    pub fn is_forced_on(descriptor: &FeatureFlagDescriptor) -> bool {
        (descriptor.enabled_for_all)()
    }

    pub fn has_flag_default<T: FeatureFlag>() -> bool {
        T::enabled_for_all() || (T::enabled_for_staff() && !*ZED_DISABLE_STAFF)
    }
}

impl Global for FeatureFlagStore {}

#[derive(Clone, Default, Debug, settings::RegisterSetting)]
pub struct FeatureFlagsSettings {
    pub overrides: HashMap<String, String>,
}

impl settings::Settings for FeatureFlagsSettings {
    fn from_settings(_content: &settings::SettingsContent) -> Self {
        Self { overrides: HashMap::default() }
    }
}

// RegisterSetting is a derive macro, not a trait

pub fn generate_feature_flags_schema() -> schemars::Schema {
    schemars::schema_for!(serde_json::Value)
}

pub struct NotebookFeatureFlag;
impl FeatureFlag for NotebookFeatureFlag {
    const NAME: &'static str = "notebooks";
    type Value = PresenceFlag;
}
register_feature_flag!(NotebookFeatureFlag);

pub struct PanicFeatureFlag;
impl FeatureFlag for PanicFeatureFlag {
    const NAME: &'static str = "panic";
    type Value = PresenceFlag;
}
register_feature_flag!(PanicFeatureFlag);

pub struct AcpBetaFeatureFlag;
impl FeatureFlag for AcpBetaFeatureFlag {
    const NAME: &'static str = "acp-beta";
    type Value = PresenceFlag;
}
register_feature_flag!(AcpBetaFeatureFlag);

pub struct AgentSharingFeatureFlag;
impl FeatureFlag for AgentSharingFeatureFlag {
    const NAME: &'static str = "agent-sharing";
    type Value = PresenceFlag;
}
register_feature_flag!(AgentSharingFeatureFlag);

pub struct DiffReviewFeatureFlag;
impl FeatureFlag for DiffReviewFeatureFlag {
    const NAME: &'static str = "diff-review";
    type Value = PresenceFlag;
    fn enabled_for_staff() -> bool {
        false
    }
}
register_feature_flag!(DiffReviewFeatureFlag);

pub struct CreateThreadToolFeatureFlag;
impl FeatureFlag for CreateThreadToolFeatureFlag {
    const NAME: &'static str = "create-thread-tool";
    type Value = PresenceFlag;
    fn enabled_for_staff() -> bool {
        true
    }
}
register_feature_flag!(CreateThreadToolFeatureFlag);

pub struct LspToolFeatureFlag;
impl FeatureFlag for LspToolFeatureFlag {
    const NAME: &'static str = "lsp-tool";
    type Value = PresenceFlag;
    fn enabled_for_staff() -> bool {
        false
    }
}
register_feature_flag!(LspToolFeatureFlag);

pub struct RenameToolFeatureFlag;
impl FeatureFlag for RenameToolFeatureFlag {
    const NAME: &'static str = "rename-tool";
    type Value = PresenceFlag;
    fn enabled_for_staff() -> bool {
        true
    }
}
register_feature_flag!(RenameToolFeatureFlag);

pub struct ProjectPanelUndoRedoFeatureFlag;
impl FeatureFlag for ProjectPanelUndoRedoFeatureFlag {
    const NAME: &'static str = "project-panel-undo-redo";
    type Value = PresenceFlag;
    fn enabled_for_staff() -> bool {
        true
    }
}
register_feature_flag!(ProjectPanelUndoRedoFeatureFlag);

#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumFeatureFlag)]
pub enum AgentThreadWorktreeLabel {
    #[default]
    Both,
    Worktree,
    Branch,
}

pub struct AgentThreadWorktreeLabelFlag;
impl FeatureFlag for AgentThreadWorktreeLabelFlag {
    const NAME: &'static str = "agent-thread-worktree-label";
    type Value = AgentThreadWorktreeLabel;
    fn enabled_for_staff() -> bool {
        false
    }
}
register_feature_flag!(AgentThreadWorktreeLabelFlag);

pub struct AutoWatchFeatureFlag;
impl FeatureFlag for AutoWatchFeatureFlag {
    const NAME: &'static str = "auto-watch-screens";
    type Value = PresenceFlag;
}
register_feature_flag!(AutoWatchFeatureFlag);

pub struct SandboxingFeatureFlag;
impl FeatureFlag for SandboxingFeatureFlag {
    const NAME: &'static str = "sandboxing";
    type Value = PresenceFlag;
    fn enabled_for_staff() -> bool {
        false
    }
}
register_feature_flag!(SandboxingFeatureFlag);
