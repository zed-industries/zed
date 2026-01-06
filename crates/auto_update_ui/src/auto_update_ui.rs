use auto_update::{AutoUpdater, release_notes_url};
use editor::{Editor, MultiBuffer};
use gpui::{App, Context, DismissEvent, Entity, Window, actions, prelude::*};
use markdown_preview::markdown_preview_view::{MarkdownPreviewMode, MarkdownPreviewView};
use release_channel::{AppVersion, ReleaseChannel};
use serde::Deserialize;
use smol::io::AsyncReadExt;
use util::ResultExt as _;
use workspace::Workspace;
use workspace::notifications::ErrorMessagePrompt;
use workspace::notifications::simple_message_notification::MessageNotification;
use workspace::notifications::{NotificationId, show_app_notification};

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
    #[expect(
        unused,
        reason = "This field was found to be unused with serde library bump; it's left as is due to insufficient context on PO's side, but it *may* be fine to remove"
    )]
    title: String,
    release_notes: String,
}

fn notify_release_notes_failed_to_show_locally(
    workspace: &mut Workspace,
    _window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    struct ViewReleaseNotesLocallyError;
    workspace.show_notification(
        NotificationId::unique::<ViewReleaseNotesLocallyError>(),
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

    workspace
        .with_local_workspace(window, cx, move |_, window, cx| {
            cx.spawn_in(window, async move |workspace, cx| {
                let markdown = markdown.await.log_err();
                let response = client.get(&url, Default::default(), true).await;
                let Some(mut response) = response.log_err() else {
                    workspace
                        .update_in(cx, notify_release_notes_failed_to_show_locally)
                        .log_err();
                    return;
                };

                let mut body = Vec::new();
                response.body_mut().read_to_end(&mut body).await.ok();

                let body: serde_json::Result<ReleaseNotesBody> =
                    serde_json::from_slice(body.as_slice());

                if let Ok(body) = body {
                    workspace
                        .update_in(cx, |workspace, window, cx| {
                            let project = workspace.project().clone();
                            let buffer = project.update(cx, |project, cx| {
                                project.create_local_buffer("", markdown, false, cx)
                            });
                            buffer.update(cx, |buffer, cx| {
                                buffer.edit([(0..0, body.release_notes)], None, cx)
                            });
                            let language_registry = project.read(cx).languages().clone();

                            let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

                            let editor = cx.new(|cx| {
                                Editor::for_multibuffer(buffer, Some(project), window, cx)
                            });
                            let workspace_handle = workspace.weak_handle();
                            let markdown_preview: Entity<MarkdownPreviewView> =
                                MarkdownPreviewView::new(
                                    MarkdownPreviewMode::Default,
                                    editor,
                                    workspace_handle,
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
                        .log_err();
                } else {
                    workspace
                        .update_in(cx, notify_release_notes_failed_to_show_locally)
                        .log_err();
                }
            })
            .detach();
        })
        .detach();
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
        if should_show_notification {
            cx.update(|cx| {
                let mut version = updater.read(cx).current_version();
                version.build = semver::BuildMetadata::EMPTY;
                version.pre = semver::Prerelease::EMPTY;
                let app_name = ReleaseChannel::global(cx).display_name();
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
                                        crate::view_release_notes_locally(workspace, window, cx);
                                    })
                                }
                                cx.emit(DismissEvent);
                            })
                            .show_suppress_button(false)
                        })
                    },
                );
                updater.update(cx, |updater, cx| {
                    updater
                        .set_should_show_update_notification(false, cx)
                        .detach_and_log_err(cx);
                })
            })?;
        }
        anyhow::Ok(())
    })
    .detach();
}
