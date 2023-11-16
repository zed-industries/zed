#![allow(unused_variables)]
//todo!(remove)

mod persistence;
pub mod terminal_element;
pub mod terminal_panel;

// todo!()
// use crate::terminal_element::TerminalElement;
use editor::{scroll::autoscroll::Autoscroll, Editor};
use gpui::{
    actions, div, img, red, register_action, AnyElement, AppContext, Component, DispatchPhase, Div,
    EventEmitter, FocusEvent, FocusHandle, Focusable, FocusableComponent, FocusableView,
    InputHandler, InteractiveComponent, KeyDownEvent, Keystroke, Model, MouseButton,
    ParentComponent, Pixels, Render, SharedString, Styled, Task, View, ViewContext, VisualContext,
    WeakView,
};
use language::Bias;
use persistence::TERMINAL_DB;
use project::{search::SearchQuery, LocalWorktree, Project};
use terminal::{
    alacritty_terminal::{
        index::Point,
        term::{search::RegexSearch, TermMode},
    },
    terminal_settings::{TerminalBlink, TerminalSettings, WorkingDirectory},
    Event, MaybeNavigationTarget, Terminal,
};
use util::{paths::PathLikeWithPosition, ResultExt};
use workspace::{
    item::{BreadcrumbText, Item, ItemEvent},
    notifications::NotifyResultExt,
    register_deserializable_item,
    searchable::{SearchEvent, SearchOptions, SearchableItem},
    ui::{ContextMenu, ContextMenuItem, Label},
    CloseActiveItem, NewCenterTerminal, Pane, ToolbarItemLocation, Workspace, WorkspaceId,
};

use anyhow::Context;
use dirs::home_dir;
use serde::Deserialize;
use settings::Settings;
use smol::Timer;

use std::{
    ops::RangeInclusive,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

const CURSOR_BLINK_INTERVAL: Duration = Duration::from_millis(500);

///Event to transmit the scroll from the element to the view
#[derive(Clone, Debug, PartialEq)]
pub struct ScrollTerminal(pub i32);

#[register_action]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct SendText(String);

#[register_action]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct SendKeystroke(String);

actions!(Clear, Copy, Paste, ShowCharacterPalette, SearchTest);

pub fn init(cx: &mut AppContext) {
    workspace::ui::init(cx);
    terminal_panel::init(cx);
    terminal::init(cx);

    register_deserializable_item::<TerminalView>(cx);

    cx.observe_new_views(
        |workspace: &mut Workspace, cx: &mut ViewContext<Workspace>| {
            workspace.register_action(TerminalView::deploy);
        },
    )
    .detach();
}

///A terminal view, maintains the PTY's file handles and communicates with the terminal
pub struct TerminalView {
    terminal: Model<Terminal>,
    focus_handle: FocusHandle,
    has_new_content: bool,
    //Currently using iTerm bell, show bell emoji in tab until input is received
    has_bell: bool,
    context_menu: Option<ContextMenu>,
    blink_state: bool,
    blinking_on: bool,
    blinking_paused: bool,
    blink_epoch: usize,
    can_navigate_to_selected_word: bool,
    workspace_id: WorkspaceId,
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
        let strategy = TerminalSettings::get_global(cx);
        let working_directory =
            get_working_directory(workspace, cx, strategy.working_directory.clone());

        let window = cx.window_handle();
        let terminal = workspace
            .project()
            .update(cx, |project, cx| {
                project.create_terminal(working_directory, window, cx)
            })
            .notify_err(workspace, cx);

        if let Some(terminal) = terminal {
            let view = cx.build_view(|cx| {
                TerminalView::new(
                    terminal,
                    workspace.weak_handle(),
                    workspace.database_id(),
                    cx,
                )
            });
            workspace.add_item(Box::new(view), cx)
        }
    }

    pub fn new(
        terminal: Model<Terminal>,
        workspace: WeakView<Workspace>,
        workspace_id: WorkspaceId,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let view_id = cx.entity_id();
        cx.observe(&terminal, |_, _, cx| cx.notify()).detach();
        cx.subscribe(&terminal, move |this, _, event, cx| match event {
            Event::Wakeup => {
                if !this.focus_handle.is_focused(cx) {
                    this.has_new_content = true;
                }
                cx.notify();
                cx.emit(Event::Wakeup);
                cx.emit(ItemEvent::UpdateTab);
                cx.emit(SearchEvent::MatchesInvalidated);
            }

            Event::Bell => {
                this.has_bell = true;
                cx.emit(Event::Wakeup);
            }

            Event::BlinkChanged => this.blinking_on = !this.blinking_on,

            Event::TitleChanged => {
                cx.emit(ItemEvent::UpdateTab);
                if let Some(foreground_info) = &this.terminal().read(cx).foreground_process_info {
                    let cwd = foreground_info.cwd.clone();

                    let item_id = cx.entity_id();
                    let workspace_id = this.workspace_id;
                    cx.background_executor()
                        .spawn(async move {
                            TERMINAL_DB
                                .save_working_directory(item_id.as_u64(), workspace_id, cwd)
                                .await
                                .log_err();
                        })
                        .detach();
                }
            }

            Event::NewNavigationTarget(maybe_navigation_target) => {
                this.can_navigate_to_selected_word = match maybe_navigation_target {
                    Some(MaybeNavigationTarget::Url(_)) => true,
                    Some(MaybeNavigationTarget::PathLike(maybe_path)) => {
                        !possible_open_targets(&workspace, maybe_path, cx).is_empty()
                    }
                    None => false,
                }
            }

            Event::Open(maybe_navigation_target) => match maybe_navigation_target {
                MaybeNavigationTarget::Url(url) => cx.open_url(url),

                MaybeNavigationTarget::PathLike(maybe_path) => {
                    if !this.can_navigate_to_selected_word {
                        return;
                    }
                    let potential_abs_paths = possible_open_targets(&workspace, maybe_path, cx);
                    if let Some(path) = potential_abs_paths.into_iter().next() {
                        let is_dir = path.path_like.is_dir();
                        let task_workspace = workspace.clone();
                        cx.spawn(|_, mut cx| async move {
                            let opened_items = task_workspace
                                .update(&mut cx, |workspace, cx| {
                                    workspace.open_paths(vec![path.path_like], is_dir, cx)
                                })
                                .context("workspace update")?
                                .await;
                            anyhow::ensure!(
                                opened_items.len() == 1,
                                "For a single path open, expected single opened item"
                            );
                            let opened_item = opened_items
                                .into_iter()
                                .next()
                                .unwrap()
                                .transpose()
                                .context("path open")?;
                            if is_dir {
                                task_workspace.update(&mut cx, |workspace, cx| {
                                    workspace.project().update(cx, |_, cx| {
                                        cx.emit(project::Event::ActivateProjectPanel);
                                    })
                                })?;
                            } else {
                                if let Some(row) = path.row {
                                    let col = path.column.unwrap_or(0);
                                    if let Some(active_editor) =
                                        opened_item.and_then(|item| item.downcast::<Editor>())
                                    {
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
                            anyhow::Ok(())
                        })
                        .detach_and_log_err(cx);
                    }
                }
            },
            Event::BreadcrumbsChanged => cx.emit(ItemEvent::UpdateBreadcrumbs),
            Event::CloseTerminal => cx.emit(ItemEvent::CloseItem),
            Event::SelectionsChanged => cx.emit(SearchEvent::ActiveMatchChanged),
        })
        .detach();

        Self {
            terminal,
            has_new_content: true,
            has_bell: false,
            focus_handle: cx.focus_handle(),
            context_menu: None,
            blink_state: true,
            blinking_on: false,
            blinking_paused: false,
            blink_epoch: 0,
            can_navigate_to_selected_word: false,
            workspace_id,
        }
    }

    pub fn model(&self) -> &Model<Terminal> {
        &self.terminal
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

    pub fn deploy_context_menu(
        &mut self,
        position: gpui::Point<Pixels>,
        cx: &mut ViewContext<Self>,
    ) {
        self.context_menu = Some(ContextMenu::new(vec![
            ContextMenuItem::entry(Label::new("Clear"), Clear),
            ContextMenuItem::entry(Label::new("Close"), CloseActiveItem { save_intent: None }),
        ]));
        dbg!(&position);
        // todo!()
        //     self.context_menu
        //         .show(position, AnchorCorner::TopLeft, menu_entries, cx);
        //     cx.notify();
    }

    fn show_character_palette(&mut self, _: &ShowCharacterPalette, cx: &mut ViewContext<Self>) {
        if !self
            .terminal
            .read(cx)
            .last_content
            .mode
            .contains(TermMode::ALT_SCREEN)
        {
            cx.show_character_palette();
        } else {
            self.terminal.update(cx, |term, cx| {
                term.try_keystroke(
                    &Keystroke::parse("ctrl-cmd-space").unwrap(),
                    TerminalSettings::get_global(cx).option_as_meta,
                )
            });
        }
    }

    fn select_all(&mut self, _: &editor::SelectAll, cx: &mut ViewContext<Self>) {
        self.terminal.update(cx, |term, _| term.select_all());
        cx.notify();
    }

    fn clear(&mut self, _: &Clear, cx: &mut ViewContext<Self>) {
        self.terminal.update(cx, |term, _| term.clear());
        cx.notify();
    }

    pub fn should_show_cursor(&self, focused: bool, cx: &mut gpui::ViewContext<Self>) -> bool {
        //Don't blink the cursor when not focused, blinking is disabled, or paused
        if !focused
            || !self.blinking_on
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
            TerminalBlink::TerminalControlled | TerminalBlink::On => self.blink_state,
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
                    .log_err();
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

    pub fn find_matches(
        &mut self,
        query: Arc<project::search::SearchQuery>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Vec<RangeInclusive<Point>>> {
        let searcher = regex_search_for_query(&query);

        if let Some(searcher) = searcher {
            self.terminal
                .update(cx, |term, cx| term.find_matches(searcher, cx))
        } else {
            cx.background_executor().spawn(async { Vec::new() })
        }
    }

    pub fn terminal(&self) -> &Model<Terminal> {
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

    fn send_text(&mut self, text: &SendText, cx: &mut ViewContext<Self>) {
        self.clear_bel(cx);
        self.terminal.update(cx, |term, _| {
            term.input(text.0.to_string());
        });
    }

    fn send_keystroke(&mut self, text: &SendKeystroke, cx: &mut ViewContext<Self>) {
        if let Some(keystroke) = Keystroke::parse(&text.0).log_err() {
            self.clear_bel(cx);
            self.terminal.update(cx, |term, cx| {
                term.try_keystroke(&keystroke, TerminalSettings::get_global(cx).option_as_meta);
            });
        }
    }
}

fn possible_open_targets(
    workspace: &WeakView<Workspace>,
    maybe_path: &String,
    cx: &mut ViewContext<'_, TerminalView>,
) -> Vec<PathLikeWithPosition<PathBuf>> {
    let path_like = PathLikeWithPosition::parse_str(maybe_path.as_str(), |path_str| {
        Ok::<_, std::convert::Infallible>(Path::new(path_str).to_path_buf())
    })
    .expect("infallible");
    let maybe_path = path_like.path_like;
    let potential_abs_paths = if maybe_path.is_absolute() {
        vec![maybe_path]
    } else if maybe_path.starts_with("~") {
        if let Some(abs_path) = maybe_path
            .strip_prefix("~")
            .ok()
            .and_then(|maybe_path| Some(dirs::home_dir()?.join(maybe_path)))
        {
            vec![abs_path]
        } else {
            Vec::new()
        }
    } else if let Some(workspace) = workspace.upgrade() {
        workspace.update(cx, |workspace, cx| {
            workspace
                .worktrees(cx)
                .map(|worktree| worktree.read(cx).abs_path().join(&maybe_path))
                .collect()
        })
    } else {
        Vec::new()
    };

    potential_abs_paths
        .into_iter()
        .filter(|path| path.exists())
        .map(|path| PathLikeWithPosition {
            path_like: path,
            row: path_like.row,
            column: path_like.column,
        })
        .collect()
}

pub fn regex_search_for_query(query: &project::search::SearchQuery) -> Option<RegexSearch> {
    let query = query.as_str();
    let searcher = RegexSearch::new(&query);
    searcher.ok()
}

impl TerminalView {
    fn key_down(
        &mut self,
        event: &KeyDownEvent,
        _dispatch_phase: DispatchPhase,
        cx: &mut ViewContext<Self>,
    ) {
        self.clear_bel(cx);
        self.pause_cursor_blinking(cx);

        self.terminal.update(cx, |term, cx| {
            term.try_keystroke(
                &event.keystroke,
                TerminalSettings::get_global(cx).option_as_meta,
            )
        });
    }

    fn focus_in(&mut self, event: &FocusEvent, cx: &mut ViewContext<Self>) {
        self.has_new_content = false;
        self.terminal.read(cx).focus_in();
        self.blink_cursors(self.blink_epoch, cx);
        cx.notify();
    }

    fn focus_out(&mut self, event: &FocusEvent, cx: &mut ViewContext<Self>) {
        self.terminal.update(cx, |terminal, _| {
            terminal.focus_out();
        });
        cx.notify();
    }
}

impl Render for TerminalView {
    type Element = Focusable<Self, Div<Self>>;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        let terminal_handle = self.terminal.clone().downgrade();

        let self_id = cx.entity_id();
        let focused = self.focus_handle.is_focused(cx);

        div()
            .relative()
            .child(
                div()
                    .z_index(0)
                    .absolute()
                    .on_key_down(Self::key_down)
                    .on_action(TerminalView::send_text)
                    .on_action(TerminalView::send_keystroke)
                    .on_action(TerminalView::copy)
                    .on_action(TerminalView::paste)
                    .on_action(TerminalView::clear)
                    .on_action(TerminalView::show_character_palette)
                    .on_action(TerminalView::select_all)
                    // todo!()
                    .child(
                        "TERMINAL HERE", //     TerminalElement::new(
                                         //     terminal_handle,
                                         //     focused,
                                         //     self.should_show_cursor(focused, cx),
                                         //     self.can_navigate_to_selected_word,
                                         // )
                    )
                    .on_mouse_down(MouseButton::Right, |this, event, cx| {
                        this.deploy_context_menu(event.position, cx);
                        cx.notify();
                    }),
            )
            .children(
                self.context_menu
                    .clone()
                    .map(|context_menu| div().z_index(1).absolute().child(context_menu.render())),
            )
            .track_focus(&self.focus_handle)
            .on_focus_in(Self::focus_in)
            .on_focus_out(Self::focus_out)
    }
}

// impl View for TerminalView {
//todo!()
// fn modifiers_changed(
//     &mut self,
//     event: &ModifiersChangedEvent,
//     cx: &mut ViewContext<Self>,
// ) -> bool {
//     let handled = self
//         .terminal()
//         .update(cx, |term, _| term.try_modifiers_change(&event.modifiers));
//     if handled {
//         cx.notify();
//     }
//     handled
// }
// }

// todo!()
// fn update_keymap_context(&self, keymap: &mut KeymapContext, cx: &gpui::AppContext) {
//     Self::reset_to_default_keymap_context(keymap);

//     let mode = self.terminal.read(cx).last_content.mode;
//     keymap.add_key(
//         "screen",
//         if mode.contains(TermMode::ALT_SCREEN) {
//             "alt"
//         } else {
//             "normal"
//         },
//     );

//     if mode.contains(TermMode::APP_CURSOR) {
//         keymap.add_identifier("DECCKM");
//     }
//     if mode.contains(TermMode::APP_KEYPAD) {
//         keymap.add_identifier("DECPAM");
//     } else {
//         keymap.add_identifier("DECPNM");
//     }
//     if mode.contains(TermMode::SHOW_CURSOR) {
//         keymap.add_identifier("DECTCEM");
//     }
//     if mode.contains(TermMode::LINE_WRAP) {
//         keymap.add_identifier("DECAWM");
//     }
//     if mode.contains(TermMode::ORIGIN) {
//         keymap.add_identifier("DECOM");
//     }
//     if mode.contains(TermMode::INSERT) {
//         keymap.add_identifier("IRM");
//     }
//     //LNM is apparently the name for this. https://vt100.net/docs/vt510-rm/LNM.html
//     if mode.contains(TermMode::LINE_FEED_NEW_LINE) {
//         keymap.add_identifier("LNM");
//     }
//     if mode.contains(TermMode::FOCUS_IN_OUT) {
//         keymap.add_identifier("report_focus");
//     }
//     if mode.contains(TermMode::ALTERNATE_SCROLL) {
//         keymap.add_identifier("alternate_scroll");
//     }
//     if mode.contains(TermMode::BRACKETED_PASTE) {
//         keymap.add_identifier("bracketed_paste");
//     }
//     if mode.intersects(TermMode::MOUSE_MODE) {
//         keymap.add_identifier("any_mouse_reporting");
//     }
//     {
//         let mouse_reporting = if mode.contains(TermMode::MOUSE_REPORT_CLICK) {
//             "click"
//         } else if mode.contains(TermMode::MOUSE_DRAG) {
//             "drag"
//         } else if mode.contains(TermMode::MOUSE_MOTION) {
//             "motion"
//         } else {
//             "off"
//         };
//         keymap.add_key("mouse_reporting", mouse_reporting);
//     }
//     {
//         let format = if mode.contains(TermMode::SGR_MOUSE) {
//             "sgr"
//         } else if mode.contains(TermMode::UTF8_MOUSE) {
//             "utf8"
//         } else {
//             "normal"
//         };
//         keymap.add_key("mouse_format", format);
//     }
// }

impl InputHandler for TerminalView {
    fn text_for_range(
        &mut self,
        range: std::ops::Range<usize>,
        cx: &mut ViewContext<Self>,
    ) -> Option<String> {
        todo!()
    }

    fn selected_text_range(
        &mut self,
        cx: &mut ViewContext<Self>,
    ) -> Option<std::ops::Range<usize>> {
        if self
            .terminal
            .read(cx)
            .last_content
            .mode
            .contains(TermMode::ALT_SCREEN)
        {
            None
        } else {
            Some(0..0)
        }
    }

    fn marked_text_range(&self, cx: &mut ViewContext<Self>) -> Option<std::ops::Range<usize>> {
        todo!()
    }

    fn unmark_text(&mut self, cx: &mut ViewContext<Self>) {
        todo!()
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

    fn replace_and_mark_text_in_range(
        &mut self,
        range: Option<std::ops::Range<usize>>,
        new_text: &str,
        new_selected_range: Option<std::ops::Range<usize>>,
        cx: &mut ViewContext<Self>,
    ) {
        todo!()
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: std::ops::Range<usize>,
        element_bounds: gpui::Bounds<Pixels>,
        cx: &mut ViewContext<Self>,
    ) -> Option<gpui::Bounds<Pixels>> {
        todo!()
    }
}

impl Item for TerminalView {
    fn tab_tooltip_text(&self, cx: &AppContext) -> Option<SharedString> {
        Some(self.terminal().read(cx).title().into())
    }

    fn tab_content<T: 'static>(
        &self,
        _detail: Option<usize>,
        cx: &gpui::AppContext,
    ) -> AnyElement<T> {
        let title = self.terminal().read(cx).title();

        div()
            .child(img().uri("icons/terminal.svg").bg(red()))
            .child(title)
            .render()
    }

    fn clone_on_split(
        &self,
        _workspace_id: WorkspaceId,
        _cx: &mut ViewContext<Self>,
    ) -> Option<View<Self>> {
        //From what I can tell, there's no  way to tell the current working
        //Directory of the terminal from outside the shell. There might be
        //solutions to this, but they are non-trivial and require more IPC

        // Some(TerminalContainer::new(
        //     Err(anyhow::anyhow!("failed to instantiate terminal")),
        //     workspace_id,
        //     cx,
        // ))

        // TODO
        None
    }

    fn is_dirty(&self, _cx: &gpui::AppContext) -> bool {
        self.has_bell()
    }

    fn has_conflict(&self, _cx: &AppContext) -> bool {
        false
    }

    // todo!()
    // fn as_searchable(&self, handle: &View<Self>) -> Option<Box<dyn SearchableItemHandle>> {
    //     Some(Box::new(handle.clone()))
    // }

    fn breadcrumb_location(&self) -> ToolbarItemLocation {
        ToolbarItemLocation::PrimaryLeft { flex: None }
    }

    fn breadcrumbs(&self, _: &theme::Theme, cx: &AppContext) -> Option<Vec<BreadcrumbText>> {
        Some(vec![BreadcrumbText {
            text: self.terminal().read(cx).breadcrumb_text.clone(),
            highlights: None,
        }])
    }

    fn serialized_item_kind() -> Option<&'static str> {
        Some("Terminal")
    }

    fn deserialize(
        project: Model<Project>,
        workspace: WeakView<Workspace>,
        workspace_id: workspace::WorkspaceId,
        item_id: workspace::ItemId,
        cx: &mut ViewContext<Pane>,
    ) -> Task<anyhow::Result<View<Self>>> {
        let window = cx.window_handle();
        cx.spawn(|pane, mut cx| async move {
            let cwd = None;
            // todo!()
            // TERMINAL_DB
            // .get_working_directory(item_id, workspace_id)
            // .log_err()
            // .flatten()
            // .or_else(|| {
            //     cx.read(|cx| {
            //         let strategy = TerminalSettings::get_global(cx).working_directory.clone();
            //         workspace
            //             .upgrade()
            //             .map(|workspace| {
            //                 get_working_directory(workspace.read(cx), cx, strategy)
            //             })
            //             .flatten()
            //     })
            // });

            let terminal = project.update(&mut cx, |project, cx| {
                project.create_terminal(cwd, window, cx)
            })??;
            pane.update(&mut cx, |_, cx| {
                cx.build_view(|cx| TerminalView::new(terminal, workspace, workspace_id, cx))
            })
        })
    }

    fn added_to_workspace(&mut self, workspace: &mut Workspace, cx: &mut ViewContext<Self>) {
        // todo!()
        // cx.background()
        //     .spawn(TERMINAL_DB.update_workspace_id(
        //         workspace.database_id(),
        //         self.workspace_id,
        //         cx.view_id(),
        //     ))
        //     .detach();
        self.workspace_id = workspace.database_id();
    }
}

impl SearchableItem for TerminalView {
    type Match = RangeInclusive<Point>;

    fn supported_options() -> SearchOptions {
        SearchOptions {
            case: false,
            word: false,
            regex: false,
            replacement: false,
        }
    }

    /// Clear stored matches
    fn clear_matches(&mut self, cx: &mut ViewContext<Self>) {
        self.terminal().update(cx, |term, _| term.matches.clear())
    }

    /// Store matches returned from find_matches somewhere for rendering
    fn update_matches(&mut self, matches: Vec<Self::Match>, cx: &mut ViewContext<Self>) {
        self.terminal().update(cx, |term, _| term.matches = matches)
    }

    /// Return the selection content to pre-load into this search
    fn query_suggestion(&mut self, cx: &mut ViewContext<Self>) -> String {
        self.terminal()
            .read(cx)
            .last_content
            .selection_text
            .clone()
            .unwrap_or_default()
    }

    /// Focus match at given index into the Vec of matches
    fn activate_match(&mut self, index: usize, _: Vec<Self::Match>, cx: &mut ViewContext<Self>) {
        self.terminal()
            .update(cx, |term, _| term.activate_match(index));
        cx.notify();
    }

    /// Add selections for all matches given.
    fn select_matches(&mut self, matches: Vec<Self::Match>, cx: &mut ViewContext<Self>) {
        self.terminal()
            .update(cx, |term, _| term.select_matches(matches));
        cx.notify();
    }

    /// Get all of the matches for this query, should be done on the background
    fn find_matches(
        &mut self,
        query: Arc<project::search::SearchQuery>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Vec<Self::Match>> {
        if let Some(searcher) = regex_search_for_query(&query) {
            self.terminal()
                .update(cx, |term, cx| term.find_matches(searcher, cx))
        } else {
            Task::ready(vec![])
        }
    }

    /// Reports back to the search toolbar what the active match should be (the selection)
    fn active_match_index(
        &mut self,
        matches: Vec<Self::Match>,
        cx: &mut ViewContext<Self>,
    ) -> Option<usize> {
        // Selection head might have a value if there's a selection that isn't
        // associated with a match. Therefore, if there are no matches, we should
        // report None, no matter the state of the terminal
        let res = if matches.len() > 0 {
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

            let res = current_project_directory(workspace, cx);
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

            let res = current_project_directory(workspace, cx);
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

            let res = current_project_directory(workspace, cx);
            assert_eq!(res, Some((Path::new("/root/")).to_path_buf()));
            let res = first_project_directory(workspace, cx);
            assert_eq!(res, Some((Path::new("/root/")).to_path_buf()));
        });
    }

    // Active entry with a work tree, worktree is a file -> home_dir()
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

            let res = current_project_directory(workspace, cx);
            assert_eq!(res, None);
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

            let res = current_project_directory(workspace, cx);
            assert_eq!(res, Some((Path::new("/root2/")).to_path_buf()));
            let res = first_project_directory(workspace, cx);
            assert_eq!(res, Some((Path::new("/root1/")).to_path_buf()));
        });
    }

    /// Creates a worktree with 1 file: /root.txt
    pub async fn init_test(cx: &mut TestAppContext) -> (Model<Project>, View<Workspace>) {
        let params = cx.update(AppState::test);
        cx.update(|cx| {
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
                project.find_or_create_local_worktree(path, true, cx)
            })
            .await
            .unwrap();

        let entry = cx
            .update(|cx| {
                wt.update(cx, |wt, cx| {
                    wt.as_local()
                        .unwrap()
                        .create_entry(Path::new(""), is_dir, cx)
                })
            })
            .await
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
}
