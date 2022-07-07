use alacritty_terminal::{
    config::{Config, Program, PtyConfig},
    event::{Event as AlacTermEvent, EventListener, Notify},
    event_loop::{EventLoop, Msg, Notifier},
    grid::Scroll,
    sync::FairMutex,
    term::{color::Rgb as AlacRgb, SizeInfo},
    tty::{self, setup_env},
    Term,
};

use dirs::home_dir;
use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    StreamExt,
};
use gpui::{
    actions, color::Color, elements::*, impl_internal_actions, platform::CursorStyle,
    ClipboardItem, Entity, MutableAppContext, View, ViewContext,
};
use project::{LocalWorktree, Project, ProjectPath};
use settings::Settings;
use smallvec::SmallVec;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use workspace::{Item, Workspace};

use crate::terminal_element::{get_color_at_index, TerminalEl};

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
const DEFAULT_TITLE: &str = "Terminal";

pub mod gpui_func_tools;
pub mod terminal_element;

///Action for carrying the input to the PTY
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Input(pub String);

///Event to transmit the scroll from the element to the view
#[derive(Clone, Debug, PartialEq)]
pub struct ScrollTerminal(pub i32);

actions!(
    terminal,
    [Sigint, Escape, Del, Return, Left, Right, Up, Down, Tab, Clear, Paste, Deploy, Quit]
);
impl_internal_actions!(terminal, [Input, ScrollTerminal]);

///Initialize and register all of our action handlers
pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(Terminal::deploy);
    cx.add_action(Terminal::write_to_pty);
    cx.add_action(Terminal::send_sigint);
    cx.add_action(Terminal::escape);
    cx.add_action(Terminal::quit);
    cx.add_action(Terminal::del);
    cx.add_action(Terminal::carriage_return); //TODO figure out how to do this properly. Should we be checking the terminal mode?
    cx.add_action(Terminal::left);
    cx.add_action(Terminal::right);
    cx.add_action(Terminal::up);
    cx.add_action(Terminal::down);
    cx.add_action(Terminal::tab);
    cx.add_action(Terminal::paste);
    cx.add_action(Terminal::scroll_terminal);
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
    pty_tx: Notifier,
    term: Arc<FairMutex<Term<ZedListener>>>,
    title: String,
    has_new_content: bool,
    has_bell: bool, //Currently using iTerm bell, show bell emoji in tab until input is received
    cur_size: SizeInfo,
}

///Upward flowing events, for changing the title and such
pub enum Event {
    TitleChanged,
    CloseTerminal,
    Activate,
}

impl Entity for Terminal {
    type Event = Event;
}

impl Terminal {
    ///Create a new Terminal view. This spawns a task, a thread, and opens the TTY devices
    fn new(cx: &mut ViewContext<Self>, working_directory: Option<PathBuf>) -> Self {
        //Spawn a task so the Alacritty EventLoop can communicate with us in a view context
        let (events_tx, mut events_rx) = unbounded();
        cx.spawn_weak(|this, mut cx| async move {
            while let Some(event) = events_rx.next().await {
                match this.upgrade(&cx) {
                    Some(handle) => {
                        handle.update(&mut cx, |this, cx| {
                            this.process_terminal_event(event, cx);
                            cx.notify();
                        });
                    }
                    None => break,
                }
            }
        })
        .detach();

        let pty_config = PtyConfig {
            shell: Some(Program::Just("zsh".to_string())),
            working_directory,
            hold: false,
        };

        //Does this mangle the zed Env? I'm guessing it does... do child processes have a seperate ENV?
        let mut env: HashMap<String, String> = HashMap::new();
        //TODO: Properly set the current locale,
        env.insert("LC_ALL".to_string(), "en_US.UTF-8".to_string());

        let config = Config {
            pty_config: pty_config.clone(),
            env,
            ..Default::default()
        };

        setup_env(&config);

        //The details here don't matter, the terminal will be resized on the first layout
        //Set to something small for easier debugging
        let size_info = SizeInfo::new(200., 100.0, 5., 5., 0., 0., false);

        //Set up the terminal...
        let term = Term::new(&config, size_info, ZedListener(events_tx.clone()));
        let term = Arc::new(FairMutex::new(term));

        //Setup the pty...
        let pty = tty::new(&pty_config, &size_info, None).expect("Could not create tty");

        //And connect them together
        let event_loop = EventLoop::new(
            term.clone(),
            ZedListener(events_tx.clone()),
            pty,
            pty_config.hold,
            false,
        );

        //Kick things off
        let pty_tx = Notifier(event_loop.channel());
        let _io_thread = event_loop.spawn();
        Terminal {
            title: DEFAULT_TITLE.to_string(),
            term,
            pty_tx,
            has_new_content: false,
            has_bell: false,
            cur_size: size_info,
        }
    }

    ///Takes events from Alacritty and translates them to behavior on this view
    fn process_terminal_event(
        &mut self,
        event: alacritty_terminal::event::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            AlacTermEvent::Wakeup => {
                if !cx.is_self_focused() {
                    self.has_new_content = true; //Change tab content
                    cx.emit(Event::TitleChanged);
                } else {
                    cx.notify()
                }
            }
            AlacTermEvent::PtyWrite(out) => self.write_to_pty(&Input(out), cx),
            AlacTermEvent::MouseCursorDirty => {
                //Calculate new cursor style.
                //TODO
                //Check on correctly handling mouse events for terminals
                cx.platform().set_cursor_style(CursorStyle::Arrow); //???
            }
            AlacTermEvent::Title(title) => {
                self.title = title;
                cx.emit(Event::TitleChanged);
            }
            AlacTermEvent::ResetTitle => {
                self.title = DEFAULT_TITLE.to_string();
                cx.emit(Event::TitleChanged);
            }
            AlacTermEvent::ClipboardStore(_, data) => {
                cx.write_to_clipboard(ClipboardItem::new(data))
            }
            AlacTermEvent::ClipboardLoad(_, format) => self.write_to_pty(
                &Input(format(
                    &cx.read_from_clipboard()
                        .map(|ci| ci.text().to_string())
                        .unwrap_or("".to_string()),
                )),
                cx,
            ),
            AlacTermEvent::ColorRequest(index, format) => {
                let color = self.term.lock().colors()[index].unwrap_or_else(|| {
                    let term_style = &cx.global::<Settings>().theme.terminal;
                    match index {
                        0..=255 => to_alac_rgb(get_color_at_index(&(index as u8), term_style)),
                        //These additional values are required to match the Alacritty Colors object's behavior
                        256 => to_alac_rgb(term_style.foreground),
                        257 => to_alac_rgb(term_style.background),
                        258 => to_alac_rgb(term_style.cursor),
                        259 => to_alac_rgb(term_style.dim_black),
                        260 => to_alac_rgb(term_style.dim_red),
                        261 => to_alac_rgb(term_style.dim_green),
                        262 => to_alac_rgb(term_style.dim_yellow),
                        263 => to_alac_rgb(term_style.dim_blue),
                        264 => to_alac_rgb(term_style.dim_magenta),
                        265 => to_alac_rgb(term_style.dim_cyan),
                        266 => to_alac_rgb(term_style.dim_white),
                        267 => to_alac_rgb(term_style.bright_foreground),
                        268 => to_alac_rgb(term_style.black), //Dim Background, non-standard
                        _ => AlacRgb { r: 0, g: 0, b: 0 },
                    }
                });
                self.write_to_pty(&Input(format(color)), cx)
            }
            AlacTermEvent::CursorBlinkingChange => {
                //TODO: Set a timer to blink the cursor on and off
            }
            AlacTermEvent::Bell => {
                self.has_bell = true;
                cx.emit(Event::TitleChanged);
            }
            AlacTermEvent::Exit => self.quit(&Quit, cx),
        }
    }

    ///Resize the terminal and the PTY. This locks the terminal.
    fn set_size(&mut self, new_size: SizeInfo) {
        if new_size != self.cur_size {
            self.pty_tx.0.send(Msg::Resize(new_size)).ok();
            self.term.lock().resize(new_size);
            self.cur_size = new_size;
        }
    }

    ///Scroll the terminal. This locks the terminal
    fn scroll_terminal(&mut self, scroll: &ScrollTerminal, _: &mut ViewContext<Self>) {
        self.term.lock().scroll_display(Scroll::Delta(scroll.0));
    }

    ///Create a new Terminal in the current working directory or the user's home directory
    fn deploy(workspace: &mut Workspace, _: &Deploy, cx: &mut ViewContext<Workspace>) {
        let project = workspace.project().read(cx);

        let abs_path = project
            .active_entry()
            .and_then(|entry_id| project.worktree_for_entry(entry_id, cx))
            .and_then(|worktree_handle| worktree_handle.read(cx).as_local())
            .and_then(get_working_directory);

        workspace.add_item(Box::new(cx.add_view(|cx| Terminal::new(cx, abs_path))), cx);
    }

    ///Send the shutdown message to Alacritty
    fn shutdown_pty(&mut self) {
        self.pty_tx.0.send(Msg::Shutdown).ok();
    }

    ///Tell Zed to close us
    fn quit(&mut self, _: &Quit, cx: &mut ViewContext<Self>) {
        cx.emit(Event::CloseTerminal);
    }

    ///Attempt to paste the clipboard into the terminal
    fn paste(&mut self, _: &Paste, cx: &mut ViewContext<Self>) {
        if let Some(item) = cx.read_from_clipboard() {
            self.write_to_pty(&Input(item.text().to_owned()), cx);
        }
    }

    ///Write the Input payload to the tty. This locks the terminal so we can scroll it.
    fn write_to_pty(&mut self, input: &Input, cx: &mut ViewContext<Self>) {
        self.write_bytes_to_pty(input.0.clone().into_bytes(), cx);
    }

    ///Write the Input payload to the tty. This locks the terminal so we can scroll it.
    fn write_bytes_to_pty(&mut self, input: Vec<u8>, cx: &mut ViewContext<Self>) {
        //iTerm bell behavior, bell stays until terminal is interacted with
        self.has_bell = false;
        cx.emit(Event::TitleChanged);
        self.term.lock().scroll_display(Scroll::Bottom);
        self.pty_tx.notify(input);
    }

    ///Send the `up` key
    fn up(&mut self, _: &Up, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(UP_SEQ.to_string()), cx);
    }

    ///Send the `down` key
    fn down(&mut self, _: &Down, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(DOWN_SEQ.to_string()), cx);
    }

    ///Send the `tab` key
    fn tab(&mut self, _: &Tab, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(TAB_CHAR.to_string()), cx);
    }

    ///Send `SIGINT` (`ctrl-c`)
    fn send_sigint(&mut self, _: &Sigint, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(ETX_CHAR.to_string()), cx);
    }

    ///Send the `escape` key
    fn escape(&mut self, _: &Escape, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(ESC_CHAR.to_string()), cx);
    }

    ///Send the `delete` key. TODO: Difference between this and backspace?
    fn del(&mut self, _: &Del, cx: &mut ViewContext<Self>) {
        // self.write_to_pty(&Input("\x1b[3~".to_string()), cx)
        self.write_to_pty(&Input(DEL_CHAR.to_string()), cx);
    }

    ///Send a carriage return. TODO: May need to check the terminal mode.
    fn carriage_return(&mut self, _: &Return, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(CARRIAGE_RETURN_CHAR.to_string()), cx);
    }

    //Send the `left` key
    fn left(&mut self, _: &Left, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(LEFT_SEQ.to_string()), cx);
    }

    //Send the `right` key
    fn right(&mut self, _: &Right, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(RIGHT_SEQ.to_string()), cx);
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.shutdown_pty();
    }
}

impl View for Terminal {
    fn ui_name() -> &'static str {
        "Terminal"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> ElementBox {
        TerminalEl::new(cx.handle()).contained().boxed()
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Activate);
        self.has_new_content = false;
    }
}

impl Item for Terminal {
    fn tab_content(&self, tab_theme: &theme::Tab, cx: &gpui::AppContext) -> ElementBox {
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
            Label::new(self.title.clone(), tab_theme.label.clone())
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

//Convenience method for less lines
fn to_alac_rgb(color: Color) -> AlacRgb {
    AlacRgb {
        r: color.r,
        g: color.g,
        b: color.g,
    }
}

fn get_working_directory(wt: &LocalWorktree) -> Option<PathBuf> {
    Some(wt.abs_path().to_path_buf())
        .filter(|path| path.is_dir())
        .or_else(|| home_dir())
}

#[cfg(test)]
mod tests {

    use std::{path::Path, sync::atomic::AtomicUsize, time::Duration};

    use super::*;
    use alacritty_terminal::{grid::GridIterator, term::cell::Cell};
    use gpui::TestAppContext;
    use itertools::Itertools;
    use project::{FakeFs, Fs, RealFs, RemoveOptions, Worktree};

    ///Basic integration test, can we get the terminal to show up, execute a command,
    //and produce noticable output?
    #[gpui::test]
    async fn test_terminal(cx: &mut TestAppContext) {
        let terminal = cx.add_view(Default::default(), |cx| Terminal::new(cx, None));
        cx.set_condition_duration(Duration::from_secs(2));

        terminal.update(cx, |terminal, cx| {
            terminal.write_to_pty(&Input(("expr 3 + 4".to_string()).to_string()), cx);
            terminal.carriage_return(&Return, cx);
        });

        terminal
            .condition(cx, |terminal, _cx| {
                let term = terminal.term.clone();
                let content = grid_as_str(term.lock().renderable_content().display_iter);
                content.contains("7")
            })
            .await;
    }

    pub(crate) fn grid_as_str(grid_iterator: GridIterator<Cell>) -> String {
        let lines = grid_iterator.group_by(|i| i.point.line.0);
        lines
            .into_iter()
            .map(|(_, line)| line.map(|i| i.c).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }

    #[gpui::test]
    async fn single_file_worktree(cx: &mut TestAppContext) {
        let mut async_cx = cx.to_async();
        let http_client = client::test::FakeHttpClient::with_404_response();
        let client = client::Client::new(http_client.clone());
        let fake_fs = FakeFs::new(cx.background().clone());

        let path = Path::new("/file/");
        fake_fs.insert_file(path, "a".to_string()).await;

        let worktree_handle = Worktree::local(
            client,
            path,
            true,
            fake_fs,
            Arc::new(AtomicUsize::new(0)),
            &mut async_cx,
        )
        .await
        .ok()
        .unwrap();

        async_cx.update(|cx| {
            let wt = worktree_handle.read(cx).as_local().unwrap();
            let wd = get_working_directory(wt);
            assert!(wd.is_some());
            let path = wd.unwrap();
            //This should be the system's working directory, so querying the real file system is probably ok.
            assert!(path.is_dir());
            assert_eq!(path, home_dir().unwrap());
        });
    }

    #[gpui::test]
    async fn test_worktree_directory(cx: &mut TestAppContext) {
        let mut async_cx = cx.to_async();
        let http_client = client::test::FakeHttpClient::with_404_response();
        let client = client::Client::new(http_client.clone());

        let fs = RealFs;
        let mut test_wd = home_dir().unwrap();
        test_wd.push("dir");

        fs.create_dir(test_wd.as_path())
            .await
            .expect("File could not be created");

        let worktree_handle = Worktree::local(
            client,
            test_wd.clone(),
            true,
            Arc::new(RealFs),
            Arc::new(AtomicUsize::new(0)),
            &mut async_cx,
        )
        .await
        .ok()
        .unwrap();

        async_cx.update(|cx| {
            let wt = worktree_handle.read(cx).as_local().unwrap();
            let wd = get_working_directory(wt);
            assert!(wd.is_some());
            let path = wd.unwrap();
            assert!(path.is_dir());
            assert_eq!(path, test_wd);
        });

        //Clean up after ourselves.
        fs.remove_dir(
            test_wd.as_path(),
            RemoveOptions {
                recursive: false,
                ignore_if_not_exists: true,
            },
        )
        .await
        .ok()
        .expect("Could not remove test directory");
    }
}
