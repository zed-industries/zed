use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context as _, Result};
use fs::{CopyOptions, Fs, copy_recursive};

use crate::ExtensionManifest;

async fn create_dir(fs: &dyn Fs, path: &Path) -> Result<()> {
    fs.create_dir(path)
        .await
        .with_context(|| format!("creating {path:?}"))
}

pub async fn copy_extension_resources(
    manifest: &ExtensionManifest,
    extension_path: &Path,
    output_dir: &Path,
    fs: Arc<dyn Fs>,
) -> Result<()> {
    create_dir(fs.as_ref(), output_dir).await?;

    let manifest_toml = toml::to_string(manifest).context("serializing extension manifest")?;
    let manifest_path = output_dir.join("extension.toml");
    fs.write(&manifest_path, manifest_toml.as_bytes())
        .await
        .with_context(|| format!("writing {manifest_path:?}"))?;

    if manifest.lib.kind.is_some() {
        let source_path = extension_path.join("extension.wasm");
        let output_path = output_dir.join("extension.wasm");
        fs.copy_file(
            &source_path,
            &output_path,
            CopyOptions {
                overwrite: true,
                ignore_if_exists: false,
            },
        )
        .await
        .with_context(|| format!("copying {source_path:?} to {output_path:?}"))?;
    }

    if !manifest.grammars.is_empty() {
        let source_grammars_dir = extension_path.join("grammars");
        let output_grammars_dir = output_dir.join("grammars");
        create_dir(fs.as_ref(), &output_grammars_dir).await?;
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
                .with_context(|| format!("copying grammar {grammar_filename:?}"))
            }
        }))
        .await?;
    }

    if !manifest.themes.is_empty() {
        let output_themes_dir = output_dir.join("themes");
        create_dir(fs.as_ref(), &output_themes_dir).await?;
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
                .with_context(|| format!("copying theme {theme_path:?}"))
            }
        }))
        .await?;
    }

    if !manifest.icon_themes.is_empty() {
        let output_icon_themes_dir = output_dir.join("icon_themes");
        create_dir(fs.as_ref(), &output_icon_themes_dir).await?;
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
                .with_context(|| format!("copying icon theme {icon_theme_path:?}"))
            }
        }))
        .await?;

        let source_icons_dir = extension_path.join("icons");
        let output_icons_dir = output_dir.join("icons");
        create_dir(fs.as_ref(), &output_icons_dir).await?;
        copy_recursive(
            fs.as_ref(),
            &source_icons_dir,
            &output_icons_dir,
            CopyOptions {
                overwrite: true,
                ignore_if_exists: false,
            },
        )
        .await
        .with_context(|| format!("copying {source_icons_dir:?} to {output_icons_dir:?}"))?;
    }

    if !manifest.languages.is_empty() {
        let output_languages_dir = output_dir.join("languages");
        create_dir(fs.as_ref(), &output_languages_dir).await?;
        futures::future::try_join_all(manifest.languages.iter().map(|language_path| {
            let fs = fs.clone();
            let output_languages_dir = output_languages_dir.clone();
            async move {
                let language_path = language_path.as_std_path();
                let source_path = extension_path.join(language_path);
                let output_path = output_languages_dir
                    .join(language_path.file_name().context("invalid language path")?);
                copy_recursive(
                    fs.as_ref(),
                    &source_path,
                    &output_path,
                    CopyOptions {
                        overwrite: true,
                        ignore_if_exists: false,
                    },
                )
                .await
                .with_context(|| format!("copying {source_path:?} to {output_path:?}"))
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
                        crate::build_debug_adapter_schema_path(&debug_adapter, &entry)?;
                    let parent = schema_path.parent().with_context(|| {
                        format!("invalid empty schema path for {debug_adapter}")
                    })?;
                    let schema_path = schema_path.as_std_path();
                    let source_path = extension_path.join(schema_path);
                    let output_path = output_dir.join(schema_path);
                    create_dir(fs.as_ref(), &output_dir.join(parent)).await?;
                    copy_recursive(
                        fs.as_ref(),
                        &source_path,
                        &output_path,
                        CopyOptions {
                            overwrite: true,
                            ignore_if_exists: false,
                        },
                    )
                    .await
                    .with_context(|| format!("copying {source_path:?} to {output_path:?}"))
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
                    create_dir(fs.as_ref(), &output_dir.join(parent)).await?;
                }
                let source_path = extension_path.join(snippets_path);
                let output_path = output_dir.join(snippets_path);
                copy_recursive(
                    fs.as_ref(),
                    &source_path,
                    &output_path,
                    CopyOptions {
                        overwrite: true,
                        ignore_if_exists: false,
                    },
                )
                .await
                .with_context(|| format!("copying {source_path:?} to {output_path:?}"))
            }
        }))
        .await?;
    }

    Ok(())
}
