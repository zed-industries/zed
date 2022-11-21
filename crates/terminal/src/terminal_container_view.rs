use crate::persistence::TERMINAL_CONNECTION;
use crate::terminal_view::TerminalView;
use crate::{Event, Terminal, TerminalBuilder, TerminalError};

use alacritty_terminal::index::Point;
use dirs::home_dir;
use gpui::{
    actions, elements::*, AnyViewHandle, AppContext, Entity, ModelHandle, MutableAppContext, Task,
    View, ViewContext, ViewHandle, WeakViewHandle,
};
use util::{truncate_and_trailoff, ResultExt};
use workspace::searchable::{SearchEvent, SearchOptions, SearchableItem, SearchableItemHandle};
use workspace::{
    item::{Item, ItemEvent},
    ToolbarItemLocation, Workspace,
};
use workspace::{register_deserializable_item, Pane};

use project::{LocalWorktree, Project, ProjectPath};
use settings::{AlternateScroll, Settings, WorkingDirectory};
use smallvec::SmallVec;
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};

use crate::terminal_element::TerminalElement;

actions!(terminal, [DeployModal]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(TerminalContainer::deploy);

    register_deserializable_item::<TerminalContainer>(cx);
}

//Make terminal view an enum, that can give you views for the error and non-error states
//Take away all the result unwrapping in the current TerminalView by making it 'infallible'
//Bubble up to deploy(_modal)() calls

pub enum TerminalContainerContent {
    Connected(ViewHandle<TerminalView>),
    Error(ViewHandle<ErrorView>),
}

impl TerminalContainerContent {
    fn handle(&self) -> AnyViewHandle {
        match self {
            Self::Connected(handle) => handle.into(),
            Self::Error(handle) => handle.into(),
        }
    }
}

pub struct TerminalContainer {
    pub content: TerminalContainerContent,
    associated_directory: Option<PathBuf>,
}

pub struct ErrorView {
    error: TerminalError,
}

impl Entity for TerminalContainer {
    type Event = Event;
}

impl Entity for ErrorView {
    type Event = Event;
}

impl TerminalContainer {
    ///Create a new Terminal in the current working directory or the user's home directory
    pub fn deploy(
        workspace: &mut Workspace,
        _: &workspace::NewTerminal,
        cx: &mut ViewContext<Workspace>,
    ) {
        let strategy = cx
            .global::<Settings>()
            .terminal_overrides
            .working_directory
            .clone()
            .unwrap_or(WorkingDirectory::CurrentProjectDirectory);

        let working_directory = get_working_directory(workspace, cx, strategy);
        let view = cx.add_view(|cx| TerminalContainer::new(working_directory, false, cx));
        workspace.add_item(Box::new(view), cx);
    }

    ///Create a new Terminal view. This spawns a task, a thread, and opens the TTY devices    
    pub fn new(
        working_directory: Option<PathBuf>,
        modal: bool,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let settings = cx.global::<Settings>();
        let shell = settings.terminal_overrides.shell.clone();
        let envs = settings.terminal_overrides.env.clone(); //Should be short and cheap.

        //TODO: move this pattern to settings
        let scroll = settings
            .terminal_overrides
            .alternate_scroll
            .as_ref()
            .unwrap_or(
                settings
                    .terminal_defaults
                    .alternate_scroll
                    .as_ref()
                    .unwrap_or_else(|| &AlternateScroll::On),
            );

        let content = match TerminalBuilder::new(
            working_directory.clone(),
            shell,
            envs,
            settings.terminal_overrides.blinking.clone(),
            scroll,
            cx.window_id(),
        ) {
            Ok(terminal) => {
                let terminal = cx.add_model(|cx| terminal.subscribe(cx));
                let view = cx.add_view(|cx| TerminalView::from_terminal(terminal, modal, cx));
                cx.subscribe(&view, |_this, _content, event, cx| cx.emit(*event))
                    .detach();
                TerminalContainerContent::Connected(view)
            }
            Err(error) => {
                let view = cx.add_view(|_| ErrorView {
                    error: error.downcast::<TerminalError>().unwrap(),
                });
                TerminalContainerContent::Error(view)
            }
        };
        // cx.focus(content.handle());

        TerminalContainer {
            content,
            associated_directory: working_directory,
        }
    }

    pub fn from_terminal(
        terminal: ModelHandle<Terminal>,
        modal: bool,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let connected_view = cx.add_view(|cx| TerminalView::from_terminal(terminal, modal, cx));
        TerminalContainer {
            content: TerminalContainerContent::Connected(connected_view),
            associated_directory: None,
        }
    }

    fn connected(&self) -> Option<ViewHandle<TerminalView>> {
        match &self.content {
            TerminalContainerContent::Connected(vh) => Some(vh.clone()),
            TerminalContainerContent::Error(_) => None,
        }
    }
}

impl View for TerminalContainer {
    fn ui_name() -> &'static str {
        "Terminal"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> ElementBox {
        match &self.content {
            TerminalContainerContent::Connected(connected) => ChildView::new(connected, cx),
            TerminalContainerContent::Error(error) => ChildView::new(error, cx),
        }
        .boxed()
    }

    fn focus_in(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        if cx.is_self_focused() {
            cx.focus(self.content.handle());
        }
    }
}

impl View for ErrorView {
    fn ui_name() -> &'static str {
        "Terminal Error"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> ElementBox {
        let settings = cx.global::<Settings>();
        let style = TerminalElement::make_text_style(cx.font_cache(), settings);

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
            .with_child(Text::new(error_text, style).contained().boxed())
            .aligned()
            .boxed()
    }
}

impl Item for TerminalContainer {
    fn tab_content(
        &self,
        _detail: Option<usize>,
        tab_theme: &theme::Tab,
        cx: &gpui::AppContext,
    ) -> ElementBox {
        let title = match &self.content {
            TerminalContainerContent::Connected(connected) => connected
                .read(cx)
                .handle()
                .read(cx)
                .foreground_process_info
                .as_ref()
                .map(|fpi| {
                    format!(
                        "{} — {}",
                        truncate_and_trailoff(
                            &fpi.cwd
                                .file_name()
                                .map(|name| name.to_string_lossy().to_string())
                                .unwrap_or_default(),
                            25
                        ),
                        truncate_and_trailoff(
                            &{
                                format!(
                                    "{}{}",
                                    fpi.name,
                                    if fpi.argv.len() >= 1 {
                                        format!(" {}", (&fpi.argv[1..]).join(" "))
                                    } else {
                                        "".to_string()
                                    }
                                )
                            },
                            25
                        )
                    )
                })
                .unwrap_or_else(|| "Terminal".to_string()),
            TerminalContainerContent::Error(_) => "Terminal".to_string(),
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
        Some(TerminalContainer::new(
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
        if let TerminalContainerContent::Connected(connected) = &self.content {
            connected.read(cx).has_bell()
        } else {
            false
        }
    }

    fn has_conflict(&self, _cx: &AppContext) -> bool {
        false
    }

    fn as_searchable(&self, handle: &ViewHandle<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(handle.clone()))
    }

    fn to_item_events(event: &Self::Event) -> Vec<ItemEvent> {
        match event {
            Event::BreadcrumbsChanged => vec![ItemEvent::UpdateBreadcrumbs],
            Event::TitleChanged | Event::Wakeup => vec![ItemEvent::UpdateTab],
            Event::CloseTerminal => vec![ItemEvent::CloseItem],
            _ => vec![],
        }
    }

    fn breadcrumb_location(&self) -> ToolbarItemLocation {
        if self.connected().is_some() {
            ToolbarItemLocation::PrimaryLeft { flex: None }
        } else {
            ToolbarItemLocation::Hidden
        }
    }

    fn breadcrumbs(&self, theme: &theme::Theme, cx: &AppContext) -> Option<Vec<ElementBox>> {
        let connected = self.connected()?;

        Some(vec![Text::new(
            connected
                .read(cx)
                .terminal()
                .read(cx)
                .breadcrumb_text
                .to_string(),
            theme.breadcrumbs.text.clone(),
        )
        .boxed()])
    }

    fn serialized_item_kind() -> Option<&'static str> {
        Some("Terminal")
    }

    fn deserialize(
        _project: ModelHandle<Project>,
        _workspace: WeakViewHandle<Workspace>,
        workspace_id: workspace::WorkspaceId,
        item_id: workspace::ItemId,
        cx: &mut ViewContext<Pane>,
    ) -> Task<anyhow::Result<ViewHandle<Self>>> {
        let working_directory = TERMINAL_CONNECTION.get_working_directory(item_id, &workspace_id);
        Task::ready(Ok(cx.add_view(|cx| {
            TerminalContainer::new(working_directory.log_err().flatten(), false, cx)
        })))
    }
}

impl SearchableItem for TerminalContainer {
    type Match = RangeInclusive<Point>;

    fn supported_options() -> SearchOptions {
        SearchOptions {
            case: false,
            word: false,
            regex: false,
        }
    }

    /// Convert events raised by this item into search-relevant events (if applicable)
    fn to_search_event(event: &Self::Event) -> Option<SearchEvent> {
        match event {
            Event::Wakeup => Some(SearchEvent::MatchesInvalidated),
            Event::SelectionsChanged => Some(SearchEvent::ActiveMatchChanged),
            _ => None,
        }
    }

    /// Clear stored matches
    fn clear_matches(&mut self, cx: &mut ViewContext<Self>) {
        if let TerminalContainerContent::Connected(connected) = &self.content {
            let terminal = connected.read(cx).terminal().clone();
            terminal.update(cx, |term, _| term.matches.clear())
        }
    }

    /// Store matches returned from find_matches somewhere for rendering
    fn update_matches(&mut self, matches: Vec<Self::Match>, cx: &mut ViewContext<Self>) {
        if let TerminalContainerContent::Connected(connected) = &self.content {
            let terminal = connected.read(cx).terminal().clone();
            terminal.update(cx, |term, _| term.matches = matches)
        }
    }

    /// Return the selection content to pre-load into this search
    fn query_suggestion(&mut self, cx: &mut ViewContext<Self>) -> String {
        if let TerminalContainerContent::Connected(connected) = &self.content {
            let terminal = connected.read(cx).terminal().clone();
            terminal
                .read(cx)
                .last_content
                .selection_text
                .clone()
                .unwrap_or_default()
        } else {
            Default::default()
        }
    }

    /// Focus match at given index into the Vec of matches
    fn activate_match(&mut self, index: usize, _: Vec<Self::Match>, cx: &mut ViewContext<Self>) {
        if let TerminalContainerContent::Connected(connected) = &self.content {
            let terminal = connected.read(cx).terminal().clone();
            terminal.update(cx, |term, _| term.activate_match(index));
            cx.notify();
        }
    }

    /// Get all of the matches for this query, should be done on the background
    fn find_matches(
        &mut self,
        query: project::search::SearchQuery,
        cx: &mut ViewContext<Self>,
    ) -> Task<Vec<Self::Match>> {
        if let TerminalContainerContent::Connected(connected) = &self.content {
            let terminal = connected.read(cx).terminal().clone();
            terminal.update(cx, |term, cx| term.find_matches(query, cx))
        } else {
            Task::ready(Vec::new())
        }
    }

    /// Reports back to the search toolbar what the active match should be (the selection)
    fn active_match_index(
        &mut self,
        matches: Vec<Self::Match>,
        cx: &mut ViewContext<Self>,
    ) -> Option<usize> {
        let connected = self.connected();
        // Selection head might have a value if there's a selection that isn't
        // associated with a match. Therefore, if there are no matches, we should
        // report None, no matter the state of the terminal
        let res = if matches.len() > 0 && connected.is_some() {
            if let Some(selection_head) = connected
                .unwrap()
                .read(cx)
                .terminal()
                .read(cx)
                .selection_head
            {
                // If selection head is contained in a match. Return that match
                if let Some(ix) = matches
                    .iter()
                    .enumerate()
                    .find(|(_, search_match)| {
                        search_match.contains(&selection_head)
                            || search_match.start() > &selection_head
                    })
                    .map(|(ix, _)| ix)
                {
                    Some(ix)
                } else {
                    // If no selection after selection head, return the last match
                    Some(matches.len().saturating_sub(1))
                }
            } else {
                // Matches found but no active selection, return the first last one (closest to cursor)
                Some(matches.len().saturating_sub(1))
            }
        } else {
            None
        };

        res
    }
}

///Get's the working directory for the given workspace, respecting the user's settings.
pub fn get_working_directory(
    workspace: &Workspace,
    cx: &AppContext,
    strategy: WorkingDirectory,
) -> Option<PathBuf> {
    let res = match strategy {
        WorkingDirectory::CurrentProjectDirectory => current_project_directory(workspace, cx)
            .or_else(|| first_project_directory(workspace, cx)),
        WorkingDirectory::FirstProjectDirectory => first_project_directory(workspace, cx),
        WorkingDirectory::AlwaysHome => None,
        WorkingDirectory::Always { directory } => {
            shellexpand::full(&directory) //TODO handle this better
                .ok()
                .map(|dir| Path::new(&dir.to_string()).to_path_buf())
                .filter(|dir| dir.is_dir())
        }
    };
    res.or_else(home_dir)
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
        let mut cx = TerminalTestContext::new(cx);
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

        let mut cx = TerminalTestContext::new(cx);
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
        let mut cx = TerminalTestContext::new(cx);
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
        let mut cx = TerminalTestContext::new(cx);
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
        let mut cx = TerminalTestContext::new(cx);
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
