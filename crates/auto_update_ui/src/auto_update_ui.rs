use auto_update::AutoUpdater;
use client::proto::UpdateNotification;
use editor::{Editor, MultiBuffer};
use gpui::{actions, prelude::*, App, Context, DismissEvent, Entity, SharedString, Window};
use http_client::HttpClient;
use markdown_preview::markdown_preview_view::{MarkdownPreviewMode, MarkdownPreviewView};
use release_channel::{AppVersion, ReleaseChannel};
use serde::Deserialize;
use smol::io::AsyncReadExt;
use util::ResultExt as _;
use workspace::notifications::simple_message_notification::MessageNotification;
use workspace::notifications::{show_app_notification, NotificationId};
use workspace::Workspace;

actions!(auto_update, [ViewReleaseNotesLocally]);

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

fn view_release_notes_locally(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let release_channel = ReleaseChannel::global(cx);

    let url = match release_channel {
        ReleaseChannel::Nightly => Some("https://github.com/zed-industries/zed/commits/nightly/"),
        ReleaseChannel::Dev => Some("https://github.com/zed-industries/zed/commits/main/"),
        _ => None,
    };

    if let Some(url) = url {
        cx.open_url(url);
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
            cx.spawn_in(window, |workspace, mut cx| async move {
                let markdown = markdown.await.log_err();
                let response = client.get(&url, Default::default(), true).await;
                let Some(mut response) = response.log_err() else {
                    return;
                };

                let mut body = Vec::new();
                response.body_mut().read_to_end(&mut body).await.ok();

                let body: serde_json::Result<ReleaseNotesBody> =
                    serde_json::from_slice(body.as_slice());

                if let Ok(body) = body {
                    workspace
                        .update_in(&mut cx, |workspace, window, cx| {
                            let project = workspace.project().clone();
                            let buffer = project.update(cx, |project, cx| {
                                project.create_local_buffer("", markdown, cx)
                            });
                            buffer.update(cx, |buffer, cx| {
                                buffer.edit([(0..0, body.release_notes)], None, cx)
                            });
                            let language_registry = project.read(cx).languages().clone();

                            let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

                            let tab_description = SharedString::from(body.title.to_string());
                            let editor = cx.new(|cx| {
                                Editor::for_multibuffer(buffer, Some(project), true, window, cx)
                            });
                            let workspace_handle = workspace.weak_handle();
                            let markdown_preview: Entity<MarkdownPreviewView> =
                                MarkdownPreviewView::new(
                                    MarkdownPreviewMode::Default,
                                    editor,
                                    workspace_handle,
                                    language_registry,
                                    Some(tab_description),
                                    window,
                                    cx,
                                );
                            workspace.add_item_to_active_pane(
                                Box::new(markdown_preview.clone()),
                                None,
                                true,
                                window,
                                cx,
                            );
                            cx.notify();
                        })
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
    let should_show_notification = updater.read(cx).should_show_update_notification(cx);
    cx.spawn(|cx| async move {
        let should_show_notification = should_show_notification.await?;
        if should_show_notification {
            cx.update(|cx| {
                let version = updater.read(cx).current_version();
                let app_name = ReleaseChannel::global(cx).display_name();
                show_app_notification(
                    NotificationId::unique::<UpdateNotification>(),
                    cx,
                    move |cx| {
                        let workspace_handle = cx.entity().downgrade();
                        cx.new(|_cx| {
                            MessageNotification::new(format!("Updated to {app_name} {}", version))
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
