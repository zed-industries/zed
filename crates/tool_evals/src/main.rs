mod eval;
mod headless_assistant;

use eval::Eval;
use gpui::Application;
use language_model::ANTHROPIC_PROVIDER_ID;
use reqwest_client::ReqwestClient;
use std::{path::PathBuf, sync::Arc};

fn main() {
    let http_client = Arc::new(ReqwestClient::new());
    let app = Application::headless().with_http_client(http_client.clone());

    app.run(|cx| {
        let app_state = headless_assistant::init(cx);

        let eval = Eval {
            repo_path: PathBuf::from("todo!"),
            system_prompt: Some("You are a helpful assistant.".to_string()),
            user_query: "write me a Limerick about code editors".to_string(),
            provider_id: ANTHROPIC_PROVIDER_ID.to_string(),
            model_name: "claude-3-sonnet-20240229".to_string(),
        };

        let task = eval.run(app_state, cx);

        cx.spawn(|_cx| async move {
            match task.await {
                Ok(response) => println!("Response: {:?}", response),
                Err(err) => println!("Error: {}", err),
            }
        })
        .detach();
    });

    println!("Test succeeded!");
}
