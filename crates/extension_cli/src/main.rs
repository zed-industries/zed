use std::collections::BTreeSet;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr as _;
use std::sync::Arc;

use ::fs::{Fs, RealFs, RemoveOptions};
use anyhow::{Context as _, Result, anyhow, bail};
use clap::Parser;
use cloud_api_types::ExtensionProvides;
use extension::build_debug_adapter_schema_path;
use extension::extension_builder::CompilationConcurrency;
use extension::extension_builder::{CompileExtensionOptions, ExtensionBuilder};
use extension::{ExtensionManifest, ExtensionSnippets, copy_extension_resources};
use http_client::Url;
use language::LanguageConfig;
use language::QueryFile;
use reqwest_client::ReqwestClient;
use settings_content::SemanticTokenRules;
use snippet_provider::file_to_snippets;
use snippet_provider::format::VsSnippetsFile;
use task::TaskTemplates;
use tokio::process::Command;
use tree_sitter::{Language, Query, WasmStore};

struct TestingContext {
    known_resources: BTreeSet<PathBuf>,
}

impl TestingContext {
    fn new(manifest: &ExtensionManifest, extension_path: &Path) -> Self {
        let known_resources = manifest
            .snippets
            .as_ref()
            .map(ExtensionSnippets::paths)
            .into_iter()
            .flatten()
            .map(|relative_path| extension_path.join(relative_path))
            .collect();
        Self { known_resources }
    }

    fn is_known_resource(&self, path: &Path) -> bool {
        self.known_resources.contains(path)
    }
}

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
    let fs = RealFs::new(None, gpui_platform::background_executor());
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

    validate_extension_manifest(&manifest)?;
    let extension_provides = manifest.provides();
    validate_extension_features(&extension_provides)?;

    let cx = TestingContext::new(&manifest, &extension_path);
    let grammars = test_grammars(&manifest, &extension_path, &mut wasm_store)?;
    test_languages(&manifest, &extension_path, &grammars, &cx)?;
    log::info!("validating themes");
    test_themes(&manifest, &extension_path, fs.clone()).await?;
    log::info!("validating snippets");
    test_snippets(&manifest, &extension_path, fs.clone()).await?;
    log::info!("validating debug adapter schemas");
    test_debug_adapter_schemas(&manifest, &extension_path, fs.clone()).await?;

    log::info!("preparing archive directory");
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
    log::info!("copying extension resources");
    copy_extension_resources(&manifest, &extension_path, &archive_dir, fs.clone())
        .await
        .context("failed to copy extension resources")?;

    log::info!("creating archive.tar.gz");
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

    log::info!("extension packaged successfully");
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

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
enum ExtensionManifestValidationError {
    #[error("extension manifest must specify a name")]
    MissingName,
    #[error("extension manifest must specify a description")]
    MissingDescription,
    #[error("extension manifest description must be more expressive than the name")]
    DescriptionNotLongerThanName,
    #[error("extension manifest must specify at least one author")]
    MissingAuthors,
    #[error("extension manifest must specify a repository")]
    MissingRepository,
    #[error("extension manifest repository is not a valid URL: {0}")]
    InvalidRepository(String),
    #[error(
        "extension manifest must not provide language model providers, \
        as these are currently unsupported"
    )]
    LanguageModelProvidersUnsupported,
    #[error(
        "language server `{language_server}` lists `{language}` in `opt_in_languages`, \
        but `{language}` is not listed in its `languages`"
    )]
    OptInLanguageNotInLanguages {
        language_server: String,
        language: String,
    },
}

fn validate_extension_manifest(
    manifest: &ExtensionManifest,
) -> Result<(), ExtensionManifestValidationError> {
    if manifest.name.trim().is_empty() {
        return Err(ExtensionManifestValidationError::MissingName);
    }

    let description_is_empty = manifest
        .description
        .as_ref()
        .is_none_or(|description| description.trim().is_empty());
    if description_is_empty {
        return Err(ExtensionManifestValidationError::MissingDescription);
    }

    let description_is_longer_than_name =
        manifest.description.as_ref().is_some_and(|description| {
            description.trim().chars().count() > manifest.name.trim().chars().count()
        });
    if !description_is_longer_than_name {
        return Err(ExtensionManifestValidationError::DescriptionNotLongerThanName);
    }

    if manifest
        .authors
        .iter()
        .all(|author| author.trim().is_empty())
    {
        return Err(ExtensionManifestValidationError::MissingAuthors);
    }

    let repository = manifest
        .repository
        .as_ref()
        .map(|repository| repository.trim())
        .filter(|repository| !repository.is_empty())
        .ok_or(ExtensionManifestValidationError::MissingRepository)?;

    let repository_url = repository
        .parse::<Url>()
        .map_err(|_| ExtensionManifestValidationError::InvalidRepository(repository.to_string()))?;

    if repository_url.host_str().is_none() {
        return Err(ExtensionManifestValidationError::InvalidRepository(
            repository.to_string(),
        ));
    };

    if !manifest.language_model_providers.is_empty() {
        return Err(ExtensionManifestValidationError::LanguageModelProvidersUnsupported);
    }

    for (language_server, entry) in &manifest.language_servers {
        let languages = entry.languages().into_iter().collect::<BTreeSet<_>>();
        if let Some(language) = entry.opt_in_languages().difference(&languages).next() {
            return Err(
                ExtensionManifestValidationError::OptInLanguageNotInLanguages {
                    language_server: language_server.to_string(),
                    language: language.to_string(),
                },
            );
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
    context: &TestingContext,
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
                _ => {
                    if let Ok(_query) = file_name.parse::<QueryFile>() {
                        let grammar = grammar.with_context(|| {
                            format! {
                                "language {} provides query {} but no grammar",
                                config.name,
                                file_path.display()
                            }
                        })?;

                        let query_source = fs::read_to_string(&file_path)?;
                        let _query = Query::new(grammar, &query_source)?;
                    } else if file_name.ends_with(".scm") {
                        bail!("query {file_name} is not supported by Zed and should be removed")
                    } else if !context.is_known_resource(&file_path) {
                        bail!(
                            "'{file_name}' is not a supported file in a language directory and should be removed"
                        )
                    }
                }
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
    use std::collections::BTreeMap;

    use cloud_api_types::ExtensionProvides;
    use extension::{LanguageModelProviderManifestEntry, SchemaVersion};

    use super::*;

    fn valid_manifest() -> ExtensionManifest {
        ExtensionManifest {
            id: "test".into(),
            name: "Test Extension".to_string(),
            version: "1.0.0".into(),
            schema_version: SchemaVersion::ZERO,
            description: Some("A test extension".to_string()),
            repository: Some("https://github.com/zed-industries/zed".to_string()),
            authors: vec!["Zed".to_string()],
            lib: Default::default(),
            themes: Vec::new(),
            icon_themes: Vec::new(),
            languages: Vec::new(),
            grammars: BTreeMap::default(),
            language_servers: BTreeMap::default(),
            context_servers: BTreeMap::default(),
            slash_commands: BTreeMap::default(),
            snippets: None,
            capabilities: Vec::new(),
            debug_adapters: BTreeMap::default(),
            debug_locators: BTreeMap::default(),
            language_model_providers: BTreeMap::default(),
        }
    }

    #[test]
    fn test_validate_manifest_valid() {
        assert!(validate_extension_manifest(&valid_manifest()).is_ok());
    }

    #[test]
    fn test_validate_manifest_missing_name() {
        let manifest = ExtensionManifest {
            name: "   ".to_string(),
            ..valid_manifest()
        };
        assert_eq!(
            validate_extension_manifest(&manifest),
            Err(ExtensionManifestValidationError::MissingName),
        );
    }

    #[test]
    fn test_validate_manifest_missing_description() {
        let manifest = ExtensionManifest {
            description: None,
            ..valid_manifest()
        };
        assert_eq!(
            validate_extension_manifest(&manifest),
            Err(ExtensionManifestValidationError::MissingDescription),
        );
    }

    #[test]
    fn test_validate_manifest_empty_description() {
        let manifest = ExtensionManifest {
            description: Some("  ".to_string()),
            ..valid_manifest()
        };
        assert_eq!(
            validate_extension_manifest(&manifest),
            Err(ExtensionManifestValidationError::MissingDescription),
        );
    }

    #[test]
    fn test_validate_manifest_description_equal_to_name() {
        let manifest = ExtensionManifest {
            description: Some("Test Extension".to_string()),
            ..valid_manifest()
        };
        assert_eq!(
            validate_extension_manifest(&manifest),
            Err(ExtensionManifestValidationError::DescriptionNotLongerThanName),
        );
    }

    #[test]
    fn test_validate_manifest_description_shorter_than_name() {
        let manifest = ExtensionManifest {
            description: Some("Test".to_string()),
            ..valid_manifest()
        };
        assert_eq!(
            validate_extension_manifest(&manifest),
            Err(ExtensionManifestValidationError::DescriptionNotLongerThanName),
        );
    }

    #[test]
    fn test_validate_manifest_missing_authors() {
        let manifest = ExtensionManifest {
            authors: Vec::new(),
            ..valid_manifest()
        };
        assert_eq!(
            validate_extension_manifest(&manifest),
            Err(ExtensionManifestValidationError::MissingAuthors),
        );
    }

    #[test]
    fn test_validate_manifest_blank_authors() {
        let manifest = ExtensionManifest {
            authors: vec!["   ".to_string()],
            ..valid_manifest()
        };
        assert_eq!(
            validate_extension_manifest(&manifest),
            Err(ExtensionManifestValidationError::MissingAuthors),
        );
    }

    #[test]
    fn test_validate_manifest_missing_repository() {
        let manifest = ExtensionManifest {
            repository: None,
            ..valid_manifest()
        };
        assert_eq!(
            validate_extension_manifest(&manifest),
            Err(ExtensionManifestValidationError::MissingRepository),
        );
    }

    #[test]
    fn test_validate_manifest_invalid_repository() {
        let manifest = ExtensionManifest {
            repository: Some("not-a-valid-url".to_string()),
            ..valid_manifest()
        };
        assert_eq!(
            validate_extension_manifest(&manifest),
            Err(ExtensionManifestValidationError::InvalidRepository(
                "not-a-valid-url".to_string()
            )),
        );
    }

    #[test]
    fn test_validate_manifest_repository_without_host() {
        let manifest = ExtensionManifest {
            repository: Some("file:///some/local/path".to_string()),
            ..valid_manifest()
        };
        assert_eq!(
            validate_extension_manifest(&manifest),
            Err(ExtensionManifestValidationError::InvalidRepository(
                "file:///some/local/path".to_string()
            )),
        );
    }

    #[test]
    fn test_validate_manifest_language_model_providers_unsupported() {
        let mut language_model_providers = BTreeMap::new();
        language_model_providers.insert(
            "provider".into(),
            LanguageModelProviderManifestEntry {
                name: "Provider".to_string(),
                icon: None,
            },
        );
        let manifest = ExtensionManifest {
            language_model_providers,
            ..valid_manifest()
        };
        assert_eq!(
            validate_extension_manifest(&manifest),
            Err(ExtensionManifestValidationError::LanguageModelProvidersUnsupported),
        );
    }

    #[test]
    fn test_validate_manifest_opt_in_languages() {
        let manifest_with_language_server = |language_server: &str| -> ExtensionManifest {
            let language_server = toml::from_str(language_server).unwrap();
            ExtensionManifest {
                language_servers: BTreeMap::from_iter([("my-server".into(), language_server)]),
                ..valid_manifest()
            }
        };

        let manifest = manifest_with_language_server(
            r#"
            languages = ["Julia", "Markdown"]
            opt_in_languages = ["Julia"]
            "#,
        );
        assert_eq!(validate_extension_manifest(&manifest), Ok(()));

        let manifest = manifest_with_language_server(
            r#"
            languages = ["Markdown"]
            opt_in_languages = ["Julia"]
            "#,
        );
        assert_eq!(
            validate_extension_manifest(&manifest),
            Err(
                ExtensionManifestValidationError::OptInLanguageNotInLanguages {
                    language_server: "my-server".to_string(),
                    language: "Julia".to_string(),
                }
            ),
        );
    }

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
