use std::sync::Arc;

use anyhow::Context;
use fs::Fs;
use settings::{KeymapFile, SettingsStore};

pub fn should_migrate_settings(settings: &serde_json::Value) -> bool {
    let Ok(old_text) = serde_json::to_string(settings) else {
        return false;
    };
    migrator::migrate_settings(&old_text)
        .ok()
        .flatten()
        .is_some()
}

pub fn migrate_settings(fs: Arc<dyn Fs>, cx: &mut gpui::App) {
    cx.background_executor()
        .spawn(async move {
            let old_text = SettingsStore::load_settings(&fs).await?;
            let Some(new_text) = migrator::migrate_settings(&old_text)? else {
                return anyhow::Ok(());
            };
            let settings_path = paths::settings_file().as_path();
            if fs.is_file(settings_path).await {
                fs.atomic_write(paths::settings_backup_file().to_path_buf(), old_text)
                    .await
                    .with_context(|| {
                        "Failed to create settings backup in home directory".to_string()
                    })?;
                let resolved_path = fs.canonicalize(settings_path).await.with_context(|| {
                    format!("Failed to canonicalize settings path {:?}", settings_path)
                })?;
                fs.atomic_write(resolved_path.clone(), new_text)
                    .await
                    .with_context(|| {
                        format!("Failed to write settings to file {:?}", resolved_path)
                    })?;
            } else {
                fs.atomic_write(settings_path.to_path_buf(), new_text)
                    .await
                    .with_context(|| {
                        format!("Failed to write settings to file {:?}", settings_path)
                    })?;
            }
            Ok(())
        })
        .detach_and_log_err(cx);
}

pub fn should_migrate_keymap(keymap_file: KeymapFile) -> bool {
    let Ok(old_text) = serde_json::to_string(&keymap_file) else {
        return false;
    };
    migrator::migrate_keymap(&old_text).ok().flatten().is_some()
}

pub async fn migrate_keymap(fs: Arc<dyn Fs>) -> anyhow::Result<()> {
    let old_text = KeymapFile::load_keymap_file(&fs).await?;
    let Some(new_text) = migrator::migrate_keymap(&old_text)? else {
        return Ok(());
    };
    let keymap_path = paths::keymap_file().as_path();
    if fs.is_file(keymap_path).await {
        fs.atomic_write(paths::keymap_backup_file().to_path_buf(), old_text)
            .await
            .with_context(|| "Failed to create settings backup in home directory".to_string())?;
        let resolved_path = fs
            .canonicalize(keymap_path)
            .await
            .with_context(|| format!("Failed to canonicalize keymap path {:?}", keymap_path))?;
        fs.atomic_write(resolved_path.clone(), new_text)
            .await
            .with_context(|| format!("Failed to write keymap to file {:?}", resolved_path))?;
    } else {
        fs.atomic_write(keymap_path.to_path_buf(), new_text)
            .await
            .with_context(|| format!("Failed to write keymap to file {:?}", keymap_path))?;
    }

    Ok(())
}
