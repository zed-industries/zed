use anyhow::{Context as _, Result};
use collections::{BTreeMap, HashMap, btree_map, hash_map};
use ec4rs::{ConfigParser, PropertiesSource, Section};
use fs::Fs;
use futures::{
    FutureExt, StreamExt,
    channel::{mpsc, oneshot},
    future::LocalBoxFuture,
};
use gpui::{App, AsyncApp, BorrowAppContext, Global, Task, UpdateGlobal};

use paths::{EDITORCONFIG_NAME, local_settings_file_relative_path, task_file_name};
use schemars::{JsonSchema, json_schema};
use serde_json::Value;
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId, type_name},
    fmt::Debug,
    ops::Range,
    path::PathBuf,
    rc::Rc,
    str::{self, FromStr},
    sync::Arc,
};
use util::{
    ResultExt as _,
    rel_path::RelPath,
    schemars::{DefaultDenyUnknownFields, replace_subschema},
};

pub type EditorconfigProperties = ec4rs::Properties;

use crate::{
    ActiveSettingsProfileName, FontFamilyName, IconThemeName, LanguageSettingsContent,
    LanguageToSettingsMap, SettingsJsonSchemaParams, ThemeName, VsCodeSettings, WorktreeId,
    merge_from::MergeFrom,
    parse_json_with_comments,
    settings_content::{
        ExtensionsSettingsContent, ProjectSettingsContent, SettingsContent, UserSettingsContent,
    },
    update_value_in_json_text,
};

pub trait SettingsKey: 'static + Send + Sync {
    /// The name of a key within the JSON file from which this setting should
    /// be deserialized. If this is `None`, then the setting will be deserialized
    /// from the root object.
    const KEY: Option<&'static str>;

    const FALLBACK_KEY: Option<&'static str> = None;
}

/// A value that can be defined as a user setting.
///
/// Settings can be loaded from a combination of multiple JSON files.
pub trait Settings: 'static + Send + Sync + Sized {
    /// The name of the keys in the [`FileContent`](Self::FileContent) that should
    /// always be written to a settings file, even if their value matches the default
    /// value.
    ///
    /// This is useful for tagged [`FileContent`](Self::FileContent)s where the tag
    /// is a "version" field that should always be persisted, even if the current
    /// user settings match the current version of the settings.
    const PRESERVED_KEYS: Option<&'static [&'static str]> = None;

    /// Read the value from default.json.
    ///
    /// This function *should* panic if default values are missing,
    /// and you should add a default to default.json for documentation.
    fn from_settings(content: &SettingsContent, cx: &mut App) -> Self;

    fn missing_default() -> anyhow::Error {
        anyhow::anyhow!("missing default for: {}", std::any::type_name::<Self>())
    }

    /// Use [the helpers in the vscode_import module](crate::vscode_import) to apply known
    /// equivalent settings from a vscode config to our config
    fn import_from_vscode(_vscode: &VsCodeSettings, _current: &mut SettingsContent) {}

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
pub struct SettingsLocation<'a> {
    pub worktree_id: WorktreeId,
    pub path: &'a RelPath,
}

pub struct SettingsStore {
    setting_values: HashMap<TypeId, Box<dyn AnySettingValue>>,
    default_settings: Rc<SettingsContent>,
    user_settings: Option<UserSettingsContent>,
    global_settings: Option<Box<SettingsContent>>,

    extension_settings: Option<Box<SettingsContent>>,
    server_settings: Option<Box<SettingsContent>>,

    merged_settings: Rc<SettingsContent>,

    local_settings: BTreeMap<(WorktreeId, Arc<RelPath>), SettingsContent>,
    raw_editorconfig_settings: BTreeMap<(WorktreeId, Arc<RelPath>), (String, Option<Editorconfig>)>,

    _setting_file_updates: Task<()>,
    setting_file_updates_tx:
        mpsc::UnboundedSender<Box<dyn FnOnce(AsyncApp) -> LocalBoxFuture<'static, Result<()>>>>,
}

#[derive(Clone, PartialEq)]
pub enum SettingsFile {
    User,
    Global,
    Extension,
    Server,
    Default,
    Local((WorktreeId, Arc<RelPath>)),
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
    local_values: Vec<(WorktreeId, Arc<RelPath>, T)>,
}

trait AnySettingValue: 'static + Send + Sync {
    fn setting_type_name(&self) -> &'static str;

    fn from_settings(&self, s: &SettingsContent, cx: &mut App) -> Box<dyn Any>;

    fn value_for_path(&self, path: Option<SettingsLocation>) -> &dyn Any;
    fn all_local_values(&self) -> Vec<(WorktreeId, Arc<RelPath>, &dyn Any)>;
    fn set_global_value(&mut self, value: Box<dyn Any>);
    fn set_local_value(&mut self, root_id: WorktreeId, path: Arc<RelPath>, value: Box<dyn Any>);
    fn import_from_vscode(
        &self,
        vscode_settings: &VsCodeSettings,
        settings_content: &mut SettingsContent,
    );
}

impl SettingsStore {
    pub fn new(cx: &App, default_settings: &str) -> Self {
        let (setting_file_updates_tx, mut setting_file_updates_rx) = mpsc::unbounded();
        let default_settings: Rc<SettingsContent> =
            parse_json_with_comments(default_settings).unwrap();
        Self {
            setting_values: Default::default(),
            default_settings: default_settings.clone(),
            global_settings: None,
            server_settings: None,
            user_settings: None,
            extension_settings: None,

            merged_settings: default_settings,
            local_settings: BTreeMap::default(),
            raw_editorconfig_settings: BTreeMap::default(),
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
        let value = T::from_settings(&self.merged_settings, cx);
        setting_value.set_global_value(Box::new(value));
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
    pub fn get_all_locals<T: Settings>(&self) -> Vec<(WorktreeId, Arc<RelPath>, &T)> {
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

    /// Get the user's settings content.
    ///
    /// For user-facing functionality use the typed setting interface.
    /// (e.g. ProjectSettings::get_global(cx))
    pub fn raw_user_settings(&self) -> Option<&UserSettingsContent> {
        self.user_settings.as_ref()
    }

    /// Get the default settings content as a raw JSON value.
    pub fn raw_default_settings(&self) -> &SettingsContent {
        &self.default_settings
    }

    /// Get the configured settings profile names.
    pub fn configured_settings_profiles(&self) -> impl Iterator<Item = &str> {
        self.user_settings
            .iter()
            .flat_map(|settings| settings.profiles.keys().map(|k| k.as_str()))
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test(cx: &mut App) -> Self {
        Self::new(cx, &crate::test_settings())
    }

    /// Updates the value of a setting in the user's global configuration.
    ///
    /// This is only for tests. Normally, settings are only loaded from
    /// JSON files.
    #[cfg(any(test, feature = "test-support"))]
    pub fn update_user_settings(
        &mut self,
        cx: &mut App,
        update: impl FnOnce(&mut SettingsContent),
    ) {
        let mut content = self.user_settings.clone().unwrap_or_default().content;
        update(&mut content);
        let new_text = serde_json::to_string(&UserSettingsContent {
            content,
            ..Default::default()
        })
        .unwrap();
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

    pub fn update_settings_file(
        &self,
        fs: Arc<dyn Fs>,
        update: impl 'static + Send + FnOnce(&mut SettingsContent, &App),
    ) {
        _ = self.update_settings_file_inner(fs, move |old_text: String, cx: AsyncApp| {
            cx.read_global(|store: &SettingsStore, cx| {
                store.new_text_for_update(old_text, |content| update(content, cx))
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

    pub fn get_all_files(&self) -> Vec<SettingsFile> {
        let mut files = Vec::from_iter(
            self.local_settings
                .keys()
                // rev because these are sorted by path, so highest precedence is last
                .rev()
                .cloned()
                .map(SettingsFile::Local),
        );

        if self.server_settings.is_some() {
            files.push(SettingsFile::Server);
        }
        // ignoring profiles
        // ignoring os profiles
        // ignoring release channel profiles

        if self.user_settings.is_some() {
            files.push(SettingsFile::User);
        }
        if self.extension_settings.is_some() {
            files.push(SettingsFile::Extension);
        }
        if self.global_settings.is_some() {
            files.push(SettingsFile::Global);
        }
        files.push(SettingsFile::Default);
        files
    }
}

impl SettingsStore {
    /// Updates the value of a setting in a JSON file, returning the new text
    /// for that JSON file.
    pub fn new_text_for_update(
        &self,
        old_text: String,
        update: impl FnOnce(&mut SettingsContent),
    ) -> String {
        let edits = self.edits_for_update(&old_text, update);
        let mut new_text = old_text;
        for (range, replacement) in edits.into_iter() {
            new_text.replace_range(range, &replacement);
        }
        new_text
    }

    pub fn get_vscode_edits(&self, old_text: String, vscode: &VsCodeSettings) -> String {
        self.new_text_for_update(old_text, |settings_content| {
            for v in self.setting_values.values() {
                v.import_from_vscode(vscode, settings_content)
            }
        })
    }

    /// Updates the value of a setting in a JSON file, returning a list
    /// of edits to apply to the JSON file.
    pub fn edits_for_update(
        &self,
        text: &str,
        update: impl FnOnce(&mut SettingsContent),
    ) -> Vec<(Range<usize>, String)> {
        let old_content: UserSettingsContent =
            parse_json_with_comments(text).log_err().unwrap_or_default();
        let mut new_content = old_content.clone();
        update(&mut new_content.content);

        let old_value = serde_json::to_value(&old_content).unwrap();
        let new_value = serde_json::to_value(new_content).unwrap();

        let mut key_path = Vec::new();
        let mut edits = Vec::new();
        let tab_size = self.json_tab_size();
        let mut text = text.to_string();
        update_value_in_json_text(
            &mut text,
            &mut key_path,
            tab_size,
            &old_value,
            &new_value,
            &mut edits,
        );
        edits
    }

    pub fn json_tab_size(&self) -> usize {
        2
    }

    /// Sets the default settings via a JSON string.
    ///
    /// The string should contain a JSON object with a default value for every setting.
    pub fn set_default_settings(
        &mut self,
        default_settings_content: &str,
        cx: &mut App,
    ) -> Result<()> {
        self.default_settings = parse_json_with_comments(default_settings_content)?;
        self.recompute_values(None, cx)?;
        Ok(())
    }

    /// Sets the user settings via a JSON string.
    pub fn set_user_settings(&mut self, user_settings_content: &str, cx: &mut App) -> Result<()> {
        let settings: UserSettingsContent = if user_settings_content.is_empty() {
            parse_json_with_comments("{}")?
        } else {
            parse_json_with_comments(user_settings_content)?
        };

        self.user_settings = Some(settings);
        self.recompute_values(None, cx)?;
        Ok(())
    }

    /// Sets the global settings via a JSON string.
    pub fn set_global_settings(
        &mut self,
        global_settings_content: &str,
        cx: &mut App,
    ) -> Result<()> {
        let settings: SettingsContent = if global_settings_content.is_empty() {
            parse_json_with_comments("{}")?
        } else {
            parse_json_with_comments(global_settings_content)?
        };

        self.global_settings = Some(Box::new(settings));
        self.recompute_values(None, cx)?;
        Ok(())
    }

    pub fn set_server_settings(
        &mut self,
        server_settings_content: &str,
        cx: &mut App,
    ) -> Result<()> {
        let settings: Option<SettingsContent> = if server_settings_content.is_empty() {
            None
        } else {
            parse_json_with_comments(server_settings_content)?
        };

        // Rewrite the server settings into a content type
        self.server_settings = settings.map(|settings| Box::new(settings));

        self.recompute_values(None, cx)?;
        Ok(())
    }

    /// Add or remove a set of local settings via a JSON string.
    pub fn set_local_settings(
        &mut self,
        root_id: WorktreeId,
        directory_path: Arc<RelPath>,
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
                    path: directory_path
                        .join(RelPath::unix(task_file_name()).unwrap())
                        .as_std_path()
                        .to_path_buf(),
                });
            }
            (LocalSettingsKind::Debug, _) => {
                return Err(InvalidSettingsError::Debug {
                    message: "Attempted to submit debugger config into the settings store"
                        .to_string(),
                    path: directory_path
                        .join(RelPath::unix(task_file_name()).unwrap())
                        .as_std_path()
                        .to_path_buf(),
                });
            }
            (LocalSettingsKind::Settings, None) => {
                zed_settings_changed = self
                    .local_settings
                    .remove(&(root_id, directory_path.clone()))
                    .is_some()
            }
            (LocalSettingsKind::Editorconfig, None) => {
                self.raw_editorconfig_settings
                    .remove(&(root_id, directory_path.clone()));
            }
            (LocalSettingsKind::Settings, Some(settings_contents)) => {
                let new_settings = parse_json_with_comments::<ProjectSettingsContent>(
                    settings_contents,
                )
                .map_err(|e| InvalidSettingsError::LocalSettings {
                    path: directory_path.join(local_settings_file_relative_path()),
                    message: e.to_string(),
                })?;
                match self.local_settings.entry((root_id, directory_path.clone())) {
                    btree_map::Entry::Vacant(v) => {
                        v.insert(SettingsContent {
                            project: new_settings,
                            ..Default::default()
                        });
                        zed_settings_changed = true;
                    }
                    btree_map::Entry::Occupied(mut o) => {
                        if &o.get().project != &new_settings {
                            o.insert(SettingsContent {
                                project: new_settings,
                                ..Default::default()
                            });
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
                                path: directory_path
                                    .join(RelPath::unix(EDITORCONFIG_NAME).unwrap()),
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
                                        path: directory_path
                                            .join(RelPath::unix(EDITORCONFIG_NAME).unwrap()),
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

    pub fn set_extension_settings(
        &mut self,
        content: ExtensionsSettingsContent,
        cx: &mut App,
    ) -> Result<()> {
        self.extension_settings = Some(Box::new(SettingsContent {
            project: ProjectSettingsContent {
                all_languages: content.all_languages,
                ..Default::default()
            },
            ..Default::default()
        }));
        self.recompute_values(None, cx)?;
        Ok(())
    }

    /// Add or remove a set of local settings via a JSON string.
    pub fn clear_local_settings(&mut self, root_id: WorktreeId, cx: &mut App) -> Result<()> {
        self.local_settings
            .retain(|(worktree_id, _), _| worktree_id != &root_id);
        self.recompute_values(Some((root_id, RelPath::empty())), cx)?;
        Ok(())
    }

    pub fn local_settings(
        &self,
        root_id: WorktreeId,
    ) -> impl '_ + Iterator<Item = (Arc<RelPath>, &ProjectSettingsContent)> {
        self.local_settings
            .range(
                (root_id, RelPath::empty().into())
                    ..(
                        WorktreeId::from_usize(root_id.to_usize() + 1),
                        RelPath::empty().into(),
                    ),
            )
            .map(|((_, path), content)| (path.clone(), &content.project))
    }

    pub fn local_editorconfig_settings(
        &self,
        root_id: WorktreeId,
    ) -> impl '_ + Iterator<Item = (Arc<RelPath>, String, Option<Editorconfig>)> {
        self.raw_editorconfig_settings
            .range(
                (root_id, RelPath::empty().into())
                    ..(
                        WorktreeId::from_usize(root_id.to_usize() + 1),
                        RelPath::empty().into(),
                    ),
            )
            .map(|((_, path), (content, parsed_content))| {
                (path.clone(), content.clone(), parsed_content.clone())
            })
    }

    pub fn json_schema(&self, params: &SettingsJsonSchemaParams) -> Value {
        let mut generator = schemars::generate::SchemaSettings::draft2019_09()
            .with_transform(DefaultDenyUnknownFields)
            .into_generator();

        UserSettingsContent::json_schema(&mut generator);

        let language_settings_content_ref = generator
            .subschema_for::<LanguageSettingsContent>()
            .to_value();

        replace_subschema::<LanguageToSettingsMap>(&mut generator, || {
            json_schema!({
                "type": "object",
                "properties": params
                    .language_names
                    .iter()
                    .map(|name| {
                        (
                            name.clone(),
                            language_settings_content_ref.clone(),
                        )
                    })
                    .collect::<serde_json::Map<_, _>>(),
                "errorMessage": "No language with this name is installed."
            })
        });

        replace_subschema::<FontFamilyName>(&mut generator, || {
            json_schema!({
                "type": "string",
                "enum": params.font_names,
            })
        });

        replace_subschema::<ThemeName>(&mut generator, || {
            json_schema!({
                "type": "string",
                "enum": params.theme_names,
            })
        });

        replace_subschema::<IconThemeName>(&mut generator, || {
            json_schema!({
                "type": "string",
                "enum": params.icon_theme_names,
            })
        });

        generator
            .root_schema_for::<UserSettingsContent>()
            .to_value()
    }

    fn recompute_values(
        &mut self,
        changed_local_path: Option<(WorktreeId, &RelPath)>,
        cx: &mut App,
    ) -> std::result::Result<(), InvalidSettingsError> {
        // Reload the global and local values for every setting.
        let mut project_settings_stack = Vec::<SettingsContent>::new();
        let mut paths_stack = Vec::<Option<(WorktreeId, &RelPath)>>::new();

        if changed_local_path.is_none() {
            let mut merged = self.default_settings.as_ref().clone();
            merged.merge_from_option(self.extension_settings.as_deref());
            merged.merge_from_option(self.global_settings.as_deref());
            if let Some(user_settings) = self.user_settings.as_ref() {
                merged.merge_from(&user_settings.content);
                merged.merge_from_option(user_settings.for_release_channel());
                merged.merge_from_option(user_settings.for_os());
                merged.merge_from_option(user_settings.for_profile(cx));
            }
            merged.merge_from_option(self.server_settings.as_deref());
            self.merged_settings = Rc::new(merged);

            for setting_value in self.setting_values.values_mut() {
                let value = setting_value.from_settings(&self.merged_settings, cx);
                setting_value.set_global_value(value);
            }
        }

        for ((root_id, directory_path), local_settings) in &self.local_settings {
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

            paths_stack.push(Some((*root_id, directory_path.as_ref())));
            let mut merged_local_settings = if let Some(deepest) = project_settings_stack.last() {
                (*deepest).clone()
            } else {
                self.merged_settings.as_ref().clone()
            };
            merged_local_settings.merge_from(local_settings);

            project_settings_stack.push(merged_local_settings);

            // If a local settings file changed, then avoid recomputing local
            // settings for any path outside of that directory.
            if changed_local_path.is_some_and(|(changed_root_id, changed_local_path)| {
                *root_id != changed_root_id || !directory_path.starts_with(changed_local_path)
            }) {
                continue;
            }

            for setting_value in self.setting_values.values_mut() {
                let value =
                    setting_value.from_settings(&project_settings_stack.last().unwrap(), cx);
                setting_value.set_local_value(*root_id, directory_path.clone(), value);
            }
        }
        Ok(())
    }

    pub fn editorconfig_properties(
        &self,
        for_worktree: WorktreeId,
        for_path: &RelPath,
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
                section
                    .apply_to(&mut properties, for_path.as_std_path())
                    .log_err()?;
            }
        }

        properties.use_fallbacks();
        Some(properties)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InvalidSettingsError {
    LocalSettings { path: Arc<RelPath>, message: String },
    UserSettings { message: String },
    ServerSettings { message: String },
    DefaultSettings { message: String },
    Editorconfig { path: Arc<RelPath>, message: String },
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
            .field("default_settings", &self.default_settings)
            .field("user_settings", &self.user_settings)
            .field("local_settings", &self.local_settings)
            .finish_non_exhaustive()
    }
}

impl<T: Settings> AnySettingValue for SettingValue<T> {
    fn from_settings(&self, s: &SettingsContent, cx: &mut App) -> Box<dyn Any> {
        Box::new(T::from_settings(s, cx)) as _
    }

    fn setting_type_name(&self) -> &'static str {
        type_name::<T>()
    }

    fn all_local_values(&self) -> Vec<(WorktreeId, Arc<RelPath>, &dyn Any)> {
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

    fn set_local_value(&mut self, root_id: WorktreeId, path: Arc<RelPath>, value: Box<dyn Any>) {
        let value = *value.downcast().unwrap();
        match self
            .local_values
            .binary_search_by_key(&(root_id, &path), |e| (e.0, &e.1))
        {
            Ok(ix) => self.local_values[ix].2 = value,
            Err(ix) => self.local_values.insert(ix, (root_id, path, value)),
        }
    }

    fn import_from_vscode(
        &self,
        vscode_settings: &VsCodeSettings,
        settings_content: &mut SettingsContent,
    ) {
        T::import_from_vscode(vscode_settings, settings_content);
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use crate::{
        ClosePosition, ItemSettingsContent, VsCodeSettingsSource, default_settings,
        settings_content::LanguageSettingsContent, test_settings,
    };

    use super::*;
    use unindent::Unindent;
    use util::rel_path::rel_path;

    #[derive(Debug, PartialEq)]
    struct AutoUpdateSetting {
        auto_update: bool,
    }

    impl Settings for AutoUpdateSetting {
        fn from_settings(content: &SettingsContent, _: &mut App) -> Self {
            AutoUpdateSetting {
                auto_update: content.auto_update.unwrap(),
            }
        }
    }

    #[derive(Debug, PartialEq)]
    struct ItemSettings {
        close_position: ClosePosition,
        git_status: bool,
    }

    impl Settings for ItemSettings {
        fn from_settings(content: &SettingsContent, _: &mut App) -> Self {
            let content = content.tabs.clone().unwrap();
            ItemSettings {
                close_position: content.close_position.unwrap(),
                git_status: content.git_status.unwrap(),
            }
        }

        fn import_from_vscode(vscode: &VsCodeSettings, content: &mut SettingsContent) {
            let mut show = None;

            vscode.bool_setting("workbench.editor.decorations.colors", &mut show);
            if let Some(show) = show {
                content
                    .tabs
                    .get_or_insert_default()
                    .git_status
                    .replace(show);
            }
        }
    }

    #[derive(Debug, PartialEq)]
    struct DefaultLanguageSettings {
        tab_size: NonZeroU32,
        preferred_line_length: u32,
    }

    impl Settings for DefaultLanguageSettings {
        fn from_settings(content: &SettingsContent, _: &mut App) -> Self {
            let content = &content.project.all_languages.defaults;
            DefaultLanguageSettings {
                tab_size: content.tab_size.unwrap(),
                preferred_line_length: content.preferred_line_length.unwrap(),
            }
        }

        fn import_from_vscode(vscode: &VsCodeSettings, content: &mut SettingsContent) {
            let content = &mut content.project.all_languages.defaults;

            if let Some(size) = vscode
                .read_value("editor.tabSize")
                .and_then(|v| v.as_u64())
                .and_then(|n| NonZeroU32::new(n as u32))
            {
                content.tab_size = Some(size);
            }
        }
    }

    #[gpui::test]
    fn test_settings_store_basic(cx: &mut App) {
        let mut store = SettingsStore::new(cx, &default_settings());
        store.register_setting::<AutoUpdateSetting>(cx);
        store.register_setting::<ItemSettings>(cx);
        store.register_setting::<DefaultLanguageSettings>(cx);

        assert_eq!(
            store.get::<AutoUpdateSetting>(None),
            &AutoUpdateSetting { auto_update: true }
        );
        assert_eq!(
            store.get::<ItemSettings>(None).close_position,
            ClosePosition::Right
        );

        store
            .set_user_settings(
                r#"{
                    "auto_update": false,
                    "tabs": {
                      "close_position": "left"
                    }
                }"#,
                cx,
            )
            .unwrap();

        assert_eq!(
            store.get::<AutoUpdateSetting>(None),
            &AutoUpdateSetting { auto_update: false }
        );
        assert_eq!(
            store.get::<ItemSettings>(None).close_position,
            ClosePosition::Left
        );

        store
            .set_local_settings(
                WorktreeId::from_usize(1),
                rel_path("root1").into(),
                LocalSettingsKind::Settings,
                Some(r#"{ "tab_size": 5 }"#),
                cx,
            )
            .unwrap();
        store
            .set_local_settings(
                WorktreeId::from_usize(1),
                rel_path("root1/subdir").into(),
                LocalSettingsKind::Settings,
                Some(r#"{ "preferred_line_length": 50 }"#),
                cx,
            )
            .unwrap();

        store
            .set_local_settings(
                WorktreeId::from_usize(1),
                rel_path("root2").into(),
                LocalSettingsKind::Settings,
                Some(r#"{ "tab_size": 9, "auto_update": true}"#),
                cx,
            )
            .unwrap();

        assert_eq!(
            store.get::<DefaultLanguageSettings>(Some(SettingsLocation {
                worktree_id: WorktreeId::from_usize(1),
                path: rel_path("root1/something"),
            })),
            &DefaultLanguageSettings {
                preferred_line_length: 80,
                tab_size: 5.try_into().unwrap(),
            }
        );
        assert_eq!(
            store.get::<DefaultLanguageSettings>(Some(SettingsLocation {
                worktree_id: WorktreeId::from_usize(1),
                path: rel_path("root1/subdir/something"),
            })),
            &DefaultLanguageSettings {
                preferred_line_length: 50,
                tab_size: 5.try_into().unwrap(),
            }
        );
        assert_eq!(
            store.get::<DefaultLanguageSettings>(Some(SettingsLocation {
                worktree_id: WorktreeId::from_usize(1),
                path: rel_path("root2/something"),
            })),
            &DefaultLanguageSettings {
                preferred_line_length: 80,
                tab_size: 9.try_into().unwrap(),
            }
        );
        assert_eq!(
            store.get::<AutoUpdateSetting>(Some(SettingsLocation {
                worktree_id: WorktreeId::from_usize(1),
                path: rel_path("root2/something")
            })),
            &AutoUpdateSetting { auto_update: false }
        );
    }

    #[gpui::test]
    fn test_setting_store_assign_json_before_register(cx: &mut App) {
        let mut store = SettingsStore::new(cx, &test_settings());
        store
            .set_user_settings(r#"{ "auto_update": false }"#, cx)
            .unwrap();
        store.register_setting::<AutoUpdateSetting>(cx);

        assert_eq!(
            store.get::<AutoUpdateSetting>(None),
            &AutoUpdateSetting { auto_update: false }
        );
    }

    #[track_caller]
    fn check_settings_update(
        store: &mut SettingsStore,
        old_json: String,
        update: fn(&mut SettingsContent),
        expected_new_json: String,
        cx: &mut App,
    ) {
        store.set_user_settings(&old_json, cx).ok();
        let edits = store.edits_for_update(&old_json, update);
        let mut new_json = old_json;
        for (range, replacement) in edits.into_iter() {
            new_json.replace_range(range, &replacement);
        }
        pretty_assertions::assert_eq!(new_json, expected_new_json);
    }

    #[gpui::test]
    fn test_setting_store_update(cx: &mut App) {
        let mut store = SettingsStore::new(cx, &test_settings());

        // entries added and updated
        check_settings_update(
            &mut store,
            r#"{
                "languages": {
                    "JSON": {
                        "auto_indent": true
                    }
                }
            }"#
            .unindent(),
            |settings| {
                settings
                    .languages_mut()
                    .get_mut("JSON")
                    .unwrap()
                    .auto_indent = Some(false);

                settings.languages_mut().insert(
                    "Rust".into(),
                    LanguageSettingsContent {
                        auto_indent: Some(true),
                        ..Default::default()
                    },
                );
            },
            r#"{
                "languages": {
                    "Rust": {
                        "auto_indent": true
                    },
                    "JSON": {
                        "auto_indent": false
                    }
                }
            }"#
            .unindent(),
            cx,
        );

        // entries removed
        check_settings_update(
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
                settings.languages_mut().remove("JSON").unwrap();
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

        check_settings_update(
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
                settings.languages_mut().remove("Rust").unwrap();
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
        check_settings_update(
            &mut store,
            r#"{
                "tabs":   { "close_position": "left", "name": "Max"  }
                }"#
            .unindent(),
            |settings| {
                settings.tabs.as_mut().unwrap().close_position = Some(ClosePosition::Left);
            },
            r#"{
                "tabs":   { "close_position": "left", "name": "Max"  }
                }"#
            .unindent(),
            cx,
        );

        // single-line formatting, other keys
        check_settings_update(
            &mut store,
            r#"{ "one": 1, "two": 2 }"#.to_owned(),
            |settings| settings.auto_update = Some(true),
            r#"{ "auto_update": true, "one": 1, "two": 2 }"#.to_owned(),
            cx,
        );

        // empty object
        check_settings_update(
            &mut store,
            r#"{
                "tabs": {}
            }"#
            .unindent(),
            |settings| settings.tabs.as_mut().unwrap().close_position = Some(ClosePosition::Left),
            r#"{
                "tabs": {
                    "close_position": "left"
                }
            }"#
            .unindent(),
            cx,
        );

        // no content
        check_settings_update(
            &mut store,
            r#""#.unindent(),
            |settings| {
                settings.tabs = Some(ItemSettingsContent {
                    git_status: Some(true),
                    ..Default::default()
                })
            },
            r#"{
                "tabs": {
                    "git_status": true
                }
            }
            "#
            .unindent(),
            cx,
        );

        check_settings_update(
            &mut store,
            r#"{
            }
            "#
            .unindent(),
            |settings| settings.title_bar.get_or_insert_default().show_branch_name = Some(true),
            r#"{
                "title_bar": {
                    "show_branch_name": true
                }
            }
            "#
            .unindent(),
            cx,
        );
    }

    #[gpui::test]
    fn test_vscode_import(cx: &mut App) {
        let mut store = SettingsStore::new(cx, &test_settings());
        store.register_setting::<DefaultLanguageSettings>(cx);
        store.register_setting::<ItemSettings>(cx);
        store.register_setting::<AutoUpdateSetting>(cx);

        // create settings that werent present
        check_vscode_import(
            &mut store,
            r#"{
            }
            "#
            .unindent(),
            r#" { "editor.tabSize": 37 } "#.to_owned(),
            r#"{
                "tab_size": 37
            }
            "#
            .unindent(),
            cx,
        );

        // persist settings that were present
        check_vscode_import(
            &mut store,
            r#"{
                "preferred_line_length": 99,
            }
            "#
            .unindent(),
            r#"{ "editor.tabSize": 42 }"#.to_owned(),
            r#"{
                "tab_size": 42,
                "preferred_line_length": 99,
            }
            "#
            .unindent(),
            cx,
        );

        // don't clobber settings that aren't present in vscode
        check_vscode_import(
            &mut store,
            r#"{
                "preferred_line_length": 99,
                "tab_size": 42
            }
            "#
            .unindent(),
            r#"{}"#.to_owned(),
            r#"{
                "preferred_line_length": 99,
                "tab_size": 42
            }
            "#
            .unindent(),
            cx,
        );

        // custom enum
        check_vscode_import(
            &mut store,
            r#"{
            }
            "#
            .unindent(),
            r#"{ "workbench.editor.decorations.colors": true }"#.to_owned(),
            r#"{
                "tabs": {
                    "git_status": true
                }
            }
            "#
            .unindent(),
            cx,
        );
    }

    #[track_caller]
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

    #[gpui::test]
    fn test_update_git_settings(cx: &mut App) {
        let store = SettingsStore::new(cx, &test_settings());

        let actual = store.new_text_for_update("{}".to_string(), |current| {
            current
                .git
                .get_or_insert_default()
                .inline_blame
                .get_or_insert_default()
                .enabled = Some(true);
        });
        assert_eq!(
            actual,
            r#"{
            "git": {
                "inline_blame": {
                    "enabled": true
                }
            }
        }
        "#
            .unindent()
        );
    }

    #[gpui::test]
    fn test_global_settings(cx: &mut App) {
        let mut store = SettingsStore::new(cx, &test_settings());
        store.register_setting::<ItemSettings>(cx);

        // Set global settings - these should override defaults but not user settings
        store
            .set_global_settings(
                r#"{
                    "tabs": {
                        "close_position": "right",
                        "git_status": true,
                    }
                }"#,
                cx,
            )
            .unwrap();

        // Before user settings, global settings should apply
        assert_eq!(
            store.get::<ItemSettings>(None),
            &ItemSettings {
                close_position: ClosePosition::Right,
                git_status: true,
            }
        );

        // Set user settings - these should override both defaults and global
        store
            .set_user_settings(
                r#"{
                    "tabs": {
                        "close_position": "left"
                    }
                }"#,
                cx,
            )
            .unwrap();

        // User settings should override global settings
        assert_eq!(
            store.get::<ItemSettings>(None),
            &ItemSettings {
                close_position: ClosePosition::Left,
                git_status: true, // Staff from global settings
            }
        );
    }
}
