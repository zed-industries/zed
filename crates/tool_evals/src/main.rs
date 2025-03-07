mod llm_request;

use client::{Client, UserStore};
use gpui::{AppContext, Application, Entity};
use language_model::{LanguageModelId, LanguageModelRegistry};
use llm_request::Eval;
use std::sync::Arc;

fn main() {
    let app = Application::headless();

    app.run(|cx| {
        let registry = LanguageModelRegistry::global(cx);

        cx.spawn(|mut cx| async move {
            let eval = Eval {
                system_prompt: "You are a helpful assistant.".to_string(),
                user_query: "write me a Limerick about code editors".to_string(),
                model_name: "claude-3-7-sonnet-20240229".to_string(),
            };
            let result = llm_request::run_eval(registry, &eval, &mut cx).await;

            match result {
                Ok(response) => println!("Response: {}", response),
                Err(err) => println!("Error: {}", err),
            }
        })
        .detach();
    });

    println!("Test succeeded!");
}
