mod persistence;
pub mod terminal_element;
pub mod terminal_panel;
pub mod terminal_scrollbar;
pub mod terminal_tab_tooltip;

use collections::HashSet;
use editor::{
    actions::SelectAll,
    scroll::{Autoscroll, ScrollbarAutoHide},
    Editor, EditorSettings,
};
use futures::{stream::FuturesUnordered, StreamExt};
use gpui::{
    anchored, deferred, div, impl_actions, AnyElement, AppContext, DismissEvent, EventEmitter,
    FocusHandle, FocusableView, KeyContext, KeyDownEvent, Keystroke, Model, MouseButton,
    MouseDownEvent, Pixels, Render, ScrollWheelEvent, Stateful, Styled, Subscription, Task, View,
    VisualContext, WeakModel, WeakView,
};
use language::Bias;
use persistence::TERMINAL_DB;
use project::{search::SearchQuery, terminals::TerminalKind, Fs, Metadata, Project};
use schemars::JsonSchema;
use terminal::{
    alacritty_terminal::{
        index::Point,
        term::{search::RegexSearch, TermMode},
    },
    terminal_settings::{self, CursorShape, TerminalBlink, TerminalSettings, WorkingDirectory},
    Clear, Copy, Event, MaybeNavigationTarget, Paste, ScrollLineDown, ScrollLineUp, ScrollPageDown,
    ScrollPageUp, ScrollToBottom, ScrollToTop, ShowCharacterPalette, TaskStatus, Terminal,
    TerminalSize, ToggleViMode,
};
use terminal_element::{is_blank, TerminalElement};
use terminal_panel::TerminalPanel;
use terminal_scrollbar::TerminalScrollHandle;
use terminal_tab_tooltip::TerminalTooltip;
use ui::{
    h_flex, prelude::*, ContextMenu, Icon, IconName, Label, Scrollbar, ScrollbarState, Tooltip,
};
use util::{
    paths::{PathWithPosition, SanitizedPath},
    ResultExt,
};
use workspace::{
    item::{
        BreadcrumbText, Item, ItemEvent, SerializableItem, TabContentParams, TabTooltipContent,
    },
    register_serializable_item,
    searchable::{SearchEvent, SearchOptions, SearchableItem, SearchableItemHandle},
    CloseActiveItem, NewCenterTerminal, NewTerminal, OpenVisible, ToolbarItemLocation, Workspace,
    WorkspaceId,
};

use anyhow::Context;
use serde::Deserialize;
use settings::{Settings, SettingsStore};
use smol::Timer;
use zed_actions::InlineAssist;

use std::{
    cmp,
    ops::RangeInclusive,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
    time::Duration,
};

const REGEX_SPECIAL_CHARS: &[char] = &[
    '\\', '.', '*', '+', '?', '|', '(', ')', '[', ']', '{', '}', '^', '$',
];

const CURSOR_BLINK_INTERVAL: Duration = Duration::from_millis(500);

const GIT_DIFF_PATH_PREFIXES: &[char] = &['a', 'b'];

/// Event to transmit the scroll from the element to the view
#[derive(Clone, Debug, PartialEq)]
pub struct ScrollTerminal(pub i32);

#[derive(Clone, Debug, Default, Deserialize, JsonSchema, PartialEq)]
pub struct SendText(String);

#[derive(Clone, Debug, Default, Deserialize, JsonSchema, PartialEq)]
pub struct SendKeystroke(String);

impl_actions!(terminal, [SendText, SendKeystroke]);

pub fn init(cx: &mut AppContext) {
    terminal_panel::init(cx);
    terminal::init(cx);

    register_serializable_item::<TerminalView>(cx);

    cx.observe_new_views(|workspace: &mut Workspace, _cx| {
        workspace.register_action(TerminalView::deploy);
    })
    .detach();
}

pub struct BlockProperties {
    pub height: u8,
    pub render: Box<dyn Send + Fn(&mut BlockContext) -> AnyElement>,
}

pub struct BlockContext<'a, 'b> {
    pub context: &'b mut WindowContext<'a>,
    pub dimensions: TerminalSize,
}

///A terminal view, maintains the PTY's file handles and communicates with the terminal
pub struct TerminalView {
    terminal: Model<Terminal>,
    workspace: WeakView<Workspace>,
    project: WeakModel<Project>,
    focus_handle: FocusHandle,
    //Currently using iTerm bell, show bell emoji in tab until input is received
    has_bell: bool,
    context_menu: Option<(View<ContextMenu>, gpui::Point<Pixels>, Subscription)>,
    cursor_shape: CursorShape,
    blink_state: bool,
    blinking_terminal_enabled: bool,
    blinking_paused: bool,
    blink_epoch: usize,
    can_navigate_to_selected_word: bool,
    workspace_id: Option<WorkspaceId>,
    show_breadcrumbs: bool,
    block_below_cursor: Option<Rc<BlockProperties>>,
    scroll_top: Pixels,
    scrollbar_state: ScrollbarState,
    scroll_handle: TerminalScrollHandle,
    show_scrollbar: bool,
    hide_scrollbar_task: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
    _terminal_subscriptions: Vec<Subscription>,
}

impl EventEmitter<Event> for TerminalView {}
impl EventEmitter<ItemEvent> for TerminalView {}
impl EventEmitter<SearchEvent> for TerminalView {}

impl FocusableView for TerminalView {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl TerminalView {
    ///Create a new Terminal in the current working directory or the user's home directory
    pub fn deploy(
        workspace: &mut Workspace,
        _: &NewCenterTerminal,
        cx: &mut ViewContext<Workspace>,
    ) {
        let working_directory = default_working_directory(workspace, cx);
        TerminalPanel::add_center_terminal(workspace, TerminalKind::Shell(working_directory), cx)
            .detach_and_log_err(cx);
    }

    pub fn new(
        terminal: Model<Terminal>,
        workspace: WeakView<Workspace>,
        workspace_id: Option<WorkspaceId>,
        project: WeakModel<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let workspace_handle = workspace.clone();
        let terminal_subscriptions = subscribe_for_terminal_events(&terminal, workspace, cx);

        let focus_handle = cx.focus_handle();
        let focus_in = cx.on_focus_in(&focus_handle, |terminal_view, cx| {
            terminal_view.focus_in(cx);
        });
        let focus_out = cx.on_focus_out(&focus_handle, |terminal_view, _event, cx| {
            terminal_view.focus_out(cx);
        });
        let cursor_shape = TerminalSettings::get_global(cx)
            .cursor_shape
            .unwrap_or_default();

        let scroll_handle = TerminalScrollHandle::new(terminal.read(cx));

        Self {
            terminal,
            workspace: workspace_handle,
            project,
            has_bell: false,
            focus_handle,
            context_menu: None,
            cursor_shape,
            blink_state: true,
            blinking_terminal_enabled: false,
            blinking_paused: false,
            blink_epoch: 0,
            can_navigate_to_selected_word: false,
            workspace_id,
            show_breadcrumbs: TerminalSettings::get_global(cx).toolbar.breadcrumbs,
            block_below_cursor: None,
            scroll_top: Pixels::ZERO,
            scrollbar_state: ScrollbarState::new(scroll_handle.clone()),
            scroll_handle,
            show_scrollbar: !Self::should_autohide_scrollbar(cx),
            hide_scrollbar_task: None,
            _subscriptions: vec![
                focus_in,
                focus_out,
                cx.observe_global::<SettingsStore>(Self::settings_changed),
            ],
            _terminal_subscriptions: terminal_subscriptions,
        }
    }

    pub fn model(&self) -> &Model<Terminal> {
        &self.terminal
    }

    pub fn has_bell(&self) -> bool {
        self.has_bell
    }

    pub fn clear_bell(&mut self, cx: &mut ViewContext<TerminalView>) {
        self.has_bell = false;
        cx.emit(Event::Wakeup);
    }

    pub fn deploy_context_menu(
        &mut self,
        position: gpui::Point<Pixels>,
        cx: &mut ViewContext<Self>,
    ) {
        let assistant_enabled = self
            .workspace
            .upgrade()
            .and_then(|workspace| workspace.read(cx).panel::<TerminalPanel>(cx))
            .map_or(false, |terminal_panel| {
                terminal_panel.read(cx).assistant_enabled()
            });
        let context_menu = ContextMenu::build(cx, |menu, _| {
            menu.context(self.focus_handle.clone())
                .action("New Terminal", Box::new(NewTerminal))
                .separator()
                .action("Copy", Box::new(Copy))
                .action("Paste", Box::new(Paste))
                .action("Select All", Box::new(SelectAll))
                .action("Clear", Box::new(Clear))
                .when(assistant_enabled, |menu| {
                    menu.separator()
                        .action("Inline Assist", Box::new(InlineAssist::default()))
                })
                .separator()
                .action("Close", Box::new(CloseActiveItem { save_intent: None }))
        });

        cx.focus_view(&context_menu);
        let subscription =
            cx.subscribe(&context_menu, |this, _, _: &DismissEvent, cx| {
                if this.context_menu.as_ref().is_some_and(|context_menu| {
                    context_menu.0.focus_handle(cx).contains_focused(cx)
                }) {
                    cx.focus_self();
                }
                this.context_menu.take();
                cx.notify();
            });

        self.context_menu = Some((context_menu, position, subscription));
    }

    fn settings_changed(&mut self, cx: &mut ViewContext<Self>) {
        let settings = TerminalSettings::get_global(cx);
        self.show_breadcrumbs = settings.toolbar.breadcrumbs;

        let new_cursor_shape = settings.cursor_shape.unwrap_or_default();
        let old_cursor_shape = self.cursor_shape;
        if old_cursor_shape != new_cursor_shape {
            self.cursor_shape = new_cursor_shape;
            self.terminal.update(cx, |term, _| {
                term.set_cursor_shape(self.cursor_shape);
            });
        }

        cx.notify();
    }

    fn show_character_palette(&mut self, _: &ShowCharacterPalette, cx: &mut ViewContext<Self>) {
        if self
            .terminal
            .read(cx)
            .last_content
            .mode
            .contains(TermMode::ALT_SCREEN)
        {
            self.terminal.update(cx, |term, cx| {
                term.try_keystroke(
                    &Keystroke::parse("ctrl-cmd-space").unwrap(),
                    TerminalSettings::get_global(cx).option_as_meta,
                )
            });
        } else {
            cx.show_character_palette();
        }
    }

    fn select_all(&mut self, _: &SelectAll, cx: &mut ViewContext<Self>) {
        self.terminal.update(cx, |term, _| term.select_all());
        cx.notify();
    }

    fn clear(&mut self, _: &Clear, cx: &mut ViewContext<Self>) {
        self.scroll_top = px(0.);
        self.terminal.update(cx, |term, _| term.clear());
        cx.notify();
    }

    fn max_scroll_top(&self, cx: &AppContext) -> Pixels {
        let terminal = self.terminal.read(cx);

        let Some(block) = self.block_below_cursor.as_ref() else {
            return Pixels::ZERO;
        };

        let line_height = terminal.last_content().size.line_height;
        let mut terminal_lines = terminal.total_lines();
        let viewport_lines = terminal.viewport_lines();
        if terminal.total_lines() == terminal.viewport_lines() {
            let mut last_line = None;
            for cell in terminal.last_content.cells.iter().rev() {
                if !is_blank(cell) {
                    break;
                }

                let last_line = last_line.get_or_insert(cell.point.line);
                if *last_line != cell.point.line {
                    terminal_lines -= 1;
                }
                *last_line = cell.point.line;
            }
        }

        let max_scroll_top_in_lines =
            (block.height as usize).saturating_sub(viewport_lines.saturating_sub(terminal_lines));

        max_scroll_top_in_lines as f32 * line_height
    }

    fn scroll_wheel(
        &mut self,
        event: &ScrollWheelEvent,
        origin: gpui::Point<Pixels>,
        cx: &mut ViewContext<Self>,
    ) {
        let terminal_content = self.terminal.read(cx).last_content();

        if self.block_below_cursor.is_some() && terminal_content.display_offset == 0 {
            let line_height = terminal_content.size.line_height;
            let y_delta = event.delta.pixel_delta(line_height).y;
            if y_delta < Pixels::ZERO || self.scroll_top > Pixels::ZERO {
                self.scroll_top = cmp::max(
                    Pixels::ZERO,
                    cmp::min(self.scroll_top - y_delta, self.max_scroll_top(cx)),
                );
                cx.notify();
                return;
            }
        }

        self.terminal
            .update(cx, |term, _| term.scroll_wheel(event, origin));
    }

    fn scroll_line_up(&mut self, _: &ScrollLineUp, cx: &mut ViewContext<Self>) {
        let terminal_content = self.terminal.read(cx).last_content();
        if self.block_below_cursor.is_some()
            && terminal_content.display_offset == 0
            && self.scroll_top > Pixels::ZERO
        {
            let line_height = terminal_content.size.line_height;
            self.scroll_top = cmp::max(self.scroll_top - line_height, Pixels::ZERO);
            return;
        }

        self.terminal.update(cx, |term, _| term.scroll_line_up());
        cx.notify();
    }

    fn scroll_line_down(&mut self, _: &ScrollLineDown, cx: &mut ViewContext<Self>) {
        let terminal_content = self.terminal.read(cx).last_content();
        if self.block_below_cursor.is_some() && terminal_content.display_offset == 0 {
            let max_scroll_top = self.max_scroll_top(cx);
            if self.scroll_top < max_scroll_top {
                let line_height = terminal_content.size.line_height;
                self.scroll_top = cmp::min(self.scroll_top + line_height, max_scroll_top);
            }
            return;
        }

        self.terminal.update(cx, |term, _| term.scroll_line_down());
        cx.notify();
    }

    fn scroll_page_up(&mut self, _: &ScrollPageUp, cx: &mut ViewContext<Self>) {
        if self.scroll_top == Pixels::ZERO {
            self.terminal.update(cx, |term, _| term.scroll_page_up());
        } else {
            let line_height = self.terminal.read(cx).last_content.size.line_height();
            let visible_block_lines = (self.scroll_top / line_height) as usize;
            let viewport_lines = self.terminal.read(cx).viewport_lines();
            let visible_content_lines = viewport_lines - visible_block_lines;

            if visible_block_lines >= viewport_lines {
                self.scroll_top = ((visible_block_lines - viewport_lines) as f32) * line_height;
            } else {
                self.scroll_top = px(0.);
                self.terminal
                    .update(cx, |term, _| term.scroll_up_by(visible_content_lines));
            }
        }
        cx.notify();
    }

    fn scroll_page_down(&mut self, _: &ScrollPageDown, cx: &mut ViewContext<Self>) {
        self.terminal.update(cx, |term, _| term.scroll_page_down());
        let terminal = self.terminal.read(cx);
        if terminal.last_content().display_offset < terminal.viewport_lines() {
            self.scroll_top = self.max_scroll_top(cx);
        }
        cx.notify();
    }

    fn scroll_to_top(&mut self, _: &ScrollToTop, cx: &mut ViewContext<Self>) {
        self.terminal.update(cx, |term, _| term.scroll_to_top());
        cx.notify();
    }

    fn scroll_to_bottom(&mut self, _: &ScrollToBottom, cx: &mut ViewContext<Self>) {
        self.terminal.update(cx, |term, _| term.scroll_to_bottom());
        if self.block_below_cursor.is_some() {
            self.scroll_top = self.max_scroll_top(cx);
        }
        cx.notify();
    }

    fn toggle_vi_mode(&mut self, _: &ToggleViMode, cx: &mut ViewContext<Self>) {
        self.terminal.update(cx, |term, _| term.toggle_vi_mode());
        cx.notify();
    }

    pub fn should_show_cursor(&self, focused: bool, cx: &mut ViewContext<Self>) -> bool {
        //Don't blink the cursor when not focused, blinking is disabled, or paused
        if !focused
            || self.blinking_paused
            || self
                .terminal
                .read(cx)
                .last_content
                .mode
                .contains(TermMode::ALT_SCREEN)
        {
            return true;
        }

        match TerminalSettings::get_global(cx).blinking {
            //If the user requested to never blink, don't blink it.
            TerminalBlink::Off => true,
            //If the terminal is controlling it, check terminal mode
            TerminalBlink::TerminalControlled => {
                !self.blinking_terminal_enabled || self.blink_state
            }
            TerminalBlink::On => self.blink_state,
        }
    }

    fn blink_cursors(&mut self, epoch: usize, cx: &mut ViewContext<Self>) {
        if epoch == self.blink_epoch && !self.blinking_paused {
            self.blink_state = !self.blink_state;
            cx.notify();

            let epoch = self.next_blink_epoch();
            cx.spawn(|this, mut cx| async move {
                Timer::after(CURSOR_BLINK_INTERVAL).await;
                this.update(&mut cx, |this, cx| this.blink_cursors(epoch, cx))
                    .ok();
            })
            .detach();
        }
    }

    pub fn pause_cursor_blinking(&mut self, cx: &mut ViewContext<Self>) {
        self.blink_state = true;
        cx.notify();

        let epoch = self.next_blink_epoch();
        cx.spawn(|this, mut cx| async move {
            Timer::after(CURSOR_BLINK_INTERVAL).await;
            this.update(&mut cx, |this, cx| this.resume_cursor_blinking(epoch, cx))
                .ok();
        })
        .detach();
    }

    pub fn terminal(&self) -> &Model<Terminal> {
        &self.terminal
    }

    pub fn set_block_below_cursor(&mut self, block: BlockProperties, cx: &mut ViewContext<Self>) {
        self.block_below_cursor = Some(Rc::new(block));
        self.scroll_to_bottom(&ScrollToBottom, cx);
        cx.notify();
    }

    pub fn clear_block_below_cursor(&mut self, cx: &mut ViewContext<Self>) {
        self.block_below_cursor = None;
        self.scroll_top = Pixels::ZERO;
        cx.notify();
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
        self.terminal.update(cx, |term, _| term.copy());
        cx.notify();
    }

    ///Attempt to paste the clipboard into the terminal
    fn paste(&mut self, _: &Paste, cx: &mut ViewContext<Self>) {
        if let Some(clipboard_string) = cx.read_from_clipboard().and_then(|item| item.text()) {
            self.terminal
                .update(cx, |terminal, _cx| terminal.paste(&clipboard_string));
        }
    }

    fn send_text(&mut self, text: &SendText, cx: &mut ViewContext<Self>) {
        self.clear_bell(cx);
        self.terminal.update(cx, |term, _| {
            term.input(text.0.to_string());
        });
    }

    fn send_keystroke(&mut self, text: &SendKeystroke, cx: &mut ViewContext<Self>) {
        if let Some(keystroke) = Keystroke::parse(&text.0).log_err() {
            self.clear_bell(cx);
            self.terminal.update(cx, |term, cx| {
                term.try_keystroke(&keystroke, TerminalSettings::get_global(cx).option_as_meta);
            });
        }
    }

    fn dispatch_context(&self, cx: &AppContext) -> KeyContext {
        let mut dispatch_context = KeyContext::new_with_defaults();
        dispatch_context.add("Terminal");

        let mode = self.terminal.read(cx).last_content.mode;
        dispatch_context.set(
            "screen",
            if mode.contains(TermMode::ALT_SCREEN) {
                "alt"
            } else {
                "normal"
            },
        );

        if mode.contains(TermMode::APP_CURSOR) {
            dispatch_context.add("DECCKM");
        }
        if mode.contains(TermMode::APP_KEYPAD) {
            dispatch_context.add("DECPAM");
        } else {
            dispatch_context.add("DECPNM");
        }
        if mode.contains(TermMode::SHOW_CURSOR) {
            dispatch_context.add("DECTCEM");
        }
        if mode.contains(TermMode::LINE_WRAP) {
            dispatch_context.add("DECAWM");
        }
        if mode.contains(TermMode::ORIGIN) {
            dispatch_context.add("DECOM");
        }
        if mode.contains(TermMode::INSERT) {
            dispatch_context.add("IRM");
        }
        //LNM is apparently the name for this. https://vt100.net/docs/vt510-rm/LNM.html
        if mode.contains(TermMode::LINE_FEED_NEW_LINE) {
            dispatch_context.add("LNM");
        }
        if mode.contains(TermMode::FOCUS_IN_OUT) {
            dispatch_context.add("report_focus");
        }
        if mode.contains(TermMode::ALTERNATE_SCROLL) {
            dispatch_context.add("alternate_scroll");
        }
        if mode.contains(TermMode::BRACKETED_PASTE) {
            dispatch_context.add("bracketed_paste");
        }
        if mode.intersects(TermMode::MOUSE_MODE) {
            dispatch_context.add("any_mouse_reporting");
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
            dispatch_context.set("mouse_reporting", mouse_reporting);
        }
        {
            let format = if mode.contains(TermMode::SGR_MOUSE) {
                "sgr"
            } else if mode.contains(TermMode::UTF8_MOUSE) {
                "utf8"
            } else {
                "normal"
            };
            dispatch_context.set("mouse_format", format);
        };
        dispatch_context
    }

    fn set_terminal(&mut self, terminal: Model<Terminal>, cx: &mut ViewContext<TerminalView>) {
        self._terminal_subscriptions =
            subscribe_for_terminal_events(&terminal, self.workspace.clone(), cx);
        self.terminal = terminal;
    }

    // Hack: Using editor in terminal causes cyclic dependency i.e. editor -> terminal -> project -> editor.
    fn map_show_scrollbar_from_editor_to_terminal(
        show_scrollbar: editor::ShowScrollbar,
    ) -> terminal_settings::ShowScrollbar {
        match show_scrollbar {
            editor::ShowScrollbar::Auto => terminal_settings::ShowScrollbar::Auto,
            editor::ShowScrollbar::System => terminal_settings::ShowScrollbar::System,
            editor::ShowScrollbar::Always => terminal_settings::ShowScrollbar::Always,
            editor::ShowScrollbar::Never => terminal_settings::ShowScrollbar::Never,
        }
    }

    fn should_show_scrollbar(cx: &AppContext) -> bool {
        let show = TerminalSettings::get_global(cx)
            .scrollbar
            .show
            .unwrap_or_else(|| {
                Self::map_show_scrollbar_from_editor_to_terminal(
                    EditorSettings::get_global(cx).scrollbar.show,
                )
            });
        match show {
            terminal_settings::ShowScrollbar::Auto => true,
            terminal_settings::ShowScrollbar::System => true,
            terminal_settings::ShowScrollbar::Always => true,
            terminal_settings::ShowScrollbar::Never => false,
        }
    }

    fn should_autohide_scrollbar(cx: &AppContext) -> bool {
        let show = TerminalSettings::get_global(cx)
            .scrollbar
            .show
            .unwrap_or_else(|| {
                Self::map_show_scrollbar_from_editor_to_terminal(
                    EditorSettings::get_global(cx).scrollbar.show,
                )
            });
        match show {
            terminal_settings::ShowScrollbar::Auto => true,
            terminal_settings::ShowScrollbar::System => cx
                .try_global::<ScrollbarAutoHide>()
                .map_or_else(|| cx.should_auto_hide_scrollbars(), |autohide| autohide.0),
            terminal_settings::ShowScrollbar::Always => false,
            terminal_settings::ShowScrollbar::Never => true,
        }
    }

    fn hide_scrollbar(&mut self, cx: &mut ViewContext<Self>) {
        const SCROLLBAR_SHOW_INTERVAL: Duration = Duration::from_secs(1);
        if !Self::should_autohide_scrollbar(cx) {
            return;
        }
        self.hide_scrollbar_task = Some(cx.spawn(|panel, mut cx| async move {
            cx.background_executor()
                .timer(SCROLLBAR_SHOW_INTERVAL)
                .await;
            panel
                .update(&mut cx, |panel, cx| {
                    panel.show_scrollbar = false;
                    cx.notify();
                })
                .log_err();
        }))
    }

    fn render_scrollbar(&self, cx: &mut ViewContext<Self>) -> Option<Stateful<Div>> {
        if !Self::should_show_scrollbar(cx)
            || !(self.show_scrollbar || self.scrollbar_state.is_dragging())
        {
            return None;
        }

        if self.terminal.read(cx).total_lines() == self.terminal.read(cx).viewport_lines() {
            return None;
        }

        self.scroll_handle.update(self.terminal.read(cx));

        if let Some(new_display_offset) = self.scroll_handle.future_display_offset.take() {
            self.terminal.update(cx, |term, _| {
                let delta = new_display_offset as i32 - term.last_content.display_offset as i32;
                match delta.cmp(&0) {
                    std::cmp::Ordering::Greater => term.scroll_up_by(delta as usize),
                    std::cmp::Ordering::Less => term.scroll_down_by(-delta as usize),
                    std::cmp::Ordering::Equal => {}
                }
            });
        }

        Some(
            div()
                .occlude()
                .id("terminal-view-scroll")
                .on_mouse_move(cx.listener(|_, _, cx| {
                    cx.notify();
                    cx.stop_propagation()
                }))
                .on_hover(|_, cx| {
                    cx.stop_propagation();
                })
                .on_any_mouse_down(|_, cx| {
                    cx.stop_propagation();
                })
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(|terminal_view, _, cx| {
                        if !terminal_view.scrollbar_state.is_dragging()
                            && !terminal_view.focus_handle.contains_focused(cx)
                        {
                            terminal_view.hide_scrollbar(cx);
                            cx.notify();
                        }
                        cx.stop_propagation();
                    }),
                )
                .on_scroll_wheel(cx.listener(|_, _, cx| {
                    cx.notify();
                }))
                .h_full()
                .absolute()
                .right_1()
                .top_1()
                .bottom_0()
                .w(px(12.))
                .cursor_default()
                .children(Scrollbar::vertical(self.scrollbar_state.clone())),
        )
    }
}

fn subscribe_for_terminal_events(
    terminal: &Model<Terminal>,
    workspace: WeakView<Workspace>,
    cx: &mut ViewContext<TerminalView>,
) -> Vec<Subscription> {
    let terminal_subscription = cx.observe(terminal, |_, _, cx| cx.notify());
    let terminal_events_subscription =
        cx.subscribe(terminal, move |this, _, event, cx| match event {
            Event::Wakeup => {
                cx.notify();
                cx.emit(Event::Wakeup);
                cx.emit(ItemEvent::UpdateTab);
                cx.emit(SearchEvent::MatchesInvalidated);
            }

            Event::Bell => {
                this.has_bell = true;
                cx.emit(Event::Wakeup);
            }

            Event::BlinkChanged(blinking) => {
                if matches!(
                    TerminalSettings::get_global(cx).blinking,
                    TerminalBlink::TerminalControlled
                ) {
                    this.blinking_terminal_enabled = *blinking;
                }
            }

            Event::TitleChanged => {
                cx.emit(ItemEvent::UpdateTab);
            }

            Event::NewNavigationTarget(maybe_navigation_target) => {
                this.can_navigate_to_selected_word = match maybe_navigation_target {
                    Some(MaybeNavigationTarget::Url(_)) => true,
                    Some(MaybeNavigationTarget::PathLike(path_like_target)) => {
                        if let Ok(fs) = workspace.update(cx, |workspace, cx| {
                            workspace.project().read(cx).fs().clone()
                        }) {
                            let valid_files_to_open_task = possible_open_targets(
                                fs,
                                &workspace,
                                &path_like_target.terminal_dir,
                                &path_like_target.maybe_path,
                                cx,
                            );
                            !smol::block_on(valid_files_to_open_task).is_empty()
                        } else {
                            false
                        }
                    }
                    None => false,
                }
            }

            Event::Open(maybe_navigation_target) => match maybe_navigation_target {
                MaybeNavigationTarget::Url(url) => cx.open_url(url),

                MaybeNavigationTarget::PathLike(path_like_target) => {
                    if !this.can_navigate_to_selected_word {
                        return;
                    }
                    let task_workspace = workspace.clone();
                    let Some(fs) = workspace
                        .update(cx, |workspace, cx| {
                            workspace.project().read(cx).fs().clone()
                        })
                        .ok()
                    else {
                        return;
                    };

                    let path_like_target = path_like_target.clone();
                    cx.spawn(|terminal_view, mut cx| async move {
                        let valid_files_to_open = terminal_view
                            .update(&mut cx, |_, cx| {
                                possible_open_targets(
                                    fs,
                                    &task_workspace,
                                    &path_like_target.terminal_dir,
                                    &path_like_target.maybe_path,
                                    cx,
                                )
                            })?
                            .await;
                        let paths_to_open = valid_files_to_open
                            .iter()
                            .map(|(p, _)| p.path.clone())
                            .collect();
                        let opened_items = task_workspace
                            .update(&mut cx, |workspace, cx| {
                                workspace.open_paths(
                                    paths_to_open,
                                    OpenVisible::OnlyDirectories,
                                    None,
                                    cx,
                                )
                            })
                            .context("workspace update")?
                            .await;

                        let mut has_dirs = false;
                        for ((path, metadata), opened_item) in valid_files_to_open
                            .into_iter()
                            .zip(opened_items.into_iter())
                        {
                            if metadata.is_dir {
                                has_dirs = true;
                            } else if let Some(Ok(opened_item)) = opened_item {
                                if let Some(row) = path.row {
                                    let col = path.column.unwrap_or(0);
                                    if let Some(active_editor) = opened_item.downcast::<Editor>() {
                                        active_editor
                                            .downgrade()
                                            .update(&mut cx, |editor, cx| {
                                                let snapshot = editor.snapshot(cx).display_snapshot;
                                                let point = snapshot.buffer_snapshot.clip_point(
                                                    language::Point::new(
                                                        row.saturating_sub(1),
                                                        col.saturating_sub(1),
                                                    ),
                                                    Bias::Left,
                                                );
                                                editor.change_selections(
                                                    Some(Autoscroll::center()),
                                                    cx,
                                                    |s| s.select_ranges([point..point]),
                                                );
                                            })
                                            .log_err();
                                    }
                                }
                            }
                        }

                        if has_dirs {
                            task_workspace.update(&mut cx, |workspace, cx| {
                                workspace.project().update(cx, |_, cx| {
                                    cx.emit(project::Event::ActivateProjectPanel);
                                })
                            })?;
                        }

                        anyhow::Ok(())
                    })
                    .detach_and_log_err(cx)
                }
            },
            Event::BreadcrumbsChanged => cx.emit(ItemEvent::UpdateBreadcrumbs),
            Event::CloseTerminal => cx.emit(ItemEvent::CloseItem),
            Event::SelectionsChanged => {
                cx.invalidate_character_coordinates();
                cx.emit(SearchEvent::ActiveMatchChanged)
            }
        });
    vec![terminal_subscription, terminal_events_subscription]
}

fn possible_open_paths_metadata(
    fs: Arc<dyn Fs>,
    row: Option<u32>,
    column: Option<u32>,
    potential_paths: HashSet<PathBuf>,
    cx: &mut ViewContext<TerminalView>,
) -> Task<Vec<(PathWithPosition, Metadata)>> {
    cx.background_executor().spawn(async move {
        let mut canonical_paths = HashSet::default();
        for path in potential_paths {
            if let Ok(canonical) = fs.canonicalize(&path).await {
                let sanitized = SanitizedPath::from(canonical);
                canonical_paths.insert(sanitized.as_path().to_path_buf());
            } else {
                canonical_paths.insert(path);
            }
        }

        let mut paths_with_metadata = Vec::with_capacity(canonical_paths.len());

        let mut fetch_metadata_tasks = canonical_paths
            .into_iter()
            .map(|potential_path| async {
                let metadata = fs.metadata(&potential_path).await.ok().flatten();
                (
                    PathWithPosition {
                        path: potential_path,
                        row,
                        column,
                    },
                    metadata,
                )
            })
            .collect::<FuturesUnordered<_>>();

        while let Some((path, metadata)) = fetch_metadata_tasks.next().await {
            if let Some(metadata) = metadata {
                paths_with_metadata.push((path, metadata));
            }
        }

        paths_with_metadata
    })
}

fn possible_open_targets(
    fs: Arc<dyn Fs>,
    workspace: &WeakView<Workspace>,
    cwd: &Option<PathBuf>,
    maybe_path: &String,
    cx: &mut ViewContext<TerminalView>,
) -> Task<Vec<(PathWithPosition, Metadata)>> {
    let path_position = PathWithPosition::parse_str(maybe_path.as_str());
    let row = path_position.row;
    let column = path_position.column;
    let maybe_path = path_position.path;

    let potential_paths = if maybe_path.is_absolute() {
        HashSet::from_iter([maybe_path])
    } else if maybe_path.starts_with("~") {
        maybe_path
            .strip_prefix("~")
            .ok()
            .and_then(|maybe_path| Some(dirs::home_dir()?.join(maybe_path)))
            .map_or_else(HashSet::default, |p| HashSet::from_iter([p]))
    } else {
        let mut potential_cwd_and_workspace_paths = HashSet::default();
        if let Some(cwd) = cwd {
            let abs_path = Path::join(cwd, &maybe_path);
            potential_cwd_and_workspace_paths.insert(abs_path);
        }
        if let Some(workspace) = workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                for potential_worktree_path in workspace
                    .worktrees(cx)
                    .map(|worktree| worktree.read(cx).abs_path().join(&maybe_path))
                {
                    potential_cwd_and_workspace_paths.insert(potential_worktree_path);
                }

                for prefix in GIT_DIFF_PATH_PREFIXES {
                    let prefix_str = &prefix.to_string();
                    if maybe_path.starts_with(prefix_str) {
                        let stripped = maybe_path.strip_prefix(prefix_str).unwrap_or(&maybe_path);
                        for potential_worktree_path in workspace
                            .worktrees(cx)
                            .map(|worktree| worktree.read(cx).abs_path().join(&stripped))
                        {
                            potential_cwd_and_workspace_paths.insert(potential_worktree_path);
                        }
                    }
                }
            });
        }
        potential_cwd_and_workspace_paths
    };

    possible_open_paths_metadata(fs, row, column, potential_paths, cx)
}

fn regex_to_literal(regex: &str) -> String {
    regex
        .chars()
        .flat_map(|c| {
            if REGEX_SPECIAL_CHARS.contains(&c) {
                vec!['\\', c]
            } else {
                vec![c]
            }
        })
        .collect()
}

pub fn regex_search_for_query(query: &project::search::SearchQuery) -> Option<RegexSearch> {
    let query = query.as_str();
    if query == "." {
        return None;
    }
    let searcher = RegexSearch::new(query);
    searcher.ok()
}

impl TerminalView {
    fn key_down(&mut self, event: &KeyDownEvent, cx: &mut ViewContext<Self>) {
        self.clear_bell(cx);
        self.pause_cursor_blinking(cx);

        self.terminal.update(cx, |term, cx| {
            let handled = term.try_keystroke(
                &event.keystroke,
                TerminalSettings::get_global(cx).option_as_meta,
            );
            if handled {
                cx.stop_propagation();
            }
        });
    }

    fn focus_in(&mut self, cx: &mut ViewContext<Self>) {
        self.terminal.update(cx, |terminal, _| {
            terminal.set_cursor_shape(self.cursor_shape);
            terminal.focus_in();
        });
        self.blink_cursors(self.blink_epoch, cx);
        cx.invalidate_character_coordinates();
        cx.notify();
    }

    fn focus_out(&mut self, cx: &mut ViewContext<Self>) {
        self.terminal.update(cx, |terminal, _| {
            terminal.focus_out();
            terminal.set_cursor_shape(CursorShape::Hollow);
        });
        self.hide_scrollbar(cx);
        cx.notify();
    }
}

impl Render for TerminalView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let terminal_handle = self.terminal.clone();
        let terminal_view_handle = cx.view().clone();

        let focused = self.focus_handle.is_focused(cx);

        div()
            .occlude()
            .id("terminal-view")
            .size_full()
            .relative()
            .track_focus(&self.focus_handle(cx))
            .key_context(self.dispatch_context(cx))
            .on_action(cx.listener(TerminalView::send_text))
            .on_action(cx.listener(TerminalView::send_keystroke))
            .on_action(cx.listener(TerminalView::copy))
            .on_action(cx.listener(TerminalView::paste))
            .on_action(cx.listener(TerminalView::clear))
            .on_action(cx.listener(TerminalView::scroll_line_up))
            .on_action(cx.listener(TerminalView::scroll_line_down))
            .on_action(cx.listener(TerminalView::scroll_page_up))
            .on_action(cx.listener(TerminalView::scroll_page_down))
            .on_action(cx.listener(TerminalView::scroll_to_top))
            .on_action(cx.listener(TerminalView::scroll_to_bottom))
            .on_action(cx.listener(TerminalView::toggle_vi_mode))
            .on_action(cx.listener(TerminalView::show_character_palette))
            .on_action(cx.listener(TerminalView::select_all))
            .on_key_down(cx.listener(Self::key_down))
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(|this, event: &MouseDownEvent, cx| {
                    if !this.terminal.read(cx).mouse_mode(event.modifiers.shift) {
                        this.deploy_context_menu(event.position, cx);
                        cx.notify();
                    }
                }),
            )
            .on_hover(cx.listener(|this, hovered, cx| {
                if *hovered {
                    this.show_scrollbar = true;
                    this.hide_scrollbar_task.take();
                    cx.notify();
                } else if !this.focus_handle.contains_focused(cx) {
                    this.hide_scrollbar(cx);
                }
            }))
            .child(
                // TODO: Oddly this wrapper div is needed for TerminalElement to not steal events from the context menu
                div()
                    .size_full()
                    .child(TerminalElement::new(
                        terminal_handle,
                        terminal_view_handle,
                        self.workspace.clone(),
                        self.focus_handle.clone(),
                        focused,
                        self.should_show_cursor(focused, cx),
                        self.can_navigate_to_selected_word,
                        self.block_below_cursor.clone(),
                    ))
                    .when_some(self.render_scrollbar(cx), |div, scrollbar| {
                        div.child(scrollbar)
                    }),
            )
            .children(self.context_menu.as_ref().map(|(menu, position, _)| {
                deferred(
                    anchored()
                        .position(*position)
                        .anchor(gpui::Corner::TopLeft)
                        .child(menu.clone()),
                )
                .with_priority(1)
            }))
    }
}

impl Item for TerminalView {
    type Event = ItemEvent;

    fn tab_tooltip_content(&self, cx: &AppContext) -> Option<TabTooltipContent> {
        let terminal = self.terminal().read(cx);
        let title = terminal.title(false);
        let pid = terminal.pty_info.pid_getter().fallback_pid();

        Some(TabTooltipContent::Custom(Box::new(
            move |cx: &mut WindowContext| {
                cx.new_view(|_| TerminalTooltip::new(title.clone(), pid))
                    .into()
            },
        )))
    }

    fn tab_content(&self, params: TabContentParams, cx: &WindowContext) -> AnyElement {
        let terminal = self.terminal().read(cx);
        let title = terminal.title(true);
        let rerun_button = |task_id: task::TaskId| {
            IconButton::new("rerun-icon", IconName::Rerun)
                .icon_size(IconSize::Small)
                .size(ButtonSize::Compact)
                .icon_color(Color::Default)
                .shape(ui::IconButtonShape::Square)
                .tooltip(|cx| Tooltip::text("Rerun task", cx))
                .on_click(move |_, cx| {
                    cx.dispatch_action(Box::new(zed_actions::Rerun {
                        task_id: Some(task_id.0.clone()),
                        allow_concurrent_runs: Some(true),
                        use_new_terminal: Some(false),
                        reevaluate_context: false,
                    }));
                })
        };

        let (icon, icon_color, rerun_button) = match terminal.task() {
            Some(terminal_task) => match &terminal_task.status {
                TaskStatus::Running => (
                    IconName::Play,
                    Color::Disabled,
                    Some(rerun_button(terminal_task.id.clone())),
                ),
                TaskStatus::Unknown => (
                    IconName::Warning,
                    Color::Warning,
                    Some(rerun_button(terminal_task.id.clone())),
                ),
                TaskStatus::Completed { success } => {
                    let rerun_button = rerun_button(terminal_task.id.clone());
                    if *success {
                        (IconName::Check, Color::Success, Some(rerun_button))
                    } else {
                        (IconName::XCircle, Color::Error, Some(rerun_button))
                    }
                }
            },
            None => (IconName::Terminal, Color::Muted, None),
        };

        h_flex()
            .gap_1()
            .group("term-tab-icon")
            .child(
                h_flex()
                    .group("term-tab-icon")
                    .child(
                        div()
                            .when(rerun_button.is_some(), |this| {
                                this.hover(|style| style.invisible().w_0())
                            })
                            .child(Icon::new(icon).color(icon_color)),
                    )
                    .when_some(rerun_button, |this, rerun_button| {
                        this.child(
                            div()
                                .absolute()
                                .visible_on_hover("term-tab-icon")
                                .child(rerun_button),
                        )
                    }),
            )
            .child(Label::new(title).color(params.text_color()))
            .into_any()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        None
    }

    fn clone_on_split(
        &self,
        workspace_id: Option<WorkspaceId>,
        cx: &mut ViewContext<Self>,
    ) -> Option<View<Self>> {
        let window = cx.window_handle();
        let terminal = self
            .project
            .update(cx, |project, cx| {
                let terminal = self.terminal().read(cx);
                let working_directory = terminal
                    .working_directory()
                    .or_else(|| Some(project.active_project_directory(cx)?.to_path_buf()));
                let python_venv_directory = terminal.python_venv_directory.clone();
                project.create_terminal_with_venv(
                    TerminalKind::Shell(working_directory),
                    python_venv_directory,
                    window,
                    cx,
                )
            })
            .ok()?
            .log_err()?;

        Some(cx.new_view(|cx| {
            TerminalView::new(
                terminal,
                self.workspace.clone(),
                workspace_id,
                self.project.clone(),
                cx,
            )
        }))
    }

    fn is_dirty(&self, cx: &gpui::AppContext) -> bool {
        match self.terminal.read(cx).task() {
            Some(task) => task.status == TaskStatus::Running,
            None => self.has_bell(),
        }
    }

    fn has_conflict(&self, _cx: &AppContext) -> bool {
        false
    }

    fn is_singleton(&self, _cx: &AppContext) -> bool {
        true
    }

    fn as_searchable(&self, handle: &View<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(handle.clone()))
    }

    fn breadcrumb_location(&self, cx: &AppContext) -> ToolbarItemLocation {
        if self.show_breadcrumbs && !self.terminal().read(cx).breadcrumb_text.trim().is_empty() {
            ToolbarItemLocation::PrimaryLeft
        } else {
            ToolbarItemLocation::Hidden
        }
    }

    fn breadcrumbs(&self, _: &theme::Theme, cx: &AppContext) -> Option<Vec<BreadcrumbText>> {
        Some(vec![BreadcrumbText {
            text: self.terminal().read(cx).breadcrumb_text.clone(),
            highlights: None,
            font: None,
        }])
    }

    fn added_to_workspace(&mut self, workspace: &mut Workspace, cx: &mut ViewContext<Self>) {
        if self.terminal().read(cx).task().is_none() {
            if let Some((new_id, old_id)) = workspace.database_id().zip(self.workspace_id) {
                cx.background_executor()
                    .spawn(TERMINAL_DB.update_workspace_id(new_id, old_id, cx.entity_id().as_u64()))
                    .detach();
            }
            self.workspace_id = workspace.database_id();
        }
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(ItemEvent)) {
        f(*event)
    }
}

impl SerializableItem for TerminalView {
    fn serialized_item_kind() -> &'static str {
        "Terminal"
    }

    fn cleanup(
        workspace_id: WorkspaceId,
        alive_items: Vec<workspace::ItemId>,
        cx: &mut WindowContext,
    ) -> Task<gpui::Result<()>> {
        cx.spawn(|_| TERMINAL_DB.delete_unloaded_items(workspace_id, alive_items))
    }

    fn serialize(
        &mut self,
        _workspace: &mut Workspace,
        item_id: workspace::ItemId,
        _closing: bool,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<gpui::Result<()>>> {
        let terminal = self.terminal().read(cx);
        if terminal.task().is_some() {
            return None;
        }

        if let Some((cwd, workspace_id)) = terminal.working_directory().zip(self.workspace_id) {
            Some(cx.background_executor().spawn(async move {
                TERMINAL_DB
                    .save_working_directory(item_id, workspace_id, cwd)
                    .await
            }))
        } else {
            None
        }
    }

    fn should_serialize(&self, event: &Self::Event) -> bool {
        matches!(event, ItemEvent::UpdateTab)
    }

    fn deserialize(
        project: Model<Project>,
        workspace: WeakView<Workspace>,
        workspace_id: workspace::WorkspaceId,
        item_id: workspace::ItemId,
        cx: &mut WindowContext,
    ) -> Task<anyhow::Result<View<Self>>> {
        let window = cx.window_handle();
        cx.spawn(|mut cx| async move {
            let cwd = cx
                .update(|cx| {
                    let from_db = TERMINAL_DB
                        .get_working_directory(item_id, workspace_id)
                        .log_err()
                        .flatten();
                    if from_db
                        .as_ref()
                        .is_some_and(|from_db| !from_db.as_os_str().is_empty())
                    {
                        from_db
                    } else {
                        workspace
                            .upgrade()
                            .and_then(|workspace| default_working_directory(workspace.read(cx), cx))
                    }
                })
                .ok()
                .flatten();

            let terminal = project
                .update(&mut cx, |project, cx| {
                    project.create_terminal(TerminalKind::Shell(cwd), window, cx)
                })?
                .await?;
            cx.update(|cx| {
                cx.new_view(|cx| {
                    TerminalView::new(
                        terminal,
                        workspace,
                        Some(workspace_id),
                        project.downgrade(),
                        cx,
                    )
                })
            })
        })
    }
}

impl SearchableItem for TerminalView {
    type Match = RangeInclusive<Point>;

    fn supported_options() -> SearchOptions {
        SearchOptions {
            case: false,
            word: false,
            regex: true,
            replacement: false,
            selection: false,
        }
    }

    /// Clear stored matches
    fn clear_matches(&mut self, cx: &mut ViewContext<Self>) {
        self.terminal().update(cx, |term, _| term.matches.clear())
    }

    /// Store matches returned from find_matches somewhere for rendering
    fn update_matches(&mut self, matches: &[Self::Match], cx: &mut ViewContext<Self>) {
        self.terminal()
            .update(cx, |term, _| term.matches = matches.to_vec())
    }

    /// Returns the selection content to pre-load into this search
    fn query_suggestion(&mut self, cx: &mut ViewContext<Self>) -> String {
        self.terminal()
            .read(cx)
            .last_content
            .selection_text
            .clone()
            .unwrap_or_default()
    }

    /// Focus match at given index into the Vec of matches
    fn activate_match(&mut self, index: usize, _: &[Self::Match], cx: &mut ViewContext<Self>) {
        self.terminal()
            .update(cx, |term, _| term.activate_match(index));
        cx.notify();
    }

    /// Add selections for all matches given.
    fn select_matches(&mut self, matches: &[Self::Match], cx: &mut ViewContext<Self>) {
        self.terminal()
            .update(cx, |term, _| term.select_matches(matches));
        cx.notify();
    }

    /// Get all of the matches for this query, should be done on the background
    fn find_matches(
        &mut self,
        query: Arc<SearchQuery>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Vec<Self::Match>> {
        let searcher = match &*query {
            SearchQuery::Text { .. } => regex_search_for_query(
                &(SearchQuery::text(
                    regex_to_literal(query.as_str()),
                    query.whole_word(),
                    query.case_sensitive(),
                    query.include_ignored(),
                    query.files_to_include().clone(),
                    query.files_to_exclude().clone(),
                    None,
                )
                .unwrap()),
            ),
            SearchQuery::Regex { .. } => regex_search_for_query(&query),
        };

        if let Some(s) = searcher {
            self.terminal()
                .update(cx, |term, cx| term.find_matches(s, cx))
        } else {
            Task::ready(vec![])
        }
    }

    /// Reports back to the search toolbar what the active match should be (the selection)
    fn active_match_index(
        &mut self,
        matches: &[Self::Match],
        cx: &mut ViewContext<Self>,
    ) -> Option<usize> {
        // Selection head might have a value if there's a selection that isn't
        // associated with a match. Therefore, if there are no matches, we should
        // report None, no matter the state of the terminal
        let res = if !matches.is_empty() {
            if let Some(selection_head) = self.terminal().read(cx).selection_head {
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
    fn replace(&mut self, _: &Self::Match, _: &SearchQuery, _: &mut ViewContext<Self>) {
        // Replacement is not supported in terminal view, so this is a no-op.
    }
}

///Gets the working directory for the given workspace, respecting the user's settings.
/// None implies "~" on whichever machine we end up on.
pub(crate) fn default_working_directory(workspace: &Workspace, cx: &AppContext) -> Option<PathBuf> {
    match &TerminalSettings::get_global(cx).working_directory {
        WorkingDirectory::CurrentProjectDirectory => workspace
            .project()
            .read(cx)
            .active_project_directory(cx)
            .as_deref()
            .map(Path::to_path_buf),
        WorkingDirectory::FirstProjectDirectory => first_project_directory(workspace, cx),
        WorkingDirectory::AlwaysHome => None,
        WorkingDirectory::Always { directory } => {
            shellexpand::full(&directory) //TODO handle this better
                .ok()
                .map(|dir| Path::new(&dir.to_string()).to_path_buf())
                .filter(|dir| dir.is_dir())
        }
    }
}
///Gets the first project's home directory, or the home directory
fn first_project_directory(workspace: &Workspace, cx: &AppContext) -> Option<PathBuf> {
    let worktree = workspace.worktrees(cx).next()?.read(cx);
    if !worktree.root_entry()?.is_dir() {
        return None;
    }
    Some(worktree.abs_path().to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use project::{Entry, Project, ProjectPath, Worktree};
    use std::path::Path;
    use workspace::AppState;

    // Working directory calculation tests

    // No Worktrees in project -> home_dir()
    #[gpui::test]
    async fn no_worktree(cx: &mut TestAppContext) {
        let (project, workspace) = init_test(cx).await;
        cx.read(|cx| {
            let workspace = workspace.read(cx);
            let active_entry = project.read(cx).active_entry();

            //Make sure environment is as expected
            assert!(active_entry.is_none());
            assert!(workspace.worktrees(cx).next().is_none());

            let res = default_working_directory(workspace, cx);
            assert_eq!(res, None);
            let res = first_project_directory(workspace, cx);
            assert_eq!(res, None);
        });
    }

    // No active entry, but a worktree, worktree is a file -> home_dir()
    #[gpui::test]
    async fn no_active_entry_worktree_is_file(cx: &mut TestAppContext) {
        let (project, workspace) = init_test(cx).await;

        create_file_wt(project.clone(), "/root.txt", cx).await;
        cx.read(|cx| {
            let workspace = workspace.read(cx);
            let active_entry = project.read(cx).active_entry();

            //Make sure environment is as expected
            assert!(active_entry.is_none());
            assert!(workspace.worktrees(cx).next().is_some());

            let res = default_working_directory(workspace, cx);
            assert_eq!(res, None);
            let res = first_project_directory(workspace, cx);
            assert_eq!(res, None);
        });
    }

    // No active entry, but a worktree, worktree is a folder -> worktree_folder
    #[gpui::test]
    async fn no_active_entry_worktree_is_dir(cx: &mut TestAppContext) {
        let (project, workspace) = init_test(cx).await;

        let (_wt, _entry) = create_folder_wt(project.clone(), "/root/", cx).await;
        cx.update(|cx| {
            let workspace = workspace.read(cx);
            let active_entry = project.read(cx).active_entry();

            assert!(active_entry.is_none());
            assert!(workspace.worktrees(cx).next().is_some());

            let res = default_working_directory(workspace, cx);
            assert_eq!(res, Some((Path::new("/root/")).to_path_buf()));
            let res = first_project_directory(workspace, cx);
            assert_eq!(res, Some((Path::new("/root/")).to_path_buf()));
        });
    }

    // Active entry with a work tree, worktree is a file -> worktree_folder()
    #[gpui::test]
    async fn active_entry_worktree_is_file(cx: &mut TestAppContext) {
        let (project, workspace) = init_test(cx).await;

        let (_wt, _entry) = create_folder_wt(project.clone(), "/root1/", cx).await;
        let (wt2, entry2) = create_file_wt(project.clone(), "/root2.txt", cx).await;
        insert_active_entry_for(wt2, entry2, project.clone(), cx);

        cx.update(|cx| {
            let workspace = workspace.read(cx);
            let active_entry = project.read(cx).active_entry();

            assert!(active_entry.is_some());

            let res = default_working_directory(workspace, cx);
            assert_eq!(res, Some((Path::new("/root1/")).to_path_buf()));
            let res = first_project_directory(workspace, cx);
            assert_eq!(res, Some((Path::new("/root1/")).to_path_buf()));
        });
    }

    // Active entry, with a worktree, worktree is a folder -> worktree_folder
    #[gpui::test]
    async fn active_entry_worktree_is_dir(cx: &mut TestAppContext) {
        let (project, workspace) = init_test(cx).await;

        let (_wt, _entry) = create_folder_wt(project.clone(), "/root1/", cx).await;
        let (wt2, entry2) = create_folder_wt(project.clone(), "/root2/", cx).await;
        insert_active_entry_for(wt2, entry2, project.clone(), cx);

        cx.update(|cx| {
            let workspace = workspace.read(cx);
            let active_entry = project.read(cx).active_entry();

            assert!(active_entry.is_some());

            let res = default_working_directory(workspace, cx);
            assert_eq!(res, Some((Path::new("/root2/")).to_path_buf()));
            let res = first_project_directory(workspace, cx);
            assert_eq!(res, Some((Path::new("/root1/")).to_path_buf()));
        });
    }

    /// Creates a worktree with 1 file: /root.txt
    pub async fn init_test(cx: &mut TestAppContext) -> (Model<Project>, View<Workspace>) {
        let params = cx.update(AppState::test);
        cx.update(|cx| {
            terminal::init(cx);
            theme::init(theme::LoadThemes::JustBase, cx);
            Project::init_settings(cx);
            language::init(cx);
        });

        let project = Project::test(params.fs.clone(), [], cx).await;
        let workspace = cx
            .add_window(|cx| Workspace::test_new(project.clone(), cx))
            .root_view(cx)
            .unwrap();

        (project, workspace)
    }

    /// Creates a worktree with 1 folder: /root{suffix}/
    async fn create_folder_wt(
        project: Model<Project>,
        path: impl AsRef<Path>,
        cx: &mut TestAppContext,
    ) -> (Model<Worktree>, Entry) {
        create_wt(project, true, path, cx).await
    }

    /// Creates a worktree with 1 file: /root{suffix}.txt
    async fn create_file_wt(
        project: Model<Project>,
        path: impl AsRef<Path>,
        cx: &mut TestAppContext,
    ) -> (Model<Worktree>, Entry) {
        create_wt(project, false, path, cx).await
    }

    async fn create_wt(
        project: Model<Project>,
        is_dir: bool,
        path: impl AsRef<Path>,
        cx: &mut TestAppContext,
    ) -> (Model<Worktree>, Entry) {
        let (wt, _) = project
            .update(cx, |project, cx| {
                project.find_or_create_worktree(path, true, cx)
            })
            .await
            .unwrap();

        let entry = cx
            .update(|cx| wt.update(cx, |wt, cx| wt.create_entry(Path::new(""), is_dir, cx)))
            .await
            .unwrap()
            .to_included()
            .unwrap();

        (wt, entry)
    }

    pub fn insert_active_entry_for(
        wt: Model<Worktree>,
        entry: Entry,
        project: Model<Project>,
        cx: &mut TestAppContext,
    ) {
        cx.update(|cx| {
            let p = ProjectPath {
                worktree_id: wt.read(cx).id(),
                path: entry.path,
            };
            project.update(cx, |project, cx| project.set_active_path(Some(p), cx));
        });
    }

    #[test]
    fn escapes_only_special_characters() {
        assert_eq!(regex_to_literal(r"test(\w)"), r"test\(\\w\)".to_string());
    }

    #[test]
    fn empty_string_stays_empty() {
        assert_eq!(regex_to_literal(""), "".to_string());
    }
}
