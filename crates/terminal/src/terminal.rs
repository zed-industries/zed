use alacritty_terminal::{
    config::{Config, Program, PtyConfig},
    event::{Event as AlacTermEvent, EventListener, Notify},
    event_loop::{EventLoop, Msg, Notifier},
    grid::Scroll,
    sync::FairMutex,
    term::{color::Rgb, SizeInfo},
    tty, Term,
};

use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    StreamExt,
};
use gpui::{
    actions, elements::*, impl_internal_actions, platform::CursorStyle, ClipboardItem, Entity,
    MutableAppContext, View, ViewContext,
};
use project::{Project, ProjectPath};
use settings::Settings;
use smallvec::SmallVec;
use std::sync::Arc;
use workspace::{Item, Workspace};

use crate::terminal_element::TerminalEl;

//ASCII Control characters on a keyboard
//Consts -> Structs -> Impls -> Functions, Vaguely in order of importance
const ETX_CHAR: char = 3_u8 as char; //'End of text', the control code for 'ctrl-c'
const TAB_CHAR: char = 9_u8 as char;
const CARRIAGE_RETURN_CHAR: char = 13_u8 as char;
const ESC_CHAR: char = 27_u8 as char;
const DEL_CHAR: char = 127_u8 as char;
const LEFT_SEQ: &str = "\x1b[D";
const RIGHT_SEQ: &str = "\x1b[C";
const UP_SEQ: &str = "\x1b[A";
const DOWN_SEQ: &str = "\x1b[B";
const DEFAULT_TITLE: &str = "Terminal";

pub mod terminal_element;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Input(pub String);

#[derive(Clone, Debug, PartialEq)]
pub struct ScrollTerminal(pub i32);

actions!(
    terminal,
    [Sigint, Escape, Del, Return, Left, Right, Up, Down, Tab, Clear, Paste, Deploy, Quit]
);
impl_internal_actions!(terminal, [Input, ScrollTerminal]);

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

#[derive(Clone)]
pub struct ZedListener(UnboundedSender<AlacTermEvent>);

impl EventListener for ZedListener {
    fn send_event(&self, event: AlacTermEvent) {
        self.0.unbounded_send(event).ok();
    }
}

///A terminal renderer.
pub struct Terminal {
    pty_tx: Notifier,
    term: Arc<FairMutex<Term<ZedListener>>>,
    title: String,
    has_new_content: bool,
    has_bell: bool, //Currently using iTerm bell, show bell emoji in tab until input is received
    cur_size: SizeInfo,
}

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
    fn new(cx: &mut ViewContext<Self>) -> Self {
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
            working_directory: None,
            hold: false,
        };

        let config = Config {
            pty_config: pty_config.clone(),
            ..Default::default()
        };

        //TODO figure out how to derive this better
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
                    //Need to figure out how to trigger a redraw when not in focus
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
                //TODO test this as well
                //TODO: change to getting the display colors, like alacrityy, instead of a default
                let color = self.term.lock().colors()[index].unwrap_or(Rgb::default());
                self.write_to_pty(&Input(format(color)), cx)
            }
            AlacTermEvent::CursorBlinkingChange => {
                //So, it's our job to set a timer and cause the cursor to blink here
                //Which means that I'm going to put this off until someone @ Zed looks at it
            }
            AlacTermEvent::Bell => {
                self.has_bell = true;
                cx.emit(Event::TitleChanged);
            }
            AlacTermEvent::Exit => self.quit(&Quit, cx),
        }
    }

    fn set_size(&mut self, new_size: SizeInfo) {
        if new_size != self.cur_size {
            self.pty_tx.0.send(Msg::Resize(new_size)).ok();
            self.term.lock().resize(new_size);
            self.cur_size = new_size;
        }
    }

    fn scroll_terminal(&mut self, scroll: &ScrollTerminal, _: &mut ViewContext<Self>) {
        self.term.lock().scroll_display(Scroll::Delta(scroll.0));
    }

    ///Create a new Terminal
    fn deploy(workspace: &mut Workspace, _: &Deploy, cx: &mut ViewContext<Workspace>) {
        workspace.add_item(Box::new(cx.add_view(|cx| Terminal::new(cx))), cx);
    }

    ///Send the shutdown message to Alacritty
    fn shutdown_pty(&mut self) {
        self.pty_tx.0.send(Msg::Shutdown).ok();
    }

    fn quit(&mut self, _: &Quit, cx: &mut ViewContext<Self>) {
        cx.emit(Event::CloseTerminal);
    }

    fn paste(&mut self, _: &Paste, cx: &mut ViewContext<Self>) {
        if let Some(item) = cx.read_from_clipboard() {
            self.write_to_pty(&Input(item.text().to_owned()), cx);
        }
    }

    fn write_to_pty(&mut self, input: &Input, cx: &mut ViewContext<Self>) {
        //iTerm bell behavior, bell stays until terminal is interacted with
        self.has_bell = false;
        self.term.lock().scroll_display(Scroll::Bottom);
        cx.emit(Event::TitleChanged);
        self.pty_tx.notify(input.0.clone().into_bytes());
    }

    fn up(&mut self, _: &Up, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(UP_SEQ.to_string()), cx);
    }

    fn down(&mut self, _: &Down, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(DOWN_SEQ.to_string()), cx);
    }

    fn tab(&mut self, _: &Tab, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(TAB_CHAR.to_string()), cx);
    }

    fn send_sigint(&mut self, _: &Sigint, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(ETX_CHAR.to_string()), cx);
    }

    fn escape(&mut self, _: &Escape, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(ESC_CHAR.to_string()), cx);
    }

    fn del(&mut self, _: &Del, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(DEL_CHAR.to_string()), cx);
    }

    fn carriage_return(&mut self, _: &Return, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(CARRIAGE_RETURN_CHAR.to_string()), cx);
    }

    fn left(&mut self, _: &Left, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(LEFT_SEQ.to_string()), cx);
    }

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
        TerminalEl::new(cx.handle())
            .contained()
            // .with_style(theme.terminal.container)
            .boxed()
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
                Svg::new("icons/zap.svg")
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terminal_element::build_chunks;
    use gpui::TestAppContext;

    #[gpui::test]
    async fn test_terminal(cx: &mut TestAppContext) {
        let terminal = cx.add_view(Default::default(), |cx| Terminal::new(cx));

        terminal.update(cx, |terminal, cx| {
            terminal.write_to_pty(&Input(("expr 3 + 4".to_string()).to_string()), cx);
            terminal.carriage_return(&Return, cx);
        });

        terminal
            .condition(cx, |terminal, _cx| {
                let term = terminal.term.clone();
                let (chunks, _) = build_chunks(
                    term.lock().renderable_content().display_iter,
                    &Default::default(),
                );
                let content = chunks.iter().map(|e| e.0.trim()).collect::<String>();
                content.contains("7")
            })
            .await;
    }
}
