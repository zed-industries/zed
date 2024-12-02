use gpui::{actions, impl_actions};
use serde::Deserialize;

// If the zed binary doesn't use anything in this crate, it will be optimized away
// and the actions won't initialize. So we just provide an empty initialization function
// to be called from main.
//
// These may provide relevant context:
// https://github.com/rust-lang/rust/issues/47384
// https://github.com/mmastrac/rust-ctor/issues/280
pub fn init() {}

#[derive(Clone, PartialEq, Deserialize)]
pub struct OpenBrowser {
    pub url: String,
}

#[derive(Clone, PartialEq, Deserialize)]
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
    use serde::Deserialize;

    #[derive(PartialEq, Clone, Default, Debug, Deserialize)]
    pub struct Toggle {
        /// A list of theme names to filter the theme selector down to.
        pub themes_filter: Option<Vec<String>>,
    }

    impl_actions!(theme_selector, [Toggle]);
}

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct InlineAssist {
    pub prompt: Option<String>,
}

impl_actions!(assistant, [InlineAssist]);

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct OpenRecent {
    #[serde(default)]
    pub create_new_window: bool,
}
gpui::impl_actions!(projects, [OpenRecent]);
gpui::actions!(projects, [OpenRemote]);

/// Spawn a task with name or open tasks modal
#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct Spawn {
    #[serde(default)]
    /// Name of the task to spawn.
    /// If it is not set, a modal with a list of available tasks is opened instead.
    /// Defaults to None.
    pub task_name: Option<String>,
}

impl Spawn {
    pub fn modal() -> Self {
        Self { task_name: None }
    }
}

/// Rerun last task
#[derive(PartialEq, Clone, Deserialize, Default)]
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
    pub task_id: Option<String>,
}

impl_actions!(task, [Spawn, Rerun]);
