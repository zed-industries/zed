mod update_notification;

use auto_update::AutoUpdater;
use editor::{Editor, MultiBuffer};
use gpui::{actions, prelude::*, AppContext, SharedString, View, ViewContext};
use http_client::HttpClient;
use markdown_preview::markdown_preview_view::{MarkdownPreviewMode, MarkdownPreviewView};
use release_channel::{AppVersion, ReleaseChannel};
use serde::Deserialize;
use smol::io::AsyncReadExt;
use util::ResultExt as _;
use workspace::notifications::NotificationId;
use workspace::Workspace;

use crate::update_notification::UpdateNotification;

actions!(auto_update, [ViewReleaseNotesLocally]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, _cx| {
        workspace.register_action(|workspace, _: &ViewReleaseNotesLocally, cx| {
            view_release_notes_locally(workspace, cx);
        });
    })
    .detach();
}

#[derive(Deserialize)]
struct ReleaseNotesBody {
    title: String,
    release_notes: String,
}

fn view_release_notes_locally(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
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
        .with_local_workspace(cx, move |_, cx| {
            cx.spawn(|workspace, mut cx| async move {
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
                        .update(&mut cx, |workspace, cx| {
                            let project = workspace.project().clone();
                            let buffer = project.update(cx, |project, cx| {
                                project.create_local_buffer("", markdown, cx)
                            });
                            buffer.update(cx, |buffer, cx| {
                                buffer.edit([(0..0, body.release_notes)], None, cx)
                            });
                            let language_registry = project.read(cx).languages().clone();

                            let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));

                            let tab_description = SharedString::from(body.title.to_string());
                            let editor = cx.new_view(|cx| {
                                Editor::for_multibuffer(buffer, Some(project), true, cx)
                            });
                            let workspace_handle = workspace.weak_handle();
                            let view: View<MarkdownPreviewView> = MarkdownPreviewView::new(
                                MarkdownPreviewMode::Default,
                                editor,
                                workspace_handle,
                                language_registry,
                                Some(tab_description),
                                cx,
                            );
                            workspace.add_item_to_active_pane(
                                Box::new(view.clone()),
                                None,
                                true,
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

pub fn notify_of_any_new_update(cx: &mut ViewContext<Workspace>) -> Option<()> {
    let updater = AutoUpdater::get(cx)?;
    let version = updater.read(cx).current_version();
    let should_show_notification = updater.read(cx).should_show_update_notification(cx);

    cx.spawn(|workspace, mut cx| async move {
        let should_show_notification = should_show_notification.await?;
        if should_show_notification {
            workspace.update(&mut cx, |workspace, cx| {
                let workspace_handle = workspace.weak_handle();
                workspace.show_notification(
                    NotificationId::unique::<UpdateNotification>(),
                    cx,
                    |cx| cx.new_view(|_| UpdateNotification::new(version, workspace_handle)),
                );
                updater.update(cx, |updater, cx| {
                    updater
                        .set_should_show_update_notification(false, cx)
                        .detach_and_log_err(cx);
                });
            })?;
        }
        anyhow::Ok(())
    })
    .detach();

    None
}
