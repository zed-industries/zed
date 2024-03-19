use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use ::fs::{Fs, RealFs};
use anyhow::{anyhow, Context, Result};
use clap::Parser;
use extension::{
    extension_builder::{CompileExtensionOptions, ExtensionBuilder},
    ExtensionStore,
};
use language::LanguageConfig;
use theme::ThemeRegistry;
use tree_sitter::{Language, Query, WasmStore};

#[derive(Parser, Debug)]
#[command(name = "zed-extension")]
struct Args {
    /// The path to the extension directory
    extension_path: PathBuf,
    /// Whether to compile with optimizations
    #[arg(long)]
    release: bool,
    /// The path to a directory where build dependencies are downloaded
    #[arg(long)]
    scratch_dir: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();
    let fs = Arc::new(RealFs);
    let engine = wasmtime::Engine::default();
    let mut wasm_store = WasmStore::new(engine)?;

    let extension_path = args
        .extension_path
        .canonicalize()
        .context("can't canonicalize extension_path")?;
    let scratch_dir = args
        .scratch_dir
        .canonicalize()
        .context("can't canonicalize scratch_dir")?;

    let manifest = ExtensionStore::load_extension_manifest(fs.clone(), &extension_path).await?;
    let builder = ExtensionBuilder::new(scratch_dir);
    builder
        .compile_extension(
            &extension_path,
            &manifest,
            CompileExtensionOptions {
                release: args.release,
            },
        )
        .await?;

    let grammars = test_grammars(&extension_path, &mut wasm_store)?;
    test_languages(&extension_path, &grammars)?;
    test_themes(&extension_path, fs.clone()).await?;

    Ok(())
}

fn test_grammars(
    extension_path: &Path,
    wasm_store: &mut WasmStore,
) -> Result<HashMap<String, Language>> {
    let mut grammars = HashMap::default();
    let grammars_dir = extension_path.join("grammars");
    if !grammars_dir.exists() {
        return Ok(grammars);
    }

    let entries = fs::read_dir(&grammars_dir)?;
    for entry in entries {
        let entry = entry?;
        let grammar_path = entry.path();
        let grammar_name = grammar_path.file_stem().unwrap().to_str().unwrap();
        if grammar_path.extension() == Some("wasm".as_ref()) {
            let wasm = fs::read(&grammar_path)?;
            let language = wasm_store.load_language(grammar_name, &wasm)?;
            log::info!("loaded grammar {grammar_name}");
            grammars.insert(grammar_name.into(), language);
        }
    }

    Ok(grammars)
}

fn test_languages(extension_path: &Path, grammars: &HashMap<String, Language>) -> Result<()> {
    let languages_dir = extension_path.join("languages");
    if !languages_dir.exists() {
        return Ok(());
    }

    let entries = fs::read_dir(&languages_dir)?;
    for entry in entries {
        let entry = entry?;
        let language_dir = entry.path();
        let config_path = language_dir.join("config.toml");
        let config_content = fs::read_to_string(&config_path)?;
        let config: LanguageConfig = toml::from_str(&config_content)?;
        let grammar = if let Some(name) = &config.grammar {
            Some(
                grammars
                    .get(name.as_ref())
                    .ok_or_else(|| anyhow!("language"))?,
            )
        } else {
            None
        };

        let query_entries = fs::read_dir(&language_dir)?;
        for entry in query_entries {
            let entry = entry?;
            let query_path = entry.path();
            if query_path.extension() == Some("scm".as_ref()) {
                let grammar = grammar.ok_or_else(|| {
                    anyhow!(
                        "language {} provides query {} but no grammar",
                        config.name,
                        query_path.display()
                    )
                })?;

                let query_source = fs::read_to_string(&query_path)?;
                let _query = Query::new(grammar, &query_source)?;
            }
        }

        log::info!("loaded language {}", config.name);
    }

    Ok(())
}

async fn test_themes(extension_path: &Path, fs: Arc<dyn Fs>) -> Result<()> {
    let themes_dir = extension_path.join("themes");
    if !themes_dir.exists() {
        return Ok(());
    }

    let entries = fs::read_dir(&themes_dir)?;
    for entry in entries {
        let entry = entry?;
        let theme_path = entry.path();
        if theme_path.extension() == Some("json".as_ref()) {
            let theme_family = ThemeRegistry::read_user_theme(&entry.path(), fs.clone()).await?;
            log::info!("loaded theme family {}", theme_family.name);
        }
    }

    Ok(())
}
