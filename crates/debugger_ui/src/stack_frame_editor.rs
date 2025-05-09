use editor::{Editor, ExcerptRange, MultiBuffer};
use gpui::{AppContext, Entity, WeakEntity, Window};
use language::{Capability, Point};
use project::{Project, ProjectPath};
use ui::Context;
use util::ResultExt as _;
use workspace::Workspace;

use crate::session::{DebugSession, running::stack_frame_list::StackFrameList};

struct StackFrameEditor {
    editor: Entity<Editor>,
    multibuffer: Entity<MultiBuffer>,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    active_session: Option<Entity<DebugSession>>,
}

impl StackFrameEditor {
    fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));
        let editor = cx.new(|cx| {
            let mut editor =
                Editor::for_multibuffer(multibuffer.clone(), Some(project.clone()), window, cx);
            editor.set_vertical_scroll_margin(5, cx);
            editor
        });

        Self {
            editor,
            multibuffer,
            workspace,
            project,
            active_session: None,
        }
    }

    fn update_excerpts(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(session) = self.active_session.clone() else {
            return;
        };

        let Some(thread_id) = session
            .read(cx)
            .running_state()
            .read(cx)
            .selected_thread_id()
        else {
            return;
        };

        let stack_frames = session.update(cx, |session, cx| {
            session.running_state().update(cx, |state, cx| {
                state
                    .session()
                    .update(cx, |session, cx| session.stack_frames(thread_id, cx))
            })
        });

        let frames_to_open: Vec<_> = stack_frames
            .into_iter()
            .filter_map(|frame| {
                Some((
                    frame.dap.line,
                    StackFrameList::abs_path_from_stack_frame(&frame.dap)?,
                ))
            })
            .collect();

        cx.spawn_in(window, async move |this, cx| {
            for (line, abs_path) in frames_to_open {
                let (worktree, relative_path) = this
                    .update(cx, |this, cx| {
                        this.workspace.update(cx, |workspace, cx| {
                            workspace.project().update(cx, |this, cx| {
                                this.find_or_create_worktree(&abs_path, false, cx)
                            })
                        })
                    })??
                    .await?;

                let project_path = ProjectPath {
                    worktree_id: worktree.read_with(cx, |tree, _| tree.id())?,
                    path: relative_path.into(),
                };

                if let Some(buffer) = this
                    .read_with(cx, |this, _| this.project.clone())?
                    .update(cx, |project, cx| project.open_buffer(project_path, cx))?
                    .await
                    .log_err()
                {
                    this.update(cx, |this, cx| {
                        this.multibuffer.update(cx, |multi_buffer, cx| {
                            let range = ExcerptRange {
                                context: Point::new((line as u32).saturating_sub(4), 0)
                                    ..Point::new((line as u32).saturating_add(4), 0),
                                primary: Point::new(line as u32, 0)..Point::new(line as u32, 10),
                            };
                            multi_buffer.push_excerpts(buffer, vec![range], cx);
                        })
                    })
                    .ok();
                }
            }

            anyhow::Ok(())
        })
        .detach();
    }
}
