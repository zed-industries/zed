use gpui::{
    actions, keymap::Keystroke, AppContext, ClipboardItem, Element, ElementBox, ModelHandle,
    MutableAppContext, View, ViewContext,
};

use crate::{
    connected_el::TerminalEl,
    model::{Event, Terminal},
};

///Event to transmit the scroll from the element to the view
#[derive(Clone, Debug, PartialEq)]
pub struct ScrollTerminal(pub i32);

actions!(
    terminal,
    [Up, Down, CtrlC, Escape, Enter, Clear, Copy, Paste,]
);

pub fn init(cx: &mut MutableAppContext) {
    //Global binding overrrides
    cx.add_action(ConnectedView::ctrl_c);
    cx.add_action(ConnectedView::up);
    cx.add_action(ConnectedView::down);
    cx.add_action(ConnectedView::escape);
    cx.add_action(ConnectedView::enter);
    //Useful terminal views
    cx.add_action(ConnectedView::copy);
    cx.add_action(ConnectedView::paste);
    cx.add_action(ConnectedView::clear);
}

///A terminal view, maintains the PTY's file handles and communicates with the terminal
pub struct ConnectedView {
    terminal: ModelHandle<Terminal>,
    has_new_content: bool,
    //Currently using iTerm bell, show bell emoji in tab until input is received
    has_bell: bool,
    // Only for styling purposes. Doesn't effect behavior
    modal: bool,
}

impl ConnectedView {
    pub fn from_terminal(
        terminal: ModelHandle<Terminal>,
        modal: bool,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        cx.observe(&terminal, |_, _, cx| cx.notify()).detach();
        cx.subscribe(&terminal, |this, _, event, cx| match event {
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

        Self {
            terminal,
            has_new_content: true,
            has_bell: false,
            modal,
        }
    }

    pub fn handle(&self) -> ModelHandle<Terminal> {
        self.terminal.clone()
    }

    pub fn has_new_content(&self) -> bool {
        self.has_new_content
    }

    pub fn has_bell(&self) -> bool {
        self.has_bell
    }

    pub fn clear_bel(&mut self, cx: &mut ViewContext<ConnectedView>) {
        self.has_bell = false;
        cx.emit(Event::TitleChanged);
    }

    fn clear(&mut self, _: &Clear, cx: &mut ViewContext<Self>) {
        self.terminal.read(cx).clear();
    }

    ///Attempt to paste the clipboard into the terminal
    fn copy(&mut self, _: &Copy, cx: &mut ViewContext<Self>) {
        self.terminal
            .read(cx)
            .copy()
            .map(|text| cx.write_to_clipboard(ClipboardItem::new(text)));
    }

    ///Attempt to paste the clipboard into the terminal
    fn paste(&mut self, _: &Paste, cx: &mut ViewContext<Self>) {
        cx.read_from_clipboard().map(|item| {
            self.terminal.read(cx).paste(item.text());
        });
    }

    ///Synthesize the keyboard event corresponding to 'up'
    fn up(&mut self, _: &Up, cx: &mut ViewContext<Self>) {
        self.terminal
            .read(cx)
            .try_keystroke(&Keystroke::parse("up").unwrap());
    }

    ///Synthesize the keyboard event corresponding to 'down'
    fn down(&mut self, _: &Down, cx: &mut ViewContext<Self>) {
        self.terminal
            .read(cx)
            .try_keystroke(&Keystroke::parse("down").unwrap());
    }

    ///Synthesize the keyboard event corresponding to 'ctrl-c'
    fn ctrl_c(&mut self, _: &CtrlC, cx: &mut ViewContext<Self>) {
        self.terminal
            .read(cx)
            .try_keystroke(&Keystroke::parse("ctrl-c").unwrap());
    }

    ///Synthesize the keyboard event corresponding to 'escape'
    fn escape(&mut self, _: &Escape, cx: &mut ViewContext<Self>) {
        self.terminal
            .read(cx)
            .try_keystroke(&Keystroke::parse("escape").unwrap());
    }

    ///Synthesize the keyboard event corresponding to 'enter'
    fn enter(&mut self, _: &Enter, cx: &mut ViewContext<Self>) {
        self.terminal
            .read(cx)
            .try_keystroke(&Keystroke::parse("enter").unwrap());
    }
}

impl View for ConnectedView {
    fn ui_name() -> &'static str {
        "Connected Terminal View"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> ElementBox {
        let terminal_handle = self.terminal.clone().downgrade();
        TerminalEl::new(cx.handle(), terminal_handle, self.modal)
            .contained()
            .boxed()
    }

    fn on_focus(&mut self, _cx: &mut ViewContext<Self>) {
        self.has_new_content = false;
    }

    fn selected_text_range(&self, _: &AppContext) -> Option<std::ops::Range<usize>> {
        Some(0..0)
    }

    fn replace_text_in_range(
        &mut self,
        _: Option<std::ops::Range<usize>>,
        text: &str,
        cx: &mut ViewContext<Self>,
    ) {
        self.terminal
            .update(cx, |terminal, _| terminal.write_to_pty(text.into()));
    }
}
