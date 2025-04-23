use gpui::{actions, impl_actions};
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

#[derive(Clone, PartialEq, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct OpenBrowser {
    pub url: String,
}

#[derive(Clone, PartialEq, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct OpenZedUrl {
    pub url: String,
}

impl_actions!(zed, [OpenBrowser, OpenZedUrl]);

actions!(
    zed,
    [
        OpenSettings,
        OpenDefaultKeymap,
        OpenAccountSettings,
        OpenServerSettings,
        Quit,
        OpenKeymap,
        About,
        OpenLicenses,
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
}

#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema)]
pub struct Extensions {
    /// Filters the extensions page down to extensions that are in the specified category.
    #[serde(default)]
    pub category_filter: Option<ExtensionCategoryFilter>,
}

#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema)]
pub struct DecreaseBufferFontSize {
    #[serde(default)]
    pub persist: bool,
}

#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema)]
pub struct IncreaseBufferFontSize {
    #[serde(default)]
    pub persist: bool,
}

#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema)]
pub struct ResetBufferFontSize {
    #[serde(default)]
    pub persist: bool,
}

#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema)]
pub struct DecreaseUiFontSize {
    #[serde(default)]
    pub persist: bool,
}

#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema)]
pub struct IncreaseUiFontSize {
    #[serde(default)]
    pub persist: bool,
}

#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema)]
pub struct ResetUiFontSize {
    #[serde(default)]
    pub persist: bool,
}

impl_actions!(
    zed,
    [
        Extensions,
        DecreaseBufferFontSize,
        IncreaseBufferFontSize,
        ResetBufferFontSize,
        DecreaseUiFontSize,
        IncreaseUiFontSize,
        ResetUiFontSize,
    ]
);

pub mod workspace {
    use gpui::action_with_deprecated_aliases;

    action_with_deprecated_aliases!(
        workspace,
        CopyPath,
        [
            "editor::CopyPath",
            "outline_panel::CopyPath",
            "project_panel::CopyPath"
        ]
    );

    action_with_deprecated_aliases!(
        workspace,
        CopyRelativePath,
        [
            "editor::CopyRelativePath",
            "outline_panel::CopyRelativePath",
            "project_panel::CopyRelativePath"
        ]
    );
}

pub mod git {
    use gpui::{action_with_deprecated_aliases, actions};

    actions!(git, [CheckoutBranch, Switch, SelectRepo]);
    action_with_deprecated_aliases!(git, Branch, ["branches::OpenRecent"]);
}

pub mod command_palette {
    use gpui::actions;

    actions!(command_palette, [Toggle]);
}

pub mod feedback {
    use gpui::actions;

    actions!(feedback, [FileBugReport, GiveFeedback]);
}

pub mod theme_selector {
    use gpui::impl_actions;
    use schemars::JsonSchema;
    use serde::Deserialize;

    #[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct Toggle {
        /// A list of theme names to filter the theme selector down to.
        pub themes_filter: Option<Vec<String>>,
    }

    impl_actions!(theme_selector, [Toggle]);
}

pub mod icon_theme_selector {
    use gpui::impl_actions;
    use schemars::JsonSchema;
    use serde::Deserialize;

    #[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct Toggle {
        /// A list of icon theme names to filter the theme selector down to.
        pub themes_filter: Option<Vec<String>>,
    }

    impl_actions!(icon_theme_selector, [Toggle]);
}

pub mod agent {
    use gpui::actions;

    actions!(agent, [OpenConfiguration]);
}

pub mod assistant {
    use gpui::{actions, impl_action_with_deprecated_aliases, impl_actions};
    use schemars::JsonSchema;
    use serde::Deserialize;
    use uuid::Uuid;

    actions!(assistant, [ToggleFocus, ShowConfiguration]);

    #[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct OpenPromptLibrary {
        #[serde(skip)]
        pub prompt_to_select: Option<Uuid>,
    }

    impl_action_with_deprecated_aliases!(
        assistant,
        OpenPromptLibrary,
        ["assistant::DeployPromptLibrary"]
    );

    #[derive(Clone, Default, Deserialize, PartialEq, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct InlineAssist {
        pub prompt: Option<String>,
    }

    impl_actions!(assistant, [InlineAssist]);
}

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct OpenRecent {
    #[serde(default)]
    pub create_new_window: bool,
}

impl_actions!(projects, [OpenRecent]);
actions!(projects, [OpenRemote]);

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
#[derive(Debug, PartialEq, Clone, Deserialize, JsonSchema)]
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
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
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

impl_actions!(task, [Spawn, Rerun]);

pub mod outline {
    use std::sync::OnceLock;

    use gpui::{AnyView, App, Window, action_as};

    action_as!(outline, ToggleOutline as Toggle);
    /// A pointer to outline::toggle function, exposed here to sewer the breadcrumbs <-> outline dependency.
    pub static TOGGLE_OUTLINE: OnceLock<fn(AnyView, &mut Window, &mut App)> = OnceLock::new();
}

actions!(zed_predict_onboarding, [OpenZedPredictOnboarding]);
actions!(git_onboarding, [OpenGitIntegrationOnboarding]);
