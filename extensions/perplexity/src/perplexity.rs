use zed::{
    serde_json::{self, json},
    HttpMethod,
};
use zed_extension_api::{self as zed, Result};

struct Perplexity;

impl zed::Extension for Perplexity {
    fn new() -> Self {
        Self
    }

    fn run_slash_command(
        &self,
        command: zed::SlashCommand,
        argument: Option<String>,
        worktree: Option<&zed::Worktree>,
    ) -> zed::Result<zed::SlashCommandOutput> {
        let worktree = worktree.ok_or("Worktree is required")?;
        // Check if argument is provided
        let Some(query) = argument else {
            return Ok(zed::SlashCommandOutput {
                text: "Error: Query not provided. Please enter a question or topic.".to_string(),
                sections: vec![],
            });
        };

        // Get the API key from the environment
        let env_vars = worktree.shell_env();
        let api_key = env_vars
            .iter()
            .find(|(key, _)| key == "PERPLEXITY_API_KEY")
            .map(|(_, value)| value.clone())
            .ok_or("PERPLEXITY_API_KEY not found in environment")?;

        // Prepare the request
        let request = zed::HttpRequest {
            method: HttpMethod::Post,
            url: "https://api.perplexity.ai/chat/completions".to_string(),
            headers: vec![
                ("Authorization".to_string(), format!("Bearer {}", api_key)),
                ("Content-Type".to_string(), "application/json".to_string()),
            ],
            body: Some(
                serde_json::to_vec(&json!({
                    "model": "llama-3.1-sonar-small-128k-online",
                    "messages": [{"role": "user", "content": query}],
                    "stream": true,
                }))
                .unwrap(),
            ),
        };

        // Make the HTTP request
        match zed::fetch_stream(&request) {
            Ok(stream) => {
                let mut full_content = String::new();
                let mut buffer = String::new();
                while let Ok(Some(chunk)) = stream.next_chunk() {
                    buffer.push_str(&String::from_utf8_lossy(&chunk));
                    for line in buffer.lines() {
                        if line.starts_with("data: ") {
                            let json = &line["data: ".len()..];
                            if let Ok(event) = serde_json::from_str::<StreamEvent>(json) {
                                if let Some(choice) = event.choices.first() {
                                    full_content.push_str(&choice.delta.content);
                                }
                            }
                        }
                    }
                    buffer.clear();
                }
                Ok(zed::SlashCommandOutput {
                    text: full_content,
                    sections: vec![],
                })
            }
            Err(e) => Ok(zed::SlashCommandOutput {
                text: format!("API request failed. Error: {}. API Key: {}", e, api_key),
                sections: vec![],
            }),
        }
    }

    fn complete_slash_command_argument(
        &self,
        _command: zed::SlashCommand,
        query: String,
    ) -> zed::Result<Vec<zed::SlashCommandArgumentCompletion>> {
        let suggestions = vec![
            "What events are happening in Boulder this month?",
            "What concerts are coming to Boulder soon?",
        ];

        Ok(suggestions
            .into_iter()
            .filter(|suggestion| suggestion.to_lowercase().contains(&query.to_lowercase()))
            .map(|suggestion| zed::SlashCommandArgumentCompletion {
                label: suggestion.to_string(),
                new_text: suggestion.to_string(),
                run_command: true,
            })
            .collect())
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<zed_extension_api::Command> {
        Err("not implemented");
    }
}

#[derive(serde::Deserialize)]
struct StreamEvent {
    id: String,
    model: String,
    created: u64,
    usage: Usage,
    object: String,
    choices: Vec<Choice>,
}

#[derive(serde::Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(serde::Deserialize)]
struct Choice {
    index: u32,
    finish_reason: Option<String>,
    message: Message,
    delta: Delta,
}

#[derive(serde::Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(serde::Deserialize)]
struct Delta {
    role: String,
    content: String,
}

zed::register_extension!(Perplexity);
