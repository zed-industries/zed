use std::collections::{BTreeSet, HashMap};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;

use ::fs::{CopyOptions, Fs, RealFs, copy_recursive};
use anyhow::{Context, Result, anyhow, bail};
use clap::Parser;
use extension::ExtensionManifest;
use extension::extension_builder::{CompileExtensionOptions, ExtensionBuilder};
use language::LanguageConfig;
use reqwest_client::ReqwestClient;
use rpc::ExtensionProvides;
use tree_sitter::{Language, Query, WasmStore};

#[derive(Parser, Debug)]
#[command(name = "zed-extension")]
struct Args {
    /// The path to the extension directory
    #[arg(long)]
    source_dir: PathBuf,
    /// The output directory to place the packaged extension.
    #[arg(long)]
    output_dir: PathBuf,
    /// The path to a directory where build dependencies are downloaded
    #[arg(long)]
    scratch_dir: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();
    let fs = Arc::new(RealFs::new(None, gpui::background_executor()));
    let engine = wasmtime::Engine::default();
    let mut wasm_store = WasmStore::new(&engine)?;

    let extension_path = args
        .source_dir
        .canonicalize()
        .context("failed to canonicalize source_dir")?;
    let scratch_dir = args
        .scratch_dir
        .canonicalize()
        .context("failed to canonicalize scratch_dir")?;
    let output_dir = if args.output_dir.is_relative() {
        env::current_dir()?.join(&args.output_dir)
    } else {
        args.output_dir
    };

    log::info!("loading extension manifest");
    let mut manifest = ExtensionManifest::load(fs.clone(), &extension_path).await?;

    log::info!("compiling extension");

    let user_agent = format!(
        "Zed Extension CLI/{} ({}; {})",
        env!("CARGO_PKG_VERSION"),
        std::env::consts::OS,
        std::env::consts::ARCH
    );
    let http_client = Arc::new(ReqwestClient::user_agent(&user_agent)?);

    let builder = ExtensionBuilder::new(http_client, scratch_dir);
    builder
        .compile_extension(
            &extension_path,
            &mut manifest,
            CompileExtensionOptions { release: true },
        )
        .await
        .context("failed to compile extension")?;

    let grammars = test_grammars(&manifest, &extension_path, &mut wasm_store)?;
    test_languages(&manifest, &extension_path, &grammars)?;
    test_themes(&manifest, &extension_path, fs.clone()).await?;

    let archive_dir = output_dir.join("archive");
    fs::remove_dir_all(&archive_dir).ok();
    copy_extension_resources(&manifest, &extension_path, &archive_dir, fs.clone())
        .await
        .context("failed to copy extension resources")?;

    let tar_output = Command::new("tar")
        .current_dir(&output_dir)
        .args(["-czvf", "archive.tar.gz", "-C", "archive", "."])
        .output()
        .context("failed to run tar")?;
    if !tar_output.status.success() {
        bail!(
            "failed to create archive.tar.gz: {}",
            String::from_utf8_lossy(&tar_output.stderr)
        );
    }

    let extension_provides = extension_provides(&manifest);

    let manifest_json = serde_json::to_string(&rpc::ExtensionApiManifest {
        name: manifest.name,
        version: manifest.version,
        description: manifest.description,
        authors: manifest.authors,
        schema_version: Some(manifest.schema_version.0),
        repository: manifest
            .repository
            .ok_or_else(|| anyhow!("missing repository in extension manifest"))?,
        wasm_api_version: manifest.lib.version.map(|version| version.to_string()),
        provides: extension_provides,
    })?;
    fs::remove_dir_all(&archive_dir)?;
    fs::write(output_dir.join("manifest.json"), manifest_json.as_bytes())?;

    Ok(())
}

/// Returns the set of features provided by the extension.
fn extension_provides(manifest: &ExtensionManifest) -> BTreeSet<ExtensionProvides> {
    let mut provides = BTreeSet::default();
    if !manifest.themes.is_empty() {
        provides.insert(ExtensionProvides::Themes);
    }

    if !manifest.icon_themes.is_empty() {
        provides.insert(ExtensionProvides::IconThemes);
    }

    if !manifest.languages.is_empty() {
        provides.insert(ExtensionProvides::Languages);
    }

    if !manifest.grammars.is_empty() {
        provides.insert(ExtensionProvides::Grammars);
    }

    if !manifest.language_servers.is_empty() {
        provides.insert(ExtensionProvides::LanguageServers);
    }

    if !manifest.context_servers.is_empty() {
        provides.insert(ExtensionProvides::ContextServers);
    }

    if !manifest.indexed_docs_providers.is_empty() {
        provides.insert(ExtensionProvides::IndexedDocsProviders);
    }

    if manifest.snippets.is_some() {
        provides.insert(ExtensionProvides::Snippets);
    }

    provides
}

async fn copy_extension_resources(
    manifest: &ExtensionManifest,
    extension_path: &Path,
    output_dir: &Path,
    fs: Arc<dyn Fs>,
) -> Result<()> {
    fs::create_dir_all(output_dir).context("failed to create output dir")?;

    let manifest_toml = toml::to_string(&manifest).context("failed to serialize manifest")?;
    fs::write(output_dir.join("extension.toml"), &manifest_toml)
        .context("failed to write extension.toml")?;

    if manifest.lib.kind.is_some() {
        fs::copy(
            extension_path.join("extension.wasm"),
            output_dir.join("extension.wasm"),
        )
        .context("failed to copy extension.wasm")?;
    }

    if !manifest.grammars.is_empty() {
        let source_grammars_dir = extension_path.join("grammars");
        let output_grammars_dir = output_dir.join("grammars");
        fs::create_dir_all(&output_grammars_dir)?;
        for grammar_name in manifest.grammars.keys() {
            let mut grammar_filename = PathBuf::from(grammar_name.as_ref());
            grammar_filename.set_extension("wasm");
            fs::copy(
                source_grammars_dir.join(&grammar_filename),
                output_grammars_dir.join(&grammar_filename),
            )
            .with_context(|| format!("failed to copy grammar '{}'", grammar_filename.display()))?;
        }
    }

    if !manifest.themes.is_empty() {
        let output_themes_dir = output_dir.join("themes");
        fs::create_dir_all(&output_themes_dir)?;
        for theme_path in &manifest.themes {
            fs::copy(
                extension_path.join(theme_path),
                output_themes_dir.join(
                    theme_path
                        .file_name()
                        .ok_or_else(|| anyhow!("invalid theme path"))?,
                ),
            )
            .with_context(|| format!("failed to copy theme '{}'", theme_path.display()))?;
        }
    }

    if !manifest.icon_themes.is_empty() {
        let output_icon_themes_dir = output_dir.join("icon_themes");
        fs::create_dir_all(&output_icon_themes_dir)?;
        for icon_theme_path in &manifest.icon_themes {
            fs::copy(
                extension_path.join(icon_theme_path),
                output_icon_themes_dir.join(
                    icon_theme_path
                        .file_name()
                        .ok_or_else(|| anyhow!("invalid icon theme path"))?,
                ),
            )
            .with_context(|| {
                format!("failed to copy icon theme '{}'", icon_theme_path.display())
            })?;
        }

        let output_icons_dir = output_dir.join("icons");
        fs::create_dir_all(&output_icons_dir)?;
        copy_recursive(
            fs.as_ref(),
            &extension_path.join("icons"),
            &output_icons_dir,
            CopyOptions {
                overwrite: true,
                ignore_if_exists: false,
            },
        )
        .await
        .with_context(|| "failed to copy icons")?;
    }

    if !manifest.languages.is_empty() {
        let output_languages_dir = output_dir.join("languages");
        fs::create_dir_all(&output_languages_dir)?;
        for language_path in &manifest.languages {
            copy_recursive(
                fs.as_ref(),
                &extension_path.join(language_path),
                &output_languages_dir.join(
                    language_path
                        .file_name()
                        .ok_or_else(|| anyhow!("invalid language path"))?,
                ),
                CopyOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await
            .with_context(|| {
                format!("failed to copy language dir '{}'", language_path.display())
            })?;
        }
    }

    Ok(())
}

fn test_grammars(
    manifest: &ExtensionManifest,
    extension_path: &Path,
    wasm_store: &mut WasmStore,
) -> Result<HashMap<String, Language>> {
    let mut grammars = HashMap::default();
    let grammars_dir = extension_path.join("grammars");

    for grammar_name in manifest.grammars.keys() {
        let mut grammar_path = grammars_dir.join(grammar_name.as_ref());
        grammar_path.set_extension("wasm");

        let wasm = fs::read(&grammar_path)?;
        let language = wasm_store.load_language(grammar_name, &wasm)?;
        log::info!("loaded grammar {grammar_name}");
        grammars.insert(grammar_name.to_string(), language);
    }

    Ok(grammars)
}

fn test_languages(
    manifest: &ExtensionManifest,
    extension_path: &Path,
    grammars: &HashMap<String, Language>,
) -> Result<()> {
    for relative_language_dir in &manifest.languages {
        let language_dir = extension_path.join(relative_language_dir);
        let config_path = language_dir.join("config.toml");
        let config_content = fs::read_to_string(&config_path)?;
        let config: LanguageConfig = toml::from_str(&config_content)?;
        let grammar = if let Some(name) = &config.grammar {
            Some(
                grammars
                    .get(name.as_ref())
                    .ok_or_else(|| anyhow!("grammar not found: '{name}'"))?,
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

async fn test_themes(
    manifest: &ExtensionManifest,
    extension_path: &Path,
    fs: Arc<dyn Fs>,
) -> Result<()> {
    for relative_theme_path in &manifest.themes {
        let theme_path = extension_path.join(relative_theme_path);
        let theme_family = theme::read_user_theme(&theme_path, fs.clone()).await?;
        log::info!("loaded theme family {}", theme_family.name);

        for theme in &theme_family.themes {
            if theme
                .style
                .colors
                .deprecated_scrollbar_thumb_background
                .is_some()
            {
                bail!(
                    r#"Theme "{theme_name}" is using a deprecated style property: scrollbar_thumb.background. Use `scrollbar.thumb.background` instead."#,
                    theme_name = theme.name
                )
            }
        }
    }

    Ok(())
}
