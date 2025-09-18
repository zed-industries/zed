use gpui::{Action, actions};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// If the zed binary doesn't use anything in this crate, it will be optimized away
// and the actions won't initialize. So we just provide an empty initialization function
// to be called from main.
//
// These may provide relevant context:
// https://github.com/rust-lang/rust/issues/47384
// https://github.com/mmastrac/rust-ctor/issues/280
pub fn init() {}

/// Opens a URL in the system's default web browser.
#[derive(Clone, PartialEq, Deserialize, JsonSchema, Action)]
#[action(namespace = zed)]
#[serde(deny_unknown_fields)]
pub struct OpenBrowser {
    pub url: String,
}

/// Opens a zed:// URL within the application.
#[derive(Clone, PartialEq, Deserialize, JsonSchema, Action)]
#[action(namespace = zed)]
#[serde(deny_unknown_fields)]
pub struct OpenZedUrl {
    pub url: String,
}

actions!(
    zed,
    [
        /// Opens the settings editor.
        OpenSettings,
        /// Opens the default keymap file.
        OpenDefaultKeymap,
        /// Opens account settings.
        OpenAccountSettings,
        /// Opens the keymap editor.
        OpenKeymapEditor,
        /// Opens server settings.
        OpenServerSettings,
        /// Quits the application.
        Quit,
        /// Opens the user keymap file.
        OpenKeymap,
        /// Shows information about Zed.
        About,
        /// Opens the documentation website.
        OpenDocs,
        /// Views open source licenses.
        OpenLicenses,
        /// Opens the telemetry log.
        OpenTelemetryLog,
    ]
);

#[derive(PartialEq, Clone, Copy, Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionCategoryFilter {
    Themes,
    IconThemes,
    Languages,
    Grammars,
    LanguageServers,
    ContextServers,
    SlashCommands,
    IndexedDocsProviders,
    Snippets,
    DebugAdapters,
}

/// Opens the extensions management interface.
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = zed)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
    /// Filters the extensions page down to extensions that are in the specified category.
    #[serde(default)]
    pub category_filter: Option<ExtensionCategoryFilter>,
    /// Focuses just the extension with the specified ID.
    #[serde(default)]
    pub id: Option<String>,
}

/// Decreases the font size in the editor buffer.
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = zed)]
#[serde(deny_unknown_fields)]
pub struct DecreaseBufferFontSize {
    #[serde(default)]
    pub persist: bool,
}

/// Increases the font size in the editor buffer.
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = zed)]
#[serde(deny_unknown_fields)]
pub struct IncreaseBufferFontSize {
    #[serde(default)]
    pub persist: bool,
}

/// Resets the buffer font size to the default value.
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = zed)]
#[serde(deny_unknown_fields)]
pub struct ResetBufferFontSize {
    #[serde(default)]
    pub persist: bool,
}

/// Decreases the font size of the user interface.
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = zed)]
#[serde(deny_unknown_fields)]
pub struct DecreaseUiFontSize {
    #[serde(default)]
    pub persist: bool,
}

/// Increases the font size of the user interface.
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = zed)]
#[serde(deny_unknown_fields)]
pub struct IncreaseUiFontSize {
    #[serde(default)]
    pub persist: bool,
}

/// Resets the UI font size to the default value.
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = zed)]
#[serde(deny_unknown_fields)]
pub struct ResetUiFontSize {
    #[serde(default)]
    pub persist: bool,
}

pub mod dev {
    use gpui::actions;

    actions!(
        dev,
        [
            /// Toggles the developer inspector for debugging UI elements.
            ToggleInspector
        ]
    );
}

pub mod workspace {
    use gpui::actions;

    actions!(
        workspace,
        [
            #[action(deprecated_aliases = ["editor::CopyPath", "outline_panel::CopyPath", "project_panel::CopyPath"])]
            CopyPath,
            #[action(deprecated_aliases = ["editor::CopyRelativePath", "outline_panel::CopyRelativePath", "project_panel::CopyRelativePath"])]
            CopyRelativePath,
            /// Opens the selected file with the system's default application.
            #[action(deprecated_aliases = ["project_panel::OpenWithSystem"])]
            OpenWithSystem,
        ]
    );
}

pub mod git {
    use gpui::actions;

    actions!(
        git,
        [
            /// Checks out a different git branch.
            CheckoutBranch,
            /// Switches to a different git branch.
            Switch,
            /// Selects a different repository.
            SelectRepo,
            /// Opens the git branch selector.
            #[action(deprecated_aliases = ["branches::OpenRecent"])]
            Branch,
            /// Opens the git stash selector.
            ViewStash
        ]
    );
}

pub mod jj {
    use gpui::actions;

    actions!(
        jj,
        [
            /// Opens the Jujutsu bookmark list.
            BookmarkList
        ]
    );
}

pub mod toast {
    use gpui::actions;

    actions!(
        toast,
        [
            /// Runs the action associated with a toast notification.
            RunAction
        ]
    );
}

pub mod command_palette {
    use gpui::actions;

    actions!(
        command_palette,
        [
            /// Toggles the command palette.
            Toggle
        ]
    );
}

pub mod feedback {
    use gpui::actions;

    actions!(
        feedback,
        [
            /// Opens the bug report form.
            FileBugReport,
            /// Opens the feedback form.
            GiveFeedback
        ]
    );
}

pub mod theme_selector {
    use gpui::Action;
    use schemars::JsonSchema;
    use serde::Deserialize;

    /// Toggles the theme selector interface.
    #[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
    #[action(namespace = theme_selector)]
    #[serde(deny_unknown_fields)]
    pub struct Toggle {
        /// A list of theme names to filter the theme selector down to.
        pub themes_filter: Option<Vec<String>>,
    }
}

pub mod icon_theme_selector {
    use gpui::Action;
    use schemars::JsonSchema;
    use serde::Deserialize;

    /// Toggles the icon theme selector interface.
    #[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
    #[action(namespace = icon_theme_selector)]
    #[serde(deny_unknown_fields)]
    pub struct Toggle {
        /// A list of icon theme names to filter the theme selector down to.
        pub themes_filter: Option<Vec<String>>,
    }
}

pub mod settings_profile_selector {
    use gpui::Action;
    use schemars::JsonSchema;
    use serde::Deserialize;

    #[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
    #[action(namespace = settings_profile_selector)]
    pub struct Toggle;
}

pub mod agent {
    use gpui::actions;

    actions!(
        agent,
        [
            /// Opens the agent settings panel.
            #[action(deprecated_aliases = ["agent::OpenConfiguration"])]
            OpenSettings,
            /// Opens the agent onboarding modal.
            OpenOnboardingModal,
            /// Opens the ACP onboarding modal.
            OpenAcpOnboardingModal,
            /// Opens the Claude Code onboarding modal.
            OpenClaudeCodeOnboardingModal,
            /// Resets the agent onboarding state.
            ResetOnboarding,
            /// Starts a chat conversation with the agent.
            Chat,
            /// Toggles the language model selector dropdown.
            #[action(deprecated_aliases = ["assistant::ToggleModelSelector", "assistant2::ToggleModelSelector"])]
            ToggleModelSelector,
            /// Triggers re-authentication on Gemini
            ReauthenticateAgent
        ]
    );
}

pub mod assistant {
    use gpui::{Action, actions};
    use schemars::JsonSchema;
    use serde::Deserialize;
    use uuid::Uuid;

    actions!(
        agent,
        [
            #[action(deprecated_aliases = ["assistant::ToggleFocus"])]
            ToggleFocus
        ]
    );

    actions!(
        assistant,
        [
            /// Shows the assistant configuration panel.
            ShowConfiguration
        ]
    );

    /// Opens the rules library for managing agent rules and prompts.
    #[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
    #[action(namespace = agent, deprecated_aliases = ["assistant::OpenRulesLibrary", "assistant::DeployPromptLibrary"])]
    #[serde(deny_unknown_fields)]
    pub struct OpenRulesLibrary {
        #[serde(skip)]
        pub prompt_to_select: Option<Uuid>,
    }

    /// Deploys the assistant interface with the specified configuration.
    #[derive(Clone, Default, Deserialize, PartialEq, JsonSchema, Action)]
    #[action(namespace = assistant)]
    #[serde(deny_unknown_fields)]
    pub struct InlineAssist {
        pub prompt: Option<String>,
    }
}

pub mod debugger {
    use gpui::actions;

    actions!(
        debugger,
        [
            /// Opens the debugger onboarding modal.
            OpenOnboardingModal,
            /// Resets the debugger onboarding state.
            ResetOnboarding
        ]
    );
}

/// Opens the recent projects interface.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = projects)]
#[serde(deny_unknown_fields)]
pub struct OpenRecent {
    #[serde(default)]
    pub create_new_window: bool,
}

/// Creates a project from a selected template.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = projects)]
#[serde(deny_unknown_fields)]
pub struct OpenRemote {
    #[serde(default)]
    pub from_existing_connection: bool,
    #[serde(default)]
    pub create_new_window: bool,
}

/// Where to spawn the task in the UI.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RevealTarget {
    /// In the central pane group, "main" editor area.
    Center,
    /// In the terminal dock, "regular" terminal items' place.
    #[default]
    Dock,
}

/// Spawns a task with name or opens tasks modal.
#[derive(Debug, PartialEq, Clone, Deserialize, JsonSchema, Action)]
#[action(namespace = task)]
#[serde(untagged)]
pub enum Spawn {
    /// Spawns a task by the name given.
    ByName {
        task_name: String,
        #[serde(default)]
        reveal_target: Option<RevealTarget>,
    },
    /// Spawns a task by the name given.
    ByTag {
        task_tag: String,
        #[serde(default)]
        reveal_target: Option<RevealTarget>,
    },
    /// Spawns a task via modal's selection.
    ViaModal {
        /// Selected task's `reveal_target` property override.
        #[serde(default)]
        reveal_target: Option<RevealTarget>,
    },
}

impl Spawn {
    pub fn modal() -> Self {
        Self::ViaModal {
            reveal_target: None,
        }
    }
}

/// Reruns the last task.
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = task)]
#[serde(deny_unknown_fields)]
pub struct Rerun {
    /// Controls whether the task context is reevaluated prior to execution of a task.
    /// If it is not, environment variables such as ZED_COLUMN, ZED_FILE are gonna be the same as in the last execution of a task
    /// If it is, these variables will be updated to reflect current state of editor at the time task::Rerun is executed.
    /// default: false
    #[serde(default)]
    pub reevaluate_context: bool,
    /// Overrides `allow_concurrent_runs` property of the task being reran.
    /// Default: null
    #[serde(default)]
    pub allow_concurrent_runs: Option<bool>,
    /// Overrides `use_new_terminal` property of the task being reran.
    /// Default: null
    #[serde(default)]
    pub use_new_terminal: Option<bool>,

    /// If present, rerun the task with this ID, otherwise rerun the last task.
    #[serde(skip)]
    pub task_id: Option<String>,
}

pub mod outline {
    use std::sync::OnceLock;

    use gpui::{AnyView, App, Window, actions};

    actions!(
        outline,
        [
            #[action(name = "Toggle")]
            ToggleOutline
        ]
    );
    /// A pointer to outline::toggle function, exposed here to sewer the breadcrumbs <-> outline dependency.
    pub static TOGGLE_OUTLINE: OnceLock<fn(AnyView, &mut Window, &mut App)> = OnceLock::new();
}

actions!(
    zed_predict_onboarding,
    [
        /// Opens the Zed Predict onboarding modal.
        OpenZedPredictOnboarding
    ]
);
actions!(
    git_onboarding,
    [
        /// Opens the git integration onboarding modal.
        OpenGitIntegrationOnboarding
    ]
);

actions!(
    debug_panel,
    [
        /// Toggles focus on the debug panel.
        ToggleFocus
    ]
);
actions!(
    debugger,
    [
        /// Toggles the enabled state of a breakpoint.
        ToggleEnableBreakpoint,
        /// Removes a breakpoint.
        UnsetBreakpoint,
        /// Opens the project debug tasks configuration.
        OpenProjectDebugTasks,
    ]
);

#[cfg(target_os = "windows")]
pub mod wsl_actions {
    use gpui::Action;
    use schemars::JsonSchema;
    use serde::Deserialize;

    /// Opens a folder inside Wsl.
    #[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
    #[action(namespace = projects)]
    #[serde(deny_unknown_fields)]
    pub struct OpenFolderInWsl {
        #[serde(default)]
        pub create_new_window: bool,
    }

    /// Open a wsl distro.
    #[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
    #[action(namespace = projects)]
    #[serde(deny_unknown_fields)]
    pub struct OpenWsl {
        #[serde(default)]
        pub create_new_window: bool,
    }
}
