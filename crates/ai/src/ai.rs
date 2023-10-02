use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use tiktoken_rs::{
    get_bpe_from_model, get_chat_completion_max_tokens, ChatCompletionRequestMessage,
};

pub mod completion;
pub mod embedding;

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
}

impl Role {
    pub fn cycle(&mut self) {
        *self = match self {
            Role::User => Role::Assistant,
            Role::Assistant => Role::System,
            Role::System => Role::User,
        }
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Assistant => write!(f, "Assistant"),
            Role::System => write!(f, "System"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct RequestMessage {
    pub role: Role,
    pub content: String,
}

impl RequestMessage {
    pub fn to_tiktoken_message(&self) -> ChatCompletionRequestMessage {
        ChatCompletionRequestMessage {
            content: Some(self.content.clone()),
            role: self.role.to_string(),
            name: None,
            function_call: None,
        }
    }
}

pub fn truncate_messages(
    messages: &Vec<RequestMessage>,
    model: &str,
    reserved_tokens: usize,
) -> anyhow::Result<Vec<RequestMessage>> {
    let mut tiktoken_messages = Vec::new();
    let mut valid_messages = Vec::new();
    for message in messages.into_iter().rev() {
        tiktoken_messages.push(message.to_tiktoken_message());

        let remaining_token_count =
            get_chat_completion_max_tokens(model, tiktoken_messages.as_slice())?;
        if remaining_token_count > reserved_tokens {
            valid_messages.insert(0, message.clone());
        } else {
            let cut_tokens = reserved_tokens - remaining_token_count;
            let bpe = get_bpe_from_model(model)?;
            let encoding = bpe.encode_with_special_tokens(message.content.as_str());
            let content = bpe.decode(encoding[cut_tokens..].to_vec())?;
            let new_message = RequestMessage {
                content,
                role: message.role,
            };
            valid_messages.insert(0, new_message);
            break;
        }
    }

    Ok(valid_messages)
}

#[cfg(test)]
mod tests {
    use rand::{distributions::Alphanumeric, thread_rng, Rng};
    use tiktoken_rs::{get_chat_completion_max_tokens, ChatCompletionRequestMessage};

    use crate::{truncate_messages, RequestMessage, Role};

    fn generate_test_string(token_count: usize, model: &str) -> String {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(token_count * 5)
            .map(char::from)
            .collect();

        let bpe = tiktoken_rs::get_bpe_from_model(model).unwrap();
        let encoding = bpe.encode_with_special_tokens(rand_string.as_str());
        let test_string = bpe.decode(encoding[..token_count].to_vec()).unwrap();

        test_string
    }

    fn tokens_remaining_in_conversation(messages: Vec<RequestMessage>, model: &str) -> usize {
        let messages = messages
            .into_iter()
            .map(|message| message.to_tiktoken_message())
            .collect::<Vec<ChatCompletionRequestMessage>>();

        get_chat_completion_max_tokens(model, messages.as_slice()).unwrap()
    }

    #[test]
    fn test_truncate_messages() {
        // GPT-4s user limit is 8189, reserved is 1k, pf 7189 of valid room
        let model = "gpt-4";

        let mut messages = Vec::new();
        // Should be at least 9010 tokens.
        // As such, when truncated we should return 10 messages, with the first one truncated.
        for _ in 0..10 {
            messages.push(RequestMessage {
                content: generate_test_string(901, model),
                role: Role::User,
            })
        }

        let truncated_messages = truncate_messages(&messages, model, 1000).unwrap();
        assert_eq!(
            tokens_remaining_in_conversation(truncated_messages, model),
            1000
        );

        // Should be at least 9010 tokens.
        // As such, when truncated we should return 10 messages, with the first one truncated.
        let mut messages = Vec::new();
        for _ in 0..10 {
            messages.push(RequestMessage {
                content: generate_test_string(101, model),
                role: Role::User,
            })
        }

        let truncated_messages = truncate_messages(&messages, model, 1000).unwrap();
        assert_eq!(messages, truncated_messages);
    }
}
