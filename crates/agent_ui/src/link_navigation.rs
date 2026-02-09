use acp_thread::{AgentSessionInfo, MentionUri};
use editor::scroll::Autoscroll;
use editor::{Editor, SelectionEffects};
use gpui::{App, SharedString, WeakEntity, Window};
use project::Event as ProjectEvent;
use prompt_store::PromptId;
use rope::Point;
use util::ResultExt;
use workspace::Workspace;
use zed_actions::assistant::OpenRulesLibrary;

use crate::AgentPanel;

pub(crate) fn open_workspace_link(
    url: SharedString,
    workspace: &WeakEntity<Workspace>,
    window: &mut Window,
    cx: &mut App,
) {
    let Some(workspace) = workspace.upgrade() else {
        cx.open_url(&url);
        return;
    };

    if let Some(mention) = MentionUri::parse(&url, workspace.read(cx).path_style(cx)).log_err() {
        open_workspace_mention(&mention, &workspace.downgrade(), window, cx);
    } else {
        cx.open_url(url.as_ref());
    }
}

pub(crate) fn open_workspace_mention(
    mention: &MentionUri,
    workspace: &WeakEntity<Workspace>,
    window: &mut Window,
    cx: &mut App,
) {
    let Some(workspace) = workspace.upgrade() else {
        return;
    };
    let mention = mention.clone();
    workspace.update(cx, move |workspace, cx| match mention {
        MentionUri::File { abs_path } => {
            let project = workspace.project();
            let Some(path) =
                project.update(cx, |project, cx| project.find_project_path(abs_path, cx))
            else {
                return;
            };

            workspace
                .open_path(path, None, true, window, cx)
                .detach_and_log_err(cx);
        }
        MentionUri::PastedImage => {}
        MentionUri::Directory { abs_path } => {
            let project = workspace.project();
            let Some(entry_id) = project.update(cx, |project, cx| {
                let path = project.find_project_path(abs_path, cx)?;
                project.entry_for_path(&path, cx).map(|entry| entry.id)
            }) else {
                return;
            };

            project.update(cx, |_, cx| {
                cx.emit(ProjectEvent::RevealInProjectPanel(entry_id));
            });
        }
        MentionUri::Symbol {
            abs_path: path,
            line_range,
            ..
        }
        | MentionUri::Selection {
            abs_path: Some(path),
            line_range,
        } => {
            let project = workspace.project();
            let Some(path) = project.update(cx, |project, cx| project.find_project_path(path, cx))
            else {
                return;
            };

            let item = workspace.open_path(path, None, true, window, cx);
            window
                .spawn(cx, async move |cx| {
                    let Some(editor) = item.await?.downcast::<Editor>() else {
                        return Ok(());
                    };
                    let range =
                        Point::new(*line_range.start(), 0)..Point::new(*line_range.start(), 0);
                    editor
                        .update_in(cx, |editor, window, cx| {
                            editor.change_selections(
                                SelectionEffects::scroll(Autoscroll::center()),
                                window,
                                cx,
                                |s| s.select_ranges(vec![range]),
                            );
                        })
                        .ok();
                    anyhow::Ok(())
                })
                .detach_and_log_err(cx);
        }
        MentionUri::Selection { abs_path: None, .. } => {}
        MentionUri::Thread { id, name } => {
            if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                panel.update(cx, |panel, cx| {
                    panel.open_thread(
                        AgentSessionInfo {
                            session_id: id,
                            cwd: None,
                            title: Some(name.into()),
                            updated_at: None,
                            meta: None,
                        },
                        window,
                        cx,
                    )
                });
            }
        }
        MentionUri::TextThread { path, .. } => {
            if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                panel.update(cx, |panel, cx| {
                    panel
                        .open_saved_text_thread(path.as_path().into(), window, cx)
                        .detach_and_log_err(cx);
                });
            }
        }
        MentionUri::Rule { id, .. } => {
            let PromptId::User { uuid } = id else {
                return;
            };
            window.dispatch_action(
                Box::new(OpenRulesLibrary {
                    prompt_to_select: Some(uuid.0),
                }),
                cx,
            );
        }
        MentionUri::Fetch { url } => {
            cx.open_url(url.as_str());
        }
        MentionUri::Diagnostics { .. } => {}
        MentionUri::TerminalSelection { .. } => {}
    });
}
