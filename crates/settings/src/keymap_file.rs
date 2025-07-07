use anyhow::{Context as _, Result};
use collections::{BTreeMap, HashMap, IndexMap};
use fs::Fs;
use gpui::{
    Action, ActionBuildError, App, InvalidKeystrokeError, KEYSTROKE_PARSE_EXPECTED_MESSAGE,
    KeyBinding, KeyBindingContextPredicate, KeyBindingMetaIndex, Keystroke, NoAction, SharedString,
};
use schemars::{JsonSchema, json_schema};
use serde::Deserialize;
use serde_json::{Value, json};
use std::borrow::Cow;
use std::{any::TypeId, fmt::Write, rc::Rc, sync::Arc, sync::LazyLock};
use util::{
    asset_str,
    markdown::{MarkdownEscaped, MarkdownInlineCode, MarkdownString},
};

use crate::{
    SettingsAssets, append_top_level_array_value_in_json_text, parse_json_with_comments,
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

    #[cfg(feature = "test-support")]
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
        let key_equivalents =
            crate::key_equivalents::get_key_equivalents(cx.keyboard_layout().id());

        if content.is_empty() {
            return KeymapFileLoadResult::Success {
                key_bindings: Vec::new(),
            };
        }
        let keymap_file = match Self::parse(content) {
            Ok(keymap_file) => keymap_file,
            Err(error) => {
                return KeymapFileLoadResult::JsonParseFailure { error };
            }
        };

        // Accumulate errors in order to support partial load of user keymap in the presence of
        // errors in context and binding parsing.
        let mut errors = Vec::new();
        let mut key_bindings = Vec::new();

        for KeymapSection {
            context,
            use_key_equivalents,
            bindings,
            unrecognized_fields,
        } in keymap_file.0.iter()
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
                            context,
                            format!(" Parse error in section `context` field: {}", err),
                        ));
                        continue;
                    }
                }
            };

            let key_equivalents = if *use_key_equivalents {
                key_equivalents.as_ref()
            } else {
                None
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

            if let Some(bindings) = bindings {
                for (keystrokes, action) in bindings {
                    let result = Self::load_keybinding(
                        keystrokes,
                        action,
                        context_predicate.clone(),
                        key_equivalents,
                        cx,
                    );
                    match result {
                        Ok(key_binding) => {
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
                errors.push((context, section_errors))
            }
        }

        if errors.is_empty() {
            KeymapFileLoadResult::Success { key_bindings }
        } else {
            let mut error_message = "Errors in user keymap file.\n".to_owned();
            for (context, section_errors) in errors {
                if context.is_empty() {
                    let _ = write!(error_message, "\n\nIn section without context predicate:");
                } else {
                    let _ = write!(
                        error_message,
                        "\n\nIn section with {}:",
                        MarkdownInlineCode(&format!("context = \"{}\"", context))
                    );
                }
                let _ = write!(error_message, "{section_errors}");
            }
            KeymapFileLoadResult::SomeFailedToLoad {
                key_bindings,
                error_message: MarkdownString(error_message),
            }
        }
    }

    fn load_keybinding(
        keystrokes: &str,
        action: &KeymapAction,
        context: Option<Rc<KeyBindingContextPredicate>>,
        key_equivalents: Option<&HashMap<char, char>>,
        cx: &App,
    ) -> std::result::Result<KeyBinding, String> {
        let (build_result, action_input_string) = match &action.0 {
            Value::Array(items) => {
                if items.len() != 2 {
                    return Err(format!(
                        "expected two-element array of `[name, input]`. \
                        Instead found {}.",
                        MarkdownInlineCode(&action.0.to_string())
                    ));
                }
                let serde_json::Value::String(ref name) = items[0] else {
                    return Err(format!(
                        "expected two-element array of `[name, input]`, \
                        but the first element is not a string in {}.",
                        MarkdownInlineCode(&action.0.to_string())
                    ));
                };
                let action_input = items[1].clone();
                let action_input_string = action_input.to_string();
                (
                    cx.build_action(&name, Some(action_input)),
                    Some(action_input_string),
                )
            }
            Value::String(name) => (cx.build_action(&name, None), None),
            Value::Null => (Ok(NoAction.boxed_clone()), None),
            _ => {
                return Err(format!(
                    "expected two-element array of `[name, input]`. \
                    Instead found {}.",
                    MarkdownInlineCode(&action.0.to_string())
                ));
            }
        };

        let action = match build_result {
            Ok(action) => action,
            Err(ActionBuildError::NotFound { name }) => {
                return Err(format!(
                    "didn't find an action named {}.",
                    MarkdownInlineCode(&format!("\"{}\"", &name))
                ));
            }
            Err(ActionBuildError::BuildError { name, error }) => match action_input_string {
                Some(action_input_string) => {
                    return Err(format!(
                        "can't build {} action from input value {}: {}",
                        MarkdownInlineCode(&format!("\"{}\"", &name)),
                        MarkdownInlineCode(&action_input_string),
                        MarkdownEscaped(&error.to_string())
                    ));
                }
                None => {
                    return Err(format!(
                        "can't build {} action - it requires input data via [name, input]: {}",
                        MarkdownInlineCode(&format!("\"{}\"", &name)),
                        MarkdownEscaped(&error.to_string())
                    ));
                }
            },
        };

        let key_binding = match KeyBinding::load(
            keystrokes,
            action,
            context,
            key_equivalents,
            action_input_string.map(SharedString::from),
        ) {
            Ok(key_binding) => key_binding,
            Err(InvalidKeystrokeError { keystroke }) => {
                return Err(format!(
                    "invalid keystroke {}. {}",
                    MarkdownInlineCode(&format!("\"{}\"", &keystroke)),
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

    pub fn generate_json_schema_for_registered_actions(cx: &mut App) -> Value {
        // instead of using DefaultDenyUnknownFields, actions typically use
        // `#[serde(deny_unknown_fields)]` so that these cases are reported as parse failures. This
        // is because the rest of the keymap will still load in these cases, whereas other settings
        // files would not.
        let mut generator = schemars::generate::SchemaSettings::draft2019_09().into_generator();

        let action_schemas = cx.action_schemas(&mut generator);
        let deprecations = cx.deprecated_actions_to_preferred_actions();
        let deprecation_messages = cx.action_deprecation_messages();
        KeymapFile::generate_json_schema(
            generator,
            action_schemas,
            deprecations,
            deprecation_messages,
        )
    }

    fn generate_json_schema(
        mut generator: schemars::SchemaGenerator,
        action_schemas: Vec<(&'static str, Option<schemars::Schema>)>,
        deprecations: &HashMap<&'static str, &'static str>,
        deprecation_messages: &HashMap<&'static str, &'static str>,
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

        fn add_description(schema: &mut schemars::Schema, description: String) {
            schema.insert("description".to_string(), Value::String(description));
        }

        let empty_object = json_schema!({
            "type": "object"
        });

        // This is a workaround for a json-language-server issue where it matches the first
        // alternative that matches the value's shape and uses that for documentation.
        //
        // In the case of the array validations, it would even provide an error saying that the name
        // must match the name of the first alternative.
        let mut plain_action = json_schema!({
            "type": "string",
            "const": ""
        });
        let no_action_message = "No action named this.";
        add_description(&mut plain_action, no_action_message.to_owned());
        add_deprecation(&mut plain_action, no_action_message.to_owned());

        let mut matches_action_name = json_schema!({
            "const": ""
        });
        let no_action_message_input = "No action named this that takes input.";
        add_description(&mut matches_action_name, no_action_message_input.to_owned());
        add_deprecation(&mut matches_action_name, no_action_message_input.to_owned());

        let action_with_input = json_schema!({
            "type": "array",
            "items": [
                matches_action_name,
                true
            ],
            "minItems": 2,
            "maxItems": 2
        });
        let mut keymap_action_alternatives = vec![plain_action, action_with_input];

        for (name, action_schema) in action_schemas.into_iter() {
            let description = action_schema.as_ref().and_then(|schema| {
                schema
                    .as_object()
                    .and_then(|obj| obj.get("description"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            });

            let deprecation = if name == NoAction.name() {
                Some("null")
            } else {
                deprecations.get(name).copied()
            };

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
            if let Some(desc) = description.clone() {
                add_description(&mut plain_action, desc);
            }
            keymap_action_alternatives.push(plain_action);

            // Add an alternative for actions with data specified as a [name, data] array.
            //
            // When a struct with no deserializable fields is added by deriving `Action`, an empty
            // object schema is produced. The action should be invoked without data in this case.
            if let Some(schema) = action_schema {
                if schema != empty_object {
                    let mut matches_action_name = json_schema!({
                        "const": name
                    });
                    if let Some(desc) = description.clone() {
                        add_description(&mut matches_action_name, desc);
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
                    keymap_action_alternatives.push(action_with_input);
                }
            }
        }

        // Placing null first causes json-language-server to default assuming actions should be
        // null, so place it last.
        keymap_action_alternatives.push(json_schema!({
            "type": "null"
        }));

        // The `KeymapSection` schema will reference the `KeymapAction` schema by name, so setting
        // the definition of `KeymapAction` results in the full action schema being used.
        generator.definitions_mut().insert(
            KeymapAction::schema_name().to_string(),
            json!({
                "oneOf": keymap_action_alternatives
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
                if let Some(e) = err.downcast_ref::<std::io::Error>() {
                    if e.kind() == std::io::ErrorKind::NotFound {
                        return Ok(crate::initial_keymap_content().to_string());
                    }
                }
                Err(err)
            }
        }
    }

    pub fn update_keybinding<'a>(
        mut operation: KeybindUpdateOperation<'a>,
        mut keymap_contents: String,
        tab_size: usize,
    ) -> Result<String> {
        // if trying to replace a keybinding that is not user-defined, treat it as an add operation
        match operation {
            KeybindUpdateOperation::Replace {
                target_keybind_source: target_source,
                source,
                ..
            } if target_source != KeybindSource::User => {
                operation = KeybindUpdateOperation::Add(source);
            }
            _ => {}
        }

        // Sanity check that keymap contents are valid, even though we only use it for Replace.
        // We don't want to modify the file if it's invalid.
        let keymap = Self::parse(&keymap_contents).context("Failed to parse keymap")?;

        if let KeybindUpdateOperation::Replace { source, target, .. } = operation {
            let mut found_index = None;
            let target_action_value = target
                .action_value()
                .context("Failed to generate target action JSON value")?;
            let source_action_value = source
                .action_value()
                .context("Failed to generate source action JSON value")?;
            'sections: for (index, section) in keymap.sections().enumerate() {
                if section.context != target.context.unwrap_or("") {
                    continue;
                }
                if section.use_key_equivalents != target.use_key_equivalents {
                    continue;
                }
                let Some(bindings) = &section.bindings else {
                    continue;
                };
                for (keystrokes, action) in bindings {
                    let Ok(keystrokes) = keystrokes
                        .split_whitespace()
                        .map(Keystroke::parse)
                        .collect::<Result<Vec<_>, _>>()
                    else {
                        continue;
                    };
                    if keystrokes.len() != target.keystrokes.len()
                        || !keystrokes
                            .iter()
                            .zip(target.keystrokes)
                            .all(|(a, b)| a.should_match(b))
                    {
                        continue;
                    }
                    if action.0 != target_action_value {
                        continue;
                    }
                    found_index = Some(index);
                    break 'sections;
                }
            }

            if let Some(index) = found_index {
                if target.context == source.context {
                    // if we are only changing the keybinding (common case)
                    // not the context, etc. Then just update the binding in place

                    let (replace_range, replace_value) =
                        replace_top_level_array_value_in_json_text(
                            &keymap_contents,
                            &["bindings", &target.keystrokes_unparsed()],
                            Some(&source_action_value),
                            Some(&source.keystrokes_unparsed()),
                            index,
                            tab_size,
                        )
                        .context("Failed to replace keybinding")?;
                    keymap_contents.replace_range(replace_range, &replace_value);

                    return Ok(keymap_contents);
                } else if keymap.0[index]
                    .bindings
                    .as_ref()
                    .map_or(true, |bindings| bindings.len() == 1)
                {
                    // if we are replacing the only binding in the section,
                    // just update the section in place, updating the context
                    // and the binding

                    let (replace_range, replace_value) =
                        replace_top_level_array_value_in_json_text(
                            &keymap_contents,
                            &["bindings", &target.keystrokes_unparsed()],
                            Some(&source_action_value),
                            Some(&source.keystrokes_unparsed()),
                            index,
                            tab_size,
                        )
                        .context("Failed to replace keybinding")?;
                    keymap_contents.replace_range(replace_range, &replace_value);

                    let (replace_range, replace_value) =
                        replace_top_level_array_value_in_json_text(
                            &keymap_contents,
                            &["context"],
                            source.context.map(Into::into).as_ref(),
                            None,
                            index,
                            tab_size,
                        )
                        .context("Failed to replace keybinding")?;
                    keymap_contents.replace_range(replace_range, &replace_value);
                    return Ok(keymap_contents);
                } else {
                    // if we are replacing one of multiple bindings in a section
                    // with a context change, remove the existing binding from the
                    // section, then treat this operation as an add operation of the
                    // new binding with the updated context.

                    let (replace_range, replace_value) =
                        replace_top_level_array_value_in_json_text(
                            &keymap_contents,
                            &["bindings", &target.keystrokes_unparsed()],
                            None,
                            None,
                            index,
                            tab_size,
                        )
                        .context("Failed to replace keybinding")?;
                    keymap_contents.replace_range(replace_range, &replace_value);
                    operation = KeybindUpdateOperation::Add(source);
                }
            } else {
                log::warn!(
                    "Failed to find keybinding to update `{:?} -> {}` creating new binding for `{:?} -> {}` instead",
                    target.keystrokes,
                    target_action_value,
                    source.keystrokes,
                    source_action_value,
                );
                operation = KeybindUpdateOperation::Add(source);
            }
        }

        if let KeybindUpdateOperation::Add(keybinding) = operation {
            let mut value = serde_json::Map::with_capacity(4);
            if let Some(context) = keybinding.context {
                value.insert("context".to_string(), context.into());
            }
            if keybinding.use_key_equivalents {
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
            )?;
            keymap_contents.replace_range(replace_range, &replace_value);
        }
        return Ok(keymap_contents);
    }
}

pub enum KeybindUpdateOperation<'a> {
    Replace {
        /// Describes the keybind to create
        source: KeybindUpdateTarget<'a>,
        /// Describes the keybind to remove
        target: KeybindUpdateTarget<'a>,
        target_keybind_source: KeybindSource,
    },
    Add(KeybindUpdateTarget<'a>),
}

pub struct KeybindUpdateTarget<'a> {
    pub context: Option<&'a str>,
    pub keystrokes: &'a [Keystroke],
    pub action_name: &'a str,
    pub use_key_equivalents: bool,
    pub input: Option<&'a str>,
}

impl<'a> KeybindUpdateTarget<'a> {
    fn action_value(&self) -> Result<Value> {
        let action_name: Value = self.action_name.into();
        let value = match self.input {
            Some(input) => {
                let input = serde_json::from_str::<Value>(input)
                    .context("Failed to parse action input as JSON")?;
                serde_json::json!([action_name, input])
            }
            None => action_name,
        };
        return Ok(value);
    }

    fn keystrokes_unparsed(&self) -> String {
        let mut keystrokes = String::with_capacity(self.keystrokes.len() * 8);
        for keystroke in self.keystrokes {
            keystrokes.push_str(&keystroke.unparse());
            keystrokes.push(' ');
        }
        keystrokes.pop();
        keystrokes
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum KeybindSource {
    User,
    Default,
    Base,
    Vim,
}

impl KeybindSource {
    const BASE: KeyBindingMetaIndex = KeyBindingMetaIndex(0);
    const DEFAULT: KeyBindingMetaIndex = KeyBindingMetaIndex(1);
    const VIM: KeyBindingMetaIndex = KeyBindingMetaIndex(2);
    const USER: KeyBindingMetaIndex = KeyBindingMetaIndex(3);

    pub fn name(&self) -> &'static str {
        match self {
            KeybindSource::User => "User",
            KeybindSource::Default => "Default",
            KeybindSource::Base => "Base",
            KeybindSource::Vim => "Vim",
        }
    }

    pub fn meta(&self) -> KeyBindingMetaIndex {
        match self {
            KeybindSource::User => Self::USER,
            KeybindSource::Default => Self::DEFAULT,
            KeybindSource::Base => Self::BASE,
            KeybindSource::Vim => Self::VIM,
        }
    }

    pub fn from_meta(index: KeyBindingMetaIndex) -> Self {
        match index {
            Self::USER => KeybindSource::User,
            Self::BASE => KeybindSource::Base,
            Self::DEFAULT => KeybindSource::Default,
            Self::VIM => KeybindSource::Vim,
            _ => unreachable!(),
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
        return source.meta();
    }
}

#[cfg(test)]
mod tests {
    use unindent::Unindent;

    use crate::{
        KeybindSource, KeymapFile,
        keymap_file::{KeybindUpdateOperation, KeybindUpdateTarget},
    };

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

    #[test]
    fn keymap_update() {
        use gpui::Keystroke;

        zlog::init_test();
        #[track_caller]
        fn check_keymap_update(
            input: impl ToString,
            operation: KeybindUpdateOperation,
            expected: impl ToString,
        ) {
            let result = KeymapFile::update_keybinding(operation, input.to_string(), 4)
                .expect("Update succeeded");
            pretty_assertions::assert_eq!(expected.to_string(), result);
        }

        #[track_caller]
        fn parse_keystrokes(keystrokes: &str) -> Vec<Keystroke> {
            return keystrokes
                .split(' ')
                .map(|s| Keystroke::parse(s).expect("Keystrokes valid"))
                .collect();
        }

        check_keymap_update(
            "[]",
            KeybindUpdateOperation::Add(KeybindUpdateTarget {
                keystrokes: &parse_keystrokes("ctrl-a"),
                action_name: "zed::SomeAction",
                context: None,
                use_key_equivalents: false,
                input: None,
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
            KeybindUpdateOperation::Add(KeybindUpdateTarget {
                keystrokes: &parse_keystrokes("ctrl-b"),
                action_name: "zed::SomeOtherAction",
                context: None,
                use_key_equivalents: false,
                input: None,
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
            KeybindUpdateOperation::Add(KeybindUpdateTarget {
                keystrokes: &parse_keystrokes("ctrl-b"),
                action_name: "zed::SomeOtherAction",
                context: None,
                use_key_equivalents: false,
                input: Some(r#"{"foo": "bar"}"#),
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
            KeybindUpdateOperation::Add(KeybindUpdateTarget {
                keystrokes: &parse_keystrokes("ctrl-b"),
                action_name: "zed::SomeOtherAction",
                context: Some("Zed > Editor && some_condition = true"),
                use_key_equivalents: true,
                input: Some(r#"{"foo": "bar"}"#),
            }),
            r#"[
                {
                    "bindings": {
                        "ctrl-a": "zed::SomeAction"
                    }
                },
                {
                    "context": "Zed > Editor && some_condition = true",
                    "use_key_equivalents": true,
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
                    use_key_equivalents: false,
                    input: None,
                },
                source: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("ctrl-b"),
                    action_name: "zed::SomeOtherAction",
                    context: None,
                    use_key_equivalents: false,
                    input: Some(r#"{"foo": "bar"}"#),
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
                    use_key_equivalents: false,
                    input: None,
                },
                source: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("ctrl-b"),
                    action_name: "zed::SomeOtherAction",
                    context: None,
                    use_key_equivalents: false,
                    input: Some(r#"{"foo": "bar"}"#),
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
                    use_key_equivalents: false,
                    input: None,
                },
                source: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("ctrl-b"),
                    action_name: "zed::SomeOtherAction",
                    context: None,
                    use_key_equivalents: false,
                    input: None,
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
                    use_key_equivalents: false,
                    input: None,
                },
                source: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("ctrl-b"),
                    action_name: "zed::SomeOtherAction",
                    context: None,
                    use_key_equivalents: false,
                    input: Some(r#"{"foo": "bar"}"#),
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
                    use_key_equivalents: false,
                    input: None,
                },
                source: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("c"),
                    action_name: "foo::baz",
                    context: Some("SomeOtherContext"),
                    use_key_equivalents: false,
                    input: None,
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
                    use_key_equivalents: false,
                    input: None,
                },
                source: KeybindUpdateTarget {
                    keystrokes: &parse_keystrokes("c"),
                    action_name: "foo::baz",
                    context: Some("SomeOtherContext"),
                    use_key_equivalents: false,
                    input: None,
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
    }
}
