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

use crate::*;

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

    pub fn read_value(&self, setting: &str) -> Option<&Value> {
        self.content.get(setting)
    }

    pub fn read_string(&self, setting: &str) -> Option<&str> {
        self.read_value(setting).and_then(|v| v.as_str())
    }

    pub fn read_bool(&self, setting: &str) -> Option<bool> {
        self.read_value(setting).and_then(|v| v.as_bool())
    }

    pub fn string_setting(&self, key: &str, setting: &mut Option<String>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_str) {
            *setting = Some(s.to_owned())
        }
    }

    pub fn bool_setting(&self, key: &str, setting: &mut Option<bool>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_bool) {
            *setting = Some(s)
        }
    }

    pub fn u32_setting(&self, key: &str, setting: &mut Option<u32>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_u64) {
            *setting = Some(s as u32)
        }
    }

    pub fn u64_setting(&self, key: &str, setting: &mut Option<u64>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_u64) {
            *setting = Some(s)
        }
    }

    pub fn usize_setting(&self, key: &str, setting: &mut Option<usize>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_u64) {
            *setting = Some(s.try_into().unwrap())
        }
    }

    pub fn f32_setting(&self, key: &str, setting: &mut Option<f32>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_f64) {
            *setting = Some(s as f32)
        }
    }

    pub fn from_f32_setting<T: From<f32>>(&self, key: &str, setting: &mut Option<T>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_f64) {
            *setting = Some(T::from(s as f32))
        }
    }

    pub fn enum_setting<T>(
        &self,
        key: &str,
        setting: &mut Option<T>,
        f: impl FnOnce(&str) -> Option<T>,
    ) {
        if let Some(s) = self.content.get(key).and_then(Value::as_str).and_then(f) {
            *setting = Some(s)
        }
    }

    pub fn read_enum<T>(&self, key: &str, f: impl FnOnce(&str) -> Option<T>) -> Option<T> {
        self.content.get(key).and_then(Value::as_str).and_then(f)
    }

    pub fn font_family_setting(
        &self,
        key: &str,
        font_family: &mut Option<FontFamilyName>,
        font_fallbacks: &mut Option<Vec<FontFamilyName>>,
    ) {
        let Some(css_name) = self.content.get(key).and_then(Value::as_str) else {
            return;
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

        let mut iter = fonts.into_iter();
        *font_family = iter.next();
        let fallbacks: Vec<_> = iter.collect();
        if !fallbacks.is_empty() {
            *font_fallbacks = Some(fallbacks);
        }
    }

    pub fn import(&self, current: &mut SettingsContent) {
        current.base_keymap = Some(BaseKeymapContent::VSCode);
        // agent settings
        if let Some(b) = self
            .read_value("chat.agent.enabled")
            .and_then(|b| b.as_bool())
        {
            current.agent.get_or_insert_default().enabled = Some(b);
            current.agent.get_or_insert_default().button = Some(b);
        }

        // client settings

        self.string_setting("http.proxy", &mut current.proxy);

        // editor settings

        self.enum_setting(
            "editor.cursorBlinking",
            &mut current.editor.cursor_blink,
            |s| match s {
                "blink" | "phase" | "expand" | "smooth" => Some(true),
                "solid" => Some(false),
                _ => None,
            },
        );
        self.enum_setting(
            "editor.cursorStyle",
            &mut current.editor.cursor_shape,
            |s| match s {
                "block" => Some(CursorShape::Block),
                "block-outline" => Some(CursorShape::Hollow),
                "line" | "line-thin" => Some(CursorShape::Bar),
                "underline" | "underline-thin" => Some(CursorShape::Underline),
                _ => None,
            },
        );

        self.enum_setting(
            "editor.renderLineHighlight",
            &mut current.editor.current_line_highlight,
            |s| match s {
                "gutter" => Some(CurrentLineHighlight::Gutter),
                "line" => Some(CurrentLineHighlight::Line),
                "all" => Some(CurrentLineHighlight::All),
                _ => None,
            },
        );

        self.bool_setting(
            "editor.selectionHighlight",
            &mut current.editor.selection_highlight,
        );
        self.bool_setting(
            "editor.roundedSelection",
            &mut current.editor.rounded_selection,
        );
        self.bool_setting(
            "editor.hover.enabled",
            &mut current.editor.hover_popover_enabled,
        );
        self.u64_setting(
            "editor.hover.delay",
            &mut current.editor.hover_popover_delay,
        );

        let mut gutter = GutterContent::default();
        self.enum_setting(
            "editor.showFoldingControls",
            &mut gutter.folds,
            |s| match s {
                "always" | "mouseover" => Some(true),
                "never" => Some(false),
                _ => None,
            },
        );
        self.enum_setting(
            "editor.lineNumbers",
            &mut gutter.line_numbers,
            |s| match s {
                "on" | "relative" => Some(true),
                "off" => Some(false),
                _ => None,
            },
        );
        if let Some(old_gutter) = current.editor.gutter.as_mut() {
            if gutter.folds.is_some() {
                old_gutter.folds = gutter.folds
            }
            if gutter.line_numbers.is_some() {
                old_gutter.line_numbers = gutter.line_numbers
            }
        } else if gutter != GutterContent::default() {
            current.editor.gutter = Some(gutter)
        }
        if let Some(b) = self.read_bool("editor.scrollBeyondLastLine") {
            current.editor.scroll_beyond_last_line = Some(if b {
                ScrollBeyondLastLine::OnePage
            } else {
                ScrollBeyondLastLine::Off
            })
        }

        let mut scrollbar_axes = crate::ScrollbarAxesContent::default();
        self.enum_setting(
            "editor.scrollbar.horizontal",
            &mut scrollbar_axes.horizontal,
            |s| match s {
                "auto" | "visible" => Some(true),
                "hidden" => Some(false),
                _ => None,
            },
        );
        self.enum_setting(
            "editor.scrollbar.vertical",
            &mut scrollbar_axes.horizontal,
            |s| match s {
                "auto" | "visible" => Some(true),
                "hidden" => Some(false),
                _ => None,
            },
        );

        if scrollbar_axes != crate::ScrollbarAxesContent::default() {
            let scrollbar_settings = current.editor.scrollbar.get_or_insert_default();
            let axes_settings = scrollbar_settings.axes.get_or_insert_default();

            if let Some(vertical) = scrollbar_axes.vertical {
                axes_settings.vertical = Some(vertical);
            }
            if let Some(horizontal) = scrollbar_axes.horizontal {
                axes_settings.horizontal = Some(horizontal);
            }
        }

        // TODO: check if this does the int->float conversion?
        self.f32_setting(
            "editor.cursorSurroundingLines",
            &mut current.editor.vertical_scroll_margin,
        );
        self.f32_setting(
            "editor.mouseWheelScrollSensitivity",
            &mut current.editor.scroll_sensitivity,
        );
        self.f32_setting(
            "editor.fastScrollSensitivity",
            &mut current.editor.fast_scroll_sensitivity,
        );
        if Some("relative") == self.read_string("editor.lineNumbers") {
            current.editor.relative_line_numbers = Some(true);
        }

        self.enum_setting(
            "editor.find.seedSearchStringFromSelection",
            &mut current.editor.seed_search_query_from_cursor,
            |s| match s {
                "always" => Some(SeedQuerySetting::Always),
                "selection" => Some(SeedQuerySetting::Selection),
                "never" => Some(SeedQuerySetting::Never),
                _ => None,
            },
        );
        self.bool_setting("search.smartCase", &mut current.editor.use_smartcase_search);
        self.enum_setting(
            "editor.multiCursorModifier",
            &mut current.editor.multi_cursor_modifier,
            |s| match s {
                "ctrlCmd" => Some(MultiCursorModifier::CmdOrCtrl),
                "alt" => Some(MultiCursorModifier::Alt),
                _ => None,
            },
        );

        self.bool_setting(
            "editor.parameterHints.enabled",
            &mut current.editor.auto_signature_help,
        );
        self.bool_setting(
            "editor.parameterHints.enabled",
            &mut current.editor.show_signature_help_after_edits,
        );

        if let Some(use_ignored) = self.read_bool("search.useIgnoreFiles") {
            let search = current.editor.search.get_or_insert_default();
            search.include_ignored = Some(use_ignored);
        }

        let mut minimap = crate::MinimapContent::default();
        let minimap_enabled = self.read_bool("editor.minimap.enabled").unwrap_or(true);
        let autohide = self.read_bool("editor.minimap.autohide");
        let mut max_width_columns: Option<u32> = None;
        self.u32_setting("editor.minimap.maxColumn", &mut max_width_columns);
        if minimap_enabled {
            if let Some(false) = autohide {
                minimap.show = Some(ShowMinimap::Always);
            } else {
                minimap.show = Some(ShowMinimap::Auto);
            }
        } else {
            minimap.show = Some(ShowMinimap::Never);
        }
        if let Some(max_width_columns) = max_width_columns {
            minimap.max_width_columns = NonZeroU32::new(max_width_columns);
        }

        self.enum_setting(
            "editor.minimap.showSlider",
            &mut minimap.thumb,
            |s| match s {
                "always" => Some(MinimapThumb::Always),
                "mouseover" => Some(MinimapThumb::Hover),
                _ => None,
            },
        );

        if minimap != crate::MinimapContent::default() {
            current.editor.minimap = Some(minimap)
        }

        // git

        if let Some(git_enabled) = self.read_bool("git.enabled") {
            current.git_panel.get_or_insert_default().button = Some(git_enabled);
        }
        if let Some(default_branch) = self.read_string("git.defaultBranchName") {
            current
                .git_panel
                .get_or_insert_default()
                .fallback_branch_name = Some(default_branch.to_string());
        }

        // langauge settings
        let d = &mut current.project.all_languages.defaults;
        if let Some(size) = self
            .read_value("editor.tabSize")
            .and_then(|v| v.as_u64())
            .and_then(|n| NonZeroU32::new(n as u32))
        {
            d.tab_size = Some(size);
        }
        if let Some(v) = self.read_bool("editor.insertSpaces") {
            d.hard_tabs = Some(!v);
        }

        self.enum_setting("editor.wordWrap", &mut d.soft_wrap, |s| match s {
            "on" => Some(SoftWrap::EditorWidth),
            "wordWrapColumn" => Some(SoftWrap::PreferLine),
            "bounded" => Some(SoftWrap::Bounded),
            "off" => Some(SoftWrap::None),
            _ => None,
        });
        self.u32_setting("editor.wordWrapColumn", &mut d.preferred_line_length);

        if let Some(arr) = self
            .read_value("editor.rulers")
            .and_then(|v| v.as_array())
            .map(|v| v.iter().map(|n| n.as_u64().map(|n| n as usize)).collect())
        {
            d.wrap_guides = arr;
        }
        if let Some(b) = self.read_bool("editor.guides.indentation") {
            d.indent_guides.get_or_insert_default().enabled = Some(b);
        }

        if let Some(b) = self.read_bool("editor.guides.formatOnSave") {
            d.format_on_save = Some(if b {
                FormatOnSave::On
            } else {
                FormatOnSave::Off
            });
        }
        self.bool_setting(
            "editor.trimAutoWhitespace",
            &mut d.remove_trailing_whitespace_on_save,
        );
        self.bool_setting(
            "files.insertFinalNewline",
            &mut d.ensure_final_newline_on_save,
        );
        self.bool_setting("editor.inlineSuggest.enabled", &mut d.show_edit_predictions);
        self.enum_setting("editor.renderWhitespace", &mut d.show_whitespaces, |s| {
            Some(match s {
                "boundary" => ShowWhitespaceSetting::Boundary,
                "trailing" => ShowWhitespaceSetting::Trailing,
                "selection" => ShowWhitespaceSetting::Selection,
                "all" => ShowWhitespaceSetting::All,
                _ => ShowWhitespaceSetting::None,
            })
        });
        self.enum_setting(
            "editor.autoSurround",
            &mut d.use_auto_surround,
            |s| match s {
                "languageDefined" | "quotes" | "brackets" => Some(true),
                "never" => Some(false),
                _ => None,
            },
        );
        self.bool_setting("editor.formatOnType", &mut d.use_on_type_format);
        self.bool_setting("editor.linkedEditing", &mut d.linked_edits);
        self.bool_setting("editor.formatOnPaste", &mut d.auto_indent_on_paste);
        self.bool_setting(
            "editor.suggestOnTriggerCharacters",
            &mut d.show_completions_on_input,
        );
        if let Some(b) = self.read_bool("editor.suggest.showWords") {
            let mode = if b {
                WordsCompletionMode::Enabled
            } else {
                WordsCompletionMode::Disabled
            };
            d.completions.get_or_insert_default().words = Some(mode);
        }
        // TODO: pull ^ out into helper and reuse for per-language settings

        // vscodes file association map is inverted from ours, so we flip the mapping before merging
        let mut associations: HashMap<Arc<str>, ExtendingVec<String>> = HashMap::default();
        if let Some(map) = self
            .read_value("files.associations")
            .and_then(|v| v.as_object())
        {
            for (k, v) in map {
                let Some(v) = v.as_str() else { continue };
                associations.entry(v.into()).or_default().0.push(k.clone());
            }
        }

        // TODO: do we want to merge imported globs per filetype? for now we'll just replace
        current
            .project
            .all_languages
            .file_types
            .get_or_insert_default()
            .extend(associations);

        // cursor global ignore list applies to cursor-tab, so transfer it to edit_predictions.disabled_globs
        if let Some(disabled_globs) = self
            .read_value("cursor.general.globalCursorIgnoreList")
            .and_then(|v| v.as_array())
        {
            current
                .project
                .all_languages
                .edit_predictions
                .get_or_insert_default()
                .disabled_globs
                .get_or_insert_default()
                .extend(
                    disabled_globs
                        .iter()
                        .filter_map(|glob| glob.as_str())
                        .map(|s| s.to_string()),
                );
        }

        // outline panel

        if let Some(b) = self.read_bool("outline.icons") {
            let outline_panel = current.outline_panel.get_or_insert_default();
            outline_panel.file_icons = Some(b);
            outline_panel.folder_icons = Some(b);
        }

        if let Some(b) = self.read_bool("git.decorations.enabled") {
            let outline_panel = current.outline_panel.get_or_insert_default();
            outline_panel.git_status = Some(b);
        }

        // project
        //
        // this just sets the binary name instead of a full path so it relies on path lookup
        // resolving to the one you want
        let npm_path = self.read_enum("npm.packageManager", |s| match s {
            v @ ("npm" | "yarn" | "bun" | "pnpm") => Some(v.to_owned()),
            _ => None,
        });
        if npm_path.is_some() {
            current.node.get_or_insert_default().npm_path = npm_path;
        }

        if let Some(b) = self.read_bool("git.blame.editorDecoration.enabled") {
            current
                .git
                .get_or_insert_default()
                .inline_blame
                .get_or_insert_default()
                .enabled = Some(b);
        }

        #[derive(Deserialize)]
        struct VsCodeContextServerCommand {
            command: PathBuf,
            args: Option<Vec<String>>,
            env: Option<HashMap<String, String>>,
            // note: we don't support envFile and type
        }
        if let Some(mcp) = self.read_value("mcp").and_then(|v| v.as_object()) {
            current
                .project
                .context_servers
                .extend(mcp.iter().filter_map(|(k, v)| {
                    Some((
                        k.clone().into(),
                        ContextServerSettingsContent::Custom {
                            enabled: true,
                            command: serde_json::from_value::<VsCodeContextServerCommand>(
                                v.clone(),
                            )
                            .ok()
                            .map(|cmd| ContextServerCommand {
                                path: cmd.command,
                                args: cmd.args.unwrap_or_default(),
                                env: cmd.env,
                                timeout: None,
                            })?,
                        },
                    ))
                }));
        }

        // project item settings

        if let Some(show) = self.read_bool("workbench.editor.decorations.colors") {
            current
                .tabs
                .get_or_insert_default()
                .git_status
                .replace(show);
        }

        // project item settings
        if let Some(hide_gitignore) = self.read_bool("explorer.excludeGitIgnore") {
            current.project_panel.get_or_insert_default().hide_gitignore = Some(hide_gitignore);
        }
        if let Some(auto_reveal) = self.read_bool("explorer.autoReveal") {
            current
                .project_panel
                .get_or_insert_default()
                .auto_reveal_entries = Some(auto_reveal);
        }
        if let Some(compact_folders) = self.read_bool("explorer.compactFolders") {
            current.project_panel.get_or_insert_default().auto_fold_dirs = Some(compact_folders);
        }

        if Some(false) == self.read_bool("git.decorations.enabled") {
            current.project_panel.get_or_insert_default().git_status = Some(false);
        }
        if Some(false) == self.read_bool("problems.decorations.enabled") {
            current
                .project_panel
                .get_or_insert_default()
                .show_diagnostics = Some(ShowDiagnostics::Off);
        }
        if let (Some(false), Some(false)) = (
            self.read_bool("explorer.decorations.badges"),
            self.read_bool("explorer.decorations.colors"),
        ) {
            current.project_panel.get_or_insert_default().git_status = Some(false);
            current
                .project_panel
                .get_or_insert_default()
                .show_diagnostics = Some(ShowDiagnostics::Off);
        }

        // telemetry

        let mut telemetry = TelemetrySettingsContent::default();
        self.enum_setting("telemetry.telemetryLevel", &mut telemetry.metrics, |s| {
            Some(s == "all")
        });
        self.enum_setting(
            "telemetry.telemetryLevel",
            &mut telemetry.diagnostics,
            |s| Some(matches!(s, "all" | "error" | "crash")),
        );
        // we could translate telemetry.telemetryLevel, but just because users didn't want
        // to send microsoft telemetry doesn't mean they don't want to send it to zed. their
        // all/error/crash/off correspond to combinations of our "diagnostics" and "metrics".
        if let Some(diagnostics) = telemetry.diagnostics {
            current.telemetry.get_or_insert_default().diagnostics = Some(diagnostics)
        }
        if let Some(metrics) = telemetry.metrics {
            current.telemetry.get_or_insert_default().metrics = Some(metrics)
        }

        // terminal settings
        let mut default = TerminalSettingsContent::default();
        let current_terminal = current.terminal.as_mut().unwrap_or(&mut default);
        let name = |s| format!("terminal.integrated.{s}");

        self.f32_setting(&name("fontSize"), &mut current_terminal.font_size);
        self.font_family_setting(
            &name("fontFamily"),
            &mut current_terminal.font_family,
            &mut current_terminal.font_fallbacks,
        );
        self.bool_setting(
            &name("copyOnSelection"),
            &mut current_terminal.copy_on_select,
        );
        self.bool_setting("macOptionIsMeta", &mut current_terminal.option_as_meta);
        self.usize_setting("scrollback", &mut current_terminal.max_scroll_history_lines);
        match self.read_bool(&name("cursorBlinking")) {
            Some(true) => current_terminal.blinking = Some(TerminalBlink::On),
            Some(false) => current_terminal.blinking = Some(TerminalBlink::Off),
            None => {}
        }
        self.enum_setting(
            &name("cursorStyle"),
            &mut current_terminal.cursor_shape,
            |s| match s {
                "block" => Some(CursorShapeContent::Block),
                "line" => Some(CursorShapeContent::Bar),
                "underline" => Some(CursorShapeContent::Underline),
                _ => None,
            },
        );
        // they also have "none" and "outline" as options but just for the "Inactive" variant
        if let Some(height) = self
            .read_value(&name("lineHeight"))
            .and_then(|v| v.as_f64())
        {
            current_terminal.line_height = Some(TerminalLineHeight::Custom(height as f32))
        }

        #[cfg(target_os = "windows")]
        let platform = "windows";
        #[cfg(target_os = "linux")]
        let platform = "linux";
        #[cfg(target_os = "macos")]
        let platform = "osx";
        #[cfg(target_os = "freebsd")]
        let platform = "freebsd";

        // TODO: handle arguments
        let shell_name = format!("{platform}Exec");
        if let Some(s) = self.read_string(&name(&shell_name)) {
            current_terminal.project.shell = Some(Shell::Program(s.to_owned()))
        }

        if let Some(env) = self
            .read_value(&name(&format!("env.{platform}")))
            .and_then(|v| v.as_object())
        {
            for (k, v) in env {
                if v.is_null()
                    && let Some(zed_env) = current_terminal.project.env.as_mut()
                {
                    zed_env.remove(k);
                }
                let Some(v) = v.as_str() else { continue };
                if let Some(zed_env) = current_terminal.project.env.as_mut() {
                    zed_env.insert(k.clone(), v.to_owned());
                } else {
                    current_terminal.project.env =
                        Some([(k.clone(), v.to_owned())].into_iter().collect())
                }
            }
        }
        if current.terminal.is_none() && default != TerminalSettingsContent::default() {
            current.terminal = Some(default)
        }

        // theme settings

        self.from_f32_setting("editor.fontWeight", &mut current.theme.buffer_font_weight);
        self.from_f32_setting("editor.fontSize", &mut current.theme.buffer_font_size);
        self.font_family_setting(
            "editor.fontFamily",
            &mut current.theme.buffer_font_family,
            &mut current.theme.buffer_font_fallbacks,
        );
        // TODO: possibly map editor.fontLigatures to buffer_font_features?

        // workspace settings

        if self
            .read_bool("accessibility.dimUnfocused.enabled")
            .unwrap_or_default()
            && let Some(opacity) = self
                .read_value("accessibility.dimUnfocused.opacity")
                .and_then(|v| v.as_f64())
        {
            current
                .workspace
                .active_pane_modifiers
                .get_or_insert_default()
                .inactive_opacity = Some(opacity as f32);
        }

        self.enum_setting(
            "window.confirmBeforeClose",
            &mut current.workspace.confirm_quit,
            |s| match s {
                "always" | "keyboardOnly" => Some(true),
                "never" => Some(false),
                _ => None,
            },
        );

        self.bool_setting(
            "workbench.editor.restoreViewState",
            &mut current.workspace.restore_on_file_reopen,
        );

        if let Some(b) = self.read_bool("window.closeWhenEmpty") {
            current.workspace.when_closing_with_no_tabs = Some(if b {
                CloseWindowWhenNoItems::CloseWindow
            } else {
                CloseWindowWhenNoItems::KeepWindowOpen
            });
        }

        if let Some(b) = self.read_bool("files.simpleDialog.enable") {
            current.workspace.use_system_path_prompts = Some(!b);
        }

        if let Some(v) = self.read_enum("files.autoSave", |s| match s {
            "off" => Some(AutosaveSetting::Off),
            "afterDelay" => Some(AutosaveSetting::AfterDelay {
                milliseconds: self
                    .read_value("files.autoSaveDelay")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(1000),
            }),
            "onFocusChange" => Some(AutosaveSetting::OnFocusChange),
            "onWindowChange" => Some(AutosaveSetting::OnWindowChange),
            _ => None,
        }) {
            current.workspace.autosave = Some(v);
        }

        // workbench.editor.limit contains "enabled", "value", and "perEditorGroup"
        // our semantics match if those are set to true, some N, and true respectively.
        // we'll ignore "perEditorGroup" for now since we only support a global max
        if let Some(n) = self
            .read_value("workbench.editor.limit.value")
            .and_then(|v| v.as_u64())
            .and_then(|n| NonZeroUsize::new(n as usize))
            && self
                .read_bool("workbench.editor.limit.enabled")
                .unwrap_or_default()
        {
            current.workspace.max_tabs = Some(n)
        }

        if let Some(b) = self.read_bool("window.nativeTabs") {
            current.workspace.use_system_window_tabs = Some(b);
        }

        // some combination of "window.restoreWindows" and "workbench.startupEditor" might
        // map to our "restore_on_startup"

        // there doesn't seem to be a way to read whether the bottom dock's "justified"
        // setting is enabled in vscode. that'd be our equivalent to "bottom_dock_layout"

        if let Some(b) = self.read_enum("workbench.editor.showTabs", |s| match s {
            "multiple" => Some(true),
            "single" | "none" => Some(false),
            _ => None,
        }) {
            current.tab_bar.get_or_insert_default().show = Some(b);
        }
        if Some("hidden") == self.read_string("workbench.editor.editorActionsLocation") {
            current.tab_bar.get_or_insert_default().show_tab_bar_buttons = Some(false)
        }

        if let Some(show) = self.read_bool("workbench.statusBar.visible") {
            current.status_bar.get_or_insert_default().show = Some(show);
        }

        if let Some(b) = self.read_bool("workbench.editor.tabActionCloseVisibility") {
            current.tabs.get_or_insert_default().show_close_button = Some(if b {
                ShowCloseButton::Always
            } else {
                ShowCloseButton::Hidden
            })
        }
        if let Some(s) = self.read_enum("workbench.editor.tabActionLocation", |s| match s {
            "right" => Some(ClosePosition::Right),
            "left" => Some(ClosePosition::Left),
            _ => None,
        }) {
            current.tabs.get_or_insert_default().close_position = Some(s)
        }
        if let Some(b) = self.read_bool("workbench.editor.focusRecentEditorAfterClose") {
            current.tabs.get_or_insert_default().activate_on_close = Some(if b {
                ActivateOnClose::History
            } else {
                ActivateOnClose::LeftNeighbour
            })
        }

        if let Some(b) = self.read_bool("workbench.editor.showIcons") {
            current.tabs.get_or_insert_default().file_icons = Some(b);
        };
        if let Some(b) = self.read_bool("git.decorations.enabled") {
            current.tabs.get_or_insert_default().git_status = Some(b);
        }

        if let Some(enabled) = self.read_bool("workbench.editor.enablePreview") {
            current.preview_tabs.get_or_insert_default().enabled = Some(enabled);
        }
        if let Some(enable_preview_from_code_navigation) =
            self.read_bool("workbench.editor.enablePreviewFromCodeNavigation")
        {
            current
                .preview_tabs
                .get_or_insert_default()
                .enable_preview_from_code_navigation = Some(enable_preview_from_code_navigation)
        }
        if let Some(enable_preview_from_file_finder) =
            self.read_bool("workbench.editor.enablePreviewFromQuickOpen")
        {
            current
                .preview_tabs
                .get_or_insert_default()
                .enable_preview_from_file_finder = Some(enable_preview_from_file_finder)
        }

        // worktree settings

        if let Some(inclusions) = self
            .read_value("files.watcherInclude")
            .and_then(|v| v.as_array())
            .and_then(|v| v.iter().map(|n| n.as_str().map(str::to_owned)).collect())
        {
            if let Some(old) = current.project.worktree.file_scan_inclusions.as_mut() {
                old.extend(inclusions)
            } else {
                current.project.worktree.file_scan_inclusions = Some(inclusions)
            }
        }
        if let Some(exclusions) = self
            .read_value("files.watcherExclude")
            .and_then(|v| v.as_array())
            .and_then(|v| v.iter().map(|n| n.as_str().map(str::to_owned)).collect())
        {
            if let Some(old) = current.project.worktree.file_scan_exclusions.as_mut() {
                old.extend(exclusions)
            } else {
                current.project.worktree.file_scan_exclusions = Some(exclusions)
            }
        }
    }
}
