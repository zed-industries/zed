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

#[derive(Clone, PartialEq, Deserialize, JsonSchema, Action)]
#[action(namespace = zed)]
#[serde(deny_unknown_fields)]
pub struct OpenBrowser {
    pub url: String,
}

#[derive(Clone, PartialEq, Deserialize, JsonSchema, Action)]
#[action(namespace = zed)]
#[serde(deny_unknown_fields)]
pub struct OpenZedUrl {
    pub url: String,
}

actions!(
    zed,
    [
        /// Open the settings editor
        OpenSettings,
        /// Open the default keymap file
        OpenDefaultKeymap,
        /// Open account settings
        OpenAccountSettings,
        /// Open server settings
        OpenServerSettings,
        /// Quit the application
        Quit,
        /// Open the user keymap file
        OpenKeymap,
        /// Show information about Zed
        About,
        /// Open the documentation website
        OpenDocs,
        /// View open source licenses
        OpenLicenses,
        /// Open the telemetry log
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

#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = zed)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
    /// Filters the extensions page down to extensions that are in the specified category.
    #[serde(default)]
    pub category_filter: Option<ExtensionCategoryFilter>,
}

#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = zed)]
#[serde(deny_unknown_fields)]
pub struct DecreaseBufferFontSize {
    #[serde(default)]
    pub persist: bool,
}

#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = zed)]
#[serde(deny_unknown_fields)]
pub struct IncreaseBufferFontSize {
    #[serde(default)]
    pub persist: bool,
}

#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = zed)]
#[serde(deny_unknown_fields)]
pub struct ResetBufferFontSize {
    #[serde(default)]
    pub persist: bool,
}

#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = zed)]
#[serde(deny_unknown_fields)]
pub struct DecreaseUiFontSize {
    #[serde(default)]
    pub persist: bool,
}

#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = zed)]
#[serde(deny_unknown_fields)]
pub struct IncreaseUiFontSize {
    #[serde(default)]
    pub persist: bool,
}

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
            /// Toggle the developer inspector for debugging UI elements
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
            CopyRelativePath
        ]
    );
}

pub mod git {
    use gpui::actions;

    actions!(
        git,
        [
            /// Checkout a different git branch
            CheckoutBranch,
            /// Switch to a different git branch
            Switch,
            /// Select a different repository
            SelectRepo,
            /// Open the git branch selector
            #[action(deprecated_aliases = ["branches::OpenRecent"])]
            Branch
        ]
    );
}

pub mod jj {
    use gpui::actions;

    actions!(
        jj,
        [
            /// Open the Jujutsu bookmark list
            BookmarkList
        ]
    );
}

pub mod toast {
    use gpui::actions;

    actions!(
        toast,
        [
            /// Run the action associated with a toast notification
            RunAction
        ]
    );
}

pub mod command_palette {
    use gpui::actions;

    actions!(
        command_palette,
        [
            /// Toggle the command palette
            Toggle
        ]
    );
}

pub mod feedback {
    use gpui::actions;

    actions!(
        feedback,
        [
            /// Open the bug report form
            FileBugReport,
            /// Open the feedback form
            GiveFeedback
        ]
    );
}

pub mod theme_selector {
    use gpui::Action;
    use schemars::JsonSchema;
    use serde::Deserialize;

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

    #[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
    #[action(namespace = icon_theme_selector)]
    #[serde(deny_unknown_fields)]
    pub struct Toggle {
        /// A list of icon theme names to filter the theme selector down to.
        pub themes_filter: Option<Vec<String>>,
    }
}

pub mod agent {
    use gpui::actions;

    actions!(
        agent,
        [
            /// Open the agent configuration panel
            OpenConfiguration,
            /// Open the agent onboarding modal
            OpenOnboardingModal,
            /// Reset the agent onboarding state
            ResetOnboarding
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
            /// Show the assistant configuration panel
            ShowConfiguration
        ]
    );

    #[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
    #[action(namespace = agent, deprecated_aliases = ["assistant::OpenRulesLibrary", "assistant::DeployPromptLibrary"])]
    #[serde(deny_unknown_fields)]
    pub struct OpenRulesLibrary {
        #[serde(skip)]
        pub prompt_to_select: Option<Uuid>,
    }

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
            /// Open the debugger onboarding modal
            OpenOnboardingModal,
            /// Reset the debugger onboarding state
            ResetOnboarding
        ]
    );
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = projects)]
#[serde(deny_unknown_fields)]
pub struct OpenRecent {
    #[serde(default)]
    pub create_new_window: bool,
}

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

/// Spawn a task with name or open tasks modal.
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

/// Rerun the last task.
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
        /// Open the Zed Predict onboarding modal
        OpenZedPredictOnboarding
    ]
);
actions!(
    git_onboarding,
    [
        /// Open the git integration onboarding modal
        OpenGitIntegrationOnboarding
    ]
);

actions!(
    debug_panel,
    [
        /// Toggle focus on the debug panel
        ToggleFocus
    ]
);
actions!(
    debugger,
    [
        /// Toggle the enabled state of a breakpoint
        ToggleEnableBreakpoint,
        /// Remove a breakpoint
        UnsetBreakpoint,
        /// Open the project debug tasks configuration
        OpenProjectDebugTasks,
    ]
);
