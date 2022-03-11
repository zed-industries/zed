use std::sync::Arc;

use client::{Contact, UserStore};
use gpui::{
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    platform::CursorStyle,
    Element, ElementBox, Entity, LayoutContext, ModelHandle, RenderContext, Subscription, View,
    ViewContext,
};
use workspace::{AppState, JoinProject, JoinProjectParams, Settings};

pub struct ContactsPanel {
    contacts: ListState,
    user_store: ModelHandle<UserStore>,
    _maintain_contacts: Subscription,
}

impl ContactsPanel {
    pub fn new(app_state: Arc<AppState>, cx: &mut ViewContext<Self>) -> Self {
        Self {
            contacts: ListState::new(
                app_state.user_store.read(cx).contacts().len(),
                Orientation::Top,
                1000.,
                {
                    let app_state = app_state.clone();
                    move |ix, cx| {
                        let user_store = app_state.user_store.read(cx);
                        let contacts = user_store.contacts().clone();
                        let current_user_id = user_store.current_user().map(|user| user.id);
                        Self::render_collaborator(
                            &contacts[ix],
                            current_user_id,
                            app_state.clone(),
                            cx,
                        )
                    }
                },
            ),
            _maintain_contacts: cx.observe(&app_state.user_store, Self::update_contacts),
            user_store: app_state.user_store.clone(),
        }
    }

    fn update_contacts(&mut self, _: ModelHandle<UserStore>, cx: &mut ViewContext<Self>) {
        self.contacts
            .reset(self.user_store.read(cx).contacts().len());
        cx.notify();
    }

    fn render_collaborator(
        collaborator: &Contact,
        current_user_id: Option<u64>,
        app_state: Arc<AppState>,
        cx: &mut LayoutContext,
    ) -> ElementBox {
        let theme = cx.app_state::<Settings>().theme.clone();
        let theme = &theme.contacts_panel;
        let project_count = collaborator.projects.len();
        let font_cache = cx.font_cache();
        let line_height = theme.unshared_project.name.text.line_height(font_cache);
        let cap_height = theme.unshared_project.name.text.cap_height(font_cache);
        let baseline_offset = theme.unshared_project.name.text.baseline_offset(font_cache)
            + (theme.unshared_project.height - line_height) / 2.;
        let tree_branch_width = theme.tree_branch_width;
        let tree_branch_color = theme.tree_branch_color;
        let host_avatar_height = theme
            .host_avatar
            .width
            .or(theme.host_avatar.height)
            .unwrap_or(0.);

        Flex::column()
            .with_child(
                Flex::row()
                    .with_children(collaborator.user.avatar.clone().map(|avatar| {
                        Image::new(avatar)
                            .with_style(theme.host_avatar)
                            .aligned()
                            .left()
                            .boxed()
                    }))
                    .with_child(
                        Label::new(
                            collaborator.user.github_login.clone(),
                            theme.host_username.text.clone(),
                        )
                        .contained()
                        .with_style(theme.host_username.container)
                        .aligned()
                        .left()
                        .boxed(),
                    )
                    .constrained()
                    .with_height(theme.host_row_height)
                    .boxed(),
            )
            .with_children(
                collaborator
                    .projects
                    .iter()
                    .enumerate()
                    .map(|(ix, project)| {
                        let project_id = project.id;

                        Flex::row()
                            .with_child(
                                Canvas::new(move |bounds, _, cx| {
                                    let start_x = bounds.min_x() + (bounds.width() / 2.)
                                        - (tree_branch_width / 2.);
                                    let end_x = bounds.max_x();
                                    let start_y = bounds.min_y();
                                    let end_y =
                                        bounds.min_y() + baseline_offset - (cap_height / 2.);

                                    cx.scene.push_quad(gpui::Quad {
                                        bounds: RectF::from_points(
                                            vec2f(start_x, start_y),
                                            vec2f(
                                                start_x + tree_branch_width,
                                                if ix + 1 == project_count {
                                                    end_y
                                                } else {
                                                    bounds.max_y()
                                                },
                                            ),
                                        ),
                                        background: Some(tree_branch_color),
                                        border: gpui::Border::default(),
                                        corner_radius: 0.,
                                    });
                                    cx.scene.push_quad(gpui::Quad {
                                        bounds: RectF::from_points(
                                            vec2f(start_x, end_y),
                                            vec2f(end_x, end_y + tree_branch_width),
                                        ),
                                        background: Some(tree_branch_color),
                                        border: gpui::Border::default(),
                                        corner_radius: 0.,
                                    });
                                })
                                .constrained()
                                .with_width(host_avatar_height)
                                .boxed(),
                            )
                            .with_child({
                                let is_host = Some(collaborator.user.id) == current_user_id;
                                let is_guest = !is_host
                                    && project
                                        .guests
                                        .iter()
                                        .any(|guest| Some(guest.id) == current_user_id);
                                let is_shared = project.is_shared;
                                let app_state = app_state.clone();

                                MouseEventHandler::new::<ContactsPanel, _, _>(
                                    project_id as usize,
                                    cx,
                                    |mouse_state, _| {
                                        let style = match (project.is_shared, mouse_state.hovered) {
                                            (false, false) => &theme.unshared_project,
                                            (false, true) => &theme.hovered_unshared_project,
                                            (true, false) => &theme.shared_project,
                                            (true, true) => &theme.hovered_shared_project,
                                        };

                                        Flex::row()
                                            .with_child(
                                                Label::new(
                                                    project.worktree_root_names.join(", "),
                                                    style.name.text.clone(),
                                                )
                                                .aligned()
                                                .left()
                                                .contained()
                                                .with_style(style.name.container)
                                                .boxed(),
                                            )
                                            .with_children(project.guests.iter().filter_map(
                                                |participant| {
                                                    participant.avatar.clone().map(|avatar| {
                                                        Image::new(avatar)
                                                            .with_style(style.guest_avatar)
                                                            .aligned()
                                                            .left()
                                                            .contained()
                                                            .with_margin_right(
                                                                style.guest_avatar_spacing,
                                                            )
                                                            .boxed()
                                                    })
                                                },
                                            ))
                                            .contained()
                                            .with_style(style.container)
                                            .constrained()
                                            .with_height(style.height)
                                            .boxed()
                                    },
                                )
                                .with_cursor_style(if is_host || is_shared {
                                    CursorStyle::PointingHand
                                } else {
                                    CursorStyle::Arrow
                                })
                                .on_click(move |cx| {
                                    if !is_host && !is_guest {
                                        cx.dispatch_global_action(JoinProject(JoinProjectParams {
                                            project_id,
                                            app_state: app_state.clone(),
                                        }));
                                    }
                                })
                                .flexible(1., true)
                                .boxed()
                            })
                            .constrained()
                            .with_height(theme.unshared_project.height)
                            .boxed()
                    }),
            )
            .boxed()
    }
}

pub enum Event {}

impl Entity for ContactsPanel {
    type Event = Event;
}

impl View for ContactsPanel {
    fn ui_name() -> &'static str {
        "ContactsPanel"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = &cx.app_state::<Settings>().theme.contacts_panel;
        Container::new(List::new(self.contacts.clone()).boxed())
            .with_style(theme.container)
            .boxed()
    }
}
