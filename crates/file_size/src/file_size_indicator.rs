use editor::Editor;
use gpui::{Entity, Subscription, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsKey, SettingsSources, SettingsUi};
use std::time::Duration;
use text::TextSummary;
use ui::{
    Button, Context, FluentBuilder, IntoElement, LabelSize, ParentElement, Render, Window, div,
};
use workspace::{StatusItemView, Workspace, item::ItemHandle};

pub struct FileSizeIndicator {
    file_size: Option<usize>,
    update_file_size: Task<()>,
    _observe_active_editor: Option<Subscription>,
}

impl FileSizeIndicator {
    pub fn new(_workspace: &Workspace) -> Self {
        Self {
            file_size: None,
            update_file_size: Task::ready(()),
            _observe_active_editor: None,
        }
    }

    fn update_file_size(
        &mut self,
        editor: Entity<Editor>,
        debounce: Option<Duration>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let editor = editor.downgrade();
        self.update_file_size = cx.spawn_in(window, async move |file_size_indicator, cx| {
            let is_singleton = editor
                .update(cx, |editor, cx| editor.buffer().read(cx).is_singleton())
                .ok()
                .unwrap_or(true);

            if !is_singleton && let Some(debounce) = debounce {
                cx.background_executor().timer(debounce).await;
            }

            editor
                .update(cx, |editor, cx| {
                    file_size_indicator.update(cx, |file_size_indicator, cx| {
                        let buffer = editor.buffer().read(cx);
                        let snapshot = buffer.snapshot(cx);
                        let full_range = 0..snapshot.len();
                        file_size_indicator.file_size = Some(
                            snapshot
                                .text_summary_for_range::<TextSummary, _>(full_range)
                                .len,
                        );
                        cx.notify();
                    })
                })
                .ok()
                .transpose()
                .ok()
                .flatten();
        });
    }

    fn format_file_size(size: usize, format: FileSizeFormat) -> String {
        match format {
            FileSizeFormat::Decimal => {
                const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
                const BASE: f64 = 1000.0;

                if size == 0 {
                    return "0 B".to_string();
                }

                let unit_index = ((size as f64).log(BASE).floor() as usize).min(UNITS.len() - 1);
                let scaled_size = size as f64 / BASE.powi(unit_index as i32);

                if unit_index == 0 {
                    format!("{} {}", size, UNITS[unit_index])
                } else {
                    format!("{:.1} {}", scaled_size, UNITS[unit_index])
                }
            }
            FileSizeFormat::IEC => {
                const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB"];
                const BASE: f64 = 1024.0;

                if size == 0 {
                    return "0 B".to_string();
                }

                let unit_index = ((size as f64).log(BASE).floor() as usize).min(UNITS.len() - 1);
                let scaled_size = size as f64 / BASE.powi(unit_index as i32);

                if unit_index == 0 {
                    format!("{} {}", size, UNITS[unit_index])
                } else {
                    format!("{:.1} {}", scaled_size, UNITS[unit_index])
                }
            }
        }
    }
}

impl Render for FileSizeIndicator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = FileSizeSettings::get_global(cx);
        if !settings.enabled {
            return div();
        }

        div().when_some(self.file_size, |el, size| {
            let format = settings.format;
            let text = Self::format_file_size(size, format);

            el.child(Button::new("file-size-indicator", text).label_size(LabelSize::Small))
        })
    }
}

const UPDATE_DEBOUNCE: Duration = Duration::from_millis(50);

impl StatusItemView for FileSizeIndicator {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.act_as::<Editor>(cx)) {
            self._observe_active_editor = Some(cx.observe_in(
                &editor,
                window,
                |file_size_indicator, editor, window, cx| {
                    Self::update_file_size(
                        file_size_indicator,
                        editor,
                        Some(UPDATE_DEBOUNCE),
                        window,
                        cx,
                    )
                },
            ));
            self.update_file_size(editor, None, window, cx);
        } else {
            self.file_size = None;
            self._observe_active_editor = None;
        }

        cx.notify();
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, JsonSchema, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FileSizeFormat {
    /// Use decimal units (KB, MB, GB) with base 1000
    #[default]
    Decimal,
    /// Use binary units (KiB, MiB, GiB) with base 1024
    #[serde(rename = "iec")]
    IEC,
}

#[derive(Copy, Clone, Debug, JsonSchema, Deserialize, Serialize, SettingsUi, SettingsKey)]
#[serde(default)]
#[settings_key(key = "file_size")]
pub struct FileSizeSettings {
    /// Whether to show the file size indicator in the status bar
    pub enabled: bool,
    /// File size format: "decimal" (KB, MB, GB) or "iec" (KiB, MiB, GiB)
    pub format: FileSizeFormat,
}

impl Default for FileSizeSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            format: FileSizeFormat::Decimal,
        }
    }
}

impl Settings for FileSizeSettings {
    type FileContent = Self;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::App,
    ) -> anyhow::Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use editor::Editor;
    use gpui::{AppContext, TestAppContext};
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use std::{sync::Arc, time::Duration};
    use util::path;
    use workspace::{AppState, Workspace};

    fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
        cx.update(|cx| {
            let state = AppState::test(cx);
            language::init(cx);
            crate::init(cx);
            editor::init(cx);
            workspace::init_settings(cx);
            project::Project::init_settings(cx);
            state
        })
    }

    #[test]
    fn test_format_file_size_decimal() {
        // Test decimal format (base 1000)
        assert_eq!(
            FileSizeIndicator::format_file_size(0, FileSizeFormat::Decimal),
            "0 B"
        );
        assert_eq!(
            FileSizeIndicator::format_file_size(123, FileSizeFormat::Decimal),
            "123 B"
        );
        assert_eq!(
            FileSizeIndicator::format_file_size(999, FileSizeFormat::Decimal),
            "999 B"
        );
        assert_eq!(
            FileSizeIndicator::format_file_size(1000, FileSizeFormat::Decimal),
            "1.0 KB"
        );
        assert_eq!(
            FileSizeIndicator::format_file_size(1500, FileSizeFormat::Decimal),
            "1.5 KB"
        );
        assert_eq!(
            FileSizeIndicator::format_file_size(1000000, FileSizeFormat::Decimal),
            "1.0 MB"
        );
        assert_eq!(
            FileSizeIndicator::format_file_size(2500000, FileSizeFormat::Decimal),
            "2.5 MB"
        );
        assert_eq!(
            FileSizeIndicator::format_file_size(1000000000, FileSizeFormat::Decimal),
            "1.0 GB"
        );
        assert_eq!(
            FileSizeIndicator::format_file_size(3750000000, FileSizeFormat::Decimal),
            "3.8 GB"
        );
    }

    #[test]
    fn test_format_file_size_iec() {
        // Test IEC format (base 1024)
        assert_eq!(
            FileSizeIndicator::format_file_size(0, FileSizeFormat::IEC),
            "0 B"
        );
        assert_eq!(
            FileSizeIndicator::format_file_size(123, FileSizeFormat::IEC),
            "123 B"
        );
        assert_eq!(
            FileSizeIndicator::format_file_size(1023, FileSizeFormat::IEC),
            "1023 B"
        );
        assert_eq!(
            FileSizeIndicator::format_file_size(1024, FileSizeFormat::IEC),
            "1.0 KiB"
        );
        assert_eq!(
            FileSizeIndicator::format_file_size(1536, FileSizeFormat::IEC),
            "1.5 KiB"
        );
        assert_eq!(
            FileSizeIndicator::format_file_size(1048576, FileSizeFormat::IEC),
            "1.0 MiB"
        );
        assert_eq!(
            FileSizeIndicator::format_file_size(2621440, FileSizeFormat::IEC),
            "2.5 MiB"
        );
        assert_eq!(
            FileSizeIndicator::format_file_size(1073741824, FileSizeFormat::IEC),
            "1.0 GiB"
        );
        assert_eq!(
            FileSizeIndicator::format_file_size(4026531840, FileSizeFormat::IEC),
            "3.8 GiB"
        );
    }

    #[test]
    fn test_format_file_size_edge_cases() {
        // Test edge cases
        assert_eq!(
            FileSizeIndicator::format_file_size(usize::MAX, FileSizeFormat::Decimal),
            format!("{:.1} GB", usize::MAX as f64 / 1_000_000_000.0)
        );
        assert_eq!(
            FileSizeIndicator::format_file_size(usize::MAX, FileSizeFormat::IEC),
            format!("{:.1} GiB", usize::MAX as f64 / (1024.0 * 1024.0 * 1024.0))
        );
    }

    #[gpui::test]
    async fn test_file_size_settings_default(cx: &mut TestAppContext) {
        init_test(cx);

        cx.update(|cx| {
            let settings = FileSizeSettings::get_global(cx);
            // Test default behavior (should be enabled with decimal format)
            assert_eq!(settings.enabled, true);
            assert_eq!(settings.format, FileSizeFormat::Decimal);
        });
    }

    #[gpui::test]
    async fn test_file_size_indicator_creation(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "test.rs": "fn main() {\n    println!(\"Hello, world!\");\n}"
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        workspace.update_in(cx, |workspace, window, cx| {
            let file_size_indicator = cx.new(|_| FileSizeIndicator::new(workspace));
            workspace.status_bar().update(cx, |status_bar, cx| {
                status_bar.add_right_item(file_size_indicator, window, cx);
            });
        });

        // Indicator should be created successfully
        let file_size_item = workspace.update(cx, |workspace, cx| {
            workspace
                .status_bar()
                .read(cx)
                .item_of_type::<FileSizeIndicator>()
        });

        assert!(
            file_size_item.is_some(),
            "FileSizeIndicator should be added to status bar"
        );
    }

    #[gpui::test]
    async fn test_file_size_indicator_updates_on_buffer_change(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "small.rs": "// Small file",
                "large.rs": "// This is a much larger file with more content\n".repeat(100)
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        workspace.update_in(cx, |workspace, window, cx| {
            let file_size_indicator = cx.new(|_| FileSizeIndicator::new(workspace));
            workspace.status_bar().update(cx, |status_bar, cx| {
                status_bar.add_right_item(file_size_indicator, window, cx);
            });
        });

        let worktree_id = workspace.update(cx, |workspace, cx| {
            workspace.project().update(cx, |project, cx| {
                project.worktrees(cx).next().unwrap().read(cx).id()
            })
        });

        // Open small file
        let _small_editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path((worktree_id, "small.rs"), None, true, window, cx)
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        cx.executor().advance_clock(Duration::from_millis(100));

        let small_file_size = workspace.update(cx, |workspace, cx| {
            workspace
                .status_bar()
                .read(cx)
                .item_of_type::<FileSizeIndicator>()
                .unwrap()
                .read(cx)
                .file_size
        });

        // Open large file
        let _large_editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path((worktree_id, "large.rs"), None, true, window, cx)
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        cx.executor().advance_clock(Duration::from_millis(100));

        let large_file_size = workspace.update(cx, |workspace, cx| {
            workspace
                .status_bar()
                .read(cx)
                .item_of_type::<FileSizeIndicator>()
                .unwrap()
                .read(cx)
                .file_size
        });

        assert!(small_file_size.is_some(), "Small file should have a size");
        assert!(large_file_size.is_some(), "Large file should have a size");
        assert!(
            large_file_size.unwrap() > small_file_size.unwrap(),
            "Large file should be bigger than small file"
        );
    }

    #[gpui::test]
    async fn test_file_size_settings_integration(cx: &mut TestAppContext) {
        cx.update(|cx| {
            // Create fresh settings store for proper settings control
            let mut store = SettingsStore::new(cx);
            store
                .set_default_settings(&settings::default_settings(), cx)
                .unwrap();
            store
                .set_user_settings("{}", cx) // Start with empty settings
                .unwrap();
            cx.set_global(store);

            // Register FileSizeSettings AFTER store setup
            FileSizeSettings::register(cx);

            let settings = FileSizeSettings::get_global(cx);
            assert_eq!(settings.enabled, true);
            assert_eq!(settings.format, FileSizeFormat::Decimal);
        });
    }
}
