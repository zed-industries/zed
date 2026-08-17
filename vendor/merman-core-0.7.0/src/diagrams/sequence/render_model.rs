use serde::{Deserialize, Serialize, Serializer};
use serde_json::{Map, Number, Value};
use std::collections::BTreeMap;

use super::{PLACEMENT_LEFT_OF, PLACEMENT_OVER, PLACEMENT_RIGHT_OF};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceDiagramRenderModel {
    #[serde(rename = "accTitle")]
    pub acc_title: Option<String>,
    #[serde(rename = "accDescr")]
    pub acc_descr: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "actorOrder")]
    pub actor_order: Vec<String>,
    pub actors: BTreeMap<String, SequenceActor>,
    #[serde(default)]
    pub boxes: Vec<SequenceBox>,
    pub messages: Vec<SequenceMessage>,
    #[serde(default)]
    pub notes: Vec<SequenceNote>,
    #[serde(rename = "createdActors", default)]
    pub created_actors: BTreeMap<String, usize>,
    #[serde(rename = "destroyedActors", default)]
    pub destroyed_actors: BTreeMap<String, usize>,
}

impl SequenceDiagramRenderModel {
    pub(crate) fn sanitize_common_db_fields(&mut self, config: &crate::MermaidConfig) {
        crate::common_db::sanitize_optional_title(&mut self.title, config);
        crate::common_db::sanitize_optional_acc_title(&mut self.acc_title, config);
        crate::common_db::sanitize_optional_acc_descr(&mut self.acc_descr, config);
    }

    pub(crate) fn to_compat_json(&self, diagram_type: &str) -> Value {
        let mut root = serde_json::Map::with_capacity(12);
        root.insert("type".to_string(), Value::String(diagram_type.to_string()));
        root.insert("title".to_string(), option_string_value(&self.title));
        root.insert("accTitle".to_string(), option_string_value(&self.acc_title));
        root.insert("accDescr".to_string(), option_string_value(&self.acc_descr));
        root.insert(
            "actorOrder".to_string(),
            string_array_value(&self.actor_order),
        );
        root.insert("actors".to_string(), actors_value(&self.actors));
        root.insert("messages".to_string(), messages_value(&self.messages));
        root.insert("notes".to_string(), notes_value(&self.notes));
        root.insert("boxes".to_string(), boxes_value(&self.boxes));
        root.insert(
            "createdActors".to_string(),
            usize_map_value(&self.created_actors),
        );
        root.insert(
            "destroyedActors".to_string(),
            usize_map_value(&self.destroyed_actors),
        );

        let mut placement = serde_json::Map::with_capacity(3);
        placement.insert(
            "leftOf".to_string(),
            Value::Number(PLACEMENT_LEFT_OF.into()),
        );
        placement.insert(
            "rightOf".to_string(),
            Value::Number(PLACEMENT_RIGHT_OF.into()),
        );
        placement.insert("over".to_string(), Value::Number(PLACEMENT_OVER.into()));
        let mut constants = serde_json::Map::with_capacity(1);
        constants.insert("placement".to_string(), Value::Object(placement));
        root.insert("constants".to_string(), Value::Object(constants));

        Value::Object(root)
    }
}

fn option_string_value(value: &Option<String>) -> Value {
    value
        .as_ref()
        .map(|value| Value::String(value.clone()))
        .unwrap_or(Value::Null)
}

fn string_array_value(values: &[String]) -> Value {
    Value::Array(values.iter().cloned().map(Value::String).collect())
}

fn usize_map_value(values: &BTreeMap<String, usize>) -> Value {
    let mut out = Map::new();
    for (key, value) in values {
        out.insert(key.clone(), Value::Number(Number::from(*value as u64)));
    }
    Value::Object(out)
}

fn actors_value(actors: &BTreeMap<String, SequenceActor>) -> Value {
    let mut out = Map::new();
    for (key, actor) in actors {
        let mut value = Map::new();
        value.insert("name".to_string(), Value::String(actor.name.clone()));
        value.insert(
            "description".to_string(),
            Value::String(actor.description.clone()),
        );
        value.insert("type".to_string(), Value::String(actor.actor_type.clone()));
        value.insert("wrap".to_string(), Value::Bool(actor.wrap));
        value.insert("links".to_string(), Value::Object(actor.links.clone()));
        value.insert(
            "properties".to_string(),
            Value::Object(actor.properties.clone()),
        );
        out.insert(key.clone(), Value::Object(value));
    }
    Value::Object(out)
}

fn messages_value(messages: &[SequenceMessage]) -> Value {
    Value::Array(messages.iter().map(message_value).collect())
}

fn message_value(message: &SequenceMessage) -> Value {
    let mut out = Map::new();
    out.insert("id".to_string(), Value::String(message.id.clone()));
    out.insert("from".to_string(), option_string_value(&message.from));
    out.insert("to".to_string(), option_string_value(&message.to));
    out.insert(
        "type".to_string(),
        Value::Number(Number::from(message.message_type)),
    );
    out.insert(
        "message".to_string(),
        message_payload_value(&message.message),
    );
    out.insert("wrap".to_string(), Value::Bool(message.wrap));
    out.insert("activate".to_string(), Value::Bool(message.activate));
    if let Some(placement) = message.placement {
        out.insert(
            "placement".to_string(),
            Value::Number(Number::from(placement)),
        );
    }
    if message.central_connection != 0 {
        out.insert(
            "centralConnection".to_string(),
            Value::Number(Number::from(message.central_connection)),
        );
    }
    Value::Object(out)
}

fn message_payload_value(message: &SequenceMessagePayload) -> Value {
    match message {
        SequenceMessagePayload::Text(text) => Value::String(text.clone()),
        SequenceMessagePayload::Autonumber(autonumber) => {
            let mut out = Map::new();
            if let Some(start) = autonumber.start {
                out.insert("start".to_string(), sequence_number_value(start));
            }
            if let Some(step) = autonumber.step {
                out.insert("step".to_string(), sequence_number_value(step));
            }
            out.insert("visible".to_string(), Value::Bool(autonumber.visible));
            Value::Object(out)
        }
    }
}

fn sequence_number_value(value: f64) -> Value {
    if value.is_finite()
        && value.fract() == 0.0
        && value >= i64::MIN as f64
        && value <= i64::MAX as f64
    {
        Value::Number(Number::from(value as i64))
    } else {
        Number::from_f64(value)
            .map(Value::Number)
            .unwrap_or(Value::Null)
    }
}

fn boxes_value(boxes: &[SequenceBox]) -> Value {
    Value::Array(boxes.iter().map(box_value).collect())
}

fn box_value(sequence_box: &SequenceBox) -> Value {
    let mut out = Map::new();
    out.insert(
        "actorKeys".to_string(),
        string_array_value(&sequence_box.actor_keys),
    );
    out.insert("fill".to_string(), Value::String(sequence_box.fill.clone()));
    out.insert("name".to_string(), option_string_value(&sequence_box.name));
    out.insert("wrap".to_string(), Value::Bool(sequence_box.wrap));
    Value::Object(out)
}

fn notes_value(notes: &[SequenceNote]) -> Value {
    Value::Array(notes.iter().map(note_value).collect())
}

fn note_value(note: &SequenceNote) -> Value {
    let mut out = Map::new();
    out.insert("actor".to_string(), note.actor.clone());
    out.insert("message".to_string(), Value::String(note.message.clone()));
    out.insert(
        "placement".to_string(),
        Value::Number(Number::from(note.placement)),
    );
    out.insert("wrap".to_string(), Value::Bool(note.wrap));
    Value::Object(out)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceActor {
    #[serde(default)]
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub actor_type: String,
    #[serde(default)]
    pub wrap: bool,
    #[serde(default)]
    pub links: serde_json::Map<String, Value>,
    #[serde(default)]
    pub properties: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceMessage {
    pub id: String,
    #[serde(default)]
    pub from: Option<String>,
    #[serde(default)]
    pub to: Option<String>,
    #[serde(rename = "type")]
    pub message_type: i32,
    pub message: SequenceMessagePayload,
    #[serde(default)]
    pub wrap: bool,
    #[serde(default)]
    pub activate: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<i32>,
    #[serde(
        rename = "centralConnection",
        default,
        skip_serializing_if = "is_zero_i32"
    )]
    pub central_connection: i32,
}

impl SequenceMessage {
    pub fn message_text(&self) -> &str {
        self.message.as_text()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SequenceMessagePayload {
    Text(String),
    Autonumber(SequenceAutonumber),
}

impl SequenceMessagePayload {
    pub fn as_text(&self) -> &str {
        match self {
            Self::Text(text) => text,
            Self::Autonumber(_) => "",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceAutonumber {
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_optional_sequence_number"
    )]
    pub start: Option<f64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_optional_sequence_number"
    )]
    pub step: Option<f64>,
    #[serde(default = "default_true")]
    pub visible: bool,
}

fn serialize_optional_sequence_number<S>(
    value: &Option<f64>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(value) => serialize_sequence_number(*value, serializer),
        None => serializer.serialize_none(),
    }
}

fn serialize_sequence_number<S>(value: f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if value.is_finite()
        && value.fract() == 0.0
        && value >= i64::MIN as f64
        && value <= i64::MAX as f64
    {
        serializer.serialize_i64(value as i64)
    } else {
        serializer.serialize_f64(value)
    }
}

fn default_true() -> bool {
    true
}

fn is_zero_i32(value: &i32) -> bool {
    *value == 0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceBox {
    #[serde(rename = "actorKeys")]
    pub actor_keys: Vec<String>,
    pub fill: String,
    pub name: Option<String>,
    #[serde(default)]
    pub wrap: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceNote {
    pub actor: Value,
    pub message: String,
    pub placement: i32,
    #[serde(default)]
    pub wrap: bool,
}
