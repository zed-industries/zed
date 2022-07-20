mod color_translation;
pub mod connection;
mod modal;
pub mod terminal_element;

use connection::{DisconnectedPTY, Event, Terminal, TerminalError};
use dirs::home_dir;
use gpui::{
    actions, elements::*, geometry::vector::vec2f, keymap::Keystroke, AppContext, ClipboardItem,
    Entity, ModelHandle, MutableAppContext, View, ViewContext,
};
use modal::deploy_modal;

use project::{LocalWorktree, Project, ProjectPath};
use settings::{Settings, WorkingDirectory};
use smallvec::SmallVec;
use std::path::{Path, PathBuf};
use terminal_element::TerminalDimensions;

use workspace::{Item, Workspace};

use crate::terminal_element::TerminalEl;

const DEBUG_TERMINAL_WIDTH: f32 = 1000.; //This needs to be wide enough that the prompt can fill the whole space.
const DEBUG_TERMINAL_HEIGHT: f32 = 200.;
const DEBUG_CELL_WIDTH: f32 = 5.;
const DEBUG_LINE_HEIGHT: f32 = 5.;

///Event to transmit the scroll from the element to the view
#[derive(Clone, Debug, PartialEq)]
pub struct ScrollTerminal(pub i32);

actions!(
    terminal,
    [
        Deploy,
        Up,
        Down,
        CtrlC,
        Escape,
        Enter,
        Clear,
        Copy,
        Paste,
        DeployModal
    ]
);

///Initialize and register all of our action handlers
pub fn init(cx: &mut MutableAppContext) {
    //Global binding overrrides
    cx.add_action(TerminalView::ctrl_c);
    cx.add_action(TerminalView::up);
    cx.add_action(TerminalView::down);
    cx.add_action(TerminalView::escape);
    cx.add_action(TerminalView::enter);
    //Useful terminal actions
    cx.add_action(TerminalView::deploy);
    cx.add_action(deploy_modal);
    cx.add_action(TerminalView::copy);
    cx.add_action(TerminalView::paste);
    cx.add_action(TerminalView::clear);
}

///A terminal view, maintains the PTY's file handles and communicates with the terminal
pub struct TerminalView {
    connection: Result<ModelHandle<Terminal>, TerminalError>,
    has_new_content: bool,
    //Currently using iTerm bell, show bell emoji in tab until input is received
    has_bell: bool,
    // Only for styling purposes. Doesn't effect behavior
    modal: bool,
}

impl Entity for TerminalView {
    type Event = Event;
}

impl TerminalView {
    ///Create a new Terminal view. This spawns a task, a thread, and opens the TTY devices
    ///To get the right working directory from a workspace, use: `get_wd_for_workspace()`
    fn new(working_directory: Option<PathBuf>, modal: bool, cx: &mut ViewContext<Self>) -> Self {
        //The details here don't matter, the terminal will be resized on the first layout
        let size_info = TerminalDimensions::new(
            DEBUG_LINE_HEIGHT,
            DEBUG_CELL_WIDTH,
            vec2f(DEBUG_TERMINAL_WIDTH, DEBUG_TERMINAL_HEIGHT),
        );

        let (shell, envs) = {
            let settings = cx.global::<Settings>();
            let shell = settings.terminal_overrides.shell.clone();
            let envs = settings.terminal_overrides.env.clone(); //Should be short and cheap.
            (shell, envs)
        };

        let connection = DisconnectedPTY::new(working_directory, shell, envs, size_info)
            .map(|pty| cx.add_model(|cx| pty.connect(cx)))
            .map_err(|err| {
                match err.downcast::<TerminalError>() {
                    Ok(err) => err,
                    Err(_) => unreachable!(), //This should never happen
                }
            });

        TerminalView::from_connection(connection, modal, cx)
    }

    fn from_connection(
        connection: Result<ModelHandle<Terminal>, TerminalError>,
        modal: bool,
        cx: &mut ViewContext<Self>,
    ) -> TerminalView {
        cx.observe(&connection, |_, _, cx| cx.notify()).detach();
        cx.subscribe(&connection, |this, _, event, cx| match event {
            Event::Wakeup => {
                if cx.is_self_focused() {
                    cx.notify()
                } else {
                    this.has_new_content = true;
                    cx.emit(Event::TitleChanged);
                }
            }
            Event::Bell => {
                this.has_bell = true;
                cx.emit(Event::TitleChanged);
            }
            // Event::Input => {
            //     this.has_bell = false;
            //     cx.emit(Event::TitleChanged);
            // }
            _ => cx.emit(*event),
        })
        .detach();

        TerminalView {
            connection,
            has_new_content: true,
            has_bell: false,
            modal,
        }
    }

    fn clear_bel(&mut self, cx: &mut ViewContext<Self>) {
        self.has_bell = false;
        cx.emit(Event::TitleChanged);
    }

    fn clear(&mut self, _: &Clear, cx: &mut ViewContext<Self>) {
        self.connection
            .read(cx)
            .get_terminal()
            .map(|term| term.clear());
    }

    ///Create a new Terminal in the current working directory or the user's home directory
    fn deploy(workspace: &mut Workspace, _: &Deploy, cx: &mut ViewContext<Workspace>) {
        let wd = get_wd_for_workspace(workspace, cx);
        let view = cx.add_view(|cx| TerminalView::new(wd, false, cx));
        workspace.add_item(Box::new(view), cx);
    }

    ///Attempt to paste the clipboard into the terminal
    fn copy(&mut self, _: &Copy, cx: &mut ViewContext<Self>) {
        self.connection
            .read(cx)
            .get_terminal()
            .and_then(|term| term.copy())
            .map(|text| cx.write_to_clipboard(ClipboardItem::new(text)));
    }

    ///Attempt to paste the clipboard into the terminal
    fn paste(&mut self, _: &Paste, cx: &mut ViewContext<Self>) {
        cx.read_from_clipboard().map(|item| {
            self.connection
                .read(cx)
                .get_terminal()
                .map(|term| term.paste(item.text()));
        });
    }

    ///Synthesize the keyboard event corresponding to 'up'
    fn up(&mut self, _: &Up, cx: &mut ViewContext<Self>) {
        self.connection
            .read(cx)
            .get_terminal()
            .map(|term| term.try_keystroke(&Keystroke::parse("up").unwrap()));
    }

    ///Synthesize the keyboard event corresponding to 'down'
    fn down(&mut self, _: &Down, cx: &mut ViewContext<Self>) {
        self.connection
            .read(cx)
            .get_terminal()
            .map(|term| term.try_keystroke(&Keystroke::parse("down").unwrap()));
    }

    ///Synthesize the keyboard event corresponding to 'ctrl-c'
    fn ctrl_c(&mut self, _: &CtrlC, cx: &mut ViewContext<Self>) {
        self.connection
            .read(cx)
            .get_terminal()
            .map(|term| term.try_keystroke(&Keystroke::parse("ctrl-c").unwrap()));
    }

    ///Synthesize the keyboard event corresponding to 'escape'
    fn escape(&mut self, _: &Escape, cx: &mut ViewContext<Self>) {
        self.connection
            .read(cx)
            .get_terminal()
            .map(|term| term.try_keystroke(&Keystroke::parse("escape").unwrap()));
    }

    ///Synthesize the keyboard event corresponding to 'enter'
    fn enter(&mut self, _: &Enter, cx: &mut ViewContext<Self>) {
        self.connection
            .read(cx)
            .get_terminal()
            .map(|term| term.try_keystroke(&Keystroke::parse("enter").unwrap()));
    }
}

impl View for TerminalView {
    fn ui_name() -> &'static str {
        "Terminal"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> ElementBox {
        let element = {
            let connection_handle = self.connection.clone().downgrade();
            TerminalEl::new(cx.handle(), connection_handle, self.modal).contained()
        };

        if self.modal {
            let settings = cx.global::<Settings>();
            let container_style = settings.theme.terminal.modal_container;
            element.with_style(container_style).boxed()
        } else {
            element.boxed()
        }
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Activate);
        self.has_new_content = false;
    }

    fn keymap_context(&self, _: &gpui::AppContext) -> gpui::keymap::Context {
        let mut context = Self::default_keymap_context();
        if self.modal {
            context.set.insert("ModalTerminal".into());
        }
        context
    }
}

impl Item for TerminalView {
    fn tab_content(
        &self,
        _detail: Option<usize>,
        tab_theme: &theme::Tab,
        cx: &gpui::AppContext,
    ) -> ElementBox {
        let title = match self.connection.read(cx) {
            TerminalConnection::Connected(conn) => conn.title.clone(),
            TerminalConnection::Disconnected { .. } => "Terminal".to_string(), //TODO ask nate about htis
        };

        Flex::row()
            .with_child(
                Label::new(title, tab_theme.label.clone())
                    .aligned()
                    .contained()
                    .boxed(),
            )
            .boxed()
    }

    fn clone_on_split(&self, cx: &mut ViewContext<Self>) -> Option<Self> {
        //From what I can tell, there's no  way to tell the current working
        //Directory of the terminal from outside the shell. There might be
        //solutions to this, but they are non-trivial and require more IPC

        let wd = self
            .connection
            .read(cx)
            .get_terminal()
            .and_then(|term| term.associated_directory.clone())
            .clone();

        Some(TerminalView::new(wd, false, cx))
    }

    fn project_path(&self, _cx: &gpui::AppContext) -> Option<ProjectPath> {
        None
    }

    fn project_entry_ids(&self, _cx: &gpui::AppContext) -> SmallVec<[project::ProjectEntryId; 3]> {
        SmallVec::new()
    }

    fn is_singleton(&self, _cx: &gpui::AppContext) -> bool {
        false
    }

    fn set_nav_history(&mut self, _: workspace::ItemNavHistory, _: &mut ViewContext<Self>) {}

    fn can_save(&self, _cx: &gpui::AppContext) -> bool {
        false
    }

    fn save(
        &mut self,
        _project: gpui::ModelHandle<Project>,
        _cx: &mut ViewContext<Self>,
    ) -> gpui::Task<gpui::anyhow::Result<()>> {
        unreachable!("save should not have been called");
    }

    fn save_as(
        &mut self,
        _project: gpui::ModelHandle<Project>,
        _abs_path: std::path::PathBuf,
        _cx: &mut ViewContext<Self>,
    ) -> gpui::Task<gpui::anyhow::Result<()>> {
        unreachable!("save_as should not have been called");
    }

    fn reload(
        &mut self,
        _project: gpui::ModelHandle<Project>,
        _cx: &mut ViewContext<Self>,
    ) -> gpui::Task<gpui::anyhow::Result<()>> {
        gpui::Task::ready(Ok(()))
    }

    fn is_dirty(&self, _: &gpui::AppContext) -> bool {
        self.has_new_content
    }

    fn has_conflict(&self, _: &AppContext) -> bool {
        self.has_bell
    }

    fn should_update_tab_on_event(event: &Self::Event) -> bool {
        matches!(event, &Event::TitleChanged)
    }

    fn should_close_item_on_event(event: &Self::Event) -> bool {
        matches!(event, &Event::CloseTerminal)
    }

    fn should_activate_item_on_event(event: &Self::Event) -> bool {
        matches!(event, &Event::Activate)
    }
}

///Get's the working directory for the given workspace, respecting the user's settings.
fn get_wd_for_workspace(workspace: &Workspace, cx: &AppContext) -> Option<PathBuf> {
    let wd_setting = cx
        .global::<Settings>()
        .terminal_overrides
        .working_directory
        .clone()
        .unwrap_or(WorkingDirectory::CurrentProjectDirectory);
    let res = match wd_setting {
        WorkingDirectory::CurrentProjectDirectory => current_project_directory(workspace, cx),
        WorkingDirectory::FirstProjectDirectory => first_project_directory(workspace, cx),
        WorkingDirectory::AlwaysHome => None,
        WorkingDirectory::Always { directory } => {
            shellexpand::full(&directory) //TODO handle this better
                .ok()
                .map(|dir| Path::new(&dir.to_string()).to_path_buf())
                .filter(|dir| dir.is_dir())
        }
    };
    res.or_else(|| home_dir())
}

///Get's the first project's home directory, or the home directory
fn first_project_directory(workspace: &Workspace, cx: &AppContext) -> Option<PathBuf> {
    workspace
        .worktrees(cx)
        .next()
        .and_then(|worktree_handle| worktree_handle.read(cx).as_local())
        .and_then(get_path_from_wt)
}

///Gets the intuitively correct working directory from the given workspace
///If there is an active entry for this project, returns that entry's worktree root.
///If there's no active entry but there is a worktree, returns that worktrees root.
///If either of these roots are files, or if there are any other query failures,
///  returns the user's home directory
fn current_project_directory(workspace: &Workspace, cx: &AppContext) -> Option<PathBuf> {
    let project = workspace.project().read(cx);

    project
        .active_entry()
        .and_then(|entry_id| project.worktree_for_entry(entry_id, cx))
        .or_else(|| workspace.worktrees(cx).next())
        .and_then(|worktree_handle| worktree_handle.read(cx).as_local())
        .and_then(get_path_from_wt)
}

fn get_path_from_wt(wt: &LocalWorktree) -> Option<PathBuf> {
    wt.root_entry()
        .filter(|re| re.is_dir())
        .map(|_| wt.abs_path().to_path_buf())
}

#[cfg(test)]
mod tests {

    use crate::tests::terminal_test_context::TerminalTestContext;

    use super::*;
    use gpui::TestAppContext;

    use std::path::Path;
    use workspace::AppState;

    mod terminal_test_context;

    ///Basic integration test, can we get the terminal to show up, execute a command,
    //and produce noticable output?
    #[gpui::test(retries = 5)]
    async fn test_terminal(cx: &mut TestAppContext) {
        let mut cx = TerminalTestContext::new(cx);

        cx.execute_and_wait("expr 3 + 4", |content, _cx| content.contains("7"))
            .await;
    }

    ///Working directory calculation tests

    ///No Worktrees in project -> home_dir()
    #[gpui::test]
    async fn no_worktree(cx: &mut TestAppContext) {
        //Setup variables
        let params = cx.update(AppState::test);
        let project = Project::test(params.fs.clone(), [], cx).await;
        let (_, workspace) = cx.add_window(|cx| Workspace::new(project.clone(), cx));

        //Test
        cx.read(|cx| {
            let workspace = workspace.read(cx);
            let active_entry = project.read(cx).active_entry();

            //Make sure enviroment is as expeted
            assert!(active_entry.is_none());
            assert!(workspace.worktrees(cx).next().is_none());

            let res = current_project_directory(workspace, cx);
            assert_eq!(res, None);
            let res = first_project_directory(workspace, cx);
            assert_eq!(res, None);
        });
    }

    ///No active entry, but a worktree, worktree is a file -> home_dir()
    #[gpui::test]
    async fn no_active_entry_worktree_is_file(cx: &mut TestAppContext) {
        //Setup variables
        let params = cx.update(AppState::test);
        let project = Project::test(params.fs.clone(), [], cx).await;
        let (_, workspace) = cx.add_window(|cx| Workspace::new(project.clone(), cx));
        let (wt, _) = project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("/root.txt", true, cx)
            })
            .await
            .unwrap();

        cx.update(|cx| {
            wt.update(cx, |wt, cx| {
                wt.as_local()
                    .unwrap()
                    .create_entry(Path::new(""), false, cx)
            })
        })
        .await
        .unwrap();

        //Test
        cx.read(|cx| {
            let workspace = workspace.read(cx);
            let active_entry = project.read(cx).active_entry();

            //Make sure enviroment is as expeted
            assert!(active_entry.is_none());
            assert!(workspace.worktrees(cx).next().is_some());

            let res = current_project_directory(workspace, cx);
            assert_eq!(res, None);
            let res = first_project_directory(workspace, cx);
            assert_eq!(res, None);
        });
    }

    //No active entry, but a worktree, worktree is a folder -> worktree_folder
    #[gpui::test]
    async fn no_active_entry_worktree_is_dir(cx: &mut TestAppContext) {
        //Setup variables
        let params = cx.update(AppState::test);
        let project = Project::test(params.fs.clone(), [], cx).await;
        let (_, workspace) = cx.add_window(|cx| Workspace::new(project.clone(), cx));
        let (wt, _) = project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("/root/", true, cx)
            })
            .await
            .unwrap();

        //Setup root folder
        cx.update(|cx| {
            wt.update(cx, |wt, cx| {
                wt.as_local().unwrap().create_entry(Path::new(""), true, cx)
            })
        })
        .await
        .unwrap();

        //Test
        cx.update(|cx| {
            let workspace = workspace.read(cx);
            let active_entry = project.read(cx).active_entry();

            assert!(active_entry.is_none());
            assert!(workspace.worktrees(cx).next().is_some());

            let res = current_project_directory(workspace, cx);
            assert_eq!(res, Some((Path::new("/root/")).to_path_buf()));
            let res = first_project_directory(workspace, cx);
            assert_eq!(res, Some((Path::new("/root/")).to_path_buf()));
        });
    }

    //Active entry with a work tree, worktree is a file -> home_dir()
    #[gpui::test]
    async fn active_entry_worktree_is_file(cx: &mut TestAppContext) {
        //Setup variables
        let params = cx.update(AppState::test);
        let project = Project::test(params.fs.clone(), [], cx).await;
        let (_, workspace) = cx.add_window(|cx| Workspace::new(project.clone(), cx));
        let (wt1, _) = project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("/root1/", true, cx)
            })
            .await
            .unwrap();

        let (wt2, _) = project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("/root2.txt", true, cx)
            })
            .await
            .unwrap();

        //Setup root
        let _ = cx
            .update(|cx| {
                wt1.update(cx, |wt, cx| {
                    wt.as_local().unwrap().create_entry(Path::new(""), true, cx)
                })
            })
            .await
            .unwrap();
        let entry2 = cx
            .update(|cx| {
                wt2.update(cx, |wt, cx| {
                    wt.as_local()
                        .unwrap()
                        .create_entry(Path::new(""), false, cx)
                })
            })
            .await
            .unwrap();

        cx.update(|cx| {
            let p = ProjectPath {
                worktree_id: wt2.read(cx).id(),
                path: entry2.path,
            };
            project.update(cx, |project, cx| project.set_active_path(Some(p), cx));
        });

        //Test
        cx.update(|cx| {
            let workspace = workspace.read(cx);
            let active_entry = project.read(cx).active_entry();

            assert!(active_entry.is_some());

            let res = current_project_directory(workspace, cx);
            assert_eq!(res, None);
            let res = first_project_directory(workspace, cx);
            assert_eq!(res, Some((Path::new("/root1/")).to_path_buf()));
        });
    }

    //Active entry, with a worktree, worktree is a folder -> worktree_folder
    #[gpui::test]
    async fn active_entry_worktree_is_dir(cx: &mut TestAppContext) {
        //Setup variables
        let params = cx.update(AppState::test);
        let project = Project::test(params.fs.clone(), [], cx).await;
        let (_, workspace) = cx.add_window(|cx| Workspace::new(project.clone(), cx));
        let (wt1, _) = project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("/root1/", true, cx)
            })
            .await
            .unwrap();

        let (wt2, _) = project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("/root2/", true, cx)
            })
            .await
            .unwrap();

        //Setup root
        let _ = cx
            .update(|cx| {
                wt1.update(cx, |wt, cx| {
                    wt.as_local().unwrap().create_entry(Path::new(""), true, cx)
                })
            })
            .await
            .unwrap();
        let entry2 = cx
            .update(|cx| {
                wt2.update(cx, |wt, cx| {
                    wt.as_local().unwrap().create_entry(Path::new(""), true, cx)
                })
            })
            .await
            .unwrap();

        cx.update(|cx| {
            let p = ProjectPath {
                worktree_id: wt2.read(cx).id(),
                path: entry2.path,
            };
            project.update(cx, |project, cx| project.set_active_path(Some(p), cx));
        });

        //Test
        cx.update(|cx| {
            let workspace = workspace.read(cx);
            let active_entry = project.read(cx).active_entry();

            assert!(active_entry.is_some());

            let res = current_project_directory(workspace, cx);
            assert_eq!(res, Some((Path::new("/root2/")).to_path_buf()));
            let res = first_project_directory(workspace, cx);
            assert_eq!(res, Some((Path::new("/root1/")).to_path_buf()));
        });
    }
}
