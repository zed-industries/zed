use auto_update::{AutoUpdater, release_notes_url};
use editor::{Editor, MultiBuffer};
use gpui::{
    App, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Window, actions, prelude::*,
};
use markdown_preview::markdown_preview_view::{MarkdownPreviewMode, MarkdownPreviewView};
use release_channel::{AppVersion, ReleaseChannel};
use semver::Version;
use serde::Deserialize;
use smol::io::AsyncReadExt;
use ui::{AnnouncementToast, ListBulletItem, prelude::*};
use util::{ResultExt as _, maybe};
use workspace::{
    Workspace,
    notifications::{
        ErrorMessagePrompt, Notification, NotificationId, SuppressEvent, show_app_notification,
        simple_message_notification::MessageNotification,
    },
};

actions!(
    auto_update,
    [
        /// Opens the release notes for the current version in a new tab.
        ViewReleaseNotesLocally
    ]
);

pub fn init(cx: &mut App) {
    notify_if_app_was_updated(cx);
    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(|workspace, _: &ViewReleaseNotesLocally, window, cx| {
            view_release_notes_locally(workspace, window, cx);
        });
    })
    .detach();
}

#[derive(Deserialize)]
struct ReleaseNotesBody {
    title: String,
    release_notes: String,
}

fn notify_release_notes_failed_to_show(
    workspace: &mut Workspace,
    _window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    struct ViewReleaseNotesError;
    workspace.show_notification(
        NotificationId::unique::<ViewReleaseNotesError>(),
        cx,
        |cx| {
            cx.new(move |cx| {
                let url = release_notes_url(cx);
                let mut prompt = ErrorMessagePrompt::new("Couldn't load release notes", cx);
                if let Some(url) = url {
                    prompt = prompt.with_link_button("View in Browser".to_string(), url);
                }
                prompt
            })
        },
    );
}

fn view_release_notes_locally(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let release_channel = ReleaseChannel::global(cx);

    if matches!(
        release_channel,
        ReleaseChannel::Nightly | ReleaseChannel::Dev
    ) {
        if let Some(url) = release_notes_url(cx) {
            cx.open_url(&url);
        }
        return;
    }

    let version = AppVersion::global(cx).to_string();

    let client = client::Client::global(cx).http_client();
    let url = client.build_url(&format!(
        "/api/release_notes/v2/{}/{}",
        release_channel.dev_name(),
        version
    ));

    let markdown = workspace
        .app_state()
        .languages
        .language_for_name("Markdown");

    cx.spawn_in(window, async move |workspace, cx| {
        let markdown = markdown.await.log_err();
        let response = client.get(&url, Default::default(), true).await;
        let Some(mut response) = response.log_err() else {
            workspace
                .update_in(cx, notify_release_notes_failed_to_show)
                .log_err();
            return;
        };

        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await.ok();

        let body: serde_json::Result<ReleaseNotesBody> = serde_json::from_slice(body.as_slice());

        let res: Option<()> = maybe!(async {
            let body = body.ok()?;
            let project = workspace
                .read_with(cx, |workspace, _| workspace.project().clone())
                .ok()?;
            let (language_registry, buffer) = project.update(cx, |project, cx| {
                (
                    project.languages().clone(),
                    project.create_buffer(markdown, false, cx),
                )
            });
            let buffer = buffer.await.ok()?;
            buffer.update(cx, |buffer, cx| {
                buffer.edit([(0..0, body.release_notes)], None, cx)
            });

            let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx).with_title(body.title));

            let ws_handle = workspace.clone();
            workspace
                .update_in(cx, |workspace, window, cx| {
                    let editor =
                        cx.new(|cx| Editor::for_multibuffer(buffer, Some(project), window, cx));
                    let markdown_preview: Entity<MarkdownPreviewView> = MarkdownPreviewView::new(
                        MarkdownPreviewMode::Default,
                        editor,
                        ws_handle,
                        language_registry,
                        window,
                        cx,
                    );
                    workspace.add_item_to_active_pane(
                        Box::new(markdown_preview),
                        None,
                        true,
                        window,
                        cx,
                    );
                    cx.notify();
                })
                .ok()
        })
        .await;
        if res.is_none() {
            workspace
                .update_in(cx, notify_release_notes_failed_to_show)
                .log_err();
        }
    })
    .detach();
}

#[derive(Clone)]
struct AnnouncementContent {
    heading: SharedString,
    description: SharedString,
    bullet_items: Vec<SharedString>,
    primary_action_label: SharedString,
    primary_action_url: Option<SharedString>,
}

fn announcement_for_version(version: &Version) -> Option<AnnouncementContent> {
    #[allow(clippy::match_single_binding)]
    match (version.major, version.minor, version.patch) {
        // TODO: Add real version when we have it
        // (0, 225, 0) => Some(AnnouncementContent {
        //     heading: "What's new in Zed 0.225".into(),
        //     description: "This release includes some exciting improvements.".into(),
        //     bullet_items: vec![
        //         "Improved agent performance".into(),
        //         "New agentic features".into(),
        //         "Better agent capabilities".into(),
        //     ],
        //     primary_action_label: "Learn More".into(),
        //     primary_action_url: Some("https://zed.dev/".into()),
        // }),
        _ => None,
    }
}

struct AnnouncementToastNotification {
    focus_handle: FocusHandle,
    content: AnnouncementContent,
}

impl AnnouncementToastNotification {
    fn new(content: AnnouncementContent, cx: &mut App) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            content,
        }
    }
}

impl Focusable for AnnouncementToastNotification {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for AnnouncementToastNotification {}
impl EventEmitter<SuppressEvent> for AnnouncementToastNotification {}
impl Notification for AnnouncementToastNotification {}

impl Render for AnnouncementToastNotification {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        AnnouncementToast::new()
            .heading(self.content.heading.clone())
            .description(self.content.description.clone())
            .bullet_items(
                self.content
                    .bullet_items
                    .iter()
                    .map(|item| ListBulletItem::new(item.clone())),
            )
            .primary_action_label(self.content.primary_action_label.clone())
            .primary_on_click(cx.listener({
                let url = self.content.primary_action_url.clone();
                move |_, _, _window, cx| {
                    if let Some(url) = &url {
                        cx.open_url(url);
                    }
                    cx.emit(DismissEvent);
                }
            }))
            .secondary_on_click(cx.listener({
                let url = self.content.primary_action_url.clone();
                move |_, _, _window, cx| {
                    if let Some(url) = &url {
                        cx.open_url(url);
                    }
                    cx.emit(DismissEvent);
                }
            }))
            .dismiss_on_click(cx.listener(|_, _, _window, cx| {
                cx.emit(DismissEvent);
            }))
    }
}

/// Shows a notification across all workspaces if an update was previously automatically installed
/// and this notification had not yet been shown.
pub fn notify_if_app_was_updated(cx: &mut App) {
    let Some(updater) = AutoUpdater::get(cx) else {
        return;
    };

    if let ReleaseChannel::Nightly = ReleaseChannel::global(cx) {
        return;
    }

    struct UpdateNotification;

    let should_show_notification = updater.read(cx).should_show_update_notification(cx);
    cx.spawn(async move |cx| {
        let should_show_notification = should_show_notification.await?;
        // if true { // Hardcode it to true for testing it outside of the component preview
        if should_show_notification {
            cx.update(|cx| {
                let mut version = updater.read(cx).current_version();
                version.build = semver::BuildMetadata::EMPTY;
                version.pre = semver::Prerelease::EMPTY;
                let app_name = ReleaseChannel::global(cx).display_name();

                if let Some(content) = announcement_for_version(&version) {
                    show_app_notification(
                        NotificationId::unique::<UpdateNotification>(),
                        cx,
                        move |cx| {
                            cx.new(|cx| AnnouncementToastNotification::new(content.clone(), cx))
                        },
                    );
                } else {
                    show_app_notification(
                        NotificationId::unique::<UpdateNotification>(),
                        cx,
                        move |cx| {
                            let workspace_handle = cx.entity().downgrade();
                            cx.new(|cx| {
                                MessageNotification::new(
                                    format!("Updated to {app_name} {}", version),
                                    cx,
                                )
                                .primary_message("View Release Notes")
                                .primary_on_click(move |window, cx| {
                                    if let Some(workspace) = workspace_handle.upgrade() {
                                        workspace.update(cx, |workspace, cx| {
                                            crate::view_release_notes_locally(
                                                workspace, window, cx,
                                            );
                                        })
                                    }
                                    cx.emit(DismissEvent);
                                })
                                .show_suppress_button(false)
                            })
                        },
                    );
                }

                updater.update(cx, |updater, cx| {
                    updater
                        .set_should_show_update_notification(false, cx)
                        .detach_and_log_err(cx);
                });
            });
        }
        anyhow::Ok(())
    })
    .detach();
}
