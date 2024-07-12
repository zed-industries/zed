use super::*;

pub fn preprocess_anthropic_request(request: &mut LanguageModelRequest) {
    let mut new_messages: Vec<LanguageModelRequestMessage> = Vec::new();
    let mut system_message = String::new();

    for message in request.messages.drain(..) {
        if message.content.is_empty() {
            continue;
        }

        match message.role {
            Role::User | Role::Assistant => {
                if let Some(last_message) = new_messages.last_mut() {
                    if last_message.role == message.role {
                        last_message.content.push_str("\n\n");
                        last_message.content.push_str(&message.content);
                        continue;
                    }
                }

                new_messages.push(message);
            }
            Role::System => {
                if !system_message.is_empty() {
                    system_message.push_str("\n\n");
                }
                system_message.push_str(&message.content);
            }
        }
    }

    if !system_message.is_empty() {
        new_messages.insert(
            0,
            LanguageModelRequestMessage {
                role: Role::System,
                content: system_message,
            },
        );
    }

    request.messages = new_messages;
}
