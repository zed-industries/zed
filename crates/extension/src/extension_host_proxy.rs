use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Result, anyhow};
use fs::Fs;
use gpui::{App, EntityId, Global, ReadGlobal, SharedString, Task};
use language::{BinaryStatus, LanguageLoader, LanguageMatcher, LanguageName};
use lsp::LanguageServerName;
use parking_lot::RwLock;

use crate::{
    Extension, ExtensionPanelAction, ExtensionPanelActionProxy, ExtensionPanelDescriptor,
    ExtensionPanelEvent, ExtensionPanelId, ExtensionPanelUiProxy,
};

#[derive(Default)]
struct GlobalExtensionHostProxy(Arc<ExtensionHostProxy>);

impl Global for GlobalExtensionHostProxy {}

/// A proxy for interacting with the extension host.
///
/// This object implements each of the individual proxy types so that their
/// methods can be called directly on it.
/// Registration function for language model providers.
pub type LanguageModelProviderRegistration = Box<dyn FnOnce(&mut App) + Send>;

#[derive(Default)]
pub struct ExtensionHostProxy {
    theme_proxy: RwLock<Option<Arc<dyn ExtensionThemeProxy>>>,
    grammar_proxy: RwLock<Option<Arc<dyn ExtensionGrammarProxy>>>,
    language_proxy: RwLock<Option<Arc<dyn ExtensionLanguageProxy>>>,
    language_server_proxy: RwLock<Option<Arc<dyn ExtensionLanguageServerProxy>>>,
    snippet_proxy: RwLock<Option<Arc<dyn ExtensionSnippetProxy>>>,
    context_server_proxy: RwLock<Option<Arc<dyn ExtensionContextServerProxy>>>,
    debug_adapter_provider_proxy: RwLock<Option<Arc<dyn ExtensionDebugAdapterProviderProxy>>>,
    language_model_provider_proxy: RwLock<Option<Arc<dyn ExtensionLanguageModelProviderProxy>>>,
    panel_ui_proxy: RwLock<Option<Arc<dyn ExtensionPanelUiProxy>>>,
    panel_action_proxy: RwLock<Option<Arc<dyn ExtensionPanelActionProxy>>>,
    editor_proxy: RwLock<Option<Arc<dyn ExtensionEditorProxy>>>,
    ui_proxy: RwLock<Option<Arc<dyn ExtensionUiProxy>>>,
    console_proxy: RwLock<Option<Arc<dyn ExtensionConsoleProxy>>>,
    command_dispatcher: RwLock<Option<Arc<dyn ExtensionCommandDispatcher>>>,
    ui_response_dispatcher: RwLock<Option<Arc<dyn ExtensionUiResponseDispatcher>>>,
    console_input_dispatcher: RwLock<Option<Arc<dyn ExtensionConsoleInputDispatcher>>>,
    events_dispatcher: RwLock<Option<Arc<dyn ExtensionEventsDispatcher>>>,
    process_dispatcher: RwLock<Option<Arc<dyn ExtensionProcessDispatcher>>>,
    tcp_dispatcher: RwLock<Option<Arc<dyn ExtensionTcpDispatcher>>>,
}

impl ExtensionHostProxy {
    /// Returns the global [`ExtensionHostProxy`].
    pub fn global(cx: &App) -> Arc<Self> {
        GlobalExtensionHostProxy::global(cx).0.clone()
    }

    /// Returns the global [`ExtensionHostProxy`].
    ///
    /// Inserts a default [`ExtensionHostProxy`] if one does not yet exist.
    pub fn default_global(cx: &mut App) -> Arc<Self> {
        cx.default_global::<GlobalExtensionHostProxy>().0.clone()
    }

    pub fn new() -> Self {
        Self {
            theme_proxy: RwLock::default(),
            grammar_proxy: RwLock::default(),
            language_proxy: RwLock::default(),
            language_server_proxy: RwLock::default(),
            snippet_proxy: RwLock::default(),
            context_server_proxy: RwLock::default(),
            debug_adapter_provider_proxy: RwLock::default(),
            language_model_provider_proxy: RwLock::default(),
            panel_ui_proxy: RwLock::default(),
            panel_action_proxy: RwLock::default(),
            editor_proxy: RwLock::default(),
            ui_proxy: RwLock::default(),
            console_proxy: RwLock::default(),
            command_dispatcher: RwLock::default(),
            ui_response_dispatcher: RwLock::default(),
            console_input_dispatcher: RwLock::default(),
            events_dispatcher: RwLock::default(),
            process_dispatcher: RwLock::default(),
            tcp_dispatcher: RwLock::default(),
        }
    }

    pub fn register_theme_proxy(&self, proxy: impl ExtensionThemeProxy) {
        self.theme_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_grammar_proxy(&self, proxy: impl ExtensionGrammarProxy) {
        self.grammar_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_language_proxy(&self, proxy: impl ExtensionLanguageProxy) {
        self.language_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_language_server_proxy(&self, proxy: impl ExtensionLanguageServerProxy) {
        self.language_server_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_snippet_proxy(&self, proxy: impl ExtensionSnippetProxy) {
        self.snippet_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_context_server_proxy(&self, proxy: impl ExtensionContextServerProxy) {
        self.context_server_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_debug_adapter_proxy(&self, proxy: impl ExtensionDebugAdapterProviderProxy) {
        self.debug_adapter_provider_proxy
            .write()
            .replace(Arc::new(proxy));
    }

    pub fn register_language_model_provider_proxy(
        &self,
        proxy: impl ExtensionLanguageModelProviderProxy,
    ) {
        self.language_model_provider_proxy
            .write()
            .replace(Arc::new(proxy));
    }

    pub fn register_panel_ui_proxy(&self, proxy: impl ExtensionPanelUiProxy) {
        self.panel_ui_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_panel_action_proxy(&self, proxy: impl ExtensionPanelActionProxy) {
        self.panel_action_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_editor_proxy(&self, proxy: impl ExtensionEditorProxy) {
        self.editor_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_ui_proxy(&self, proxy: impl ExtensionUiProxy) {
        self.ui_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_console_proxy(&self, proxy: impl ExtensionConsoleProxy) {
        self.console_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_command_dispatcher(&self, proxy: impl ExtensionCommandDispatcher) {
        self.command_dispatcher.write().replace(Arc::new(proxy));
    }

    pub fn register_ui_response_dispatcher(&self, proxy: impl ExtensionUiResponseDispatcher) {
        self.ui_response_dispatcher.write().replace(Arc::new(proxy));
    }

    pub fn register_console_input_dispatcher(&self, proxy: impl ExtensionConsoleInputDispatcher) {
        self.console_input_dispatcher
            .write()
            .replace(Arc::new(proxy));
    }

    pub fn register_events_dispatcher(&self, proxy: impl ExtensionEventsDispatcher) {
        self.events_dispatcher.write().replace(Arc::new(proxy));
    }

    pub fn register_process_dispatcher(&self, proxy: impl ExtensionProcessDispatcher) {
        self.process_dispatcher.write().replace(Arc::new(proxy));
    }

    pub fn register_tcp_dispatcher(&self, proxy: impl ExtensionTcpDispatcher) {
        self.tcp_dispatcher.write().replace(Arc::new(proxy));
    }
}

impl ExtensionPanelUiProxy for ExtensionHostProxy {
    fn open_panel(&self, descriptor: ExtensionPanelDescriptor, cx: &mut App) -> Result<()> {
        let Some(proxy) = self.panel_ui_proxy.read().clone() else {
            return Err(anyhow!("extension panels are not available"));
        };
        proxy.open_panel(descriptor, cx)
    }

    fn send_panel_event(
        &self,
        panel: ExtensionPanelId,
        event: ExtensionPanelEvent,
        cx: &mut App,
    ) -> Result<()> {
        let Some(proxy) = self.panel_ui_proxy.read().clone() else {
            return Err(anyhow!("extension panels are not available"));
        };
        proxy.send_panel_event(panel, event, cx)
    }

    fn active_worktree_root(&self, cx: &mut App) -> Result<String> {
        let Some(proxy) = self.panel_ui_proxy.read().clone() else {
            return Err(anyhow!("extension panels are not available"));
        };
        proxy.active_worktree_root(cx)
    }

    fn read_active_worktree_file(&self, path: &str, cx: &mut App) -> Result<String> {
        let Some(proxy) = self.panel_ui_proxy.read().clone() else {
            return Err(anyhow!("extension panels are not available"));
        };
        proxy.read_active_worktree_file(path, cx)
    }
}

impl ExtensionPanelActionProxy for ExtensionHostProxy {
    fn dispatch_panel_action(
        &self,
        action: ExtensionPanelAction,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let Some(proxy) = self.panel_action_proxy.read().clone() else {
            return Task::ready(Err(anyhow!("extension panel actions are not available")));
        };
        proxy.dispatch_panel_action(action, cx)
    }
}

pub trait ExtensionThemeProxy: Send + Sync + 'static {
    fn set_extensions_loaded(&self);

    fn list_theme_names(&self, theme_path: PathBuf, fs: Arc<dyn Fs>) -> Task<Result<Vec<String>>>;

    fn remove_user_themes(&self, themes: Vec<SharedString>);

    fn load_user_theme(&self, theme_path: PathBuf, fs: Arc<dyn Fs>) -> Task<Result<()>>;

    fn reload_current_theme(&self, cx: &mut App);

    fn list_icon_theme_names(
        &self,
        icon_theme_path: PathBuf,
        fs: Arc<dyn Fs>,
    ) -> Task<Result<Vec<String>>>;

    fn remove_icon_themes(&self, icon_themes: Vec<SharedString>);

    fn load_icon_theme(
        &self,
        icon_theme_path: PathBuf,
        icons_root_dir: PathBuf,
        fs: Arc<dyn Fs>,
    ) -> Task<Result<()>>;

    fn reload_current_icon_theme(&self, cx: &mut App);
}

impl ExtensionThemeProxy for ExtensionHostProxy {
    fn set_extensions_loaded(&self) {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return;
        };

        proxy.set_extensions_loaded()
    }

    fn list_theme_names(&self, theme_path: PathBuf, fs: Arc<dyn Fs>) -> Task<Result<Vec<String>>> {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return Task::ready(Ok(Vec::new()));
        };

        proxy.list_theme_names(theme_path, fs)
    }

    fn remove_user_themes(&self, themes: Vec<SharedString>) {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return;
        };

        proxy.remove_user_themes(themes)
    }

    fn load_user_theme(&self, theme_path: PathBuf, fs: Arc<dyn Fs>) -> Task<Result<()>> {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return Task::ready(Ok(()));
        };

        proxy.load_user_theme(theme_path, fs)
    }

    fn reload_current_theme(&self, cx: &mut App) {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return;
        };

        proxy.reload_current_theme(cx)
    }

    fn list_icon_theme_names(
        &self,
        icon_theme_path: PathBuf,
        fs: Arc<dyn Fs>,
    ) -> Task<Result<Vec<String>>> {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return Task::ready(Ok(Vec::new()));
        };

        proxy.list_icon_theme_names(icon_theme_path, fs)
    }

    fn remove_icon_themes(&self, icon_themes: Vec<SharedString>) {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return;
        };

        proxy.remove_icon_themes(icon_themes)
    }

    fn load_icon_theme(
        &self,
        icon_theme_path: PathBuf,
        icons_root_dir: PathBuf,
        fs: Arc<dyn Fs>,
    ) -> Task<Result<()>> {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return Task::ready(Ok(()));
        };

        proxy.load_icon_theme(icon_theme_path, icons_root_dir, fs)
    }

    fn reload_current_icon_theme(&self, cx: &mut App) {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return;
        };

        proxy.reload_current_icon_theme(cx)
    }
}

pub trait ExtensionGrammarProxy: Send + Sync + 'static {
    fn register_grammars(&self, grammars: Vec<(Arc<str>, PathBuf)>);
}

impl ExtensionGrammarProxy for ExtensionHostProxy {
    #[ztracing::instrument(skip_all)]
    fn register_grammars(&self, grammars: Vec<(Arc<str>, PathBuf)>) {
        let Some(proxy) = self.grammar_proxy.read().clone() else {
            return;
        };

        proxy.register_grammars(grammars)
    }
}

pub trait ExtensionLanguageProxy: Send + Sync + 'static {
    fn register_language(
        &self,
        language: LanguageName,
        grammar: Option<Arc<str>>,
        matcher: Arc<LanguageMatcher>,
        hidden: bool,
        load: LanguageLoader,
    ) -> bool;

    fn is_language_registered(&self, language: &LanguageName) -> bool;

    fn remove_languages(
        &self,
        languages_to_remove: &[LanguageName],
        grammars_to_remove: &[Arc<str>],
    );
}

impl ExtensionLanguageProxy for ExtensionHostProxy {
    #[ztracing::instrument(skip_all, fields(lang = language.0.as_str()))]
    fn register_language(
        &self,
        language: LanguageName,
        grammar: Option<Arc<str>>,
        matcher: Arc<LanguageMatcher>,
        hidden: bool,
        load: LanguageLoader,
    ) -> bool {
        let Some(proxy) = self.language_proxy.read().clone() else {
            return false;
        };

        proxy.register_language(language, grammar, matcher, hidden, load)
    }

    fn is_language_registered(&self, language: &LanguageName) -> bool {
        let Some(proxy) = self.language_proxy.read().clone() else {
            return false;
        };

        proxy.is_language_registered(language)
    }

    fn remove_languages(
        &self,
        languages_to_remove: &[LanguageName],
        grammars_to_remove: &[Arc<str>],
    ) {
        let Some(proxy) = self.language_proxy.read().clone() else {
            return;
        };

        proxy.remove_languages(languages_to_remove, grammars_to_remove)
    }
}

pub trait ExtensionLanguageServerProxy: Send + Sync + 'static {
    fn register_language_server(
        &self,
        extension: Arc<dyn Extension>,
        language_server_id: LanguageServerName,
        language: LanguageName,
    );

    fn remove_language_server(
        &self,
        language: &LanguageName,
        language_server_id: &LanguageServerName,
        cx: &mut App,
    ) -> Task<Result<()>>;

    fn update_language_server_status(
        &self,
        source: Option<EntityId>,
        language_server_id: LanguageServerName,
        status: BinaryStatus,
    );
}

impl ExtensionLanguageServerProxy for ExtensionHostProxy {
    fn register_language_server(
        &self,
        extension: Arc<dyn Extension>,
        language_server_id: LanguageServerName,
        language: LanguageName,
    ) {
        let Some(proxy) = self.language_server_proxy.read().clone() else {
            return;
        };

        proxy.register_language_server(extension, language_server_id, language)
    }

    fn remove_language_server(
        &self,
        language: &LanguageName,
        language_server_id: &LanguageServerName,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let Some(proxy) = self.language_server_proxy.read().clone() else {
            return Task::ready(Ok(()));
        };

        proxy.remove_language_server(language, language_server_id, cx)
    }

    fn update_language_server_status(
        &self,
        source: Option<EntityId>,
        language_server_id: LanguageServerName,
        status: BinaryStatus,
    ) {
        let Some(proxy) = self.language_server_proxy.read().clone() else {
            return;
        };

        proxy.update_language_server_status(source, language_server_id, status)
    }
}

pub trait ExtensionSnippetProxy: Send + Sync + 'static {
    fn register_snippet(&self, path: &PathBuf, snippet_contents: &str) -> Result<()>;
}

impl ExtensionSnippetProxy for ExtensionHostProxy {
    fn register_snippet(&self, path: &PathBuf, snippet_contents: &str) -> Result<()> {
        let Some(proxy) = self.snippet_proxy.read().clone() else {
            return Ok(());
        };

        proxy.register_snippet(path, snippet_contents)
    }
}

pub trait ExtensionContextServerProxy: Send + Sync + 'static {
    fn register_context_server(
        &self,
        extension: Arc<dyn Extension>,
        server_id: Arc<str>,
        cx: &mut App,
    );

    fn unregister_context_server(&self, server_id: Arc<str>, cx: &mut App);
}

impl ExtensionContextServerProxy for ExtensionHostProxy {
    fn register_context_server(
        &self,
        extension: Arc<dyn Extension>,
        server_id: Arc<str>,
        cx: &mut App,
    ) {
        let Some(proxy) = self.context_server_proxy.read().clone() else {
            return;
        };

        proxy.register_context_server(extension, server_id, cx)
    }

    fn unregister_context_server(&self, server_id: Arc<str>, cx: &mut App) {
        let Some(proxy) = self.context_server_proxy.read().clone() else {
            return;
        };

        proxy.unregister_context_server(server_id, cx)
    }
}

pub trait ExtensionDebugAdapterProviderProxy: Send + Sync + 'static {
    fn register_debug_adapter(
        &self,
        extension: Arc<dyn Extension>,
        debug_adapter_name: Arc<str>,
        schema_path: &Path,
    );
    fn register_debug_locator(&self, extension: Arc<dyn Extension>, locator_name: Arc<str>);
    fn unregister_debug_adapter(&self, debug_adapter_name: Arc<str>);
    fn unregister_debug_locator(&self, locator_name: Arc<str>);
}

impl ExtensionDebugAdapterProviderProxy for ExtensionHostProxy {
    fn register_debug_adapter(
        &self,
        extension: Arc<dyn Extension>,
        debug_adapter_name: Arc<str>,
        schema_path: &Path,
    ) {
        let Some(proxy) = self.debug_adapter_provider_proxy.read().clone() else {
            return;
        };

        proxy.register_debug_adapter(extension, debug_adapter_name, schema_path)
    }

    fn register_debug_locator(&self, extension: Arc<dyn Extension>, locator_name: Arc<str>) {
        let Some(proxy) = self.debug_adapter_provider_proxy.read().clone() else {
            return;
        };

        proxy.register_debug_locator(extension, locator_name)
    }
    fn unregister_debug_adapter(&self, debug_adapter_name: Arc<str>) {
        let Some(proxy) = self.debug_adapter_provider_proxy.read().clone() else {
            return;
        };

        proxy.unregister_debug_adapter(debug_adapter_name)
    }
    fn unregister_debug_locator(&self, locator_name: Arc<str>) {
        let Some(proxy) = self.debug_adapter_provider_proxy.read().clone() else {
            return;
        };

        proxy.unregister_debug_locator(locator_name)
    }
}

pub trait ExtensionLanguageModelProviderProxy: Send + Sync + 'static {
    fn register_language_model_provider(
        &self,
        provider_id: Arc<str>,
        register_fn: LanguageModelProviderRegistration,
        cx: &mut App,
    );

    fn unregister_language_model_provider(&self, provider_id: Arc<str>, cx: &mut App);
}

impl ExtensionLanguageModelProviderProxy for ExtensionHostProxy {
    fn register_language_model_provider(
        &self,
        provider_id: Arc<str>,
        register_fn: LanguageModelProviderRegistration,
        cx: &mut App,
    ) {
        let Some(proxy) = self.language_model_provider_proxy.read().clone() else {
            return;
        };

        proxy.register_language_model_provider(provider_id, register_fn, cx)
    }

    fn unregister_language_model_provider(&self, provider_id: Arc<str>, cx: &mut App) {
        let Some(proxy) = self.language_model_provider_proxy.read().clone() else {
            return;
        };

        proxy.unregister_language_model_provider(provider_id, cx)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionEditorState {
    pub path: String,
    pub language: String,
    pub text: String,
    pub selections: Vec<std::ops::Range<u32>>,
    pub version: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionTextEdit {
    pub start: u32,
    pub end: u32,
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtensionDecorationStyle {
    Info,
    Success,
    Error,
    Warning,
    Muted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtensionDecoration {
    InlineAfter {
        range: std::ops::Range<u32>,
        text: String,
        style: ExtensionDecorationStyle,
    },
    Block {
        row: u32,
        text: String,
        style: ExtensionDecorationStyle,
    },
    Highlight {
        range: std::ops::Range<u32>,
        style: ExtensionDecorationStyle,
    },
    Gutter {
        row: u32,
        style: ExtensionDecorationStyle,
    },
}

pub trait ExtensionEditorProxy: Send + Sync + 'static {
    fn active_editor(&self, cx: &mut App) -> Result<Option<ExtensionEditorState>>;
    fn apply_edits(&self, path: &str, edits: Vec<ExtensionTextEdit>, cx: &mut App) -> Result<()>;
    fn set_selections(
        &self,
        path: &str,
        selections: Vec<std::ops::Range<u32>>,
        cx: &mut App,
    ) -> Result<()>;
    fn set_decorations(
        &self,
        path: &str,
        key: &str,
        decorations: Vec<ExtensionDecoration>,
        cx: &mut App,
    ) -> Result<()>;
    fn clear_decorations(&self, path: Option<&str>, key: &str, cx: &mut App) -> Result<()>;
}

impl ExtensionEditorProxy for ExtensionHostProxy {
    fn active_editor(&self, cx: &mut App) -> Result<Option<ExtensionEditorState>> {
        let Some(proxy) = self.editor_proxy.read().clone() else {
            return Ok(None);
        };
        proxy.active_editor(cx)
    }

    fn apply_edits(&self, path: &str, edits: Vec<ExtensionTextEdit>, cx: &mut App) -> Result<()> {
        let Some(proxy) = self.editor_proxy.read().clone() else {
            return Err(anyhow!("editor is not available"));
        };
        proxy.apply_edits(path, edits, cx)
    }

    fn set_selections(
        &self,
        path: &str,
        selections: Vec<std::ops::Range<u32>>,
        cx: &mut App,
    ) -> Result<()> {
        let Some(proxy) = self.editor_proxy.read().clone() else {
            return Err(anyhow!("editor is not available"));
        };
        proxy.set_selections(path, selections, cx)
    }

    fn set_decorations(
        &self,
        path: &str,
        key: &str,
        decorations: Vec<ExtensionDecoration>,
        cx: &mut App,
    ) -> Result<()> {
        let Some(proxy) = self.editor_proxy.read().clone() else {
            return Err(anyhow!("editor is not available"));
        };
        proxy.set_decorations(path, key, decorations, cx)
    }

    fn clear_decorations(&self, path: Option<&str>, key: &str, cx: &mut App) -> Result<()> {
        let Some(proxy) = self.editor_proxy.read().clone() else {
            return Err(anyhow!("editor is not available"));
        };
        proxy.clear_decorations(path, key, cx)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtensionNotificationLevel {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionQuickPickItem {
    pub label: String,
    pub detail: Option<String>,
}

pub trait ExtensionUiProxy: Send + Sync + 'static {
    fn notify(&self, level: ExtensionNotificationLevel, message: &str, cx: &mut App) -> Result<()>;
    fn quick_pick(
        &self,
        extension_id: Arc<str>,
        request_id: u64,
        items: Vec<ExtensionQuickPickItem>,
        placeholder: Option<String>,
        cx: &mut App,
    ) -> Result<()>;
    fn input_box(
        &self,
        extension_id: Arc<str>,
        request_id: u64,
        prompt: &str,
        default_value: Option<String>,
        cx: &mut App,
    ) -> Result<()>;
    fn set_status(
        &self,
        extension_id: Arc<str>,
        id: &str,
        text: Option<String>,
        cx: &mut App,
    ) -> Result<()>;
    fn write_clipboard(&self, text: &str, cx: &mut App) -> Result<()>;
    fn open_path(
        &self,
        path: &str,
        row: Option<u32>,
        column: Option<u32>,
        cx: &mut App,
    ) -> Result<()>;
}

impl ExtensionUiProxy for ExtensionHostProxy {
    fn notify(&self, level: ExtensionNotificationLevel, message: &str, cx: &mut App) -> Result<()> {
        let Some(proxy) = self.ui_proxy.read().clone() else {
            return Err(anyhow!("ui is not available"));
        };
        proxy.notify(level, message, cx)
    }

    fn quick_pick(
        &self,
        extension_id: Arc<str>,
        request_id: u64,
        items: Vec<ExtensionQuickPickItem>,
        placeholder: Option<String>,
        cx: &mut App,
    ) -> Result<()> {
        let Some(proxy) = self.ui_proxy.read().clone() else {
            return Err(anyhow!("ui is not available"));
        };
        proxy.quick_pick(extension_id, request_id, items, placeholder, cx)
    }

    fn input_box(
        &self,
        extension_id: Arc<str>,
        request_id: u64,
        prompt: &str,
        default_value: Option<String>,
        cx: &mut App,
    ) -> Result<()> {
        let Some(proxy) = self.ui_proxy.read().clone() else {
            return Err(anyhow!("ui is not available"));
        };
        proxy.input_box(extension_id, request_id, prompt, default_value, cx)
    }

    fn set_status(
        &self,
        extension_id: Arc<str>,
        id: &str,
        text: Option<String>,
        cx: &mut App,
    ) -> Result<()> {
        let Some(proxy) = self.ui_proxy.read().clone() else {
            return Err(anyhow!("ui is not available"));
        };
        proxy.set_status(extension_id, id, text, cx)
    }

    fn write_clipboard(&self, text: &str, cx: &mut App) -> Result<()> {
        let Some(proxy) = self.ui_proxy.read().clone() else {
            return Err(anyhow!("ui is not available"));
        };
        proxy.write_clipboard(text, cx)
    }

    fn open_path(
        &self,
        path: &str,
        row: Option<u32>,
        column: Option<u32>,
        cx: &mut App,
    ) -> Result<()> {
        let Some(proxy) = self.ui_proxy.read().clone() else {
            return Err(anyhow!("ui is not available"));
        };
        proxy.open_path(path, row, column, cx)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtensionConsoleOutputKind {
    Output,
    Error,
    Result,
    Input,
}

pub trait ExtensionConsoleProxy: Send + Sync + 'static {
    fn open_console(
        &self,
        extension_id: Arc<str>,
        id: &str,
        title: &str,
        cx: &mut App,
    ) -> Result<()>;
    fn append(
        &self,
        extension_id: Arc<str>,
        id: &str,
        text: &str,
        kind: ExtensionConsoleOutputKind,
        cx: &mut App,
    ) -> Result<()>;
    fn clear(&self, extension_id: Arc<str>, id: &str, cx: &mut App) -> Result<()>;
    fn set_prompt(&self, extension_id: Arc<str>, id: &str, text: &str, cx: &mut App) -> Result<()>;
}

impl ExtensionConsoleProxy for ExtensionHostProxy {
    fn open_console(
        &self,
        extension_id: Arc<str>,
        id: &str,
        title: &str,
        cx: &mut App,
    ) -> Result<()> {
        let Some(proxy) = self.console_proxy.read().clone() else {
            return Err(anyhow!("console is not available"));
        };
        proxy.open_console(extension_id, id, title, cx)
    }

    fn append(
        &self,
        extension_id: Arc<str>,
        id: &str,
        text: &str,
        kind: ExtensionConsoleOutputKind,
        cx: &mut App,
    ) -> Result<()> {
        let Some(proxy) = self.console_proxy.read().clone() else {
            return Err(anyhow!("console is not available"));
        };
        proxy.append(extension_id, id, text, kind, cx)
    }

    fn clear(&self, extension_id: Arc<str>, id: &str, cx: &mut App) -> Result<()> {
        let Some(proxy) = self.console_proxy.read().clone() else {
            return Err(anyhow!("console is not available"));
        };
        proxy.clear(extension_id, id, cx)
    }

    fn set_prompt(&self, extension_id: Arc<str>, id: &str, text: &str, cx: &mut App) -> Result<()> {
        let Some(proxy) = self.console_proxy.read().clone() else {
            return Err(anyhow!("console is not available"));
        };
        proxy.set_prompt(extension_id, id, text, cx)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionCommandContext {
    pub editor: Option<ExtensionEditorState>,
}

pub trait ExtensionCommandDispatcher: Send + Sync + 'static {
    fn run_command(
        &self,
        extension_id: Arc<str>,
        command_id: &str,
        context: ExtensionCommandContext,
        cx: &mut App,
    ) -> Task<Result<()>>;
}

impl ExtensionCommandDispatcher for ExtensionHostProxy {
    fn run_command(
        &self,
        extension_id: Arc<str>,
        command_id: &str,
        context: ExtensionCommandContext,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let Some(dispatcher) = self.command_dispatcher.read().clone() else {
            return Task::ready(Err(anyhow!("command dispatcher is not available")));
        };
        dispatcher.run_command(extension_id, command_id, context, cx)
    }
}

pub trait ExtensionUiResponseDispatcher: Send + Sync + 'static {
    fn ui_response(
        &self,
        extension_id: Arc<str>,
        request_id: u64,
        selected: Option<String>,
        cx: &mut App,
    ) -> Task<Result<()>>;
}

impl ExtensionUiResponseDispatcher for ExtensionHostProxy {
    fn ui_response(
        &self,
        extension_id: Arc<str>,
        request_id: u64,
        selected: Option<String>,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let Some(dispatcher) = self.ui_response_dispatcher.read().clone() else {
            return Task::ready(Err(anyhow!("ui response dispatcher is not available")));
        };
        dispatcher.ui_response(extension_id, request_id, selected, cx)
    }
}

pub trait ExtensionConsoleInputDispatcher: Send + Sync + 'static {
    fn console_input(
        &self,
        extension_id: Arc<str>,
        id: &str,
        text: &str,
        cx: &mut App,
    ) -> Task<Result<()>>;
}

impl ExtensionConsoleInputDispatcher for ExtensionHostProxy {
    fn console_input(
        &self,
        extension_id: Arc<str>,
        id: &str,
        text: &str,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let Some(dispatcher) = self.console_input_dispatcher.read().clone() else {
            return Task::ready(Err(anyhow!("console input dispatcher is not available")));
        };
        dispatcher.console_input(extension_id, id, text, cx)
    }
}

pub trait ExtensionEventsDispatcher: Send + Sync + 'static {
    fn dispatch_event(&self, kind: &str, payload_json: &str, cx: &mut App) -> Task<Result<()>>;
}

impl ExtensionEventsDispatcher for ExtensionHostProxy {
    fn dispatch_event(&self, kind: &str, payload_json: &str, cx: &mut App) -> Task<Result<()>> {
        let Some(dispatcher) = self.events_dispatcher.read().clone() else {
            return Task::ready(Ok(()));
        };
        dispatcher.dispatch_event(kind, payload_json, cx)
    }
}

pub trait ExtensionProcessDispatcher: Send + Sync + 'static {
    fn process_output(
        &self,
        extension_id: Arc<str>,
        handle: u64,
        is_stderr: bool,
        text: String,
        cx: &mut App,
    ) -> Task<Result<()>>;
    fn process_exit(
        &self,
        extension_id: Arc<str>,
        handle: u64,
        code: Option<i32>,
        cx: &mut App,
    ) -> Task<Result<()>>;
}

impl ExtensionProcessDispatcher for ExtensionHostProxy {
    fn process_output(
        &self,
        extension_id: Arc<str>,
        handle: u64,
        is_stderr: bool,
        text: String,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let Some(dispatcher) = self.process_dispatcher.read().clone() else {
            return Task::ready(Ok(()));
        };
        dispatcher.process_output(extension_id, handle, is_stderr, text, cx)
    }

    fn process_exit(
        &self,
        extension_id: Arc<str>,
        handle: u64,
        code: Option<i32>,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let Some(dispatcher) = self.process_dispatcher.read().clone() else {
            return Task::ready(Ok(()));
        };
        dispatcher.process_exit(extension_id, handle, code, cx)
    }
}

pub trait ExtensionTcpDispatcher: Send + Sync + 'static {
    fn tcp_data(
        &self,
        extension_id: Arc<str>,
        stream_id: u64,
        data: Vec<u8>,
        cx: &mut App,
    ) -> Task<Result<()>>;
    fn tcp_closed(
        &self,
        extension_id: Arc<str>,
        stream_id: u64,
        error: Option<String>,
        cx: &mut App,
    ) -> Task<Result<()>>;
}

impl ExtensionTcpDispatcher for ExtensionHostProxy {
    fn tcp_data(
        &self,
        extension_id: Arc<str>,
        stream_id: u64,
        data: Vec<u8>,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let Some(dispatcher) = self.tcp_dispatcher.read().clone() else {
            return Task::ready(Ok(()));
        };
        dispatcher.tcp_data(extension_id, stream_id, data, cx)
    }

    fn tcp_closed(
        &self,
        extension_id: Arc<str>,
        stream_id: u64,
        error: Option<String>,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let Some(dispatcher) = self.tcp_dispatcher.read().clone() else {
            return Task::ready(Ok(()));
        };
        dispatcher.tcp_closed(extension_id, stream_id, error, cx)
    }
}
