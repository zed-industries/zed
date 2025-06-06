use CodeOrbit::{
    http_client::HttpMethod,
    http_client::HttpRequest,
    serde_json::{self, json},
};
use codeorbit_extension_api::{self as CodeOrbit, Result, http_client::RedirectPolicy};

struct Perplexity;

impl CodeOrbit::Extension for Perplexity {
    fn new() -> Self {
        Self
    }

    fn run_slash_command(
        &self,
        command: CodeOrbit::SlashCommand,
        argument: Vec<String>,
        worktree: Option<&CodeOrbit::Worktree>,
    ) -> CodeOrbit::Result<CodeOrbit::SlashCommandOutput> {
        // Check if the command is 'perplexity'
        if command.name != "perplexity" {
            return Err("Invalid command. Expected 'perplexity'.".into());
        }

        let worktree = worktree.ok_or("Worktree is required")?;
        // Join arguments with space as the query
        let query = argument.join(" ");
        if query.is_empty() {
            return Ok(CodeOrbit::SlashCommandOutput {
                text: "Error: Query not provided. Please enter a question or topic.".to_string(),
                sections: vec![],
            });
        }

        // Get the API key from the environment
        let env_vars = worktree.shell_env();
        let api_key = env_vars
            .iter()
            .find(|(key, _)| key == "PERPLEXITY_API_KEY")
            .map(|(_, value)| value.clone())
            .ok_or("PERPLEXITY_API_KEY not found in environment")?;

        // Prepare the request
        let request = HttpRequest {
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
            redirect_policy: RedirectPolicy::FollowAll,
        };

        // Make the HTTP request
        match CodeOrbit::http_client::fetch_stream(&request) {
            Ok(stream) => {
                let mut full_content = String::new();
                let mut buffer = String::new();
                while let Ok(Some(chunk)) = stream.next_chunk() {
                    buffer.push_str(&String::from_utf8_lossy(&chunk));
                    for line in buffer.lines() {
                        if let Some(json) = line.strip_prefix("data: ") {
                            if let Ok(event) = serde_json::from_str::<StreamEvent>(json) {
                                if let Some(choice) = event.choices.first() {
                                    full_content.push_str(&choice.delta.content);
                                }
                            }
                        }
                    }
                    buffer.clear();
                }
                Ok(CodeOrbit::SlashCommandOutput {
                    text: full_content,
                    sections: vec![],
                })
            }
            Err(e) => Ok(CodeOrbit::SlashCommandOutput {
                text: format!("API request failed. Error: {}. API Key: {}", e, api_key),
                sections: vec![],
            }),
        }
    }

    fn complete_slash_command_argument(
        &self,
        _command: CodeOrbit::SlashCommand,
        query: Vec<String>,
    ) -> CodeOrbit::Result<Vec<CodeOrbit::SlashCommandArgumentCompletion>> {
        let suggestions = vec!["How do I develop a CodeOrbit extension?"];
        let query = query.join(" ").to_lowercase();

        Ok(suggestions
            .into_iter()
            .filter(|suggestion| suggestion.to_lowercase().contains(&query))
            .map(|suggestion| CodeOrbit::SlashCommandArgumentCompletion {
                label: suggestion.to_string(),
                new_text: suggestion.to_string(),
                run_command: true,
            })
            .collect())
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &codeorbit_extension_api::LanguageServerId,
        _worktree: &codeorbit_extension_api::Worktree,
    ) -> Result<codeorbit_extension_api::Command> {
        Err("Not implemented".into())
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

CodeOrbit::register_extension!(Perplexity);
