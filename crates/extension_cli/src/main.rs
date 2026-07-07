use std::collections::BTreeSet;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr as _;
use std::sync::Arc;

use ::fs::{CopyOptions, Fs, RealFs, RemoveOptions, copy_recursive};
use anyhow::{Context as _, Result, anyhow, bail};
use clap::Parser;
use cloud_api_types::ExtensionProvides;
use extension::build_debug_adapter_schema_path;
use extension::extension_builder::CompilationConcurrency;
use extension::extension_builder::{CompileExtensionOptions, ExtensionBuilder};
use extension::{ExtensionManifest, ExtensionSnippets};
use language::LanguageConfig;
use reqwest_client::ReqwestClient;
use settings_content::SemanticTokenRules;
use snippet_provider::file_to_snippets;
use snippet_provider::format::VsSnippetsFile;
use task::TaskTemplates;
use tokio::process::Command;
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
    let fs = Arc::new(RealFs::new(None, gpui_platform::background_executor()));
    let engine = wasmtime::Engine::default();
    let mut wasm_store = WasmStore::new(&engine)?;

    let extension_path = args
        .source_dir
        .canonicalize()
        .context("failed to canonicalize source_dir")?;

    fs.create_dir(&args.scratch_dir)
        .await
        .context("failed to create scratch dir")?;

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
            CompileExtensionOptions {
                release: true,
                max_concurrency: CompilationConcurrency::Unbounded,
            },
            fs.clone(),
        )
        .await
        .context("failed to compile extension")?;

    let extension_provides = manifest.provides();
    validate_extension_features(&extension_provides)?;

    let grammars = test_grammars(&manifest, &extension_path, &mut wasm_store)?;
    test_languages(&manifest, &extension_path, &grammars)?;
    test_themes(&manifest, &extension_path, fs.clone()).await?;
    test_snippets(&manifest, &extension_path, fs.clone()).await?;
    test_debug_adapter_schemas(&manifest, &extension_path, fs.clone()).await?;

    let archive_dir = output_dir.join("archive");
    fs.remove_dir(
        &archive_dir,
        RemoveOptions {
            recursive: true,
            ignore_if_not_exists: true,
        },
    )
    .await
    .ok();
    copy_extension_resources(&manifest, &extension_path, &archive_dir, fs.clone())
        .await
        .context("failed to copy extension resources")?;

    let tar_output = Command::new("tar")
        .current_dir(&output_dir)
        .args(["-czvf", "archive.tar.gz", "-C", "archive", "."])
        .output()
        .await
        .context("failed to run tar")?;
    if !tar_output.status.success() {
        bail!(
            "failed to create archive.tar.gz: {}",
            String::from_utf8_lossy(&tar_output.stderr)
        );
    }

    let manifest_json = serde_json::to_string(&cloud_api_types::ExtensionApiManifest {
        name: manifest.name,
        version: manifest.version,
        description: manifest.description,
        authors: manifest.authors,
        schema_version: Some(manifest.schema_version.0),
        repository: manifest
            .repository
            .context("missing repository in extension manifest")?,
        wasm_api_version: manifest.lib.version.map(|version| version.to_string()),
        provides: extension_provides,
    })?;
    fs.remove_dir(
        &archive_dir,
        RemoveOptions {
            recursive: true,
            ignore_if_not_exists: false,
        },
    )
    .await?;
    fs.write(&output_dir.join("manifest.json"), manifest_json.as_bytes())
        .await?;

    Ok(())
}

async fn copy_extension_resources(
    manifest: &ExtensionManifest,
    extension_path: &Path,
    output_dir: &Path,
    fs: Arc<dyn Fs>,
) -> Result<()> {
    fs.create_dir(output_dir)
        .await
        .context("failed to create output dir")?;

    let manifest_toml = toml::to_string(&manifest).context("failed to serialize manifest")?;
    fs.write(&output_dir.join("extension.toml"), manifest_toml.as_bytes())
        .await
        .context("failed to write extension.toml")?;

    if manifest.lib.kind.is_some() {
        fs.copy_file(
            &extension_path.join("extension.wasm"),
            &output_dir.join("extension.wasm"),
            CopyOptions {
                overwrite: true,
                ignore_if_exists: false,
            },
        )
        .await
        .context("failed to copy extension.wasm")?;
    }

    if !manifest.grammars.is_empty() {
        let source_grammars_dir = extension_path.join("grammars");
        let output_grammars_dir = output_dir.join("grammars");
        fs.create_dir(&output_grammars_dir).await?;
        futures::future::try_join_all(manifest.grammars.keys().map(|grammar_name| {
            let fs = fs.clone();
            let source_grammars_dir = source_grammars_dir.as_path();
            let output_grammars_dir = output_grammars_dir.as_path();
            async move {
                let mut grammar_filename = PathBuf::from(grammar_name.as_ref());
                grammar_filename.set_extension("wasm");
                fs.copy_file(
                    &source_grammars_dir.join(&grammar_filename),
                    &output_grammars_dir.join(&grammar_filename),
                    CopyOptions {
                        overwrite: true,
                        ignore_if_exists: false,
                    },
                )
                .await
                .with_context(|| format!("failed to copy grammar '{}'", grammar_filename.display()))
            }
        }))
        .await?;
    }

    if !manifest.themes.is_empty() {
        let output_themes_dir = output_dir.join("themes");
        fs.create_dir(&output_themes_dir).await?;
        futures::future::try_join_all(manifest.themes.iter().map(|theme_path| {
            let fs = fs.clone();
            let output_themes_dir = output_themes_dir.as_path();
            async move {
                let theme_path = theme_path.as_std_path();
                fs.copy_file(
                    &extension_path.join(theme_path),
                    &output_themes_dir.join(theme_path.file_name().context("invalid theme path")?),
                    CopyOptions {
                        overwrite: true,
                        ignore_if_exists: false,
                    },
                )
                .await
                .with_context(|| format!("failed to copy theme '{}'", theme_path.display()))
            }
        }))
        .await?;
    }

    if !manifest.icon_themes.is_empty() {
        let output_icon_themes_dir = output_dir.join("icon_themes");
        fs.create_dir(&output_icon_themes_dir).await?;
        futures::future::try_join_all(manifest.icon_themes.iter().map(|icon_theme_path| {
            let fs = fs.clone();
            let output_icon_themes_dir = output_icon_themes_dir.as_path();
            async move {
                let icon_theme_path = icon_theme_path.as_std_path();
                fs.copy_file(
                    &extension_path.join(icon_theme_path),
                    &output_icon_themes_dir.join(
                        icon_theme_path
                            .file_name()
                            .context("invalid icon theme path")?,
                    ),
                    CopyOptions {
                        overwrite: true,
                        ignore_if_exists: false,
                    },
                )
                .await
                .with_context(|| {
                    format!("failed to copy icon theme '{}'", icon_theme_path.display())
                })
            }
        }))
        .await?;

        let output_icons_dir = output_dir.join("icons");
        fs.create_dir(&output_icons_dir).await?;
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
        .context("failed to copy icons")?;
    }

    if !manifest.languages.is_empty() {
        let output_languages_dir = output_dir.join("languages");
        fs.create_dir(&output_languages_dir).await?;
        futures::future::try_join_all(manifest.languages.iter().map(|language_path| {
            let fs = fs.clone();
            let output_languages_dir = output_languages_dir.clone();
            async move {
                let language_path = language_path.as_std_path();
                copy_recursive(
                    fs.as_ref(),
                    &extension_path.join(language_path),
                    &output_languages_dir
                        .join(language_path.file_name().context("invalid language path")?),
                    CopyOptions {
                        overwrite: true,
                        ignore_if_exists: false,
                    },
                )
                .await
                .with_context(|| {
                    format!("failed to copy language dir '{}'", language_path.display())
                })
            }
        }))
        .await?;
    }

    if !manifest.debug_adapters.is_empty() {
        futures::future::try_join_all(manifest.debug_adapters.iter().map(
            |(debug_adapter, entry)| {
                let fs = fs.clone();
                let debug_adapter = debug_adapter.clone();
                async move {
                    let schema_path =
                        extension::build_debug_adapter_schema_path(&debug_adapter, &entry)?;
                    let parent = schema_path.parent().with_context(|| {
                        format!("invalid empty schema path for {debug_adapter}")
                    })?;
                    let schema_path = schema_path.as_std_path();
                    fs.create_dir(&output_dir.join(parent)).await?;
                    copy_recursive(
                        fs.as_ref(),
                        &extension_path.join(schema_path),
                        &output_dir.join(schema_path),
                        CopyOptions {
                            overwrite: true,
                            ignore_if_exists: false,
                        },
                    )
                    .await
                    .with_context(|| {
                        format!(
                            "failed to copy debug adapter schema '{}'",
                            schema_path.display(),
                        )
                    })
                }
            },
        ))
        .await?;
    }

    if let Some(snippets) = manifest.snippets.as_ref() {
        futures::future::try_join_all(snippets.paths().map(|snippets_path| {
            let fs = fs.clone();
            async move {
                let parent = snippets_path.parent();
                if let Some(parent) = parent.filter(|p| p.components().next().is_some()) {
                    fs.create_dir(&output_dir.join(parent)).await?;
                }
                copy_recursive(
                    fs.as_ref(),
                    &extension_path.join(&snippets_path),
                    &output_dir.join(&snippets_path),
                    CopyOptions {
                        overwrite: true,
                        ignore_if_exists: false,
                    },
                )
                .await
                .with_context(|| {
                    format!("failed to copy snippets from '{}'", snippets_path.display())
                })
            }
        }))
        .await?;
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
enum ExtensionFeatureError {
    #[error("extension does not provide any features")]
    NoFeatures,
    #[error("extension must not provide other features along with themes")]
    ThemesMixedWithOtherFeatures,
    #[error("extension must not provide other features along with icon themes")]
    IconThemesMixedWithOtherFeatures,
    #[error(
        "Slash commands have been deprecated and \
        the slash command API will be removed in a future release. {}",
        if *.sole_feature {
            "Slash command extensions will no longer be accepted at this time."
        } else {
            "Please remove any slash-command related code from your extension."
        }
    )]
    SlashCommandsDeprecated { sole_feature: bool },
}

fn validate_extension_features(
    provides: &BTreeSet<ExtensionProvides>,
) -> Result<(), ExtensionFeatureError> {
    if provides.is_empty() {
        return Err(ExtensionFeatureError::NoFeatures);
    }

    let provides_single_feature = provides.len() == 1;

    if provides.contains(&ExtensionProvides::Themes) && !provides_single_feature {
        return Err(ExtensionFeatureError::ThemesMixedWithOtherFeatures);
    }

    if provides.contains(&ExtensionProvides::IconThemes) && !provides_single_feature {
        return Err(ExtensionFeatureError::IconThemesMixedWithOtherFeatures);
    }

    if provides.contains(&ExtensionProvides::SlashCommands) {
        return Err(ExtensionFeatureError::SlashCommandsDeprecated {
            sole_feature: provides_single_feature,
        });
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
        let config_path = language_dir.join(LanguageConfig::FILE_NAME);
        let config = LanguageConfig::load(&config_path)?;
        let grammar = if let Some(name) = &config.grammar {
            Some(
                grammars
                    .get(name.as_ref())
                    .with_context(|| format!("grammar not found: '{name}'"))?,
            )
        } else {
            None
        };

        let query_entries = fs::read_dir(&language_dir)?;
        for entry in query_entries {
            let entry = entry?;
            let file_path = entry.path();

            let Some(file_name) = file_path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };

            match file_name {
                LanguageConfig::FILE_NAME => {
                    // Loaded above
                }
                SemanticTokenRules::FILE_NAME => {
                    let _token_rules = SemanticTokenRules::load(&file_path)?;
                }
                TaskTemplates::FILE_NAME => {
                    let task_file_content = std::fs::read(&file_path).with_context(|| {
                        anyhow!(
                            "Failed to read tasks file at {path}",
                            path = file_path.display()
                        )
                    })?;
                    let _task_templates =
                        serde_json_lenient::from_slice::<TaskTemplates>(&task_file_content)
                            .with_context(|| {
                                anyhow!(
                                    "Failed to parse tasks file at {path}",
                                    path = file_path.display()
                                )
                            })?;
                }
                _ if file_name.ends_with(".scm") => {
                    let grammar = grammar.with_context(|| {
                        format! {
                            "language {} provides query {} but no grammar",
                            config.name,
                            file_path.display()
                        }
                    })?;

                    let query_source = fs::read_to_string(&file_path)?;
                    let _query = Query::new(grammar, &query_source)?;
                }
                _ => {}
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
        let theme_family =
            theme_settings::deserialize_user_theme(&fs.load_bytes(&theme_path).await?)?;
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

async fn test_snippets(
    manifest: &ExtensionManifest,
    extension_path: &Path,
    fs: Arc<dyn Fs>,
) -> Result<()> {
    for relative_snippet_path in manifest
        .snippets
        .as_ref()
        .map(ExtensionSnippets::paths)
        .into_iter()
        .flatten()
    {
        let snippet_path = extension_path.join(relative_snippet_path);
        let snippets_content = fs.load_bytes(&snippet_path).await?;
        let snippets_file = serde_json_lenient::from_slice::<VsSnippetsFile>(&snippets_content)
            .with_context(|| anyhow!("Failed to parse snippet file at {snippet_path:?}"))?;
        let snippet_errors = file_to_snippets(snippets_file, &snippet_path)
            .flat_map(Result::err)
            .collect::<Vec<_>>();
        let error_count = snippet_errors.len();

        anyhow::ensure!(
            error_count == 0,
            "Could not parse {error_count} snippet{suffix} in file {snippet_path:?}:\n\n{snippet_errors}",
            suffix = if error_count == 1 { "" } else { "s" },
            snippet_errors = snippet_errors
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n")
        );
    }

    Ok(())
}

async fn test_debug_adapter_schemas(
    manifest: &ExtensionManifest,
    extension_path: &Path,
    fs: Arc<dyn Fs>,
) -> Result<()> {
    futures::future::try_join_all(manifest.debug_adapters.iter().map(
        |(debug_adapter_name, meta)| {
            let fs = fs.clone();
            async move {
                let debug_adapter_schema_path =
                    extension_path.join(build_debug_adapter_schema_path(debug_adapter_name, meta)?);

                let debug_adapter_schema =
                    fs.load(&debug_adapter_schema_path).await.with_context(|| {
                        anyhow::anyhow!(
                            "failed to read debug adapter schema for \
                        `{debug_adapter_name}` from `{debug_adapter_schema_path:?}`"
                        )
                    })?;
                _ = serde_json::Value::from_str(&debug_adapter_schema).with_context(|| {
                    anyhow::anyhow!(
                        "Debug adapter schema for `{debug_adapter_name}`\
                        (path: `{debug_adapter_schema_path:?}`) is not a valid JSON"
                    )
                })?;

                Ok(())
            }
        },
    ))
    .await
    .map(|_| ())
}

#[cfg(test)]
mod tests {
    use cloud_api_types::ExtensionProvides;

    use super::*;

    #[test]
    fn test_validate_empty_features() {
        let provides = BTreeSet::new();
        assert_eq!(
            validate_extension_features(&provides),
            Err(ExtensionFeatureError::NoFeatures),
        );
    }

    #[test]
    fn test_validate_single_language_feature() {
        let provides = BTreeSet::from([ExtensionProvides::Languages]);
        assert_eq!(validate_extension_features(&provides), Ok(()));
    }

    #[test]
    fn test_validate_single_themes_feature() {
        let provides = BTreeSet::from([ExtensionProvides::Themes]);
        assert_eq!(validate_extension_features(&provides), Ok(()));
    }

    #[test]
    fn test_validate_themes_with_other_features() {
        let provides = BTreeSet::from([ExtensionProvides::Themes, ExtensionProvides::Languages]);
        assert_eq!(
            validate_extension_features(&provides),
            Err(ExtensionFeatureError::ThemesMixedWithOtherFeatures),
        );
    }

    #[test]
    fn test_validate_single_icon_themes_feature() {
        let provides = BTreeSet::from([ExtensionProvides::IconThemes]);
        assert_eq!(validate_extension_features(&provides), Ok(()));
    }

    #[test]
    fn test_validate_icon_themes_with_other_features() {
        let provides = BTreeSet::from([ExtensionProvides::IconThemes, ExtensionProvides::Grammars]);
        assert_eq!(
            validate_extension_features(&provides),
            Err(ExtensionFeatureError::IconThemesMixedWithOtherFeatures),
        );
    }

    #[test]
    fn test_validate_slash_commands_only() {
        let provides = BTreeSet::from([ExtensionProvides::SlashCommands]);
        assert_eq!(
            validate_extension_features(&provides),
            Err(ExtensionFeatureError::SlashCommandsDeprecated { sole_feature: true }),
        );
    }

    #[test]
    fn test_validate_slash_commands_with_other_features() {
        let provides = BTreeSet::from([
            ExtensionProvides::SlashCommands,
            ExtensionProvides::Languages,
        ]);
        assert_eq!(
            validate_extension_features(&provides),
            Err(ExtensionFeatureError::SlashCommandsDeprecated {
                sole_feature: false
            }),
        );
    }

    #[test]
    fn test_validate_multiple_non_theme_features() {
        let provides = BTreeSet::from([
            ExtensionProvides::Languages,
            ExtensionProvides::Grammars,
            ExtensionProvides::LanguageServers,
        ]);
        assert_eq!(validate_extension_features(&provides), Ok(()));
    }
}
