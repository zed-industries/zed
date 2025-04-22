use collections::HashMap;
use schemars::{
    JsonSchema,
    r#gen::SchemaSettings,
    schema::{ObjectValidation, Schema, SchemaObject},
};
use serde::Deserialize;
use serde_json_lenient::Value;

#[derive(Deserialize)]
pub struct VsSnippetsFile {
    #[serde(flatten)]
    pub(crate) snippets: HashMap<String, VsCodeSnippet>,
}

impl VsSnippetsFile {
    pub fn generate_json_schema() -> Value {
        let schema = SchemaSettings::draft07()
            .with(|settings| settings.option_add_null_type = false)
            .into_generator()
            .into_root_schema_for::<Self>();

        serde_json_lenient::to_value(schema).unwrap()
    }
}

impl JsonSchema for VsSnippetsFile {
    fn schema_name() -> String {
        "VsSnippetsFile".into()
    }

    fn json_schema(r#gen: &mut schemars::r#gen::SchemaGenerator) -> Schema {
        SchemaObject {
            object: Some(Box::new(ObjectValidation {
                additional_properties: Some(Box::new(r#gen.subschema_for::<VsCodeSnippet>())),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }
}

#[derive(Deserialize, JsonSchema)]
#[serde(untagged)]
pub(crate) enum ListOrDirect {
    Single(String),
    List(Vec<String>),
}

impl From<ListOrDirect> for Vec<String> {
    fn from(list: ListOrDirect) -> Self {
        match list {
            ListOrDirect::Single(entry) => vec![entry],
            ListOrDirect::List(entries) => entries,
        }
    }
}

impl std::fmt::Display for ListOrDirect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Single(v) => v.to_owned(),
                Self::List(v) => v.join("\n"),
            }
        )
    }
}

#[derive(Deserialize, JsonSchema)]
pub(crate) struct VsCodeSnippet {
    /// The snippet prefix used to decide whether a completion menu should be shown.
    pub(crate) prefix: Option<ListOrDirect>,

    /// The snippet content. Use `$1` and `${1:defaultText}` to define cursor positions and `$0` for final cursor position.
    pub(crate) body: ListOrDirect,

    /// The snippet description displayed inside the completion menu.
    pub(crate) description: Option<ListOrDirect>,
}
