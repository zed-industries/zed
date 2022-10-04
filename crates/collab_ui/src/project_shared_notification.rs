use call::{room, ActiveCall};
use client::User;
use gpui::{
    actions,
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    Entity, MouseButton, MutableAppContext, RenderContext, View, ViewContext, WindowBounds,
    WindowKind, WindowOptions,
};
use project::Project;
use settings::Settings;
use std::sync::Arc;
use workspace::{AppState, Workspace};

actions!(project_shared_notification, [JoinProject, DismissProject]);

pub fn init(app_state: Arc<AppState>, cx: &mut MutableAppContext) {
    cx.add_action(ProjectSharedNotification::join);
    cx.add_action(ProjectSharedNotification::dismiss);

    let active_call = ActiveCall::global(cx);
    let mut _room_subscription = None;
    cx.observe(&active_call, move |active_call, cx| {
        if let Some(room) = active_call.read(cx).room().cloned() {
            let app_state = app_state.clone();
            _room_subscription = Some(cx.subscribe(&room, move |_, event, cx| match event {
                room::Event::RemoteProjectShared { owner, project_id } => {
                    cx.add_window(
                        WindowOptions {
                            bounds: WindowBounds::Fixed(RectF::new(
                                vec2f(0., 0.),
                                vec2f(300., 400.),
                            )),
                            titlebar: None,
                            center: true,
                            kind: WindowKind::PopUp,
                            is_movable: false,
                        },
                        |_| {
                            ProjectSharedNotification::new(
                                *project_id,
                                owner.clone(),
                                app_state.clone(),
                            )
                        },
                    );
                }
            }));
        } else {
            _room_subscription = None;
        }
    })
    .detach();
}

pub struct ProjectSharedNotification {
    project_id: u64,
    owner: Arc<User>,
    app_state: Arc<AppState>,
}

impl ProjectSharedNotification {
    fn new(project_id: u64, owner: Arc<User>, app_state: Arc<AppState>) -> Self {
        Self {
            project_id,
            owner,
            app_state,
        }
    }

    fn join(&mut self, _: &JoinProject, cx: &mut ViewContext<Self>) {
        let project_id = self.project_id;
        let app_state = self.app_state.clone();
        cx.spawn_weak(|_, mut cx| async move {
            let project = Project::remote(
                project_id,
                app_state.client.clone(),
                app_state.user_store.clone(),
                app_state.project_store.clone(),
                app_state.languages.clone(),
                app_state.fs.clone(),
                cx.clone(),
            )
            .await?;

            cx.add_window((app_state.build_window_options)(), |cx| {
                let mut workspace = Workspace::new(project, app_state.default_item_factory, cx);
                (app_state.initialize_workspace)(&mut workspace, &app_state, cx);
                workspace
            });

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        let window_id = cx.window_id();
        cx.remove_window(window_id);
    }

    fn dismiss(&mut self, _: &DismissProject, cx: &mut ViewContext<Self>) {
        let window_id = cx.window_id();
        cx.remove_window(window_id);
    }

    fn render_owner(&self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = &cx.global::<Settings>().theme.project_shared_notification;
        Flex::row()
            .with_children(
                self.owner
                    .avatar
                    .clone()
                    .map(|avatar| Image::new(avatar).with_style(theme.owner_avatar).boxed()),
            )
            .with_child(
                Label::new(
                    format!("{} has shared a new project", self.owner.github_login),
                    theme.message.text.clone(),
                )
                .boxed(),
            )
            .boxed()
    }

    fn render_buttons(&self, cx: &mut RenderContext<Self>) -> ElementBox {
        enum Join {}
        enum Dismiss {}

        Flex::row()
            .with_child(
                MouseEventHandler::<Join>::new(0, cx, |_, cx| {
                    let theme = &cx.global::<Settings>().theme.project_shared_notification;
                    Label::new("Join".to_string(), theme.join_button.text.clone())
                        .contained()
                        .with_style(theme.join_button.container)
                        .boxed()
                })
                .on_click(MouseButton::Left, |_, cx| {
                    cx.dispatch_action(JoinProject);
                })
                .boxed(),
            )
            .with_child(
                MouseEventHandler::<Dismiss>::new(0, cx, |_, cx| {
                    let theme = &cx.global::<Settings>().theme.project_shared_notification;
                    Label::new("Dismiss".to_string(), theme.dismiss_button.text.clone())
                        .contained()
                        .with_style(theme.dismiss_button.container)
                        .boxed()
                })
                .on_click(MouseButton::Left, |_, cx| {
                    cx.dispatch_action(DismissProject);
                })
                .boxed(),
            )
            .boxed()
    }
}

impl Entity for ProjectSharedNotification {
    type Event = ();
}

impl View for ProjectSharedNotification {
    fn ui_name() -> &'static str {
        "ProjectSharedNotification"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> gpui::ElementBox {
        Flex::row()
            .with_child(self.render_owner(cx))
            .with_child(self.render_buttons(cx))
            .boxed()
    }
}
