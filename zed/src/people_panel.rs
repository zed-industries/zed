use crate::{
    theme::Theme,
    user::{Collaborator, UserStore},
    Settings,
};
use gpui::{
    action,
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    platform::CursorStyle,
    Element, ElementBox, Entity, LayoutContext, ModelHandle, RenderContext, Subscription, View,
    ViewContext,
};
use postage::watch;

action!(JoinWorktree, u64);

pub struct PeoplePanel {
    collaborators: ListState,
    user_store: ModelHandle<UserStore>,
    settings: watch::Receiver<Settings>,
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
                    let settings = settings.clone();
                    move |ix, cx| {
                        let collaborators = user_store.read(cx).collaborators().clone();
                        Self::render_collaborator(&collaborators[ix], &settings.borrow().theme, cx)
                    }
                },
            ),
            _maintain_collaborators: cx.observe(&user_store, Self::update_collaborators),
            user_store,
            settings,
        }
    }

    fn update_collaborators(&mut self, _: ModelHandle<UserStore>, cx: &mut ViewContext<Self>) {
        self.collaborators
            .reset(self.user_store.read(cx).collaborators().len());
        cx.notify();
    }

    fn render_collaborator(
        collaborator: &Collaborator,
        theme: &Theme,
        cx: &mut LayoutContext,
    ) -> ElementBox {
        let theme = &theme.people_panel;
        let worktree_count = collaborator.worktrees.len();
        let font_cache = cx.font_cache();
        let line_height = theme.worktree.text.line_height(font_cache);
        let cap_height = theme.worktree.text.cap_height(font_cache);
        let baseline_offset = theme.worktree.text.baseline_offset(font_cache);
        let tree_branch = theme.tree_branch;

        Flex::column()
            .with_child(
                Flex::row()
                    .with_children(
                        collaborator
                            .user
                            .avatar
                            .clone()
                            .map(|avatar| Image::new(avatar).with_style(theme.host_avatar).boxed()),
                    )
                    .with_child(
                        Container::new(
                            Label::new(
                                collaborator.user.github_login.clone(),
                                theme.host_username.text.clone(),
                            )
                            .boxed(),
                        )
                        .with_style(theme.host_username.container)
                        .boxed(),
                    )
                    .boxed(),
            )
            .with_children(
                collaborator
                    .worktrees
                    .iter()
                    .enumerate()
                    .map(|(ix, worktree)| {
                        let worktree_id = worktree.id;
                        Flex::row()
                            .with_child(
                                ConstrainedBox::new(
                                    Canvas::new(move |bounds, _, cx| {
                                        let start_x = bounds.min_x() + (bounds.width() / 2.)
                                            - (tree_branch.width / 2.);
                                        let end_x = bounds.max_x();
                                        let start_y = bounds.min_y();
                                        let end_y =
                                            bounds.min_y() + baseline_offset - (cap_height / 2.);

                                        cx.scene.push_quad(gpui::Quad {
                                            bounds: RectF::from_points(
                                                vec2f(start_x, start_y),
                                                vec2f(
                                                    start_x + tree_branch.width,
                                                    if ix + 1 == worktree_count {
                                                        end_y
                                                    } else {
                                                        bounds.max_y()
                                                    },
                                                ),
                                            ),
                                            background: Some(tree_branch.color),
                                            border: gpui::Border::default(),
                                            corner_radius: 0.,
                                        });
                                        cx.scene.push_quad(gpui::Quad {
                                            bounds: RectF::from_points(
                                                vec2f(start_x, end_y),
                                                vec2f(end_x, end_y + tree_branch.width),
                                            ),
                                            background: Some(tree_branch.color),
                                            border: gpui::Border::default(),
                                            corner_radius: 0.,
                                        });
                                    })
                                    .boxed(),
                                )
                                .with_width(20.)
                                .with_height(line_height)
                                .boxed(),
                            )
                            .with_child(
                                MouseEventHandler::new::<PeoplePanel, _, _, _>(
                                    worktree_id as usize,
                                    cx,
                                    |mouse_state, _| {
                                        let style = if mouse_state.hovered {
                                            &theme.hovered_worktree
                                        } else {
                                            &theme.worktree
                                        };
                                        Container::new(
                                            Label::new(
                                                worktree.root_name.clone(),
                                                style.text.clone(),
                                            )
                                            .boxed(),
                                        )
                                        .with_style(style.container)
                                        .boxed()
                                    },
                                )
                                .with_cursor_style(CursorStyle::PointingHand)
                                .on_click(move |cx| cx.dispatch_action(JoinWorktree(worktree_id)))
                                .boxed(),
                            )
                            .with_children(worktree.guests.iter().filter_map(|participant| {
                                participant.avatar.clone().map(|avatar| {
                                    Image::new(avatar).with_style(theme.guest_avatar).boxed()
                                })
                            }))
                            .boxed()
                    }),
            )
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
        let theme = &self.settings.borrow().theme.people_panel;
        Container::new(List::new(self.collaborators.clone()).boxed())
            .with_style(theme.container)
            .boxed()
    }
}
