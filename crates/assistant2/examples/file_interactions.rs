//! This example creates a basic Chat UI for interacting with the filesystem.

use anyhow::{Context as _, Result};
use assets::Assets;
use assistant2::AssistantPanel;
use assistant_tooling::{LanguageModelTool, ToolRegistry};
use client::Client;
use fs::Fs;
use futures::StreamExt;
use gpui::{actions, App, AppContext, KeyBinding, Task, View, WindowOptions};
use language::LanguageRegistry;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{KeymapFile, DEFAULT_KEYMAP_PATH};
use std::path::PathBuf;
use std::sync::Arc;
use theme::LoadThemes;
use ui::{div, prelude::*, Render};
use util::ResultExt as _;

actions!(example, [Quit]);

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

    fn execute(&self, input: &Self::Input, cx: &AppContext) -> Task<gpui::Result<Self::Output>> {
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

    fn new_view(
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
        let node_runtime = node_runtime::RealNodeRuntime::new(client.http_client());
        languages::init(language_registry.clone(), node_runtime, cx);

        cx.spawn(|cx| async move {
            cx.update(|cx| {
                let fs = Arc::new(fs::RealFs::new(None));
                let cwd = std::env::current_dir().expect("Failed to get current working directory");

                let mut tool_registry = ToolRegistry::new();
                tool_registry
                    .register(FileBrowserTool::new(fs, cwd))
                    .context("failed to register FileBrowserTool")
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
