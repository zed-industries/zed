use language::BufferSnapshot;
use std::{fmt::Write, ops::Range};

pub fn generate_content_prompt(
    user_prompt: String,
    language_name: Option<&str>,
    buffer: BufferSnapshot,
    range: Range<usize>,
    project_name: Option<String>,
) -> anyhow::Result<String> {
    todo!()

    // let mut prompt = String::new();

    // match language_name {
    //     None | Some("Markdown" | "Plain Text") => writeln!(prompt, "You are an expert engineer.")?,
    //     Some(language_name) => writeln!(prompt, "You are an expert {language_name} engineer.")?,
    // }

    // if let Some(project_name) = project_name {
    //     writeln!(
    //         prompt,
    //         "You are currently working inside the '{project_name}' project in code editor Zed."
    //     )?;
    // }

    // let prefix = buffer.text_for_range(0..range.start).collect::<String>();

    // let mut selected_window = String::new();
    // if start == end {
    //     write!(selected_window, "<|START|>").unwrap();
    // } else {
    //     write!(selected_window, "<|START|").unwrap();
    // }

    // write!(
    //     selected_window,
    //     "{}",
    //     buffer.text_for_range(start..end).collect::<String>()
    // )
    // .unwrap();

    // if start != end {
    //     write!(selected_window, "|END|>").unwrap();
    // }

    // let end_window = buffer.text_for_range(end..buffer.len()).collect::<String>();

    // Ok(prompt)

    // Using new Prompt Templates
    // let openai_model: Arc<dyn LanguageModel> = Arc::new(OpenAiLanguageModel::load(model));

    // let args = PromptArguments {
    //     model: openai_model,
    //     language_name: lang_name.clone(),
    //     project_name,
    //     reserved_tokens: 1000,
    //     buffer: Some(buffer),
    //     selected_range: Some(range),
    //     user_prompt: Some(user_prompt.clone()),
    // };

    // let templates: Vec<(PromptPriority, Box<dyn PromptTemplate>)> = vec![
    //     (PromptPriority::Mandatory, Box::new(EngineerPreamble {})),
    //     (
    //         PromptPriority::Ordered { order: 1 },
    //         Box::new(RepositoryContext {}),
    //     ),
    //     (
    //         PromptPriority::Ordered { order: 0 },
    //         Box::new(FileContext {}),
    //     ),
    //     (
    //         PromptPriority::Mandatory,
    //         Box::new(GenerateInlineContent {}),
    //     ),
    // ];
    // let chain = PromptChain::new(args, templates);
    // let (prompt, _) = chain.generate(true)?;

    // anyhow::Ok(prompt)
}
