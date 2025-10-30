use agent_settings::AgentSettings;
use anyhow::Result;
use cloud_llm_client::CompletionIntent;
use command_palette::humanize_action_name;
use futures::StreamExt as _;
use gpui::{
    Action, AppContext as _, DismissEvent, Entity, EventEmitter, Focusable, IntoElement, Task,
    WeakEntity,
};
use language_model::{
    ConfiguredModel, LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage, Role,
};
use picker::{Picker, PickerDelegate};
use settings::Settings as _;
use ui::{
    App, Context, InteractiveElement, ListItem, ParentElement as _, Render, Styled as _, Window,
    div, rems,
};
use util::ResultExt;
use workspace::{ModalView, Workspace};

pub fn init(cx: &mut App) {
    cx.observe_new(MagicPalette::register).detach();
}

gpui::actions!(magic_palette, [Toggle]);

struct MagicPalette {
    picker: Entity<Picker<MagicPaletteDelegate>>,
    matches: Vec<Command>,
}

impl ModalView for MagicPalette {}

impl EventEmitter<DismissEvent> for MagicPalette {}

impl Focusable for MagicPalette {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl MagicPalette {
    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _cx: &mut Context<Workspace>,
    ) {
        workspace.register_action(|workspace, _: &Toggle, window, cx| {
            Self::toggle(workspace, window, cx)
        });
    }

    fn toggle(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
        if agent_settings::AgentSettings::get_global(cx).enabled(cx) {
            workspace.toggle_modal(window, cx, |window, cx| MagicPalette::new(window, cx));
        }
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let this = cx.weak_entity();
        let delegate = MagicPaletteDelegate::new(this);
        let picker = cx.new(|cx| {
            let picker = Picker::uniform_list(delegate, window, cx);
            picker
        });
        Self {
            picker,
            matches: vec![],
        }
    }
}

impl Render for MagicPalette {
    fn render(&mut self, _window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .key_context("MagicPalette")
            .w(rems(34.))
            .child(self.picker.clone())
    }
}

#[derive(Debug)]
struct Command {
    name: String,
    action: Box<dyn Action>,
}

enum MagicPaletteMode {
    WriteQuery,
    SelectResult(Vec<Command>),
}

struct MagicPaletteDelegate {
    query: String,
    llm_generation_task: Task<Result<()>>,
    magic_palette: WeakEntity<MagicPalette>,
    mode: MagicPaletteMode,
    selected_index: usize,
}

impl MagicPaletteDelegate {
    fn new(magic_palette: WeakEntity<MagicPalette>) -> Self {
        Self {
            query: String::new(),
            llm_generation_task: Task::ready(Ok(())),
            magic_palette,
            mode: MagicPaletteMode::WriteQuery,
            selected_index: 0,
        }
    }
}

impl PickerDelegate for MagicPaletteDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        match &self.mode {
            MagicPaletteMode::WriteQuery => 0,
            MagicPaletteMode::SelectResult(commands) => commands.len(),
        }
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<picker::Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut ui::App) -> std::sync::Arc<str> {
        "Ask Zed AI what actions you want to perform...".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        _cx: &mut Context<picker::Picker<Self>>,
    ) -> gpui::Task<()> {
        self.query = query;
        Task::ready(())
    }

    fn confirm(
        &mut self,
        _secondary: bool,
        window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) {
        match &self.mode {
            MagicPaletteMode::WriteQuery => {
                let Some(ConfiguredModel { provider, model }) =
                    LanguageModelRegistry::read_global(cx).commit_message_model()
                else {
                    return;
                };
                let temperature = AgentSettings::temperature_for_model(&model, cx);
                let query = self.query.clone();
                let actions = window.available_actions(cx);
                self.llm_generation_task = cx.spawn_in(window, async move |this, cx| {
                    if let Some(task) = cx.update(|_, cx| {
                        if !provider.is_authenticated(cx) {
                            Some(provider.authenticate(cx))
                        } else {
                            None
                        }
                    })? {
                        task.await.log_err();
                    };

                    let actions = actions
                        .into_iter()
                        .map(|actions| actions.name())
                        .collect::<Vec<&'static str>>();
                    let actions = actions.join("\n");
                    let prompt = format!(
                        "You are helping a user find the most relevant actions in Zed editor based on their natural language query.

User query: \"{query}\"

Available actions in Zed:
{actions}

Instructions:
1. Analyze the user's query to understand their intent
2. Match the query against the available actions, considering:
   - Exact keyword matches
   - Semantic similarity (e.g., \"open file\" matches \"workspace::Open\")
   - Common synonyms and alternative phrasings
   - Partial matches where relevant
3. Return the top 5-10 most relevant actions in order of relevance
4. Return each action name exactly as shown in the list above
5. If no good matches exist, return the closest alternatives

Format your response as a simple list of action names, one per line, with no additional text or explanation."
                    );
                    dbg!(&prompt);

                    let request = LanguageModelRequest {
                        thread_id: None,
                        prompt_id: None,
                        intent: Some(CompletionIntent::GenerateGitCommitMessage),
                        mode: None,
                        messages: vec![LanguageModelRequestMessage {
                            role: Role::User,
                            content: vec![prompt.into()],
                            cache: false,
                        }],
                        tools: Vec::new(),
                        tool_choice: None,
                        stop: Vec::new(),
                        temperature,
                        thinking_allowed: false,
                    };

                    let stream = model.stream_completion_text(request, cx);
                    dbg!("pinging stream");
                    let mut messages = stream.await?;
                    let mut buffer = String::new();
                    while let Some(Ok(message)) = messages.stream.next().await {
                        buffer.push_str(&message);
                    }

                    dbg!(&buffer);

                    // Split result by `\n` and for each string, call `cx.build_action`.
                    let commands = cx.update(move |_window, cx| {
                        let mut commands: Vec<Command> = vec![];

                        for name in buffer.lines() {
                            dbg!(name);

                            let action = cx.build_action(name, None);
                            match action {
                                Ok(action) => {
                                    commands.push(Command { action: action, name: humanize_action_name(name) })
                                    },
                                Err(err) => {
                                    log::error!("Failed to build action: {}", err);
                                }
                            }
                        }

                        commands
                    });

                    dbg!(&commands);
                    if let Ok(commands) = commands {
                        let _ = this.update(cx, |this, cx| {
                            let _ = this.delegate.magic_palette.update(cx, |magic_palette, _| {
                                magic_palette.matches = commands;
                            });
                        });
                    }

                    //
                    Ok(())
                });
            }
            MagicPaletteMode::SelectResult(commands) => todo!(),
        }
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<picker::Picker<Self>>) {
        self.magic_palette
            .update(cx, |_, cx| {
                cx.emit(DismissEvent);
            })
            .ok();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) -> Option<Self::ListItem> {
        None
    }

    fn confirm_input(
        &mut self,
        _secondary: bool,
        _window: &mut Window,
        _: &mut Context<picker::Picker<Self>>,
    ) {
    }
}
