//! Provides `language`-related settings.

use crate::{File, Language, LanguageName, LanguageServerName};
use collections::{FxHashMap, HashMap, HashSet};
use ec4rs::{
    Properties as EditorconfigProperties,
    property::{FinalNewline, IndentSize, IndentStyle, MaxLineLen, TabWidth, TrimTrailingWs},
};
use globset::{Glob, GlobMatcher, GlobSet, GlobSetBuilder};
use gpui::{App, Modifiers};
use itertools::{Either, Itertools};
use schemars::{JsonSchema, json_schema};
use serde::{Deserialize, Serialize};

pub use settings::{
    CompletionSettings, EditPredictionProvider, EditPredictionsMode, FormatOnSave, Formatter,
    FormatterList, IndentGuideSettings, InlayHintKind, LanguageSettingsContent, LspInsertMode,
    RewrapBehavior, SelectedFormatter, ShowWhitespaceSetting, SoftWrap, WordsCompletionMode,
};
use settings::{
    ParameterizedJsonSchema, Settings, SettingsContent, SettingsLocation, SettingsStore, SettingsUi,
};
use shellexpand;
use std::{borrow::Cow, num::NonZeroU32, path::Path, sync::Arc};
use util::MergeFrom;
use util::{ResultExt, schemars::replace_subschema};

/// Initializes the language settings.
pub fn init(cx: &mut App) {
    AllLanguageSettings::register(cx);
}

/// Returns the settings for the specified language from the provided file.
pub fn language_settings<'a>(
    language: Option<LanguageName>,
    file: Option<&'a Arc<dyn File>>,
    cx: &'a App,
) -> Cow<'a, LanguageSettings> {
    let location = file.map(|f| SettingsLocation {
        worktree_id: f.worktree_id(cx),
        path: f.path().as_ref(),
    });
    AllLanguageSettings::get(location, cx).language(location, language.as_ref(), cx)
}

/// Returns the settings for all languages from the provided file.
pub fn all_language_settings<'a>(
    file: Option<&'a Arc<dyn File>>,
    cx: &'a App,
) -> &'a AllLanguageSettings {
    let location = file.map(|f| SettingsLocation {
        worktree_id: f.worktree_id(cx),
        path: f.path().as_ref(),
    });
    AllLanguageSettings::get(location, cx)
}

/// The settings for all languages.
#[derive(Debug, Clone)]
pub struct AllLanguageSettings {
    /// The edit prediction settings.
    pub edit_predictions: EditPredictionSettings,
    pub defaults: LanguageSettings,
    languages: HashMap<LanguageName, LanguageSettings>,
    pub(crate) file_types: FxHashMap<Arc<str>, GlobSet>,
    pub(crate) file_globs: FxHashMap<Arc<str>, Vec<String>>,
}

/// The settings for a particular language.
#[derive(Debug, Clone)]
pub struct LanguageSettings {
    /// How many columns a tab should occupy.
    pub tab_size: NonZeroU32,
    /// Whether to indent lines using tab characters, as opposed to multiple
    /// spaces.
    pub hard_tabs: bool,
    /// How to soft-wrap long lines of text.
    pub soft_wrap: settings::SoftWrap,
    /// The column at which to soft-wrap lines, for buffers where soft-wrap
    /// is enabled.
    pub preferred_line_length: u32,
    /// Whether to show wrap guides (vertical rulers) in the editor.
    /// Setting this to true will show a guide at the 'preferred_line_length' value
    /// if softwrap is set to 'preferred_line_length', and will show any
    /// additional guides as specified by the 'wrap_guides' setting.
    pub show_wrap_guides: bool,
    /// Character counts at which to show wrap guides (vertical rulers) in the editor.
    pub wrap_guides: Vec<usize>,
    /// Indent guide related settings.
    /// todo!() shouldthis be not the content type?
    pub indent_guides: IndentGuideSettings,
    /// Whether or not to perform a buffer format before saving.
    pub format_on_save: FormatOnSave,
    /// Whether or not to remove any trailing whitespace from lines of a buffer
    /// before saving it.
    pub remove_trailing_whitespace_on_save: bool,
    /// Whether or not to ensure there's a single newline at the end of a buffer
    /// when saving it.
    pub ensure_final_newline_on_save: bool,
    /// How to perform a buffer format.
    pub formatter: settings::SelectedFormatter,
    /// Zed's Prettier integration settings.
    pub prettier: settings::PrettierSettingsContent,
    /// Whether to automatically close JSX tags.
    pub jsx_tag_auto_close: settings::JsxTagAutoCloseSettings,
    /// Whether to use language servers to provide code intelligence.
    pub enable_language_server: bool,
    /// The list of language servers to use (or disable) for this language.
    ///
    /// This array should consist of language server IDs, as well as the following
    /// special tokens:
    /// - `"!<language_server_id>"` - A language server ID prefixed with a `!` will be disabled.
    /// - `"..."` - A placeholder to refer to the **rest** of the registered language servers for this language.
    pub language_servers: Vec<String>,
    /// Controls where the `editor::Rewrap` action is allowed for this language.
    ///
    /// Note: This setting has no effect in Vim mode, as rewrap is already
    /// allowed everywhere.
    pub allow_rewrap: RewrapBehavior,
    /// Controls whether edit predictions are shown immediately (true)
    /// or manually by triggering `editor::ShowEditPrediction` (false).
    pub show_edit_predictions: bool,
    /// Controls whether edit predictions are shown in the given language
    /// scopes.
    pub edit_predictions_disabled_in: Vec<String>,
    /// Whether to show tabs and spaces in the editor.
    pub show_whitespaces: settings::ShowWhitespaceSetting,
    /// Visible characters used to render whitespace when show_whitespaces is enabled.
    pub whitespace_map: settings::WhitespaceMap,
    /// Whether to start a new line with a comment when a previous line is a comment as well.
    pub extend_comment_on_newline: bool,
    /// Inlay hint related settings.
    pub inlay_hints: InlayHintSettings,
    /// Whether to automatically close brackets.
    pub use_autoclose: bool,
    /// Whether to automatically surround text with brackets.
    pub use_auto_surround: bool,
    /// Whether to use additional LSP queries to format (and amend) the code after
    /// every "trigger" symbol input, defined by LSP server capabilities.
    pub use_on_type_format: bool,
    /// Whether indentation should be adjusted based on the context whilst typing.
    pub auto_indent: bool,
    /// Whether indentation of pasted content should be adjusted based on the context.
    pub auto_indent_on_paste: bool,
    /// Controls how the editor handles the autoclosed characters.
    pub always_treat_brackets_as_autoclosed: bool,
    /// Which code actions to run on save
    pub code_actions_on_format: HashMap<String, bool>,
    /// Whether to perform linked edits
    pub linked_edits: bool,
    /// Task configuration for this language.
    pub tasks: settings::LanguageTaskConfig,
    /// Whether to pop the completions menu while typing in an editor without
    /// explicitly requesting it.
    pub show_completions_on_input: bool,
    /// Whether to display inline and alongside documentation for items in the
    /// completions menu.
    pub show_completion_documentation: bool,
    /// Completion settings for this language.
    pub completions: settings::CompletionSettings,
    /// Preferred debuggers for this language.
    pub debuggers: Vec<String>,
}

impl LanguageSettings {
    /// A token representing the rest of the available language servers.
    const REST_OF_LANGUAGE_SERVERS: &'static str = "...";

    /// Returns the customized list of language servers from the list of
    /// available language servers.
    pub fn customized_language_servers(
        &self,
        available_language_servers: &[LanguageServerName],
    ) -> Vec<LanguageServerName> {
        Self::resolve_language_servers(&self.language_servers, available_language_servers)
    }

    pub(crate) fn resolve_language_servers(
        configured_language_servers: &[String],
        available_language_servers: &[LanguageServerName],
    ) -> Vec<LanguageServerName> {
        let (disabled_language_servers, enabled_language_servers): (
            Vec<LanguageServerName>,
            Vec<LanguageServerName>,
        ) = configured_language_servers.iter().partition_map(
            |language_server| match language_server.strip_prefix('!') {
                Some(disabled) => Either::Left(LanguageServerName(disabled.to_string().into())),
                None => Either::Right(LanguageServerName(language_server.clone().into())),
            },
        );

        let rest = available_language_servers
            .iter()
            .filter(|&available_language_server| {
                !disabled_language_servers.contains(available_language_server)
                    && !enabled_language_servers.contains(available_language_server)
            })
            .cloned()
            .collect::<Vec<_>>();

        enabled_language_servers
            .into_iter()
            .flat_map(|language_server| {
                if language_server.0.as_ref() == Self::REST_OF_LANGUAGE_SERVERS {
                    rest.clone()
                } else {
                    vec![language_server]
                }
            })
            .collect::<Vec<_>>()
    }
}

// The settings for inlay hints.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct InlayHintSettings {
    /// Global switch to toggle hints on and off.
    ///
    /// Default: false
    pub enabled: bool,
    /// Global switch to toggle inline values on and off when debugging.
    ///
    /// Default: true
    pub show_value_hints: bool,
    /// Whether type hints should be shown.
    ///
    /// Default: true
    pub show_type_hints: bool,
    /// Whether parameter hints should be shown.
    ///
    /// Default: true
    pub show_parameter_hints: bool,
    /// Whether other hints should be shown.
    ///
    /// Default: true
    pub show_other_hints: bool,
    /// Whether to show a background for inlay hints.
    ///
    /// If set to `true`, the background will use the `hint.background` color
    /// from the current theme.
    ///
    /// Default: false
    pub show_background: bool,
    /// Whether or not to debounce inlay hints updates after buffer edits.
    ///
    /// Set to 0 to disable debouncing.
    ///
    /// Default: 700
    pub edit_debounce_ms: u64,
    /// Whether or not to debounce inlay hints updates after buffer scrolls.
    ///
    /// Set to 0 to disable debouncing.
    ///
    /// Default: 50
    pub scroll_debounce_ms: u64,
    /// Toggles inlay hints (hides or shows) when the user presses the modifiers specified.
    /// If only a subset of the modifiers specified is pressed, hints are not toggled.
    /// If no modifiers are specified, this is equivalent to `None`.
    ///
    /// Default: None
    pub toggle_on_modifiers_press: Option<Modifiers>,
}

impl InlayHintSettings {
    /// Returns the kinds of inlay hints that are enabled based on the settings.
    pub fn enabled_inlay_hint_kinds(&self) -> HashSet<Option<InlayHintKind>> {
        let mut kinds = HashSet::default();
        if self.show_type_hints {
            kinds.insert(Some(InlayHintKind::Type));
        }
        if self.show_parameter_hints {
            kinds.insert(Some(InlayHintKind::Parameter));
        }
        if self.show_other_hints {
            kinds.insert(None);
        }
        kinds
    }
}

/// The settings for edit predictions, such as [GitHub Copilot](https://github.com/features/copilot)
/// or [Supermaven](https://supermaven.com).
#[derive(Clone, Debug, Default, SettingsUi)]
pub struct EditPredictionSettings {
    /// The provider that supplies edit predictions.
    pub provider: settings::EditPredictionProvider,
    /// A list of globs representing files that edit predictions should be disabled for.
    /// This list adds to a pre-existing, sensible default set of globs.
    /// Any additional ones you add are combined with them.
    #[settings_ui(skip)]
    pub disabled_globs: Vec<DisabledGlob>,
    /// Configures how edit predictions are displayed in the buffer.
    pub mode: settings::EditPredictionsMode,
    /// Settings specific to GitHub Copilot.
    pub copilot: CopilotSettings,
    /// Whether edit predictions are enabled in the assistant panel.
    /// This setting has no effect if globally disabled.
    pub enabled_in_text_threads: bool,
}

impl EditPredictionSettings {
    /// Returns whether edit predictions are enabled for the given path.
    pub fn enabled_for_file(&self, file: &Arc<dyn File>, cx: &App) -> bool {
        !self.disabled_globs.iter().any(|glob| {
            if glob.is_absolute {
                file.as_local()
                    .is_some_and(|local| glob.matcher.is_match(local.abs_path(cx)))
            } else {
                glob.matcher.is_match(file.path())
            }
        })
    }
}

#[derive(Clone, Debug)]
pub struct DisabledGlob {
    matcher: GlobMatcher,
    is_absolute: bool,
}

#[derive(Clone, Debug, Default, SettingsUi)]
pub struct CopilotSettings {
    /// HTTP/HTTPS proxy to use for Copilot.
    #[settings_ui(skip)]
    pub proxy: Option<String>,
    /// Disable certificate verification for proxy (not recommended).
    pub proxy_no_verify: Option<bool>,
    /// Enterprise URI for Copilot.
    #[settings_ui(skip)]
    pub enterprise_uri: Option<String>,
}

inventory::submit! {
    ParameterizedJsonSchema {
        add_and_get_ref: |generator, params, _cx| {
            let language_settings_content_ref = generator
                .subschema_for::<LanguageSettingsContent>()
                .to_value();
            replace_subschema::<settings::LanguageToSettingsMap>(generator, || json_schema!({
                "type": "object",
                "properties": params
                    .language_names
                    .iter()
                    .map(|name| {
                        (
                            name.clone(),
                            language_settings_content_ref.clone(),
                        )
                    })
                    .collect::<serde_json::Map<_, _>>()
            }))
        }
    }
}

impl AllLanguageSettings {
    /// Returns the [`LanguageSettings`] for the language with the specified name.
    pub fn language<'a>(
        &'a self,
        location: Option<SettingsLocation<'a>>,
        language_name: Option<&LanguageName>,
        cx: &'a App,
    ) -> Cow<'a, LanguageSettings> {
        let settings = language_name
            .and_then(|name| self.languages.get(name))
            .unwrap_or(&self.defaults);

        let editorconfig_properties = location.and_then(|location| {
            cx.global::<SettingsStore>()
                .editorconfig_properties(location.worktree_id, location.path)
        });
        if let Some(editorconfig_properties) = editorconfig_properties {
            let mut settings = settings.clone();
            merge_with_editorconfig(&mut settings, &editorconfig_properties);
            Cow::Owned(settings)
        } else {
            Cow::Borrowed(settings)
        }
    }

    /// Returns whether edit predictions are enabled for the given path.
    pub fn edit_predictions_enabled_for_file(&self, file: &Arc<dyn File>, cx: &App) -> bool {
        self.edit_predictions.enabled_for_file(file, cx)
    }

    /// Returns whether edit predictions are enabled for the given language and path.
    pub fn show_edit_predictions(&self, language: Option<&Arc<Language>>, cx: &App) -> bool {
        self.language(None, language.map(|l| l.name()).as_ref(), cx)
            .show_edit_predictions
    }

    /// Returns the edit predictions preview mode for the given language and path.
    pub fn edit_predictions_mode(&self) -> EditPredictionsMode {
        self.edit_predictions.mode
    }
}

fn merge_with_editorconfig(settings: &mut LanguageSettings, cfg: &EditorconfigProperties) {
    let preferred_line_length = cfg.get::<MaxLineLen>().ok().and_then(|v| match v {
        MaxLineLen::Value(u) => Some(u as u32),
        MaxLineLen::Off => None,
    });
    let tab_size = cfg.get::<IndentSize>().ok().and_then(|v| match v {
        IndentSize::Value(u) => NonZeroU32::new(u as u32),
        IndentSize::UseTabWidth => cfg.get::<TabWidth>().ok().and_then(|w| match w {
            TabWidth::Value(u) => NonZeroU32::new(u as u32),
        }),
    });
    let hard_tabs = cfg
        .get::<IndentStyle>()
        .map(|v| v.eq(&IndentStyle::Tabs))
        .ok();
    let ensure_final_newline_on_save = cfg
        .get::<FinalNewline>()
        .map(|v| match v {
            FinalNewline::Value(b) => b,
        })
        .ok();
    let remove_trailing_whitespace_on_save = cfg
        .get::<TrimTrailingWs>()
        .map(|v| match v {
            TrimTrailingWs::Value(b) => b,
        })
        .ok();
    fn merge<T>(target: &mut T, value: Option<T>) {
        if let Some(value) = value {
            *target = value;
        }
    }
    merge(&mut settings.preferred_line_length, preferred_line_length);
    merge(&mut settings.tab_size, tab_size);
    merge(&mut settings.hard_tabs, hard_tabs);
    merge(
        &mut settings.remove_trailing_whitespace_on_save,
        remove_trailing_whitespace_on_save,
    );
    merge(
        &mut settings.ensure_final_newline_on_save,
        ensure_final_newline_on_save,
    );
}

impl settings::Settings for AllLanguageSettings {
    fn from_defaults(content: &settings::SettingsContent, _cx: &mut App) -> Self {
        let all_languages = &content.project.all_languages;
        let defaults = all_languages.defaults.clone();
        let inlay_hints = defaults.inlay_hints.unwrap();
        let default_language_settings = LanguageSettings {
            tab_size: defaults.tab_size.unwrap(),
            hard_tabs: defaults.hard_tabs.unwrap(),
            soft_wrap: defaults.soft_wrap.unwrap(),
            preferred_line_length: defaults.preferred_line_length.unwrap(),
            show_wrap_guides: defaults.show_wrap_guides.unwrap(),
            wrap_guides: defaults.wrap_guides.unwrap(),
            indent_guides: defaults.indent_guides.unwrap(),
            format_on_save: defaults.format_on_save.unwrap(),
            remove_trailing_whitespace_on_save: defaults
                .remove_trailing_whitespace_on_save
                .unwrap(),
            ensure_final_newline_on_save: defaults.ensure_final_newline_on_save.unwrap(),
            formatter: defaults.formatter.unwrap(),
            prettier: defaults.prettier.unwrap(),
            jsx_tag_auto_close: defaults.jsx_tag_auto_close.unwrap(),
            enable_language_server: defaults.enable_language_server.unwrap(),
            language_servers: defaults.language_servers.unwrap(),
            allow_rewrap: defaults.allow_rewrap.unwrap(),
            show_edit_predictions: defaults.show_edit_predictions.unwrap(),
            edit_predictions_disabled_in: defaults.edit_predictions_disabled_in.unwrap(),
            show_whitespaces: defaults.show_whitespaces.unwrap(),
            whitespace_map: defaults.whitespace_map.unwrap(),
            extend_comment_on_newline: defaults.extend_comment_on_newline.unwrap(),
            inlay_hints: InlayHintSettings {
                enabled: inlay_hints.enabled.unwrap(),
                show_value_hints: inlay_hints.show_value_hints.unwrap(),
                show_type_hints: inlay_hints.show_type_hints.unwrap(),
                show_parameter_hints: inlay_hints.show_parameter_hints.unwrap(),
                show_other_hints: inlay_hints.show_other_hints.unwrap(),
                show_background: inlay_hints.show_background.unwrap(),
                edit_debounce_ms: inlay_hints.edit_debounce_ms.unwrap(),
                scroll_debounce_ms: inlay_hints.scroll_debounce_ms.unwrap(),
                toggle_on_modifiers_press: inlay_hints.toggle_on_modifiers_press,
            },
            use_autoclose: defaults.use_autoclose.unwrap(),
            use_auto_surround: defaults.use_auto_surround.unwrap(),
            use_on_type_format: defaults.use_on_type_format.unwrap(),
            auto_indent: defaults.auto_indent.unwrap(),
            auto_indent_on_paste: defaults.auto_indent_on_paste.unwrap(),
            always_treat_brackets_as_autoclosed: defaults
                .always_treat_brackets_as_autoclosed
                .unwrap(),
            code_actions_on_format: defaults.code_actions_on_format.unwrap(),
            linked_edits: defaults.linked_edits.unwrap(),
            tasks: defaults.tasks.unwrap(),
            show_completions_on_input: defaults.show_completions_on_input.unwrap(),
            show_completion_documentation: defaults.show_completion_documentation.unwrap(),
            completions: defaults.completions.unwrap(),
            debuggers: defaults.debuggers.unwrap(),
        };

        let mut languages = HashMap::default();
        for (language_name, settings) in &all_languages.languages.0 {
            let mut language_settings = default_language_settings.clone();
            merge_settings(&mut language_settings, settings);
            languages.insert(LanguageName(language_name.clone()), language_settings);
        }

        let edit_prediction_provider = all_languages
            .features
            .as_ref()
            .and_then(|f| f.edit_prediction_provider);

        let edit_predictions = all_languages.edit_predictions.clone().unwrap();
        let edit_predictions_mode = edit_predictions.mode.unwrap();

        let disabled_globs: HashSet<&String> = edit_predictions
            .disabled_globs
            .as_ref()
            .unwrap()
            .iter()
            .collect();

        let copilot = edit_predictions.copilot.unwrap();
        let copilot_settings = CopilotSettings {
            proxy: copilot.proxy,
            proxy_no_verify: copilot.proxy_no_verify,
            enterprise_uri: copilot.enterprise_uri,
        };

        let enabled_in_text_threads = edit_predictions.enabled_in_text_threads.unwrap();

        let mut file_types: FxHashMap<Arc<str>, GlobSet> = FxHashMap::default();
        let mut file_globs: FxHashMap<Arc<str>, Vec<String>> = FxHashMap::default();

        for (language, patterns) in &all_languages.file_types {
            let mut builder = GlobSetBuilder::new();

            for pattern in patterns {
                builder.add(Glob::new(pattern).unwrap());
            }

            file_types.insert(language.clone(), builder.build().unwrap());
            file_globs.insert(language.clone(), patterns.clone());
        }

        Self {
            edit_predictions: EditPredictionSettings {
                provider: if let Some(provider) = edit_prediction_provider {
                    provider
                } else {
                    EditPredictionProvider::None
                },
                disabled_globs: disabled_globs
                    .iter()
                    .filter_map(|g| {
                        let expanded_g = shellexpand::tilde(g).into_owned();
                        Some(DisabledGlob {
                            matcher: globset::Glob::new(&expanded_g).ok()?.compile_matcher(),
                            is_absolute: Path::new(&expanded_g).is_absolute(),
                        })
                    })
                    .collect(),
                mode: edit_predictions_mode,
                copilot: copilot_settings,
                enabled_in_text_threads,
            },
            defaults: default_language_settings,
            languages,
            file_types,
            file_globs,
        }
    }

    fn refine(&mut self, content: &SettingsContent, _cx: &mut App) {
        let all_languages = &content.project.all_languages;
        if let Some(provider) = all_languages
            .features
            .as_ref()
            .and_then(|f| f.edit_prediction_provider)
        {
            self.edit_predictions.provider = provider;
        }

        if let Some(edit_predictions) = all_languages.edit_predictions.as_ref() {
            self.edit_predictions
                .mode
                .merge_from(&edit_predictions.mode);
            self.edit_predictions
                .enabled_in_text_threads
                .merge_from(&edit_predictions.enabled_in_text_threads);

            if let Some(disabled_globs) = edit_predictions.disabled_globs.as_ref() {
                self.edit_predictions
                    .disabled_globs
                    .extend(disabled_globs.iter().filter_map(|g| {
                        let expanded_g = shellexpand::tilde(g).into_owned();
                        Some(DisabledGlob {
                            matcher: globset::Glob::new(&expanded_g).ok()?.compile_matcher(),
                            is_absolute: Path::new(&expanded_g).is_absolute(),
                        })
                    }));
            }
        }

        if let Some(proxy) = all_languages
            .edit_predictions
            .as_ref()
            .and_then(|settings| settings.copilot.as_ref()?.proxy.clone())
        {
            self.edit_predictions.copilot.proxy = Some(proxy);
        }

        if let Some(proxy_no_verify) = all_languages
            .edit_predictions
            .as_ref()
            .and_then(|settings| settings.copilot.as_ref()?.proxy_no_verify)
        {
            self.edit_predictions.copilot.proxy_no_verify = Some(proxy_no_verify);
        }

        if let Some(enterprise_uri) = all_languages
            .edit_predictions
            .as_ref()
            .and_then(|settings| settings.copilot.as_ref()?.enterprise_uri.clone())
        {
            self.edit_predictions.copilot.enterprise_uri = Some(enterprise_uri);
        }

        // A user's global settings override the default global settings and
        // all default language-specific settings.
        merge_settings(&mut self.defaults, &all_languages.defaults);
        for language_settings in self.languages.values_mut() {
            merge_settings(language_settings, &all_languages.defaults);
        }

        // A user's language-specific settings override default language-specific settings.
        for (language_name, user_language_settings) in &all_languages.languages.0 {
            merge_settings(
                self.languages
                    .entry(LanguageName(language_name.clone()))
                    .or_insert_with(|| self.defaults.clone()),
                user_language_settings,
            );
        }

        for (language, patterns) in &all_languages.file_types {
            let mut builder = GlobSetBuilder::new();

            let default_value = self.file_globs.get(&language.clone());

            // Merge the default value with the user's value.
            if let Some(patterns) = default_value {
                for pattern in patterns {
                    if let Some(glob) = Glob::new(pattern).log_err() {
                        builder.add(glob);
                    }
                }
            }

            for pattern in patterns {
                if let Some(glob) = Glob::new(pattern).log_err() {
                    builder.add(glob);
                }
            }

            self.file_globs
                .entry(language.clone())
                .or_default()
                .extend(patterns.clone());

            if let Some(matcher) = builder.build().log_err() {
                self.file_types.insert(language.clone(), matcher);
            }
        }
    }

    fn import_from_vscode(vscode: &settings::VsCodeSettings, current: &mut SettingsContent) {
        let d = &mut current.project.all_languages.defaults;
        if let Some(size) = vscode
            .read_value("editor.tabSize")
            .and_then(|v| v.as_u64())
            .and_then(|n| NonZeroU32::new(n as u32))
        {
            d.tab_size = Some(size);
        }
        if let Some(v) = vscode.read_bool("editor.insertSpaces") {
            d.hard_tabs = Some(!v);
        }

        vscode.enum_setting("editor.wordWrap", &mut d.soft_wrap, |s| match s {
            "on" => Some(SoftWrap::EditorWidth),
            "wordWrapColumn" => Some(SoftWrap::PreferLine),
            "bounded" => Some(SoftWrap::Bounded),
            "off" => Some(SoftWrap::None),
            _ => None,
        });
        vscode.u32_setting("editor.wordWrapColumn", &mut d.preferred_line_length);

        if let Some(arr) = vscode
            .read_value("editor.rulers")
            .and_then(|v| v.as_array())
            .map(|v| v.iter().map(|n| n.as_u64().map(|n| n as usize)).collect())
        {
            d.wrap_guides = arr;
        }
        if let Some(b) = vscode.read_bool("editor.guides.indentation") {
            if let Some(guide_settings) = d.indent_guides.as_mut() {
                guide_settings.enabled = b;
            } else {
                d.indent_guides = Some(IndentGuideSettings {
                    enabled: b,
                    ..Default::default()
                });
            }
        }

        if let Some(b) = vscode.read_bool("editor.guides.formatOnSave") {
            d.format_on_save = Some(if b {
                FormatOnSave::On
            } else {
                FormatOnSave::Off
            });
        }
        vscode.bool_setting(
            "editor.trimAutoWhitespace",
            &mut d.remove_trailing_whitespace_on_save,
        );
        vscode.bool_setting(
            "files.insertFinalNewline",
            &mut d.ensure_final_newline_on_save,
        );
        vscode.bool_setting("editor.inlineSuggest.enabled", &mut d.show_edit_predictions);
        vscode.enum_setting("editor.renderWhitespace", &mut d.show_whitespaces, |s| {
            Some(match s {
                "boundary" => ShowWhitespaceSetting::Boundary,
                "trailing" => ShowWhitespaceSetting::Trailing,
                "selection" => ShowWhitespaceSetting::Selection,
                "all" => ShowWhitespaceSetting::All,
                _ => ShowWhitespaceSetting::None,
            })
        });
        vscode.enum_setting(
            "editor.autoSurround",
            &mut d.use_auto_surround,
            |s| match s {
                "languageDefined" | "quotes" | "brackets" => Some(true),
                "never" => Some(false),
                _ => None,
            },
        );
        vscode.bool_setting("editor.formatOnType", &mut d.use_on_type_format);
        vscode.bool_setting("editor.linkedEditing", &mut d.linked_edits);
        vscode.bool_setting("editor.formatOnPaste", &mut d.auto_indent_on_paste);
        vscode.bool_setting(
            "editor.suggestOnTriggerCharacters",
            &mut d.show_completions_on_input,
        );
        if let Some(b) = vscode.read_bool("editor.suggest.showWords") {
            let mode = if b {
                WordsCompletionMode::Enabled
            } else {
                WordsCompletionMode::Disabled
            };
            if let Some(completion_settings) = d.completions.as_mut() {
                completion_settings.words = mode;
            } else {
                d.completions = Some(CompletionSettings {
                    words: mode,
                    words_min_length: 3,
                    lsp: true,
                    lsp_fetch_timeout_ms: 0,
                    lsp_insert_mode: LspInsertMode::ReplaceSuffix,
                });
            }
        }
        // TODO: pull ^ out into helper and reuse for per-language settings

        // vscodes file association map is inverted from ours, so we flip the mapping before merging
        let mut associations: HashMap<Arc<str>, Vec<String>> = HashMap::default();
        if let Some(map) = vscode
            .read_value("files.associations")
            .and_then(|v| v.as_object())
        {
            for (k, v) in map {
                let Some(v) = v.as_str() else { continue };
                associations.entry(v.into()).or_default().push(k.clone());
            }
        }

        // TODO: do we want to merge imported globs per filetype? for now we'll just replace
        current
            .project
            .all_languages
            .file_types
            .extend(associations);

        // cursor global ignore list applies to cursor-tab, so transfer it to edit_predictions.disabled_globs
        if let Some(disabled_globs) = vscode
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
    }
}

fn merge_settings(settings: &mut LanguageSettings, src: &LanguageSettingsContent) {
    settings.tab_size.merge_from(&src.tab_size);
    settings.tab_size = settings
        .tab_size
        .clamp(NonZeroU32::new(1).unwrap(), NonZeroU32::new(16).unwrap());

    settings.hard_tabs.merge_from(&src.hard_tabs);
    settings.soft_wrap.merge_from(&src.soft_wrap);
    settings.use_autoclose.merge_from(&src.use_autoclose);
    settings
        .use_auto_surround
        .merge_from(&src.use_auto_surround);
    settings
        .use_on_type_format
        .merge_from(&src.use_on_type_format);
    settings.auto_indent.merge_from(&src.auto_indent);
    settings
        .auto_indent_on_paste
        .merge_from(&src.auto_indent_on_paste);
    settings
        .always_treat_brackets_as_autoclosed
        .merge_from(&src.always_treat_brackets_as_autoclosed);
    settings.show_wrap_guides.merge_from(&src.show_wrap_guides);
    settings.wrap_guides.merge_from(&src.wrap_guides);
    settings.indent_guides.merge_from(&src.indent_guides);
    settings
        .code_actions_on_format
        .merge_from(&src.code_actions_on_format.clone());
    settings.linked_edits.merge_from(&src.linked_edits);
    settings.tasks.merge_from(&src.tasks.clone());

    settings
        .preferred_line_length
        .merge_from(&src.preferred_line_length);
    settings.formatter.merge_from(&src.formatter.clone());
    settings.prettier.merge_from(&src.prettier.clone());
    settings
        .jsx_tag_auto_close
        .merge_from(&src.jsx_tag_auto_close.clone());
    settings
        .format_on_save
        .merge_from(&src.format_on_save.clone());
    settings
        .remove_trailing_whitespace_on_save
        .merge_from(&src.remove_trailing_whitespace_on_save);
    settings
        .ensure_final_newline_on_save
        .merge_from(&src.ensure_final_newline_on_save);
    settings
        .enable_language_server
        .merge_from(&src.enable_language_server);
    settings
        .language_servers
        .merge_from(&src.language_servers.clone());
    settings.allow_rewrap.merge_from(&src.allow_rewrap);
    settings
        .show_edit_predictions
        .merge_from(&src.show_edit_predictions);
    settings
        .edit_predictions_disabled_in
        .merge_from(&src.edit_predictions_disabled_in.clone());
    settings.show_whitespaces.merge_from(&src.show_whitespaces);
    settings
        .whitespace_map
        .merge_from(&src.whitespace_map.clone());
    settings
        .extend_comment_on_newline
        .merge_from(&src.extend_comment_on_newline);
    if let Some(inlay_hints) = &src.inlay_hints {
        settings
            .inlay_hints
            .enabled
            .merge_from(&inlay_hints.enabled);
        settings
            .inlay_hints
            .show_value_hints
            .merge_from(&inlay_hints.show_value_hints);
        settings
            .inlay_hints
            .show_type_hints
            .merge_from(&inlay_hints.show_type_hints);
        settings
            .inlay_hints
            .show_parameter_hints
            .merge_from(&inlay_hints.show_parameter_hints);
        settings
            .inlay_hints
            .show_other_hints
            .merge_from(&inlay_hints.show_other_hints);
        settings
            .inlay_hints
            .show_background
            .merge_from(&inlay_hints.show_background);
        settings
            .inlay_hints
            .edit_debounce_ms
            .merge_from(&inlay_hints.edit_debounce_ms);
        settings
            .inlay_hints
            .scroll_debounce_ms
            .merge_from(&inlay_hints.scroll_debounce_ms);
        if let Some(toggle_on_modifiers_press) = &inlay_hints.toggle_on_modifiers_press {
            settings.inlay_hints.toggle_on_modifiers_press =
                Some(toggle_on_modifiers_press.clone());
        }
    }
    settings
        .show_completions_on_input
        .merge_from(&src.show_completions_on_input);
    settings
        .show_completion_documentation
        .merge_from(&src.show_completion_documentation);
    settings.completions.merge_from(&src.completions.clone());
}

/// Allows to enable/disable formatting with Prettier
/// and configure default Prettier, used when no project-level Prettier installation is found.
/// Prettier formatting is disabled by default.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, SettingsUi)]
pub struct PrettierSettings {
    /// Enables or disables formatting with Prettier for a given language.
    #[serde(default)]
    pub allowed: bool,

    /// Forces Prettier integration to use a specific parser name when formatting files with the language.
    #[serde(default)]
    pub parser: Option<String>,

    /// Forces Prettier integration to use specific plugins when formatting files with the language.
    /// The default Prettier will be installed with these plugins.
    #[serde(default)]
    #[settings_ui(skip)]
    pub plugins: HashSet<String>,

    /// Default Prettier options, in the format as in package.json section for Prettier.
    /// If project installs Prettier via its package.json, these options will be ignored.
    #[serde(flatten)]
    #[settings_ui(skip)]
    pub options: HashMap<String, serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, SettingsUi)]
pub struct JsxTagAutoCloseSettings {
    /// Enables or disables auto-closing of JSX tags.
    #[serde(default)]
    pub enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[gpui::test]
    fn test_edit_predictions_enabled_for_file(cx: &mut TestAppContext) {
        use crate::TestFile;
        use std::path::PathBuf;

        let cx = cx.app.borrow_mut();

        let build_settings = |globs: &[&str]| -> EditPredictionSettings {
            EditPredictionSettings {
                disabled_globs: globs
                    .iter()
                    .map(|glob_str| {
                        #[cfg(windows)]
                        let glob_str = {
                            let mut g = String::new();

                            if glob_str.starts_with('/') {
                                g.push_str("C:");
                            }

                            g.push_str(&glob_str.replace('/', "\\"));
                            g
                        };
                        #[cfg(windows)]
                        let glob_str = glob_str.as_str();
                        let expanded_glob_str = shellexpand::tilde(glob_str).into_owned();
                        DisabledGlob {
                            matcher: globset::Glob::new(&expanded_glob_str)
                                .unwrap()
                                .compile_matcher(),
                            is_absolute: Path::new(&expanded_glob_str).is_absolute(),
                        }
                    })
                    .collect(),
                ..Default::default()
            }
        };

        const WORKTREE_NAME: &str = "project";
        let make_test_file = |segments: &[&str]| -> Arc<dyn File> {
            let mut path_buf = PathBuf::new();
            path_buf.extend(segments);

            Arc::new(TestFile {
                path: path_buf.as_path().into(),
                root_name: WORKTREE_NAME.to_string(),
                local_root: Some(PathBuf::from(if cfg!(windows) {
                    "C:\\absolute\\"
                } else {
                    "/absolute/"
                })),
            })
        };

        let test_file = make_test_file(&["src", "test", "file.rs"]);

        // Test relative globs
        let settings = build_settings(&["*.rs"]);
        assert!(!settings.enabled_for_file(&test_file, &cx));
        let settings = build_settings(&["*.txt"]);
        assert!(settings.enabled_for_file(&test_file, &cx));

        // Test absolute globs
        let settings = build_settings(&["/absolute/**/*.rs"]);
        assert!(!settings.enabled_for_file(&test_file, &cx));
        let settings = build_settings(&["/other/**/*.rs"]);
        assert!(settings.enabled_for_file(&test_file, &cx));

        // Test exact path match relative
        let settings = build_settings(&["src/test/file.rs"]);
        assert!(!settings.enabled_for_file(&test_file, &cx));
        let settings = build_settings(&["src/test/otherfile.rs"]);
        assert!(settings.enabled_for_file(&test_file, &cx));

        // Test exact path match absolute
        let settings = build_settings(&[&format!("/absolute/{}/src/test/file.rs", WORKTREE_NAME)]);
        assert!(!settings.enabled_for_file(&test_file, &cx));
        let settings = build_settings(&["/other/test/otherfile.rs"]);
        assert!(settings.enabled_for_file(&test_file, &cx));

        // Test * glob
        let settings = build_settings(&["*"]);
        assert!(!settings.enabled_for_file(&test_file, &cx));
        let settings = build_settings(&["*.txt"]);
        assert!(settings.enabled_for_file(&test_file, &cx));

        // Test **/* glob
        let settings = build_settings(&["**/*"]);
        assert!(!settings.enabled_for_file(&test_file, &cx));
        let settings = build_settings(&["other/**/*"]);
        assert!(settings.enabled_for_file(&test_file, &cx));

        // Test directory/** glob
        let settings = build_settings(&["src/**"]);
        assert!(!settings.enabled_for_file(&test_file, &cx));

        let test_file_root: Arc<dyn File> = Arc::new(TestFile {
            path: PathBuf::from("file.rs").as_path().into(),
            root_name: WORKTREE_NAME.to_string(),
            local_root: Some(PathBuf::from("/absolute/")),
        });
        assert!(settings.enabled_for_file(&test_file_root, &cx));

        let settings = build_settings(&["other/**"]);
        assert!(settings.enabled_for_file(&test_file, &cx));

        // Test **/directory/* glob
        let settings = build_settings(&["**/test/*"]);
        assert!(!settings.enabled_for_file(&test_file, &cx));
        let settings = build_settings(&["**/other/*"]);
        assert!(settings.enabled_for_file(&test_file, &cx));

        // Test multiple globs
        let settings = build_settings(&["*.rs", "*.txt", "src/**"]);
        assert!(!settings.enabled_for_file(&test_file, &cx));
        let settings = build_settings(&["*.txt", "*.md", "other/**"]);
        assert!(settings.enabled_for_file(&test_file, &cx));

        // Test dot files
        let dot_file = make_test_file(&[".config", "settings.json"]);
        let settings = build_settings(&[".*/**"]);
        assert!(!settings.enabled_for_file(&dot_file, &cx));

        let dot_env_file = make_test_file(&[".env"]);
        let settings = build_settings(&[".env"]);
        assert!(!settings.enabled_for_file(&dot_env_file, &cx));

        // Test tilde expansion
        let home = shellexpand::tilde("~").into_owned();
        let home_file = make_test_file(&[&home, "test.rs"]);
        let settings = build_settings(&["~/test.rs"]);
        assert!(!settings.enabled_for_file(&home_file, &cx));
    }

    #[test]
    fn test_resolve_language_servers() {
        fn language_server_names(names: &[&str]) -> Vec<LanguageServerName> {
            names
                .iter()
                .copied()
                .map(|name| LanguageServerName(name.to_string().into()))
                .collect::<Vec<_>>()
        }

        let available_language_servers = language_server_names(&[
            "typescript-language-server",
            "biome",
            "deno",
            "eslint",
            "tailwind",
        ]);

        // A value of just `["..."]` is the same as taking all of the available language servers.
        assert_eq!(
            LanguageSettings::resolve_language_servers(
                &[LanguageSettings::REST_OF_LANGUAGE_SERVERS.into()],
                &available_language_servers,
            ),
            available_language_servers
        );

        // Referencing one of the available language servers will change its order.
        assert_eq!(
            LanguageSettings::resolve_language_servers(
                &[
                    "biome".into(),
                    LanguageSettings::REST_OF_LANGUAGE_SERVERS.into(),
                    "deno".into()
                ],
                &available_language_servers
            ),
            language_server_names(&[
                "biome",
                "typescript-language-server",
                "eslint",
                "tailwind",
                "deno",
            ])
        );

        // Negating an available language server removes it from the list.
        assert_eq!(
            LanguageSettings::resolve_language_servers(
                &[
                    "deno".into(),
                    "!typescript-language-server".into(),
                    "!biome".into(),
                    LanguageSettings::REST_OF_LANGUAGE_SERVERS.into()
                ],
                &available_language_servers
            ),
            language_server_names(&["deno", "eslint", "tailwind"])
        );

        // Adding a language server not in the list of available language servers adds it to the list.
        assert_eq!(
            LanguageSettings::resolve_language_servers(
                &[
                    "my-cool-language-server".into(),
                    LanguageSettings::REST_OF_LANGUAGE_SERVERS.into()
                ],
                &available_language_servers
            ),
            language_server_names(&[
                "my-cool-language-server",
                "typescript-language-server",
                "biome",
                "deno",
                "eslint",
                "tailwind",
            ])
        );
    }
}
