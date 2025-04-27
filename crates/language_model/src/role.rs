use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
}

impl Role {
    pub fn from_proto(role: i32) -> Role {
        match proto::LanguageModelRole::from_i32(role) {
            Some(proto::LanguageModelRole::LanguageModelUser) => Role::User,
            Some(proto::LanguageModelRole::LanguageModelAssistant) => Role::Assistant,
            Some(proto::LanguageModelRole::LanguageModelSystem) => Role::System,
            None => Role::User,
        }
    }

    pub fn to_proto(&self) -> proto::LanguageModelRole {
        match self {
            Role::User => proto::LanguageModelRole::LanguageModelUser,
            Role::Assistant => proto::LanguageModelRole::LanguageModelAssistant,
            Role::System => proto::LanguageModelRole::LanguageModelSystem,
        }
    }

    pub fn cycle(self) -> Role {
        match self {
            Role::User => Role::Assistant,
            Role::Assistant => Role::System,
            Role::System => Role::User,
        }
    }
}

impl From<anthropic::Role> for Role {
    fn from(role: anthropic::Role) -> Self {
        match role {
            anthropic::Role::User => Role::User,
            anthropic::Role::Assistant => Role::Assistant,
        }
    }
}

impl TryFrom<Role> for anthropic::Role {
    type Error = anyhow::Error;

    fn try_from(role: Role) -> Result<Self, Self::Error> {
        match role {
            Role::User => Ok(anthropic::Role::User),
            Role::Assistant => Ok(anthropic::Role::Assistant),
            Role::System => Err(anyhow::anyhow!(
                "System role is not supported in anthropic API"
            )),
        }
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::User => write!(f, "user"),
            Role::Assistant => write!(f, "assistant"),
            Role::System => write!(f, "system"),
        }
    }
}
