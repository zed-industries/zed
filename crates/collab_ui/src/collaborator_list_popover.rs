use call::ActiveCall;
use gpui::{elements::*, Entity, MouseButton, RenderContext, View, ViewContext};
use settings::Settings;

use crate::collab_titlebar_item::ToggleCollaboratorList;

pub(crate) enum Event {
    Dismissed,
}

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
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let active_call = ActiveCall::global(cx);
        let collaborator_count = active_call
            .read(cx)
            .room()
            .map_or(0, |room| room.read(cx).remote_participants().len());
        Self {
            list_state: ListState::new(
                collaborator_count,
                Orientation::Top,
                0.,
                cx,
                |_, index, cx| {
                    let theme = &cx.global::<Settings>().theme;
                    Label::new(
                        format!("Participant {index}"),
                        theme.contact_list.contact_username.text.clone(),
                    )
                    .boxed()
                },
            ),
        }
    }
}
