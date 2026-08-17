// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use find::find_and_report_envs;
use find::SearchScope;
use locators::create_locators;
use pet_conda::Conda;
use pet_conda::CondaLocator;
use pet_core::os_environment::Environment;
use pet_core::python_environment::PythonEnvironmentKind;
use pet_core::Locator;
use pet_core::{os_environment::EnvironmentApi, reporter::Reporter, Configuration};
use pet_fs::glob::expand_glob_patterns;
use pet_poetry::Poetry;
use pet_poetry::PoetryLocator;
use pet_python_utils::cache::set_cache_directory;
use pet_reporter::{self, cache::CacheReporter, collect, stdio};
use resolve::resolve_environment;
use serde::Serialize;
use std::path::PathBuf;
use std::{collections::BTreeMap, env, sync::Arc, time::SystemTime};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub mod find;
pub mod locators;
pub mod resolve;

/// Initialize tracing subscriber for performance profiling.
/// Set RUST_LOG=info or RUST_LOG=pet=debug for more detailed traces.
/// Set PET_TRACE_FORMAT=json for JSON output (useful for analysis tools).
///
/// Note: This replaces the env_logger initialization since tracing-subscriber
/// provides a log compatibility layer via tracing-log.
pub fn initialize_tracing(verbose: bool) {
    use std::sync::Once;
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        let filter = if verbose {
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("pet=debug"))
        } else {
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"))
        };

        let use_json = env::var("PET_TRACE_FORMAT")
            .map(|v| v == "json")
            .unwrap_or(false);

        if use_json {
            tracing_subscriber::registry()
                .with(filter)
                .with(fmt::layer().json().with_writer(std::io::stderr))
                .init();
        } else {
            tracing_subscriber::registry()
                .with(filter)
                .with(
                    fmt::layer()
                        .with_target(true)
                        .with_timer(fmt::time::uptime())
                        .with_writer(std::io::stderr),
                )
                .init();
        }
    });
}

#[derive(Debug, Clone)]
pub struct FindOptions {
    pub print_list: bool,
    pub print_summary: bool,
    pub verbose: bool,
    pub report_missing: bool,
    pub search_paths: Option<Vec<PathBuf>>,
    pub workspace_only: bool,
    pub cache_directory: Option<PathBuf>,
    pub kind: Option<PythonEnvironmentKind>,
    pub json: bool,
    pub conda_executable: Option<PathBuf>,
    pub pipenv_executable: Option<PathBuf>,
    pub poetry_executable: Option<PathBuf>,
    pub environment_directories: Option<Vec<PathBuf>>,
}

pub fn find_and_report_envs_stdio(options: FindOptions) {
    // Initialize tracing for performance profiling (includes log compatibility)
    initialize_tracing(options.verbose);

    // Note: We don't call stdio::initialize_logger here anymore since
    // tracing-subscriber provides log compatibility via tracing-log crate.
    // stdio::initialize_logger would conflict with our tracing subscriber.

    let now = SystemTime::now();
    let config = create_config(&options);
    let search_scope = if options.workspace_only {
        Some(SearchScope::Workspace)
    } else {
        options.kind.map(SearchScope::Global)
    };

    if let Some(cache_directory) = options.cache_directory.clone() {
        set_cache_directory(cache_directory);
    }
    let environment = EnvironmentApi::new();
    let conda_locator = Arc::new(Conda::from(&environment));
    let poetry_locator = Arc::new(Poetry::from(&environment));

    let locators = create_locators(conda_locator.clone(), poetry_locator.clone(), &environment);
    for locator in locators.iter() {
        locator.configure(&config);
    }

    if options.json {
        find_envs_json(
            &options,
            &locators,
            config,
            conda_locator.as_ref(),
            poetry_locator.as_ref(),
            &environment,
            search_scope,
        );
    } else {
        find_envs(
            &options,
            &locators,
            config,
            conda_locator.as_ref(),
            poetry_locator.as_ref(),
            &environment,
            search_scope,
        );

        println!("Completed in {}ms", now.elapsed().unwrap().as_millis())
    }
}

fn create_config(options: &FindOptions) -> Configuration {
    let mut config = Configuration::default();

    let mut search_paths = vec![];
    if let Some(dirs) = options.search_paths.as_ref() {
        search_paths.extend(expand_glob_patterns(dirs));
    }
    // If workspace folders have been provided do not add cwd.
    if search_paths.is_empty() {
        if let Ok(cwd) = env::current_dir() {
            search_paths.push(cwd);
        }
    }
    search_paths.sort();
    search_paths.dedup();

    config.workspace_directories = Some(
        search_paths
            .iter()
            .filter(|d| d.is_dir())
            .cloned()
            .collect(),
    );
    config.executables = Some(
        search_paths
            .iter()
            .filter(|d| d.is_file())
            .cloned()
            .collect(),
    );

    config.conda_executable = options.conda_executable.clone();
    config.pipenv_executable = options.pipenv_executable.clone();
    config.poetry_executable = options.poetry_executable.clone();
    config.environment_directories = options.environment_directories.as_ref().map(|dirs| {
        expand_glob_patterns(dirs)
            .into_iter()
            .filter(|p| p.is_dir())
            .collect()
    });

    config
}

fn find_envs(
    options: &FindOptions,
    locators: &Arc<Vec<Arc<dyn Locator>>>,
    config: Configuration,
    conda_locator: &Conda,
    poetry_locator: &Poetry,
    environment: &dyn Environment,
    search_scope: Option<SearchScope>,
) {
    let kind = match search_scope {
        Some(SearchScope::Global(kind)) => Some(kind),
        _ => None,
    };
    let stdio_reporter = Arc::new(stdio::create_reporter(options.print_list, kind));
    let reporter = CacheReporter::new(stdio_reporter.clone());

    let summary =
        find_and_report_envs(&reporter, config, locators, environment, search_scope, None);
    if options.report_missing {
        // By now all conda envs have been found
        // Spawn conda
        // & see if we can find more environments by spawning conda.
        let _ =
            conda_locator.find_and_report_missing_envs(&reporter, options.conda_executable.clone());
        let _ = poetry_locator
            .find_and_report_missing_envs(&reporter, options.poetry_executable.clone());
    }

    if options.print_summary {
        let summary = summary.lock().expect("summary mutex poisoned");
        if !summary.locators.is_empty() {
            println!();
            println!("Breakdown by each locator:");
            println!("--------------------------");
            for locator in summary.locators.iter() {
                println!("{:<20} : {:?}", format!("{:?}", locator.0), locator.1);
            }
            println!()
        }

        if !summary.breakdown.is_empty() {
            println!("Breakdown for finding Environments:");
            println!("-----------------------------------");
            for item in summary.breakdown.iter() {
                println!("{:<20} : {:?}", item.0, item.1);
            }
            println!();
        }

        let summary = stdio_reporter.get_summary();

        // If verbose, print the paths of discovered environments first
        if options.verbose && !summary.environment_paths.is_empty() {
            println!("Environment Paths:");
            println!("------------------");
            for (kind, envs) in summary.environment_paths.iter() {
                let kind_str = kind
                    .map(|v| format!("{v:?}"))
                    .unwrap_or("Unknown".to_string());
                println!("\n{kind_str}:");
                for env in envs {
                    if let Some(executable) = &env.executable {
                        println!("  - {}", executable.display());
                    }
                }
            }
            println!()
        }

        if !summary.managers.is_empty() {
            println!("Managers:");
            println!("---------");
            for (k, v) in summary
                .managers
                .clone()
                .into_iter()
                .map(|(k, v)| (format!("{k:?}"), v))
                .collect::<BTreeMap<String, u16>>()
            {
                println!("{k:<20} : {v:?}");
            }
            println!()
        }
        if !summary.environments.is_empty() {
            let total = summary
                .environments
                .clone()
                .iter()
                .fold(0, |total, b| total + b.1);
            println!("Environments ({total}):");
            println!("------------------");
            for (k, v) in summary
                .environments
                .clone()
                .into_iter()
                .map(|(k, v)| {
                    (
                        k.map(|v| format!("{v:?}")).unwrap_or("Unknown".to_string()),
                        v,
                    )
                })
                .collect::<BTreeMap<String, u16>>()
            {
                println!("{k:<20} : {v:?}");
            }
            println!()
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonOutput {
    managers: Vec<pet_core::manager::EnvManager>,
    environments: Vec<pet_core::python_environment::PythonEnvironment>,
}

fn find_envs_json(
    options: &FindOptions,
    locators: &Arc<Vec<Arc<dyn Locator>>>,
    config: Configuration,
    conda_locator: &Conda,
    poetry_locator: &Poetry,
    environment: &dyn Environment,
    search_scope: Option<SearchScope>,
) {
    let collect_reporter = Arc::new(collect::create_reporter());
    let reporter = CacheReporter::new(collect_reporter.clone());

    find_and_report_envs(&reporter, config, locators, environment, search_scope, None);
    if options.report_missing {
        let _ =
            conda_locator.find_and_report_missing_envs(&reporter, options.conda_executable.clone());
        let _ = poetry_locator
            .find_and_report_missing_envs(&reporter, options.poetry_executable.clone());
    }

    let managers = collect_reporter
        .managers
        .lock()
        .expect("managers mutex poisoned")
        .clone();
    let mut environments = collect_reporter
        .environments
        .lock()
        .expect("environments mutex poisoned")
        .clone();

    if let Some(kind) = options.kind {
        environments.retain(|e| e.kind == Some(kind));
    }

    let output = JsonOutput {
        managers,
        environments,
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&output).expect("failed to serialize environments as JSON")
    );
}

pub fn resolve_report_stdio(
    executable: PathBuf,
    verbose: bool,
    cache_directory: Option<PathBuf>,
    json: bool,
) {
    // Initialize tracing for performance profiling (includes log compatibility)
    initialize_tracing(verbose);

    // Note: We don't call stdio::initialize_logger here anymore since
    // tracing-subscriber provides log compatibility via tracing-log crate.

    let now = SystemTime::now();

    if let Some(cache_directory) = cache_directory.clone() {
        set_cache_directory(cache_directory);
    }

    let stdio_reporter = Arc::new(stdio::create_reporter(true, None));
    let reporter = CacheReporter::new(stdio_reporter.clone());
    let environment = EnvironmentApi::new();
    let conda_locator = Arc::new(Conda::from(&environment));
    let poetry_locator = Arc::new(Poetry::from(&environment));

    let mut config = Configuration::default();
    if let Ok(cwd) = env::current_dir() {
        config.workspace_directories = Some(vec![cwd]);
    }

    let locators = create_locators(conda_locator.clone(), poetry_locator.clone(), &environment);
    for locator in locators.iter() {
        locator.configure(&config);
    }

    if let Some(result) = resolve_environment(&executable, &locators, &environment) {
        let env = &result.resolved.unwrap_or(result.discovered);
        if json {
            println!(
                "{}",
                serde_json::to_string_pretty(env).expect("failed to serialize environment as JSON")
            );
        } else {
            println!("Environment found for {executable:?}");
            if let Some(manager) = &env.manager {
                reporter.report_manager(manager);
            }
            reporter.report_environment(env);
        }
    } else if json {
        println!("null");
    } else {
        println!("No environment found for {executable:?}");
    }

    if !json {
        println!(
            "Resolve completed in {}ms",
            now.elapsed().unwrap().as_millis()
        )
    }
}
