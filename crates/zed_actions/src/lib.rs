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
pub struct OpenBrowser {
    pub url: String,
}

#[derive(Clone, PartialEq, Deserialize, JsonSchema)]
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
        Extensions,
        OpenLicenses,
        OpenTelemetryLog,
        DecreaseBufferFontSize,
        IncreaseBufferFontSize,
        ResetBufferFontSize,
        DecreaseUiFontSize,
        IncreaseUiFontSize,
        ResetUiFontSize
    ]
);

pub mod branches {
    use gpui::actions;

    actions!(branches, [OpenRecent]);
}

pub mod command_palette {
    use gpui::actions;

    actions!(command_palette, [Toggle]);
}

pub mod feedback {
    use gpui::actions;

    actions!(feedback, [GiveFeedback]);
}

pub mod theme_selector {
    use gpui::impl_actions;
    use schemars::JsonSchema;
    use serde::Deserialize;

    #[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema)]
    pub struct Toggle {
        /// A list of theme names to filter the theme selector down to.
        pub themes_filter: Option<Vec<String>>,
    }

    impl_actions!(theme_selector, [Toggle]);
}

#[derive(Clone, Default, Deserialize, PartialEq, JsonSchema)]
pub struct InlineAssist {
    pub prompt: Option<String>,
}

impl_actions!(assistant, [InlineAssist]);

#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema)]
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

    use gpui::{action_as, AnyView, WindowContext};

    action_as!(outline, ToggleOutline as Toggle);
    /// A pointer to outline::toggle function, exposed here to sewer the breadcrumbs <-> outline dependency.
    pub static TOGGLE_OUTLINE: OnceLock<fn(AnyView, &mut WindowContext<'_>)> = OnceLock::new();
}
