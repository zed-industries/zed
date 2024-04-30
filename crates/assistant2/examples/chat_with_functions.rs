//! This example creates a basic Chat UI with a function for rolling a die.

use anyhow::{Context as _, Result};
use assets::Assets;
use assistant2::AssistantPanel;
use assistant_tooling::{LanguageModelTool, ToolRegistry};
use client::{Client, UserStore};
use fs::Fs;
use futures::StreamExt as _;
use gpui::{actions, AnyElement, App, AppContext, KeyBinding, Model, Task, View, WindowOptions};
use language::LanguageRegistry;
use project::Project;
use rand::Rng;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{KeymapFile, DEFAULT_KEYMAP_PATH};
use std::{path::PathBuf, sync::Arc};
use theme::LoadThemes;
use ui::{div, prelude::*, Render};
use util::ResultExt as _;

actions!(example, [Quit]);

struct RollDiceTool {}

impl RollDiceTool {
    fn new() -> Self {
        Self {}
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "snake_case")]
enum Die {
    D6 = 6,
    D20 = 20,
}

impl Die {
    fn into_str(&self) -> &'static str {
        match self {
            Die::D6 => "d6",
            Die::D20 => "d20",
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
struct DiceParams {
    /// The number of dice to roll.
    num_dice: u8,
    /// Which die to roll. Defaults to a d6 if not provided.
    die_type: Option<Die>,
}

#[derive(Serialize, Deserialize)]
struct DieRoll {
    die: Die,
    roll: u8,
}

impl DieRoll {
    fn render(&self) -> AnyElement {
        match self.die {
            Die::D6 => {
                let face = match self.roll {
                    6 => div().child("⚅"),
                    5 => div().child("⚄"),
                    4 => div().child("⚃"),
                    3 => div().child("⚂"),
                    2 => div().child("⚁"),
                    1 => div().child("⚀"),
                    _ => div().child("😅"),
                };
                face.text_3xl().into_any_element()
            }
            _ => div()
                .child(format!("{}", self.roll))
                .text_3xl()
                .into_any_element(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct DiceRoll {
    rolls: Vec<DieRoll>,
}

pub struct DiceView {
    result: Result<DiceRoll>,
}

impl Render for DiceView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let output = match &self.result {
            Ok(output) => output,
            Err(_) => return "Somehow dice failed 🎲".into_any_element(),
        };

        h_flex()
            .children(
                output
                    .rolls
                    .iter()
                    .map(|roll| div().p_2().child(roll.render())),
            )
            .into_any_element()
    }
}

impl LanguageModelTool for RollDiceTool {
    type Input = DiceParams;
    type Output = DiceRoll;
    type View = DiceView;

    fn name(&self) -> String {
        "roll_dice".to_string()
    }

    fn description(&self) -> String {
        "Rolls N many dice and returns the results.".to_string()
    }

    fn execute(
        &self,
        input: &Self::Input,
        _cx: &mut WindowContext,
    ) -> Task<gpui::Result<Self::Output>> {
        let rolls = (0..input.num_dice)
            .map(|_| {
                let die_type = input.die_type.as_ref().unwrap_or(&Die::D6).clone();

                DieRoll {
                    die: die_type.clone(),
                    roll: rand::thread_rng().gen_range(1..=die_type as u8),
                }
            })
            .collect();

        return Task::ready(Ok(DiceRoll { rolls }));
    }

    fn output_view(
        _tool_call_id: String,
        _input: Self::Input,
        result: Result<Self::Output>,
        cx: &mut WindowContext,
    ) -> gpui::View<Self::View> {
        cx.new_view(|_cx| DiceView { result })
    }

    fn format(_: &Self::Input, output: &Result<Self::Output>) -> String {
        let output = match output {
            Ok(output) => output,
            Err(_) => return "Somehow dice failed 🎲".to_string(),
        };

        let mut result = String::new();
        for roll in &output.rolls {
            let die = &roll.die;
            result.push_str(&format!("{}: {}\n", die.into_str(), roll.roll));
        }
        result
    }
}

struct FileBrowserTool {
    fs: Arc<dyn Fs>,
    root_dir: PathBuf,
}

impl FileBrowserTool {
    fn new(fs: Arc<dyn Fs>, root_dir: PathBuf) -> Self {
        Self { fs, root_dir }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct FileBrowserParams {
    command: FileBrowserCommand,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
enum FileBrowserCommand {
    Ls { path: PathBuf },
    Cat { path: PathBuf },
}

#[derive(Serialize, Deserialize)]
enum FileBrowserOutput {
    Ls { entries: Vec<String> },
    Cat { content: String },
}

pub struct FileBrowserView {
    result: Result<FileBrowserOutput>,
}

impl Render for FileBrowserView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let Ok(output) = self.result.as_ref() else {
            return h_flex().child("Failed to perform operation");
        };

        match output {
            FileBrowserOutput::Ls { entries } => v_flex().children(
                entries
                    .into_iter()
                    .map(|entry| h_flex().text_ui(cx).child(entry.clone())),
            ),
            FileBrowserOutput::Cat { content } => h_flex().child(content.clone()),
        }
    }
}

impl LanguageModelTool for FileBrowserTool {
    type Input = FileBrowserParams;
    type Output = FileBrowserOutput;
    type View = FileBrowserView;

    fn name(&self) -> String {
        "file_browser".to_string()
    }

    fn description(&self) -> String {
        "A tool for browsing the filesystem.".to_string()
    }

    fn execute(
        &self,
        input: &Self::Input,
        cx: &mut WindowContext,
    ) -> Task<gpui::Result<Self::Output>> {
        cx.spawn({
            let fs = self.fs.clone();
            let root_dir = self.root_dir.clone();
            let input = input.clone();
            |_cx| async move {
                match input.command {
                    FileBrowserCommand::Ls { path } => {
                        let path = root_dir.join(path);

                        let mut output = fs.read_dir(&path).await?;

                        let mut entries = Vec::new();
                        while let Some(entry) = output.next().await {
                            let entry = entry?;
                            entries.push(entry.display().to_string());
                        }

                        Ok(FileBrowserOutput::Ls { entries })
                    }
                    FileBrowserCommand::Cat { path } => {
                        let path = root_dir.join(path);

                        let output = fs.load(&path).await?;

                        Ok(FileBrowserOutput::Cat { content: output })
                    }
                }
            }
        })
    }

    fn output_view(
        _tool_call_id: String,
        _input: Self::Input,
        result: Result<Self::Output>,
        cx: &mut WindowContext,
    ) -> gpui::View<Self::View> {
        cx.new_view(|_cx| FileBrowserView { result })
    }

    fn format(_input: &Self::Input, output: &Result<Self::Output>) -> String {
        let Ok(output) = output else {
            return "Failed to perform command: {input:?}".to_string();
        };

        match output {
            FileBrowserOutput::Ls { entries } => entries.join("\n"),
            FileBrowserOutput::Cat { content } => content.to_owned(),
        }
    }
}

fn main() {
    env_logger::init();
    App::new().with_assets(Assets).run(|cx| {
        cx.bind_keys(Some(KeyBinding::new("cmd-q", Quit, None)));
        cx.on_action(|_: &Quit, cx: &mut AppContext| {
            cx.quit();
        });

        settings::init(cx);
        language::init(cx);
        Project::init_settings(cx);
        editor::init(cx);
        theme::init(LoadThemes::JustBase, cx);
        Assets.load_fonts(cx).unwrap();
        KeymapFile::load_asset(DEFAULT_KEYMAP_PATH, cx).unwrap();
        client::init_settings(cx);
        release_channel::init("0.130.0", cx);

        let client = Client::production(cx);
        {
            let client = client.clone();
            cx.spawn(|cx| async move { client.authenticate_and_connect(false, &cx).await })
                .detach_and_log_err(cx);
        }
        assistant2::init(client.clone(), cx);

        let language_registry = Arc::new(LanguageRegistry::new(
            Task::ready(()),
            cx.background_executor().clone(),
        ));

        let user_store = cx.new_model(|cx| UserStore::new(client.clone(), cx));
        let node_runtime = node_runtime::RealNodeRuntime::new(client.http_client());
        languages::init(language_registry.clone(), node_runtime, cx);

        cx.spawn(|cx| async move {
            cx.update(|cx| {
                let fs = Arc::new(fs::RealFs::new(None));
                let cwd = std::env::current_dir().expect("Failed to get current working directory");

                cx.open_window(WindowOptions::default(), |cx| {
                    let mut tool_registry = ToolRegistry::new();
                    tool_registry
                        .register(RollDiceTool::new(), cx)
                        .context("failed to register DummyTool")
                        .log_err();

                    tool_registry
                        .register(FileBrowserTool::new(fs, cwd), cx)
                        .context("failed to register FileBrowserTool")
                        .log_err();

                    let tool_registry = Arc::new(tool_registry);

                    println!("Tools registered");
                    for definition in tool_registry.definitions() {
                        println!("{}", definition);
                    }

                    cx.new_view(|cx| Example::new(language_registry, tool_registry, user_store, cx))
                });
                cx.activate(true);
            })
        })
        .detach_and_log_err(cx);
    })
}

struct Example {
    assistant_panel: View<AssistantPanel>,
}

impl Example {
    fn new(
        language_registry: Arc<LanguageRegistry>,
        tool_registry: Arc<ToolRegistry>,
        user_store: Model<UserStore>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            assistant_panel: cx.new_view(|cx| {
                AssistantPanel::new(language_registry, tool_registry, user_store, None, cx)
            }),
        }
    }
}

impl Render for Example {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl ui::prelude::IntoElement {
        div().size_full().child(self.assistant_panel.clone())
    }
}
