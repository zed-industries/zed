use anyhow::{anyhow, Context, Result};
use collections::{btree_map, hash_map, BTreeMap, HashMap};
use gpui::{AppContext, AsyncAppContext, Global};
use lazy_static::lazy_static;
use schemars::{gen::SchemaGenerator, schema::RootSchema, JsonSchema};
use serde::{de::DeserializeOwned, Deserialize as _, Serialize};
use smallvec::SmallVec;
use std::{
    any::{type_name, Any, TypeId},
    fmt::Debug,
    ops::Range,
    path::Path,
    str,
    sync::Arc,
};
use util::{merge_non_null_json_value_into, RangeExt, ResultExt as _};

/// A value that can be defined as a user setting.
///
/// Settings can be loaded from a combination of multiple JSON files.
pub trait Settings: 'static + Send + Sync {
    /// The name of a key within the JSON file from which this setting should
    /// be deserialized. If this is `None`, then the setting will be deserialized
    /// from the root object.
    const KEY: Option<&'static str>;

    /// The type that is stored in an individual JSON file.
    type FileContent: Clone + Default + Serialize + DeserializeOwned + JsonSchema;

    /// The logic for combining together values from one or more JSON files into the
    /// final value for this setting.
    ///
    /// The user values are ordered from least specific (the global settings file)
    /// to most specific (the innermost local settings file).
    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        cx: &mut AppContext,
    ) -> Result<Self>
    where
        Self: Sized;

    fn json_schema(
        generator: &mut SchemaGenerator,
        _: &SettingsJsonSchemaParams,
        _: &AppContext,
    ) -> RootSchema {
        generator.root_schema_for::<Self::FileContent>()
    }

    fn json_merge(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
    ) -> Result<Self::FileContent> {
        let mut merged = serde_json::Value::Null;
        for value in [default_value].iter().chain(user_values) {
            merge_non_null_json_value_into(serde_json::to_value(value).unwrap(), &mut merged);
        }
        Ok(serde_json::from_value(merged)?)
    }

    fn load_via_json_merge(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
    ) -> Result<Self>
    where
        Self: DeserializeOwned,
    {
        let mut merged = serde_json::Value::Null;
        for value in [default_value].iter().chain(user_values) {
            merge_non_null_json_value_into(serde_json::to_value(value).unwrap(), &mut merged);
        }
        Ok(serde_json::from_value(merged)?)
    }

    fn missing_default() -> anyhow::Error {
        anyhow::anyhow!("missing default")
    }

    fn register(cx: &mut AppContext)
    where
        Self: Sized,
    {
        cx.update_global(|store: &mut SettingsStore, cx| {
            store.register_setting::<Self>(cx);
        });
    }

    /// path is a (worktree ID, Path)
    #[track_caller]
    fn get<'a>(path: Option<(usize, &Path)>, cx: &'a AppContext) -> &'a Self
    where
        Self: Sized,
    {
        cx.global::<SettingsStore>().get(path)
    }

    #[track_caller]
    fn get_global(cx: &AppContext) -> &Self
    where
        Self: Sized,
    {
        cx.global::<SettingsStore>().get(None)
    }

    #[track_caller]
    fn try_read_global<R>(cx: &AsyncAppContext, f: impl FnOnce(&Self) -> R) -> Option<R>
    where
        Self: Sized,
    {
        cx.try_read_global(|s: &SettingsStore, _| f(s.get(None)))
    }

    #[track_caller]
    fn override_global(settings: Self, cx: &mut AppContext)
    where
        Self: Sized,
    {
        cx.global_mut::<SettingsStore>().override_global(settings)
    }
}

pub struct SettingsJsonSchemaParams<'a> {
    pub staff_mode: bool,
    pub language_names: &'a [String],
    pub font_names: &'a [String],
}

/// A set of strongly-typed setting values defined via multiple JSON files.
pub struct SettingsStore {
    setting_values: HashMap<TypeId, Box<dyn AnySettingValue>>,
    raw_default_settings: serde_json::Value,
    raw_user_settings: serde_json::Value,
    raw_local_settings: BTreeMap<(usize, Arc<Path>), serde_json::Value>,
    tab_size_callback: Option<(
        TypeId,
        Box<dyn Fn(&dyn Any) -> Option<usize> + Send + Sync + 'static>,
    )>,
}

impl Global for SettingsStore {}

impl Default for SettingsStore {
    fn default() -> Self {
        SettingsStore {
            setting_values: Default::default(),
            raw_default_settings: serde_json::json!({}),
            raw_user_settings: serde_json::json!({}),
            raw_local_settings: Default::default(),
            tab_size_callback: Default::default(),
        }
    }
}

#[derive(Debug)]
struct SettingValue<T> {
    global_value: Option<T>,
    local_values: Vec<(usize, Arc<Path>, T)>,
}

trait AnySettingValue: 'static + Send + Sync {
    fn key(&self) -> Option<&'static str>;
    fn setting_type_name(&self) -> &'static str;
    fn deserialize_setting(&self, json: &serde_json::Value) -> Result<DeserializedSetting>;
    fn load_setting(
        &self,
        default_value: &DeserializedSetting,
        custom: &[DeserializedSetting],
        cx: &mut AppContext,
    ) -> Result<Box<dyn Any>>;
    fn value_for_path(&self, path: Option<(usize, &Path)>) -> &dyn Any;
    fn set_global_value(&mut self, value: Box<dyn Any>);
    fn set_local_value(&mut self, root_id: usize, path: Arc<Path>, value: Box<dyn Any>);
    fn json_schema(
        &self,
        generator: &mut SchemaGenerator,
        _: &SettingsJsonSchemaParams,
        cx: &AppContext,
    ) -> RootSchema;
}

struct DeserializedSetting(Box<dyn Any>);

impl SettingsStore {
    /// Add a new type of setting to the store.
    pub fn register_setting<T: Settings>(&mut self, cx: &mut AppContext) {
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
            let mut user_values_stack = Vec::new();

            if let Some(user_settings) = setting_value
                .deserialize_setting(&self.raw_user_settings)
                .log_err()
            {
                user_values_stack = vec![user_settings];
            }

            if let Some(release_settings) = &self
                .raw_user_settings
                .get(release_channel::RELEASE_CHANNEL.dev_name())
            {
                if let Some(release_settings) = setting_value
                    .deserialize_setting(release_settings)
                    .log_err()
                {
                    user_values_stack.push(release_settings);
                }
            }

            if let Some(setting) = setting_value
                .load_setting(&default_settings, &user_values_stack, cx)
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
    pub fn get<T: Settings>(&self, path: Option<(usize, &Path)>) -> &T {
        self.setting_values
            .get(&TypeId::of::<T>())
            .unwrap_or_else(|| panic!("unregistered setting type {}", type_name::<T>()))
            .value_for_path(path)
            .downcast_ref::<T>()
            .expect("no default value for setting type")
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
    /// This is only for debugging and reporting. For user-facing functionality,
    /// use the typed setting interface.
    pub fn raw_user_settings(&self) -> &serde_json::Value {
        &self.raw_user_settings
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test(cx: &mut AppContext) -> Self {
        let mut this = Self::default();
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
        cx: &mut AppContext,
        update: impl FnOnce(&mut T::FileContent),
    ) {
        let old_text = serde_json::to_string(&self.raw_user_settings).unwrap();
        let new_text = self.new_text_for_update::<T>(old_text, update);
        self.set_user_settings(&new_text, cx).unwrap();
    }

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

    /// Updates the value of a setting in a JSON file, returning a list
    /// of edits to apply to the JSON file.
    pub fn edits_for_update<T: Settings>(
        &self,
        text: &str,
        update: impl FnOnce(&mut T::FileContent),
    ) -> Vec<(Range<usize>, String)> {
        let setting_type_id = TypeId::of::<T>();

        let setting = self
            .setting_values
            .get(&setting_type_id)
            .unwrap_or_else(|| panic!("unregistered setting type {}", type_name::<T>()));
        let raw_settings = parse_json_with_comments::<serde_json::Value>(text).unwrap_or_default();
        let old_content = match setting.deserialize_setting(&raw_settings) {
            Ok(content) => content.0.downcast::<T::FileContent>().unwrap(),
            Err(_) => Box::<<T as Settings>::FileContent>::default(),
        };
        let mut new_content = old_content.clone();
        update(&mut new_content);

        let old_value = serde_json::to_value(&old_content).unwrap();
        let new_value = serde_json::to_value(new_content).unwrap();

        let mut key_path = Vec::new();
        if let Some(key) = T::KEY {
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

    fn json_tab_size(&self) -> usize {
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
        cx: &mut AppContext,
    ) -> Result<()> {
        let settings: serde_json::Value = parse_json_with_comments(default_settings_content)?;
        if settings.is_object() {
            self.raw_default_settings = settings;
            self.recompute_values(None, cx)?;
            Ok(())
        } else {
            Err(anyhow!("settings must be an object"))
        }
    }

    /// Sets the user settings via a JSON string.
    pub fn set_user_settings(
        &mut self,
        user_settings_content: &str,
        cx: &mut AppContext,
    ) -> Result<()> {
        let settings: serde_json::Value = parse_json_with_comments(user_settings_content)?;
        if settings.is_object() {
            self.raw_user_settings = settings;
            self.recompute_values(None, cx)?;
            Ok(())
        } else {
            Err(anyhow!("settings must be an object"))
        }
    }

    /// Add or remove a set of local settings via a JSON string.
    pub fn set_local_settings(
        &mut self,
        root_id: usize,
        path: Arc<Path>,
        settings_content: Option<&str>,
        cx: &mut AppContext,
    ) -> Result<()> {
        if let Some(content) = settings_content {
            self.raw_local_settings
                .insert((root_id, path.clone()), parse_json_with_comments(content)?);
        } else {
            self.raw_local_settings.remove(&(root_id, path.clone()));
        }
        self.recompute_values(Some((root_id, &path)), cx)?;
        Ok(())
    }

    /// Add or remove a set of local settings via a JSON string.
    pub fn clear_local_settings(&mut self, root_id: usize, cx: &mut AppContext) -> Result<()> {
        self.raw_local_settings.retain(|k, _| k.0 != root_id);
        self.recompute_values(Some((root_id, "".as_ref())), cx)?;
        Ok(())
    }

    pub fn local_settings(&self, root_id: usize) -> impl '_ + Iterator<Item = (Arc<Path>, String)> {
        self.raw_local_settings
            .range((root_id, Path::new("").into())..(root_id + 1, Path::new("").into()))
            .map(|((_, path), content)| (path.clone(), serde_json::to_string(content).unwrap()))
    }

    pub fn json_schema(
        &self,
        schema_params: &SettingsJsonSchemaParams,
        cx: &AppContext,
    ) -> serde_json::Value {
        use schemars::{
            gen::SchemaSettings,
            schema::{Schema, SchemaObject},
        };

        let settings = SchemaSettings::draft07().with(|settings| {
            settings.option_add_null_type = false;
        });
        let mut generator = SchemaGenerator::new(settings);
        let mut combined_schema = RootSchema::default();

        for setting_value in self.setting_values.values() {
            let setting_schema = setting_value.json_schema(&mut generator, schema_params, cx);
            combined_schema
                .definitions
                .extend(setting_schema.definitions);

            let target_schema = if let Some(key) = setting_value.key() {
                let key_schema = combined_schema
                    .schema
                    .object()
                    .properties
                    .entry(key.to_string())
                    .or_insert_with(|| Schema::Object(SchemaObject::default()));
                if let Schema::Object(key_schema) = key_schema {
                    key_schema
                } else {
                    continue;
                }
            } else {
                &mut combined_schema.schema
            };

            merge_schema(target_schema, setting_schema.schema);
        }

        fn merge_schema(target: &mut SchemaObject, source: SchemaObject) {
            if let Some(source) = source.object {
                let target_properties = &mut target.object().properties;
                for (key, value) in source.properties {
                    match target_properties.entry(key) {
                        btree_map::Entry::Vacant(e) => {
                            e.insert(value);
                        }
                        btree_map::Entry::Occupied(e) => {
                            if let (Schema::Object(target), Schema::Object(src)) =
                                (e.into_mut(), value)
                            {
                                merge_schema(target, src);
                            }
                        }
                    }
                }
            }

            overwrite(&mut target.instance_type, source.instance_type);
            overwrite(&mut target.string, source.string);
            overwrite(&mut target.number, source.number);
            overwrite(&mut target.reference, source.reference);
            overwrite(&mut target.array, source.array);
            overwrite(&mut target.enum_values, source.enum_values);

            fn overwrite<T>(target: &mut Option<T>, source: Option<T>) {
                if let Some(source) = source {
                    *target = Some(source);
                }
            }
        }

        for release_stage in ["dev", "nightly", "stable", "preview"] {
            let schema = combined_schema.schema.clone();
            combined_schema
                .schema
                .object()
                .properties
                .insert(release_stage.to_string(), schema.into());
        }

        serde_json::to_value(&combined_schema).unwrap()
    }

    fn recompute_values(
        &mut self,
        changed_local_path: Option<(usize, &Path)>,
        cx: &mut AppContext,
    ) -> Result<()> {
        // Reload the global and local values for every setting.
        let mut user_settings_stack = Vec::<DeserializedSetting>::new();
        let mut paths_stack = Vec::<Option<(usize, &Path)>>::new();
        for setting_value in self.setting_values.values_mut() {
            let default_settings = setting_value.deserialize_setting(&self.raw_default_settings)?;

            user_settings_stack.clear();
            paths_stack.clear();

            if let Some(user_settings) = setting_value
                .deserialize_setting(&self.raw_user_settings)
                .log_err()
            {
                user_settings_stack.push(user_settings);
                paths_stack.push(None);
            }

            if let Some(release_settings) = &self
                .raw_user_settings
                .get(release_channel::RELEASE_CHANNEL.dev_name())
            {
                if let Some(release_settings) = setting_value
                    .deserialize_setting(release_settings)
                    .log_err()
                {
                    user_settings_stack.push(release_settings);
                    paths_stack.push(None);
                }
            }

            // If the global settings file changed, reload the global value for the field.
            if changed_local_path.is_none() {
                if let Some(value) = setting_value
                    .load_setting(&default_settings, &user_settings_stack, cx)
                    .log_err()
                {
                    setting_value.set_global_value(value);
                }
            }

            // Reload the local values for the setting.
            for ((root_id, path), local_settings) in &self.raw_local_settings {
                // Build a stack of all of the local values for that setting.
                while let Some(prev_entry) = paths_stack.last() {
                    if let Some((prev_root_id, prev_path)) = prev_entry {
                        if root_id != prev_root_id || !path.starts_with(prev_path) {
                            paths_stack.pop();
                            user_settings_stack.pop();
                            continue;
                        }
                    }
                    break;
                }

                if let Some(local_settings) =
                    setting_value.deserialize_setting(local_settings).log_err()
                {
                    paths_stack.push(Some((*root_id, path.as_ref())));
                    user_settings_stack.push(local_settings);

                    // If a local settings file changed, then avoid recomputing local
                    // settings for any path outside of that directory.
                    if changed_local_path.map_or(false, |(changed_root_id, changed_local_path)| {
                        *root_id != changed_root_id || !path.starts_with(changed_local_path)
                    }) {
                        continue;
                    }

                    if let Some(value) = setting_value
                        .load_setting(&default_settings, &user_settings_stack, cx)
                        .log_err()
                    {
                        setting_value.set_local_value(*root_id, path.clone(), value);
                    }
                }
            }
        }
        Ok(())
    }
}

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
        default_value: &DeserializedSetting,
        user_values: &[DeserializedSetting],
        cx: &mut AppContext,
    ) -> Result<Box<dyn Any>> {
        let default_value = default_value.0.downcast_ref::<T::FileContent>().unwrap();
        let values: SmallVec<[&T::FileContent; 6]> = user_values
            .iter()
            .map(|value| value.0.downcast_ref().unwrap())
            .collect();
        Ok(Box::new(T::load(default_value, &values, cx)?))
    }

    fn deserialize_setting(&self, mut json: &serde_json::Value) -> Result<DeserializedSetting> {
        if let Some(key) = T::KEY {
            if let Some(value) = json.get(key) {
                json = value;
            } else {
                let value = T::FileContent::default();
                return Ok(DeserializedSetting(Box::new(value)));
            }
        }
        let value = T::FileContent::deserialize(json)?;
        Ok(DeserializedSetting(Box::new(value)))
    }

    fn value_for_path(&self, path: Option<(usize, &Path)>) -> &dyn Any {
        if let Some((root_id, path)) = path {
            for (settings_root_id, settings_path, value) in self.local_values.iter().rev() {
                if root_id == *settings_root_id && path.starts_with(settings_path) {
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

    fn set_local_value(&mut self, root_id: usize, path: Arc<Path>, value: Box<dyn Any>) {
        let value = *value.downcast().unwrap();
        match self
            .local_values
            .binary_search_by_key(&(root_id, &path), |e| (e.0, &e.1))
        {
            Ok(ix) => self.local_values[ix].2 = value,
            Err(ix) => self.local_values.insert(ix, (root_id, path, value)),
        }
    }

    fn json_schema(
        &self,
        generator: &mut SchemaGenerator,
        params: &SettingsJsonSchemaParams,
        cx: &AppContext,
    ) -> RootSchema {
        T::json_schema(generator, params, cx)
    }
}

fn update_value_in_json_text<'a>(
    text: &mut String,
    key_path: &mut Vec<&'a str>,
    tab_size: usize,
    old_value: &'a serde_json::Value,
    new_value: &'a serde_json::Value,
    edits: &mut Vec<(Range<usize>, String)>,
) {
    // If the old and new values are both objects, then compare them key by key,
    // preserving the comments and formatting of the unchanged parts. Otherwise,
    // replace the old value with the new value.
    if let (serde_json::Value::Object(old_object), serde_json::Value::Object(new_object)) =
        (old_value, new_value)
    {
        for (key, old_sub_value) in old_object.iter() {
            key_path.push(key);
            let new_sub_value = new_object.get(key).unwrap_or(&serde_json::Value::Null);
            update_value_in_json_text(
                text,
                key_path,
                tab_size,
                old_sub_value,
                new_sub_value,
                edits,
            );
            key_path.pop();
        }
        for (key, new_sub_value) in new_object.iter() {
            key_path.push(key);
            if !old_object.contains_key(key) {
                update_value_in_json_text(
                    text,
                    key_path,
                    tab_size,
                    &serde_json::Value::Null,
                    new_sub_value,
                    edits,
                );
            }
            key_path.pop();
        }
    } else if old_value != new_value {
        let mut new_value = new_value.clone();
        if let Some(new_object) = new_value.as_object_mut() {
            new_object.retain(|_, v| !v.is_null());
        }
        let (range, replacement) = replace_value_in_json_text(text, key_path, tab_size, &new_value);
        text.replace_range(range.clone(), &replacement);
        edits.push((range, replacement));
    }
}

fn replace_value_in_json_text(
    text: &str,
    key_path: &[&str],
    tab_size: usize,
    new_value: &serde_json::Value,
) -> (Range<usize>, String) {
    const LANGUAGE_OVERRIDES: &str = "language_overrides";
    const LANGUAGES: &str = "languages";

    lazy_static! {
        static ref PAIR_QUERY: tree_sitter::Query = tree_sitter::Query::new(
            &tree_sitter_json::language(),
            "(pair key: (string) @key value: (_) @value)",
        )
        .unwrap();
    }

    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_json::language()).unwrap();
    let syntax_tree = parser.parse(text, None).unwrap();

    let mut cursor = tree_sitter::QueryCursor::new();

    let has_language_overrides = text.contains(LANGUAGE_OVERRIDES);

    let mut depth = 0;
    let mut last_value_range = 0..0;
    let mut first_key_start = None;
    let mut existing_value_range = 0..text.len();
    let matches = cursor.matches(&PAIR_QUERY, syntax_tree.root_node(), text.as_bytes());
    for mat in matches {
        if mat.captures.len() != 2 {
            continue;
        }

        let key_range = mat.captures[0].node.byte_range();
        let value_range = mat.captures[1].node.byte_range();

        // Don't enter sub objects until we find an exact
        // match for the current keypath
        if last_value_range.contains_inclusive(&value_range) {
            continue;
        }

        last_value_range = value_range.clone();

        if key_range.start > existing_value_range.end {
            break;
        }

        first_key_start.get_or_insert(key_range.start);

        let found_key = text
            .get(key_range.clone())
            .map(|key_text| {
                if key_path[depth] == LANGUAGES && has_language_overrides {
                    key_text == format!("\"{}\"", LANGUAGE_OVERRIDES)
                } else {
                    key_text == format!("\"{}\"", key_path[depth])
                }
            })
            .unwrap_or(false);

        if found_key {
            existing_value_range = value_range;
            // Reset last value range when increasing in depth
            last_value_range = existing_value_range.start..existing_value_range.start;
            depth += 1;

            if depth == key_path.len() {
                break;
            }

            first_key_start = None;
        }
    }

    // We found the exact key we want, insert the new value
    if depth == key_path.len() {
        let new_val = to_pretty_json(&new_value, tab_size, tab_size * depth);
        (existing_value_range, new_val)
    } else {
        // We have key paths, construct the sub objects
        let new_key = if has_language_overrides && key_path[depth] == LANGUAGES {
            LANGUAGE_OVERRIDES
        } else {
            key_path[depth]
        };

        // We don't have the key, construct the nested objects
        let mut new_value = serde_json::to_value(new_value).unwrap();
        for key in key_path[(depth + 1)..].iter().rev() {
            if has_language_overrides && key == &LANGUAGES {
                new_value = serde_json::json!({ LANGUAGE_OVERRIDES.to_string(): new_value });
            } else {
                new_value = serde_json::json!({ key.to_string(): new_value });
            }
        }

        if let Some(first_key_start) = first_key_start {
            let mut row = 0;
            let mut column = 0;
            for (ix, char) in text.char_indices() {
                if ix == first_key_start {
                    break;
                }
                if char == '\n' {
                    row += 1;
                    column = 0;
                } else {
                    column += char.len_utf8();
                }
            }

            if row > 0 {
                // depth is 0 based, but division needs to be 1 based.
                let new_val = to_pretty_json(&new_value, column / (depth + 1), column);
                let space = ' ';
                let content = format!("\"{new_key}\": {new_val},\n{space:width$}", width = column);
                (first_key_start..first_key_start, content)
            } else {
                let new_val = serde_json::to_string(&new_value).unwrap();
                let mut content = format!(r#""{new_key}": {new_val},"#);
                content.push(' ');
                (first_key_start..first_key_start, content)
            }
        } else {
            new_value = serde_json::json!({ new_key.to_string(): new_value });
            let indent_prefix_len = 4 * depth;
            let mut new_val = to_pretty_json(&new_value, 4, indent_prefix_len);
            if depth == 0 {
                new_val.push('\n');
            }

            (existing_value_range, new_val)
        }
    }
}

fn to_pretty_json(value: &impl Serialize, indent_size: usize, indent_prefix_len: usize) -> String {
    const SPACES: [u8; 32] = [b' '; 32];

    debug_assert!(indent_size <= SPACES.len());
    debug_assert!(indent_prefix_len <= SPACES.len());

    let mut output = Vec::new();
    let mut ser = serde_json::Serializer::with_formatter(
        &mut output,
        serde_json::ser::PrettyFormatter::with_indent(&SPACES[0..indent_size.min(SPACES.len())]),
    );

    value.serialize(&mut ser).unwrap();
    let text = String::from_utf8(output).unwrap();

    let mut adjusted_text = String::new();
    for (i, line) in text.split('\n').enumerate() {
        if i > 0 {
            adjusted_text.push_str(str::from_utf8(&SPACES[0..indent_prefix_len]).unwrap());
        }
        adjusted_text.push_str(line);
        adjusted_text.push('\n');
    }
    adjusted_text.pop();
    adjusted_text
}

pub fn parse_json_with_comments<T: DeserializeOwned>(content: &str) -> Result<T> {
    Ok(serde_json_lenient::from_str(content)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_derive::Deserialize;
    use unindent::Unindent;

    #[gpui::test]
    fn test_settings_store_basic(cx: &mut AppContext) {
        let mut store = SettingsStore::default();
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
                1,
                Path::new("/root1").into(),
                Some(r#"{ "user": { "staff": true } }"#),
                cx,
            )
            .unwrap();
        store
            .set_local_settings(
                1,
                Path::new("/root1/subdir").into(),
                Some(r#"{ "user": { "name": "Jane Doe" } }"#),
                cx,
            )
            .unwrap();

        store
            .set_local_settings(
                1,
                Path::new("/root2").into(),
                Some(r#"{ "user": { "age": 42 }, "key2": "b" }"#),
                cx,
            )
            .unwrap();

        assert_eq!(
            store.get::<UserSettings>(Some((1, Path::new("/root1/something")))),
            &UserSettings {
                name: "John Doe".to_string(),
                age: 31,
                staff: true
            }
        );
        assert_eq!(
            store.get::<UserSettings>(Some((1, Path::new("/root1/subdir/something")))),
            &UserSettings {
                name: "Jane Doe".to_string(),
                age: 31,
                staff: true
            }
        );
        assert_eq!(
            store.get::<UserSettings>(Some((1, Path::new("/root2/something")))),
            &UserSettings {
                name: "John Doe".to_string(),
                age: 42,
                staff: false
            }
        );
        assert_eq!(
            store.get::<MultiKeySettings>(Some((1, Path::new("/root2/something")))),
            &MultiKeySettings {
                key1: "a".to_string(),
                key2: "b".to_string(),
            }
        );
    }

    #[gpui::test]
    fn test_setting_store_assign_json_before_register(cx: &mut AppContext) {
        let mut store = SettingsStore::default();
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

    #[gpui::test]
    fn test_setting_store_update(cx: &mut AppContext) {
        let mut store = SettingsStore::default();
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

    fn check_settings_update<T: Settings>(
        store: &mut SettingsStore,
        old_json: String,
        update: fn(&mut T::FileContent),
        expected_new_json: String,
        cx: &mut AppContext,
    ) {
        store.set_user_settings(&old_json, cx).ok();
        let edits = store.edits_for_update::<T>(&old_json, update);
        let mut new_json = old_json;
        for (range, replacement) in edits.into_iter() {
            new_json.replace_range(range, &replacement);
        }
        pretty_assertions::assert_eq!(new_json, expected_new_json);
    }

    #[derive(Debug, PartialEq, Deserialize)]
    struct UserSettings {
        name: String,
        age: u32,
        staff: bool,
    }

    #[derive(Default, Clone, Serialize, Deserialize, JsonSchema)]
    struct UserSettingsJson {
        name: Option<String>,
        age: Option<u32>,
        staff: Option<bool>,
    }

    impl Settings for UserSettings {
        const KEY: Option<&'static str> = Some("user");
        type FileContent = UserSettingsJson;

        fn load(
            default_value: &UserSettingsJson,
            user_values: &[&UserSettingsJson],
            _: &mut AppContext,
        ) -> Result<Self> {
            Self::load_via_json_merge(default_value, user_values)
        }
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct TurboSetting(bool);

    impl Settings for TurboSetting {
        const KEY: Option<&'static str> = Some("turbo");
        type FileContent = Option<bool>;

        fn load(
            default_value: &Option<bool>,
            user_values: &[&Option<bool>],
            _: &mut AppContext,
        ) -> Result<Self> {
            Self::load_via_json_merge(default_value, user_values)
        }
    }

    #[derive(Clone, Debug, PartialEq, Deserialize)]
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

        fn load(
            default_value: &MultiKeySettingsJson,
            user_values: &[&MultiKeySettingsJson],
            _: &mut AppContext,
        ) -> Result<Self> {
            Self::load_via_json_merge(default_value, user_values)
        }
    }

    #[derive(Debug, Deserialize)]
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

        fn load(
            default_value: &JournalSettingsJson,
            user_values: &[&JournalSettingsJson],
            _: &mut AppContext,
        ) -> Result<Self> {
            Self::load_via_json_merge(default_value, user_values)
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
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

        fn load(default_value: &Self, user_values: &[&Self], _: &mut AppContext) -> Result<Self> {
            Self::load_via_json_merge(default_value, user_values)
        }
    }
}
