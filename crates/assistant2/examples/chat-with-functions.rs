/// This example creates a basic Chat UI with a function for rolling a die.
use anyhow::{Context as _, Result};
use assets::Assets;
use assistant2::AssistantPanel;
use assistant_tooling::{LanguageModelTool, ToolRegistry};
use client::Client;
use gpui::{actions, AnyElement, App, AppContext, KeyBinding, Task, View, WindowOptions};
use language::LanguageRegistry;
use project::Project;
use rand::Rng;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{KeymapFile, DEFAULT_KEYMAP_PATH};
use std::sync::Arc;
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

    fn execute(&self, input: &Self::Input, _cx: &AppContext) -> Task<gpui::Result<Self::Output>> {
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

    fn new_view(
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
        let node_runtime = node_runtime::RealNodeRuntime::new(client.http_client());
        languages::init(language_registry.clone(), node_runtime, cx);

        cx.spawn(|cx| async move {
            cx.update(|cx| {
                let mut tool_registry = ToolRegistry::new();
                tool_registry
                    .register(RollDiceTool::new())
                    .context("failed to register DummyTool")
                    .log_err();

                let tool_registry = Arc::new(tool_registry);

                println!("Tools registered");
                for definition in tool_registry.definitions() {
                    println!("{}", definition);
                }

                cx.open_window(WindowOptions::default(), |cx| {
                    cx.new_view(|cx| Example::new(language_registry, tool_registry, cx))
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
        cx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            assistant_panel: cx
                .new_view(|cx| AssistantPanel::new(language_registry, tool_registry, cx)),
        }
    }
}

impl Render for Example {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl ui::prelude::IntoElement {
        div().size_full().child(self.assistant_panel.clone())
    }
}
