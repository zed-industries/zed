use call::ActiveCall;
use client::UserStore;
use gpui::Action;
use gpui::{
    actions, elements::*, Entity, ModelHandle, MouseButton, RenderContext, View, ViewContext,
};
use settings::Settings;

use crate::collab_titlebar_item::ToggleCollaboratorList;

pub(crate) enum Event {
    Dismissed,
}

enum Collaborator {
    SelfUser { username: String },
    RemoteUser { username: String },
}

actions!(collaborator_list_popover, [NoOp]);

pub(crate) struct CollaboratorListPopover {
    list_state: ListState,
}

impl Entity for CollaboratorListPopover {
    type Event = Event;
}

impl View for CollaboratorListPopover {
    fn ui_name() -> &'static str {
        "CollaboratorListPopover"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = cx.global::<Settings>().theme.clone();

        MouseEventHandler::<Self>::new(0, cx, |_, _| {
            List::new(self.list_state.clone())
                .contained()
                .with_style(theme.contacts_popover.container) //TODO: Change the name of this theme key
                .constrained()
                .with_width(theme.contacts_popover.width)
                .with_height(theme.contacts_popover.height)
                .boxed()
        })
        .on_down_out(MouseButton::Left, move |_, cx| {
            cx.dispatch_action(ToggleCollaboratorList);
        })
        .boxed()
    }

    fn focus_out(&mut self, _: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Dismissed);
    }
}

impl CollaboratorListPopover {
    pub fn new(user_store: ModelHandle<UserStore>, cx: &mut ViewContext<Self>) -> Self {
        let active_call = ActiveCall::global(cx);

        let mut collaborators = user_store
            .read(cx)
            .current_user()
            .map(|u| Collaborator::SelfUser {
                username: u.github_login.clone(),
            })
            .into_iter()
            .collect::<Vec<_>>();

        //TODO: What should the canonical sort here look like, consult contacts list implementation
        if let Some(room) = active_call.read(cx).room() {
            for participant in room.read(cx).remote_participants() {
                collaborators.push(Collaborator::RemoteUser {
                    username: participant.1.user.github_login.clone(),
                });
            }
        }

        Self {
            list_state: ListState::new(
                collaborators.len(),
                Orientation::Top,
                0.,
                cx,
                move |_, index, cx| match &collaborators[index] {
                    Collaborator::SelfUser { username } => render_collaborator_list_entry(
                        index,
                        username,
                        None::<NoOp>,
                        None,
                        Svg::new("icons/chevron_right_12.svg"),
                        NoOp,
                        "Leave call".to_owned(),
                        cx,
                    ),

                    Collaborator::RemoteUser { username } => render_collaborator_list_entry(
                        index,
                        username,
                        Some(NoOp),
                        Some(format!("Follow {username}")),
                        Svg::new("icons/x_mark_12.svg"),
                        NoOp,
                        format!("Remove {username} from call"),
                        cx,
                    ),
                },
            ),
        }
    }
}

fn render_collaborator_list_entry<UA: Action + Clone, IA: Action + Clone>(
    index: usize,
    username: &str,
    username_action: Option<UA>,
    username_tooltip: Option<String>,
    icon: Svg,
    icon_action: IA,
    icon_tooltip: String,
    cx: &mut RenderContext<CollaboratorListPopover>,
) -> ElementBox {
    enum Username {}
    enum UsernameTooltip {}
    enum Icon {}
    enum IconTooltip {}

    let theme = &cx.global::<Settings>().theme;
    let username_theme = theme.contact_list.contact_username.text.clone();
    let tooltip_theme = theme.tooltip.clone();

    let username = MouseEventHandler::<Username>::new(index, cx, |_, _| {
        Label::new(username.to_owned(), username_theme.clone()).boxed()
    })
    .on_click(MouseButton::Left, move |_, cx| {
        if let Some(username_action) = username_action.clone() {
            cx.dispatch_action(username_action);
        }
    });

    Flex::row()
        .with_child(if let Some(username_tooltip) = username_tooltip {
            username
                .with_tooltip::<UsernameTooltip, _>(
                    index,
                    username_tooltip,
                    None,
                    tooltip_theme.clone(),
                    cx,
                )
                .boxed()
        } else {
            username.boxed()
        })
        .with_child(
            MouseEventHandler::<Icon>::new(index, cx, |_, _| icon.boxed())
                .on_click(MouseButton::Left, move |_, cx| {
                    cx.dispatch_action(icon_action.clone())
                })
                .with_tooltip::<IconTooltip, _>(index, icon_tooltip, None, tooltip_theme, cx)
                .boxed(),
        )
        .boxed()
}
