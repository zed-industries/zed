use acp_thread::MentionUri;
use agent::DbThreadMetadata;
use editor::scroll::Autoscroll;
use editor::{Editor, SelectionEffects};
use gpui::{App, WeakEntity, Window};
use prompt_store::PromptId;
use rope::Point;
use workspace::Workspace;
use zed_actions::assistant::OpenRulesLibrary;

use crate::AgentPanel;

pub trait MentionUriExt {
    fn open(&self, workspace: &WeakEntity<Workspace>, window: &mut Window, cx: &mut App) -> bool;
}

impl MentionUriExt for MentionUri {
    fn open(&self, workspace: &WeakEntity<Workspace>, window: &mut Window, cx: &mut App) -> bool {
        let Some(workspace) = workspace.upgrade() else {
            return false;
        };

        match self.clone() {
            MentionUri::File { abs_path } => {
                workspace.update(cx, |workspace, cx| {
                    let project = workspace.project();
                    let Some(path) =
                        project.update(cx, |project, cx| project.find_project_path(abs_path, cx))
                    else {
                        return;
                    };

                    workspace
                        .open_path(path, None, true, window, cx)
                        .detach_and_log_err(cx);
                });
                true
            }
            MentionUri::PastedImage => true,
            MentionUri::Directory { abs_path } => {
                workspace.update(cx, |workspace, cx| {
                    let project = workspace.project();
                    let Some(entry_id) = project.update(cx, |project, cx| {
                        let path = project.find_project_path(abs_path, cx)?;
                        project.entry_for_path(&path, cx).map(|entry| entry.id)
                    }) else {
                        return;
                    };

                    project.update(cx, |_, cx| {
                        cx.emit(project::Event::RevealInProjectPanel(entry_id));
                    });
                });
                true
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
                workspace.update(cx, |workspace, cx| {
                    let project = workspace.project();
                    let Some(path) =
                        project.update(cx, |project, cx| project.find_project_path(path, cx))
                    else {
                        return;
                    };

                    let item = workspace.open_path(path, None, true, window, cx);
                    let start_line = *line_range.start();
                    window
                        .spawn(cx, async move |cx| {
                            let Some(editor) = item.await?.downcast::<Editor>() else {
                                return Ok(());
                            };
                            let range = Point::new(start_line, 0)..Point::new(start_line, 0);
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
                });
                true
            }
            MentionUri::Selection { abs_path: None, .. } => true,
            MentionUri::Thread { id, name } => {
                workspace.update(cx, |workspace, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        panel.update(cx, |panel, cx| {
                            panel.load_agent_thread(
                                DbThreadMetadata {
                                    id: id.clone(),
                                    title: name.clone().into(),
                                    updated_at: Default::default(),
                                },
                                window,
                                cx,
                            )
                        });
                    }
                });
                true
            }
            MentionUri::TextThread { path, .. } => {
                workspace.update(cx, |workspace, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        panel.update(cx, |panel, cx| {
                            panel
                                .open_saved_text_thread(path.as_path().into(), window, cx)
                                .detach_and_log_err(cx);
                        });
                    }
                });
                true
            }
            MentionUri::Rule { id, .. } => {
                if let PromptId::User { uuid } = id.clone() {
                    window.dispatch_action(
                        Box::new(OpenRulesLibrary {
                            prompt_to_select: Some(uuid.0),
                        }),
                        cx,
                    )
                }
                true
            }
            MentionUri::Fetch { url } => {
                workspace.update(cx, |_workspace, cx| {
                    cx.open_url(url.as_str());
                });
                true
            }
        }
    }
}
