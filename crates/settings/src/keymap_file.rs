use std::rc::Rc;

use crate::{settings_store::parse_json_with_comments, SettingsAssets};
use anyhow::anyhow;
use collections::{HashMap, IndexMap};
use gpui::{
    Action, ActionBuildError, AppContext, InvalidKeystrokeError, KeyBinding,
    KeyBindingContextPredicate, NoAction, SharedString, KEYSTROKE_PARSE_EXPECTED_MESSAGE,
};
use schemars::{
    gen::{SchemaGenerator, SchemaSettings},
    schema::{ArrayValidation, InstanceType, Schema, SchemaObject, SubschemaValidation},
    JsonSchema,
};
use serde::Deserialize;
use serde_json::Value;
use std::fmt::Write;
use util::{asset_str, markdown::MarkdownString};

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
    context: String,
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
    fn schema_name() -> String {
        "KeymapAction".into()
    }

    /// This schema will be replaced with the full action schema in
    /// `KeymapFile::generate_json_schema`.
    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        Schema::Bool(true)
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
    AllFailedToLoad {
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

    pub fn load_asset(asset_path: &str, cx: &AppContext) -> anyhow::Result<Vec<KeyBinding>> {
        match Self::load(asset_str::<SettingsAssets>(asset_path).as_ref(), cx) {
            KeymapFileLoadResult::Success { key_bindings, .. } => Ok(key_bindings),
            KeymapFileLoadResult::SomeFailedToLoad { error_message, .. }
            | KeymapFileLoadResult::AllFailedToLoad { error_message } => Err(anyhow!(
                "Error loading built-in keymap \"{asset_path}\": {error_message}"
            )),
            KeymapFileLoadResult::JsonParseFailure { error } => Err(anyhow!(
                "JSON parse error in built-in keymap \"{asset_path}\": {error}"
            )),
        }
    }

    #[cfg(feature = "test-support")]
    pub fn load_asset_allow_partial_failure(
        asset_path: &str,
        cx: &AppContext,
    ) -> anyhow::Result<Vec<KeyBinding>> {
        match Self::load(asset_str::<SettingsAssets>(asset_path).as_ref(), cx) {
            KeymapFileLoadResult::Success { key_bindings, .. }
            | KeymapFileLoadResult::SomeFailedToLoad { key_bindings, .. } => Ok(key_bindings),
            KeymapFileLoadResult::AllFailedToLoad { error_message } => Err(anyhow!(
                "Error loading built-in keymap \"{asset_path}\": {error_message}"
            )),
            KeymapFileLoadResult::JsonParseFailure { error } => Err(anyhow!(
                "JSON parse error in built-in keymap \"{asset_path}\": {error}"
            )),
        }
    }

    #[cfg(feature = "test-support")]
    pub fn load_panic_on_failure(content: &str, cx: &AppContext) -> Vec<KeyBinding> {
        match Self::load(content, cx) {
            KeymapFileLoadResult::Success { key_bindings } => key_bindings,
            KeymapFileLoadResult::SomeFailedToLoad { error_message, .. }
            | KeymapFileLoadResult::AllFailedToLoad { error_message, .. } => {
                panic!("{error_message}");
            }
            KeymapFileLoadResult::JsonParseFailure { error } => {
                panic!("JSON parse error: {error}");
            }
        }
    }

    pub fn load(content: &str, cx: &AppContext) -> KeymapFileLoadResult {
        let key_equivalents = crate::key_equivalents::get_key_equivalents(&cx.keyboard_layout());

        if content.is_empty() {
            return KeymapFileLoadResult::Success {
                key_bindings: Vec::new(),
            };
        }
        let keymap_file = match parse_json_with_comments::<Self>(content) {
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
                    MarkdownString::inline_code(&format!("{:?}", unrecognized_fields.keys()))
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
                            write!(
                                section_errors,
                                "\n\n - In binding {}, {err}",
                                inline_code_string(keystrokes),
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
                    write!(error_message, "\n\nIn section without context predicate:").unwrap()
                } else {
                    write!(
                        error_message,
                        "\n\nIn section with {}:",
                        MarkdownString::inline_code(&format!("context = \"{}\"", context))
                    )
                    .unwrap()
                }
                write!(error_message, "{section_errors}").unwrap();
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
        cx: &AppContext,
    ) -> std::result::Result<KeyBinding, String> {
        let (build_result, action_input_string) = match &action.0 {
            Value::Array(items) => {
                if items.len() != 2 {
                    return Err(format!(
                        "expected two-element array of `[name, input]`. \
                        Instead found {}.",
                        MarkdownString::inline_code(&action.0.to_string())
                    ));
                }
                let serde_json::Value::String(ref name) = items[0] else {
                    return Err(format!(
                        "expected two-element array of `[name, input]`, \
                        but the first element is not a string in {}.",
                        MarkdownString::inline_code(&action.0.to_string())
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
                    MarkdownString::inline_code(&action.0.to_string())
                ));
            }
        };

        let action = match build_result {
            Ok(action) => action,
            Err(ActionBuildError::NotFound { name }) => {
                return Err(format!(
                    "didn't find an action named {}.",
                    inline_code_string(&name)
                ))
            }
            Err(ActionBuildError::BuildError { name, error }) => match action_input_string {
                Some(action_input_string) => {
                    return Err(format!(
                        "can't build {} action from input value {}: {}",
                        inline_code_string(&name),
                        MarkdownString::inline_code(&action_input_string),
                        MarkdownString::escape(&error.to_string())
                    ))
                }
                None => {
                    return Err(format!(
                        "can't build {} action - it requires input data via [name, input]: {}",
                        inline_code_string(&name),
                        MarkdownString::escape(&error.to_string())
                    ))
                }
            },
        };

        match KeyBinding::load(keystrokes, action, context, key_equivalents) {
            Ok(binding) => Ok(binding),
            Err(InvalidKeystrokeError { keystroke }) => Err(format!(
                "invalid keystroke {}. {}",
                inline_code_string(&keystroke),
                KEYSTROKE_PARSE_EXPECTED_MESSAGE
            )),
        }
    }

    pub fn generate_json_schema_for_registered_actions(cx: &mut AppContext) -> Value {
        let mut generator = SchemaSettings::draft07()
            .with(|settings| settings.option_add_null_type = false)
            .into_generator();

        let action_schemas = cx.action_schemas(&mut generator);
        let deprecations = cx.action_deprecations();
        KeymapFile::generate_json_schema(generator, action_schemas, deprecations)
    }

    fn generate_json_schema(
        generator: SchemaGenerator,
        action_schemas: Vec<(SharedString, Option<Schema>)>,
        deprecations: &HashMap<SharedString, SharedString>,
    ) -> serde_json::Value {
        fn set<I, O>(input: I) -> Option<O>
        where
            I: Into<O>,
        {
            Some(input.into())
        }

        fn add_deprecation(schema_object: &mut SchemaObject, message: String) {
            schema_object.extensions.insert(
                // deprecationMessage is not part of the JSON Schema spec,
                // but json-language-server recognizes it.
                "deprecationMessage".to_owned(),
                Value::String(message),
            );
        }

        fn add_deprecation_preferred_name(schema_object: &mut SchemaObject, new_name: &str) {
            add_deprecation(schema_object, format!("Deprecated, use {new_name}"));
        }

        fn add_description(schema_object: &mut SchemaObject, description: String) {
            schema_object
                .metadata
                .get_or_insert(Default::default())
                .description = Some(description);
        }

        let empty_object: SchemaObject = SchemaObject {
            instance_type: set(InstanceType::Object),
            ..Default::default()
        };

        // This is a workaround for a json-language-server issue where it matches the first
        // alternative that matches the value's shape and uses that for documentation.
        //
        // In the case of the array validations, it would even provide an error saying that the name
        // must match the name of the first alternative.
        let mut plain_action = SchemaObject {
            instance_type: set(InstanceType::String),
            const_value: Some(Value::String("".to_owned())),
            ..Default::default()
        };
        let no_action_message = "No action named this.";
        add_description(&mut plain_action, no_action_message.to_owned());
        add_deprecation(&mut plain_action, no_action_message.to_owned());
        let mut matches_action_name = SchemaObject {
            const_value: Some(Value::String("".to_owned())),
            ..Default::default()
        };
        let no_action_message = "No action named this that takes input.";
        add_description(&mut matches_action_name, no_action_message.to_owned());
        add_deprecation(&mut matches_action_name, no_action_message.to_owned());
        let action_with_input = SchemaObject {
            instance_type: set(InstanceType::Array),
            array: set(ArrayValidation {
                items: set(vec![
                    matches_action_name.into(),
                    // Accept any value, as we want this to be the preferred match when there is a
                    // typo in the name.
                    Schema::Bool(true),
                ]),
                min_items: Some(2),
                max_items: Some(2),
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut keymap_action_alternatives = vec![plain_action.into(), action_with_input.into()];

        for (name, action_schema) in action_schemas.iter() {
            let schema = if let Some(Schema::Object(schema)) = action_schema {
                Some(schema.clone())
            } else {
                None
            };

            let description = schema.as_ref().and_then(|schema| {
                schema
                    .metadata
                    .as_ref()
                    .and_then(|metadata| metadata.description.clone())
            });

            let deprecation = if name == NoAction.name() {
                Some("null")
            } else {
                deprecations.get(name).map(|new_name| new_name.as_ref())
            };

            // Add an alternative for plain action names.
            let mut plain_action = SchemaObject {
                instance_type: set(InstanceType::String),
                const_value: Some(Value::String(name.to_string())),
                ..Default::default()
            };
            if let Some(new_name) = deprecation {
                add_deprecation_preferred_name(&mut plain_action, new_name);
            }
            if let Some(description) = description.clone() {
                add_description(&mut plain_action, description);
            }
            keymap_action_alternatives.push(plain_action.into());

            // Add an alternative for actions with data specified as a [name, data] array.
            //
            // When a struct with no deserializable fields is added with impl_actions! /
            // impl_actions_as! an empty object schema is produced. The action should be invoked
            // without data in this case.
            if let Some(schema) = schema {
                if schema != empty_object {
                    let mut matches_action_name = SchemaObject {
                        const_value: Some(Value::String(name.to_string())),
                        ..Default::default()
                    };
                    if let Some(description) = description.clone() {
                        add_description(&mut matches_action_name, description.to_string());
                    }
                    if let Some(new_name) = deprecation {
                        add_deprecation_preferred_name(&mut matches_action_name, new_name);
                    }
                    let action_with_input = SchemaObject {
                        instance_type: set(InstanceType::Array),
                        array: set(ArrayValidation {
                            items: set(vec![matches_action_name.into(), schema.into()]),
                            min_items: Some(2),
                            max_items: Some(2),
                            ..Default::default()
                        }),
                        ..Default::default()
                    };
                    keymap_action_alternatives.push(action_with_input.into());
                }
            }
        }

        // Placing null first causes json-language-server to default assuming actions should be
        // null, so place it last.
        keymap_action_alternatives.push(
            SchemaObject {
                instance_type: set(InstanceType::Null),
                ..Default::default()
            }
            .into(),
        );

        let action_schema = SchemaObject {
            subschemas: set(SubschemaValidation {
                one_of: Some(keymap_action_alternatives),
                ..Default::default()
            }),
            ..Default::default()
        }
        .into();

        // The `KeymapSection` schema will reference the `KeymapAction` schema by name, so replacing
        // the definition of `KeymapAction` results in the full action schema being used.
        let mut root_schema = generator.into_root_schema_for::<KeymapFile>();
        root_schema
            .definitions
            .insert(KeymapAction::schema_name(), action_schema);

        // This and other json schemas can be viewed via `debug: open language server logs` ->
        // `json-language-server` -> `Server Info`.
        serde_json::to_value(root_schema).unwrap()
    }

    pub fn sections(&self) -> impl DoubleEndedIterator<Item = &KeymapSection> {
        self.0.iter()
    }
}

// Double quotes a string and wraps it in backticks for markdown inline code..
fn inline_code_string(text: &str) -> MarkdownString {
    MarkdownString::inline_code(&format!("\"{}\"", text))
}

#[cfg(test)]
mod tests {
    use crate::KeymapFile;

    #[test]
    fn can_deserialize_keymap_with_trailing_comma() {
        let json = indoc::indoc! {"[
              // Standard macOS bindings
              {
                \"bindings\": {
                  \"up\": \"menu::SelectPrev\",
                },
              },
            ]
                  "
        };
        KeymapFile::parse(json).unwrap();
    }
}
