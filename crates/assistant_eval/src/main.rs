mod eval;
mod headless_assistant;
mod judge;
mod templates_eval;

use anyhow::{anyhow, Result};
use clap::Parser;
use eval::Eval;
use futures::stream::{self, StreamExt};
use gpui::{Application, AsyncApp};
use headless_assistant::{authenticate_model_provider, find_model, HeadlessAppState};
use language_model::{LanguageModel, LanguageModelRegistry};
use reqwest_client::ReqwestClient;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
    time::SystemTime,
};
use tempfile::TempDir;
use templates_eval::{all_templates, Template};
use util::command::new_smol_command;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(
    name = "assistant_eval",
    disable_version_flag = true,
    before_help = "Tool eval runner"
)]
struct Args {
    /// Regexes to match the names of evals to run.
    #[arg(long)]
    exercise_names: Vec<String>,
    /// Runs all exercises, causes the exercise_names to be ignored.
    #[arg(long)]
    all: bool,
    /// Supported language types to evaluate (default: python,go,rust,typescript,javascript,ruby,php,bash)
    #[arg(
        long,
        default_value = "python,go,rust,typescript,javascript,ruby,php,bash"
    )]
    languages: String,
    /// Name of the model (default: "claude-3-7-sonnet-latest")
    #[arg(long, default_value = "claude-3-7-sonnet-latest")]
    model_name: String,
    /// Name of the editor model (default: value of `--model_name`).
    #[arg(long)]
    editor_model_name: Option<String>,
    /// Name of the judge model (default: value of `--model_name`).
    #[arg(long)]
    judge_model_name: Option<String>,
    /// Number of evaluations to run concurrently (default: 3)
    #[arg(short, long, default_value = "3")]
    concurrency: usize,
    /// Maximum number of exercises to evaluate per language
    #[arg(long)]
    max_exercises_per_language: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct SetupConfig {
    #[serde(rename = "base.sha")]
    base_sha: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct EvalResult {
    exercise_name: String,
    template_name: String,
    score: String,
    diff: String,
    assistant_response: String,
    elapsed_time_ms: u128,
    timestamp: u128,
    // Rename fields to match TokenUsage struct
    input_tokens: usize,
    output_tokens: usize,
    total_tokens: usize,
    tool_use_counts: usize,
}

async fn run_git_command(repo_path: &Path, args: Vec<&str>) -> Result<String> {
    let output = new_smol_command("git")
        .current_dir(repo_path)
        .args(args.clone())
        .output()
        .await?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    } else {
        Err(anyhow!(
            "Git command failed: {} with status: {}",
            args.join(" "),
            output.status
        ))
    }
}

async fn setup_temp_repo(exercise_path: &Path, _base_sha: &str) -> Result<TempDir> {
    let temp_dir = TempDir::new()?;

    // Copy the exercise files to the temp directory, excluding .docs and .meta
    for entry in WalkDir::new(exercise_path).min_depth(0).max_depth(10) {
        let entry = entry?;
        let source_path = entry.path();

        // Skip .docs and .meta directories completely
        if source_path.starts_with(exercise_path.join(".docs"))
            || source_path.starts_with(exercise_path.join(".meta"))
        {
            continue;
        }

        if source_path.is_file() {
            let relative_path = source_path.strip_prefix(exercise_path)?;
            let dest_path = temp_dir.path().join(relative_path);

            // Make sure parent directories exist
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::copy(source_path, dest_path)?;
        }
    }

    // Initialize git repo in the temp directory
    run_git_command(temp_dir.path(), vec!["init"]).await?;
    run_git_command(temp_dir.path(), vec!["add", "."]).await?;
    run_git_command(temp_dir.path(), vec!["commit", "-m", "Initial commit"]).await?;

    println!("Created temp repo without .docs and .meta directories");

    Ok(temp_dir)
}

fn find_exercises(
    framework_path: &Path,
    languages: &[&str],
    max_per_language: Option<usize>,
) -> Result<Vec<PathBuf>> {
    let mut all_exercises = Vec::new();

    println!("Searching for exercises in languages: {:?}", languages);

    for language in languages {
        let language_dir = framework_path
            .join("data")
            .join(language)
            .join("exercises")
            .join("practice");

        println!("Checking language directory: {:?}", language_dir);
        if !language_dir.exists() {
            println!("Warning: Language directory not found: {:?}", language_dir);
            continue;
        }

        let mut exercises = Vec::new();
        match fs::read_dir(&language_dir) {
            Ok(entries) => {
                for entry_result in entries {
                    match entry_result {
                        Ok(entry) => {
                            let path = entry.path();

                            if path.is_dir() {
                                // Map the language to the file extension
                                let language_extension = match *language {
                                    "python" => "py",
                                    "go" => "go",
                                    "rust" => "rs",
                                    "typescript" => "ts",
                                    "javascript" => "js",
                                    "ruby" => "rb",
                                    "php" => "php",
                                    "bash" => "sh",
                                    "multi" => "diff",
                                    _ => continue, // Skip unsupported languages
                                };

                                // Check if this is a valid exercise with instructions and example
                                let instructions_path = path.join(".docs").join("instructions.md");
                                let has_instructions = instructions_path.exists();
                                let example_path = path
                                    .join(".meta")
                                    .join(format!("example.{}", language_extension));
                                let has_example = example_path.exists();

                                if has_instructions && has_example {
                                    exercises.push(path);
                                }
                            }
                        }
                        Err(err) => println!("Error reading directory entry: {}", err),
                    }
                }
            }
            Err(err) => println!(
                "Error reading directory {}: {}",
                language_dir.display(),
                err
            ),
        }

        // Sort exercises by name for consistent selection
        exercises.sort_by(|a, b| {
            let a_name = a.file_name().unwrap_or_default().to_string_lossy();
            let b_name = b.file_name().unwrap_or_default().to_string_lossy();
            a_name.cmp(&b_name)
        });

        // Apply the limit if specified
        if let Some(limit) = max_per_language {
            if exercises.len() > limit {
                println!(
                    "Limiting {} exercises to {} for language {}",
                    exercises.len(),
                    limit,
                    language
                );
                exercises.truncate(limit);
            }
        }

        println!(
            "Found {} exercises for language {}: {:?}",
            exercises.len(),
            language,
            exercises
                .iter()
                .map(|p| p.file_name().unwrap_or_default().to_string_lossy())
                .collect::<Vec<_>>()
        );
        all_exercises.extend(exercises);
    }

    Ok(all_exercises)
}

fn get_exercise_name(exercise_path: &Path) -> String {
    exercise_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

fn get_exercise_language(exercise_path: &Path) -> Result<String> {
    // Extract the language from path (data/python/exercises/... => python)
    let parts: Vec<_> = exercise_path.components().collect();

    for (i, part) in parts.iter().enumerate() {
        if i > 0 && part.as_os_str() == "data" {
            if i + 1 < parts.len() {
                return Ok(parts[i + 1].as_os_str().to_string_lossy().to_string());
            }
        }
    }

    Err(anyhow!(
        "Could not determine language from path: {:?}",
        exercise_path
    ))
}

async fn read_instructions(exercise_path: &Path) -> Result<String> {
    let instructions_path = exercise_path.join(".docs").join("instructions.md");
    println!("Reading instructions from: {}", instructions_path.display());
    let instructions = smol::unblock(move || std::fs::read_to_string(&instructions_path)).await?;
    Ok(instructions)
}

async fn read_example_solution(exercise_path: &Path, language: &str) -> Result<String> {
    // Map the language to the file extension
    let language_extension = match language {
        "python" => "py",
        "go" => "go",
        "rust" => "rs",
        "typescript" => "ts",
        "javascript" => "js",
        "ruby" => "rb",
        "php" => "php",
        "bash" => "sh",
        "multi" => "diff",
        _ => return Err(anyhow!("Unsupported language: {}", language)),
    };
    let example_path = exercise_path
        .join(".meta")
        .join(format!("example.{}", language_extension));
    println!("Reading example solution from: {}", example_path.display());
    let example = smol::unblock(move || std::fs::read_to_string(&example_path)).await?;
    Ok(example)
}

async fn read_base_sha(framework_path: &Path) -> Result<String> {
    let setup_path = framework_path.join("setup.json");
    let setup_content = smol::unblock(move || std::fs::read_to_string(&setup_path)).await?;
    let setup_config: SetupConfig = serde_json_lenient::from_str_lenient(&setup_content)?;
    Ok(setup_config.base_sha)
}

async fn save_eval_results(exercise_path: &Path, results: Vec<EvalResult>) -> Result<()> {
    let eval_dir = exercise_path.join("evaluation");
    fs::create_dir_all(&eval_dir)?;

    let eval_file = eval_dir.join("evals.json");

    println!("Saving evaluation results to: {}", eval_file.display());
    println!(
        "Results to save: {} evaluations for exercise path: {}",
        results.len(),
        exercise_path.display()
    );

    // Check file existence before reading/writing
    if eval_file.exists() {
        println!("Existing evals.json file found, will update it");
    } else {
        println!("No existing evals.json file found, will create new one");
    }

    // Structure to organize evaluations by test name and timestamp
    let mut eval_data: serde_json::Value = if eval_file.exists() {
        let content = fs::read_to_string(&eval_file)?;
        serde_json::from_str(&content).unwrap_or_else(|_| serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    // Get current timestamp for this batch of results
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_millis()
        .to_string();

    // Group the new results by test name (exercise name)
    for result in results {
        let exercise_name = &result.exercise_name;
        let template_name = &result.template_name;

        println!(
            "Adding result: exercise={}, template={}",
            exercise_name, template_name
        );

        // Ensure the exercise entry exists
        if !eval_data.get(exercise_name).is_some() {
            eval_data[exercise_name] = serde_json::json!({});
        }

        // Ensure the timestamp entry exists as an object
        if !eval_data[exercise_name].get(&timestamp).is_some() {
            eval_data[exercise_name][&timestamp] = serde_json::json!({});
        }

        // Add this result under the timestamp with template name as key
        eval_data[exercise_name][&timestamp][template_name] = serde_json::to_value(&result)?;
    }

    // Write back to file with pretty formatting
    let json_content = serde_json::to_string_pretty(&eval_data)?;
    match fs::write(&eval_file, json_content) {
        Ok(_) => println!("✓ Successfully saved results to {}", eval_file.display()),
        Err(e) => println!("✗ Failed to write results file: {}", e),
    }

    Ok(())
}

async fn run_exercise_eval(
    exercise_path: PathBuf,
    template: Template,
    model: Arc<dyn LanguageModel>,
    judge_model: Arc<dyn LanguageModel>,
    app_state: Arc<HeadlessAppState>,
    base_sha: String,
    _framework_path: PathBuf,
    cx: AsyncApp,
) -> Result<EvalResult> {
    let exercise_name = get_exercise_name(&exercise_path);
    let language = get_exercise_language(&exercise_path)?;
    let mut instructions = read_instructions(&exercise_path).await?;
    instructions.push_str(&format!(
        "\n\nWhen writing the code for this prompt, use {} to achieve the goal.",
        language
    ));
    let example_solution = read_example_solution(&exercise_path, &language).await?;

    println!(
        "Running evaluation for exercise: {} with template: {}",
        exercise_name, template.name
    );

    // Create temporary directory with exercise files
    let temp_dir = setup_temp_repo(&exercise_path, &base_sha).await?;
    let temp_path = temp_dir.path().to_path_buf();

    if template.name == "ProjectCreation" {
        for entry in fs::read_dir(&temp_path)? {
            let entry = entry?;
            let path = entry.path();

            // Skip directories that start with dot (like .docs, .meta, .git)
            if path.is_dir()
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .map(|name| name.starts_with("."))
                    .unwrap_or(false)
            {
                continue;
            }

            // Delete regular files
            if path.is_file() {
                println!("  Deleting file: {}", path.display());
                fs::remove_file(path)?;
            }
        }

        // Commit the deletion so it shows up in the diff
        run_git_command(&temp_path, vec!["add", "."]).await?;
        run_git_command(
            &temp_path,
            vec!["commit", "-m", "Remove root files for clean slate"],
        )
        .await?;
    }

    let local_commit_sha = run_git_command(&temp_path, vec!["rev-parse", "HEAD"]).await?;

    // Prepare prompt based on template
    let prompt = match template.name {
        "ProjectCreation" => format!(
            "I need to create a new implementation for this exercise. Please create all the necessary files in the best location.\n\n{}",
            instructions
        ),
        "CodeModification" => format!(
            "I need help updating my code to meet these requirements. Please modify the appropriate files:\n\n{}",
            instructions
        ),
        "ConversationalGuidance" => format!(
            "I'm trying to solve this coding exercise but I'm not sure where to start. Can you help me understand the requirements and guide me through the solution process without writing code for me?\n\n{}",
            instructions
        ),
        _ => instructions.clone(),
    };

    let start_time = SystemTime::now();

    // Create a basic eval struct to work with the existing system
    let eval = Eval {
        name: exercise_name.clone(),
        path: exercise_path.clone(),
        repo_path: temp_path.clone(),
        eval_setup: eval::EvalSetup {
            url: format!("file://{}", temp_path.display()),
            base_sha: local_commit_sha, // Use the local commit SHA instead of the framework base SHA
        },
        user_prompt: prompt,
    };

    // Run the evaluation
    let eval_output = cx
        .update(|cx| eval.run(app_state.clone(), model.clone(), cx))?
        .await?;

    // Get diff from git
    let diff = eval_output.diff.clone();

    // For project creation template, we need to compare with reference implementation
    let judge_output = if template.name == "ProjectCreation" {
        let project_judge_prompt = template
            .content
            .replace(
                "<!-- ```requirements go here``` -->",
                &format!("```\n{}\n```", instructions),
            )
            .replace(
                "<!-- ```reference code goes here``` -->",
                &format!("```{}\n{}\n```", language, example_solution),
            )
            .replace(
                "<!-- ```git diff goes here``` -->",
                &format!("```\n{}\n```", diff),
            );

        // Use the run_with_prompt method which we'll add to judge.rs
        let judge = judge::Judge {
            original_diff: None,
            original_message: Some(project_judge_prompt),
            model: judge_model.clone(),
        };

        cx.update(|cx| judge.run_with_prompt(cx))?.await?
    } else if template.name == "CodeModification" {
        // For CodeModification, we'll compare the example solution with the LLM-generated solution
        let code_judge_prompt = template
            .content
            .replace(
                "<!-- ```reference code goes here``` -->",
                &format!("```{}\n{}\n```", language, example_solution),
            )
            .replace(
                "<!-- ```git diff goes here``` -->",
                &format!("```\n{}\n```", diff),
            );

        // Use the run_with_prompt method
        let judge = judge::Judge {
            original_diff: None,
            original_message: Some(code_judge_prompt),
            model: judge_model.clone(),
        };

        cx.update(|cx| judge.run_with_prompt(cx))?.await?
    } else {
        // Conversational template
        let conv_judge_prompt = template
            .content
            .replace(
                "<!-- ```query goes here``` -->",
                &format!("```\n{}\n```", instructions),
            )
            .replace(
                "<!-- ```transcript goes here``` -->",
                &format!("```\n{}\n```", eval_output.last_message),
            )
            .replace(
                "<!-- ```git diff goes here``` -->",
                &format!("```\n{}\n```", diff),
            );

        // Use the run_with_prompt method for consistency
        let judge = judge::Judge {
            original_diff: None,
            original_message: Some(conv_judge_prompt),
            model: judge_model.clone(),
        };

        cx.update(|cx| judge.run_with_prompt(cx))?.await?
    };

    let elapsed_time = start_time.elapsed()?;

    // Calculate total tokens as the sum of input and output tokens
    let input_tokens = eval_output.token_usage.input_tokens;
    let output_tokens = eval_output.token_usage.output_tokens;
    let tool_use_counts = eval_output.tool_use_counts.values().sum::<u32>();
    let total_tokens = input_tokens + output_tokens;

    // Save results to evaluation directory
    let result = EvalResult {
        exercise_name: exercise_name.clone(),
        template_name: template.name.to_string(),
        score: judge_output.trim().to_string(),
        diff,
        assistant_response: eval_output.last_message.clone(),
        elapsed_time_ms: elapsed_time.as_millis(),
        timestamp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis(),
        // Convert u32 token counts to usize
        input_tokens: input_tokens.try_into().unwrap(),
        output_tokens: output_tokens.try_into().unwrap(),
        total_tokens: total_tokens.try_into().unwrap(),
        tool_use_counts: tool_use_counts.try_into().unwrap(),
    };

    Ok(result)
}

// First, let's define the order in which templates should be executed
const TEMPLATE_EXECUTION_ORDER: [&str; 3] = [
    "ProjectCreation",
    "CodeModification",
    "ConversationalGuidance",
];

fn main() {
    env_logger::init();
    let args = Args::parse();
    let http_client = Arc::new(ReqwestClient::new());
    let app = Application::headless().with_http_client(http_client.clone());

    // Path to the zed-ace-framework repo
    let framework_path = PathBuf::from("../zed-ace-framework")
        .canonicalize()
        .unwrap();

    // Fix the 'languages' lifetime issue by creating owned Strings instead of slices
    let languages: Vec<String> = args.languages.split(',').map(|s| s.to_string()).collect();

    println!("Using zed-ace-framework at: {:?}", framework_path);
    println!("Evaluating languages: {:?}", languages);

    app.run(move |cx| {
        let app_state = headless_assistant::init(cx);

        let model = find_model(&args.model_name, cx).unwrap();
        let editor_model = if let Some(model_name) = &args.editor_model_name {
            find_model(model_name, cx).unwrap()
        } else {
            model.clone()
        };
        let judge_model = if let Some(model_name) = &args.judge_model_name {
            find_model(model_name, cx).unwrap()
        } else {
            model.clone()
        };

        LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
            registry.set_active_model(Some(model.clone()), cx);
            registry.set_editor_model(Some(editor_model.clone()), cx);
        });

        let model_provider_id = model.provider_id();
        let editor_model_provider_id = editor_model.provider_id();
        let judge_model_provider_id = judge_model.provider_id();

        let framework_path_clone = framework_path.clone();
        let languages_clone = languages.clone();
        let exercise_names = args.exercise_names.clone();
        let all_flag = args.all;

        cx.spawn(move |cx| async move {
            // Authenticate all model providers first
            cx.update(|cx| authenticate_model_provider(model_provider_id.clone(), cx))
                .unwrap()
                .await
                .unwrap();
            cx.update(|cx| authenticate_model_provider(editor_model_provider_id.clone(), cx))
                .unwrap()
                .await
                .unwrap();
            cx.update(|cx| authenticate_model_provider(judge_model_provider_id.clone(), cx))
                .unwrap()
                .await
                .unwrap();

            // Read base SHA from setup.json
            let base_sha = read_base_sha(&framework_path_clone).await.unwrap();
            println!("Using base SHA: {}", base_sha);

            // Find all exercises for the specified languages
            let all_exercises = find_exercises(
                &framework_path_clone,
                &languages_clone
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>(),
                args.max_exercises_per_language,
            )
            .unwrap();
            println!("Found {} exercises total", all_exercises.len());

            // Filter exercises if specific ones were requested
            let exercises_to_run = if all_flag {
                all_exercises
            } else if !exercise_names.is_empty() {
                all_exercises
                    .into_iter()
                    .filter(|path| {
                        let name = get_exercise_name(path);
                        exercise_names.iter().any(|filter| name.contains(filter))
                    })
                    .collect()
            } else {
                all_exercises
            };

            println!("Will run {} exercises", exercises_to_run.len());

            // Get all templates and sort them according to the execution order
            let mut templates = all_templates();
            templates.sort_by_key(|template| {
                TEMPLATE_EXECUTION_ORDER
                    .iter()
                    .position(|&name| name == template.name)
                    .unwrap_or(usize::MAX)
            });

            // Create exercise eval tasks - each exercise is a single task that will run templates sequentially
            let exercise_tasks: Vec<_> = exercises_to_run
                .into_iter()
                .map(|exercise_path| {
                    let exercise_name = get_exercise_name(&exercise_path);
                    let templates_clone = templates.clone();
                    let model_clone = model.clone();
                    let judge_model_clone = judge_model.clone();
                    let app_state_clone = app_state.clone();
                    let base_sha_clone = base_sha.clone();
                    let framework_path_clone = framework_path_clone.clone();
                    let cx_clone = cx.clone();

                    async move {
                        println!("Processing exercise: {}", exercise_name);
                        let mut exercise_results = Vec::new();

                        // Determine the language for this exercise
                        let language = match get_exercise_language(&exercise_path) {
                            Ok(lang) => lang,
                            Err(err) => {
                                println!("Error determining language for {}: {}", exercise_name, err);
                                return exercise_results;
                            }
                        };

                        // Run each template sequentially for this exercise
                        for template in templates_clone {
                            // For "multi" language, only run the CodeModification template
                            if language == "multi" && template.name != "CodeModification" {
                                println!("Skipping {} template for multi language", template.name);
                                continue;
                            }

                            match run_exercise_eval(
                                exercise_path.clone(),
                                template.clone(),
                                model_clone.clone(),
                                judge_model_clone.clone(),
                                app_state_clone.clone(),
                                base_sha_clone.clone(),
                                framework_path_clone.clone(),
                                cx_clone.clone(),
                            )
                            .await
                            {
                                Ok(result) => {
                                    println!(
                                        "Completed {} with template {} - score: {}",
                                        exercise_name, template.name, result.score
                                    );
                                    exercise_results.push(result);
                                }
                                Err(err) => {
                                    println!(
                                        "Error running {} with template {}: {}",
                                        exercise_name, template.name, err
                                    );
                                }
                            }
                        }

                        // Save results for this exercise
                        if !exercise_results.is_empty() {
                            if let Err(err) =
                                save_eval_results(&exercise_path, exercise_results.clone()).await
                            {
                                println!("Error saving results for {}: {}", exercise_name, err);
                            } else {
                                println!("Saved results for {}", exercise_name);
                            }
                        }

                        exercise_results
                    }
                })
                .collect();

            println!(
                "Running {} exercises with concurrency: {}",
                exercise_tasks.len(),
                args.concurrency
            );

            // Run exercises concurrently, with each exercise running its templates sequentially
            let all_results = stream::iter(exercise_tasks)
                .buffer_unordered(args.concurrency)
                .flat_map(|results| stream::iter(results))
                .collect::<Vec<_>>()
                .await;

            println!("Completed {} evaluation runs", all_results.len());
            cx.update(|cx| cx.quit()).unwrap();
        })
        .detach();
    });

    println!("Done running evals");
}
