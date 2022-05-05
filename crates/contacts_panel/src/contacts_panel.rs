use client::{Contact, User, UserStore};
use editor::Editor;
use gpui::{
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    platform::CursorStyle,
    Element, ElementBox, Entity, LayoutContext, ModelHandle, RenderContext, Subscription, Task,
    View, ViewContext, ViewHandle,
};
use settings::Settings;
use std::sync::Arc;
use util::ResultExt;
use workspace::{AppState, JoinProject};

pub struct ContactsPanel {
    list_state: ListState,
    potential_contacts: Vec<Arc<User>>,
    user_store: ModelHandle<UserStore>,
    contacts_search_task: Option<Task<Option<()>>>,
    user_query_editor: ViewHandle<Editor>,
    _maintain_contacts: Subscription,
}

impl ContactsPanel {
    pub fn new(app_state: Arc<AppState>, cx: &mut ViewContext<Self>) -> Self {
        let user_query_editor = cx.add_view(|cx| {
            Editor::single_line(
                Some(|theme| theme.contacts_panel.user_query_editor.clone()),
                cx,
            )
        });

        cx.subscribe(&user_query_editor, |this, _, event, cx| {
            if let editor::Event::BufferEdited = event {
                this.user_query_changed(cx)
            }
        })
        .detach();

        Self {
            list_state: ListState::new(
                1 + app_state.user_store.read(cx).contacts().len(), // Add 1 for the "Contacts" header
                Orientation::Top,
                1000.,
                {
                    let this = cx.weak_handle();
                    let app_state = app_state.clone();
                    move |ix, cx| {
                        let this = this.upgrade(cx).unwrap();
                        let this = this.read(cx);
                        let user_store = this.user_store.read(cx);
                        let contacts = user_store.contacts().clone();
                        let current_user_id = user_store.current_user().map(|user| user.id);
                        let theme = cx.global::<Settings>().theme.clone();
                        let theme = &theme.contacts_panel;

                        if ix == 0 {
                            Label::new("contacts".to_string(), theme.header.text.clone())
                                .contained()
                                .with_style(theme.header.container)
                                .aligned()
                                .left()
                                .constrained()
                                .with_height(theme.row_height)
                                .boxed()
                        } else if ix < contacts.len() + 1 {
                            let contact_ix = ix - 1;
                            Self::render_contact(
                                &contacts[contact_ix],
                                current_user_id,
                                app_state.clone(),
                                theme,
                                cx,
                            )
                        } else if ix == contacts.len() + 1 {
                            Label::new("add contacts".to_string(), theme.header.text.clone())
                                .contained()
                                .with_style(theme.header.container)
                                .aligned()
                                .left()
                                .constrained()
                                .with_height(theme.row_height)
                                .boxed()
                        } else {
                            let potential_contact_ix = ix - 2 - contacts.len();
                            Self::render_potential_contact(
                                &this.potential_contacts[potential_contact_ix],
                                theme,
                            )
                        }
                    }
                },
            ),
            potential_contacts: Default::default(),
            user_query_editor,
            _maintain_contacts: cx.observe(&app_state.user_store, |this, _, cx| {
                this.update_contacts(cx)
            }),
            contacts_search_task: None,
            user_store: app_state.user_store.clone(),
        }
    }

    fn update_contacts(&mut self, cx: &mut ViewContext<Self>) {
        let mut list_len = 1 + self.user_store.read(cx).contacts().len();
        if !self.potential_contacts.is_empty() {
            list_len += 1 + self.potential_contacts.len();
        }

        self.list_state.reset(list_len);
        cx.notify();
    }

    fn render_contact(
        contact: &Contact,
        current_user_id: Option<u64>,
        app_state: Arc<AppState>,
        theme: &theme::ContactsPanel,
        cx: &mut LayoutContext,
    ) -> ElementBox {
        let project_count = contact.projects.len();
        let font_cache = cx.font_cache();
        let line_height = theme.unshared_project.name.text.line_height(font_cache);
        let cap_height = theme.unshared_project.name.text.cap_height(font_cache);
        let baseline_offset = theme.unshared_project.name.text.baseline_offset(font_cache)
            + (theme.unshared_project.height - line_height) / 2.;
        let tree_branch_width = theme.tree_branch_width;
        let tree_branch_color = theme.tree_branch_color;
        let host_avatar_height = theme
            .contact_avatar
            .width
            .or(theme.contact_avatar.height)
            .unwrap_or(0.);

        Flex::column()
            .with_child(
                Flex::row()
                    .with_children(contact.user.avatar.clone().map(|avatar| {
                        Image::new(avatar)
                            .with_style(theme.contact_avatar)
                            .aligned()
                            .left()
                            .boxed()
                    }))
                    .with_child(
                        Label::new(
                            contact.user.github_login.clone(),
                            theme.contact_username.text.clone(),
                        )
                        .contained()
                        .with_style(theme.contact_username.container)
                        .aligned()
                        .left()
                        .boxed(),
                    )
                    .constrained()
                    .with_height(theme.row_height)
                    .boxed(),
            )
            .with_children(contact.projects.iter().enumerate().map(|(ix, project)| {
                let project_id = project.id;

                Flex::row()
                    .with_child(
                        Canvas::new(move |bounds, _, cx| {
                            let start_x =
                                bounds.min_x() + (bounds.width() / 2.) - (tree_branch_width / 2.);
                            let end_x = bounds.max_x();
                            let start_y = bounds.min_y();
                            let end_y = bounds.min_y() + baseline_offset - (cap_height / 2.);

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
                        let is_host = Some(contact.user.id) == current_user_id;
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
                                                    .with_margin_right(style.guest_avatar_spacing)
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
                        .on_click(move |_, cx| {
                            if !is_host && !is_guest {
                                cx.dispatch_global_action(JoinProject {
                                    project_id,
                                    app_state: app_state.clone(),
                                });
                            }
                        })
                        .flex(1., true)
                        .boxed()
                    })
                    .constrained()
                    .with_height(theme.unshared_project.height)
                    .boxed()
            }))
            .boxed()
    }

    fn render_potential_contact(contact: &User, theme: &theme::ContactsPanel) -> ElementBox {
        Flex::row()
            .with_children(contact.avatar.clone().map(|avatar| {
                Image::new(avatar)
                    .with_style(theme.contact_avatar)
                    .aligned()
                    .left()
                    .boxed()
            }))
            .with_child(
                Label::new(
                    contact.github_login.clone(),
                    theme.contact_username.text.clone(),
                )
                .contained()
                .with_style(theme.contact_username.container)
                .aligned()
                .left()
                .boxed(),
            )
            .constrained()
            .with_height(theme.row_height)
            .boxed()
    }

    fn user_query_changed(&mut self, cx: &mut ViewContext<Self>) {
        let query = self.user_query_editor.read(cx).text(cx);
        if query.is_empty() {
            self.potential_contacts.clear();
            self.update_contacts(cx);
            return;
        }

        let search = self
            .user_store
            .update(cx, |store, cx| store.fuzzy_search_users(query, cx));
        self.contacts_search_task = Some(cx.spawn(|this, mut cx| async move {
            let users = search.await.log_err()?;
            this.update(&mut cx, |this, cx| {
                let user_store = this.user_store.read(cx);
                this.potential_contacts = users;
                this.potential_contacts
                    .retain(|user| !user_store.has_contact(&user));
                this.update_contacts(cx);
            });
            None
        }));
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
        let theme = &cx.global::<Settings>().theme.contacts_panel;
        Container::new(
            Flex::column()
                .with_child(
                    Container::new(ChildView::new(self.user_query_editor.clone()).boxed())
                        .with_style(theme.user_query_editor.container)
                        .boxed(),
                )
                .with_child(List::new(self.list_state.clone()).flex(1., false).boxed())
                .boxed(),
        )
        .with_style(theme.container)
        .boxed()
    }
}
