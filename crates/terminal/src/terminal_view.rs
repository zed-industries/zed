use crate::connected_view::ConnectedView;
use crate::{Event, Terminal, TerminalBuilder, TerminalError};
use dirs::home_dir;
use gpui::{
    actions, elements::*, AnyViewHandle, AppContext, Entity, ModelHandle, View, ViewContext,
    ViewHandle,
};

use crate::TermDimensions;
use project::{LocalWorktree, Project, ProjectPath};
use settings::{Settings, WorkingDirectory};
use smallvec::SmallVec;
use std::path::{Path, PathBuf};
use workspace::{Item, Workspace};

use crate::connected_el::TerminalEl;

actions!(terminal, [Deploy, DeployModal]);

//Make terminal view an enum, that can give you views for the error and non-error states
//Take away all the result unwrapping in the current TerminalView by making it 'infallible'
//Bubble up to deploy(_modal)() calls

pub enum TerminalContent {
    Connected(ViewHandle<ConnectedView>),
    Error(ViewHandle<ErrorView>),
}

impl TerminalContent {
    fn handle(&self) -> AnyViewHandle {
        match self {
            Self::Connected(handle) => handle.into(),
            Self::Error(handle) => handle.into(),
        }
    }
}

pub struct TerminalView {
    modal: bool,
    pub content: TerminalContent,
    associated_directory: Option<PathBuf>,
}

pub struct ErrorView {
    error: TerminalError,
}

impl Entity for TerminalView {
    type Event = Event;
}

impl Entity for ConnectedView {
    type Event = Event;
}

impl Entity for ErrorView {
    type Event = Event;
}

impl TerminalView {
    ///Create a new Terminal in the current working directory or the user's home directory
    pub fn deploy(workspace: &mut Workspace, _: &Deploy, cx: &mut ViewContext<Workspace>) {
        let working_directory = get_working_directory(workspace, cx);
        let view = cx.add_view(|cx| TerminalView::new(working_directory, false, cx));
        workspace.add_item(Box::new(view), cx);
    }

    ///Create a new Terminal view. This spawns a task, a thread, and opens the TTY devices
    ///To get the right working directory from a workspace, use: `get_wd_for_workspace()`
    pub fn new(
        working_directory: Option<PathBuf>,
        modal: bool,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        //The details here don't matter, the terminal will be resized on the first layout
        let size_info = TermDimensions::default();

        let settings = cx.global::<Settings>();
        let shell = settings.terminal_overrides.shell.clone();
        let envs = settings.terminal_overrides.env.clone(); //Should be short and cheap.

        let content = match TerminalBuilder::new(working_directory.clone(), shell, envs, size_info)
        {
            Ok(terminal) => {
                let terminal = cx.add_model(|cx| terminal.subscribe(cx));
                let view = cx.add_view(|cx| ConnectedView::from_terminal(terminal, modal, cx));
                cx.subscribe(&view, |_this, _content, event, cx| cx.emit(event.clone()))
                    .detach();
                TerminalContent::Connected(view)
            }
            Err(error) => {
                let view = cx.add_view(|_| ErrorView {
                    error: error.downcast::<TerminalError>().unwrap(),
                });
                TerminalContent::Error(view)
            }
        };
        cx.focus(content.handle());

        TerminalView {
            modal,
            content,
            associated_directory: working_directory,
        }
    }

    pub fn from_terminal(
        terminal: ModelHandle<Terminal>,
        modal: bool,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let connected_view = cx.add_view(|cx| ConnectedView::from_terminal(terminal, modal, cx));
        TerminalView {
            modal,
            content: TerminalContent::Connected(connected_view),
            associated_directory: None,
        }
    }
}

impl View for TerminalView {
    fn ui_name() -> &'static str {
        "Terminal"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> ElementBox {
        let child_view = match &self.content {
            TerminalContent::Connected(connected) => ChildView::new(connected),
            TerminalContent::Error(error) => ChildView::new(error),
        };

        if self.modal {
            let settings = cx.global::<Settings>();
            let container_style = settings.theme.terminal.modal_container;
            child_view.contained().with_style(container_style).boxed()
        } else {
            child_view.boxed()
        }
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Activate);
        cx.defer(|view, cx| {
            cx.focus(view.content.handle());
        });
    }

    fn keymap_context(&self, _: &gpui::AppContext) -> gpui::keymap::Context {
        let mut context = Self::default_keymap_context();
        if self.modal {
            context.set.insert("ModalTerminal".into());
        }
        context
    }
}

impl View for ErrorView {
    fn ui_name() -> &'static str {
        "Terminal Error"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> ElementBox {
        let settings = cx.global::<Settings>();
        let style = TerminalEl::make_text_style(cx.font_cache(), settings);

        //TODO:
        //We want markdown style highlighting so we can format the program and working directory with ``
        //We want a max-width of 75% with word-wrap
        //We want to be able to select the text
        //Want to be able to scroll if the error message is massive somehow (resiliency)

        let program_text = {
            match self.error.shell_to_string() {
                Some(shell_txt) => format!("Shell Program: `{}`", shell_txt),
                None => "No program specified".to_string(),
            }
        };

        let directory_text = {
            match self.error.directory.as_ref() {
                Some(path) => format!("Working directory: `{}`", path.to_string_lossy()),
                None => "No working directory specified".to_string(),
            }
        };

        let error_text = self.error.source.to_string();

        Flex::column()
            .with_child(
                Text::new("Failed to open the terminal.".to_string(), style.clone())
                    .contained()
                    .boxed(),
            )
            .with_child(Text::new(program_text, style.clone()).contained().boxed())
            .with_child(Text::new(directory_text, style.clone()).contained().boxed())
            .with_child(Text::new(error_text, style.clone()).contained().boxed())
            .aligned()
            .boxed()
    }
}

impl Item for TerminalView {
    fn tab_content(
        &self,
        _detail: Option<usize>,
        tab_theme: &theme::Tab,
        cx: &gpui::AppContext,
    ) -> ElementBox {
        let title = match &self.content {
            TerminalContent::Connected(connected) => {
                connected.read(cx).handle().read(cx).title.to_string()
            }
            TerminalContent::Error(_) => "Terminal".to_string(),
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
        Some(TerminalView::new(
            self.associated_directory.clone(),
            false,
            cx,
        ))
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

    fn is_dirty(&self, cx: &gpui::AppContext) -> bool {
        if let TerminalContent::Connected(connected) = &self.content {
            connected.read(cx).has_new_content()
        } else {
            false
        }
    }

    fn has_conflict(&self, cx: &AppContext) -> bool {
        if let TerminalContent::Connected(connected) = &self.content {
            connected.read(cx).has_bell()
        } else {
            false
        }
    }

    fn should_update_tab_on_event(event: &Self::Event) -> bool {
        matches!(event, &Event::TitleChanged | &Event::Wakeup)
    }

    fn should_close_item_on_event(event: &Self::Event) -> bool {
        matches!(event, &Event::CloseTerminal)
    }

    fn should_activate_item_on_event(event: &Self::Event) -> bool {
        matches!(event, &Event::Activate)
    }
}

///Get's the working directory for the given workspace, respecting the user's settings.
pub fn get_working_directory(workspace: &Workspace, cx: &AppContext) -> Option<PathBuf> {
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

    use super::*;
    use gpui::TestAppContext;

    use std::path::Path;

    use crate::tests::terminal_test_context::TerminalTestContext;

    ///Working directory calculation tests

    ///No Worktrees in project -> home_dir()
    #[gpui::test]
    async fn no_worktree(cx: &mut TestAppContext) {
        //Setup variables
        let mut cx = TerminalTestContext::new(cx, true);
        let (project, workspace) = cx.blank_workspace().await;
        //Test
        cx.cx.read(|cx| {
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

        let mut cx = TerminalTestContext::new(cx, true);
        let (project, workspace) = cx.blank_workspace().await;
        cx.create_file_wt(project.clone(), "/root.txt").await;

        cx.cx.read(|cx| {
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
        let mut cx = TerminalTestContext::new(cx, true);
        let (project, workspace) = cx.blank_workspace().await;
        let (_wt, _entry) = cx.create_folder_wt(project.clone(), "/root/").await;

        //Test
        cx.cx.update(|cx| {
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
        let mut cx = TerminalTestContext::new(cx, true);
        let (project, workspace) = cx.blank_workspace().await;
        let (_wt, _entry) = cx.create_folder_wt(project.clone(), "/root1/").await;
        let (wt2, entry2) = cx.create_file_wt(project.clone(), "/root2.txt").await;
        cx.insert_active_entry_for(wt2, entry2, project.clone());

        //Test
        cx.cx.update(|cx| {
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
        let mut cx = TerminalTestContext::new(cx, true);
        let (project, workspace) = cx.blank_workspace().await;
        let (_wt, _entry) = cx.create_folder_wt(project.clone(), "/root1/").await;
        let (wt2, entry2) = cx.create_folder_wt(project.clone(), "/root2/").await;
        cx.insert_active_entry_for(wt2, entry2, project.clone());

        //Test
        cx.cx.update(|cx| {
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
