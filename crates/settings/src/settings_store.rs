use anyhow::{Context as _, Result};
use collections::{BTreeMap, HashMap, btree_map, hash_map};
use ec4rs::{ConfigParser, PropertiesSource, Section};
use fs::Fs;
use futures::{
    FutureExt, StreamExt,
    channel::{mpsc, oneshot},
    future::LocalBoxFuture,
};
use gpui::{App, AsyncApp, BorrowAppContext, Global, SharedString, Task, UpdateGlobal};

use paths::{EDITORCONFIG_NAME, local_settings_file_relative_path, task_file_name};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId, type_name},
    env,
    fmt::Debug,
    ops::Range,
    path::{Path, PathBuf},
    str::{self, FromStr},
    sync::Arc,
};
use util::{
    ResultExt as _, merge_non_null_json_value_into,
    schemars::{DefaultDenyUnknownFields, add_new_subschema},
};

pub type EditorconfigProperties = ec4rs::Properties;

use crate::{
    ActiveSettingsProfileName, ParameterizedJsonSchema, SettingsJsonSchemaParams, SettingsUiEntry,
    VsCodeSettings, WorktreeId, parse_json_with_comments, replace_value_in_json_text,
    settings_ui::SettingsUi, update_value_in_json_text,
};

/// A value that can be defined as a user setting.
///
/// Settings can be loaded from a combination of multiple JSON files.
pub trait Settings: SettingsUi + 'static + Send + Sync {
    /// The name of a key within the JSON file from which this setting should
    /// be deserialized. If this is `None`, then the setting will be deserialized
    /// from the root object.
    const KEY: Option<&'static str>;

    const FALLBACK_KEY: Option<&'static str> = None;

    /// The name of the keys in the [`FileContent`](Self::FileContent) that should
    /// always be written to a settings file, even if their value matches the default
    /// value.
    ///
    /// This is useful for tagged [`FileContent`](Self::FileContent)s where the tag
    /// is a "version" field that should always be persisted, even if the current
    /// user settings match the current version of the settings.
    const PRESERVED_KEYS: Option<&'static [&'static str]> = None;

    /// The type that is stored in an individual JSON file.
    type FileContent: Clone + Default + Serialize + DeserializeOwned + JsonSchema;

    /// The logic for combining together values from one or more JSON files into the
    /// final value for this setting.
    ///
    /// # Warning
    /// `Self::FileContent` deserialized field names should match with `Self` deserialized field names
    /// otherwise the field won't be deserialized properly and you will get the error:
    /// "A default setting must be added to the `default.json` file"
    fn load(sources: SettingsSources<Self::FileContent>, cx: &mut App) -> Result<Self>
    where
        Self: Sized;

    fn missing_default() -> anyhow::Error {
        anyhow::anyhow!("missing default")
    }

    /// Use [the helpers in the vscode_import module](crate::vscode_import) to apply known
    /// equivalent settings from a vscode config to our config
    fn import_from_vscode(vscode: &VsCodeSettings, current: &mut Self::FileContent);

    #[track_caller]
    fn register(cx: &mut App)
    where
        Self: Sized,
    {
        SettingsStore::update_global(cx, |store, cx| {
            store.register_setting::<Self>(cx);
        });
    }

    #[track_caller]
    fn get<'a>(path: Option<SettingsLocation>, cx: &'a App) -> &'a Self
    where
        Self: Sized,
    {
        cx.global::<SettingsStore>().get(path)
    }

    #[track_caller]
    fn get_global(cx: &App) -> &Self
    where
        Self: Sized,
    {
        cx.global::<SettingsStore>().get(None)
    }

    #[track_caller]
    fn try_get(cx: &App) -> Option<&Self>
    where
        Self: Sized,
    {
        if cx.has_global::<SettingsStore>() {
            cx.global::<SettingsStore>().try_get(None)
        } else {
            None
        }
    }

    #[track_caller]
    fn try_read_global<R>(cx: &AsyncApp, f: impl FnOnce(&Self) -> R) -> Option<R>
    where
        Self: Sized,
    {
        cx.try_read_global(|s: &SettingsStore, _| f(s.get(None)))
    }

    #[track_caller]
    fn override_global(settings: Self, cx: &mut App)
    where
        Self: Sized,
    {
        cx.global_mut::<SettingsStore>().override_global(settings)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SettingsSources<'a, T> {
    /// The default Zed settings.
    pub default: &'a T,
    /// Global settings (loaded before user settings).
    pub global: Option<&'a T>,
    /// Settings provided by extensions.
    pub extensions: Option<&'a T>,
    /// The user settings.
    pub user: Option<&'a T>,
    /// The user settings for the current release channel.
    pub release_channel: Option<&'a T>,
    /// The user settings for the current operating system.
    pub operating_system: Option<&'a T>,
    /// The settings associated with an enabled settings profile
    pub profile: Option<&'a T>,
    /// The server's settings.
    pub server: Option<&'a T>,
    /// The project settings, ordered from least specific to most specific.
    pub project: &'a [&'a T],
}

impl<'a, T: Serialize> SettingsSources<'a, T> {
    /// Returns an iterator over the default settings as well as all settings customizations.
    pub fn defaults_and_customizations(&self) -> impl Iterator<Item = &T> {
        [self.default].into_iter().chain(self.customizations())
    }

    /// Returns an iterator over all of the settings customizations.
    pub fn customizations(&self) -> impl Iterator<Item = &T> {
        self.global
            .into_iter()
            .chain(self.extensions)
            .chain(self.user)
            .chain(self.release_channel)
            .chain(self.operating_system)
            .chain(self.profile)
            .chain(self.server)
            .chain(self.project.iter().copied())
    }

    /// Returns the settings after performing a JSON merge of the provided customizations.
    ///
    /// Customizations later in the iterator win out over the earlier ones.
    pub fn json_merge_with<O: DeserializeOwned>(
        customizations: impl Iterator<Item = &'a T>,
    ) -> Result<O> {
        let mut merged = Value::Null;
        for value in customizations {
            merge_non_null_json_value_into(serde_json::to_value(value).unwrap(), &mut merged);
        }
        Ok(serde_json::from_value(merged)?)
    }

    /// Returns the settings after performing a JSON merge of the customizations into the
    /// default settings.
    ///
    /// More-specific customizations win out over the less-specific ones.
    pub fn json_merge<O: DeserializeOwned>(&'a self) -> Result<O> {
        Self::json_merge_with(self.defaults_and_customizations())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SettingsLocation<'a> {
    pub worktree_id: WorktreeId,
    pub path: &'a Path,
}

/// A set of strongly-typed setting values defined via multiple config files.
pub struct SettingsStore {
    setting_values: HashMap<TypeId, Box<dyn AnySettingValue>>,
    raw_default_settings: Value,
    raw_global_settings: Option<Value>,
    raw_user_settings: Value,
    raw_server_settings: Option<Value>,
    raw_extension_settings: Value,
    raw_local_settings: BTreeMap<(WorktreeId, Arc<Path>), Value>,
    raw_editorconfig_settings: BTreeMap<(WorktreeId, Arc<Path>), (String, Option<Editorconfig>)>,
    tab_size_callback: Option<(
        TypeId,
        Box<dyn Fn(&dyn Any) -> Option<usize> + Send + Sync + 'static>,
    )>,
    _setting_file_updates: Task<()>,
    setting_file_updates_tx:
        mpsc::UnboundedSender<Box<dyn FnOnce(AsyncApp) -> LocalBoxFuture<'static, Result<()>>>>,
}

#[derive(Clone)]
pub struct Editorconfig {
    pub is_root: bool,
    pub sections: SmallVec<[Section; 5]>,
}

impl FromStr for Editorconfig {
    type Err = anyhow::Error;

    fn from_str(contents: &str) -> Result<Self, Self::Err> {
        let parser = ConfigParser::new_buffered(contents.as_bytes())
            .context("creating editorconfig parser")?;
        let is_root = parser.is_root;
        let sections = parser
            .collect::<Result<SmallVec<_>, _>>()
            .context("parsing editorconfig sections")?;
        Ok(Self { is_root, sections })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LocalSettingsKind {
    Settings,
    Tasks,
    Editorconfig,
    Debug,
}

impl Global for SettingsStore {}

#[derive(Debug)]
struct SettingValue<T> {
    global_value: Option<T>,
    local_values: Vec<(WorktreeId, Arc<Path>, T)>,
}

trait AnySettingValue: 'static + Send + Sync {
    fn key(&self) -> Option<&'static str>;
    fn setting_type_name(&self) -> &'static str;
    fn deserialize_setting(&self, json: &Value) -> Result<DeserializedSetting> {
        self.deserialize_setting_with_key(json).1
    }
    fn deserialize_setting_with_key(
        &self,
        json: &Value,
    ) -> (Option<&'static str>, Result<DeserializedSetting>);
    fn load_setting(
        &self,
        sources: SettingsSources<DeserializedSetting>,
        cx: &mut App,
    ) -> Result<Box<dyn Any>>;
    fn value_for_path(&self, path: Option<SettingsLocation>) -> &dyn Any;
    fn all_local_values(&self) -> Vec<(WorktreeId, Arc<Path>, &dyn Any)>;
    fn set_global_value(&mut self, value: Box<dyn Any>);
    fn set_local_value(&mut self, root_id: WorktreeId, path: Arc<Path>, value: Box<dyn Any>);
    fn json_schema(&self, generator: &mut schemars::SchemaGenerator) -> schemars::Schema;
    fn edits_for_update(
        &self,
        raw_settings: &serde_json::Value,
        tab_size: usize,
        vscode_settings: &VsCodeSettings,
        text: &mut String,
        edits: &mut Vec<(Range<usize>, String)>,
    );
    fn settings_ui_item(&self) -> SettingsUiEntry;
}

struct DeserializedSetting(Box<dyn Any>);

impl SettingsStore {
    pub fn new(cx: &App) -> Self {
        let (setting_file_updates_tx, mut setting_file_updates_rx) = mpsc::unbounded();
        Self {
            setting_values: Default::default(),
            raw_default_settings: json!({}),
            raw_global_settings: None,
            raw_user_settings: json!({}),
            raw_server_settings: None,
            raw_extension_settings: json!({}),
            raw_local_settings: Default::default(),
            raw_editorconfig_settings: BTreeMap::default(),
            tab_size_callback: Default::default(),
            setting_file_updates_tx,
            _setting_file_updates: cx.spawn(async move |cx| {
                while let Some(setting_file_update) = setting_file_updates_rx.next().await {
                    (setting_file_update)(cx.clone()).await.log_err();
                }
            }),
        }
    }

    pub fn observe_active_settings_profile_name(cx: &mut App) -> gpui::Subscription {
        cx.observe_global::<ActiveSettingsProfileName>(|cx| {
            Self::update_global(cx, |store, cx| {
                store.recompute_values(None, cx).log_err();
            });
        })
    }

    pub fn update<C, R>(cx: &mut C, f: impl FnOnce(&mut Self, &mut C) -> R) -> R
    where
        C: BorrowAppContext,
    {
        cx.update_global(f)
    }

    /// Add a new type of setting to the store.
    pub fn register_setting<T: Settings>(&mut self, cx: &mut App) {
        let setting_type_id = TypeId::of::<T>();
        let entry = self.setting_values.entry(setting_type_id);

        if matches!(entry, hash_map::Entry::Occupied(_)) {
            return;
        }

        let setting_value = entry.or_insert(Box::new(SettingValue::<T> {
            global_value: None,
            local_values: Vec::new(),
        }));

        if let Some(default_settings) = setting_value
            .deserialize_setting(&self.raw_default_settings)
            .log_err()
        {
            let user_value = setting_value
                .deserialize_setting(&self.raw_user_settings)
                .log_err();

            let mut release_channel_value = None;
            if let Some(release_settings) = &self
                .raw_user_settings
                .get(release_channel::RELEASE_CHANNEL.dev_name())
            {
                release_channel_value = setting_value
                    .deserialize_setting(release_settings)
                    .log_err();
            }

            let mut os_settings_value = None;
            if let Some(os_settings) = &self.raw_user_settings.get(env::consts::OS) {
                os_settings_value = setting_value.deserialize_setting(os_settings).log_err();
            }

            let mut profile_value = None;
            if let Some(active_profile) = cx.try_global::<ActiveSettingsProfileName>()
                && let Some(profiles) = self.raw_user_settings.get("profiles")
                && let Some(profile_settings) = profiles.get(&active_profile.0)
            {
                profile_value = setting_value
                    .deserialize_setting(profile_settings)
                    .log_err();
            }

            let server_value = self
                .raw_server_settings
                .as_ref()
                .and_then(|server_setting| {
                    setting_value.deserialize_setting(server_setting).log_err()
                });

            let extension_value = setting_value
                .deserialize_setting(&self.raw_extension_settings)
                .log_err();

            if let Some(setting) = setting_value
                .load_setting(
                    SettingsSources {
                        default: &default_settings,
                        global: None,
                        extensions: extension_value.as_ref(),
                        user: user_value.as_ref(),
                        release_channel: release_channel_value.as_ref(),
                        operating_system: os_settings_value.as_ref(),
                        profile: profile_value.as_ref(),
                        server: server_value.as_ref(),
                        project: &[],
                    },
                    cx,
                )
                .context("A default setting must be added to the `default.json` file")
                .log_err()
            {
                setting_value.set_global_value(setting);
            }
        }
    }

    /// Get the value of a setting.
    ///
    /// Panics if the given setting type has not been registered, or if there is no
    /// value for this setting.
    pub fn get<T: Settings>(&self, path: Option<SettingsLocation>) -> &T {
        self.setting_values
            .get(&TypeId::of::<T>())
            .unwrap_or_else(|| panic!("unregistered setting type {}", type_name::<T>()))
            .value_for_path(path)
            .downcast_ref::<T>()
            .expect("no default value for setting type")
    }

    /// Get the value of a setting.
    ///
    /// Does not panic
    pub fn try_get<T: Settings>(&self, path: Option<SettingsLocation>) -> Option<&T> {
        self.setting_values
            .get(&TypeId::of::<T>())
            .map(|value| value.value_for_path(path))
            .and_then(|value| value.downcast_ref::<T>())
    }

    /// Get all values from project specific settings
    pub fn get_all_locals<T: Settings>(&self) -> Vec<(WorktreeId, Arc<Path>, &T)> {
        self.setting_values
            .get(&TypeId::of::<T>())
            .unwrap_or_else(|| panic!("unregistered setting type {}", type_name::<T>()))
            .all_local_values()
            .into_iter()
            .map(|(id, path, any)| {
                (
                    id,
                    path,
                    any.downcast_ref::<T>()
                        .expect("wrong value type for setting"),
                )
            })
            .collect()
    }

    /// Override the global value for a setting.
    ///
    /// The given value will be overwritten if the user settings file changes.
    pub fn override_global<T: Settings>(&mut self, value: T) {
        self.setting_values
            .get_mut(&TypeId::of::<T>())
            .unwrap_or_else(|| panic!("unregistered setting type {}", type_name::<T>()))
            .set_global_value(Box::new(value))
    }

    /// Get the user's settings as a raw JSON value.
    ///
    /// For user-facing functionality use the typed setting interface.
    /// (e.g. ProjectSettings::get_global(cx))
    pub fn raw_user_settings(&self) -> &Value {
        &self.raw_user_settings
    }

    /// Get the configured settings profile names.
    pub fn configured_settings_profiles(&self) -> impl Iterator<Item = &str> {
        self.raw_user_settings
            .get("profiles")
            .and_then(|v| v.as_object())
            .into_iter()
            .flat_map(|obj| obj.keys())
            .map(|s| s.as_str())
    }

    /// Access the raw JSON value of the global settings.
    pub fn raw_global_settings(&self) -> Option<&Value> {
        self.raw_global_settings.as_ref()
    }

    /// Access the raw JSON value of the default settings.
    pub fn raw_default_settings(&self) -> &Value {
        &self.raw_default_settings
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test(cx: &mut App) -> Self {
        let mut this = Self::new(cx);
        this.set_default_settings(&crate::test_settings(), cx)
            .unwrap();
        this.set_user_settings("{}", cx).unwrap();
        this
    }

    /// Updates the value of a setting in the user's global configuration.
    ///
    /// This is only for tests. Normally, settings are only loaded from
    /// JSON files.
    #[cfg(any(test, feature = "test-support"))]
    pub fn update_user_settings<T: Settings>(
        &mut self,
        cx: &mut App,
        update: impl FnOnce(&mut T::FileContent),
    ) {
        let old_text = serde_json::to_string(&self.raw_user_settings).unwrap();
        let new_text = self.new_text_for_update::<T>(old_text, update);
        self.set_user_settings(&new_text, cx).unwrap();
    }

    pub async fn load_settings(fs: &Arc<dyn Fs>) -> Result<String> {
        match fs.load(paths::settings_file()).await {
            result @ Ok(_) => result,
            Err(err) => {
                if let Some(e) = err.downcast_ref::<std::io::Error>()
                    && e.kind() == std::io::ErrorKind::NotFound
                {
                    return Ok(crate::initial_user_settings_content().to_string());
                }
                Err(err)
            }
        }
    }

    pub async fn load_global_settings(fs: &Arc<dyn Fs>) -> Result<String> {
        match fs.load(paths::global_settings_file()).await {
            result @ Ok(_) => result,
            Err(err) => {
                if let Some(e) = err.downcast_ref::<std::io::Error>()
                    && e.kind() == std::io::ErrorKind::NotFound
                {
                    return Ok("{}".to_string());
                }
                Err(err)
            }
        }
    }

    fn update_settings_file_inner(
        &self,
        fs: Arc<dyn Fs>,
        update: impl 'static + Send + FnOnce(String, AsyncApp) -> Result<String>,
    ) -> oneshot::Receiver<Result<()>> {
        let (tx, rx) = oneshot::channel::<Result<()>>();
        self.setting_file_updates_tx
            .unbounded_send(Box::new(move |cx: AsyncApp| {
                async move {
                    let res = async move {
                        let old_text = Self::load_settings(&fs).await?;
                        let new_text = update(old_text, cx)?;
                        let settings_path = paths::settings_file().as_path();
                        if fs.is_file(settings_path).await {
                            let resolved_path =
                                fs.canonicalize(settings_path).await.with_context(|| {
                                    format!(
                                        "Failed to canonicalize settings path {:?}",
                                        settings_path
                                    )
                                })?;

                            fs.atomic_write(resolved_path.clone(), new_text)
                                .await
                                .with_context(|| {
                                    format!("Failed to write settings to file {:?}", resolved_path)
                                })?;
                        } else {
                            fs.atomic_write(settings_path.to_path_buf(), new_text)
                                .await
                                .with_context(|| {
                                    format!("Failed to write settings to file {:?}", settings_path)
                                })?;
                        }
                        anyhow::Ok(())
                    }
                    .await;

                    let new_res = match &res {
                        Ok(_) => anyhow::Ok(()),
                        Err(e) => Err(anyhow::anyhow!("Failed to write settings to file {:?}", e)),
                    };

                    _ = tx.send(new_res);
                    res
                }
                .boxed_local()
            }))
            .map_err(|err| anyhow::format_err!("Failed to update settings file: {}", err))
            .log_with_level(log::Level::Warn);
        return rx;
    }

    pub fn update_settings_file_at_path(
        &self,
        fs: Arc<dyn Fs>,
        path: &[&str],
        new_value: serde_json::Value,
    ) -> oneshot::Receiver<Result<()>> {
        let key_path = path
            .into_iter()
            .cloned()
            .map(SharedString::new)
            .collect::<Vec<_>>();
        let update = move |mut old_text: String, cx: AsyncApp| {
            cx.read_global(|store: &SettingsStore, _cx| {
                // todo(settings_ui) use `update_value_in_json_text` for merging new and old objects with comment preservation, needs old value though...
                let (range, replacement) = replace_value_in_json_text(
                    &old_text,
                    key_path.as_slice(),
                    store.json_tab_size(),
                    Some(&new_value),
                    None,
                );
                old_text.replace_range(range, &replacement);
                old_text
            })
        };
        self.update_settings_file_inner(fs, update)
    }

    pub fn update_settings_file<T: Settings>(
        &self,
        fs: Arc<dyn Fs>,
        update: impl 'static + Send + FnOnce(&mut T::FileContent, &App),
    ) {
        _ = self.update_settings_file_inner(fs, move |old_text: String, cx: AsyncApp| {
            cx.read_global(|store: &SettingsStore, cx| {
                store.new_text_for_update::<T>(old_text, |content| update(content, cx))
            })
        });
    }

    pub fn import_vscode_settings(
        &self,
        fs: Arc<dyn Fs>,
        vscode_settings: VsCodeSettings,
    ) -> oneshot::Receiver<Result<()>> {
        self.update_settings_file_inner(fs, move |old_text: String, cx: AsyncApp| {
            cx.read_global(|store: &SettingsStore, _cx| {
                store.get_vscode_edits(old_text, &vscode_settings)
            })
        })
    }

    pub fn settings_ui_items(&self) -> impl IntoIterator<Item = SettingsUiEntry> {
        self.setting_values
            .values()
            .map(|item| item.settings_ui_item())
    }
}

impl SettingsStore {
    /// Updates the value of a setting in a JSON file, returning the new text
    /// for that JSON file.
    pub fn new_text_for_update<T: Settings>(
        &self,
        old_text: String,
        update: impl FnOnce(&mut T::FileContent),
    ) -> String {
        let edits = self.edits_for_update::<T>(&old_text, update);
        let mut new_text = old_text;
        for (range, replacement) in edits.into_iter() {
            new_text.replace_range(range, &replacement);
        }
        new_text
    }

    pub fn get_vscode_edits(&self, mut old_text: String, vscode: &VsCodeSettings) -> String {
        let mut new_text = old_text.clone();
        let mut edits: Vec<(Range<usize>, String)> = Vec::new();
        let raw_settings = parse_json_with_comments::<Value>(&old_text).unwrap_or_default();
        let tab_size = self.json_tab_size();
        for v in self.setting_values.values() {
            v.edits_for_update(&raw_settings, tab_size, vscode, &mut old_text, &mut edits);
        }
        for (range, replacement) in edits.into_iter() {
            new_text.replace_range(range, &replacement);
        }
        new_text
    }

    /// Updates the value of a setting in a JSON file, returning a list
    /// of edits to apply to the JSON file.
    pub fn edits_for_update<T: Settings>(
        &self,
        text: &str,
        update: impl FnOnce(&mut T::FileContent),
    ) -> Vec<(Range<usize>, String)> {
        let setting_type_id = TypeId::of::<T>();

        let preserved_keys = T::PRESERVED_KEYS.unwrap_or_default();

        let setting = self
            .setting_values
            .get(&setting_type_id)
            .unwrap_or_else(|| panic!("unregistered setting type {}", type_name::<T>()));
        let raw_settings = parse_json_with_comments::<Value>(text).unwrap_or_default();
        let (key, deserialized_setting) = setting.deserialize_setting_with_key(&raw_settings);
        let old_content = match deserialized_setting {
            Ok(content) => content.0.downcast::<T::FileContent>().unwrap(),
            Err(_) => Box::<<T as Settings>::FileContent>::default(),
        };
        let mut new_content = old_content.clone();
        update(&mut new_content);

        let old_value = serde_json::to_value(&old_content).unwrap();
        let new_value = serde_json::to_value(new_content).unwrap();

        let mut key_path = Vec::new();
        if let Some(key) = key {
            key_path.push(key);
        }

        let mut edits = Vec::new();
        let tab_size = self.json_tab_size();
        let mut text = text.to_string();
        update_value_in_json_text(
            &mut text,
            &mut key_path,
            tab_size,
            &old_value,
            &new_value,
            preserved_keys,
            &mut edits,
        );
        edits
    }

    /// Configure the tab sized when updating JSON files.
    pub fn set_json_tab_size_callback<T: Settings>(
        &mut self,
        get_tab_size: fn(&T) -> Option<usize>,
    ) {
        self.tab_size_callback = Some((
            TypeId::of::<T>(),
            Box::new(move |value| get_tab_size(value.downcast_ref::<T>().unwrap())),
        ));
    }

    pub fn json_tab_size(&self) -> usize {
        const DEFAULT_JSON_TAB_SIZE: usize = 2;

        if let Some((setting_type_id, callback)) = &self.tab_size_callback {
            let setting_value = self.setting_values.get(setting_type_id).unwrap();
            let value = setting_value.value_for_path(None);
            if let Some(value) = callback(value) {
                return value;
            }
        }

        DEFAULT_JSON_TAB_SIZE
    }

    /// Sets the default settings via a JSON string.
    ///
    /// The string should contain a JSON object with a default value for every setting.
    pub fn set_default_settings(
        &mut self,
        default_settings_content: &str,
        cx: &mut App,
    ) -> Result<()> {
        let settings: Value = parse_json_with_comments(default_settings_content)?;
        anyhow::ensure!(settings.is_object(), "settings must be an object");
        self.raw_default_settings = settings;
        self.recompute_values(None, cx)?;
        Ok(())
    }

    /// Sets the user settings via a JSON string.
    pub fn set_user_settings(
        &mut self,
        user_settings_content: &str,
        cx: &mut App,
    ) -> Result<Value> {
        let settings: Value = if user_settings_content.is_empty() {
            parse_json_with_comments("{}")?
        } else {
            parse_json_with_comments(user_settings_content)?
        };

        anyhow::ensure!(settings.is_object(), "settings must be an object");
        self.raw_user_settings = settings.clone();
        self.recompute_values(None, cx)?;
        Ok(settings)
    }

    /// Sets the global settings via a JSON string.
    pub fn set_global_settings(
        &mut self,
        global_settings_content: &str,
        cx: &mut App,
    ) -> Result<Value> {
        let settings: Value = if global_settings_content.is_empty() {
            parse_json_with_comments("{}")?
        } else {
            parse_json_with_comments(global_settings_content)?
        };

        anyhow::ensure!(settings.is_object(), "settings must be an object");
        self.raw_global_settings = Some(settings.clone());
        self.recompute_values(None, cx)?;
        Ok(settings)
    }

    pub fn set_server_settings(
        &mut self,
        server_settings_content: &str,
        cx: &mut App,
    ) -> Result<()> {
        let settings: Option<Value> = if server_settings_content.is_empty() {
            None
        } else {
            parse_json_with_comments(server_settings_content)?
        };

        anyhow::ensure!(
            settings
                .as_ref()
                .map(|value| value.is_object())
                .unwrap_or(true),
            "settings must be an object"
        );
        self.raw_server_settings = settings;
        self.recompute_values(None, cx)?;
        Ok(())
    }

    /// Add or remove a set of local settings via a JSON string.
    pub fn set_local_settings(
        &mut self,
        root_id: WorktreeId,
        directory_path: Arc<Path>,
        kind: LocalSettingsKind,
        settings_content: Option<&str>,
        cx: &mut App,
    ) -> std::result::Result<(), InvalidSettingsError> {
        let mut zed_settings_changed = false;
        match (
            kind,
            settings_content
                .map(|content| content.trim())
                .filter(|content| !content.is_empty()),
        ) {
            (LocalSettingsKind::Tasks, _) => {
                return Err(InvalidSettingsError::Tasks {
                    message: "Attempted to submit tasks into the settings store".to_string(),
                    path: directory_path.join(task_file_name()),
                });
            }
            (LocalSettingsKind::Debug, _) => {
                return Err(InvalidSettingsError::Debug {
                    message: "Attempted to submit debugger config into the settings store"
                        .to_string(),
                    path: directory_path.join(task_file_name()),
                });
            }
            (LocalSettingsKind::Settings, None) => {
                zed_settings_changed = self
                    .raw_local_settings
                    .remove(&(root_id, directory_path.clone()))
                    .is_some()
            }
            (LocalSettingsKind::Editorconfig, None) => {
                self.raw_editorconfig_settings
                    .remove(&(root_id, directory_path.clone()));
            }
            (LocalSettingsKind::Settings, Some(settings_contents)) => {
                let new_settings =
                    parse_json_with_comments::<Value>(settings_contents).map_err(|e| {
                        InvalidSettingsError::LocalSettings {
                            path: directory_path.join(local_settings_file_relative_path()),
                            message: e.to_string(),
                        }
                    })?;
                match self
                    .raw_local_settings
                    .entry((root_id, directory_path.clone()))
                {
                    btree_map::Entry::Vacant(v) => {
                        v.insert(new_settings);
                        zed_settings_changed = true;
                    }
                    btree_map::Entry::Occupied(mut o) => {
                        if o.get() != &new_settings {
                            o.insert(new_settings);
                            zed_settings_changed = true;
                        }
                    }
                }
            }
            (LocalSettingsKind::Editorconfig, Some(editorconfig_contents)) => {
                match self
                    .raw_editorconfig_settings
                    .entry((root_id, directory_path.clone()))
                {
                    btree_map::Entry::Vacant(v) => match editorconfig_contents.parse() {
                        Ok(new_contents) => {
                            v.insert((editorconfig_contents.to_owned(), Some(new_contents)));
                        }
                        Err(e) => {
                            v.insert((editorconfig_contents.to_owned(), None));
                            return Err(InvalidSettingsError::Editorconfig {
                                message: e.to_string(),
                                path: directory_path.join(EDITORCONFIG_NAME),
                            });
                        }
                    },
                    btree_map::Entry::Occupied(mut o) => {
                        if o.get().0 != editorconfig_contents {
                            match editorconfig_contents.parse() {
                                Ok(new_contents) => {
                                    o.insert((
                                        editorconfig_contents.to_owned(),
                                        Some(new_contents),
                                    ));
                                }
                                Err(e) => {
                                    o.insert((editorconfig_contents.to_owned(), None));
                                    return Err(InvalidSettingsError::Editorconfig {
                                        message: e.to_string(),
                                        path: directory_path.join(EDITORCONFIG_NAME),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        };

        if zed_settings_changed {
            self.recompute_values(Some((root_id, &directory_path)), cx)?;
        }
        Ok(())
    }

    pub fn set_extension_settings<T: Serialize>(&mut self, content: T, cx: &mut App) -> Result<()> {
        let settings: Value = serde_json::to_value(content)?;
        anyhow::ensure!(settings.is_object(), "settings must be an object");
        self.raw_extension_settings = settings;
        self.recompute_values(None, cx)?;
        Ok(())
    }

    /// Add or remove a set of local settings via a JSON string.
    pub fn clear_local_settings(&mut self, root_id: WorktreeId, cx: &mut App) -> Result<()> {
        self.raw_local_settings
            .retain(|(worktree_id, _), _| worktree_id != &root_id);
        self.recompute_values(Some((root_id, "".as_ref())), cx)?;
        Ok(())
    }

    pub fn local_settings(
        &self,
        root_id: WorktreeId,
    ) -> impl '_ + Iterator<Item = (Arc<Path>, String)> {
        self.raw_local_settings
            .range(
                (root_id, Path::new("").into())
                    ..(
                        WorktreeId::from_usize(root_id.to_usize() + 1),
                        Path::new("").into(),
                    ),
            )
            .map(|((_, path), content)| (path.clone(), serde_json::to_string(content).unwrap()))
    }

    pub fn local_editorconfig_settings(
        &self,
        root_id: WorktreeId,
    ) -> impl '_ + Iterator<Item = (Arc<Path>, String, Option<Editorconfig>)> {
        self.raw_editorconfig_settings
            .range(
                (root_id, Path::new("").into())
                    ..(
                        WorktreeId::from_usize(root_id.to_usize() + 1),
                        Path::new("").into(),
                    ),
            )
            .map(|((_, path), (content, parsed_content))| {
                (path.clone(), content.clone(), parsed_content.clone())
            })
    }

    pub fn json_schema(&self, schema_params: &SettingsJsonSchemaParams, cx: &App) -> Value {
        let mut generator = schemars::generate::SchemaSettings::draft2019_09()
            .with_transform(DefaultDenyUnknownFields)
            .into_generator();
        let mut combined_schema = json!({
            "type": "object",
            "properties": {}
        });

        // Merge together settings schemas, similarly to json schema's "allOf". This merging is
        // recursive, though at time of writing this recursive nature isn't used very much. An
        // example of it is the schema for `jupyter` having contribution from both `EditorSettings`
        // and `JupyterSettings`.
        //
        // This logic could be removed in favor of "allOf", but then there isn't the opportunity to
        // validate and fully control the merge.
        for setting_value in self.setting_values.values() {
            let mut setting_schema = setting_value.json_schema(&mut generator);

            if let Some(key) = setting_value.key() {
                if let Some(properties) = combined_schema.get_mut("properties")
                    && let Some(properties_obj) = properties.as_object_mut()
                {
                    if let Some(target) = properties_obj.get_mut(key) {
                        merge_schema(target, setting_schema.to_value());
                    } else {
                        properties_obj.insert(key.to_string(), setting_schema.to_value());
                    }
                }
            } else {
                setting_schema.remove("description");
                setting_schema.remove("additionalProperties");
                merge_schema(&mut combined_schema, setting_schema.to_value());
            }
        }

        fn merge_schema(target: &mut serde_json::Value, source: serde_json::Value) {
            let (Some(target_obj), serde_json::Value::Object(source_obj)) =
                (target.as_object_mut(), source)
            else {
                return;
            };

            for (source_key, source_value) in source_obj {
                match source_key.as_str() {
                    "properties" => {
                        let serde_json::Value::Object(source_properties) = source_value else {
                            log::error!(
                                "bug: expected object for `{}` json schema field, but got: {}",
                                source_key,
                                source_value
                            );
                            continue;
                        };
                        let target_properties =
                            target_obj.entry(source_key.clone()).or_insert(json!({}));
                        let Some(target_properties) = target_properties.as_object_mut() else {
                            log::error!(
                                "bug: expected object for `{}` json schema field, but got: {}",
                                source_key,
                                target_properties
                            );
                            continue;
                        };
                        for (key, value) in source_properties {
                            if let Some(existing) = target_properties.get_mut(&key) {
                                merge_schema(existing, value);
                            } else {
                                target_properties.insert(key, value);
                            }
                        }
                    }
                    "allOf" | "anyOf" | "oneOf" => {
                        let serde_json::Value::Array(source_array) = source_value else {
                            log::error!(
                                "bug: expected array for `{}` json schema field, but got: {}",
                                source_key,
                                source_value,
                            );
                            continue;
                        };
                        let target_array =
                            target_obj.entry(source_key.clone()).or_insert(json!([]));
                        let Some(target_array) = target_array.as_array_mut() else {
                            log::error!(
                                "bug: expected array for `{}` json schema field, but got: {}",
                                source_key,
                                target_array,
                            );
                            continue;
                        };
                        target_array.extend(source_array);
                    }
                    "type"
                    | "$ref"
                    | "enum"
                    | "minimum"
                    | "maximum"
                    | "pattern"
                    | "description"
                    | "additionalProperties" => {
                        if let Some(old_value) =
                            target_obj.insert(source_key.clone(), source_value.clone())
                            && old_value != source_value
                        {
                            log::error!(
                                "bug: while merging JSON schemas, \
                                    mismatch `\"{}\": {}` (before was `{}`)",
                                source_key,
                                old_value,
                                source_value
                            );
                        }
                    }
                    _ => {
                        log::error!(
                            "bug: while merging settings JSON schemas, \
                            encountered unexpected `\"{}\": {}`",
                            source_key,
                            source_value
                        );
                    }
                }
            }
        }

        // add schemas which are determined at runtime
        for parameterized_json_schema in inventory::iter::<ParameterizedJsonSchema>() {
            (parameterized_json_schema.add_and_get_ref)(&mut generator, schema_params, cx);
        }

        // add merged settings schema to the definitions
        const ZED_SETTINGS: &str = "ZedSettings";
        let zed_settings_ref = add_new_subschema(&mut generator, ZED_SETTINGS, combined_schema);

        // add `ZedSettingsOverride` which is the same as `ZedSettings` except that unknown
        // fields are rejected. This is used for release stage settings and profiles.
        let mut zed_settings_override = zed_settings_ref.clone();
        zed_settings_override.insert("unevaluatedProperties".to_string(), false.into());
        let zed_settings_override_ref = add_new_subschema(
            &mut generator,
            "ZedSettingsOverride",
            zed_settings_override.to_value(),
        );

        // Remove `"additionalProperties": false` added by `DefaultDenyUnknownFields` so that
        // unknown fields can be handled by the root schema and `ZedSettingsOverride`.
        let mut definitions = generator.take_definitions(true);
        definitions
            .get_mut(ZED_SETTINGS)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .remove("additionalProperties");

        let meta_schema = generator
            .settings()
            .meta_schema
            .as_ref()
            .expect("meta_schema should be present in schemars settings")
            .to_string();

        json!({
            "$schema": meta_schema,
            "title": "Zed Settings",
            "unevaluatedProperties": false,
            // ZedSettings + settings overrides for each release stage / OS / profiles
            "allOf": [
                zed_settings_ref,
                {
                    "properties": {
                        "dev": zed_settings_override_ref,
                        "nightly": zed_settings_override_ref,
                        "stable": zed_settings_override_ref,
                        "preview": zed_settings_override_ref,
                        "linux": zed_settings_override_ref,
                        "macos": zed_settings_override_ref,
                        "windows": zed_settings_override_ref,
                        "profiles": {
                            "type": "object",
                            "description": "Configures any number of settings profiles.",
                            "additionalProperties": zed_settings_override_ref
                        }
                    }
                }
            ],
            "$defs": definitions,
        })
    }

    fn recompute_values(
        &mut self,
        changed_local_path: Option<(WorktreeId, &Path)>,
        cx: &mut App,
    ) -> std::result::Result<(), InvalidSettingsError> {
        // Reload the global and local values for every setting.
        let mut project_settings_stack = Vec::<DeserializedSetting>::new();
        let mut paths_stack = Vec::<Option<(WorktreeId, &Path)>>::new();
        for setting_value in self.setting_values.values_mut() {
            let default_settings = setting_value
                .deserialize_setting(&self.raw_default_settings)
                .map_err(|e| InvalidSettingsError::DefaultSettings {
                    message: e.to_string(),
                })?;

            let global_settings = self
                .raw_global_settings
                .as_ref()
                .and_then(|setting| setting_value.deserialize_setting(setting).log_err());

            let extension_settings = setting_value
                .deserialize_setting(&self.raw_extension_settings)
                .log_err();

            let user_settings = match setting_value.deserialize_setting(&self.raw_user_settings) {
                Ok(settings) => Some(settings),
                Err(error) => {
                    return Err(InvalidSettingsError::UserSettings {
                        message: error.to_string(),
                    });
                }
            };

            let server_settings = self
                .raw_server_settings
                .as_ref()
                .and_then(|setting| setting_value.deserialize_setting(setting).log_err());

            let mut release_channel_settings = None;
            if let Some(release_settings) = &self
                .raw_user_settings
                .get(release_channel::RELEASE_CHANNEL.dev_name())
                && let Some(release_settings) = setting_value
                    .deserialize_setting(release_settings)
                    .log_err()
            {
                release_channel_settings = Some(release_settings);
            }

            let mut os_settings = None;
            if let Some(settings) = &self.raw_user_settings.get(env::consts::OS)
                && let Some(settings) = setting_value.deserialize_setting(settings).log_err()
            {
                os_settings = Some(settings);
            }

            let mut profile_settings = None;
            if let Some(active_profile) = cx.try_global::<ActiveSettingsProfileName>()
                && let Some(profiles) = self.raw_user_settings.get("profiles")
                && let Some(profile_json) = profiles.get(&active_profile.0)
            {
                profile_settings = setting_value.deserialize_setting(profile_json).log_err();
            }

            // If the global settings file changed, reload the global value for the field.
            if changed_local_path.is_none()
                && let Some(value) = setting_value
                    .load_setting(
                        SettingsSources {
                            default: &default_settings,
                            global: global_settings.as_ref(),
                            extensions: extension_settings.as_ref(),
                            user: user_settings.as_ref(),
                            release_channel: release_channel_settings.as_ref(),
                            operating_system: os_settings.as_ref(),
                            profile: profile_settings.as_ref(),
                            server: server_settings.as_ref(),
                            project: &[],
                        },
                        cx,
                    )
                    .log_err()
            {
                setting_value.set_global_value(value);
            }

            // Reload the local values for the setting.
            paths_stack.clear();
            project_settings_stack.clear();
            for ((root_id, directory_path), local_settings) in &self.raw_local_settings {
                // Build a stack of all of the local values for that setting.
                while let Some(prev_entry) = paths_stack.last() {
                    if let Some((prev_root_id, prev_path)) = prev_entry
                        && (root_id != prev_root_id || !directory_path.starts_with(prev_path))
                    {
                        paths_stack.pop();
                        project_settings_stack.pop();
                        continue;
                    }
                    break;
                }

                match setting_value.deserialize_setting(local_settings) {
                    Ok(local_settings) => {
                        paths_stack.push(Some((*root_id, directory_path.as_ref())));
                        project_settings_stack.push(local_settings);

                        // If a local settings file changed, then avoid recomputing local
                        // settings for any path outside of that directory.
                        if changed_local_path.is_some_and(
                            |(changed_root_id, changed_local_path)| {
                                *root_id != changed_root_id
                                    || !directory_path.starts_with(changed_local_path)
                            },
                        ) {
                            continue;
                        }

                        if let Some(value) = setting_value
                            .load_setting(
                                SettingsSources {
                                    default: &default_settings,
                                    global: global_settings.as_ref(),
                                    extensions: extension_settings.as_ref(),
                                    user: user_settings.as_ref(),
                                    release_channel: release_channel_settings.as_ref(),
                                    operating_system: os_settings.as_ref(),
                                    profile: profile_settings.as_ref(),
                                    server: server_settings.as_ref(),
                                    project: &project_settings_stack.iter().collect::<Vec<_>>(),
                                },
                                cx,
                            )
                            .log_err()
                        {
                            setting_value.set_local_value(*root_id, directory_path.clone(), value);
                        }
                    }
                    Err(error) => {
                        return Err(InvalidSettingsError::LocalSettings {
                            path: directory_path.join(local_settings_file_relative_path()),
                            message: error.to_string(),
                        });
                    }
                }
            }
        }
        Ok(())
    }

    pub fn editorconfig_properties(
        &self,
        for_worktree: WorktreeId,
        for_path: &Path,
    ) -> Option<EditorconfigProperties> {
        let mut properties = EditorconfigProperties::new();

        for (directory_with_config, _, parsed_editorconfig) in
            self.local_editorconfig_settings(for_worktree)
        {
            if !for_path.starts_with(&directory_with_config) {
                properties.use_fallbacks();
                return Some(properties);
            }
            let parsed_editorconfig = parsed_editorconfig?;
            if parsed_editorconfig.is_root {
                properties = EditorconfigProperties::new();
            }
            for section in parsed_editorconfig.sections {
                section.apply_to(&mut properties, for_path).log_err()?;
            }
        }

        properties.use_fallbacks();
        Some(properties)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InvalidSettingsError {
    LocalSettings { path: PathBuf, message: String },
    UserSettings { message: String },
    ServerSettings { message: String },
    DefaultSettings { message: String },
    Editorconfig { path: PathBuf, message: String },
    Tasks { path: PathBuf, message: String },
    Debug { path: PathBuf, message: String },
}

impl std::fmt::Display for InvalidSettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidSettingsError::LocalSettings { message, .. }
            | InvalidSettingsError::UserSettings { message }
            | InvalidSettingsError::ServerSettings { message }
            | InvalidSettingsError::DefaultSettings { message }
            | InvalidSettingsError::Tasks { message, .. }
            | InvalidSettingsError::Editorconfig { message, .. }
            | InvalidSettingsError::Debug { message, .. } => {
                write!(f, "{message}")
            }
        }
    }
}
impl std::error::Error for InvalidSettingsError {}

impl Debug for SettingsStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SettingsStore")
            .field(
                "types",
                &self
                    .setting_values
                    .values()
                    .map(|value| value.setting_type_name())
                    .collect::<Vec<_>>(),
            )
            .field("default_settings", &self.raw_default_settings)
            .field("user_settings", &self.raw_user_settings)
            .field("local_settings", &self.raw_local_settings)
            .finish_non_exhaustive()
    }
}

impl<T: Settings> AnySettingValue for SettingValue<T> {
    fn key(&self) -> Option<&'static str> {
        T::KEY
    }

    fn setting_type_name(&self) -> &'static str {
        type_name::<T>()
    }

    fn load_setting(
        &self,
        values: SettingsSources<DeserializedSetting>,
        cx: &mut App,
    ) -> Result<Box<dyn Any>> {
        Ok(Box::new(T::load(
            SettingsSources {
                default: values.default.0.downcast_ref::<T::FileContent>().unwrap(),
                global: values
                    .global
                    .map(|value| value.0.downcast_ref::<T::FileContent>().unwrap()),
                extensions: values
                    .extensions
                    .map(|value| value.0.downcast_ref::<T::FileContent>().unwrap()),
                user: values
                    .user
                    .map(|value| value.0.downcast_ref::<T::FileContent>().unwrap()),
                release_channel: values
                    .release_channel
                    .map(|value| value.0.downcast_ref::<T::FileContent>().unwrap()),
                operating_system: values
                    .operating_system
                    .map(|value| value.0.downcast_ref::<T::FileContent>().unwrap()),
                profile: values
                    .profile
                    .map(|value| value.0.downcast_ref::<T::FileContent>().unwrap()),
                server: values
                    .server
                    .map(|value| value.0.downcast_ref::<T::FileContent>().unwrap()),
                project: values
                    .project
                    .iter()
                    .map(|value| value.0.downcast_ref().unwrap())
                    .collect::<SmallVec<[_; 3]>>()
                    .as_slice(),
            },
            cx,
        )?))
    }

    fn deserialize_setting_with_key(
        &self,
        mut json: &Value,
    ) -> (Option<&'static str>, Result<DeserializedSetting>) {
        let mut key = None;
        if let Some(k) = T::KEY {
            if let Some(value) = json.get(k) {
                json = value;
                key = Some(k);
            } else if let Some((k, value)) = T::FALLBACK_KEY.and_then(|k| Some((k, json.get(k)?))) {
                json = value;
                key = Some(k);
            } else {
                let value = T::FileContent::default();
                return (T::KEY, Ok(DeserializedSetting(Box::new(value))));
            }
        }
        let value = T::FileContent::deserialize(json)
            .map(|value| DeserializedSetting(Box::new(value)))
            .map_err(anyhow::Error::from);
        (key, value)
    }

    fn all_local_values(&self) -> Vec<(WorktreeId, Arc<Path>, &dyn Any)> {
        self.local_values
            .iter()
            .map(|(id, path, value)| (*id, path.clone(), value as _))
            .collect()
    }

    fn value_for_path(&self, path: Option<SettingsLocation>) -> &dyn Any {
        if let Some(SettingsLocation { worktree_id, path }) = path {
            for (settings_root_id, settings_path, value) in self.local_values.iter().rev() {
                if worktree_id == *settings_root_id && path.starts_with(settings_path) {
                    return value;
                }
            }
        }
        self.global_value
            .as_ref()
            .unwrap_or_else(|| panic!("no default value for setting {}", self.setting_type_name()))
    }

    fn set_global_value(&mut self, value: Box<dyn Any>) {
        self.global_value = Some(*value.downcast().unwrap());
    }

    fn set_local_value(&mut self, root_id: WorktreeId, path: Arc<Path>, value: Box<dyn Any>) {
        let value = *value.downcast().unwrap();
        match self
            .local_values
            .binary_search_by_key(&(root_id, &path), |e| (e.0, &e.1))
        {
            Ok(ix) => self.local_values[ix].2 = value,
            Err(ix) => self.local_values.insert(ix, (root_id, path, value)),
        }
    }

    fn json_schema(&self, generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        T::FileContent::json_schema(generator)
    }

    fn edits_for_update(
        &self,
        raw_settings: &serde_json::Value,
        tab_size: usize,
        vscode_settings: &VsCodeSettings,
        text: &mut String,
        edits: &mut Vec<(Range<usize>, String)>,
    ) {
        let (key, deserialized_setting) = self.deserialize_setting_with_key(raw_settings);
        let old_content = match deserialized_setting {
            Ok(content) => content.0.downcast::<T::FileContent>().unwrap(),
            Err(_) => Box::<<T as Settings>::FileContent>::default(),
        };
        let mut new_content = old_content.clone();
        T::import_from_vscode(vscode_settings, &mut new_content);

        let old_value = serde_json::to_value(&old_content).unwrap();
        let new_value = serde_json::to_value(new_content).unwrap();

        let mut key_path = Vec::new();
        if let Some(key) = key {
            key_path.push(key);
        }

        update_value_in_json_text(
            text,
            &mut key_path,
            tab_size,
            &old_value,
            &new_value,
            T::PRESERVED_KEYS.unwrap_or_default(),
            edits,
        );
    }

    fn settings_ui_item(&self) -> SettingsUiEntry {
        <T as SettingsUi>::settings_ui_entry()
    }
}

#[cfg(test)]
mod tests {
    use crate::VsCodeSettingsSource;

    use super::*;
    // This is so the SettingsUi macro can still work properly
    use crate as settings;
    use serde_derive::Deserialize;
    use settings_ui_macros::SettingsUi;
    use unindent::Unindent;

    #[gpui::test]
    fn test_settings_store_basic(cx: &mut App) {
        let mut store = SettingsStore::new(cx);
        store.register_setting::<UserSettings>(cx);
        store.register_setting::<TurboSetting>(cx);
        store.register_setting::<MultiKeySettings>(cx);
        store
            .set_default_settings(
                r#"{
                    "turbo": false,
                    "user": {
                        "name": "John Doe",
                        "age": 30,
                        "staff": false
                    }
                }"#,
                cx,
            )
            .unwrap();

        assert_eq!(store.get::<TurboSetting>(None), &TurboSetting(false));
        assert_eq!(
            store.get::<UserSettings>(None),
            &UserSettings {
                name: "John Doe".to_string(),
                age: 30,
                staff: false,
            }
        );
        assert_eq!(
            store.get::<MultiKeySettings>(None),
            &MultiKeySettings {
                key1: String::new(),
                key2: String::new(),
            }
        );

        store
            .set_user_settings(
                r#"{
                    "turbo": true,
                    "user": { "age": 31 },
                    "key1": "a"
                }"#,
                cx,
            )
            .unwrap();

        assert_eq!(store.get::<TurboSetting>(None), &TurboSetting(true));
        assert_eq!(
            store.get::<UserSettings>(None),
            &UserSettings {
                name: "John Doe".to_string(),
                age: 31,
                staff: false
            }
        );

        store
            .set_local_settings(
                WorktreeId::from_usize(1),
                Path::new("/root1").into(),
                LocalSettingsKind::Settings,
                Some(r#"{ "user": { "staff": true } }"#),
                cx,
            )
            .unwrap();
        store
            .set_local_settings(
                WorktreeId::from_usize(1),
                Path::new("/root1/subdir").into(),
                LocalSettingsKind::Settings,
                Some(r#"{ "user": { "name": "Jane Doe" } }"#),
                cx,
            )
            .unwrap();

        store
            .set_local_settings(
                WorktreeId::from_usize(1),
                Path::new("/root2").into(),
                LocalSettingsKind::Settings,
                Some(r#"{ "user": { "age": 42 }, "key2": "b" }"#),
                cx,
            )
            .unwrap();

        assert_eq!(
            store.get::<UserSettings>(Some(SettingsLocation {
                worktree_id: WorktreeId::from_usize(1),
                path: Path::new("/root1/something"),
            })),
            &UserSettings {
                name: "John Doe".to_string(),
                age: 31,
                staff: true
            }
        );
        assert_eq!(
            store.get::<UserSettings>(Some(SettingsLocation {
                worktree_id: WorktreeId::from_usize(1),
                path: Path::new("/root1/subdir/something")
            })),
            &UserSettings {
                name: "Jane Doe".to_string(),
                age: 31,
                staff: true
            }
        );
        assert_eq!(
            store.get::<UserSettings>(Some(SettingsLocation {
                worktree_id: WorktreeId::from_usize(1),
                path: Path::new("/root2/something")
            })),
            &UserSettings {
                name: "John Doe".to_string(),
                age: 42,
                staff: false
            }
        );
        assert_eq!(
            store.get::<MultiKeySettings>(Some(SettingsLocation {
                worktree_id: WorktreeId::from_usize(1),
                path: Path::new("/root2/something")
            })),
            &MultiKeySettings {
                key1: "a".to_string(),
                key2: "b".to_string(),
            }
        );
    }

    #[gpui::test]
    fn test_setting_store_assign_json_before_register(cx: &mut App) {
        let mut store = SettingsStore::new(cx);
        store
            .set_default_settings(
                r#"{
                    "turbo": true,
                    "user": {
                        "name": "John Doe",
                        "age": 30,
                        "staff": false
                    },
                    "key1": "x"
                }"#,
                cx,
            )
            .unwrap();
        store
            .set_user_settings(r#"{ "turbo": false }"#, cx)
            .unwrap();
        store.register_setting::<UserSettings>(cx);
        store.register_setting::<TurboSetting>(cx);

        assert_eq!(store.get::<TurboSetting>(None), &TurboSetting(false));
        assert_eq!(
            store.get::<UserSettings>(None),
            &UserSettings {
                name: "John Doe".to_string(),
                age: 30,
                staff: false,
            }
        );

        store.register_setting::<MultiKeySettings>(cx);
        assert_eq!(
            store.get::<MultiKeySettings>(None),
            &MultiKeySettings {
                key1: "x".into(),
                key2: String::new(),
            }
        );
    }

    fn check_settings_update<T: Settings>(
        store: &mut SettingsStore,
        old_json: String,
        update: fn(&mut T::FileContent),
        expected_new_json: String,
        cx: &mut App,
    ) {
        store.set_user_settings(&old_json, cx).ok();
        let edits = store.edits_for_update::<T>(&old_json, update);
        let mut new_json = old_json;
        for (range, replacement) in edits.into_iter() {
            new_json.replace_range(range, &replacement);
        }
        pretty_assertions::assert_eq!(new_json, expected_new_json);
    }

    #[gpui::test]
    fn test_setting_store_update(cx: &mut App) {
        let mut store = SettingsStore::new(cx);
        store.register_setting::<MultiKeySettings>(cx);
        store.register_setting::<UserSettings>(cx);
        store.register_setting::<LanguageSettings>(cx);

        // entries added and updated
        check_settings_update::<LanguageSettings>(
            &mut store,
            r#"{
                "languages": {
                    "JSON": {
                        "language_setting_1": true
                    }
                }
            }"#
            .unindent(),
            |settings| {
                settings
                    .languages
                    .get_mut("JSON")
                    .unwrap()
                    .language_setting_1 = Some(false);
                settings.languages.insert(
                    "Rust".into(),
                    LanguageSettingEntry {
                        language_setting_2: Some(true),
                        ..Default::default()
                    },
                );
            },
            r#"{
                "languages": {
                    "Rust": {
                        "language_setting_2": true
                    },
                    "JSON": {
                        "language_setting_1": false
                    }
                }
            }"#
            .unindent(),
            cx,
        );

        // entries removed
        check_settings_update::<LanguageSettings>(
            &mut store,
            r#"{
                "languages": {
                    "Rust": {
                        "language_setting_2": true
                    },
                    "JSON": {
                        "language_setting_1": false
                    }
                }
            }"#
            .unindent(),
            |settings| {
                settings.languages.remove("JSON").unwrap();
            },
            r#"{
                "languages": {
                    "Rust": {
                        "language_setting_2": true
                    }
                }
            }"#
            .unindent(),
            cx,
        );

        check_settings_update::<LanguageSettings>(
            &mut store,
            r#"{
                "languages": {
                    "Rust": {
                        "language_setting_2": true
                    },
                    "JSON": {
                        "language_setting_1": false
                    }
                }
            }"#
            .unindent(),
            |settings| {
                settings.languages.remove("Rust").unwrap();
            },
            r#"{
                "languages": {
                    "JSON": {
                        "language_setting_1": false
                    }
                }
            }"#
            .unindent(),
            cx,
        );

        // weird formatting
        check_settings_update::<UserSettings>(
            &mut store,
            r#"{
                "user":   { "age": 36, "name": "Max", "staff": true }
                }"#
            .unindent(),
            |settings| settings.age = Some(37),
            r#"{
                "user":   { "age": 37, "name": "Max", "staff": true }
                }"#
            .unindent(),
            cx,
        );

        // single-line formatting, other keys
        check_settings_update::<MultiKeySettings>(
            &mut store,
            r#"{ "one": 1, "two": 2 }"#.unindent(),
            |settings| settings.key1 = Some("x".into()),
            r#"{ "key1": "x", "one": 1, "two": 2 }"#.unindent(),
            cx,
        );

        // empty object
        check_settings_update::<UserSettings>(
            &mut store,
            r#"{
                "user": {}
            }"#
            .unindent(),
            |settings| settings.age = Some(37),
            r#"{
                "user": {
                    "age": 37
                }
            }"#
            .unindent(),
            cx,
        );

        // no content
        check_settings_update::<UserSettings>(
            &mut store,
            r#""#.unindent(),
            |settings| settings.age = Some(37),
            r#"{
                "user": {
                    "age": 37
                }
            }
            "#
            .unindent(),
            cx,
        );

        check_settings_update::<UserSettings>(
            &mut store,
            r#"{
            }
            "#
            .unindent(),
            |settings| settings.age = Some(37),
            r#"{
                "user": {
                    "age": 37
                }
            }
            "#
            .unindent(),
            cx,
        );
    }

    #[gpui::test]
    fn test_vscode_import(cx: &mut App) {
        let mut store = SettingsStore::new(cx);
        store.register_setting::<UserSettings>(cx);
        store.register_setting::<JournalSettings>(cx);
        store.register_setting::<LanguageSettings>(cx);
        store.register_setting::<MultiKeySettings>(cx);

        // create settings that werent present
        check_vscode_import(
            &mut store,
            r#"{
            }
            "#
            .unindent(),
            r#" { "user.age": 37 } "#.to_owned(),
            r#"{
                "user": {
                    "age": 37
                }
            }
            "#
            .unindent(),
            cx,
        );

        // persist settings that were present
        check_vscode_import(
            &mut store,
            r#"{
                "user": {
                    "staff": true,
                    "age": 37
                }
            }
            "#
            .unindent(),
            r#"{ "user.age": 42 }"#.to_owned(),
            r#"{
                "user": {
                    "staff": true,
                    "age": 42
                }
            }
            "#
            .unindent(),
            cx,
        );

        // don't clobber settings that aren't present in vscode
        check_vscode_import(
            &mut store,
            r#"{
                "user": {
                    "staff": true,
                    "age": 37
                }
            }
            "#
            .unindent(),
            r#"{}"#.to_owned(),
            r#"{
                "user": {
                    "staff": true,
                    "age": 37
                }
            }
            "#
            .unindent(),
            cx,
        );

        // custom enum
        check_vscode_import(
            &mut store,
            r#"{
                "journal": {
                "hour_format": "hour12"
                }
            }
            "#
            .unindent(),
            r#"{ "time_format": "24" }"#.to_owned(),
            r#"{
                "journal": {
                "hour_format": "hour24"
                }
            }
            "#
            .unindent(),
            cx,
        );

        // Multiple keys for one setting
        check_vscode_import(
            &mut store,
            r#"{
                "key1": "value"
            }
            "#
            .unindent(),
            r#"{
                "key_1_first": "hello",
                "key_1_second": "world"
            }"#
            .to_owned(),
            r#"{
                "key1": "hello world"
            }
            "#
            .unindent(),
            cx,
        );

        // Merging lists together entries added and updated
        check_vscode_import(
            &mut store,
            r#"{
                "languages": {
                    "JSON": {
                        "language_setting_1": true
                    },
                    "Rust": {
                        "language_setting_2": true
                    }
                }
            }"#
            .unindent(),
            r#"{
                "vscode_languages": [
                    {
                        "name": "JavaScript",
                        "language_setting_1": true
                    },
                    {
                        "name": "Rust",
                        "language_setting_2": false
                    }
                ]
            }"#
            .to_owned(),
            r#"{
                "languages": {
                    "JavaScript": {
                        "language_setting_1": true
                    },
                    "JSON": {
                        "language_setting_1": true
                    },
                    "Rust": {
                        "language_setting_2": false
                    }
                }
            }"#
            .unindent(),
            cx,
        );
    }

    fn check_vscode_import(
        store: &mut SettingsStore,
        old: String,
        vscode: String,
        expected: String,
        cx: &mut App,
    ) {
        store.set_user_settings(&old, cx).ok();
        let new = store.get_vscode_edits(
            old,
            &VsCodeSettings::from_str(&vscode, VsCodeSettingsSource::VsCode).unwrap(),
        );
        pretty_assertions::assert_eq!(new, expected);
    }

    #[derive(Debug, PartialEq, Deserialize, SettingsUi)]
    struct UserSettings {
        name: String,
        age: u32,
        staff: bool,
    }

    #[derive(Default, Clone, Serialize, Deserialize, JsonSchema, SettingsUi)]
    struct UserSettingsContent {
        name: Option<String>,
        age: Option<u32>,
        staff: Option<bool>,
    }

    impl Settings for UserSettings {
        const KEY: Option<&'static str> = Some("user");
        type FileContent = UserSettingsContent;

        fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
            sources.json_merge()
        }

        fn import_from_vscode(vscode: &VsCodeSettings, current: &mut Self::FileContent) {
            vscode.u32_setting("user.age", &mut current.age);
        }
    }

    #[derive(Debug, Deserialize, PartialEq, SettingsUi)]
    struct TurboSetting(bool);

    impl Settings for TurboSetting {
        const KEY: Option<&'static str> = Some("turbo");
        type FileContent = Option<bool>;

        fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
            sources.json_merge()
        }

        fn import_from_vscode(_vscode: &VsCodeSettings, _current: &mut Self::FileContent) {}
    }

    #[derive(Clone, Debug, PartialEq, Deserialize, SettingsUi)]
    struct MultiKeySettings {
        #[serde(default)]
        key1: String,
        #[serde(default)]
        key2: String,
    }

    #[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
    struct MultiKeySettingsJson {
        key1: Option<String>,
        key2: Option<String>,
    }

    impl Settings for MultiKeySettings {
        const KEY: Option<&'static str> = None;

        type FileContent = MultiKeySettingsJson;

        fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
            sources.json_merge()
        }

        fn import_from_vscode(vscode: &VsCodeSettings, current: &mut Self::FileContent) {
            let first_value = vscode.read_string("key_1_first");
            let second_value = vscode.read_string("key_1_second");

            if let Some((first, second)) = first_value.zip(second_value) {
                current.key1 = Some(format!("{} {}", first, second));
            }
        }
    }

    #[derive(Debug, Deserialize, SettingsUi)]
    struct JournalSettings {
        pub path: String,
        pub hour_format: HourFormat,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
    #[serde(rename_all = "snake_case")]
    enum HourFormat {
        Hour12,
        Hour24,
    }

    #[derive(Clone, Default, Debug, Serialize, Deserialize, JsonSchema)]
    struct JournalSettingsJson {
        pub path: Option<String>,
        pub hour_format: Option<HourFormat>,
    }

    impl Settings for JournalSettings {
        const KEY: Option<&'static str> = Some("journal");

        type FileContent = JournalSettingsJson;

        fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
            sources.json_merge()
        }

        fn import_from_vscode(vscode: &VsCodeSettings, current: &mut Self::FileContent) {
            vscode.enum_setting("time_format", &mut current.hour_format, |s| match s {
                "12" => Some(HourFormat::Hour12),
                "24" => Some(HourFormat::Hour24),
                _ => None,
            });
        }
    }

    #[gpui::test]
    fn test_global_settings(cx: &mut App) {
        let mut store = SettingsStore::new(cx);
        store.register_setting::<UserSettings>(cx);
        store
            .set_default_settings(
                r#"{
                    "user": {
                        "name": "John Doe",
                        "age": 30,
                        "staff": false
                    }
                }"#,
                cx,
            )
            .unwrap();

        // Set global settings - these should override defaults but not user settings
        store
            .set_global_settings(
                r#"{
                    "user": {
                        "name": "Global User",
                        "age": 35,
                        "staff": true
                    }
                }"#,
                cx,
            )
            .unwrap();

        // Before user settings, global settings should apply
        assert_eq!(
            store.get::<UserSettings>(None),
            &UserSettings {
                name: "Global User".to_string(),
                age: 35,
                staff: true,
            }
        );

        // Set user settings - these should override both defaults and global
        store
            .set_user_settings(
                r#"{
                    "user": {
                        "age": 40
                    }
                }"#,
                cx,
            )
            .unwrap();

        // User settings should override global settings
        assert_eq!(
            store.get::<UserSettings>(None),
            &UserSettings {
                name: "Global User".to_string(), // Name from global settings
                age: 40,                         // Age from user settings
                staff: true,                     // Staff from global settings
            }
        );
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, SettingsUi)]
    struct LanguageSettings {
        #[serde(default)]
        languages: HashMap<String, LanguageSettingEntry>,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
    struct LanguageSettingEntry {
        language_setting_1: Option<bool>,
        language_setting_2: Option<bool>,
    }

    impl Settings for LanguageSettings {
        const KEY: Option<&'static str> = None;

        type FileContent = Self;

        fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
            sources.json_merge()
        }

        fn import_from_vscode(vscode: &VsCodeSettings, current: &mut Self::FileContent) {
            current.languages.extend(
                vscode
                    .read_value("vscode_languages")
                    .and_then(|value| value.as_array())
                    .map(|languages| {
                        languages
                            .iter()
                            .filter_map(|value| value.as_object())
                            .filter_map(|item| {
                                let mut rest = item.clone();
                                let name = rest.remove("name")?.as_str()?.to_string();
                                let entry = serde_json::from_value::<LanguageSettingEntry>(
                                    serde_json::Value::Object(rest),
                                )
                                .ok()?;

                                Some((name, entry))
                            })
                    })
                    .into_iter()
                    .flatten(),
            );
        }
    }
}
