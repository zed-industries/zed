use agent_settings::AgentSettings;
use anyhow::Result;
use cloud_llm_client::CompletionIntent;
use command_palette::humanize_action_name;
use futures::StreamExt as _;
use gpui::{
    Action, AppContext as _, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    IntoElement, Task, WeakEntity,
};
use language_model::{
    ConfiguredModel, LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage, Role,
};
use picker::{Picker, PickerDelegate};
use settings::Settings as _;
use ui::{
    App, Context, InteractiveElement, KeyBinding, Label, ListItem, ListItemSpacing,
    ParentElement as _, Render, Styled as _, Toggleable as _, Window, div, h_flex, rems,
};
use util::ResultExt;
use workspace::{ModalView, Workspace};

pub fn init(cx: &mut App) {
    cx.observe_new(MagicPalette::register).detach();
}

gpui::actions!(magic_palette, [Toggle]);

fn format_prompt(query: &str, actions: &str) -> String {
    format!(
        "Match the query: \"{query}\" to relevant actions. Return 5-10 action names, most relevant first, one per line.
        Actions:
        {actions}"
    )
}

struct MagicPalette {
    picker: Entity<Picker<MagicPaletteDelegate>>,
}

impl ModalView for MagicPalette {}

impl EventEmitter<DismissEvent> for MagicPalette {}

impl Focusable for MagicPalette {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
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
        let Some(previous_focus_handle) = window.focused(cx) else {
            return;
        };

        if agent_settings::AgentSettings::get_global(cx).enabled(cx) {
            workspace.toggle_modal(window, cx, |window, cx| {
                MagicPalette::new(previous_focus_handle, window, cx)
            });
        }
    }

    fn new(
        previous_focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let this = cx.weak_entity();
        let delegate = MagicPaletteDelegate::new(this, previous_focus_handle);
        let picker = cx.new(|cx| {
            let picker = Picker::uniform_list(delegate, window, cx);
            picker
        });
        Self { picker }
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

struct MagicPaletteDelegate {
    query: String,
    llm_generation_task: Option<Task<Result<()>>>,
    magic_palette: WeakEntity<MagicPalette>,
    matches: Vec<Command>,
    selected_index: usize,
    previous_focus_handle: FocusHandle,
}

impl MagicPaletteDelegate {
    fn new(magic_palette: WeakEntity<MagicPalette>, previous_focus_handle: FocusHandle) -> Self {
        Self {
            query: String::new(),
            llm_generation_task: None,
            magic_palette,
            matches: vec![],
            selected_index: 0,
            previous_focus_handle,
        }
    }
}

impl PickerDelegate for MagicPaletteDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
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

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<ui::SharedString> {
        if self.llm_generation_task.is_some() {
            Some("Generating...".into())
        } else {
            None
        }
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
        if self.matches.is_empty() {
            let Some(ConfiguredModel { provider, model }) =
                LanguageModelRegistry::read_global(cx).commit_message_model()
            else {
                return;
            };
            let temperature = AgentSettings::temperature_for_model(&model, cx);
            let query = self.query.clone();
            cx.notify();
            self.llm_generation_task = Some(cx.spawn_in(window, async move |this, cx| {
                let actions = cx.update(|_, cx| cx.action_documentation().clone())?;

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
                    .filter(|(action, _)| !action.starts_with("vim") && !action.starts_with("dev"))
                    .map(|(name, description)| {
                        let short = description
                            .split_whitespace()
                            .take(5)
                            .collect::<Vec<_>>()
                            .join(" ");

                        format!("{} | {}", name, short)
                    })
                    .collect::<Vec<String>>();
                let actions = actions.join("\n");
                let prompt = format_prompt(&query, &actions);
                println!("{}", prompt);

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

                // Split result by `\n` and for each string, call `cx.build_action`.
                let commands = cx.update(move |_window, cx| {
                    let mut commands: Vec<Command> = vec![];

                    for name in buffer.lines() {
                        dbg!(name);

                        let action = cx.build_action(name, None);
                        match action {
                            Ok(action) => commands.push(Command {
                                action: action,
                                name: humanize_action_name(name),
                            }),
                            Err(err) => {
                                log::error!("Failed to build action: {}", err);
                            }
                        }
                    }

                    commands
                })?;

                this.update(cx, |this, cx| {
                    this.delegate.matches = commands;
                    this.delegate.llm_generation_task = None;
                    this.delegate.selected_index = 0;
                    cx.notify();
                })?;

                Ok(())
            }));
        } else {
            let command = self.matches.swap_remove(self.selected_index);
            telemetry::event!(
                "Action Invoked",
                source = "magic palette",
                action = command.name
            );
            self.matches.clear();
            self.query.clear();
            self.llm_generation_task.take();

            let action = command.action;
            window.focus(&self.previous_focus_handle);
            self.dismissed(window, cx);
            window.dispatch_action(action, cx);
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
        _window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let command = self.matches.get(ix)?;

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    h_flex()
                        .w_full()
                        .py_px()
                        .justify_between()
                        .child(Label::new(command.name.clone()))
                        .child(KeyBinding::for_action_in(
                            &*command.action,
                            &self.previous_focus_handle,
                            cx,
                        )),
                ),
        )
    }
}
