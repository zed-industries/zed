use call::{room, ActiveCall};
use client::User;
use gpui::{
    actions,
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    CursorStyle, Entity, MouseButton, MutableAppContext, RenderContext, View, ViewContext,
    WindowBounds, WindowKind, WindowOptions,
};
use settings::Settings;
use std::sync::Arc;
use workspace::JoinProject;

actions!(project_shared_notification, [DismissProject]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(ProjectSharedNotification::join);
    cx.add_action(ProjectSharedNotification::dismiss);

    let active_call = ActiveCall::global(cx);
    cx.subscribe(&active_call, move |_, event, cx| match event {
        room::Event::RemoteProjectShared { owner, project_id } => {
            const PADDING: f32 = 16.;
            let screen_size = cx.platform().screen_size();
            let window_size = vec2f(366., 64.);
            cx.add_window(
                WindowOptions {
                    bounds: WindowBounds::Fixed(RectF::new(
                        vec2f(screen_size.x() - window_size.x() - PADDING, PADDING),
                        window_size,
                    )),
                    titlebar: None,
                    center: false,
                    kind: WindowKind::PopUp,
                    is_movable: false,
                },
                |_| ProjectSharedNotification::new(*project_id, owner.clone()),
            );
        }
    })
    .detach();
}

pub struct ProjectSharedNotification {
    project_id: u64,
    owner: Arc<User>,
}

impl ProjectSharedNotification {
    fn new(project_id: u64, owner: Arc<User>) -> Self {
        Self { project_id, owner }
    }

    fn join(&mut self, _: &JoinProject, cx: &mut ViewContext<Self>) {
        let window_id = cx.window_id();
        cx.remove_window(window_id);
        cx.propagate_action();
    }

    fn dismiss(&mut self, _: &DismissProject, cx: &mut ViewContext<Self>) {
        let window_id = cx.window_id();
        cx.remove_window(window_id);
    }

    fn render_owner(&self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = &cx.global::<Settings>().theme.project_shared_notification;
        Flex::row()
            .with_children(self.owner.avatar.clone().map(|avatar| {
                Image::new(avatar)
                    .with_style(theme.owner_avatar)
                    .aligned()
                    .boxed()
            }))
            .with_child(
                Flex::column()
                    .with_child(
                        Label::new(
                            self.owner.github_login.clone(),
                            theme.owner_username.text.clone(),
                        )
                        .contained()
                        .with_style(theme.owner_username.container)
                        .boxed(),
                    )
                    .with_child(
                        Label::new(
                            "has shared a project with you".into(),
                            theme.message.text.clone(),
                        )
                        .contained()
                        .with_style(theme.message.container)
                        .boxed(),
                    )
                    .contained()
                    .with_style(theme.owner_metadata)
                    .aligned()
                    .boxed(),
            )
            .contained()
            .with_style(theme.owner_container)
            .flex(1., true)
            .boxed()
    }

    fn render_buttons(&self, cx: &mut RenderContext<Self>) -> ElementBox {
        enum Join {}
        enum Dismiss {}

        let project_id = self.project_id;
        let owner_user_id = self.owner.id;

        Flex::column()
            .with_child(
                MouseEventHandler::<Join>::new(0, cx, |_, cx| {
                    let theme = &cx.global::<Settings>().theme.project_shared_notification;
                    Label::new("Join".to_string(), theme.join_button.text.clone())
                        .aligned()
                        .contained()
                        .with_style(theme.join_button.container)
                        .boxed()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, cx| {
                    cx.dispatch_action(JoinProject {
                        project_id,
                        follow_user_id: owner_user_id,
                    });
                })
                .flex(1., true)
                .boxed(),
            )
            .with_child(
                MouseEventHandler::<Dismiss>::new(0, cx, |_, cx| {
                    let theme = &cx.global::<Settings>().theme.project_shared_notification;
                    Label::new("Dismiss".to_string(), theme.dismiss_button.text.clone())
                        .aligned()
                        .contained()
                        .with_style(theme.dismiss_button.container)
                        .boxed()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, |_, cx| {
                    cx.dispatch_action(DismissProject);
                })
                .flex(1., true)
                .boxed(),
            )
            .constrained()
            .with_width(
                cx.global::<Settings>()
                    .theme
                    .project_shared_notification
                    .button_width,
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
        let background = cx
            .global::<Settings>()
            .theme
            .project_shared_notification
            .background;
        Flex::row()
            .with_child(self.render_owner(cx))
            .with_child(self.render_buttons(cx))
            .contained()
            .with_background_color(background)
            .expanded()
            .boxed()
    }
}
