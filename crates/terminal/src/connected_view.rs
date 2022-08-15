use alacritty_terminal::term::TermMode;
use context_menu::{ContextMenu, ContextMenuItem};
use gpui::{
    actions,
    elements::{ChildView, ParentElement, Stack},
    geometry::vector::Vector2F,
    impl_internal_actions,
    keymap::Keystroke,
    AnyViewHandle, AppContext, Element, ElementBox, ModelHandle, MutableAppContext, View,
    ViewContext, ViewHandle,
};
use workspace::pane;

use crate::{connected_el::TerminalEl, Event, Terminal};

///Event to transmit the scroll from the element to the view
#[derive(Clone, Debug, PartialEq)]
pub struct ScrollTerminal(pub i32);

#[derive(Clone, PartialEq)]
pub struct DeployContextMenu {
    pub position: Vector2F,
}

actions!(
    terminal,
    [Up, Down, CtrlC, Escape, Enter, Clear, Copy, Paste,]
);
impl_internal_actions!(project_panel, [DeployContextMenu]);

pub fn init(cx: &mut MutableAppContext) {
    //Global binding overrrides
    cx.add_action(ConnectedView::ctrl_c);
    cx.add_action(ConnectedView::up);
    cx.add_action(ConnectedView::down);
    cx.add_action(ConnectedView::escape);
    cx.add_action(ConnectedView::enter);
    //Useful terminal views
    cx.add_action(ConnectedView::deploy_context_menu);
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
    context_menu: ViewHandle<ContextMenu>,
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
                if !cx.is_self_focused() {
                    this.has_new_content = true;
                    cx.notify();
                    cx.emit(Event::Wakeup);
                }
            }
            Event::Bell => {
                this.has_bell = true;
                cx.emit(Event::Wakeup);
            }

            _ => cx.emit(*event),
        })
        .detach();

        Self {
            terminal,
            has_new_content: true,
            has_bell: false,
            modal,
            context_menu: cx.add_view(ContextMenu::new),
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
        cx.emit(Event::Wakeup);
    }

    pub fn deploy_context_menu(&mut self, action: &DeployContextMenu, cx: &mut ViewContext<Self>) {
        let menu_entries = vec![
            ContextMenuItem::item("Clear Buffer", Clear),
            ContextMenuItem::item("Close Terminal", pane::CloseActiveItem),
        ];

        self.context_menu
            .update(cx, |menu, cx| menu.show(action.position, menu_entries, cx));

        cx.notify();
    }

    fn clear(&mut self, _: &Clear, cx: &mut ViewContext<Self>) {
        self.terminal.update(cx, |term, _| term.clear());
        cx.notify();
    }

    ///Attempt to paste the clipboard into the terminal
    fn copy(&mut self, _: &Copy, cx: &mut ViewContext<Self>) {
        self.terminal.update(cx, |term, _| term.copy())
    }

    ///Attempt to paste the clipboard into the terminal
    fn paste(&mut self, _: &Paste, cx: &mut ViewContext<Self>) {
        if let Some(item) = cx.read_from_clipboard() {
            self.terminal.read(cx).paste(item.text());
        }
    }

    ///Synthesize the keyboard event corresponding to 'up'
    fn up(&mut self, _: &Up, cx: &mut ViewContext<Self>) {
        self.clear_bel(cx);
        self.terminal
            .read(cx)
            .try_keystroke(&Keystroke::parse("up").unwrap());
    }

    ///Synthesize the keyboard event corresponding to 'down'
    fn down(&mut self, _: &Down, cx: &mut ViewContext<Self>) {
        self.clear_bel(cx);
        self.terminal
            .read(cx)
            .try_keystroke(&Keystroke::parse("down").unwrap());
    }

    ///Synthesize the keyboard event corresponding to 'ctrl-c'
    fn ctrl_c(&mut self, _: &CtrlC, cx: &mut ViewContext<Self>) {
        self.clear_bel(cx);
        self.terminal
            .read(cx)
            .try_keystroke(&Keystroke::parse("ctrl-c").unwrap());
    }

    ///Synthesize the keyboard event corresponding to 'escape'
    fn escape(&mut self, _: &Escape, cx: &mut ViewContext<Self>) {
        self.clear_bel(cx);
        self.terminal
            .read(cx)
            .try_keystroke(&Keystroke::parse("escape").unwrap());
    }

    ///Synthesize the keyboard event corresponding to 'enter'
    fn enter(&mut self, _: &Enter, cx: &mut ViewContext<Self>) {
        self.clear_bel(cx);
        self.terminal
            .read(cx)
            .try_keystroke(&Keystroke::parse("enter").unwrap());
    }
}

impl View for ConnectedView {
    fn ui_name() -> &'static str {
        "Terminal"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> ElementBox {
        let terminal_handle = self.terminal.clone().downgrade();

        let self_id = cx.view_id();
        let focused = cx
            .focused_view_id(cx.window_id())
            .filter(|view_id| *view_id == self_id)
            .is_some();

        Stack::new()
            .with_child(
                TerminalEl::new(cx.handle(), terminal_handle, self.modal, focused)
                    .contained()
                    .boxed(),
            )
            .with_child(ChildView::new(&self.context_menu).boxed())
            .boxed()
    }

    fn on_focus_in(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        self.has_new_content = false;
        self.terminal.read(cx).focus_in();
        cx.notify();
    }

    fn on_focus_out(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        self.terminal.read(cx).focus_out();
        cx.notify();
    }

    fn selected_text_range(&self, cx: &AppContext) -> Option<std::ops::Range<usize>> {
        if self
            .terminal
            .read(cx)
            .last_mode
            .contains(TermMode::ALT_SCREEN)
        {
            None
        } else {
            Some(0..0)
        }
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

    fn keymap_context(&self, _: &gpui::AppContext) -> gpui::keymap::Context {
        let mut context = Self::default_keymap_context();
        if self.modal {
            context.set.insert("ModalTerminal".into());
        }
        context
    }
}
