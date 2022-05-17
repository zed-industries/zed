use crate::{
    sidebar::{Side, ToggleSidebarItem},
    AppState,
};
use anyhow::Result;
use client::{proto, Client, Contact};
use gpui::{
    elements::*, ElementBox, Entity, ImageData, MutableAppContext, RenderContext, Task, View,
    ViewContext,
};
use project::Project;
use settings::Settings;
use std::sync::Arc;
use util::ResultExt;

pub struct WaitingRoom {
    project_id: u64,
    avatar: Option<Arc<ImageData>>,
    message: String,
    waiting: bool,
    client: Arc<Client>,
    _join_task: Task<Result<()>>,
}

impl Entity for WaitingRoom {
    type Event = ();

    fn release(&mut self, _: &mut MutableAppContext) {
        if self.waiting {
            self.client
                .send(proto::LeaveProject {
                    project_id: self.project_id,
                })
                .log_err();
        }
    }
}

impl View for WaitingRoom {
    fn ui_name() -> &'static str {
        "WaitingRoom"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = &cx.global::<Settings>().theme.workspace;

        Flex::column()
            .with_children(self.avatar.clone().map(|avatar| {
                Image::new(avatar)
                    .with_style(theme.joining_project_avatar)
                    .aligned()
                    .boxed()
            }))
            .with_child(
                Text::new(
                    self.message.clone(),
                    theme.joining_project_message.text.clone(),
                )
                .contained()
                .with_style(theme.joining_project_message.container)
                .aligned()
                .boxed(),
            )
            .aligned()
            .contained()
            .with_background_color(theme.background)
            .boxed()
    }
}

impl WaitingRoom {
    pub fn new(
        contact: Arc<Contact>,
        project_index: usize,
        app_state: Arc<AppState>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let project_id = contact.projects[project_index].id;
        let client = app_state.client.clone();
        let _join_task = cx.spawn_weak({
            let contact = contact.clone();
            |this, mut cx| async move {
                let project = Project::remote(
                    project_id,
                    app_state.client.clone(),
                    app_state.user_store.clone(),
                    app_state.languages.clone(),
                    app_state.fs.clone(),
                    &mut cx,
                )
                .await;

                if let Some(this) = this.upgrade(&cx) {
                    this.update(&mut cx, |this, cx| {
                        this.waiting = false;
                        match project {
                            Ok(project) => {
                                cx.replace_root_view(|cx| {
                                    let mut workspace =
                                        (app_state.build_workspace)(project, &app_state, cx);
                                    workspace.toggle_sidebar_item(
                                        &ToggleSidebarItem {
                                            side: Side::Left,
                                            item_index: 0,
                                        },
                                        cx,
                                    );
                                    workspace
                                });
                            }
                            Err(error @ _) => {
                                let login = &contact.user.github_login;
                                let message = match error {
                                    project::JoinProjectError::HostDeclined => {
                                        format!("@{} declined your request.", login)
                                    }
                                    project::JoinProjectError::HostClosedProject => {
                                        format!(
                                            "@{} closed their copy of {}.",
                                            login,
                                            humanize_list(
                                                &contact.projects[project_index]
                                                    .worktree_root_names
                                            )
                                        )
                                    }
                                    project::JoinProjectError::HostWentOffline => {
                                        format!("@{} went offline.", login)
                                    }
                                    project::JoinProjectError::Other(error) => {
                                        log::error!("error joining project: {}", error);
                                        "An error occurred.".to_string()
                                    }
                                };
                                this.message = message;
                                cx.notify();
                            }
                        }
                    })
                }

                Ok(())
            }
        });

        Self {
            project_id,
            avatar: contact.user.avatar.clone(),
            message: format!(
                "Asking to join @{}'s copy of {}...",
                contact.user.github_login,
                humanize_list(&contact.projects[project_index].worktree_root_names)
            ),
            waiting: true,
            client,
            _join_task,
        }
    }
}

fn humanize_list<'a>(items: impl IntoIterator<Item = &'a String>) -> String {
    let mut list = String::new();
    let mut items = items.into_iter().enumerate().peekable();
    while let Some((ix, item)) = items.next() {
        if ix > 0 {
            list.push_str(", ");
            if items.peek().is_none() {
                list.push_str("and ");
            }
        }

        list.push_str(item);
    }
    list
}
