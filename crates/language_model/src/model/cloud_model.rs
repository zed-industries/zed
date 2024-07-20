use crate::LanguageModelRequest;
pub use anthropic::Model as AnthropicModel;
pub use ollama::Model as OllamaModel;
pub use open_ai::Model as OpenAiModel;
use schemars::{
    schema::{InstanceType, Metadata, Schema, SchemaObject},
    JsonSchema,
};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, Debug, Default, PartialEq, EnumIter)]
pub enum CloudModel {
    Gpt3Point5Turbo,
    Gpt4,
    Gpt4Turbo,
    #[default]
    Gpt4Omni,
    Gpt4OmniMini,
    Claude3_5Sonnet,
    Claude3Opus,
    Claude3Sonnet,
    Claude3Haiku,
    Gemini15Pro,
    Gemini15Flash,
    Custom(String),
}

impl Serialize for CloudModel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.id())
    }
}

impl<'de> Deserialize<'de> for CloudModel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ZedDotDevModelVisitor;

        impl<'de> Visitor<'de> for ZedDotDevModelVisitor {
            type Value = CloudModel;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string for a ZedDotDevModel variant or a custom model")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let model = CloudModel::iter()
                    .find(|model| model.id() == value)
                    .unwrap_or_else(|| CloudModel::Custom(value.to_string()));
                Ok(model)
            }
        }

        deserializer.deserialize_str(ZedDotDevModelVisitor)
    }
}

impl JsonSchema for CloudModel {
    fn schema_name() -> String {
        "ZedDotDevModel".to_owned()
    }

    fn json_schema(_generator: &mut schemars::gen::SchemaGenerator) -> Schema {
        let variants = CloudModel::iter()
            .filter_map(|model| {
                let id = model.id();
                if id.is_empty() {
                    None
                } else {
                    Some(id.to_string())
                }
            })
            .collect::<Vec<_>>();
        Schema::Object(SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            enum_values: Some(variants.iter().map(|s| s.clone().into()).collect()),
            metadata: Some(Box::new(Metadata {
                title: Some("ZedDotDevModel".to_owned()),
                default: Some(CloudModel::default().id().into()),
                examples: variants.into_iter().map(Into::into).collect(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}

impl CloudModel {
    pub fn id(&self) -> &str {
        match self {
            Self::Gpt3Point5Turbo => "gpt-3.5-turbo",
            Self::Gpt4 => "gpt-4",
            Self::Gpt4Turbo => "gpt-4-turbo-preview",
            Self::Gpt4Omni => "gpt-4o",
            Self::Gpt4OmniMini => "gpt-4o-mini",
            Self::Claude3_5Sonnet => "claude-3-5-sonnet",
            Self::Claude3Opus => "claude-3-opus",
            Self::Claude3Sonnet => "claude-3-sonnet",
            Self::Claude3Haiku => "claude-3-haiku",
            Self::Gemini15Pro => "gemini-1.5-pro",
            Self::Gemini15Flash => "gemini-1.5-flash",
            Self::Custom(id) => id,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Gpt3Point5Turbo => "GPT 3.5 Turbo",
            Self::Gpt4 => "GPT 4",
            Self::Gpt4Turbo => "GPT 4 Turbo",
            Self::Gpt4Omni => "GPT 4 Omni",
            Self::Gpt4OmniMini => "GPT 4 Omni Mini",
            Self::Claude3_5Sonnet => "Claude 3.5 Sonnet",
            Self::Claude3Opus => "Claude 3 Opus",
            Self::Claude3Sonnet => "Claude 3 Sonnet",
            Self::Claude3Haiku => "Claude 3 Haiku",
            Self::Gemini15Pro => "Gemini 1.5 Pro",
            Self::Gemini15Flash => "Gemini 1.5 Flash",
            Self::Custom(id) => id.as_str(),
        }
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            Self::Gpt3Point5Turbo => 2048,
            Self::Gpt4 => 4096,
            Self::Gpt4Turbo | Self::Gpt4Omni => 128000,
            Self::Gpt4OmniMini => 128000,
            Self::Claude3_5Sonnet
            | Self::Claude3Opus
            | Self::Claude3Sonnet
            | Self::Claude3Haiku => 200000,
            Self::Gemini15Pro => 128000,
            Self::Gemini15Flash => 32000,
            Self::Custom(_) => 4096, // TODO: Make this configurable
        }
    }

    pub fn preprocess_request(&self, request: &mut LanguageModelRequest) {
        match self {
            Self::Claude3Opus | Self::Claude3Sonnet | Self::Claude3Haiku => {
                request.preprocess_anthropic()
            }
            _ => {}
        }
    }
}
