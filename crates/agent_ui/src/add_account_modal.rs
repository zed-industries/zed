//! "Add account…": names a new login profile for an agent, creates its home
//! directory, records it in settings, and opens a thread with it. The
//! thread's sign-in prompt then logs in inside that profile.

use agent_accounts::{AccountId, AccountProvider};
use editor::Editor;
use fs::Fs;
use gpui::{AppContext as _, DismissEvent, Entity, EventEmitter, Focusable};
use project::AgentId;
use settings::{AgentAccountSettingsContent, update_settings_file};
use ui::{Label, LabelSize, prelude::*};
use workspace::{ModalView, Workspace};

use crate::{AddAgentAccount, NewExternalAgentThread};

pub struct AddAccountModal {
    agent_id: AgentId,
    provider: AccountProvider,
    editor: Entity<Editor>,
    error: Option<SharedString>,
}

impl EventEmitter<DismissEvent> for AddAccountModal {}
impl ModalView for AddAccountModal {}

impl Focusable for AddAccountModal {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.editor.focus_handle(cx)
    }
}

pub fn register(workspace: &mut Workspace) {
    workspace.register_action(|workspace, action: &AddAgentAccount, window, cx| {
        let Some(provider) = AccountProvider::for_agent(action.agent.as_ref()) else {
            return;
        };
        let agent_id = action.agent.clone();
        workspace.toggle_modal(window, cx, |window, cx| {
            AddAccountModal::new(agent_id, provider, window, cx)
        });
    });
}

impl AddAccountModal {
    fn new(
        agent_id: AgentId,
        provider: AccountProvider,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Account name, e.g. Work", window, cx);
            editor
        });
        Self {
            agent_id,
            provider,
            editor,
            error: None,
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let name = self.editor.read(cx).text(cx).trim().to_string();
        if name.is_empty() {
            self.error = Some("Enter a name for the account.".into());
            cx.notify();
            return;
        }
        let home_dir = util::paths::home_dir();
        let home = agent_accounts::new_account_home(self.provider, &name, home_dir);
        if let Err(error) = agent_accounts::prepare_account_home(self.provider, &home) {
            self.error = Some(format!("Couldn't create {}: {error}", home.display()).into());
            cx.notify();
            return;
        }

        let entry = AgentAccountSettingsContent {
            agent: self.agent_id.to_string(),
            home: agent_accounts::fallback_account_label(&AccountId::new(&home), home_dir),
            name: Some(name),
        };
        update_settings_file(<dyn Fs>::global(cx), cx, move |content, _| {
            content
                .agent_accounts
                .get_or_insert_default()
                .accounts
                .get_or_insert_default()
                .push(entry);
        });

        window.dispatch_action(
            Box::new(NewExternalAgentThread {
                agent: self.agent_id.clone(),
                account: Some(AccountId::new(&home)),
            }),
            cx,
        );
        cx.emit(DismissEvent);
    }
}

impl Render for AddAccountModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let hint = match self.provider {
            AccountProvider::Cursor => {
                "Creates a separate Cursor profile. Its agent runs with that \
                 profile as HOME, so link .gitconfig or .ssh into it if needed."
            }
            _ => {
                "Creates a separate profile and opens a thread with it; sign in \
                 there with the other account."
            }
        };
        v_flex()
            .key_context("AddAccountModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_3(cx)
            .w_96()
            .overflow_hidden()
            .child(
                v_flex()
                    .p_2()
                    .gap_1()
                    .border_b_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(Label::new(format!(
                        "Add {} account",
                        self.provider.display_name()
                    )))
                    .child(self.editor.clone()),
            )
            .child(
                div()
                    .bg(cx.theme().colors().editor_background)
                    .w_full()
                    .p_2()
                    .child(match &self.error {
                        Some(error) => Label::new(error.clone())
                            .size(LabelSize::Small)
                            .color(Color::Error),
                        None => Label::new(hint).size(LabelSize::Small).color(Color::Muted),
                    }),
            )
    }
}
