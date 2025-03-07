use anyhow::{Context as _, Result};
use editor::Editor;
use fs::Fs;
use migrator::{migrate_keymap, migrate_settings};
use settings::{KeymapFile, SettingsStore};
use util::ResultExt;

use std::sync::Arc;

use gpui::{Entity, EventEmitter, Global};
use ui::prelude::*;
use workspace::item::ItemHandle;
use workspace::{ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MigrationType {
    Keymap,
    Settings,
}

pub struct MigrationBanner {
    migration_type: Option<MigrationType>,
}

pub enum MigrationEvent {
    ContentChanged {
        migration_type: MigrationType,
        migrated: bool,
    },
}

pub struct MigrationNotification;

impl EventEmitter<MigrationEvent> for MigrationNotification {}

impl MigrationNotification {
    pub fn try_global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalMigrationNotification>()
            .map(|notifier| notifier.0.clone())
    }

    pub fn set_global(notifier: Entity<Self>, cx: &mut App) {
        cx.set_global(GlobalMigrationNotification(notifier));
    }
}

struct GlobalMigrationNotification(Entity<MigrationNotification>);

impl Global for GlobalMigrationNotification {}

impl MigrationBanner {
    pub fn new(_: &Workspace, cx: &mut Context<'_, Self>) -> Self {
        if let Some(notifier) = MigrationNotification::try_global(cx) {
            cx.subscribe(
                &notifier,
                move |migrator_banner, _, event: &MigrationEvent, cx| {
                    migrator_banner.handle_notification(event, cx);
                },
            )
            .detach();
        }
        Self {
            migration_type: None,
        }
    }

    fn backup_file_name(&self) -> String {
        match self.migration_type {
            Some(MigrationType::Keymap) => paths::keymap_backup_file()
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
            Some(MigrationType::Settings) => paths::settings_backup_file()
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
            None => String::new(),
        }
    }

    fn handle_notification(&mut self, event: &MigrationEvent, cx: &mut Context<'_, Self>) {
        match event {
            MigrationEvent::ContentChanged {
                migration_type,
                migrated,
            } => {
                if self.migration_type == Some(*migration_type) {
                    let location = if *migrated {
                        ToolbarItemLocation::Secondary
                    } else {
                        ToolbarItemLocation::Hidden
                    };
                    cx.emit(ToolbarItemEvent::ChangeLocation(location));
                    cx.notify();
                }
            }
        }
    }
}

impl EventEmitter<ToolbarItemEvent> for MigrationBanner {}

impl ToolbarItemView for MigrationBanner {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        cx.notify();
        let Some(target) = active_pane_item
            .and_then(|item| item.act_as::<Editor>(cx))
            .and_then(|editor| editor.update(cx, |editor, cx| editor.target_file_abs_path(cx)))
        else {
            return ToolbarItemLocation::Hidden;
        };

        if &target == paths::keymap_file() {
            self.migration_type = Some(MigrationType::Keymap);
            let fs = <dyn Fs>::global(cx);
            let should_migrate = should_migrate_keymap(fs);
            cx.spawn_in(window, |this, mut cx| async move {
                if let Ok(true) = should_migrate.await {
                    this.update(&mut cx, |_, cx| {
                        cx.emit(ToolbarItemEvent::ChangeLocation(
                            ToolbarItemLocation::Secondary,
                        ));
                        cx.notify();
                    })
                    .log_err();
                }
            })
            .detach();
        } else if &target == paths::settings_file() {
            self.migration_type = Some(MigrationType::Settings);
            let fs = <dyn Fs>::global(cx);
            let should_migrate = should_migrate_settings(fs);
            cx.spawn_in(window, |this, mut cx| async move {
                if let Ok(true) = should_migrate.await {
                    this.update(&mut cx, |_, cx| {
                        cx.emit(ToolbarItemEvent::ChangeLocation(
                            ToolbarItemLocation::Secondary,
                        ));
                        cx.notify();
                    })
                    .log_err();
                }
            })
            .detach();
        }

        return ToolbarItemLocation::Hidden;
    }
}

impl Render for MigrationBanner {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let migration_type = self.migration_type;
        let file_type = match migration_type {
            Some(MigrationType::Keymap) => "keymap",
            Some(MigrationType::Settings) => "settings",
            None => "",
        };
        let backup_file_name = self.backup_file_name();

        h_flex()
            .py_1()
            .pl_2()
            .pr_1()
            .flex_wrap()
            .justify_between()
            .bg(cx.theme().status().info_background.opacity(0.6))
            .border_1()
            .border_color(cx.theme().colors().border_variant)
            .rounded_sm()
            .overflow_hidden()
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Icon::new(IconName::Warning)
                            .size(IconSize::XSmall)
                            .color(Color::Warning),
                    )
                    .child(
                        h_flex()
                            .gap_0p5()
                            .child(
                                Label::new(format!(
                                    "Your {} file uses deprecated settings which can be \
                                    automatically updated. A backup will be saved to",
                                    file_type
                                ))
                                .color(Color::Default),
                            )
                            .child(
                                div()
                                    .px_1()
                                    .bg(cx.theme().colors().background)
                                    .rounded_xs()
                                    .child(
                                        Label::new(backup_file_name)
                                            .buffer_font(cx)
                                            .size(LabelSize::Small),
                                    ),
                            ),
                    ),
            )
            .child(
                Button::new("backup-and-migrate", "Backup and Update").on_click(move |_, _, cx| {
                    let fs = <dyn Fs>::global(cx);
                    match migration_type {
                        Some(MigrationType::Keymap) => {
                            cx.spawn(
                                move |_| async move { write_keymap_migration(&fs).await.ok() },
                            )
                            .detach();
                        }
                        Some(MigrationType::Settings) => {
                            cx.spawn(
                                move |_| async move { write_settings_migration(&fs).await.ok() },
                            )
                            .detach();
                        }
                        None => unreachable!(),
                    }
                }),
            )
            .into_any_element()
    }
}

async fn should_migrate_keymap(fs: Arc<dyn Fs>) -> Result<bool> {
    let old_text = KeymapFile::load_keymap_file(&fs).await?;
    if let Ok(Some(_)) = migrate_keymap(&old_text) {
        return Ok(true);
    };
    Ok(false)
}

async fn should_migrate_settings(fs: Arc<dyn Fs>) -> Result<bool> {
    let old_text = SettingsStore::load_settings(&fs).await?;
    if let Ok(Some(_)) = migrate_settings(&old_text) {
        return Ok(true);
    };
    Ok(false)
}

async fn write_keymap_migration(fs: &Arc<dyn Fs>) -> Result<()> {
    let old_text = KeymapFile::load_keymap_file(fs).await?;
    let Ok(Some(new_text)) = migrate_keymap(&old_text) else {
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

async fn write_settings_migration(fs: &Arc<dyn Fs>) -> Result<()> {
    let old_text = SettingsStore::load_settings(fs).await?;
    let Ok(Some(new_text)) = migrate_settings(&old_text) else {
        return Ok(());
    };
    let settings_path = paths::settings_file().as_path();
    if fs.is_file(settings_path).await {
        fs.atomic_write(paths::settings_backup_file().to_path_buf(), old_text)
            .await
            .with_context(|| "Failed to create settings backup in home directory".to_string())?;
        let resolved_path = fs
            .canonicalize(settings_path)
            .await
            .with_context(|| format!("Failed to canonicalize settings path {:?}", settings_path))?;
        fs.atomic_write(resolved_path.clone(), new_text)
            .await
            .with_context(|| format!("Failed to write settings to file {:?}", resolved_path))?;
    } else {
        fs.atomic_write(settings_path.to_path_buf(), new_text)
            .await
            .with_context(|| format!("Failed to write settings to file {:?}", settings_path))?;
    }
    Ok(())
}
