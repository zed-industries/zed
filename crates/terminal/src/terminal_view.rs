use std::{ops::RangeInclusive, time::Duration};

use alacritty_terminal::{index::Point, term::TermMode};
use context_menu::{ContextMenu, ContextMenuItem};
use gpui::{
    actions,
    elements::{ChildView, ParentElement, Stack},
    geometry::vector::Vector2F,
    impl_internal_actions,
    keymap::Keystroke,
    AnyViewHandle, AppContext, Element, ElementBox, Entity, ModelHandle, MutableAppContext, Task,
    View, ViewContext, ViewHandle,
};
use settings::{Settings, TerminalBlink};
use smol::Timer;
use workspace::pane;

use crate::{terminal_element::TerminalElement, Event, Terminal};

const CURSOR_BLINK_INTERVAL: Duration = Duration::from_millis(500);

///Event to transmit the scroll from the element to the view
#[derive(Clone, Debug, PartialEq)]
pub struct ScrollTerminal(pub i32);

#[derive(Clone, PartialEq)]
pub struct DeployContextMenu {
    pub position: Vector2F,
}

actions!(
    terminal,
    [
        Up,
        Down,
        CtrlC,
        Escape,
        Enter,
        Clear,
        Copy,
        Paste,
        ShowCharacterPalette,
        SearchTest
    ]
);
impl_internal_actions!(project_panel, [DeployContextMenu]);

pub fn init(cx: &mut MutableAppContext) {
    //Global binding overrrides
    cx.add_action(TerminalView::ctrl_c);
    cx.add_action(TerminalView::up);
    cx.add_action(TerminalView::down);
    cx.add_action(TerminalView::escape);
    cx.add_action(TerminalView::enter);
    //Useful terminal views
    cx.add_action(TerminalView::deploy_context_menu);
    cx.add_action(TerminalView::copy);
    cx.add_action(TerminalView::paste);
    cx.add_action(TerminalView::clear);
    cx.add_action(TerminalView::show_character_palette);
}

///A terminal view, maintains the PTY's file handles and communicates with the terminal
pub struct TerminalView {
    terminal: ModelHandle<Terminal>,
    has_new_content: bool,
    //Currently using iTerm bell, show bell emoji in tab until input is received
    has_bell: bool,
    // Only for styling purposes. Doesn't effect behavior
    modal: bool,
    context_menu: ViewHandle<ContextMenu>,
    blink_state: bool,
    blinking_on: bool,
    blinking_paused: bool,
    blink_epoch: usize,
}

impl Entity for TerminalView {
    type Event = Event;
}

impl TerminalView {
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
            Event::BlinkChanged => this.blinking_on = !this.blinking_on,
            _ => cx.emit(*event),
        })
        .detach();

        Self {
            terminal,
            has_new_content: true,
            has_bell: false,
            modal,
            context_menu: cx.add_view(ContextMenu::new),
            blink_state: true,
            blinking_on: false,
            blinking_paused: false,
            blink_epoch: 0,
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

    pub fn clear_bel(&mut self, cx: &mut ViewContext<TerminalView>) {
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

    fn show_character_palette(&mut self, _: &ShowCharacterPalette, cx: &mut ViewContext<Self>) {
        if !self
            .terminal
            .read(cx)
            .last_mode
            .contains(TermMode::ALT_SCREEN)
        {
            cx.show_character_palette();
        } else {
            self.terminal.update(cx, |term, _| {
                term.try_keystroke(&Keystroke::parse("ctrl-cmd-space").unwrap())
            });
        }
    }

    fn clear(&mut self, _: &Clear, cx: &mut ViewContext<Self>) {
        self.terminal.update(cx, |term, _| term.clear());
        cx.notify();
    }

    pub fn should_show_cursor(
        &self,
        focused: bool,
        cx: &mut gpui::RenderContext<'_, Self>,
    ) -> bool {
        //Don't blink the cursor when not focused, blinking is disabled, or paused
        if !focused
            || !self.blinking_on
            || self.blinking_paused
            || self
                .terminal
                .read(cx)
                .last_mode
                .contains(TermMode::ALT_SCREEN)
        {
            return true;
        }

        let setting = {
            let settings = cx.global::<Settings>();
            settings
                .terminal_overrides
                .blinking
                .clone()
                .unwrap_or(TerminalBlink::TerminalControlled)
        };

        match setting {
            //If the user requested to never blink, don't blink it.
            TerminalBlink::Off => true,
            //If the terminal is controlling it, check terminal mode
            TerminalBlink::TerminalControlled | TerminalBlink::On => self.blink_state,
        }
    }

    fn blink_cursors(&mut self, epoch: usize, cx: &mut ViewContext<Self>) {
        if epoch == self.blink_epoch && !self.blinking_paused {
            self.blink_state = !self.blink_state;
            cx.notify();

            let epoch = self.next_blink_epoch();
            cx.spawn(|this, mut cx| {
                let this = this.downgrade();
                async move {
                    Timer::after(CURSOR_BLINK_INTERVAL).await;
                    if let Some(this) = this.upgrade(&cx) {
                        this.update(&mut cx, |this, cx| this.blink_cursors(epoch, cx));
                    }
                }
            })
            .detach();
        }
    }

    pub fn pause_cursor_blinking(&mut self, cx: &mut ViewContext<Self>) {
        self.blink_state = true;
        cx.notify();

        let epoch = self.next_blink_epoch();
        cx.spawn(|this, mut cx| {
            let this = this.downgrade();
            async move {
                Timer::after(CURSOR_BLINK_INTERVAL).await;
                if let Some(this) = this.upgrade(&cx) {
                    this.update(&mut cx, |this, cx| this.resume_cursor_blinking(epoch, cx))
                }
            }
        })
        .detach();
    }

    pub fn find_matches(
        &mut self,
        query: project::search::SearchQuery,
        cx: &mut ViewContext<Self>,
    ) -> Task<Vec<RangeInclusive<Point>>> {
        self.terminal
            .update(cx, |term, cx| term.find_matches(query, cx))
    }

    pub fn terminal(&self) -> &ModelHandle<Terminal> {
        &self.terminal
    }

    fn next_blink_epoch(&mut self) -> usize {
        self.blink_epoch += 1;
        self.blink_epoch
    }

    fn resume_cursor_blinking(&mut self, epoch: usize, cx: &mut ViewContext<Self>) {
        if epoch == self.blink_epoch {
            self.blinking_paused = false;
            self.blink_cursors(epoch, cx);
        }
    }

    ///Attempt to paste the clipboard into the terminal
    fn copy(&mut self, _: &Copy, cx: &mut ViewContext<Self>) {
        self.terminal.update(cx, |term, _| term.copy())
    }

    ///Attempt to paste the clipboard into the terminal
    fn paste(&mut self, _: &Paste, cx: &mut ViewContext<Self>) {
        if let Some(item) = cx.read_from_clipboard() {
            self.terminal
                .update(cx, |terminal, _cx| terminal.paste(item.text()));
        }
    }

    ///Synthesize the keyboard event corresponding to 'up'
    fn up(&mut self, _: &Up, cx: &mut ViewContext<Self>) {
        self.clear_bel(cx);
        self.terminal.update(cx, |term, _| {
            term.try_keystroke(&Keystroke::parse("up").unwrap())
        });
    }

    ///Synthesize the keyboard event corresponding to 'down'
    fn down(&mut self, _: &Down, cx: &mut ViewContext<Self>) {
        self.clear_bel(cx);
        self.terminal.update(cx, |term, _| {
            term.try_keystroke(&Keystroke::parse("down").unwrap())
        });
    }

    ///Synthesize the keyboard event corresponding to 'ctrl-c'
    fn ctrl_c(&mut self, _: &CtrlC, cx: &mut ViewContext<Self>) {
        self.clear_bel(cx);
        self.terminal.update(cx, |term, _| {
            term.try_keystroke(&Keystroke::parse("ctrl-c").unwrap())
        });
    }

    ///Synthesize the keyboard event corresponding to 'escape'
    fn escape(&mut self, _: &Escape, cx: &mut ViewContext<Self>) {
        self.clear_bel(cx);
        self.terminal.update(cx, |term, _| {
            term.try_keystroke(&Keystroke::parse("escape").unwrap())
        });
    }

    ///Synthesize the keyboard event corresponding to 'enter'
    fn enter(&mut self, _: &Enter, cx: &mut ViewContext<Self>) {
        self.clear_bel(cx);
        self.terminal.update(cx, |term, _| {
            term.try_keystroke(&Keystroke::parse("enter").unwrap())
        });
    }
}

impl View for TerminalView {
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
                TerminalElement::new(
                    cx.handle(),
                    terminal_handle,
                    self.modal,
                    focused,
                    self.should_show_cursor(focused, cx),
                )
                .contained()
                .boxed(),
            )
            .with_child(ChildView::new(&self.context_menu).boxed())
            .boxed()
    }

    fn on_focus_in(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        self.has_new_content = false;
        self.terminal.read(cx).focus_in();
        self.blink_cursors(self.blink_epoch, cx);
        cx.notify();
    }

    fn on_focus_out(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        self.terminal.read(cx).focus_out();
        cx.notify();
    }

    //IME stuff
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
        self.terminal.update(cx, |terminal, _| {
            terminal.input(text.into());
        });
    }

    fn keymap_context(&self, cx: &gpui::AppContext) -> gpui::keymap::Context {
        let mut context = Self::default_keymap_context();
        if self.modal {
            context.set.insert("ModalTerminal".into());
        }
        let mode = self.terminal.read(cx).last_mode;
        context.map.insert(
            "screen".to_string(),
            (if mode.contains(TermMode::ALT_SCREEN) {
                "alt"
            } else {
                "normal"
            })
            .to_string(),
        );

        if mode.contains(TermMode::APP_CURSOR) {
            context.set.insert("DECCKM".to_string());
        }
        if mode.contains(TermMode::APP_KEYPAD) {
            context.set.insert("DECPAM".to_string());
        }
        //Note the ! here
        if !mode.contains(TermMode::APP_KEYPAD) {
            context.set.insert("DECPNM".to_string());
        }
        if mode.contains(TermMode::SHOW_CURSOR) {
            context.set.insert("DECTCEM".to_string());
        }
        if mode.contains(TermMode::LINE_WRAP) {
            context.set.insert("DECAWM".to_string());
        }
        if mode.contains(TermMode::ORIGIN) {
            context.set.insert("DECOM".to_string());
        }
        if mode.contains(TermMode::INSERT) {
            context.set.insert("IRM".to_string());
        }
        //LNM is apparently the name for this. https://vt100.net/docs/vt510-rm/LNM.html
        if mode.contains(TermMode::LINE_FEED_NEW_LINE) {
            context.set.insert("LNM".to_string());
        }
        if mode.contains(TermMode::FOCUS_IN_OUT) {
            context.set.insert("report_focus".to_string());
        }
        if mode.contains(TermMode::ALTERNATE_SCROLL) {
            context.set.insert("alternate_scroll".to_string());
        }
        if mode.contains(TermMode::BRACKETED_PASTE) {
            context.set.insert("bracketed_paste".to_string());
        }
        if mode.intersects(TermMode::MOUSE_MODE) {
            context.set.insert("any_mouse_reporting".to_string());
        }
        {
            let mouse_reporting = if mode.contains(TermMode::MOUSE_REPORT_CLICK) {
                "click"
            } else if mode.contains(TermMode::MOUSE_DRAG) {
                "drag"
            } else if mode.contains(TermMode::MOUSE_MOTION) {
                "motion"
            } else {
                "off"
            };
            context
                .map
                .insert("mouse_reporting".to_string(), mouse_reporting.to_string());
        }
        {
            let format = if mode.contains(TermMode::SGR_MOUSE) {
                "sgr"
            } else if mode.contains(TermMode::UTF8_MOUSE) {
                "utf8"
            } else {
                "normal"
            };
            context
                .map
                .insert("mouse_format".to_string(), format.to_string());
        }
        context
    }
}
