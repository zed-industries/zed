use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context as _, Result};
use fs::{CopyOptions, Fs, copy_recursive};
use futures::{StreamExt as _, TryStreamExt as _};

use crate::ExtensionManifest;

pub async fn copy_extension_resources(
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
        futures::stream::iter(manifest.grammars.keys().map(|grammar_name| {
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
        .then(std::convert::identity)
        .try_for_each(|()| futures::future::ready(Ok(())))
        .await?;
    }

    if !manifest.themes.is_empty() {
        let output_themes_dir = output_dir.join("themes");
        fs.create_dir(&output_themes_dir).await?;
        futures::stream::iter(manifest.themes.iter().map(|theme_path| {
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
        .then(std::convert::identity)
        .try_for_each(|()| futures::future::ready(Ok(())))
        .await?;
    }

    if !manifest.icon_themes.is_empty() {
        let output_icon_themes_dir = output_dir.join("icon_themes");
        fs.create_dir(&output_icon_themes_dir).await?;
        futures::stream::iter(manifest.icon_themes.iter().map(|icon_theme_path| {
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
        .then(std::convert::identity)
        .try_for_each(|()| futures::future::ready(Ok(())))
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
        futures::stream::iter(manifest.languages.iter().map(|language_path| {
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
        .then(std::convert::identity)
        .try_for_each(|()| futures::future::ready(Ok(())))
        .await?;
    }

    if !manifest.debug_adapters.is_empty() {
        futures::stream::iter(
            manifest
                .debug_adapters
                .iter()
                .map(|(debug_adapter, entry)| {
                    let fs = fs.clone();
                    let debug_adapter = debug_adapter.clone();
                    async move {
                        let schema_path =
                            crate::build_debug_adapter_schema_path(&debug_adapter, &entry)?;
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
                }),
        )
        .then(std::convert::identity)
        .try_for_each(|()| futures::future::ready(Ok(())))
        .await?;
    }

    if let Some(snippets) = manifest.snippets.as_ref() {
        futures::stream::iter(snippets.paths().map(|snippets_path| {
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
        .then(std::convert::identity)
        .try_for_each(|()| futures::future::ready(Ok(())))
        .await?;
    }

    Ok(())
}
