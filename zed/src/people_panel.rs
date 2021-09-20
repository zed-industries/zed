use crate::{
    theme::Theme,
    user::{Collaborator, UserStore},
    Settings,
};
use gpui::{
    elements::*, Element, ElementBox, Entity, ModelHandle, RenderContext, Subscription, View,
    ViewContext,
};
use postage::watch;

pub struct PeoplePanel {
    collaborators: ListState,
    user_store: ModelHandle<UserStore>,
    _maintain_collaborators: Subscription,
}

impl PeoplePanel {
    pub fn new(
        user_store: ModelHandle<UserStore>,
        settings: watch::Receiver<Settings>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            collaborators: ListState::new(
                user_store.read(cx).collaborators().len(),
                Orientation::Top,
                1000.,
                {
                    let user_store = user_store.clone();
                    move |ix, cx| {
                        let user_store = user_store.read(cx);
                        let settings = settings.borrow();
                        Self::render_collaborator(&user_store.collaborators()[ix], &settings.theme)
                    }
                },
            ),
            _maintain_collaborators: cx.observe(&user_store, Self::update_collaborators),
            user_store,
        }
    }

    fn update_collaborators(&mut self, _: ModelHandle<UserStore>, cx: &mut ViewContext<Self>) {
        self.collaborators
            .reset(self.user_store.read(cx).collaborators().len());
        cx.notify();
    }

    fn render_collaborator(collaborator: &Collaborator, theme: &Theme) -> ElementBox {
        Flex::column()
            .with_child(
                Flex::row()
                    .with_children(collaborator.user.avatar.clone().map(|avatar| {
                        ConstrainedBox::new(
                            Image::new(avatar)
                                .with_style(theme.people_panel.worktree_host_avatar)
                                .boxed(),
                        )
                        .with_width(20.)
                        .boxed()
                    }))
                    .with_child(
                        Label::new(
                            collaborator.user.github_login.clone(),
                            theme.people_panel.host_username.clone(),
                        )
                        .boxed(),
                    )
                    .boxed(),
            )
            .with_children(collaborator.worktrees.iter().map(|worktree| {
                Flex::row()
                    .with_child(
                        Container::new(
                            Label::new(
                                worktree.root_name.clone(),
                                theme.people_panel.worktree_name.text.clone(),
                            )
                            .boxed(),
                        )
                        .with_style(theme.people_panel.worktree_name.container)
                        .boxed(),
                    )
                    .with_children(worktree.participants.iter().filter_map(|participant| {
                        participant.avatar.clone().map(|avatar| {
                            ConstrainedBox::new(
                                Image::new(avatar)
                                    .with_style(theme.people_panel.worktree_guest_avatar)
                                    .boxed(),
                            )
                            .with_width(16.)
                            .boxed()
                        })
                    }))
                    .boxed()
            }))
            .boxed()
    }
}

pub enum Event {}

impl Entity for PeoplePanel {
    type Event = Event;
}

impl View for PeoplePanel {
    fn ui_name() -> &'static str {
        "PeoplePanel"
    }

    fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
        List::new(self.collaborators.clone()).boxed()
    }
}
