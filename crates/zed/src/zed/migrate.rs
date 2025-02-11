use anyhow::{Context as _, Result};
use editor::Editor;
use fs::Fs;
use markdown_preview::markdown_elements::ParsedMarkdown;
use markdown_preview::markdown_renderer::render_parsed_markdown;
use migrator::{migrate_keymap, migrate_settings};
use settings::{KeymapFile, SettingsStore};
use util::markdown::MarkdownString;
use util::ResultExt;

use std::sync::Arc;

use gpui::{EventEmitter, Task, WeakEntity};
use ui::prelude::*;
use workspace::item::ItemHandle;
use workspace::{ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace};

#[derive(Debug, Copy, Clone)]
enum MigrationType {
    Keymap,
    Settings,
}

pub struct MigratorBanner {
    migration_type: Option<MigrationType>,
    message: ParsedMarkdown,
    should_migrate_task: Option<Task<()>>,
    workspace: WeakEntity<Workspace>,
}

impl MigratorBanner {
    pub fn new(workspace: &Workspace) -> Self {
        Self {
            migration_type: None,
            message: ParsedMarkdown { children: vec![] },
            should_migrate_task: None,
            workspace: workspace.weak_handle(),
        }
    }
}

impl MigratorBanner {}

impl EventEmitter<ToolbarItemEvent> for MigratorBanner {}

impl ToolbarItemView for MigratorBanner {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        cx.notify();
        self.migration_type = None;
        self.message = ParsedMarkdown { children: vec![] };
        self.should_migrate_task.take();
        let Some(target) = active_pane_item
            .and_then(|item| item.act_as::<Editor>(cx))
            .and_then(|editor| editor.update(cx, |editor, cx| editor.target_file_abs_path(cx)))
        else {
            return ToolbarItemLocation::Hidden;
        };
        if &target == paths::keymap_file() {
            let fs = <dyn Fs>::global(cx);
            let should_migrate = should_migrate_keymap(fs);
            self.should_migrate_task =
                Some(cx.spawn_in(window, |migrator_banner, mut cx| async move {
                    if let Ok(true) = should_migrate.await {
                        let message = MarkdownString(format!(
                            "Your keymap require migration to support this version of Zed. A backup will be saved to {}.",
                            MarkdownString::inline_code(&paths::keymap_backup_file().to_string_lossy())
                        ));
                        let parsed_markdown = cx.background_executor().spawn(async move {
                            let file_location_directory = None;
                            let language_registry = None;
                            markdown_preview::markdown_parser::parse_markdown(
                                &message.0,
                                file_location_directory,
                                language_registry,
                            )
                            .await
                        }).await;
                        migrator_banner
                            .update(&mut cx, |this, cx| {
                                this.migration_type = Some(MigrationType::Keymap);
                                this.message = parsed_markdown;
                                cx.emit(ToolbarItemEvent::ChangeLocation(
                                    ToolbarItemLocation::Secondary,
                                ));
                                cx.notify();
                            })
                            .log_err();
                    }
                }));
        } else if &target == paths::settings_file() {
            let fs = <dyn Fs>::global(cx);
            let should_migrate = should_migrate_settings(fs);
            self.should_migrate_task =
                Some(cx.spawn_in(window, |migrator_banner, mut cx| async move {
                    if let Ok(true) = should_migrate.await {
                        let message = MarkdownString(format!(
                            "Your settings require migration to support this version of Zed. A backup will be saved to {}.",
                            MarkdownString::inline_code(&paths::keymap_backup_file().to_string_lossy())
                        ));
                        let parsed_markdown = cx.background_executor().spawn(async move {
                            let file_location_directory = None;
                            let language_registry = None;
                            markdown_preview::markdown_parser::parse_markdown(
                                &message.0,
                                file_location_directory,
                                language_registry,
                            )
                            .await
                        }).await;
                        migrator_banner
                            .update(&mut cx, |this, cx| {
                                this.migration_type = Some(MigrationType::Settings);
                                this.message = parsed_markdown;
                                cx.emit(ToolbarItemEvent::ChangeLocation(
                                    ToolbarItemLocation::Secondary,
                                ));
                                cx.notify();
                            })
                            .log_err();
                    }
                }));
        }
        return ToolbarItemLocation::Hidden;
    }
}

impl Render for MigratorBanner {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let migration_type = self.migration_type;
        h_flex()
            .py_1()
            .px_2()
            .justify_between()
            .bg(cx.theme().status().info_background)
            .rounded_md()
            .gap_2()
            .overflow_hidden()
            .child(
                render_parsed_markdown(&self.message, Some(self.workspace.clone()), window, cx)
                    .text_ellipsis(),
            )
            .child(
                Button::new(
                    SharedString::from("backup-and-migrate"),
                    "Backup and Migrate",
                )
                .style(ButtonStyle::Filled)
                .on_click(move |_, _, cx| {
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

pub fn migrate_keymap_in_memory(old_text: String) -> String {
    if let Some(new_text) = migrate_keymap(&old_text) {
        return new_text;
    };
    old_text
}

pub fn migrate_settings_in_memory(old_text: String) -> String {
    if let Some(new_text) = migrate_settings(&old_text) {
        return new_text;
    };
    old_text
}

async fn should_migrate_keymap(fs: Arc<dyn Fs>) -> Result<bool> {
    let old_text = KeymapFile::load_keymap_file(&fs).await?;
    Ok(migrate_keymap(&old_text).is_some())
}

async fn should_migrate_settings(fs: Arc<dyn Fs>) -> Result<bool> {
    let old_text = SettingsStore::load_settings(&fs).await?;
    Ok(migrate_settings(&old_text).is_some())
}

async fn write_keymap_migration(fs: &Arc<dyn Fs>) -> Result<()> {
    let old_text = KeymapFile::load_keymap_file(fs).await?;
    let Some(new_text) = migrate_keymap(&old_text) else {
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
    let Some(new_text) = migrate_settings(&old_text) else {
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
