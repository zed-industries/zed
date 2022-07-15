mod color_translation;
pub mod connection;
mod modal;
pub mod terminal_element;

use alacritty_terminal::{
    event::{Event as AlacTermEvent, EventListener},
    grid::Scroll,
    term::SizeInfo,
};

use connection::{Event, TerminalConnection};
use dirs::home_dir;
use editor::Input;
use futures::channel::mpsc::UnboundedSender;
use gpui::{
    actions, elements::*, impl_internal_actions, AppContext, ClipboardItem, Entity, ModelHandle,
    MutableAppContext, View, ViewContext,
};
use modal::deploy_modal;

use project::{Project, ProjectPath};
use settings::Settings;
use smallvec::SmallVec;
use std::path::PathBuf;
use workspace::{Item, Workspace};

use crate::terminal_element::TerminalEl;

//ASCII Control characters on a keyboard
const ETX_CHAR: char = 3_u8 as char; //'End of text', the control code for 'ctrl-c'
const TAB_CHAR: char = 9_u8 as char;
const CARRIAGE_RETURN_CHAR: char = 13_u8 as char;
const ESC_CHAR: char = 27_u8 as char; // == \x1b
const DEL_CHAR: char = 127_u8 as char;
const LEFT_SEQ: &str = "\x1b[D";
const RIGHT_SEQ: &str = "\x1b[C";
const UP_SEQ: &str = "\x1b[A";
const DOWN_SEQ: &str = "\x1b[B";
const DEBUG_TERMINAL_WIDTH: f32 = 1000.; //This needs to be wide enough that the prompt can fill the whole space.
const DEBUG_TERMINAL_HEIGHT: f32 = 200.;
const DEBUG_CELL_WIDTH: f32 = 5.;
const DEBUG_LINE_HEIGHT: f32 = 5.;

//For bel, use a yellow dot. (equivalent to dirty file with conflict)
//For title, introduce max title length and

///Event to transmit the scroll from the element to the view
#[derive(Clone, Debug, PartialEq)]
pub struct ScrollTerminal(pub i32);

actions!(
    terminal,
    [
        Sigint,
        Escape,
        Del,
        Return,
        Left,
        Right,
        Up,
        Down,
        Tab,
        Clear,
        Copy,
        Paste,
        Deploy,
        Quit,
        DeployModal,
    ]
);
impl_internal_actions!(terminal, [ScrollTerminal]);

///Initialize and register all of our action handlers
pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(Terminal::deploy);
    cx.add_action(Terminal::send_sigint);
    cx.add_action(Terminal::escape);
    cx.add_action(Terminal::quit);
    cx.add_action(Terminal::del);
    cx.add_action(Terminal::carriage_return);
    cx.add_action(Terminal::left);
    cx.add_action(Terminal::right);
    cx.add_action(Terminal::up);
    cx.add_action(Terminal::down);
    cx.add_action(Terminal::tab);
    cx.add_action(Terminal::copy);
    cx.add_action(Terminal::paste);
    cx.add_action(Terminal::scroll_terminal);
    cx.add_action(Terminal::input);
    cx.add_action(Terminal::clear);
    cx.add_action(deploy_modal);
}

///A translation struct for Alacritty to communicate with us from their event loop
#[derive(Clone)]
pub struct ZedListener(UnboundedSender<AlacTermEvent>);

impl EventListener for ZedListener {
    fn send_event(&self, event: AlacTermEvent) {
        self.0.unbounded_send(event).ok();
    }
}

///A terminal view, maintains the PTY's file handles and communicates with the terminal
pub struct Terminal {
    connection: ModelHandle<TerminalConnection>,
    has_new_content: bool,
    //Currently using iTerm bell, show bell emoji in tab until input is received
    has_bell: bool,
    // Only for styling purposes. Doesn't effect behavior
    modal: bool,
}

impl Entity for Terminal {
    type Event = Event;
}

impl Terminal {
    ///Create a new Terminal view. This spawns a task, a thread, and opens the TTY devices
    ///To get the right working directory from a workspace, use: `get_wd_for_workspace()`
    fn new(working_directory: Option<PathBuf>, modal: bool, cx: &mut ViewContext<Self>) -> Self {
        //The details here don't matter, the terminal will be resized on the first layout
        let size_info = SizeInfo::new(
            DEBUG_TERMINAL_WIDTH,
            DEBUG_TERMINAL_HEIGHT,
            DEBUG_CELL_WIDTH,
            DEBUG_LINE_HEIGHT,
            0.,
            0.,
            false,
        );

        let connection =
            cx.add_model(|cx| TerminalConnection::new(working_directory, size_info, cx));

        Terminal::from_connection(connection, modal, cx)
    }

    fn from_connection(
        connection: ModelHandle<TerminalConnection>,
        modal: bool,
        cx: &mut ViewContext<Self>,
    ) -> Terminal {
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
            _ => cx.emit(*event),
        })
        .detach();

        Terminal {
            connection,
            has_new_content: true,
            has_bell: false,
            modal,
        }
    }

    ///Scroll the terminal. This locks the terminal
    fn scroll_terminal(&mut self, scroll: &ScrollTerminal, cx: &mut ViewContext<Self>) {
        self.connection
            .read(cx)
            .term
            .lock()
            .scroll_display(Scroll::Delta(scroll.0));
    }

    fn input(&mut self, Input(text): &Input, cx: &mut ViewContext<Self>) {
        self.connection.update(cx, |connection, _| {
            //TODO: This is probably not encoding UTF8 correctly (see alacritty/src/input.rs:L825-837)
            connection.write_to_pty(text.clone());
        });

        if self.has_bell {
            self.has_bell = false;
            cx.emit(Event::TitleChanged);
        }
    }

    fn clear(&mut self, _: &Clear, cx: &mut ViewContext<Self>) {
        self.connection
            .update(cx, |connection, _| connection.clear());
    }

    ///Create a new Terminal in the current working directory or the user's home directory
    fn deploy(workspace: &mut Workspace, _: &Deploy, cx: &mut ViewContext<Workspace>) {
        let wd = get_wd_for_workspace(workspace, cx);
        workspace.add_item(Box::new(cx.add_view(|cx| Terminal::new(wd, false, cx))), cx);
    }

    ///Tell Zed to close us
    fn quit(&mut self, _: &Quit, cx: &mut ViewContext<Self>) {
        cx.emit(Event::CloseTerminal);
    }

    ///Attempt to paste the clipboard into the terminal
    fn copy(&mut self, _: &Copy, cx: &mut ViewContext<Self>) {
        let term = self.connection.read(cx).term.lock();
        let copy_text = term.selection_to_string();
        match copy_text {
            Some(s) => cx.write_to_clipboard(ClipboardItem::new(s)),
            None => (),
        }
    }

    ///Attempt to paste the clipboard into the terminal
    fn paste(&mut self, _: &Paste, cx: &mut ViewContext<Self>) {
        if let Some(item) = cx.read_from_clipboard() {
            self.connection.update(cx, |connection, _| {
                connection.write_to_pty(item.text().to_owned());
            })
        }
    }

    ///Send the `up` key
    fn up(&mut self, _: &Up, cx: &mut ViewContext<Self>) {
        self.connection.update(cx, |connection, _| {
            connection.write_to_pty(UP_SEQ.to_string());
        });
    }

    ///Send the `down` key
    fn down(&mut self, _: &Down, cx: &mut ViewContext<Self>) {
        self.connection.update(cx, |connection, _| {
            connection.write_to_pty(DOWN_SEQ.to_string());
        });
    }

    ///Send the `tab` key
    fn tab(&mut self, _: &Tab, cx: &mut ViewContext<Self>) {
        self.connection.update(cx, |connection, _| {
            connection.write_to_pty(TAB_CHAR.to_string());
        });
    }

    ///Send `SIGINT` (`ctrl-c`)
    fn send_sigint(&mut self, _: &Sigint, cx: &mut ViewContext<Self>) {
        self.connection.update(cx, |connection, _| {
            connection.write_to_pty(ETX_CHAR.to_string());
        });
    }

    ///Send the `escape` key
    fn escape(&mut self, _: &Escape, cx: &mut ViewContext<Self>) {
        self.connection.update(cx, |connection, _| {
            connection.write_to_pty(ESC_CHAR.to_string());
        });
    }

    ///Send the `delete` key. TODO: Difference between this and backspace?
    fn del(&mut self, _: &Del, cx: &mut ViewContext<Self>) {
        self.connection.update(cx, |connection, _| {
            connection.write_to_pty(DEL_CHAR.to_string());
        });
    }

    ///Send a carriage return. TODO: May need to check the terminal mode.
    fn carriage_return(&mut self, _: &Return, cx: &mut ViewContext<Self>) {
        self.connection.update(cx, |connection, _| {
            connection.write_to_pty(CARRIAGE_RETURN_CHAR.to_string());
        });
    }

    //Send the `left` key
    fn left(&mut self, _: &Left, cx: &mut ViewContext<Self>) {
        self.connection.update(cx, |connection, _| {
            connection.write_to_pty(LEFT_SEQ.to_string());
        });
    }

    //Send the `right` key
    fn right(&mut self, _: &Right, cx: &mut ViewContext<Self>) {
        self.connection.update(cx, |connection, _| {
            connection.write_to_pty(RIGHT_SEQ.to_string());
        });
    }
}

impl View for Terminal {
    fn ui_name() -> &'static str {
        "Terminal"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> ElementBox {
        let element = {
            let connection_handle = self.connection.clone().downgrade();
            let view_id = cx.view_id();
            TerminalEl::new(view_id, connection_handle, self.modal).contained()
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

impl Item for Terminal {
    fn tab_content(
        &self,
        _detail: Option<usize>,
        tab_theme: &theme::Tab,
        cx: &gpui::AppContext,
    ) -> ElementBox {
        let settings = cx.global::<Settings>();
        let search_theme = &settings.theme.search; //TODO properly integrate themes

        let mut flex = Flex::row();

        if self.has_bell {
            flex.add_child(
                Svg::new("icons/zap.svg") //TODO: Swap out for a better icon, or at least resize this
                    .with_color(tab_theme.label.text.color)
                    .constrained()
                    .with_width(search_theme.tab_icon_width)
                    .aligned()
                    .boxed(),
            );
        };

        flex.with_child(
            Label::new(
                self.connection.read(cx).title.clone(),
                tab_theme.label.clone(),
            )
            .aligned()
            .contained()
            .with_margin_left(if self.has_bell {
                search_theme.tab_icon_spacing
            } else {
                0.
            })
            .boxed(),
        )
        .boxed()
    }

    fn clone_on_split(&self, cx: &mut ViewContext<Self>) -> Option<Self> {
        //From what I can tell, there's no  way to tell the current working
        //Directory of the terminal from outside the terminal. There might be
        //solutions to this, but they are non-trivial and require more IPC
        Some(Terminal::new(
            self.connection.read(cx).associated_directory.clone(),
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

    fn is_dirty(&self, _: &gpui::AppContext) -> bool {
        self.has_new_content
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

///Gets the intuitively correct working directory from the given workspace
///If there is an active entry for this project, returns that entry's worktree root.
///If there's no active entry but there is a worktree, returns that worktrees root.
///If either of these roots are files, or if there are any other query failures,
///  returns the user's home directory
fn get_wd_for_workspace(workspace: &Workspace, cx: &AppContext) -> Option<PathBuf> {
    let project = workspace.project().read(cx);

    project
        .active_entry()
        .and_then(|entry_id| project.worktree_for_entry(entry_id, cx))
        .or_else(|| workspace.worktrees(cx).next())
        .and_then(|worktree_handle| worktree_handle.read(cx).as_local())
        .and_then(|wt| {
            wt.root_entry()
                .filter(|re| re.is_dir())
                .map(|_| wt.abs_path().to_path_buf())
        })
        .or_else(|| home_dir())
}

#[cfg(test)]
mod tests {

    use super::*;
    use alacritty_terminal::{
        grid::GridIterator,
        index::{Column, Line, Point, Side},
        selection::{Selection, SelectionType},
        term::cell::Cell,
    };
    use gpui::TestAppContext;
    use itertools::Itertools;

    use std::{path::Path, time::Duration};
    use workspace::AppState;

    ///Basic integration test, can we get the terminal to show up, execute a command,
    //and produce noticable output?
    #[gpui::test(retries = 5)]
    async fn test_terminal(cx: &mut TestAppContext) {
        let terminal = cx.add_view(Default::default(), |cx| Terminal::new(None, false, cx));

        terminal.update(cx, |terminal, cx| {
            terminal.connection.update(cx, |connection, _| {
                connection.write_to_pty("expr 3 + 4".to_string());
            });
            terminal.carriage_return(&Return, cx);
        });

        cx.set_condition_duration(Some(Duration::from_secs(5)));
        terminal
            .condition(cx, |terminal, cx| {
                let term = terminal.connection.read(cx).term.clone();
                let content = grid_as_str(term.lock().renderable_content().display_iter);
                content.contains("7")
            })
            .await;
        cx.set_condition_duration(None);
    }

    /// Integration test for selections, clipboard, and terminal execution
    #[gpui::test]
    async fn test_copy(cx: &mut TestAppContext) {
        let mut result_line: i32 = 0;
        let terminal = cx.add_view(Default::default(), |cx| Terminal::new(None, false, cx));
        cx.set_condition_duration(Some(Duration::from_secs(2)));

        terminal.update(cx, |terminal, cx| {
            terminal.connection.update(cx, |connection, _| {
                connection.write_to_pty("expr 3 + 4".to_string());
            });
            terminal.carriage_return(&Return, cx);
        });

        terminal
            .condition(cx, |terminal, cx| {
                let term = terminal.connection.read(cx).term.clone();
                let content = grid_as_str(term.lock().renderable_content().display_iter);

                if content.contains("7") {
                    let idx = content.chars().position(|c| c == '7').unwrap();
                    result_line = content.chars().take(idx).filter(|c| *c == '\n').count() as i32;
                    true
                } else {
                    false
                }
            })
            .await;

        terminal.update(cx, |terminal, cx| {
            let mut term = terminal.connection.read(cx).term.lock();
            term.selection = Some(Selection::new(
                SelectionType::Semantic,
                Point::new(Line(2), Column(0)),
                Side::Right,
            ));
            drop(term);
            terminal.copy(&Copy, cx)
        });

        cx.assert_clipboard_content(Some(&"7"));
        cx.set_condition_duration(None);
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

            let res = get_wd_for_workspace(workspace, cx);
            assert_eq!(res, home_dir())
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

            let res = get_wd_for_workspace(workspace, cx);
            assert_eq!(res, home_dir())
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

            let res = get_wd_for_workspace(workspace, cx);
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
        let (wt, _) = project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("/root.txt", true, cx)
            })
            .await
            .unwrap();

        //Setup root
        let entry = cx
            .update(|cx| {
                wt.update(cx, |wt, cx| {
                    wt.as_local()
                        .unwrap()
                        .create_entry(Path::new(""), false, cx)
                })
            })
            .await
            .unwrap();

        cx.update(|cx| {
            let p = ProjectPath {
                worktree_id: wt.read(cx).id(),
                path: entry.path,
            };
            project.update(cx, |project, cx| project.set_active_path(Some(p), cx));
        });

        //Test
        cx.update(|cx| {
            let workspace = workspace.read(cx);
            let active_entry = project.read(cx).active_entry();

            assert!(active_entry.is_some());

            let res = get_wd_for_workspace(workspace, cx);
            assert_eq!(res, home_dir());
        });
    }

    //Active entry, with a worktree, worktree is a folder -> worktree_folder
    #[gpui::test]
    async fn active_entry_worktree_is_dir(cx: &mut TestAppContext) {
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

        //Setup root
        let entry = cx
            .update(|cx| {
                wt.update(cx, |wt, cx| {
                    wt.as_local().unwrap().create_entry(Path::new(""), true, cx)
                })
            })
            .await
            .unwrap();

        cx.update(|cx| {
            let p = ProjectPath {
                worktree_id: wt.read(cx).id(),
                path: entry.path,
            };
            project.update(cx, |project, cx| project.set_active_path(Some(p), cx));
        });

        //Test
        cx.update(|cx| {
            let workspace = workspace.read(cx);
            let active_entry = project.read(cx).active_entry();

            assert!(active_entry.is_some());

            let res = get_wd_for_workspace(workspace, cx);
            assert_eq!(res, Some((Path::new("/root/")).to_path_buf()));
        });
    }

    pub(crate) fn grid_as_str(grid_iterator: GridIterator<Cell>) -> String {
        let lines = grid_iterator.group_by(|i| i.point.line.0);
        lines
            .into_iter()
            .map(|(_, line)| line.map(|i| i.c).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
