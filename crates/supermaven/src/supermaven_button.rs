use crate::{AccountStatus, Supermaven};
use gpui::{AnchorCorner, Render, View};
use language::language_settings::{all_language_settings, InlineCompletionProvider};
#[allow(unused_imports)]
use ui::{
    div, popover_menu, ContextMenu, IconButton, IconName, IntoElement, ParentElement as _,
    ViewContext,
};
use ui::{ButtonCommon as _, Tooltip};

use workspace::item::ItemHandle;

use workspace::StatusItemView;

pub struct SupermavenButton {}

// Button that allows you to authenticate with the Supermaven API
// the signup/auth URL will not be known until the `sm-agent` lets us know what it is
// We'll be tracking the status for that.

impl SupermavenButton {
    pub fn new() -> Self {
        Self {}
    }

    fn build_activation_menu(
        &mut self,
        activation_url: String,
        cx: &mut ViewContext<Self>,
    ) -> View<ContextMenu> {
        ContextMenu::build(cx, |menu, _| {
            let activation_url = activation_url.clone();
            menu.entry("Sign In", None, move |cx| {
                cx.open_url(activation_url.as_str())
            })
        })
    }
}

enum SupermavenButtonStatus {
    Ready,
    Errored(String),
    NeedsActivation(String),
    Disabled,
    Initializing,
}

impl SupermavenButtonStatus {
    fn to_icon(&self) -> IconName {
        match self {
            SupermavenButtonStatus::Ready => IconName::Supermaven,
            SupermavenButtonStatus::Errored(_) => IconName::SupermavenError,
            SupermavenButtonStatus::NeedsActivation(_) => IconName::SupermavenInit,
            SupermavenButtonStatus::Disabled => IconName::SupermavenDisabled,
            SupermavenButtonStatus::Initializing => IconName::SupermavenInit,
        }
    }
}

impl Render for SupermavenButton {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let all_language_settings = all_language_settings(None, cx);
        if all_language_settings.inline_completions.provider != InlineCompletionProvider::Supermaven
        {
            return div();
        }

        let Some(supermaven) = Supermaven::global(cx) else {
            return div();
        };

        let supermaven = supermaven.read(cx);

        let status = match supermaven {
            Supermaven::Starting => SupermavenButtonStatus::Initializing,
            Supermaven::FailedDownload { error } => {
                SupermavenButtonStatus::Errored(error.to_string())
            }
            Supermaven::Spawned(agent) => {
                let account_status = agent.account_status.clone();

                match account_status {
                    AccountStatus::NeedsActivation { activate_url } => {
                        SupermavenButtonStatus::NeedsActivation(activate_url.clone())
                    }
                    AccountStatus::Unknown => SupermavenButtonStatus::Initializing,
                    AccountStatus::Ready => SupermavenButtonStatus::Ready,
                }
            }
        };

        let this = cx.view().clone();

        div().child(
            popover_menu("supermaven")
                .menu(move |cx| match &status {
                    SupermavenButtonStatus::NeedsActivation(activate_url) => {
                        Some(this.update(cx, |this, cx| {
                            this.build_activation_menu(activate_url.clone(), cx)
                        }))
                    }
                    _ => None,
                })
                .anchor(AnchorCorner::BottomRight)
                .trigger(
                    IconButton::new("supermaven-icon", status.to_icon())
                        .tooltip(|cx| Tooltip::text("Supermaven", cx)),
                ),
        )
    }
}

impl StatusItemView for SupermavenButton {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn ItemHandle>,
        _cx: &mut ViewContext<Self>,
    ) {
    }
}
