use anyhow::{Context as _, Result};
use collections::{BTreeMap, HashMap, IndexMap};
use fs::Fs;
use gpui::{
    Action, ActionBuildError, App, InvalidKeystrokeError, KEYSTROKE_PARSE_EXPECTED_MESSAGE,
    KeyBinding, KeyBindingContextPredicate, KeyBindingMetaIndex, KeybindingKeystroke, Keystroke,
    NoAction, SharedString, Unbind, generate_list_of_all_registered_actions, register_action,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeymapEntryCollection {
    Bindings,
    Unbind,
}

impl KeymapEntryCollection {
    fn as_kind(self) -> u8 {
        match self {
            KeymapEntryCollection::Bindings => 0,
            KeymapEntryCollection::Unbind => 1,
        }
    }

    fn from_kind(kind: u8) -> Option<Self> {
        match kind {
            0 => Some(KeymapEntryCollection::Bindings),
            1 => Some(KeymapEntryCollection::Unbind),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeymapEntryLocation {
    pub section_index: usize,
    pub collection: KeymapEntryCollection,
    pub entry_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeymapRevision(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeymapEntryProvenance {
    pub location: KeymapEntryLocation,
    pub revision: KeymapRevision,
}

impl KeymapEntryProvenance {
    pub fn to_gpui(self) -> gpui::KeyBindingProvenance {
        gpui::KeyBindingProvenance {
            group: self.location.section_index,
            kind: self.location.collection.as_kind(),
            index: self.location.entry_index,
            revision: self.revision.0,
        }
    }

    pub fn from_gpui(provenance: gpui::KeyBindingProvenance) -> Option<Self> {
        let collection = KeymapEntryCollection::from_kind(provenance.kind)?;
        Some(KeymapEntryProvenance {
            location: KeymapEntryLocation {
                section_index: provenance.group,
                collection,
                entry_index: provenance.index,
            },
            revision: KeymapRevision(provenance.revision),
        })
    }
}

fn keymap_fingerprint(keymap: &KeymapFile) -> KeymapRevision {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    keymap.0.len().hash(&mut hasher);
    for section in keymap.0.iter() {
        section.context.hash(&mut hasher);
        section.use_key_equivalents.hash(&mut hasher);
        if let Some(unbind) = section.unbind.as_ref() {
            unbind.len().hash(&mut hasher);
            for (key, action) in unbind.iter() {
                key.hash(&mut hasher);
                action.0.to_string().hash(&mut hasher);
            }
        } else {
            0usize.hash(&mut hasher);
        }
        if let Some(bindings) = section.bindings.as_ref() {
            bindings.len().hash(&mut hasher);
            for (key, action) in bindings.iter() {
                key.hash(&mut hasher);
                action.0.to_string().hash(&mut hasher);
            }
        } else {
            0usize.hash(&mut hasher);
        }
        section.unrecognized_fields.len().hash(&mut hasher);
        for (key, value) in section.unrecognized_fields.iter() {
            key.hash(&mut hasher);
            value.to_string().hash(&mut hasher);
        }
    }
    KeymapRevision(hasher.finish())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SuppressingEntry {
    pub source: KeybindSource,
    pub provenance: Option<KeymapEntryProvenance>,
}

fn keystrokes_match_exactly(
    keystrokes1: &[KeybindingKeystroke],
    keystrokes2: &[KeybindingKeystroke],
) -> bool {
    keystrokes1.len() == keystrokes2.len()
        && keystrokes1.iter().zip(keystrokes2).all(|(k1, k2)| {
            k1.inner().key == k2.inner().key && k1.inner().modifiers == k2.inner().modifiers
        })
}

fn disabled_binding_matches_context(disabled_binding: &KeyBinding, binding: &KeyBinding) -> bool {
    match (
        disabled_binding.predicate().as_deref(),
        binding.predicate().as_deref(),
    ) {
        (None, _) => true,
        (Some(_), None) => false,
        (Some(disabled_predicate), Some(predicate)) => disabled_predicate.is_superset(predicate),
    }
}

pub fn find_suppressing_entries(
    binding: &KeyBinding,
    binding_index: usize,
    all_bindings: &[&KeyBinding],
) -> Vec<SuppressingEntry> {
    all_bindings[binding_index + 1..]
        .iter()
        .filter_map(|disabled_binding| {
            let matches = gpui::is_unbind(disabled_binding.action())
                && keystrokes_match_exactly(disabled_binding.keystrokes(), binding.keystrokes())
                && disabled_binding
                    .action()
                    .as_any()
                    .downcast_ref::<gpui::Unbind>()
                    .is_some_and(|unbind| unbind.0.as_ref() == binding.action().name())
                && disabled_binding_matches_context(disabled_binding, binding);
            if !matches {
                return None;
            }
            let source = disabled_binding
                .meta()
                .map(KeybindSource::from_meta)
                .unwrap_or(KeybindSource::Unknown);
            Some(SuppressingEntry {
                source,
                provenance: disabled_binding
                    .provenance()
                    .and_then(KeymapEntryProvenance::from_gpui),
            })
        })
        .collect()
}
use schemars::{JsonSchema, json_schema};
use serde::Deserialize;
use serde_json::{Value, json};
use std::borrow::Cow;
use std::{any::TypeId, fmt::Write, rc::Rc, sync::Arc, sync::LazyLock};
use util::ResultExt as _;
use util::{
    asset_str,
    markdown::{MarkdownEscaped, MarkdownInlineCode, MarkdownString},
    schemars::AllowTrailingCommas,
};

use crate::SettingsAssets;
use settings_content::{ActionName, ActionWithArguments};
use settings_json::{
    append_top_level_array_value_in_json_text, parse_json_with_comments,
    replace_top_level_array_value_in_json_text,
};

pub trait KeyBindingValidator: Send + Sync {
    fn action_type_id(&self) -> TypeId;
    fn validate(&self, binding: &KeyBinding) -> Result<(), MarkdownString>;
}

pub struct KeyBindingValidatorRegistration(pub fn() -> Box<dyn KeyBindingValidator>);

inventory::collect!(KeyBindingValidatorRegistration);

pub(crate) static KEY_BINDING_VALIDATORS: LazyLock<BTreeMap<TypeId, Box<dyn KeyBindingValidator>>> =
    LazyLock::new(|| {
        let mut validators = BTreeMap::new();
        for validator_registration in inventory::iter::<KeyBindingValidatorRegistration> {
            let validator = validator_registration.0();
            validators.insert(validator.action_type_id(), validator);
        }
        validators
    });

// Note that the doc comments on these are shown by json-language-server when editing the keymap, so
// they should be considered user-facing documentation. Documentation is not handled well with
// schemars-0.8 - when there are newlines, it is rendered as plaintext (see
// https://github.com/GREsau/schemars/issues/38#issuecomment-2282883519). So for now these docs
// avoid newlines.
//
// TODO: Update to schemars-1.0 once it's released, and add more docs as newlines would be
// supported. Tracking issue is https://github.com/GREsau/schemars/issues/112.

/// Keymap configuration consisting of sections. Each section may have a context predicate which
/// determines whether its bindings are used.
#[derive(Debug, Deserialize, Default, Clone, JsonSchema)]
#[serde(transparent)]
pub struct KeymapFile(Vec<KeymapSection>);

/// Keymap section which binds keystrokes to actions.
#[derive(Debug, Deserialize, Default, Clone, JsonSchema)]
pub struct KeymapSection {
    /// Determines when these bindings are active. When just a name is provided, like `Editor` or
    /// `Workspace`, the bindings will be active in that context. Boolean expressions like `X && Y`,
    /// `X || Y`, `!X` are also supported. Some more complex logic including checking OS and the
    /// current file extension are also supported - see [the
    /// documentation](https://zed.dev/docs/key-bindings#contexts) for more details.
    #[serde(default)]
    pub context: String,
    /// This option enables specifying keys based on their position on a QWERTY keyboard, by using
    /// position-equivalent mappings for some non-QWERTY keyboards. This is currently only supported
    /// on macOS. See the documentation for more details.
    #[serde(default)]
    use_key_equivalents: bool,
    /// This keymap section's unbindings, as a JSON object mapping keystrokes to actions. These are
    /// parsed before `bindings`, so bindings later in the same section can still take precedence.
    #[serde(default)]
    unbind: Option<IndexMap<String, UnbindTargetAction>>,
    /// This keymap section's bindings, as a JSON object mapping keystrokes to actions. The
    /// keystrokes key is a string representing a sequence of keystrokes to type, where the
    /// keystrokes are separated by whitespace. Each keystroke is a sequence of modifiers (`ctrl`,
    /// `alt`, `shift`, `fn`, `cmd`, `super`, or `win`) followed by a key, separated by `-`. The
    /// order of bindings does matter. When the same keystrokes are bound at the same context depth,
    /// the binding that occurs later in the file is preferred. For displaying keystrokes in the UI,
    /// the later binding for the same action is preferred.
    #[serde(default)]
    bindings: Option<IndexMap<String, KeymapAction>>,
    #[serde(flatten)]
    unrecognized_fields: IndexMap<String, Value>,
    // This struct intentionally uses permissive types for its fields, rather than validating during
    // deserialization. The purpose of this is to allow loading the portion of the keymap that doesn't
    // have errors. The downside of this is that the errors are not reported with line+column info.
    // Unfortunately the implementations of the `Spanned` types for preserving this information are
    // highly inconvenient (`serde_spanned`) and in some cases don't work at all here
    // (`json_spanned_>value`). Serde should really have builtin support for this.
}

impl KeymapSection {
    pub fn bindings(&self) -> impl DoubleEndedIterator<Item = (&String, &KeymapAction)> {
        self.bindings.iter().flatten()
    }
}

/// Keymap action as a JSON value, since it can either be null for no action, or the name of the
/// action, or an array of the name of the action and the action input.
///
/// Unlike the other json types involved in keymaps (including actions), this doc-comment will not
/// be included in the generated JSON schema, as it manually defines its `JsonSchema` impl. The
/// actual schema used for it is automatically generated in `KeymapFile::generate_json_schema`.
#[derive(Debug, Deserialize, Default, Clone)]
#[serde(transparent)]
pub struct KeymapAction(Value);

impl std::fmt::Display for KeymapAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Value::String(s) => write!(f, "{}", s),
            Value::Array(arr) => {
                let strings: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                write!(f, "{}", strings.join(", "))
            }
            _ => write!(f, "{}", self.0),
        }
    }
}

impl JsonSchema for KeymapAction {
    /// This is used when generating the JSON schema for the `KeymapAction` type, so that it can
    /// reference the keymap action schema.
    fn schema_name() -> Cow<'static, str> {
        "KeymapAction".into()
    }

    /// This schema will be replaced with the full action schema in
    /// `KeymapFile::generate_json_schema`.
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!(true)
    }
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(transparent)]
pub struct UnbindTargetAction(Value);

impl JsonSchema for UnbindTargetAction {
    fn schema_name() -> Cow<'static, str> {
        "UnbindTargetAction".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!(true)
    }
}

#[derive(Debug)]
#[must_use]
pub enum KeymapFileLoadResult {
    Success {
        key_bindings: Vec<KeyBinding>,
    },
    SomeFailedToLoad {
        key_bindings: Vec<KeyBinding>,
        error_message: MarkdownString,
    },
    JsonParseFailure {
        error: anyhow::Error,
    },
}

impl KeymapFile {
    pub fn parse(content: &str) -> anyhow::Result<Self> {
        if content.trim().is_empty() {
            return Ok(Self(Vec::new()));
        }
        parse_json_with_comments::<Self>(content)
    }

    pub fn load_asset(
        asset_path: &str,
        source: Option<KeybindSource>,
        cx: &App,
    ) -> anyhow::Result<Vec<KeyBinding>> {
        match Self::load(asset_str::<SettingsAssets>(asset_path).as_ref(), cx) {
            KeymapFileLoadResult::Success { mut key_bindings } => match source {
                Some(source) => Ok({
                    for key_binding in &mut key_bindings {
                        key_binding.set_meta(source.meta());
                    }
                    key_bindings
                }),
                None => Ok(key_bindings),
            },
            KeymapFileLoadResult::SomeFailedToLoad { error_message, .. } => {
                anyhow::bail!("Error loading built-in keymap \"{asset_path}\": {error_message}",)
            }
            KeymapFileLoadResult::JsonParseFailure { error } => {
                anyhow::bail!("JSON parse error in built-in keymap \"{asset_path}\": {error}")
            }
        }
    }

    pub fn load_asset_allow_partial_failure(
        asset_path: &str,
        cx: &App,
    ) -> anyhow::Result<Vec<KeyBinding>> {
        match Self::load(asset_str::<SettingsAssets>(asset_path).as_ref(), cx) {
            KeymapFileLoadResult::SomeFailedToLoad {
                key_bindings,
                error_message,
                ..
            } if key_bindings.is_empty() => {
                anyhow::bail!("Error loading built-in keymap \"{asset_path}\": {error_message}",)
            }
            KeymapFileLoadResult::Success { key_bindings, .. }
            | KeymapFileLoadResult::SomeFailedToLoad { key_bindings, .. } => Ok(key_bindings),
            KeymapFileLoadResult::JsonParseFailure { error } => {
                anyhow::bail!("JSON parse error in built-in keymap \"{asset_path}\": {error}")
            }
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn load_asset_cached(asset_path: &str, cx: &App) -> anyhow::Result<Vec<KeyBinding>> {
        static CACHED: std::sync::OnceLock<KeymapFile> = std::sync::OnceLock::new();
        let keymap = CACHED
            .get_or_init(|| Self::parse(asset_str::<SettingsAssets>(asset_path).as_ref()).unwrap());
        match keymap.load_keymap(cx) {
            KeymapFileLoadResult::SomeFailedToLoad {
                key_bindings,
                error_message,
                ..
            } if key_bindings.is_empty() => {
                anyhow::bail!("Error loading built-in keymap \"{asset_path}\": {error_message}")
            }
            KeymapFileLoadResult::Success { key_bindings, .. }
            | KeymapFileLoadResult::SomeFailedToLoad { key_bindings, .. } => Ok(key_bindings),
            KeymapFileLoadResult::JsonParseFailure { error } => {
                anyhow::bail!("JSON parse error in built-in keymap \"{asset_path}\": {error}")
            }
        }
    }

    #[cfg(feature = "test-support")]
    pub fn load_panic_on_failure(content: &str, cx: &App) -> Vec<KeyBinding> {
        match Self::load(content, cx) {
            KeymapFileLoadResult::Success { key_bindings, .. } => key_bindings,
            KeymapFileLoadResult::SomeFailedToLoad { error_message, .. } => {
                panic!("{error_message}");
            }
            KeymapFileLoadResult::JsonParseFailure { error } => {
                panic!("JSON parse error: {error}");
            }
        }
    }

    pub fn load(content: &str, cx: &App) -> KeymapFileLoadResult {
        let keymap_file = match Self::parse(content) {
            Ok(keymap_file) => keymap_file,
            Err(error) => {
                return KeymapFileLoadResult::JsonParseFailure { error };
            }
        };
        keymap_file.load_keymap(cx)
    }

    pub fn load_keymap(&self, cx: &App) -> KeymapFileLoadResult {
        let (result, _) = self.load_keymap_with_locations(cx);
        result
    }

    pub fn load_keymap_with_locations(
        &self,
        cx: &App,
    ) -> (KeymapFileLoadResult, Vec<KeymapEntryProvenance>) {
        // Accumulate errors in order to support partial load of user keymap in the presence of
        // errors in context and binding parsing.
        let mut errors = Vec::new();
        let mut key_bindings = Vec::new();
        let mut provenances = Vec::new();
        let revision = keymap_fingerprint(self);

        for (
            section_index,
            KeymapSection {
                context,
                use_key_equivalents,
                unbind,
                bindings,
                unrecognized_fields,
            },
        ) in self.0.iter().enumerate()
        {
            let context_predicate: Option<Rc<KeyBindingContextPredicate>> = if context.is_empty() {
                None
            } else {
                match KeyBindingContextPredicate::parse(context) {
                    Ok(context_predicate) => Some(context_predicate.into()),
                    Err(err) => {
                        // Leading space is to separate from the message indicating which section
                        // the error occurred in.
                        errors.push((
                            context.clone(),
                            format!(" Parse error in section `context` field: {}", err),
                        ));
                        continue;
                    }
                }
            };

            let mut section_errors = String::new();

            if !unrecognized_fields.is_empty() {
                write!(
                    section_errors,
                    "\n\n - Unrecognized fields: {}",
                    MarkdownInlineCode(&format!("{:?}", unrecognized_fields.keys()))
                )
                .unwrap();
            }

            if let Some(unbind) = unbind {
                for (entry_index, (keystrokes, action)) in unbind.iter().enumerate() {
                    let result = Self::load_unbinding(
                        keystrokes,
                        action,
                        context_predicate.clone(),
                        *use_key_equivalents,
                        cx,
                    );
                    match result {
                        Ok(mut key_binding) => {
                            let provenance = KeymapEntryProvenance {
                                location: KeymapEntryLocation {
                                    section_index,
                                    collection: KeymapEntryCollection::Unbind,
                                    entry_index,
                                },
                                revision,
                            };
                            key_binding.set_provenance(provenance.to_gpui());
                            provenances.push(provenance);
                            key_bindings.push(key_binding);
                        }
                        Err(err) => {
                            let mut lines = err.lines();
                            let mut indented_err = lines.next().unwrap().to_string();
                            for line in lines {
                                indented_err.push_str("  ");
                                indented_err.push_str(line);
                                indented_err.push_str("\n");
                            }
                            write!(
                                section_errors,
                                "\n\n- In unbind {}, {indented_err}",
                                MarkdownInlineCode(&format!("\"{}\"", keystrokes))
                            )
                            .unwrap();
                        }
                    }
                }
            }

            if let Some(bindings) = bindings {
                for (entry_index, (keystrokes, action)) in bindings.iter().enumerate() {
                    let result = Self::load_keybinding(
                        keystrokes,
                        action,
                        context_predicate.clone(),
                        *use_key_equivalents,
                        cx,
                    );
                    match result {
                        Ok(mut key_binding) => {
                            let provenance = KeymapEntryProvenance {
                                location: KeymapEntryLocation {
                                    section_index,
                                    collection: KeymapEntryCollection::Bindings,
                                    entry_index,
                                },
                                revision,
                            };
                            key_binding.set_provenance(provenance.to_gpui());
                            provenances.push(provenance);
                            key_bindings.push(key_binding);
                        }
                        Err(err) => {
                            let mut lines = err.lines();
                            let mut indented_err = lines.next().unwrap().to_string();
                            for line in lines {
                                indented_err.push_str("  ");
                                indented_err.push_str(line);
                                indented_err.push_str("\n");
                            }
                            write!(
                                section_errors,
                                "\n\n- In binding {}, {indented_err}",
                                MarkdownInlineCode(&format!("\"{}\"", keystrokes))
                            )
                            .unwrap();
                        }
                    }
                }
            }

            if !section_errors.is_empty() {
                errors.push((context.clone(), section_errors))
            }
        }

        if errors.is_empty() {
            (KeymapFileLoadResult::Success { key_bindings }, provenances)
        } else {
            let mut error_message = "Errors in user keymap file.".to_owned();

            for (context, section_errors) in errors {
                if context.is_empty() {
                    let _ = write!(error_message, "\nIn section without context predicate:");
                } else {
                    let _ = write!(
                        error_message,
                        "\nIn section with {}:",
                        MarkdownInlineCode(&format!("context = \"{}\"", context))
                    );
                }
                let _ = write!(error_message, "{section_errors}");
            }

            (
                KeymapFileLoadResult::SomeFailedToLoad {
                    key_bindings,
                    error_message: MarkdownString(error_message),
                },
                provenances,
            )
        }
    }

    fn load_keybinding(
        keystrokes: &str,
        action: &KeymapAction,
        context: Option<Rc<KeyBindingContextPredicate>>,
        use_key_equivalents: bool,
        cx: &App,
    ) -> std::result::Result<KeyBinding, String> {
        Self::load_keybinding_action_value(keystrokes, &action.0, context, use_key_equivalents, cx)
    }

    fn load_keybinding_action_value(
        keystrokes: &str,
        action: &Value,
        context: Option<Rc<KeyBindingContextPredicate>>,
        use_key_equivalents: bool,
        cx: &App,
    ) -> std::result::Result<KeyBinding, String> {
        let (action, action_input_string) = Self::build_keymap_action_value(action, cx)?;

        let key_binding = match KeyBinding::load(
            keystrokes,
            action,
            context,
            use_key_equivalents,
            action_input_string.map(SharedString::from),
            cx.keyboard_mapper().as_ref(),
        ) {
            Ok(key_binding) => key_binding,
            Err(InvalidKeystrokeError { keystroke }) => {
                return Err(format!(
                    "invalid keystroke {}. {}",
                    MarkdownInlineCode(&format!("\"{keystroke}\"")),
                    KEYSTROKE_PARSE_EXPECTED_MESSAGE
                ));
            }
        };

        if let Some(validator) = KEY_BINDING_VALIDATORS.get(&key_binding.action().type_id()) {
            match validator.validate(&key_binding) {
                Ok(()) => Ok(key_binding),
                Err(error) => Err(error.0),
            }
        } else {
            Ok(key_binding)
        }
    }

    fn load_unbinding(
        keystrokes: &str,
        action: &UnbindTargetAction,
        context: Option<Rc<KeyBindingContextPredicate>>,
        use_key_equivalents: bool,
        cx: &App,
    ) -> std::result::Result<KeyBinding, String> {
        let key_binding = Self::load_keybinding_action_value(
            keystrokes,
            &action.0,
            context,
            use_key_equivalents,
            cx,
        )?;

        if key_binding.action().partial_eq(&NoAction) {
            return Err("expected action name string or [name, input] array.".to_string());
        }

        if key_binding.action().name() == Unbind::name_for_type() {
            return Err(format!(
                "can't use {} as an unbind target.",
                MarkdownInlineCode(&format!("\"{}\"", Unbind::name_for_type()))
            ));
        }

        KeyBinding::load(
            keystrokes,
            Box::new(Unbind(key_binding.action().name().into())),
            key_binding.predicate(),
            use_key_equivalents,
            key_binding.action_input(),
            cx.keyboard_mapper().as_ref(),
        )
        .map_err(|InvalidKeystrokeError { keystroke }| {
            format!(
                "invalid keystroke {}. {}",
                MarkdownInlineCode(&format!("\"{keystroke}\"")),
                KEYSTROKE_PARSE_EXPECTED_MESSAGE
            )
        })
    }

    pub fn parse_action(
        action: &KeymapAction,
    ) -> Result<Option<(&String, Option<&Value>)>, String> {
        Self::parse_action_value(&action.0)
    }

    fn parse_action_value(action: &Value) -> Result<Option<(&String, Option<&Value>)>, String> {
        let name_and_input = match action {
            Value::Array(items) => {
                if items.len() != 2 {
                    return Err(format!(
                        "expected two-element array of `[name, input]`. \
                        Instead found {}.",
                        MarkdownInlineCode(&action.to_string())
                    ));
                }
                let serde_json::Value::String(ref name) = items[0] else {
                    return Err(format!(
                        "expected two-element array of `[name, input]`, \
                        but the first element is not a string in {}.",
                        MarkdownInlineCode(&action.to_string())
                    ));
                };
                Some((name, Some(&items[1])))
            }
            Value::String(name) => Some((name, None)),
            Value::Null => None,
            _ => {
                return Err(format!(
                    "expected two-element array of `[name, input]`. \
                    Instead found {}.",
                    MarkdownInlineCode(&action.to_string())
                ));
            }
        };
        Ok(name_and_input)
    }

    fn build_keymap_action(
        action: &KeymapAction,
        cx: &App,
    ) -> std::result::Result<(Box<dyn Action>, Option<String>), String> {
        Self::build_keymap_action_value(&action.0, cx)
    }

    fn build_keymap_action_value(
        action: &Value,
        cx: &App,
    ) -> std::result::Result<(Box<dyn Action>, Option<String>), String> {
        let (build_result, action_input_string) = match Self::parse_action_value(action)? {
            Some((name, action_input)) if name.as_str() == ActionSequence::name_for_type() => {
                match action_input {
                    Some(action_input) => (
                        ActionSequence::build_sequence(action_input.clone(), cx),
                        None,
                    ),
                    None => (Err(ActionSequence::expected_array_error()), None),
                }
            }
            Some((name, Some(action_input))) => {
                let action_input_string = action_input.to_string();
                (
                    cx.build_action(name, Some(action_input.clone())),
                    Some(action_input_string),
                )
            }
            Some((name, None)) => (cx.build_action(name, None), None),
            None => (Ok(NoAction.boxed_clone()), None),
        };

        let action = match build_result {
            Ok(action) => action,
            Err(ActionBuildError::NotFound { name }) => {
                return Err(format!(
                    "didn't find an action named {}.",
                    MarkdownInlineCode(&format!("\"{name}\""))
                ));
            }
            Err(ActionBuildError::BuildError { name, error }) => match action_input_string {
                Some(action_input_string) => {
                    return Err(format!(
                        "can't build {} action from input value {}: {}",
                        MarkdownInlineCode(&format!("\"{name}\"")),
                        MarkdownInlineCode(&action_input_string),
                        MarkdownEscaped(&error.to_string())
                    ));
                }
                None => {
                    return Err(format!(
                        "can't build {} action - it requires input data via [name, input]: {}",
                        MarkdownInlineCode(&format!("\"{name}\"")),
                        MarkdownEscaped(&error.to_string())
                    ));
                }
            },
        };

        Ok((action, action_input_string))
    }

    /// Creates a JSON schema generator, suitable for generating json schemas
    /// for actions
    pub fn action_schema_generator() -> schemars::SchemaGenerator {
        schemars::generate::SchemaSettings::draft2019_09()
            .with_transform(AllowTrailingCommas)
            .into_generator()
    }

    pub fn generate_json_schema_for_registered_actions(cx: &mut App) -> Value {
        // instead of using DefaultDenyUnknownFields, actions typically use
        // `#[serde(deny_unknown_fields)]` so that these cases are reported as parse failures. This
        // is because the rest of the keymap will still load in these cases, whereas other settings
        // files would not.
        let mut generator = Self::action_schema_generator();

        let action_schemas = cx.action_schemas(&mut generator);
        let action_documentation = cx.action_documentation();
        let deprecations = cx.deprecated_actions_to_preferred_actions();
        let deprecation_messages = cx.action_deprecation_messages();
        KeymapFile::generate_json_schema(
            generator,
            action_schemas,
            action_documentation,
            deprecations,
            deprecation_messages,
        )
    }

    pub fn generate_json_schema_from_inventory() -> Value {
        let mut generator = Self::action_schema_generator();

        let mut action_schemas = Vec::new();
        let mut documentation = HashMap::default();
        let mut deprecations = HashMap::default();
        let mut deprecation_messages = HashMap::default();

        for action_data in generate_list_of_all_registered_actions() {
            let schema = (action_data.json_schema)(&mut generator);
            action_schemas.push((action_data.name, schema));

            if let Some(doc) = action_data.documentation {
                documentation.insert(action_data.name, doc);
            }
            if let Some(msg) = action_data.deprecation_message {
                deprecation_messages.insert(action_data.name, msg);
            }
            for &alias in action_data.deprecated_aliases {
                deprecations.insert(alias, action_data.name);

                let alias_schema = (action_data.json_schema)(&mut generator);
                action_schemas.push((alias, alias_schema));
            }
        }

        KeymapFile::generate_json_schema(
            generator,
            action_schemas,
            &documentation,
            &deprecations,
            &deprecation_messages,
        )
    }

    pub fn get_action_schema_by_name(
        action_name: &str,
        generator: &mut schemars::SchemaGenerator,
    ) -> Option<schemars::Schema> {
        for action_data in generate_list_of_all_registered_actions() {
            if action_data.name == action_name {
                return (action_data.json_schema)(generator);
            }
            for &alias in action_data.deprecated_aliases {
                if alias == action_name {
                    return (action_data.json_schema)(generator);
                }
            }
        }
        None
    }

    pub fn generate_json_schema<'a>(
        mut generator: schemars::SchemaGenerator,
        action_schemas: Vec<(&'a str, Option<schemars::Schema>)>,
        action_documentation: &HashMap<&'a str, &'a str>,
        deprecations: &HashMap<&'a str, &'a str>,
        deprecation_messages: &HashMap<&'a str, &'a str>,
    ) -> serde_json::Value {
        fn add_deprecation(schema: &mut schemars::Schema, message: String) {
            schema.insert(
                // deprecationMessage is not part of the JSON Schema spec, but
                // json-language-server recognizes it.
                "deprecationMessage".to_string(),
                Value::String(message),
            );
        }

        fn add_deprecation_preferred_name(schema: &mut schemars::Schema, new_name: &str) {
            add_deprecation(schema, format!("Deprecated, use {new_name}"));
        }

        fn add_description(schema: &mut schemars::Schema, description: &str) {
            schema.insert(
                "description".to_string(),
                Value::String(description.to_string()),
            );
        }

        let empty_object = json_schema!({
            "type": "object"
        });

        // This is a workaround for a json-language-server issue where it matches the first
        // alternative that matches the value's shape and uses that for documentation.
        //
        // In the case of the array validations, it would even provide an error saying that the name
        // must match the name of the first alternative.
        let mut empty_action_name = json_schema!({
            "type": "string",
            "const": ""
        });
        let no_action_message = "No action named this.";
        add_description(&mut empty_action_name, no_action_message);
        add_deprecation(&mut empty_action_name, no_action_message.to_string());
        let empty_action_name_with_input = json_schema!({
            "type": "array",
            "items": [
                empty_action_name,
                true
            ],
            "minItems": 2,
            "maxItems": 2
        });

        let mut keymap_deprecations = deprecations.clone();
        keymap_deprecations.insert(NoAction.name(), "null");
        let action_name_schema = ActionName::build_schema(
            action_schemas.iter().map(|(name, _)| *name),
            action_documentation,
            &keymap_deprecations,
            deprecation_messages,
        );

        let mut action_with_arguments_alternatives = vec![empty_action_name_with_input.clone()];
        let mut unbind_target_action_alternatives =
            vec![empty_action_name, empty_action_name_with_input];

        let mut empty_schema_action_names = vec![];
        let mut empty_schema_unbind_target_action_names = vec![];
        for (name, action_schema) in action_schemas.into_iter() {
            let deprecation = if name == NoAction.name() {
                Some("null")
            } else {
                deprecations.get(name).copied()
            };

            let include_in_unbind_target_schema =
                name != NoAction.name() && name != Unbind::name_for_type();

            // Add an alternative for plain action names.
            let mut plain_action = json_schema!({
                "type": "string",
                "const": name
            });
            if let Some(message) = deprecation_messages.get(name) {
                add_deprecation(&mut plain_action, message.to_string());
            } else if let Some(new_name) = deprecation {
                add_deprecation_preferred_name(&mut plain_action, new_name);
            }
            let description = action_documentation.get(name);
            if let Some(description) = &description {
                add_description(&mut plain_action, description);
            }
            if include_in_unbind_target_schema {
                unbind_target_action_alternatives.push(plain_action);
            }

            // Add an alternative for actions with data specified as a [name, data] array.
            //
            // When a struct with no deserializable fields is added by deriving `Action`, an empty
            // object schema is produced. The action should be invoked without data in this case.
            if let Some(schema) = action_schema
                && schema != empty_object
            {
                let mut matches_action_name = json_schema!({
                    "const": name
                });
                if let Some(description) = &description {
                    add_description(&mut matches_action_name, description);
                }
                if let Some(message) = deprecation_messages.get(name) {
                    add_deprecation(&mut matches_action_name, message.to_string());
                } else if let Some(new_name) = deprecation {
                    add_deprecation_preferred_name(&mut matches_action_name, new_name);
                }
                let action_with_input = json_schema!({
                    "type": "array",
                    "items": [matches_action_name, schema],
                    "minItems": 2,
                    "maxItems": 2
                });
                action_with_arguments_alternatives.push(action_with_input.clone());
                if include_in_unbind_target_schema {
                    unbind_target_action_alternatives.push(action_with_input);
                }
            } else {
                empty_schema_action_names.push(name);
                if include_in_unbind_target_schema {
                    empty_schema_unbind_target_action_names.push(name);
                }
            }
        }

        if !empty_schema_action_names.is_empty() {
            let action_names = json_schema!({ "enum": empty_schema_action_names });
            let no_properties_allowed = json_schema!({
                "type": "object",
                "additionalProperties": false
            });
            let mut actions_with_empty_input = json_schema!({
                "type": "array",
                "items": [action_names, no_properties_allowed],
                "minItems": 2,
                "maxItems": 2
            });
            add_deprecation(
                &mut actions_with_empty_input,
                "This action does not take input - just the action name string should be used."
                    .to_string(),
            );
            action_with_arguments_alternatives.push(actions_with_empty_input);
        }

        if !empty_schema_unbind_target_action_names.is_empty() {
            let action_names = json_schema!({ "enum": empty_schema_unbind_target_action_names });
            let no_properties_allowed = json_schema!({
                "type": "object",
                "additionalProperties": false
            });
            let mut actions_with_empty_input = json_schema!({
                "type": "array",
                "items": [action_names, no_properties_allowed],
                "minItems": 2,
                "maxItems": 2
            });
            add_deprecation(
                &mut actions_with_empty_input,
                "This action does not take input - just the action name string should be used."
                    .to_string(),
            );
            unbind_target_action_alternatives.push(actions_with_empty_input);
        }

        generator.definitions_mut().insert(
            ActionName::schema_name().to_string(),
            action_name_schema.to_value(),
        );
        generator.definitions_mut().insert(
            ActionWithArguments::schema_name().to_string(),
            json!({ "anyOf": action_with_arguments_alternatives }),
        );

        generator.definitions_mut().insert(
            KeymapAction::schema_name().to_string(),
            json!({ "anyOf": [
                { "$ref": format!("#/$defs/{}", ActionName::schema_name().to_string()) },
                { "$ref": format!("#/$defs/{}", ActionWithArguments::schema_name().to_string()) },
                { "type": "null" }
            ] }),
        );
        generator.definitions_mut().insert(
            UnbindTargetAction::schema_name().to_string(),
            json!({
                "anyOf": unbind_target_action_alternatives
            }),
        );

        generator.root_schema_for::<KeymapFile>().to_value()
    }

    pub fn sections(&self) -> impl DoubleEndedIterator<Item = &KeymapSection> {
        self.0.iter()
    }

    pub async fn load_keymap_file(fs: &Arc<dyn Fs>) -> Result<String> {
        match fs.load(paths::keymap_file()).await {
            result @ Ok(_) => result,
            Err(err) => {
                if let Some(e) = err.downcast_ref::<std::io::Error>()
                    && e.kind() == std::io::ErrorKind::NotFound
                {
                    return Ok(crate::initial_keymap_content().to_string());
                }
                Err(err)
            }
        }
    }

    pub fn update_keybinding<'a>(
        mut operation: KeybindUpdateOperation<'a>,
        mut keymap_contents: String,
        tab_size: usize,
        keyboard_mapper: &dyn gpui::PlatformKeyboardMapper,
        deprecated_aliases: &HashMap<&'static str, &'static str>,
    ) -> Result<String> {
        // When replacing or removing a non-user binding, we may need to write an unbind entry
        // to suppress the original default binding.
        let mut suppression_unbind: Option<KeybindUpdateTarget<'_>> = None;

        match &operation {
            // if trying to replace a keybinding that is not user-defined, treat it as an add operation
            KeybindUpdateOperation::Replace {
                target_keybind_source: target_source,
                source,
                target,
            } if *target_source != KeybindSource::User => {
                if target.keystrokes_unparsed() != source.keystrokes_unparsed() {
                    suppression_unbind = Some(target.clone());
                }
                operation = KeybindUpdateOperation::Add {
                    source: source.clone(),
                    from: Some(target.clone()),
                };
            }
            // if trying to remove a keybinding that is not user-defined, treat it as creating an
            // unbind entry for the removed action
            KeybindUpdateOperation::Remove {
                target,
                target_keybind_source,
            } if *target_keybind_source != KeybindSource::User => {
                suppression_unbind = Some(target.clone());
            }
            _ => {}
        }

        // Sanity check that keymap contents are valid, even though we only use it for Replace.
        // We don't want to modify the file if it's invalid.
        let keymap = Self::parse(&keymap_contents).context("Failed to parse keymap")?;

        if let KeybindUpdateOperation::RemoveEntries { provenances } = &operation {
            if provenances.is_empty() {
                anyhow::bail!("No entries to remove");
            }
            // Reject stale models before mutating the file.
            let current_revision = keymap_fingerprint(&keymap);
            for provenance in provenances.iter() {
                if provenance.revision != current_revision {
                    anyhow::bail!(
                        "Stale restore provenance: keymap changed since the entry was loaded; refresh and try again"
                    );
                }
            }
            // Validate all locations before mutating.
            struct ResolvedRemoval {
                section_index: usize,
                key_path: Vec<String>,
            }
            let mut resolved: Vec<ResolvedRemoval> = Vec::with_capacity(provenances.len());
            let mut seen = std::collections::HashSet::new();
            for provenance in provenances {
                let location = &provenance.location;
                if !seen.insert((
                    location.section_index,
                    location.collection as u8,
                    location.entry_index,
                )) {
                    continue;
                }
                let section = keymap.0.get(location.section_index).with_context(|| {
                    format!(
                        "Stale restore location: section {} no longer exists",
                        location.section_index
                    )
                })?;
                let (keystrokes_str, kind_path) = match location.collection {
                    KeymapEntryCollection::Bindings => {
                        let bindings = section.bindings.as_ref().with_context(|| {
                            format!(
                                "Stale restore location: section {} has no bindings",
                                location.section_index
                            )
                        })?;
                        let (key, _) = bindings.get_index(location.entry_index).with_context(|| {
                            format!(
                                "Stale restore location: bindings entry {} in section {} no longer exists",
                                location.entry_index, location.section_index
                            )
                        })?;
                        (key.clone(), "bindings")
                    }
                    KeymapEntryCollection::Unbind => {
                        let unbind = section.unbind.as_ref().with_context(|| {
                            format!(
                                "Stale restore location: section {} has no unbind",
                                location.section_index
                            )
                        })?;
                        let (key, _) = unbind.get_index(location.entry_index).with_context(|| {
                            format!(
                                "Stale restore location: unbind entry {} in section {} no longer exists",
                                location.entry_index, location.section_index
                            )
                        })?;
                        (key.clone(), "unbind")
                    }
                };
                let bindings_len = section.bindings.as_ref().map_or(0, IndexMap::len);
                let unbind_len = section.unbind.as_ref().map_or(0, IndexMap::len);
                // Don't collapse sections containing unrecognized fields.
                if bindings_len + unbind_len == 1 && section.unrecognized_fields.is_empty() {
                    resolved.push(ResolvedRemoval {
                        section_index: location.section_index,
                        key_path: Vec::new(),
                    });
                } else {
                    resolved.push(ResolvedRemoval {
                        section_index: location.section_index,
                        key_path: vec![kind_path.to_string(), keystrokes_str],
                    });
                }
            }
            // Prune now-empty collections in sections with unrecognized fields.
            {
                use std::collections::{HashMap, HashSet};
                let mut removal_count_by_collection: HashMap<(usize, String), usize> =
                    HashMap::new();
                for r in &resolved {
                    if r.key_path.len() == 2 {
                        *removal_count_by_collection
                            .entry((r.section_index, r.key_path[0].clone()))
                            .or_default() += 1;
                    }
                }
                let mut collections_to_prune: HashSet<(usize, String)> = HashSet::new();
                for ((section_index, kind), count) in removal_count_by_collection {
                    if let Some(section) = keymap.0.get(section_index) {
                        if section.unrecognized_fields.is_empty() {
                            continue;
                        }
                        let total = if kind == "bindings" {
                            section.bindings.as_ref().map_or(0, IndexMap::len)
                        } else {
                            section.unbind.as_ref().map_or(0, IndexMap::len)
                        };
                        if count >= total && total > 0 {
                            collections_to_prune.insert((section_index, kind));
                        }
                    }
                }
                if !collections_to_prune.is_empty() {
                    let mut pruned: Vec<ResolvedRemoval> = Vec::new();
                    let mut done_collections = HashSet::new();
                    for r in resolved.drain(..) {
                        if r.key_path.len() == 2
                            && collections_to_prune
                                .contains(&(r.section_index, r.key_path[0].clone()))
                        {
                            let key = (r.section_index, r.key_path[0].clone());
                            if done_collections.insert(key.clone()) {
                                pruned.push(ResolvedRemoval {
                                    section_index: key.0,
                                    key_path: vec![key.1],
                                });
                            }
                        } else {
                            pruned.push(r);
                        }
                    }
                    resolved = pruned;
                }
            }
            // Drop the entire section if all entries are removed and no unrecognized fields exist.
            {
                use std::collections::{HashMap, HashSet};
                let mut removal_count_by_section: HashMap<usize, usize> = HashMap::new();
                for r in &resolved {
                    if !r.key_path.is_empty() {
                        *removal_count_by_section.entry(r.section_index).or_default() += 1;
                    }
                }
                let mut sections_to_collapse = HashSet::new();
                for (section_index, count) in removal_count_by_section {
                    if let Some(section) = keymap.0.get(section_index) {
                        if !section.unrecognized_fields.is_empty() {
                            continue;
                        }
                        let total = section.bindings.as_ref().map_or(0, IndexMap::len)
                            + section.unbind.as_ref().map_or(0, IndexMap::len);
                        if count >= total && total > 0 {
                            sections_to_collapse.insert(section_index);
                        }
                    }
                }
                if !sections_to_collapse.is_empty() {
                    let mut collapsed: Vec<ResolvedRemoval> = Vec::new();
                    let mut done_sections = HashSet::new();
                    for r in resolved.drain(..) {
                        if sections_to_collapse.contains(&r.section_index) {
                            if done_sections.insert(r.section_index) {
                                collapsed.push(ResolvedRemoval {
                                    section_index: r.section_index,
                                    key_path: Vec::new(),
                                });
                            }
                        } else {
                            collapsed.push(r);
                        }
                    }
                    resolved = collapsed;
                }
            }
            // Remove in reverse document order so earlier positions stay stable.
            resolved.sort_by_key(|a| std::cmp::Reverse(a.section_index));
            // Deduplicate identical removals after section collapsing.
            {
                let mut seen_paths = std::collections::HashSet::new();
                resolved.retain(|r| {
                    let key = (r.section_index, r.key_path.join("\u{1f}"));
                    seen_paths.insert(key)
                });
            }
            for removal in resolved {
                let key_path_refs: Vec<&str> =
                    removal.key_path.iter().map(|s| s.as_str()).collect();
                let (replace_range, replace_value) = replace_top_level_array_value_in_json_text(
                    &keymap_contents,
                    &key_path_refs,
                    None,
                    None,
                    removal.section_index,
                    tab_size,
                );
                if replace_range.is_empty() && replace_value.is_empty() {
                    anyhow::bail!(
                        "Stale restore location: section {} changed",
                        removal.section_index
                    );
                }
                keymap_contents.replace_range(replace_range, &replace_value);
            }
            return Ok(keymap_contents);
        }

        if let KeybindUpdateOperation::Remove {
            target,
            target_keybind_source,
        } = &operation
        {
            if *target_keybind_source == KeybindSource::User {
                let target_action_value = target
                    .action_value()
                    .context("Failed to generate target action JSON value")?;
                let Some(binding_location) = find_binding(
                    &keymap,
                    target,
                    &target_action_value,
                    keyboard_mapper,
                    deprecated_aliases,
                ) else {
                    anyhow::bail!("Failed to find keybinding to remove");
                };
                let is_only_binding = binding_location.is_only_entry_in_section(&keymap);
                let key_path: &[&str] = if is_only_binding {
                    &[]
                } else {
                    &[
                        binding_location.kind.key_path(),
                        binding_location.keystrokes_str,
                    ]
                };
                let (replace_range, replace_value) = replace_top_level_array_value_in_json_text(
                    &keymap_contents,
                    key_path,
                    None,
                    None,
                    binding_location.index,
                    tab_size,
                );
                keymap_contents.replace_range(replace_range, &replace_value);

                return Ok(keymap_contents);
            }
        }

        if let KeybindUpdateOperation::Replace { source, target, .. } = operation {
            let target_action_value = target
                .action_value()
                .context("Failed to generate target action JSON value")?;
            let source_action_value = source
                .action_value()
                .context("Failed to generate source action JSON value")?;

            if let Some(binding_location) = find_binding(
                &keymap,
                &target,
                &target_action_value,
                keyboard_mapper,
                deprecated_aliases,
            ) {
                if target.context == source.context {
                    // if we are only changing the keybinding (common case)
                    // not the context, etc. Then just update the binding in place

                    let (replace_range, replace_value) = replace_top_level_array_value_in_json_text(
                        &keymap_contents,
                        &[
                            binding_location.kind.key_path(),
                            binding_location.keystrokes_str,
                        ],
                        Some(&source_action_value),
                        Some(&source.keystrokes_unparsed()),
                        binding_location.index,
                        tab_size,
                    );
                    keymap_contents.replace_range(replace_range, &replace_value);

                    return Ok(keymap_contents);
                } else if binding_location.is_only_entry_in_section(&keymap) {
                    // if we are replacing the only binding in the section,
                    // just update the section in place, updating the context
                    // and the binding

                    let (replace_range, replace_value) = replace_top_level_array_value_in_json_text(
                        &keymap_contents,
                        &[
                            binding_location.kind.key_path(),
                            binding_location.keystrokes_str,
                        ],
                        Some(&source_action_value),
                        Some(&source.keystrokes_unparsed()),
                        binding_location.index,
                        tab_size,
                    );
                    keymap_contents.replace_range(replace_range, &replace_value);

                    let (replace_range, replace_value) = replace_top_level_array_value_in_json_text(
                        &keymap_contents,
                        &["context"],
                        source.context.map(Into::into).as_ref(),
                        None,
                        binding_location.index,
                        tab_size,
                    );
                    keymap_contents.replace_range(replace_range, &replace_value);
                    return Ok(keymap_contents);
                } else {
                    // if we are replacing one of multiple bindings in a section
                    // with a context change, remove the existing binding from the
                    // section, then treat this operation as an add operation of the
                    // new binding with the updated context.

                    let (replace_range, replace_value) = replace_top_level_array_value_in_json_text(
                        &keymap_contents,
                        &[
                            binding_location.kind.key_path(),
                            binding_location.keystrokes_str,
                        ],
                        None,
                        None,
                        binding_location.index,
                        tab_size,
                    );
                    keymap_contents.replace_range(replace_range, &replace_value);
                    operation = KeybindUpdateOperation::Add {
                        source,
                        from: Some(target),
                    };
                }
            } else {
                log::warn!(
                    "Failed to find keybinding to update `{:?} -> {}` creating new binding for `{:?} -> {}` instead",
                    target.keystrokes,
                    target_action_value,
                    source.keystrokes,
                    source_action_value,
                );
                operation = KeybindUpdateOperation::Add {
                    source,
                    from: Some(target),
                };
            }
        }

        if let KeybindUpdateOperation::Add {
            source: keybinding,
            from,
        } = operation
        {
            let mut value = serde_json::Map::with_capacity(4);
            if let Some(context) = keybinding.context {
                value.insert("context".to_string(), context.into());
            }
            let use_key_equivalents = from.and_then(|from| {
                let action_value = from.action_value().context("Failed to serialize action value. `use_key_equivalents` on new keybinding may be incorrect.").log_err()?;
                let binding_location =
                    find_binding(&keymap, &from, &action_value, keyboard_mapper, deprecated_aliases)?;
                Some(keymap.0[binding_location.index].use_key_equivalents)
            }).unwrap_or(false);
            if use_key_equivalents {
                value.insert("use_key_equivalents".to_string(), true.into());
            }

            value.insert("bindings".to_string(), {
                let mut bindings = serde_json::Map::new();
                let action = keybinding.action_value()?;
                bindings.insert(keybinding.keystrokes_unparsed(), action);
                bindings.into()
            });

            let (replace_range, replace_value) = append_top_level_array_value_in_json_text(
                &keymap_contents,
                &value.into(),
                tab_size,
            );
            keymap_contents.replace_range(replace_range, &replace_value);
        }

        if let Some(suppression_unbind) = suppression_unbind {
            let mut value = serde_json::Map::with_capacity(2);
            if let Some(context) = suppression_unbind.context {
                value.insert("context".to_string(), context.into());
            }
            value.insert("unbind".to_string(), {
                let mut unbind = serde_json::Map::new();
                unbind.insert(
                    suppression_unbind.keystrokes_unparsed(),
                    suppression_unbind.action_value()?,
                );
                unbind.into()
            });
            let (replace_range, replace_value) = append_top_level_array_value_in_json_text(
                &keymap_contents,
                &value.into(),
                tab_size,
            );
            keymap_contents.replace_range(replace_range, &replace_value);
        }

        return Ok(keymap_contents);

        fn find_binding<'a, 'b>(
            keymap: &'b KeymapFile,
            target: &KeybindUpdateTarget<'a>,
            target_action_value: &Value,
            keyboard_mapper: &dyn gpui::PlatformKeyboardMapper,
            deprecated_aliases: &HashMap<&'static str, &'static str>,
        ) -> Option<BindingLocation<'b>> {
            let target_context_parsed =
                KeyBindingContextPredicate::parse(target.context.unwrap_or("")).ok();
            for (index, section) in keymap.sections().enumerate() {
                let section_context_parsed =
                    KeyBindingContextPredicate::parse(&section.context).ok();
                if section_context_parsed != target_context_parsed {
                    continue;
                }

                if let Some(binding_location) = find_binding_in_entries(
                    section.bindings.as_ref(),
                    BindingKind::Binding,
                    index,
                    target,
                    target_action_value,
                    keyboard_mapper,
                    deprecated_aliases,
                    |action| &action.0,
                ) {
                    return Some(binding_location);
                }

                if let Some(binding_location) = find_binding_in_entries(
                    section.unbind.as_ref(),
                    BindingKind::Unbind,
                    index,
                    target,
                    target_action_value,
                    keyboard_mapper,
                    deprecated_aliases,
                    |action| &action.0,
                ) {
                    return Some(binding_location);
                }
            }
            None
        }

        fn find_binding_in_entries<'a, 'b, T>(
            entries: Option<&'b IndexMap<String, T>>,
            kind: BindingKind,
            index: usize,
            target: &KeybindUpdateTarget<'a>,
            target_action_value: &Value,
            keyboard_mapper: &dyn gpui::PlatformKeyboardMapper,
            deprecated_aliases: &HashMap<&'static str, &'static str>,
            action_value: impl Fn(&T) -> &Value,
        ) -> Option<BindingLocation<'b>> {
            let entries = entries?;
            for (keystrokes_str, action) in entries {
                let Ok(keystrokes) = keystrokes_str
                    .split_whitespace()
                    .map(|source| {
                        let keystroke = Keystroke::parse(source)?;
                        Ok(KeybindingKeystroke::new_with_mapper(
                            keystroke,
                            false,
                            keyboard_mapper,
                        ))
                    })
                    .collect::<Result<Vec<_>, InvalidKeystrokeError>>()
                else {
                    continue;
                };
                if keystrokes.len() != target.keystrokes.len()
                    || !keystrokes
                        .iter()
                        .zip(target.keystrokes)
                        .all(|(a, b)| a.inner().should_match(b))
                {
                    continue;
                }
                if !action_value_matches_target(
                    action_value(action),
                    target_action_value,
                    deprecated_aliases,
                ) {
                    continue;
                }
                return Some(BindingLocation {
                    index,
                    kind,
                    keystrokes_str,
                });
            }
            None
        }

        /// Compares a keymap file entry's action value against the target action
        /// value. The target is built from a loaded binding's canonical action
        /// name, while the file entry may still use a deprecated alias.
        fn action_value_matches_target(
            action_value: &Value,
            target_action_value: &Value,
            deprecated_aliases: &HashMap<&'static str, &'static str>,
        ) -> bool {
            if action_value == target_action_value {
                return true;
            }
            let (name, arguments) = match action_value {
                Value::String(name) => (name.as_str(), None),
                Value::Array(items) => match items.as_slice() {
                    [Value::String(name), arguments] => (name.as_str(), Some(arguments)),
                    _ => return false,
                },
                _ => return false,
            };
            let Some(&preferred_name) = deprecated_aliases.get(name) else {
                return false;
            };
            match target_action_value {
                Value::String(target_name) => target_name == preferred_name && arguments.is_none(),
                Value::Array(items) => match items.as_slice() {
                    [Value::String(target_name), target_arguments] => {
                        target_name == preferred_name && arguments == Some(target_arguments)
                    }
                    _ => false,
                },
                _ => false,
            }
        }

        #[derive(Copy, Clone)]
        enum BindingKind {
            Binding,
            Unbind,
        }

        impl BindingKind {
            fn key_path(self) -> &'static str {
                match self {
                    Self::Binding => "bindings",
                    Self::Unbind => "unbind",
                }
            }
        }

        struct BindingLocation<'a> {
            index: usize,
            kind: BindingKind,
            keystrokes_str: &'a str,
        }

        impl BindingLocation<'_> {
            fn is_only_entry_in_section(&self, keymap: &KeymapFile) -> bool {
                let section = &keymap.0[self.index];
                if !section.unrecognized_fields.is_empty() {
                    return false;
                }
                let binding_count = section.bindings.as_ref().map_or(0, IndexMap::len);
                let unbind_count = section.unbind.as_ref().map_or(0, IndexMap::len);
                binding_count + unbind_count == 1
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum KeybindUpdateOperation<'a> {
    Replace {
        /// Describes the keybind to create
        source: KeybindUpdateTarget<'a>,
        /// Describes the keybind to remove
        target: KeybindUpdateTarget<'a>,
        target_keybind_source: KeybindSource,
    },
    Add {
        source: KeybindUpdateTarget<'a>,
        from: Option<KeybindUpdateTarget<'a>>,
    },
    Remove {
        target: KeybindUpdateTarget<'a>,
        target_keybind_source: KeybindSource,
    },
    RemoveEntries {
        provenances: Vec<KeymapEntryProvenance>,
    },
}

impl KeybindUpdateOperation<'_> {
    pub fn generate_telemetry(
        &self,
    ) -> (
        // The keybind that is created
        String,
        // The keybinding that was removed
        String,
        // The source of the keybinding
        String,
    ) {
        let (new_binding, removed_binding, source) = match &self {
            KeybindUpdateOperation::Replace {
                source,
                target,
                target_keybind_source,
            } => (Some(source), Some(target), Some(*target_keybind_source)),
            KeybindUpdateOperation::Add { source, .. } => (Some(source), None, None),
            KeybindUpdateOperation::Remove {
                target,
                target_keybind_source,
            } => (None, Some(target), Some(*target_keybind_source)),
            KeybindUpdateOperation::RemoveEntries { .. } => (None, None, Some(KeybindSource::User)),
        };

        let new_binding = new_binding
            .map(KeybindUpdateTarget::telemetry_string)
            .unwrap_or("null".to_owned());
        let removed_binding = removed_binding
            .map(KeybindUpdateTarget::telemetry_string)
            .unwrap_or("null".to_owned());

        let source = source
            .as_ref()
            .map(KeybindSource::name)
            .map(ToOwned::to_owned)
            .unwrap_or("null".to_owned());

        (new_binding, removed_binding, source)
    }
}

impl<'a> KeybindUpdateOperation<'a> {
    pub fn add(source: KeybindUpdateTarget<'a>) -> Self {
        Self::Add { source, from: None }
    }
}

#[derive(Debug, Clone)]
pub struct KeybindUpdateTarget<'a> {
    pub context: Option<&'a str>,
    pub keystrokes: &'a [KeybindingKeystroke],
    pub action_name: &'a str,
    pub action_arguments: Option<&'a str>,
}

impl<'a> KeybindUpdateTarget<'a> {
    fn action_value(&self) -> Result<Value> {
        if self.action_name == gpui::NoAction.name() {
            return Ok(Value::Null);
        }
        let action_name: Value = self.action_name.into();
        let value = match self.action_arguments {
            Some(args) if !args.is_empty() => {
                let args = serde_json::from_str::<Value>(args)
                    .context("Failed to parse action arguments as JSON")?;
                serde_json::json!([action_name, args])
            }
            _ => action_name,
        };
        Ok(value)
    }

    fn keystrokes_unparsed(&self) -> String {
        let mut keystrokes = String::with_capacity(self.keystrokes.len() * 8);
        for keystroke in self.keystrokes {
            // The reason use `keystroke.unparse()` instead of `keystroke.inner.unparse()`
            // here is that, we want the user to use `ctrl-shift-4` instead of `ctrl-$`
            // by default on Windows.
            keystrokes.push_str(&keystroke.unparse());
            keystrokes.push(' ');
        }
        keystrokes.pop();
        keystrokes
    }

    fn telemetry_string(&self) -> String {
        format!(
            "action_name: {}, context: {}, action_arguments: {}, keystrokes: {}",
            self.action_name,
            self.context.unwrap_or("global"),
            self.action_arguments.unwrap_or("none"),
            self.keystrokes_unparsed()
        )
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum KeybindSource {
    User,
    Vim,
    Base,
    #[default]
    Default,
    Unknown,
}

impl KeybindSource {
    const BASE: KeyBindingMetaIndex = KeyBindingMetaIndex(KeybindSource::Base as u32);
    const DEFAULT: KeyBindingMetaIndex = KeyBindingMetaIndex(KeybindSource::Default as u32);
    const VIM: KeyBindingMetaIndex = KeyBindingMetaIndex(KeybindSource::Vim as u32);
    const USER: KeyBindingMetaIndex = KeyBindingMetaIndex(KeybindSource::User as u32);

    pub fn name(&self) -> &'static str {
        match self {
            KeybindSource::User => "User",
            KeybindSource::Default => "Default",
            KeybindSource::Base => "Base",
            KeybindSource::Vim => "Vim",
            KeybindSource::Unknown => "Unknown",
        }
    }

    pub fn meta(&self) -> KeyBindingMetaIndex {
        match self {
            KeybindSource::User => Self::USER,
            KeybindSource::Default => Self::DEFAULT,
            KeybindSource::Base => Self::BASE,
            KeybindSource::Vim => Self::VIM,
            KeybindSource::Unknown => KeyBindingMetaIndex(*self as u32),
        }
    }

    pub fn from_meta(index: KeyBindingMetaIndex) -> Self {
        match index {
            Self::USER => KeybindSource::User,
            Self::BASE => KeybindSource::Base,
            Self::DEFAULT => KeybindSource::Default,
            Self::VIM => KeybindSource::Vim,
            _ => KeybindSource::Unknown,
        }
    }
}

impl From<KeyBindingMetaIndex> for KeybindSource {
    fn from(index: KeyBindingMetaIndex) -> Self {
        Self::from_meta(index)
    }
}

impl From<KeybindSource> for KeyBindingMetaIndex {
    fn from(source: KeybindSource) -> Self {
        source.meta()
    }
}

/// Runs a sequence of actions. Does not wait for asynchronous actions to complete before running
/// the next action. Currently only works in workspace windows.
///
/// This action is special-cased in keymap parsing to allow it to access `App` while parsing, so
/// that it can parse its input actions.
pub struct ActionSequence(pub Vec<Box<dyn Action>>);

register_action!(ActionSequence);

impl ActionSequence {
    fn build_sequence(
        value: Value,
        cx: &App,
    ) -> std::result::Result<Box<dyn Action>, ActionBuildError> {
        match value {
            Value::Array(values) => {
                let actions = values
                    .into_iter()
                    .enumerate()
                    .map(|(index, action)| {
                        match KeymapFile::build_keymap_action(&KeymapAction(action), cx) {
                            Ok((action, _)) => Ok(action),
                            Err(err) => {
                                return Err(ActionBuildError::BuildError {
                                    name: Self::name_for_type().to_string(),
                                    error: anyhow::anyhow!(
                                        "error at sequence index {index}: {err}"
                                    ),
                                });
                            }
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Box::new(Self(actions)))
            }
            _ => Err(Self::expected_array_error()),
        }
    }

    fn expected_array_error() -> ActionBuildError {
        ActionBuildError::BuildError {
            name: Self::name_for_type().to_string(),
            error: anyhow::anyhow!("expected array of actions"),
        }
    }
}

impl Action for ActionSequence {
    fn name(&self) -> &'static str {
        Self::name_for_type()
    }

    fn name_for_type() -> &'static str
    where
        Self: Sized,
    {
        "action::Sequence"
    }

    fn partial_eq(&self, action: &dyn Action) -> bool {
        action
            .as_any()
            .downcast_ref::<Self>()
            .map_or(false, |other| {
                self.0.len() == other.0.len()
                    && self
                        .0
                        .iter()
                        .zip(other.0.iter())
                        .all(|(a, b)| a.partial_eq(b.as_ref()))
            })
    }

    fn boxed_clone(&self) -> Box<dyn Action> {
        Box::new(ActionSequence(
            self.0
                .iter()
                .map(|action| action.boxed_clone())
                .collect::<Vec<_>>(),
        ))
    }

    fn build(_value: Value) -> Result<Box<dyn Action>> {
        Err(anyhow::anyhow!(
            "{} cannot be built directly",
            Self::name_for_type()
        ))
    }

    fn action_json_schema(generator: &mut schemars::SchemaGenerator) -> Option<schemars::Schema> {
        let keymap_action_schema = generator.subschema_for::<KeymapAction>();
        Some(json_schema!({
            "type": "array",
            "items": keymap_action_schema
        }))
    }

    fn deprecated_aliases() -> &'static [&'static str] {
        &[]
    }

    fn deprecation_message() -> Option<&'static str> {
        None
    }

    fn documentation() -> Option<&'static str> {
        Some(
            "Runs a sequence of actions.\n\n\
            NOTE: This does **not** wait for asynchronous actions to complete before running the next action.",
        )
    }
}

#[cfg(test)]
mod tests {
    use collections::HashMap;
    use gpui::{Action, App, DummyKeyboardMapper, KeybindingKeystroke, Keystroke, Unbind};
    use serde_json::Value;
    use unindent::Unindent;

    use crate::{
        KeybindSource, KeymapFile,
        keymap_file::{
            KeybindUpdateOperation, KeybindUpdateTarget, KeymapEntryCollection,
            KeymapEntryLocation, KeymapEntryProvenance,
        },
    };

    gpui::actions!(test_keymap_file, [StringAction, InputAction]);

    #[test]
    fn can_deserialize_keymap_with_trailing_comma() {
        let json = indoc::indoc! {"[
              // Standard macOS bindings
              {
                \"bindings\": {
                  \"up\": \"menu::SelectPrevious\",
                },
              },
            ]
                  "
        };
        KeymapFile::parse(json).unwrap();
    }

    #[gpui::test]
    fn keymap_section_unbinds_are_loaded_before_bindings(cx: &mut App) {
        let key_bindings = match KeymapFile::load(
            indoc::indoc! {r#"
                [
                    {
                        "unbind": {
                            "ctrl-a": "test_keymap_file::StringAction",
                            "ctrl-b": ["test_keymap_file::InputAction", {}]
                        },
                        "bindings": {
                            "ctrl-c": "test_keymap_file::StringAction"
                        }
                    }
                ]
            "#},
            cx,
        ) {
            crate::keymap_file::KeymapFileLoadResult::Success { key_bindings } => key_bindings,
            crate::keymap_file::KeymapFileLoadResult::SomeFailedToLoad {
                error_message, ..
            } => {
                panic!("{error_message}");
            }
            crate::keymap_file::KeymapFileLoadResult::JsonParseFailure { error } => {
                panic!("JSON parse error: {error}");
            }
        };

        assert_eq!(key_bindings.len(), 3);
        assert!(
            key_bindings[0]
                .action()
                .partial_eq(&Unbind("test_keymap_file::StringAction".into()))
        );
        assert_eq!(key_bindings[0].action_input(), None);
        assert!(
            key_bindings[1]
                .action()
                .partial_eq(&Unbind("test_keymap_file::InputAction".into()))
        );
        assert_eq!(
            key_bindings[1]
                .action_input()
                .as_ref()
                .map(ToString::to_string),
            Some("{}".to_string())
        );
        assert_eq!(
            key_bindings[2].action().name(),
            "test_keymap_file::StringAction"
        );
    }

    #[gpui::test]
    fn keymap_unbind_loads_valid_target_action_with_input(cx: &mut App) {
        let key_bindings = match KeymapFile::load(
            indoc::indoc! {r#"
                [
                    {
                        "unbind": {
                            "ctrl-a": ["test_keymap_file::InputAction", {}]
                        }
                    }
                ]
            "#},
            cx,
        ) {
            crate::keymap_file::KeymapFileLoadResult::Success { key_bindings } => key_bindings,
            other => panic!("expected Success, got {other:?}"),
        };

        assert_eq!(key_bindings.len(), 1);
        assert!(
            key_bindings[0]
                .action()
                .partial_eq(&Unbind("test_keymap_file::InputAction".into()))
        );
        assert_eq!(
            key_bindings[0]
                .action_input()
                .as_ref()
                .map(ToString::to_string),
            Some("{}".to_string())
        );
    }

    #[gpui::test]
    fn keymap_unbind_rejects_null(cx: &mut App) {
        match KeymapFile::load(
            indoc::indoc! {r#"
                [
                    {
                        "unbind": {
                            "ctrl-a": null
                        }
                    }
                ]
            "#},
            cx,
        ) {
            crate::keymap_file::KeymapFileLoadResult::SomeFailedToLoad {
                key_bindings,
                error_message,
            } => {
                assert!(key_bindings.is_empty());
                assert!(
                    error_message
                        .0
                        .contains("expected action name string or [name, input] array.")
                );
            }
            other => panic!("expected SomeFailedToLoad, got {other:?}"),
        }
    }

    #[gpui::test]
    fn keymap_unbind_rejects_unbind_action(cx: &mut App) {
        match KeymapFile::load(
            indoc::indoc! {r#"
                [
                    {
                        "unbind": {
                            "ctrl-a": ["zed::Unbind", "test_keymap_file::StringAction"]
                        }
                    }
                ]
            "#},
            cx,
        ) {
            crate::keymap_file::KeymapFileLoadResult::SomeFailedToLoad {
                key_bindings,
                error_message,
            } => {
                assert!(key_bindings.is_empty());
                assert!(
                    error_message
                        .0
                        .contains("can't use `\"zed::Unbind\"` as an unbind target.")
                );
            }
            other => panic!("expected SomeFailedToLoad, got {other:?}"),
        }
    }

    #[test]
    fn keymap_schema_for_unbind_excludes_null_and_unbind_action() {
        fn schema_allows(schema: &Value, expected: &Value) -> bool {
            match schema {
                Value::Object(object) => {
                    if object.get("const") == Some(expected) {
                        return true;
                    }
                    if object.get("type") == Some(&Value::String("null".to_string()))
                        && expected == &Value::Null
                    {
                        return true;
                    }
                    object.values().any(|value| schema_allows(value, expected))
                }
                Value::Array(items) => items.iter().any(|value| schema_allows(value, expected)),
                _ => false,
            }
        }

        let schema = KeymapFile::generate_json_schema_from_inventory();
        let unbind_schema = schema
            .pointer("/$defs/UnbindTargetAction")
            .expect("missing UnbindTargetAction schema");

        assert!(!schema_allows(unbind_schema, &Value::Null));
        assert!(!schema_allows(
            unbind_schema,
            &Value::String(Unbind::name_for_type().to_string())
        ));
        assert!(schema_allows(
            unbind_schema,
            &Value::String("test_keymap_file::StringAction".to_string())
        ));
        assert!(schema_allows(
            unbind_schema,
            &Value::String("test_keymap_file::InputAction".to_string())
        ));
    }

    #[track_caller]
    fn check_keymap_update(
        input: impl ToString,
        operation: KeybindUpdateOperation,
        expected: impl ToString,
    ) {
        check_keymap_update_with_deprecated_aliases(input, operation, expected, &[]);
    }

    #[track_caller]
    fn check_keymap_update_with_deprecated_aliases(
        input: impl ToString,
        operation: KeybindUpdateOperation,
        expected: impl ToString,
        deprecated_aliases: &[(&'static str, &'static str)],
    ) {
        let deprecated_aliases = HashMap::from_iter(deprecated_aliases.iter().copied());
        let result = KeymapFile::update_keybinding(
            operation,
            input.to_string(),
            4,
            &gpui::DummyKeyboardMapper,
            &deprecated_aliases,
        )
        .expect("Update succeeded");
        pretty_assertions::assert_eq!(expected.to_string(), result);
    }

    #[track_caller]
    fn parse_keystrokes(keystrokes: &str) -> Vec<KeybindingKeystroke> {
        keystrokes
            .split(' ')
            .map(|s| {
                KeybindingKeystroke::new_with_mapper(
                    Keystroke::parse(s).expect("Keystrokes valid"),
                    false,
                    &DummyKeyboardMapper,
                )
            })
            .collect()
    }

    #[test]
    fn keymap_update() {
        zlog::init_test();

        check_keymap_update(
            "[]",
            KeybindUpdateOperation::add(KeybindUpdateTarget {
                keystrokes: &parse_keystrokes("ctrl-a"),
                action_name: "zed::SomeAction",
                context: None,
                action_arguments: None,
            }),
            r#"[
                {
                    "bindings": {
                        "ctrl-a": "zed::SomeAction"
                    }
                }
            ]"#
            .unindent(),
        );

        check_keymap_update(
            "[]",
            KeybindUpdateOperation::add(KeybindUpdateTarget {
                keystrokes: &parse_keystrokes("\\ a"),
                action_name: "zed::SomeAction",
                context: None,
                action_arguments: None,
            }),
            r#"[
                {
                    "bindings": {
                        "\\ a": "zed::SomeAction"
                    }
                }
            ]"#
            .unindent(),
        );

        check_keymap_update(
            "[]",
            KeybindUpdateOperation::add(KeybindUpdateTarget {
                keystrokes: &parse_keystrokes("ctrl-a"),
                action_name: "zed::SomeAction",
                context: None,
                action_arguments: Some(""),
            }),
            r#"[
                {
                    "bindings": {
                        "ctrl-a": "zed::SomeAction"
                    }
                }
            ]"#
            .unindent(),
        );

        check_keymap_update(
            r#"[
                {
                    "bindings": {
                        "ctrl-a": "zed::SomeAction"
                    }
                }
            ]"#
            .unindent(),
            KeybindUpdateOperation::add(KeybindUpdateTarget {
                keystrokes: &parse_keystrokes("ctrl-b"),
                action_name: "zed::SomeOtherAction",
                context: None,
                action_arguments: None,
            }),
            r#"[
                {
                    "bindings": {
                        "ctrl-a": "zed::SomeAction"
                    }
                },
                {
                    "bindings": {
                        "ctrl-b": "zed::SomeOtherAction"
                    }
                }
            ]"#
            .unindent(),
        );

        check_keymap_update(
            r#"[
                {
                    "bindings": {
                        "ctrl-a": "zed::SomeAction"
                    }
                }
            ]"#
            .unindent(),
            KeybindUpdateOperation::add(KeybindUpdateTarget {
                keystrokes: &parse_keystrokes("ctrl-b"),
                action_name: "zed::SomeOtherAction",
                context: None,
                action_arguments: Some(r#"{"foo": "bar"}"#),
            }),
            r#"[
                {
                    "bindings": {
                        "ctrl-a": "zed::SomeAction"
                    }
                },
                {
                    "bindings": {
                        "ctrl-b": [
                            "zed::SomeOtherAction",
                            {
                                "foo": "bar"
                            }
                        ]
                    }
                }
            ]"#
            .unindent(),
        );

        check_keymap_update(
            r#"[
                {
                    "bindings": {
                        "ctrl-a": "zed::SomeAction"
                    }
                }
            ]"#
            .unindent(),
            KeybindUpdateOperation::add(KeybindUpdateTarget {
                keystrokes: &parse_keystrokes("ctrl-b"),
                action_name: "zed::SomeOtherAction",
                context: Some("Zed > Editor && some_condition = true"),
                action_arguments: Some(r#"{"foo": "bar"}"#),
            }),
            r#"[
                {
                    "bindings": {
                        "ctrl-a": "zed::SomeAction"
                    }
                },
                {
                    "context": "Zed > Editor && some_condition = true",
                    "bindings": {
                        "ctrl-b": [
                            "zed::SomeOtherAction",
                            {
                                "foo": "bar"
                            }
                        ]
                    }
                }
            ]"#
            .unindent(),
        );

        check_keymap_update(
            r#"[
                {
                    "bindings": {
                        "ctrl-a": "zed::SomeAction"
                    }
                }
            ]"#
            .unindent(),
            KeybindUpdateOperation::Replace {
                target: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("ctrl-a"),
                    action_name: "zed::SomeAction",
                    context: None,
                    action_arguments: None,
                },
                source: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("ctrl-b"),
                    action_name: "zed::SomeOtherAction",
                    context: None,
                    action_arguments: Some(r#"{"foo": "bar"}"#),
                },
                target_keybind_source: KeybindSource::Base,
            },
            r#"[
                {
                    "bindings": {
                        "ctrl-a": "zed::SomeAction"
                    }
                },
                {
                    "bindings": {
                        "ctrl-b": [
                            "zed::SomeOtherAction",
                            {
                                "foo": "bar"
                            }
                        ]
                    }
                },
                {
                    "unbind": {
                        "ctrl-a": "zed::SomeAction"
                    }
                }
            ]"#
            .unindent(),
        );

        // Replacing a non-user binding without changing the keystroke should
        // not produce an unbind suppression entry.
        check_keymap_update(
            r#"[
                {
                    "bindings": {
                        "ctrl-a": "zed::SomeAction"
                    }
                }
            ]"#
            .unindent(),
            KeybindUpdateOperation::Replace {
                target: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("ctrl-a"),
                    action_name: "zed::SomeAction",
                    context: None,
                    action_arguments: None,
                },
                source: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("ctrl-a"),
                    action_name: "zed::SomeOtherAction",
                    context: None,
                    action_arguments: None,
                },
                target_keybind_source: KeybindSource::Base,
            },
            r#"[
                {
                    "bindings": {
                        "ctrl-a": "zed::SomeAction"
                    }
                },
                {
                    "bindings": {
                        "ctrl-a": "zed::SomeOtherAction"
                    }
                }
            ]"#
            .unindent(),
        );

        // Replacing a non-user binding with a context and a keystroke change
        // should produce a suppression entry that preserves the context.
        check_keymap_update(
            r#"[
                {
                    "context": "SomeContext",
                    "bindings": {
                        "ctrl-a": "zed::SomeAction"
                    }
                }
            ]"#
            .unindent(),
            KeybindUpdateOperation::Replace {
                target: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("ctrl-a"),
                    action_name: "zed::SomeAction",
                    context: Some("SomeContext"),
                    action_arguments: None,
                },
                source: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("ctrl-b"),
                    action_name: "zed::SomeOtherAction",
                    context: Some("SomeContext"),
                    action_arguments: None,
                },
                target_keybind_source: KeybindSource::Default,
            },
            r#"[
                {
                    "context": "SomeContext",
                    "bindings": {
                        "ctrl-a": "zed::SomeAction"
                    }
                },
                {
                    "context": "SomeContext",
                    "bindings": {
                        "ctrl-b": "zed::SomeOtherAction"
                    }
                },
                {
                    "context": "SomeContext",
                    "unbind": {
                        "ctrl-a": "zed::SomeAction"
                    }
                }
            ]"#
            .unindent(),
        );

        check_keymap_update(
            r#"[
                {
                    "bindings": {
                        "a": "zed::SomeAction"
                    }
                }
            ]"#
            .unindent(),
            KeybindUpdateOperation::Replace {
                target: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("a"),
                    action_name: "zed::SomeAction",
                    context: None,
                    action_arguments: None,
                },
                source: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("ctrl-b"),
                    action_name: "zed::SomeOtherAction",
                    context: None,
                    action_arguments: Some(r#"{"foo": "bar"}"#),
                },
                target_keybind_source: KeybindSource::User,
            },
            r#"[
                {
                    "bindings": {
                        "ctrl-b": [
                            "zed::SomeOtherAction",
                            {
                                "foo": "bar"
                            }
                        ]
                    }
                }
            ]"#
            .unindent(),
        );

        check_keymap_update(
            r#"[
                {
                    "bindings": {
                        "\\ a": "zed::SomeAction"
                    }
                }
            ]"#
            .unindent(),
            KeybindUpdateOperation::Replace {
                target: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("\\ a"),
                    action_name: "zed::SomeAction",
                    context: None,
                    action_arguments: None,
                },
                source: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("\\ b"),
                    action_name: "zed::SomeOtherAction",
                    context: None,
                    action_arguments: Some(r#"{"foo": "bar"}"#),
                },
                target_keybind_source: KeybindSource::User,
            },
            r#"[
                {
                    "bindings": {
                        "\\ b": [
                            "zed::SomeOtherAction",
                            {
                                "foo": "bar"
                            }
                        ]
                    }
                }
            ]"#
            .unindent(),
        );

        check_keymap_update(
            r#"[
                {
                    "bindings": {
                        "\\ a": "zed::SomeAction"
                    }
                }
            ]"#
            .unindent(),
            KeybindUpdateOperation::Replace {
                target: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("\\ a"),
                    action_name: "zed::SomeAction",
                    context: None,
                    action_arguments: None,
                },
                source: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("\\ a"),
                    action_name: "zed::SomeAction",
                    context: None,
                    action_arguments: None,
                },
                target_keybind_source: KeybindSource::User,
            },
            r#"[
                {
                    "bindings": {
                        "\\ a": "zed::SomeAction"
                    }
                }
            ]"#
            .unindent(),
        );

        check_keymap_update(
            r#"[
                {
                    "bindings": {
                        "ctrl-a": "zed::SomeAction"
                    }
                }
            ]"#
            .unindent(),
            KeybindUpdateOperation::Replace {
                target: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("ctrl-a"),
                    action_name: "zed::SomeNonexistentAction",
                    context: None,
                    action_arguments: None,
                },
                source: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("ctrl-b"),
                    action_name: "zed::SomeOtherAction",
                    context: None,
                    action_arguments: None,
                },
                target_keybind_source: KeybindSource::User,
            },
            r#"[
                {
                    "bindings": {
                        "ctrl-a": "zed::SomeAction"
                    }
                },
                {
                    "bindings": {
                        "ctrl-b": "zed::SomeOtherAction"
                    }
                }
            ]"#
            .unindent(),
        );

        check_keymap_update(
            r#"[
                {
                    "bindings": {
                        // some comment
                        "ctrl-a": "zed::SomeAction"
                        // some other comment
                    }
                }
            ]"#
            .unindent(),
            KeybindUpdateOperation::Replace {
                target: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("ctrl-a"),
                    action_name: "zed::SomeAction",
                    context: None,
                    action_arguments: None,
                },
                source: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("ctrl-b"),
                    action_name: "zed::SomeOtherAction",
                    context: None,
                    action_arguments: Some(r#"{"foo": "bar"}"#),
                },
                target_keybind_source: KeybindSource::User,
            },
            r#"[
                {
                    "bindings": {
                        // some comment
                        "ctrl-b": [
                            "zed::SomeOtherAction",
                            {
                                "foo": "bar"
                            }
                        ]
                        // some other comment
                    }
                }
            ]"#
            .unindent(),
        );

        check_keymap_update(
            r#"[
                {
                    "context": "SomeContext",
                    "bindings": {
                        "a": "foo::bar",
                        "b": "baz::qux",
                    }
                }
            ]"#
            .unindent(),
            KeybindUpdateOperation::Replace {
                target: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("a"),
                    action_name: "foo::bar",
                    context: Some("SomeContext"),
                    action_arguments: None,
                },
                source: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("c"),
                    action_name: "foo::baz",
                    context: Some("SomeOtherContext"),
                    action_arguments: None,
                },
                target_keybind_source: KeybindSource::User,
            },
            r#"[
                {
                    "context": "SomeContext",
                    "bindings": {
                        "b": "baz::qux",
                    }
                },
                {
                    "context": "SomeOtherContext",
                    "bindings": {
                        "c": "foo::baz"
                    }
                }
            ]"#
            .unindent(),
        );

        check_keymap_update(
            r#"[
                {
                    "context": "SomeContext",
                    "bindings": {
                        "a": "foo::bar",
                    }
                }
            ]"#
            .unindent(),
            KeybindUpdateOperation::Replace {
                target: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("a"),
                    action_name: "foo::bar",
                    context: Some("SomeContext"),
                    action_arguments: None,
                },
                source: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("c"),
                    action_name: "foo::baz",
                    context: Some("SomeOtherContext"),
                    action_arguments: None,
                },
                target_keybind_source: KeybindSource::User,
            },
            r#"[
                {
                    "context": "SomeOtherContext",
                    "bindings": {
                        "c": "foo::baz",
                    }
                }
            ]"#
            .unindent(),
        );

        check_keymap_update(
            r#"[
                {
                    "context": "SomeContext",
                    "bindings": {
                        "a": "foo::bar",
                        "c": "foo::baz",
                    }
                },
            ]"#
            .unindent(),
            KeybindUpdateOperation::Remove {
                target: KeybindUpdateTarget {
                    context: Some("SomeContext"),
                    keystrokes: &parse_keystrokes("a"),
                    action_name: "foo::bar",
                    action_arguments: None,
                },
                target_keybind_source: KeybindSource::User,
            },
            r#"[
                {
                    "context": "SomeContext",
                    "bindings": {
                        "c": "foo::baz",
                    }
                },
            ]"#
            .unindent(),
        );

        check_keymap_update(
            r#"[
                {
                    "context": "SomeContext",
                    "bindings": {
                        "\\ a": "foo::bar",
                        "c": "foo::baz",
                    }
                },
            ]"#
            .unindent(),
            KeybindUpdateOperation::Remove {
                target: KeybindUpdateTarget {
                    context: Some("SomeContext"),
                    keystrokes: &parse_keystrokes("\\ a"),
                    action_name: "foo::bar",
                    action_arguments: None,
                },
                target_keybind_source: KeybindSource::User,
            },
            r#"[
                {
                    "context": "SomeContext",
                    "bindings": {
                        "c": "foo::baz",
                    }
                },
            ]"#
            .unindent(),
        );

        check_keymap_update(
            r#"[
                {
                    "context": "SomeContext",
                    "bindings": {
                        "a": ["foo::bar", true],
                        "c": "foo::baz",
                    }
                },
            ]"#
            .unindent(),
            KeybindUpdateOperation::Remove {
                target: KeybindUpdateTarget {
                    context: Some("SomeContext"),
                    keystrokes: &parse_keystrokes("a"),
                    action_name: "foo::bar",
                    action_arguments: Some("true"),
                },
                target_keybind_source: KeybindSource::User,
            },
            r#"[
                {
                    "context": "SomeContext",
                    "bindings": {
                        "c": "foo::baz",
                    }
                },
            ]"#
            .unindent(),
        );

        check_keymap_update(
            r#"[
                {
                    "context": "SomeContext",
                    "bindings": {
                        "b": "foo::baz",
                    }
                },
                {
                    "context": "SomeContext",
                    "bindings": {
                        "a": ["foo::bar", true],
                    }
                },
                {
                    "context": "SomeContext",
                    "bindings": {
                        "c": "foo::baz",
                    }
                },
            ]"#
            .unindent(),
            KeybindUpdateOperation::Remove {
                target: KeybindUpdateTarget {
                    context: Some("SomeContext"),
                    keystrokes: &parse_keystrokes("a"),
                    action_name: "foo::bar",
                    action_arguments: Some("true"),
                },
                target_keybind_source: KeybindSource::User,
            },
            r#"[
                {
                    "context": "SomeContext",
                    "bindings": {
                        "b": "foo::baz",
                    }
                },
                {
                    "context": "SomeContext",
                    "bindings": {
                        "c": "foo::baz",
                    }
                },
            ]"#
            .unindent(),
        );
        check_keymap_update(
            r#"[
                {
                    "context": "SomeOtherContext",
                    "use_key_equivalents": true,
                    "bindings": {
                        "b": "foo::bar",
                    }
                },
            ]"#
            .unindent(),
            KeybindUpdateOperation::Add {
                source: KeybindUpdateTarget {
                    context: Some("SomeContext"),
                    keystrokes: &parse_keystrokes("a"),
                    action_name: "foo::baz",
                    action_arguments: Some("true"),
                },
                from: Some(KeybindUpdateTarget {
                    context: Some("SomeOtherContext"),
                    keystrokes: &parse_keystrokes("b"),
                    action_name: "foo::bar",
                    action_arguments: None,
                }),
            },
            r#"[
                {
                    "context": "SomeOtherContext",
                    "use_key_equivalents": true,
                    "bindings": {
                        "b": "foo::bar",
                    }
                },
                {
                    "context": "SomeContext",
                    "use_key_equivalents": true,
                    "bindings": {
                        "a": [
                            "foo::baz",
                            true
                        ]
                    }
                }
            ]"#
            .unindent(),
        );

        check_keymap_update(
            r#"[
                {
                    "context": "SomeOtherContext",
                    "use_key_equivalents": true,
                    "bindings": {
                        "b": "foo::bar",
                    }
                },
            ]"#
            .unindent(),
            KeybindUpdateOperation::Remove {
                target: KeybindUpdateTarget {
                    context: Some("SomeContext"),
                    keystrokes: &parse_keystrokes("a"),
                    action_name: "foo::baz",
                    action_arguments: Some("true"),
                },
                target_keybind_source: KeybindSource::Default,
            },
            r#"[
                {
                    "context": "SomeOtherContext",
                    "use_key_equivalents": true,
                    "bindings": {
                        "b": "foo::bar",
                    }
                },
                {
                    "context": "SomeContext",
                    "unbind": {
                        "a": [
                            "foo::baz",
                            true
                        ]
                    }
                }
            ]"#
            .unindent(),
        );
    }

    #[test]
    fn test_keymap_remove_duplicate_binding() {
        zlog::init_test();

        // Repro: user created two identical bindings via the keymap editor UI,
        // resulting in two sections with the same keystrokes and action.
        // Deleting one of them should remove exactly one section.
        check_keymap_update(
            r#"
            [
              {
                "bindings": {
                  "alt-cmd-shift-c": "workspace::CopyRelativePath"
                }
              },
              {
                "bindings": {
                  "alt-cmd-shift-c": "workspace::CopyRelativePath"
                }
              }
            ]
            "#
            .unindent(),
            KeybindUpdateOperation::Remove {
                target: KeybindUpdateTarget {
                    context: None,
                    keystrokes: &parse_keystrokes("alt-cmd-shift-c"),
                    action_name: "workspace::CopyRelativePath",
                    action_arguments: None,
                },
                target_keybind_source: KeybindSource::User,
            },
            r#"
            [
              {
                "bindings": {
                  "alt-cmd-shift-c": "workspace::CopyRelativePath"
                }
              }
            ]
            "#
            .unindent(),
        );
    }

    #[test]
    fn test_keymap_update_deprecated_alias_binding() {
        zlog::init_test();

        // The keymap file entry uses a deprecated alias of the action, while the
        // update target uses the canonical name (as reported by the loaded binding).
        let deprecated_aliases = &[("editor::CopyRelativePath", "workspace::CopyRelativePath")];

        check_keymap_update_with_deprecated_aliases(
            r#"
            [
              {
                "bindings": {
                  "alt-cmd-shift-c": "editor::CopyRelativePath",
                  "a": "foo::Bar"
                }
              }
            ]
            "#
            .unindent(),
            KeybindUpdateOperation::Remove {
                target: KeybindUpdateTarget {
                    context: None,
                    keystrokes: &parse_keystrokes("alt-cmd-shift-c"),
                    action_name: "workspace::CopyRelativePath",
                    action_arguments: None,
                },
                target_keybind_source: KeybindSource::User,
            },
            r#"
            [
              {
                "bindings": {
                  "a": "foo::Bar"
                }
              }
            ]
            "#
            .unindent(),
            deprecated_aliases,
        );

        // Editing an alias entry should update it in place (rewriting it with the
        // canonical name) instead of falling back to appending a new binding.
        check_keymap_update_with_deprecated_aliases(
            r#"
            [
              {
                "bindings": {
                  "alt-cmd-shift-c": "editor::CopyRelativePath"
                }
              }
            ]
            "#
            .unindent(),
            KeybindUpdateOperation::Replace {
                target: KeybindUpdateTarget {
                    context: None,
                    keystrokes: &parse_keystrokes("alt-cmd-shift-c"),
                    action_name: "workspace::CopyRelativePath",
                    action_arguments: None,
                },
                source: KeybindUpdateTarget {
                    context: None,
                    keystrokes: &parse_keystrokes("ctrl-alt-c"),
                    action_name: "workspace::CopyRelativePath",
                    action_arguments: None,
                },
                target_keybind_source: KeybindSource::User,
            },
            r#"
            [
              {
                "bindings": {
                  "ctrl-alt-c": "workspace::CopyRelativePath"
                }
              }
            ]
            "#
            .unindent(),
            deprecated_aliases,
        );
    }

    #[test]
    fn test_keymap_remove() {
        zlog::init_test();

        check_keymap_update(
            r#"
            [
              {
                "context": "Editor",
                "bindings": {
                  "cmd-k cmd-u": "editor::ConvertToUpperCase",
                  "cmd-k cmd-l": "editor::ConvertToLowerCase",
                  "cmd-[": "pane::GoBack",
                }
              },
            ]
            "#,
            KeybindUpdateOperation::Remove {
                target: KeybindUpdateTarget {
                    context: Some("Editor"),
                    keystrokes: &parse_keystrokes("cmd-k cmd-l"),
                    action_name: "editor::ConvertToLowerCase",
                    action_arguments: None,
                },
                target_keybind_source: KeybindSource::User,
            },
            r#"
            [
              {
                "context": "Editor",
                "bindings": {
                  "cmd-k cmd-u": "editor::ConvertToUpperCase",
                  "cmd-[": "pane::GoBack",
                }
              },
            ]
            "#,
        );
    }

    #[gpui::test]
    fn loader_provenance_tracks_section_collection_and_entry(cx: &mut App) {
        let file = KeymapFile::parse(
            r#"[
                {"bindings": {"a": "test_keymap_file::StringAction"}},
                {"unbind": {"b": "test_keymap_file::StringAction"}, "bindings": {"c": "test_keymap_file::StringAction"}}
            ]"#,
        )
        .unwrap();
        let (result, provenances) = file.load_keymap_with_locations(cx);
        let bindings = match result {
            crate::keymap_file::KeymapFileLoadResult::Success { key_bindings } => key_bindings,
            other => panic!("expected success, got {other:?}"),
        };
        assert_eq!(bindings.len(), 3);
        assert_eq!(provenances.len(), 3);
        // Loader order: section 0 bindings, then section 1 unbind, then bindings.
        assert_eq!(
            provenances[0].location,
            KeymapEntryLocation {
                section_index: 0,
                collection: KeymapEntryCollection::Bindings,
                entry_index: 0,
            }
        );
        assert_eq!(
            provenances[1].location,
            KeymapEntryLocation {
                section_index: 1,
                collection: KeymapEntryCollection::Unbind,
                entry_index: 0,
            }
        );
        assert_eq!(
            provenances[2].location,
            KeymapEntryLocation {
                section_index: 1,
                collection: KeymapEntryCollection::Bindings,
                entry_index: 0,
            }
        );
        // All provenances from one load share its content revision, and each
        // binding's generic runtime token round-trips back to the same identity.
        for provenance in provenances.iter() {
            assert_eq!(provenance.revision, provenances[0].revision);
        }
        for (binding, provenance) in bindings.iter().zip(provenances.iter()) {
            assert_eq!(
                KeymapEntryProvenance::from_gpui(binding.provenance().expect("provenance")),
                Some(*provenance)
            );
        }
    }

    #[gpui::test]
    fn loader_provenance_skips_invalid_entries(cx: &mut App) {
        let file = KeymapFile::parse(
            r#"[
                {"bindings": {
                    "a": "test_keymap_file::StringAction",
                    "b": ["test_keymap_file::InputAction", 42, "extra"]
                }}
            ]"#,
        )
        .unwrap();
        let (result, provenances) = file.load_keymap_with_locations(cx);
        match result {
            crate::keymap_file::KeymapFileLoadResult::SomeFailedToLoad { key_bindings, .. } => {
                assert_eq!(key_bindings.len(), 1);
                assert_eq!(provenances.len(), 1);
                assert_eq!(provenances[0].location.entry_index, 0);
                assert_eq!(
                    KeymapEntryProvenance::from_gpui(
                        key_bindings[0].provenance().expect("provenance")
                    ),
                    Some(provenances[0])
                );
            }
            other => panic!("expected partial load, got {other:?}"),
        }
    }

    #[gpui::test]
    fn loader_provenance_marks_direct_unbind_as_bindings(cx: &mut App) {
        gpui::actions!(provenance_unbind_check, [DoThing]);
        let file = KeymapFile::parse(
            r#"[{"bindings": {"tab": ["zed::Unbind", "provenance_unbind_check::DoThing"]}}]"#,
        )
        .unwrap();
        let (result, provenances) = file.load_keymap_with_locations(cx);
        let bindings = match result {
            crate::keymap_file::KeymapFileLoadResult::Success { key_bindings } => key_bindings,
            other => panic!("expected success, got {other:?}"),
        };
        assert_eq!(bindings.len(), 1);
        assert!(gpui::is_unbind(bindings[0].action()));
        assert_eq!(provenances.len(), 1);
        assert_eq!(
            provenances[0].location.collection,
            KeymapEntryCollection::Bindings
        );
    }

    #[gpui::test]
    fn provenance_revision_changes_with_contents(cx: &mut App) {
        let before =
            KeymapFile::parse(r#"[{"bindings": {"a": "test_keymap_file::StringAction"}}]"#)
                .unwrap();
        let after = KeymapFile::parse(
            r#"[{"bindings": {"a": "test_keymap_file::StringAction"}}, {"bindings": {"b": "test_keymap_file::StringAction"}}]"#,
        )
        .unwrap();
        let (_, before_provenances) = before.load_keymap_with_locations(cx);
        let (_, after_provenances) = after.load_keymap_with_locations(cx);
        assert_ne!(
            before_provenances[0].revision, after_provenances[0].revision,
            "inserting a section must invalidate outstanding provenances"
        );
        // Cosmetic-only differences (whitespace/comments) share a revision.
        let spaced = KeymapFile::parse(
            "[ { \"bindings\" : { \"a\" : \"test_keymap_file::StringAction\" } } ]",
        )
        .unwrap();
        let (_, spaced_provenances) = spaced.load_keymap_with_locations(cx);
        assert_eq!(
            before_provenances[0].revision,
            spaced_provenances[0].revision
        );
    }

    #[gpui::test]
    fn remove_entries_by_provenance_keeps_siblings_and_drops_empty_sections(cx: &mut App) {
        // Remove one unbind entry while keeping the sibling binding.
        let input = r#"[
            {"bindings": {"tab": "test_keymap_file::StringAction"}},
            {"unbind": {"tab": "test_keymap_file::StringAction"}}
        ]"#;
        let file = KeymapFile::parse(input).unwrap();
        let (_, provenances) = file.load_keymap_with_locations(cx);
        let unbind = provenances
            .iter()
            .find(|p| p.location.collection == KeymapEntryCollection::Unbind)
            .copied()
            .expect("unbind provenance");
        check_remove_entries_json(
            input,
            vec![unbind],
            r#"[{"bindings": {"tab": "test_keymap_file::StringAction"}}]"#,
        );

        // Same-section case-sensitive-distinct keys: only "Tab" is removed.
        let input = r#"[{"bindings": {"tab": "test_keymap_file::StringAction", "Tab": ["zed::Unbind", "test_keymap_file::StringAction"]}}]"#;
        let file = KeymapFile::parse(input).unwrap();
        let (_, provenances) = file.load_keymap_with_locations(cx);
        let tab = provenances
            .iter()
            .find(|p| p.location.entry_index == 1)
            .copied()
            .expect("second-entry provenance");
        check_remove_entries_json(
            input,
            vec![tab],
            r#"[{"bindings": {"tab": "test_keymap_file::StringAction"}}]"#,
        );
    }

    #[gpui::test]
    fn remove_entries_rejects_stale_provenance(cx: &mut App) {
        // Out-of-range coordinates still fail.
        let input = r#"[{"bindings": {"tab": "test_keymap_file::StringAction"}}]"#;
        let file = KeymapFile::parse(input).unwrap();
        let (_, provenances) = file.load_keymap_with_locations(cx);
        let mut out_of_range = provenances[0];
        out_of_range.location.section_index = 5;
        let result = KeymapFile::update_keybinding(
            KeybindUpdateOperation::RemoveEntries {
                provenances: vec![out_of_range],
            },
            input.to_string(),
            2,
            &DummyKeyboardMapper,
            &HashMap::default(),
        );
        assert!(result.is_err());

        // The coordinate-collision attack from review: record section 1,
        // unbind, entry 0, then insert a section before it. The same numeric
        // position now holds an unrelated declaration; the revision check
        // must fail instead of deleting it.
        let before = r#"[
            {"bindings": {"a": "test_keymap_file::StringAction"}},
            {"unbind": {"victim": "test_keymap_file::StringAction"}}
        ]"#;
        let file = KeymapFile::parse(before).unwrap();
        let (_, provenances) = file.load_keymap_with_locations(cx);
        let recorded = provenances
            .iter()
            .find(|p| p.location.section_index == 1)
            .copied()
            .expect("section-1 provenance");
        let after_insert = r#"[
            {"bindings": {"inserted": "test_keymap_file::StringAction"}},
            {"bindings": {"a": "test_keymap_file::StringAction"}},
            {"unbind": {"victim": "test_keymap_file::StringAction"}}
        ]"#;
        let err = KeymapFile::update_keybinding(
            KeybindUpdateOperation::RemoveEntries {
                provenances: vec![recorded],
            },
            after_insert.to_string(),
            2,
            &DummyKeyboardMapper,
            &HashMap::default(),
        )
        .expect_err("stale provenance must not delete the declaration now at section 1");
        assert!(
            err.to_string().contains("Stale restore provenance"),
            "unexpected error (revision check must fire first): {err}"
        );
    }

    fn check_remove_entries_json(
        input: &str,
        provenances: Vec<KeymapEntryProvenance>,
        expected: &str,
    ) {
        let result = KeymapFile::update_keybinding(
            KeybindUpdateOperation::RemoveEntries { provenances },
            input.to_string(),
            2,
            &DummyKeyboardMapper,
            &HashMap::default(),
        )
        .expect("RemoveEntries should succeed");
        assert_json_eq(input, &result, expected);
    }

    /// Shared whole-file JSON comparison for keymap persistence tests.
    ///
    /// Parses both sides to `serde_json::Value` so formatting/ordering
    /// differences don't mask or fake a pass. (The older
    /// `check_keymap_update` helper above compares exact strings, which is
    /// stricter about formatting; both assert the complete file, per
    /// review, rather than substring-matching.)
    #[track_caller]
    fn assert_json_eq(input: &str, result: &str, expected: &str) {
        let result_value: Value =
            serde_json::from_str(result).expect("result should be valid JSON");
        let expected_value: Value =
            serde_json::from_str(expected).expect("expected should be valid JSON");
        assert_eq!(
            result_value, expected_value,
            "\ninput: {input}\nresult: {result}\nexpected: {expected}"
        );
    }

    fn load_partial_bindings(
        content: &str,
        cx: &App,
    ) -> (Vec<gpui::KeyBinding>, Vec<KeymapEntryProvenance>) {
        let file = KeymapFile::parse(content).expect("parse should succeed");
        let (result, provenances) = file.load_keymap_with_locations(cx);
        match result {
            crate::keymap_file::KeymapFileLoadResult::SomeFailedToLoad { key_bindings, .. } => {
                (key_bindings, provenances)
            }
            other => panic!("expected partial load, got {other:?}"),
        }
    }

    #[gpui::test]
    fn remove_entries_preserves_unrecognized_fields(cx: &mut App) {
        // Loader-rejected declarations must survive restore: the valid
        // suppressor is removed, `invalid_field` stays.
        let input = r#"[{"unbind":{"tab":"test_keymap_file::StringAction"},"invalid_field":true}]"#;
        let (bindings, provenances) = load_partial_bindings(input, cx);
        assert_eq!(bindings.len(), 1);
        assert_eq!(provenances.len(), 1);
        assert_eq!(provenances[0].location.section_index, 0);
        check_remove_entries_json(input, provenances, r#"[{"invalid_field":true}]"#);

        // Fingerprint covers unrecognized fields: a provenance issued before
        // `invalid_field` appeared must not apply after it is added.
        let before = r#"[{"unbind":{"tab":"test_keymap_file::StringAction"}}]"#;
        let file = KeymapFile::parse(before).unwrap();
        let (_, before_provenances) = file.load_keymap_with_locations(cx);
        assert_eq!(before_provenances.len(), 1);
        let after = r#"[{"unbind":{"tab":"test_keymap_file::StringAction"},"invalid_field":true}]"#;
        let err = KeymapFile::update_keybinding(
            KeybindUpdateOperation::RemoveEntries {
                provenances: before_provenances,
            },
            after.to_string(),
            2,
            &DummyKeyboardMapper,
            &HashMap::default(),
        )
        .expect_err("unrecognized-field change must invalidate provenance");
        assert!(
            err.to_string().contains("Stale restore provenance"),
            "unexpected error: {err}"
        );
    }
}
