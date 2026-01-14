use crate::*;
use anyhow::{Context as _, Result, anyhow};
use collections::HashMap;
use fs::Fs;
use paths::{cursor_settings_file_paths, vscode_settings_file_paths};
use serde::Deserialize;
use serde_json::{Map, Value};
use std::{
    num::{NonZeroU32, NonZeroUsize},
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VsCodeSettingsSource {
    VsCode,
    Cursor,
}

impl std::fmt::Display for VsCodeSettingsSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VsCodeSettingsSource::VsCode => write!(f, "VS Code"),
            VsCodeSettingsSource::Cursor => write!(f, "Cursor"),
        }
    }
}

pub struct VsCodeSettings {
    pub source: VsCodeSettingsSource,
    pub path: Arc<Path>,
    content: Map<String, Value>,
}

impl VsCodeSettings {
    #[cfg(any(test, feature = "test-support"))]
    pub fn from_str(content: &str, source: VsCodeSettingsSource) -> Result<Self> {
        Ok(Self {
            source,
            path: Path::new("/example-path/Code/User/settings.json").into(),
            content: serde_json_lenient::from_str(content)?,
        })
    }

    pub async fn load_user_settings(source: VsCodeSettingsSource, fs: Arc<dyn Fs>) -> Result<Self> {
        let candidate_paths = match source {
            VsCodeSettingsSource::VsCode => vscode_settings_file_paths(),
            VsCodeSettingsSource::Cursor => cursor_settings_file_paths(),
        };
        let mut path = None;
        for candidate_path in candidate_paths.iter() {
            if fs.is_file(candidate_path).await {
                path = Some(candidate_path.clone());
            }
        }
        let Some(path) = path else {
            return Err(anyhow!(
                "No settings file found, expected to find it in one of the following paths:\n{}",
                candidate_paths
                    .into_iter()
                    .map(|path| path.to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
                    .join("\n")
            ));
        };
        let content = fs.load(&path).await.with_context(|| {
            format!(
                "Error loading {} settings file from {}",
                source,
                path.display()
            )
        })?;
        let content = serde_json_lenient::from_str(&content).with_context(|| {
            format!(
                "Error parsing {} settings file from {}",
                source,
                path.display()
            )
        })?;
        Ok(Self {
            source,
            path: path.into(),
            content,
        })
    }

    fn read_value(&self, setting: &str) -> Option<&Value> {
        self.content.get(setting)
    }

    fn read_str(&self, setting: &str) -> Option<&str> {
        self.read_value(setting).and_then(|v| v.as_str())
    }

    fn read_string(&self, setting: &str) -> Option<String> {
        self.read_value(setting)
            .and_then(|v| v.as_str())
            .map(|s| s.to_owned())
    }

    fn read_bool(&self, setting: &str) -> Option<bool> {
        self.read_value(setting).and_then(|v| v.as_bool())
    }

    fn read_f32(&self, setting: &str) -> Option<f32> {
        self.read_value(setting)
            .and_then(|v| v.as_f64())
            .map(|v| v as f32)
    }

    fn read_u64(&self, setting: &str) -> Option<u64> {
        self.read_value(setting).and_then(|v| v.as_u64())
    }

    fn read_usize(&self, setting: &str) -> Option<usize> {
        self.read_value(setting)
            .and_then(|v| v.as_u64())
            .and_then(|v| v.try_into().ok())
    }

    fn read_u32(&self, setting: &str) -> Option<u32> {
        self.read_value(setting)
            .and_then(|v| v.as_u64())
            .and_then(|v| v.try_into().ok())
    }

    fn read_enum<T>(&self, key: &str, f: impl FnOnce(&str) -> Option<T>) -> Option<T> {
        self.content.get(key).and_then(Value::as_str).and_then(f)
    }

    fn read_fonts(&self, key: &str) -> (Option<FontFamilyName>, Option<Vec<FontFamilyName>>) {
        let Some(css_name) = self.content.get(key).and_then(Value::as_str) else {
            return (None, None);
        };

        let mut name_buffer = String::new();
        let mut quote_char: Option<char> = None;
        let mut fonts = Vec::new();
        let mut add_font = |buffer: &mut String| {
            let trimmed = buffer.trim();
            if !trimmed.is_empty() {
                fonts.push(trimmed.to_string().into());
            }

            buffer.clear();
        };

        for ch in css_name.chars() {
            match (ch, quote_char) {
                ('"' | '\'', None) => {
                    quote_char = Some(ch);
                }
                (_, Some(q)) if ch == q => {
                    quote_char = None;
                }
                (',', None) => {
                    add_font(&mut name_buffer);
                }
                _ => {
                    name_buffer.push(ch);
                }
            }
        }

        add_font(&mut name_buffer);
        if fonts.is_empty() {
            return (None, None);
        }
        (Some(fonts.remove(0)), skip_default(fonts))
    }

    pub fn settings_content(&self) -> SettingsContent {
        SettingsContent {
            agent: self.agent_settings_content(),
            agent_servers: None,
            audio: None,
            auto_update: None,
            base_keymap: Some(BaseKeymapContent::VSCode),
            calls: None,
            collaboration_panel: None,
            debugger: None,
            diagnostics: None,
            disable_ai: None,
            editor: self.editor_settings_content(),
            extension: ExtensionSettingsContent::default(),
            file_finder: None,
            git: self.git_settings_content(),
            git_panel: self.git_panel_settings_content(),
            global_lsp_settings: None,
            helix_mode: None,
            image_viewer: None,
            journal: None,
            language_models: None,
            line_indicator_format: None,
            log: None,
            message_editor: None,
            node: self.node_binary_settings(),
            notification_panel: None,
            outline_panel: self.outline_panel_settings_content(),
            preview_tabs: self.preview_tabs_settings_content(),
            project: self.project_settings_content(),
            project_panel: self.project_panel_settings_content(),
            proxy: self.read_string("http.proxy"),
            remote: RemoteSettingsContent::default(),
            repl: None,
            server_url: None,
            session: None,
            status_bar: self.status_bar_settings_content(),
            tab_bar: self.tab_bar_settings_content(),
            tabs: self.item_settings_content(),
            telemetry: self.telemetry_settings_content(),
            terminal: self.terminal_settings_content(),
            theme: Box::new(self.theme_settings_content()),
            title_bar: None,
            vim: None,
            vim_mode: None,
            workspace: self.workspace_settings_content(),
            which_key: None,
        }
    }

    fn agent_settings_content(&self) -> Option<AgentSettingsContent> {
        let enabled = self.read_bool("chat.agent.enabled");
        skip_default(AgentSettingsContent {
            enabled: enabled,
            button: enabled,
            ..Default::default()
        })
    }

    fn editor_settings_content(&self) -> EditorSettingsContent {
        EditorSettingsContent {
            auto_signature_help: self.read_bool("editor.parameterHints.enabled"),
            autoscroll_on_clicks: None,
            cursor_blink: self.read_enum("editor.cursorBlinking", |s| match s {
                "blink" | "phase" | "expand" | "smooth" => Some(true),
                "solid" => Some(false),
                _ => None,
            }),
            cursor_shape: self.read_enum("editor.cursorStyle", |s| match s {
                "block" => Some(CursorShape::Block),
                "block-outline" => Some(CursorShape::Hollow),
                "line" | "line-thin" => Some(CursorShape::Bar),
                "underline" | "underline-thin" => Some(CursorShape::Underline),
                _ => None,
            }),
            current_line_highlight: self.read_enum("editor.renderLineHighlight", |s| match s {
                "gutter" => Some(CurrentLineHighlight::Gutter),
                "line" => Some(CurrentLineHighlight::Line),
                "all" => Some(CurrentLineHighlight::All),
                _ => None,
            }),
            diagnostics_max_severity: None,
            double_click_in_multibuffer: None,
            drag_and_drop_selection: None,
            excerpt_context_lines: None,
            expand_excerpt_lines: None,
            fast_scroll_sensitivity: self.read_f32("editor.fastScrollSensitivity"),
            sticky_scroll: self.sticky_scroll_content(),
            go_to_definition_fallback: None,
            gutter: self.gutter_content(),
            hide_mouse: None,
            horizontal_scroll_margin: None,
            hover_popover_delay: self.read_u64("editor.hover.delay").map(Into::into),
            hover_popover_enabled: self.read_bool("editor.hover.enabled"),
            inline_code_actions: None,
            jupyter: None,
            lsp_document_colors: None,
            lsp_highlight_debounce: None,
            middle_click_paste: None,
            minimap: self.minimap_content(),
            minimum_contrast_for_highlights: None,
            multi_cursor_modifier: self.read_enum("editor.multiCursorModifier", |s| match s {
                "ctrlCmd" => Some(MultiCursorModifier::CmdOrCtrl),
                "alt" => Some(MultiCursorModifier::Alt),
                _ => None,
            }),
            redact_private_values: None,
            relative_line_numbers: self.read_enum("editor.lineNumbers", |s| match s {
                "relative" => Some(RelativeLineNumbers::Enabled),
                _ => None,
            }),
            rounded_selection: self.read_bool("editor.roundedSelection"),
            scroll_beyond_last_line: None,
            scroll_sensitivity: self.read_f32("editor.mouseWheelScrollSensitivity"),
            scrollbar: self.scrollbar_content(),
            search: self.search_content(),
            search_wrap: None,
            seed_search_query_from_cursor: self.read_enum(
                "editor.find.seedSearchStringFromSelection",
                |s| match s {
                    "always" => Some(SeedQuerySetting::Always),
                    "selection" => Some(SeedQuerySetting::Selection),
                    "never" => Some(SeedQuerySetting::Never),
                    _ => None,
                },
            ),
            selection_highlight: self.read_bool("editor.selectionHighlight"),
            show_signature_help_after_edits: self.read_bool("editor.parameterHints.enabled"),
            snippet_sort_order: None,
            toolbar: None,
            use_smartcase_search: self.read_bool("search.smartCase"),
            vertical_scroll_margin: self.read_f32("editor.cursorSurroundingLines"),
            completion_menu_scrollbar: None,
            completion_detail_alignment: None,
        }
    }

    fn sticky_scroll_content(&self) -> Option<StickyScrollContent> {
        skip_default(StickyScrollContent {
            enabled: self.read_bool("editor.stickyScroll.enabled"),
        })
    }

    fn gutter_content(&self) -> Option<GutterContent> {
        skip_default(GutterContent {
            line_numbers: self.read_enum("editor.lineNumbers", |s| match s {
                "on" | "relative" => Some(true),
                "off" => Some(false),
                _ => None,
            }),
            min_line_number_digits: None,
            runnables: None,
            breakpoints: None,
            folds: self.read_enum("editor.showFoldingControls", |s| match s {
                "always" | "mouseover" => Some(true),
                "never" => Some(false),
                _ => None,
            }),
        })
    }

    fn scrollbar_content(&self) -> Option<ScrollbarContent> {
        let scrollbar_axes = skip_default(ScrollbarAxesContent {
            horizontal: self.read_enum("editor.scrollbar.horizontal", |s| match s {
                "auto" | "visible" => Some(true),
                "hidden" => Some(false),
                _ => None,
            }),
            vertical: self.read_enum("editor.scrollbar.vertical", |s| match s {
                "auto" | "visible" => Some(true),
                "hidden" => Some(false),
                _ => None,
            }),
        })?;

        Some(ScrollbarContent {
            axes: Some(scrollbar_axes),
            ..Default::default()
        })
    }

    fn search_content(&self) -> Option<SearchSettingsContent> {
        skip_default(SearchSettingsContent {
            include_ignored: self.read_bool("search.useIgnoreFiles"),
            ..Default::default()
        })
    }

    fn minimap_content(&self) -> Option<MinimapContent> {
        let minimap_enabled = self.read_bool("editor.minimap.enabled");
        let autohide = self.read_bool("editor.minimap.autohide");
        let show = match (minimap_enabled, autohide) {
            (Some(true), Some(false)) => Some(ShowMinimap::Always),
            (Some(true), _) => Some(ShowMinimap::Auto),
            (Some(false), _) => Some(ShowMinimap::Never),
            _ => None,
        };

        skip_default(MinimapContent {
            show,
            thumb: self.read_enum("editor.minimap.showSlider", |s| match s {
                "always" => Some(MinimapThumb::Always),
                "mouseover" => Some(MinimapThumb::Hover),
                _ => None,
            }),
            max_width_columns: self
                .read_u32("editor.minimap.maxColumn")
                .and_then(|v| NonZeroU32::new(v)),
            ..Default::default()
        })
    }

    fn git_panel_settings_content(&self) -> Option<GitPanelSettingsContent> {
        skip_default(GitPanelSettingsContent {
            button: self.read_bool("git.enabled"),
            fallback_branch_name: self.read_string("git.defaultBranchName"),
            ..Default::default()
        })
    }

    fn project_settings_content(&self) -> ProjectSettingsContent {
        ProjectSettingsContent {
            all_languages: AllLanguageSettingsContent {
                features: None,
                edit_predictions: self.edit_predictions_settings_content(),
                defaults: self.default_language_settings_content(),
                languages: Default::default(),
                file_types: self.file_types(),
            },
            worktree: self.worktree_settings_content(),
            lsp: Default::default(),
            terminal: None,
            dap: Default::default(),
            context_servers: self.context_servers(),
            context_server_timeout: None,
            load_direnv: None,
            slash_commands: None,
            git_hosting_providers: None,
        }
    }

    fn default_language_settings_content(&self) -> LanguageSettingsContent {
        LanguageSettingsContent {
            allow_rewrap: None,
            always_treat_brackets_as_autoclosed: None,
            auto_indent: None,
            auto_indent_on_paste: self.read_bool("editor.formatOnPaste"),
            code_actions_on_format: None,
            completions: skip_default(CompletionSettingsContent {
                words: self.read_bool("editor.suggest.showWords").map(|b| {
                    if b {
                        WordsCompletionMode::Enabled
                    } else {
                        WordsCompletionMode::Disabled
                    }
                }),
                ..Default::default()
            }),
            debuggers: None,
            edit_predictions_disabled_in: None,
            enable_language_server: None,
            ensure_final_newline_on_save: self.read_bool("files.insertFinalNewline"),
            extend_comment_on_newline: None,
            extend_list_on_newline: None,
            indent_list_on_tab: None,
            format_on_save: self.read_bool("editor.guides.formatOnSave").map(|b| {
                if b {
                    FormatOnSave::On
                } else {
                    FormatOnSave::Off
                }
            }),
            formatter: None,
            hard_tabs: self.read_bool("editor.insertSpaces").map(|v| !v),
            indent_guides: skip_default(IndentGuideSettingsContent {
                enabled: self.read_bool("editor.guides.indentation"),
                ..Default::default()
            }),
            inlay_hints: None,
            jsx_tag_auto_close: None,
            language_servers: None,
            linked_edits: self.read_bool("editor.linkedEditing"),
            preferred_line_length: self.read_u32("editor.wordWrapColumn"),
            prettier: None,
            remove_trailing_whitespace_on_save: self.read_bool("editor.trimAutoWhitespace"),
            show_completion_documentation: None,
            colorize_brackets: self.read_bool("editor.bracketPairColorization.enabled"),
            show_completions_on_input: self.read_bool("editor.suggestOnTriggerCharacters"),
            show_edit_predictions: self.read_bool("editor.inlineSuggest.enabled"),
            show_whitespaces: self.read_enum("editor.renderWhitespace", |s| {
                Some(match s {
                    "boundary" => ShowWhitespaceSetting::Boundary,
                    "trailing" => ShowWhitespaceSetting::Trailing,
                    "selection" => ShowWhitespaceSetting::Selection,
                    "all" => ShowWhitespaceSetting::All,
                    _ => ShowWhitespaceSetting::None,
                })
            }),
            show_wrap_guides: None,
            soft_wrap: self.read_enum("editor.wordWrap", |s| match s {
                "on" => Some(SoftWrap::EditorWidth),
                "wordWrapColumn" => Some(SoftWrap::PreferLine),
                "bounded" => Some(SoftWrap::Bounded),
                "off" => Some(SoftWrap::None),
                _ => None,
            }),
            tab_size: self
                .read_u32("editor.tabSize")
                .and_then(|n| NonZeroU32::new(n)),
            tasks: None,
            use_auto_surround: self.read_enum("editor.autoSurround", |s| match s {
                "languageDefined" | "quotes" | "brackets" => Some(true),
                "never" => Some(false),
                _ => None,
            }),
            use_autoclose: None,
            use_on_type_format: self.read_bool("editor.formatOnType"),
            whitespace_map: None,
            wrap_guides: self
                .read_value("editor.rulers")
                .and_then(|v| v.as_array())
                .map(|v| {
                    v.iter()
                        .flat_map(|n| n.as_u64().map(|n| n as usize))
                        .collect()
                }),
            word_diff_enabled: None,
        }
    }

    fn file_types(&self) -> Option<HashMap<Arc<str>, ExtendingVec<String>>> {
        // vscodes file association map is inverted from ours, so we flip the mapping before merging
        let mut associations: HashMap<Arc<str>, ExtendingVec<String>> = HashMap::default();
        let map = self.read_value("files.associations")?.as_object()?;
        for (k, v) in map {
            let Some(v) = v.as_str() else { continue };
            associations.entry(v.into()).or_default().0.push(k.clone());
        }
        skip_default(associations)
    }

    fn edit_predictions_settings_content(&self) -> Option<EditPredictionSettingsContent> {
        let disabled_globs = self
            .read_value("cursor.general.globalCursorIgnoreList")?
            .as_array()?;

        skip_default(EditPredictionSettingsContent {
            disabled_globs: skip_default(
                disabled_globs
                    .iter()
                    .filter_map(|glob| glob.as_str())
                    .map(|s| s.to_string())
                    .collect(),
            ),
            ..Default::default()
        })
    }

    fn outline_panel_settings_content(&self) -> Option<OutlinePanelSettingsContent> {
        skip_default(OutlinePanelSettingsContent {
            file_icons: self.read_bool("outline.icons"),
            folder_icons: self.read_bool("outline.icons"),
            git_status: self.read_bool("git.decorations.enabled"),
            ..Default::default()
        })
    }

    fn node_binary_settings(&self) -> Option<NodeBinarySettings> {
        // this just sets the binary name instead of a full path so it relies on path lookup
        // resolving to the one you want
        skip_default(NodeBinarySettings {
            npm_path: self.read_enum("npm.packageManager", |s| match s {
                v @ ("npm" | "yarn" | "bun" | "pnpm") => Some(v.to_owned()),
                _ => None,
            }),
            ..Default::default()
        })
    }

    fn git_settings_content(&self) -> Option<GitSettings> {
        let inline_blame = self.read_bool("git.blame.editorDecoration.enabled")?;
        skip_default(GitSettings {
            inline_blame: Some(InlineBlameSettings {
                enabled: Some(inline_blame),
                ..Default::default()
            }),
            ..Default::default()
        })
    }

    fn context_servers(&self) -> HashMap<Arc<str>, ContextServerSettingsContent> {
        #[derive(Deserialize)]
        struct VsCodeContextServerCommand {
            command: PathBuf,
            args: Option<Vec<String>>,
            env: Option<HashMap<String, String>>,
            // note: we don't support envFile and type
        }
        let Some(mcp) = self.read_value("mcp").and_then(|v| v.as_object()) else {
            return Default::default();
        };
        mcp.iter()
            .filter_map(|(k, v)| {
                Some((
                    k.clone().into(),
                    ContextServerSettingsContent::Stdio {
                        enabled: true,
                        command: serde_json::from_value::<VsCodeContextServerCommand>(v.clone())
                            .ok()
                            .map(|cmd| ContextServerCommand {
                                path: cmd.command,
                                args: cmd.args.unwrap_or_default(),
                                env: cmd.env,
                                timeout: None,
                            })?,
                    },
                ))
            })
            .collect()
    }

    fn item_settings_content(&self) -> Option<ItemSettingsContent> {
        skip_default(ItemSettingsContent {
            git_status: self.read_bool("git.decorations.enabled"),
            close_position: self.read_enum("workbench.editor.tabActionLocation", |s| match s {
                "right" => Some(ClosePosition::Right),
                "left" => Some(ClosePosition::Left),
                _ => None,
            }),
            file_icons: self.read_bool("workbench.editor.showIcons"),
            activate_on_close: self
                .read_bool("workbench.editor.focusRecentEditorAfterClose")
                .map(|b| {
                    if b {
                        ActivateOnClose::History
                    } else {
                        ActivateOnClose::LeftNeighbour
                    }
                }),
            show_diagnostics: None,
            show_close_button: self
                .read_bool("workbench.editor.tabActionCloseVisibility")
                .map(|b| {
                    if b {
                        ShowCloseButton::Always
                    } else {
                        ShowCloseButton::Hidden
                    }
                }),
        })
    }

    fn preview_tabs_settings_content(&self) -> Option<PreviewTabsSettingsContent> {
        skip_default(PreviewTabsSettingsContent {
            enabled: self.read_bool("workbench.editor.enablePreview"),
            enable_preview_from_project_panel: None,
            enable_preview_from_file_finder: self
                .read_bool("workbench.editor.enablePreviewFromQuickOpen"),
            enable_preview_from_multibuffer: None,
            enable_preview_multibuffer_from_code_navigation: None,
            enable_preview_file_from_code_navigation: None,
            enable_keep_preview_on_code_navigation: self
                .read_bool("workbench.editor.enablePreviewFromCodeNavigation"),
        })
    }

    fn tab_bar_settings_content(&self) -> Option<TabBarSettingsContent> {
        skip_default(TabBarSettingsContent {
            show: self.read_enum("workbench.editor.showTabs", |s| match s {
                "multiple" => Some(true),
                "single" | "none" => Some(false),
                _ => None,
            }),
            show_nav_history_buttons: None,
            show_tab_bar_buttons: self
                .read_str("workbench.editor.editorActionsLocation")
                .and_then(|str| if str == "hidden" { Some(false) } else { None }),
            show_pinned_tabs_in_separate_row: None,
        })
    }

    fn status_bar_settings_content(&self) -> Option<StatusBarSettingsContent> {
        skip_default(StatusBarSettingsContent {
            show: self.read_bool("workbench.statusBar.visible"),
            active_language_button: None,
            cursor_position_button: None,
            line_endings_button: None,
            active_encoding_button: None,
        })
    }

    fn project_panel_settings_content(&self) -> Option<ProjectPanelSettingsContent> {
        let mut project_panel_settings = ProjectPanelSettingsContent {
            auto_fold_dirs: self.read_bool("explorer.compactFolders"),
            auto_reveal_entries: self.read_bool("explorer.autoReveal"),
            button: None,
            default_width: None,
            dock: None,
            drag_and_drop: None,
            entry_spacing: None,
            file_icons: None,
            folder_icons: None,
            git_status: self.read_bool("git.decorations.enabled"),
            hide_gitignore: self.read_bool("explorer.excludeGitIgnore"),
            hide_hidden: None,
            hide_root: None,
            indent_guides: None,
            indent_size: None,
            scrollbar: None,
            show_diagnostics: self
                .read_bool("problems.decorations.enabled")
                .and_then(|b| if b { Some(ShowDiagnostics::Off) } else { None }),
            sort_mode: None,
            starts_open: None,
            sticky_scroll: None,
            auto_open: None,
        };

        if let (Some(false), Some(false)) = (
            self.read_bool("explorer.decorations.badges"),
            self.read_bool("explorer.decorations.colors"),
        ) {
            project_panel_settings.git_status = Some(false);
            project_panel_settings.show_diagnostics = Some(ShowDiagnostics::Off);
        }

        skip_default(project_panel_settings)
    }

    fn telemetry_settings_content(&self) -> Option<TelemetrySettingsContent> {
        self.read_enum("telemetry.telemetryLevel", |level| {
            let (metrics, diagnostics) = match level {
                "all" => (true, true),
                "error" | "crash" => (false, true),
                "off" => (false, false),
                _ => return None,
            };
            Some(TelemetrySettingsContent {
                metrics: Some(metrics),
                diagnostics: Some(diagnostics),
            })
        })
    }

    fn terminal_settings_content(&self) -> Option<TerminalSettingsContent> {
        let (font_family, font_fallbacks) = self.read_fonts("terminal.integrated.fontFamily");
        skip_default(TerminalSettingsContent {
            alternate_scroll: None,
            blinking: self
                .read_bool("terminal.integrated.cursorBlinking")
                .map(|b| {
                    if b {
                        TerminalBlink::On
                    } else {
                        TerminalBlink::Off
                    }
                }),
            button: None,
            copy_on_select: self.read_bool("terminal.integrated.copyOnSelection"),
            cursor_shape: self.read_enum("terminal.integrated.cursorStyle", |s| match s {
                "block" => Some(CursorShapeContent::Block),
                "line" => Some(CursorShapeContent::Bar),
                "underline" => Some(CursorShapeContent::Underline),
                _ => None,
            }),
            default_height: None,
            default_width: None,
            dock: None,
            font_fallbacks,
            font_family,
            font_features: None,
            font_size: self
                .read_f32("terminal.integrated.fontSize")
                .map(FontSize::from),
            font_weight: None,
            keep_selection_on_copy: None,
            line_height: self
                .read_f32("terminal.integrated.lineHeight")
                .map(|lh| TerminalLineHeight::Custom(lh)),
            max_scroll_history_lines: self.read_usize("terminal.integrated.scrollback"),
            minimum_contrast: None,
            option_as_meta: self.read_bool("terminal.integrated.macOptionIsMeta"),
            project: self.project_terminal_settings_content(),
            scrollbar: None,
            scroll_multiplier: None,
            toolbar: None,
        })
    }

    fn project_terminal_settings_content(&self) -> ProjectTerminalSettingsContent {
        #[cfg(target_os = "windows")]
        let platform = "windows";
        #[cfg(target_os = "linux")]
        let platform = "linux";
        #[cfg(target_os = "macos")]
        let platform = "osx";
        #[cfg(target_os = "freebsd")]
        let platform = "freebsd";
        let env = self
            .read_value(&format!("terminal.integrated.env.{platform}"))
            .and_then(|v| v.as_object())
            .map(|v| {
                v.iter()
                    .map(|(k, v)| (k.clone(), v.to_string()))
                    // zed does not support substitutions, so this can break env vars
                    .filter(|(_, v)| !v.contains('$'))
                    .collect()
            });

        ProjectTerminalSettingsContent {
            // TODO: handle arguments
            shell: self
                .read_string(&format!("terminal.integrated.{platform}Exec"))
                .map(|s| Shell::Program(s)),
            working_directory: None,
            env,
            detect_venv: None,
            path_hyperlink_regexes: None,
            path_hyperlink_timeout_ms: None,
        }
    }

    fn theme_settings_content(&self) -> ThemeSettingsContent {
        let (buffer_font_family, buffer_font_fallbacks) = self.read_fonts("editor.fontFamily");
        ThemeSettingsContent {
            ui_font_size: None,
            ui_font_family: None,
            ui_font_fallbacks: None,
            ui_font_features: None,
            ui_font_weight: None,
            buffer_font_family,
            buffer_font_fallbacks,
            buffer_font_size: self.read_f32("editor.fontSize").map(FontSize::from),
            buffer_font_weight: self.read_f32("editor.fontWeight").map(|w| w.into()),
            buffer_line_height: None,
            buffer_font_features: None,
            agent_ui_font_size: None,
            agent_buffer_font_size: None,
            theme: None,
            icon_theme: None,
            ui_density: None,
            unnecessary_code_fade: None,
            experimental_theme_overrides: None,
            theme_overrides: Default::default(),
        }
    }

    fn workspace_settings_content(&self) -> WorkspaceSettingsContent {
        WorkspaceSettingsContent {
            active_pane_modifiers: self.active_pane_modifiers(),
            text_rendering_mode: None,
            autosave: self.read_enum("files.autoSave", |s| match s {
                "off" => Some(AutosaveSetting::Off),
                "afterDelay" => Some(AutosaveSetting::AfterDelay {
                    milliseconds: self
                        .read_value("files.autoSaveDelay")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(1000)
                        .into(),
                }),
                "onFocusChange" => Some(AutosaveSetting::OnFocusChange),
                "onWindowChange" => Some(AutosaveSetting::OnWindowChange),
                _ => None,
            }),
            bottom_dock_layout: None,
            centered_layout: None,
            close_on_file_delete: None,
            command_aliases: Default::default(),
            confirm_quit: self.read_enum("window.confirmBeforeClose", |s| match s {
                "always" | "keyboardOnly" => Some(true),
                "never" => Some(false),
                _ => None,
            }),
            drop_target_size: None,
            // workbench.editor.limit contains "enabled", "value", and "perEditorGroup"
            // our semantics match if those are set to true, some N, and true respectively.
            // we'll ignore "perEditorGroup" for now since we only support a global max
            max_tabs: if self.read_bool("workbench.editor.limit.enabled") == Some(true) {
                self.read_usize("workbench.editor.limit.value")
                    .and_then(|n| NonZeroUsize::new(n))
            } else {
                None
            },
            on_last_window_closed: None,
            pane_split_direction_horizontal: None,
            pane_split_direction_vertical: None,
            resize_all_panels_in_dock: None,
            restore_on_file_reopen: self.read_bool("workbench.editor.restoreViewState"),
            restore_on_startup: None,
            window_decorations: None,
            show_call_status_icon: None,
            use_system_path_prompts: self.read_bool("files.simpleDialog.enable"),
            use_system_prompts: None,
            use_system_window_tabs: self.read_bool("window.nativeTabs"),
            when_closing_with_no_tabs: self.read_bool("window.closeWhenEmpty").map(|b| {
                if b {
                    CloseWindowWhenNoItems::CloseWindow
                } else {
                    CloseWindowWhenNoItems::KeepWindowOpen
                }
            }),
            zoomed_padding: None,
        }
    }

    fn active_pane_modifiers(&self) -> Option<ActivePaneModifiers> {
        if self.read_bool("accessibility.dimUnfocused.enabled") == Some(true)
            && let Some(opacity) = self.read_f32("accessibility.dimUnfocused.opacity")
        {
            Some(ActivePaneModifiers {
                border_size: None,
                inactive_opacity: Some(InactiveOpacity(opacity)),
            })
        } else {
            None
        }
    }

    fn worktree_settings_content(&self) -> WorktreeSettingsContent {
        WorktreeSettingsContent {
            project_name: None,
            prevent_sharing_in_public_channels: false,
            file_scan_exclusions: self
                .read_value("files.watcherExclude")
                .and_then(|v| v.as_array())
                .map(|v| {
                    v.iter()
                        .filter_map(|n| n.as_str().map(str::to_owned))
                        .collect::<Vec<_>>()
                })
                .filter(|r| !r.is_empty()),
            file_scan_inclusions: self
                .read_value("files.watcherInclude")
                .and_then(|v| v.as_array())
                .map(|v| {
                    v.iter()
                        .filter_map(|n| n.as_str().map(str::to_owned))
                        .collect::<Vec<_>>()
                })
                .filter(|r| !r.is_empty()),
            private_files: None,
            hidden_files: None,
            read_only_files: self
                .read_value("files.readonlyExclude")
                .and_then(|v| v.as_object())
                .map(|v| {
                    v.iter()
                        .filter_map(|(k, v)| {
                            if v.as_bool().unwrap_or(false) {
                                Some(k.to_owned())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .filter(|r| !r.is_empty()),
        }
    }
}

fn skip_default<T: Default + PartialEq>(value: T) -> Option<T> {
    if value == T::default() {
        None
    } else {
        Some(value)
    }
}
