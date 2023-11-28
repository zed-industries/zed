use crate::{
    history::SearchHistory,
    mode::{next_mode, SearchMode},
    search_bar::{render_nav_button, render_search_mode_button},
    ActivateRegexMode, ActivateTextMode, CycleMode, NextHistoryQuery, PreviousHistoryQuery,
    ReplaceAll, ReplaceNext, SearchOptions, SelectAllMatches, SelectNextMatch, SelectPrevMatch,
    ToggleCaseSensitive, ToggleReplace, ToggleWholeWord,
};
use collections::HashMap;
use editor::{Editor, EditorMode};
use futures::channel::oneshot;
use gpui::{
    actions, div, red, Action, AppContext, Div, EventEmitter, InteractiveElement as _, IntoElement,
    ParentElement as _, Render, Styled, Subscription, Task, View, ViewContext, VisualContext as _,
    WeakView, WindowContext,
};
use project::search::SearchQuery;
use serde::Deserialize;
use std::{any::Any, sync::Arc};

use ui::{h_stack, ButtonGroup, Icon, IconButton, IconElement};
use util::ResultExt;
use workspace::{
    item::ItemHandle,
    searchable::{Direction, SearchEvent, SearchableItemHandle, WeakSearchableItemHandle},
    ToolbarItemLocation, ToolbarItemView,
};

#[derive(PartialEq, Clone, Deserialize, Default, Action)]
pub struct Deploy {
    pub focus: bool,
}

actions!(Dismiss, FocusEditor);

pub enum Event {
    UpdateLocation,
}

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|editor: &mut Editor, cx| BufferSearchBar::register(editor, cx))
        .detach();
}

pub struct BufferSearchBar {
    query_editor: View<Editor>,
    replacement_editor: View<Editor>,
    active_searchable_item: Option<Box<dyn SearchableItemHandle>>,
    active_match_index: Option<usize>,
    active_searchable_item_subscription: Option<Subscription>,
    active_search: Option<Arc<SearchQuery>>,
    searchable_items_with_matches:
        HashMap<Box<dyn WeakSearchableItemHandle>, Vec<Box<dyn Any + Send>>>,
    pending_search: Option<Task<()>>,
    search_options: SearchOptions,
    default_options: SearchOptions,
    query_contains_error: bool,
    dismissed: bool,
    search_history: SearchHistory,
    current_mode: SearchMode,
    replace_enabled: bool,
}

impl EventEmitter<Event> for BufferSearchBar {}
impl EventEmitter<workspace::ToolbarItemEvent> for BufferSearchBar {}
impl Render for BufferSearchBar {
    type Element = Div;
    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        // let query_container_style = if self.query_contains_error {
        //     theme.search.invalid_editor
        // } else {
        //     theme.search.editor.input.container
        // };
        if self.dismissed {
            return div();
        }
        let supported_options = self.supported_options();

        let previous_query_keystrokes = cx
            .bindings_for_action(&PreviousHistoryQuery {})
            .into_iter()
            .next()
            .map(|binding| {
                binding
                    .keystrokes()
                    .iter()
                    .map(|k| k.to_string())
                    .collect::<Vec<_>>()
            });
        let next_query_keystrokes = cx
            .bindings_for_action(&NextHistoryQuery {})
            .into_iter()
            .next()
            .map(|binding| {
                binding
                    .keystrokes()
                    .iter()
                    .map(|k| k.to_string())
                    .collect::<Vec<_>>()
            });
        let new_placeholder_text = match (previous_query_keystrokes, next_query_keystrokes) {
            (Some(previous_query_keystrokes), Some(next_query_keystrokes)) => {
                format!(
                    "Search ({}/{} for previous/next query)",
                    previous_query_keystrokes.join(" "),
                    next_query_keystrokes.join(" ")
                )
            }
            (None, Some(next_query_keystrokes)) => {
                format!(
                    "Search ({} for next query)",
                    next_query_keystrokes.join(" ")
                )
            }
            (Some(previous_query_keystrokes), None) => {
                format!(
                    "Search ({} for previous query)",
                    previous_query_keystrokes.join(" ")
                )
            }
            (None, None) => String::new(),
        };
        let new_placeholder_text = Arc::from(new_placeholder_text);
        self.query_editor.update(cx, |editor, cx| {
            editor.set_placeholder_text(new_placeholder_text, cx);
        });
        self.replacement_editor.update(cx, |editor, cx| {
            editor.set_placeholder_text("Replace with...", cx);
        });

        let search_button_for_mode = |mode| {
            let is_active = self.current_mode == mode;

            render_search_mode_button(
                mode,
                is_active,
                cx.listener(move |this, _, cx| {
                    this.activate_search_mode(mode, cx);
                }),
            )
        };
        let search_option_button = |option| {
            let is_active = self.search_options.contains(option);
            option.as_button(is_active)
        };
        let match_count = self
            .active_searchable_item
            .as_ref()
            .and_then(|searchable_item| {
                if self.query(cx).is_empty() {
                    return None;
                }
                let matches = self
                    .searchable_items_with_matches
                    .get(&searchable_item.downgrade())?;
                let message = if let Some(match_ix) = self.active_match_index {
                    format!("{}/{}", match_ix + 1, matches.len())
                } else {
                    "No matches".to_string()
                };

                Some(ui::Label::new(message))
            });
        let nav_button_for_direction = |icon, direction| {
            render_nav_button(
                icon,
                self.active_match_index.is_some(),
                cx.listener(move |this, _, cx| match direction {
                    Direction::Prev => this.select_prev_match(&Default::default(), cx),
                    Direction::Next => this.select_next_match(&Default::default(), cx),
                }),
            )
        };
        let should_show_replace_input = self.replace_enabled && supported_options.replacement;
        let replace_all = should_show_replace_input
            .then(|| super::render_replace_button(ReplaceAll, ui::Icon::ReplaceAll));
        let replace_next = should_show_replace_input
            .then(|| super::render_replace_button(ReplaceNext, ui::Icon::Replace));
        let in_replace = self.replacement_editor.focus_handle(cx).is_focused(cx);

        h_stack()
            .key_context("BufferSearchBar")
            .when(in_replace, |this| {
                this.key_context("in_replace")
                    .on_action(cx.listener(Self::replace_next))
                    .on_action(cx.listener(Self::replace_all))
            })
            .on_action(cx.listener(Self::previous_history_query))
            .on_action(cx.listener(Self::next_history_query))
            .on_action(cx.listener(Self::dismiss))
            .w_full()
            .p_1()
            .child(
                div()
                    .flex()
                    .flex_1()
                    .border_1()
                    .border_color(red())
                    .rounded_md()
                    .items_center()
                    .child(IconElement::new(Icon::MagnifyingGlass))
                    .child(self.query_editor.clone())
                    .children(
                        supported_options
                            .case
                            .then(|| search_option_button(SearchOptions::CASE_SENSITIVE)),
                    )
                    .children(
                        supported_options
                            .word
                            .then(|| search_option_button(SearchOptions::WHOLE_WORD)),
                    ),
            )
            .child(
                h_stack()
                    .flex_none()
                    .child(ButtonGroup::new(vec![
                        search_button_for_mode(SearchMode::Text),
                        search_button_for_mode(SearchMode::Regex),
                    ]))
                    .when(supported_options.replacement, |this| {
                        this.child(super::toggle_replace_button(self.replace_enabled))
                    }),
            )
            .child(
                h_stack()
                    .gap_0p5()
                    .flex_1()
                    .when(self.replace_enabled, |this| {
                        this.child(self.replacement_editor.clone())
                            .children(replace_next)
                            .children(replace_all)
                    }),
            )
            .child(
                h_stack()
                    .gap_0p5()
                    .flex_none()
                    .child(self.render_action_button())
                    .children(match_count)
                    .child(nav_button_for_direction(
                        ui::Icon::ChevronLeft,
                        Direction::Prev,
                    ))
                    .child(nav_button_for_direction(
                        ui::Icon::ChevronRight,
                        Direction::Next,
                    )),
            )
    }
}

impl ToolbarItemView for BufferSearchBar {
    fn set_active_pane_item(
        &mut self,
        item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) -> ToolbarItemLocation {
        cx.notify();
        self.active_searchable_item_subscription.take();
        self.active_searchable_item.take();

        self.pending_search.take();

        if let Some(searchable_item_handle) =
            item.and_then(|item| item.to_searchable_item_handle(cx))
        {
            let this = cx.view().downgrade();

            searchable_item_handle
                .subscribe_to_search_events(
                    cx,
                    Box::new(move |search_event, cx| {
                        if let Some(this) = this.upgrade() {
                            this.update(cx, |this, cx| {
                                this.on_active_searchable_item_event(search_event, cx)
                            });
                        }
                    }),
                )
                .detach();

            self.active_searchable_item = Some(searchable_item_handle);
            let _ = self.update_matches(cx);
            if !self.dismissed {
                return ToolbarItemLocation::Secondary;
            }
        }
        ToolbarItemLocation::Hidden
    }

    fn row_count(&self, _: &WindowContext<'_>) -> usize {
        1
    }
}

impl BufferSearchBar {
    pub fn register(editor: &mut Editor, cx: &mut ViewContext<Editor>) {
        if editor.mode() != EditorMode::Full {
            return;
        };

        let handle = cx.view().downgrade();

        editor.register_action(move |a: &Deploy, cx| {
            let Some(pane) = handle.upgrade().and_then(|editor| editor.read(cx).pane(cx)) else {
                return;
            };

            pane.update(cx, |this, cx| {
                this.toolbar().update(cx, |this, cx| {
                    if let Some(search_bar) = this.item_of_type::<BufferSearchBar>() {
                        search_bar.update(cx, |this, cx| {
                            if this.is_dismissed() {
                                this.show(cx);
                            } else {
                                this.dismiss(&Dismiss, cx);
                            }
                        });
                        return;
                    }
                    let view = cx.build_view(|cx| BufferSearchBar::new(cx));
                    this.add_item(view.clone(), cx);
                    view.update(cx, |this, cx| this.deploy(a, cx));
                    cx.notify();
                })
            });
        });
        fn register_action<A: Action>(
            editor: &mut Editor,
            handle: WeakView<Editor>,
            update: fn(&mut BufferSearchBar, &A, &mut ViewContext<BufferSearchBar>),
        ) {
            editor.register_action(move |action: &A, cx| {
                let Some(pane) = handle.upgrade().and_then(|editor| editor.read(cx).pane(cx))
                else {
                    return;
                };
                pane.update(cx, move |this, cx| {
                    this.toolbar().update(cx, move |this, cx| {
                        if let Some(search_bar) = this.item_of_type::<BufferSearchBar>() {
                            search_bar.update(cx, move |this, cx| update(this, action, cx));
                            cx.notify();
                        }
                    })
                });
            });
        }

        let handle = cx.view().downgrade();
        register_action(
            editor,
            handle.clone(),
            |this, action: &ToggleCaseSensitive, cx| {
                if this.supported_options().case {
                    this.toggle_case_sensitive(action, cx);
                }
            },
        );
        register_action(
            editor,
            handle.clone(),
            |this, action: &ToggleWholeWord, cx| {
                if this.supported_options().word {
                    this.toggle_whole_word(action, cx);
                }
            },
        );
        register_action(
            editor,
            handle.clone(),
            |this, action: &ToggleReplace, cx| {
                if this.supported_options().replacement {
                    this.toggle_replace(action, cx);
                }
            },
        );
        register_action(editor, handle.clone(), |this, _: &ActivateRegexMode, cx| {
            if this.supported_options().regex {
                this.activate_search_mode(SearchMode::Regex, cx);
            }
        });
        register_action(editor, handle.clone(), |this, _: &ActivateTextMode, cx| {
            this.activate_search_mode(SearchMode::Text, cx);
        });
        register_action(editor, handle.clone(), |this, action: &CycleMode, cx| {
            if this.supported_options().regex {
                // If regex is not supported then search has just one mode (text) - in that case there's no point in supporting
                // cycling.
                this.cycle_mode(action, cx)
            }
        });
        register_action(
            editor,
            handle.clone(),
            |this, action: &SelectNextMatch, cx| {
                this.select_next_match(action, cx);
            },
        );
        register_action(
            editor,
            handle.clone(),
            |this, action: &SelectPrevMatch, cx| {
                this.select_prev_match(action, cx);
            },
        );
        register_action(
            editor,
            handle.clone(),
            |this, action: &SelectAllMatches, cx| {
                this.select_all_matches(action, cx);
            },
        );
        register_action(editor, handle.clone(), |this, _: &editor::Cancel, cx| {
            if !this.dismissed {
                this.dismiss(&Dismiss, cx);
                return;
            }
            cx.propagate();
        });
    }
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let query_editor = cx.build_view(|cx| Editor::single_line(cx));
        cx.subscribe(&query_editor, Self::on_query_editor_event)
            .detach();
        let replacement_editor = cx.build_view(|cx| Editor::single_line(cx));
        cx.subscribe(&replacement_editor, Self::on_query_editor_event)
            .detach();
        Self {
            query_editor,
            replacement_editor,
            active_searchable_item: None,
            active_searchable_item_subscription: None,
            active_match_index: None,
            searchable_items_with_matches: Default::default(),
            default_options: SearchOptions::NONE,
            search_options: SearchOptions::NONE,
            pending_search: None,
            query_contains_error: false,
            dismissed: true,
            search_history: SearchHistory::default(),
            current_mode: SearchMode::default(),
            active_search: None,
            replace_enabled: false,
        }
    }

    pub fn is_dismissed(&self) -> bool {
        self.dismissed
    }

    pub fn dismiss(&mut self, _: &Dismiss, cx: &mut ViewContext<Self>) {
        self.dismissed = true;
        for searchable_item in self.searchable_items_with_matches.keys() {
            if let Some(searchable_item) =
                WeakSearchableItemHandle::upgrade(searchable_item.as_ref(), cx)
            {
                searchable_item.clear_matches(cx);
            }
        }
        if let Some(active_editor) = self.active_searchable_item.as_ref() {
            let handle = active_editor.focus_handle(cx);
            cx.focus(&handle);
        }
        cx.emit(Event::UpdateLocation);
        cx.notify();
    }

    pub fn deploy(&mut self, deploy: &Deploy, cx: &mut ViewContext<Self>) -> bool {
        if self.show(cx) {
            self.search_suggested(cx);
            if deploy.focus {
                self.select_query(cx);
                let handle = cx.focus_handle();
                cx.focus(&handle);
            }
            return true;
        }

        false
    }

    pub fn show(&mut self, cx: &mut ViewContext<Self>) -> bool {
        if self.active_searchable_item.is_none() {
            return false;
        }
        self.dismissed = false;
        cx.notify();
        cx.emit(Event::UpdateLocation);
        true
    }

    fn supported_options(&self) -> workspace::searchable::SearchOptions {
        self.active_searchable_item
            .as_deref()
            .map(SearchableItemHandle::supported_options)
            .unwrap_or_default()
    }
    pub fn search_suggested(&mut self, cx: &mut ViewContext<Self>) {
        let search = self
            .query_suggestion(cx)
            .map(|suggestion| self.search(&suggestion, Some(self.default_options), cx));

        if let Some(search) = search {
            cx.spawn(|this, mut cx| async move {
                search.await?;
                this.update(&mut cx, |this, cx| this.activate_current_match(cx))
            })
            .detach_and_log_err(cx);
        }
    }

    pub fn activate_current_match(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(match_ix) = self.active_match_index {
            if let Some(active_searchable_item) = self.active_searchable_item.as_ref() {
                if let Some(matches) = self
                    .searchable_items_with_matches
                    .get(&active_searchable_item.downgrade())
                {
                    active_searchable_item.activate_match(match_ix, matches, cx)
                }
            }
        }
    }

    pub fn select_query(&mut self, cx: &mut ViewContext<Self>) {
        self.query_editor.update(cx, |query_editor, cx| {
            query_editor.select_all(&Default::default(), cx);
        });
    }

    pub fn query(&self, cx: &WindowContext) -> String {
        self.query_editor.read(cx).text(cx)
    }
    pub fn replacement(&self, cx: &WindowContext) -> String {
        self.replacement_editor.read(cx).text(cx)
    }
    pub fn query_suggestion(&mut self, cx: &mut ViewContext<Self>) -> Option<String> {
        self.active_searchable_item
            .as_ref()
            .map(|searchable_item| searchable_item.query_suggestion(cx))
            .filter(|suggestion| !suggestion.is_empty())
    }

    pub fn set_replacement(&mut self, replacement: Option<&str>, cx: &mut ViewContext<Self>) {
        if replacement.is_none() {
            self.replace_enabled = false;
            return;
        }
        self.replace_enabled = true;
        self.replacement_editor
            .update(cx, |replacement_editor, cx| {
                replacement_editor
                    .buffer()
                    .update(cx, |replacement_buffer, cx| {
                        let len = replacement_buffer.len(cx);
                        replacement_buffer.edit([(0..len, replacement.unwrap())], None, cx);
                    });
            });
    }

    pub fn search(
        &mut self,
        query: &str,
        options: Option<SearchOptions>,
        cx: &mut ViewContext<Self>,
    ) -> oneshot::Receiver<()> {
        let options = options.unwrap_or(self.default_options);
        if query != self.query(cx) || self.search_options != options {
            self.query_editor.update(cx, |query_editor, cx| {
                query_editor.buffer().update(cx, |query_buffer, cx| {
                    let len = query_buffer.len(cx);
                    query_buffer.edit([(0..len, query)], None, cx);
                });
            });
            self.search_options = options;
            self.query_contains_error = false;
            self.clear_matches(cx);
            cx.notify();
        }
        self.update_matches(cx)
    }

    fn render_action_button(&self) -> impl IntoElement {
        // let tooltip_style = theme.tooltip.clone();

        // let style = theme.search.action_button.clone();

        IconButton::new(0, ui::Icon::SelectAll)
            .on_click(|_, cx| cx.dispatch_action(Box::new(SelectAllMatches)))
    }

    pub fn activate_search_mode(&mut self, mode: SearchMode, cx: &mut ViewContext<Self>) {
        assert_ne!(
            mode,
            SearchMode::Semantic,
            "Semantic search is not supported in buffer search"
        );
        if mode == self.current_mode {
            return;
        }
        self.current_mode = mode;
        let _ = self.update_matches(cx);
        cx.notify();
    }

    pub fn focus_editor(&mut self, _: &FocusEditor, cx: &mut ViewContext<Self>) {
        if let Some(active_editor) = self.active_searchable_item.as_ref() {
            let handle = active_editor.focus_handle(cx);
            cx.focus(&handle);
        }
    }

    fn toggle_search_option(&mut self, search_option: SearchOptions, cx: &mut ViewContext<Self>) {
        self.search_options.toggle(search_option);
        self.default_options = self.search_options;
        let _ = self.update_matches(cx);
        cx.notify();
    }

    pub fn set_search_options(
        &mut self,
        search_options: SearchOptions,
        cx: &mut ViewContext<Self>,
    ) {
        self.search_options = search_options;
        cx.notify();
    }

    fn select_next_match(&mut self, _: &SelectNextMatch, cx: &mut ViewContext<Self>) {
        self.select_match(Direction::Next, 1, cx);
    }

    fn select_prev_match(&mut self, _: &SelectPrevMatch, cx: &mut ViewContext<Self>) {
        self.select_match(Direction::Prev, 1, cx);
    }

    fn select_all_matches(&mut self, _: &SelectAllMatches, cx: &mut ViewContext<Self>) {
        if !self.dismissed && self.active_match_index.is_some() {
            if let Some(searchable_item) = self.active_searchable_item.as_ref() {
                if let Some(matches) = self
                    .searchable_items_with_matches
                    .get(&searchable_item.downgrade())
                {
                    searchable_item.select_matches(matches, cx);
                    self.focus_editor(&FocusEditor, cx);
                }
            }
        }
    }

    pub fn select_match(&mut self, direction: Direction, count: usize, cx: &mut ViewContext<Self>) {
        if let Some(index) = self.active_match_index {
            if let Some(searchable_item) = self.active_searchable_item.as_ref() {
                if let Some(matches) = self
                    .searchable_items_with_matches
                    .get(&searchable_item.downgrade())
                {
                    let new_match_index = searchable_item
                        .match_index_for_direction(matches, index, direction, count, cx);

                    searchable_item.update_matches(matches, cx);
                    searchable_item.activate_match(new_match_index, matches, cx);
                }
            }
        }
    }

    pub fn select_last_match(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(searchable_item) = self.active_searchable_item.as_ref() {
            if let Some(matches) = self
                .searchable_items_with_matches
                .get(&searchable_item.downgrade())
            {
                if matches.len() == 0 {
                    return;
                }
                let new_match_index = matches.len() - 1;
                searchable_item.update_matches(matches, cx);
                searchable_item.activate_match(new_match_index, matches, cx);
            }
        }
    }

    fn on_query_editor_event(
        &mut self,
        _: View<Editor>,
        event: &editor::EditorEvent,
        cx: &mut ViewContext<Self>,
    ) {
        if let editor::EditorEvent::Edited { .. } = event {
            self.query_contains_error = false;
            self.clear_matches(cx);
            let search = self.update_matches(cx);
            cx.spawn(|this, mut cx| async move {
                search.await?;
                this.update(&mut cx, |this, cx| this.activate_current_match(cx))
            })
            .detach_and_log_err(cx);
        }
    }

    fn on_active_searchable_item_event(&mut self, event: &SearchEvent, cx: &mut ViewContext<Self>) {
        match event {
            SearchEvent::MatchesInvalidated => {
                let _ = self.update_matches(cx);
            }
            SearchEvent::ActiveMatchChanged => self.update_match_index(cx),
        }
    }

    fn toggle_case_sensitive(&mut self, _: &ToggleCaseSensitive, cx: &mut ViewContext<Self>) {
        self.toggle_search_option(SearchOptions::CASE_SENSITIVE, cx)
    }
    fn toggle_whole_word(&mut self, _: &ToggleWholeWord, cx: &mut ViewContext<Self>) {
        self.toggle_search_option(SearchOptions::WHOLE_WORD, cx)
    }
    fn clear_matches(&mut self, cx: &mut ViewContext<Self>) {
        let mut active_item_matches = None;
        for (searchable_item, matches) in self.searchable_items_with_matches.drain() {
            if let Some(searchable_item) =
                WeakSearchableItemHandle::upgrade(searchable_item.as_ref(), cx)
            {
                if Some(&searchable_item) == self.active_searchable_item.as_ref() {
                    active_item_matches = Some((searchable_item.downgrade(), matches));
                } else {
                    searchable_item.clear_matches(cx);
                }
            }
        }

        self.searchable_items_with_matches
            .extend(active_item_matches);
    }

    fn update_matches(&mut self, cx: &mut ViewContext<Self>) -> oneshot::Receiver<()> {
        let (done_tx, done_rx) = oneshot::channel();
        let query = self.query(cx);
        self.pending_search.take();

        if let Some(active_searchable_item) = self.active_searchable_item.as_ref() {
            if query.is_empty() {
                self.active_match_index.take();
                active_searchable_item.clear_matches(cx);
                let _ = done_tx.send(());
                cx.notify();
            } else {
                let query: Arc<_> = if self.current_mode == SearchMode::Regex {
                    match SearchQuery::regex(
                        query,
                        self.search_options.contains(SearchOptions::WHOLE_WORD),
                        self.search_options.contains(SearchOptions::CASE_SENSITIVE),
                        false,
                        Vec::new(),
                        Vec::new(),
                    ) {
                        Ok(query) => query.with_replacement(self.replacement(cx)),
                        Err(_) => {
                            self.query_contains_error = true;
                            cx.notify();
                            return done_rx;
                        }
                    }
                } else {
                    match SearchQuery::text(
                        query,
                        self.search_options.contains(SearchOptions::WHOLE_WORD),
                        self.search_options.contains(SearchOptions::CASE_SENSITIVE),
                        false,
                        Vec::new(),
                        Vec::new(),
                    ) {
                        Ok(query) => query.with_replacement(self.replacement(cx)),
                        Err(_) => {
                            self.query_contains_error = true;
                            cx.notify();
                            return done_rx;
                        }
                    }
                }
                .into();
                self.active_search = Some(query.clone());
                let query_text = query.as_str().to_string();

                let matches = active_searchable_item.find_matches(query, cx);

                let active_searchable_item = active_searchable_item.downgrade();
                self.pending_search = Some(cx.spawn(|this, mut cx| async move {
                    let matches = matches.await;

                    this.update(&mut cx, |this, cx| {
                        if let Some(active_searchable_item) =
                            WeakSearchableItemHandle::upgrade(active_searchable_item.as_ref(), cx)
                        {
                            this.searchable_items_with_matches
                                .insert(active_searchable_item.downgrade(), matches);

                            this.update_match_index(cx);
                            this.search_history.add(query_text);
                            if !this.dismissed {
                                let matches = this
                                    .searchable_items_with_matches
                                    .get(&active_searchable_item.downgrade())
                                    .unwrap();
                                active_searchable_item.update_matches(matches, cx);
                                let _ = done_tx.send(());
                            }
                            cx.notify();
                        }
                    })
                    .log_err();
                }));
            }
        }
        done_rx
    }

    fn update_match_index(&mut self, cx: &mut ViewContext<Self>) {
        let new_index = self
            .active_searchable_item
            .as_ref()
            .and_then(|searchable_item| {
                let matches = self
                    .searchable_items_with_matches
                    .get(&searchable_item.downgrade())?;
                searchable_item.active_match_index(matches, cx)
            });
        if new_index != self.active_match_index {
            self.active_match_index = new_index;
            cx.notify();
        }
    }

    fn next_history_query(&mut self, _: &NextHistoryQuery, cx: &mut ViewContext<Self>) {
        if let Some(new_query) = self.search_history.next().map(str::to_string) {
            let _ = self.search(&new_query, Some(self.search_options), cx);
        } else {
            self.search_history.reset_selection();
            let _ = self.search("", Some(self.search_options), cx);
        }
    }

    fn previous_history_query(&mut self, _: &PreviousHistoryQuery, cx: &mut ViewContext<Self>) {
        if self.query(cx).is_empty() {
            if let Some(new_query) = self.search_history.current().map(str::to_string) {
                let _ = self.search(&new_query, Some(self.search_options), cx);
                return;
            }
        }

        if let Some(new_query) = self.search_history.previous().map(str::to_string) {
            let _ = self.search(&new_query, Some(self.search_options), cx);
        }
    }
    fn cycle_mode(&mut self, _: &CycleMode, cx: &mut ViewContext<Self>) {
        self.activate_search_mode(next_mode(&self.current_mode, false), cx);
    }
    fn toggle_replace(&mut self, _: &ToggleReplace, cx: &mut ViewContext<Self>) {
        if let Some(_) = &self.active_searchable_item {
            self.replace_enabled = !self.replace_enabled;
            if !self.replace_enabled {
                let handle = self.query_editor.focus_handle(cx);
                cx.focus(&handle);
            }
            cx.notify();
        }
    }
    fn replace_next(&mut self, _: &ReplaceNext, cx: &mut ViewContext<Self>) {
        let mut should_propagate = true;
        if !self.dismissed && self.active_search.is_some() {
            if let Some(searchable_item) = self.active_searchable_item.as_ref() {
                if let Some(query) = self.active_search.as_ref() {
                    if let Some(matches) = self
                        .searchable_items_with_matches
                        .get(&searchable_item.downgrade())
                    {
                        if let Some(active_index) = self.active_match_index {
                            let query = query
                                .as_ref()
                                .clone()
                                .with_replacement(self.replacement(cx));
                            searchable_item.replace(&matches[active_index], &query, cx);
                            self.select_next_match(&SelectNextMatch, cx);
                        }
                        should_propagate = false;
                        self.focus_editor(&FocusEditor, cx);
                    }
                }
            }
        }
        if !should_propagate {
            cx.stop_propagation();
        }
    }
    pub fn replace_all(&mut self, _: &ReplaceAll, cx: &mut ViewContext<Self>) {
        if !self.dismissed && self.active_search.is_some() {
            if let Some(searchable_item) = self.active_searchable_item.as_ref() {
                if let Some(query) = self.active_search.as_ref() {
                    if let Some(matches) = self
                        .searchable_items_with_matches
                        .get(&searchable_item.downgrade())
                    {
                        let query = query
                            .as_ref()
                            .clone()
                            .with_replacement(self.replacement(cx));
                        for m in matches {
                            searchable_item.replace(m, &query, cx);
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use super::*;
    use editor::{DisplayPoint, Editor};
    use gpui::{Context, EmptyView, Hsla, TestAppContext, VisualTestContext};
    use language::Buffer;
    use smol::stream::StreamExt as _;
    use unindent::Unindent as _;

    fn init_globals(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
            editor::init(cx);

            language::init(cx);
            theme::init(theme::LoadThemes::JustBase, cx);
        });
    }
    fn init_test(
        cx: &mut TestAppContext,
    ) -> (
        View<Editor>,
        View<BufferSearchBar>,
        &mut VisualTestContext<'_>,
    ) {
        init_globals(cx);
        let buffer = cx.build_model(|cx| {
            Buffer::new(
                0,
                cx.entity_id().as_u64(),
                r#"
                A regular expression (shortened as regex or regexp;[1] also referred to as
                rational expression[2][3]) is a sequence of characters that specifies a search
                pattern in text. Usually such patterns are used by string-searching algorithms
                for "find" or "find and replace" operations on strings, or for input validation.
                "#
                .unindent(),
            )
        });
        let (_, cx) = cx.add_window_view(|_| EmptyView {});
        let editor = cx.build_view(|cx| Editor::for_buffer(buffer.clone(), None, cx));

        let search_bar = cx.build_view(|cx| {
            let mut search_bar = BufferSearchBar::new(cx);
            search_bar.set_active_pane_item(Some(&editor), cx);
            search_bar.show(cx);
            search_bar
        });

        (editor, search_bar, cx)
    }

    #[gpui::test]
    async fn test_search_simple(cx: &mut TestAppContext) {
        let (editor, search_bar, cx) = init_test(cx);
        // todo! osiewicz: these tests asserted on background color as well, that should be brought back.
        let display_points_of = |background_highlights: Vec<(Range<DisplayPoint>, Hsla)>| {
            background_highlights
                .into_iter()
                .map(|(range, _)| range)
                .collect::<Vec<_>>()
        };
        // Search for a string that appears with different casing.
        // By default, search is case-insensitive.
        search_bar
            .update(cx, |search_bar, cx| search_bar.search("us", None, cx))
            .await
            .unwrap();
        editor.update(cx, |editor, cx| {
            assert_eq!(
                display_points_of(editor.all_text_background_highlights(cx)),
                &[
                    DisplayPoint::new(2, 17)..DisplayPoint::new(2, 19),
                    DisplayPoint::new(2, 43)..DisplayPoint::new(2, 45),
                ]
            );
        });

        // Switch to a case sensitive search.
        search_bar.update(cx, |search_bar, cx| {
            search_bar.toggle_search_option(SearchOptions::CASE_SENSITIVE, cx);
        });
        let mut editor_notifications = cx.notifications(&editor);
        editor_notifications.next().await;
        editor.update(cx, |editor, cx| {
            assert_eq!(
                display_points_of(editor.all_text_background_highlights(cx)),
                &[DisplayPoint::new(2, 43)..DisplayPoint::new(2, 45),]
            );
        });

        // Search for a string that appears both as a whole word and
        // within other words. By default, all results are found.
        search_bar
            .update(cx, |search_bar, cx| search_bar.search("or", None, cx))
            .await
            .unwrap();
        editor.update(cx, |editor, cx| {
            assert_eq!(
                display_points_of(editor.all_text_background_highlights(cx)),
                &[
                    DisplayPoint::new(0, 24)..DisplayPoint::new(0, 26),
                    DisplayPoint::new(0, 41)..DisplayPoint::new(0, 43),
                    DisplayPoint::new(2, 71)..DisplayPoint::new(2, 73),
                    DisplayPoint::new(3, 1)..DisplayPoint::new(3, 3),
                    DisplayPoint::new(3, 11)..DisplayPoint::new(3, 13),
                    DisplayPoint::new(3, 56)..DisplayPoint::new(3, 58),
                    DisplayPoint::new(3, 60)..DisplayPoint::new(3, 62),
                ]
            );
        });

        // Switch to a whole word search.
        search_bar.update(cx, |search_bar, cx| {
            search_bar.toggle_search_option(SearchOptions::WHOLE_WORD, cx);
        });
        let mut editor_notifications = cx.notifications(&editor);
        editor_notifications.next().await;
        editor.update(cx, |editor, cx| {
            assert_eq!(
                display_points_of(editor.all_text_background_highlights(cx)),
                &[
                    DisplayPoint::new(0, 41)..DisplayPoint::new(0, 43),
                    DisplayPoint::new(3, 11)..DisplayPoint::new(3, 13),
                    DisplayPoint::new(3, 56)..DisplayPoint::new(3, 58),
                ]
            );
        });

        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)])
            });
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.active_match_index, Some(0));
            search_bar.select_next_match(&SelectNextMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(0, 41)..DisplayPoint::new(0, 43)]
            );
        });
        search_bar.update(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_next_match(&SelectNextMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(3, 11)..DisplayPoint::new(3, 13)]
            );
        });
        search_bar.update(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(1));
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_next_match(&SelectNextMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(3, 56)..DisplayPoint::new(3, 58)]
            );
        });
        search_bar.update(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(2));
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_next_match(&SelectNextMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(0, 41)..DisplayPoint::new(0, 43)]
            );
        });
        search_bar.update(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_prev_match(&SelectPrevMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(3, 56)..DisplayPoint::new(3, 58)]
            );
        });
        search_bar.update(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(2));
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_prev_match(&SelectPrevMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(3, 11)..DisplayPoint::new(3, 13)]
            );
        });
        search_bar.update(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(1));
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_prev_match(&SelectPrevMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(0, 41)..DisplayPoint::new(0, 43)]
            );
        });
        search_bar.update(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        // Park the cursor in between matches and ensure that going to the previous match selects
        // the closest match to the left.
        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)])
            });
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.active_match_index, Some(1));
            search_bar.select_prev_match(&SelectPrevMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(0, 41)..DisplayPoint::new(0, 43)]
            );
        });
        search_bar.update(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        // Park the cursor in between matches and ensure that going to the next match selects the
        // closest match to the right.
        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)])
            });
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.active_match_index, Some(1));
            search_bar.select_next_match(&SelectNextMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(3, 11)..DisplayPoint::new(3, 13)]
            );
        });
        search_bar.update(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(1));
        });

        // Park the cursor after the last match and ensure that going to the previous match selects
        // the last match.
        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(3, 60)..DisplayPoint::new(3, 60)])
            });
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.active_match_index, Some(2));
            search_bar.select_prev_match(&SelectPrevMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(3, 56)..DisplayPoint::new(3, 58)]
            );
        });
        search_bar.update(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(2));
        });

        // Park the cursor after the last match and ensure that going to the next match selects the
        // first match.
        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(3, 60)..DisplayPoint::new(3, 60)])
            });
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.active_match_index, Some(2));
            search_bar.select_next_match(&SelectNextMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(0, 41)..DisplayPoint::new(0, 43)]
            );
        });
        search_bar.update(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        // Park the cursor before the first match and ensure that going to the previous match
        // selects the last match.
        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)])
            });
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.active_match_index, Some(0));
            search_bar.select_prev_match(&SelectPrevMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(3, 56)..DisplayPoint::new(3, 58)]
            );
        });
        search_bar.update(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(2));
        });
    }

    #[gpui::test]
    async fn test_search_option_handling(cx: &mut TestAppContext) {
        let (editor, search_bar, cx) = init_test(cx);

        // show with options should make current search case sensitive
        search_bar
            .update(cx, |search_bar, cx| {
                search_bar.show(cx);
                search_bar.search("us", Some(SearchOptions::CASE_SENSITIVE), cx)
            })
            .await
            .unwrap();
        // todo! osiewicz: these tests previously asserted on background color highlights; that should be introduced back.
        let display_points_of = |background_highlights: Vec<(Range<DisplayPoint>, Hsla)>| {
            background_highlights
                .into_iter()
                .map(|(range, _)| range)
                .collect::<Vec<_>>()
        };
        editor.update(cx, |editor, cx| {
            assert_eq!(
                display_points_of(editor.all_text_background_highlights(cx)),
                &[DisplayPoint::new(2, 43)..DisplayPoint::new(2, 45),]
            );
        });

        // search_suggested should restore default options
        search_bar.update(cx, |search_bar, cx| {
            search_bar.search_suggested(cx);
            assert_eq!(search_bar.search_options, SearchOptions::NONE)
        });

        // toggling a search option should update the defaults
        search_bar
            .update(cx, |search_bar, cx| {
                search_bar.search("regex", Some(SearchOptions::CASE_SENSITIVE), cx)
            })
            .await
            .unwrap();
        search_bar.update(cx, |search_bar, cx| {
            search_bar.toggle_search_option(SearchOptions::WHOLE_WORD, cx)
        });
        let mut editor_notifications = cx.notifications(&editor);
        editor_notifications.next().await;
        editor.update(cx, |editor, cx| {
            assert_eq!(
                display_points_of(editor.all_text_background_highlights(cx)),
                &[DisplayPoint::new(0, 35)..DisplayPoint::new(0, 40),]
            );
        });

        // defaults should still include whole word
        search_bar.update(cx, |search_bar, cx| {
            search_bar.search_suggested(cx);
            assert_eq!(
                search_bar.search_options,
                SearchOptions::CASE_SENSITIVE | SearchOptions::WHOLE_WORD
            )
        });
    }

    #[gpui::test]
    async fn test_search_select_all_matches(cx: &mut TestAppContext) {
        init_globals(cx);
        let buffer_text = r#"
        A regular expression (shortened as regex or regexp;[1] also referred to as
        rational expression[2][3]) is a sequence of characters that specifies a search
        pattern in text. Usually such patterns are used by string-searching algorithms
        for "find" or "find and replace" operations on strings, or for input validation.
        "#
        .unindent();
        let expected_query_matches_count = buffer_text
            .chars()
            .filter(|c| c.to_ascii_lowercase() == 'a')
            .count();
        assert!(
            expected_query_matches_count > 1,
            "Should pick a query with multiple results"
        );
        let buffer = cx.build_model(|cx| Buffer::new(0, cx.entity_id().as_u64(), buffer_text));
        let window = cx.add_window(|_| EmptyView {});

        let editor = window.build_view(cx, |cx| Editor::for_buffer(buffer.clone(), None, cx));

        let search_bar = window.build_view(cx, |cx| {
            let mut search_bar = BufferSearchBar::new(cx);
            search_bar.set_active_pane_item(Some(&editor), cx);
            search_bar.show(cx);
            search_bar
        });

        window
            .update(cx, |_, cx| {
                search_bar.update(cx, |search_bar, cx| search_bar.search("a", None, cx))
            })
            .unwrap()
            .await
            .unwrap();
        let initial_selections = window
            .update(cx, |_, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    let handle = search_bar.query_editor.focus_handle(cx);
                    cx.focus(&handle);
                    search_bar.activate_current_match(cx);
                });
                assert!(
                    !editor.read(cx).is_focused(cx),
                    "Initially, the editor should not be focused"
                );
                let initial_selections = editor.update(cx, |editor, cx| {
                    let initial_selections = editor.selections.display_ranges(cx);
                    assert_eq!(
                        initial_selections.len(), 1,
                        "Expected to have only one selection before adding carets to all matches, but got: {initial_selections:?}",
                    );
                    initial_selections
                });
                search_bar.update(cx, |search_bar, cx| {
                    assert_eq!(search_bar.active_match_index, Some(0));
                    let handle = search_bar.query_editor.focus_handle(cx);
                    cx.focus(&handle);
                    search_bar.select_all_matches(&SelectAllMatches, cx);
                });
                assert!(
                    editor.read(cx).is_focused(cx),
                    "Should focus editor after successful SelectAllMatches"
                );
                search_bar.update(cx, |search_bar, cx| {
                    let all_selections =
                        editor.update(cx, |editor, cx| editor.selections.display_ranges(cx));
                    assert_eq!(
                        all_selections.len(),
                        expected_query_matches_count,
                        "Should select all `a` characters in the buffer, but got: {all_selections:?}"
                    );
                    assert_eq!(
                        search_bar.active_match_index,
                        Some(0),
                        "Match index should not change after selecting all matches"
                    );
                });

                search_bar.update(cx, |this, cx| this.select_next_match(&SelectNextMatch, cx));
                initial_selections
            }).unwrap();

        window
            .update(cx, |_, cx| {
                assert!(
                    editor.read(cx).is_focused(cx),
                    "Should still have editor focused after SelectNextMatch"
                );
                search_bar.update(cx, |search_bar, cx| {
                    let all_selections =
                        editor.update(cx, |editor, cx| editor.selections.display_ranges(cx));
                    assert_eq!(
                        all_selections.len(),
                        1,
                        "On next match, should deselect items and select the next match"
                    );
                    assert_ne!(
                        all_selections, initial_selections,
                        "Next match should be different from the first selection"
                    );
                    assert_eq!(
                        search_bar.active_match_index,
                        Some(1),
                        "Match index should be updated to the next one"
                    );
                    let handle = search_bar.query_editor.focus_handle(cx);
                    cx.focus(&handle);
                    search_bar.select_all_matches(&SelectAllMatches, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, cx| {
                assert!(
                    editor.read(cx).is_focused(cx),
                    "Should focus editor after successful SelectAllMatches"
                );
                search_bar.update(cx, |search_bar, cx| {
                    let all_selections =
                        editor.update(cx, |editor, cx| editor.selections.display_ranges(cx));
                    assert_eq!(
                    all_selections.len(),
                    expected_query_matches_count,
                    "Should select all `a` characters in the buffer, but got: {all_selections:?}"
                );
                    assert_eq!(
                        search_bar.active_match_index,
                        Some(1),
                        "Match index should not change after selecting all matches"
                    );
                });
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.select_prev_match(&SelectPrevMatch, cx);
                });
            })
            .unwrap();
        let last_match_selections = window
            .update(cx, |_, cx| {
                assert!(
                    editor.read(cx).is_focused(&cx),
                    "Should still have editor focused after SelectPrevMatch"
                );

                search_bar.update(cx, |search_bar, cx| {
                    let all_selections =
                        editor.update(cx, |editor, cx| editor.selections.display_ranges(cx));
                    assert_eq!(
                        all_selections.len(),
                        1,
                        "On previous match, should deselect items and select the previous item"
                    );
                    assert_eq!(
                        all_selections, initial_selections,
                        "Previous match should be the same as the first selection"
                    );
                    assert_eq!(
                        search_bar.active_match_index,
                        Some(0),
                        "Match index should be updated to the previous one"
                    );
                    all_selections
                })
            })
            .unwrap();

        window
            .update(cx, |_, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    let handle = search_bar.query_editor.focus_handle(cx);
                    cx.focus(&handle);
                    search_bar.search("abas_nonexistent_match", None, cx)
                })
            })
            .unwrap()
            .await
            .unwrap();
        window
            .update(cx, |_, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.select_all_matches(&SelectAllMatches, cx);
                });
                assert!(
                editor.update(cx, |this, cx| !this.is_focused(cx.window_context())),
                "Should not switch focus to editor if SelectAllMatches does not find any matches"
            );
                search_bar.update(cx, |search_bar, cx| {
                    let all_selections =
                        editor.update(cx, |editor, cx| editor.selections.display_ranges(cx));
                    assert_eq!(
                        all_selections, last_match_selections,
                        "Should not select anything new if there are no matches"
                    );
                    assert!(
                        search_bar.active_match_index.is_none(),
                        "For no matches, there should be no active match index"
                    );
                });
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_search_query_history(cx: &mut TestAppContext) {
        //crate::project_search::tests::init_test(cx);
        init_globals(cx);
        let buffer_text = r#"
        A regular expression (shortened as regex or regexp;[1] also referred to as
        rational expression[2][3]) is a sequence of characters that specifies a search
        pattern in text. Usually such patterns are used by string-searching algorithms
        for "find" or "find and replace" operations on strings, or for input validation.
        "#
        .unindent();
        let buffer = cx.build_model(|cx| Buffer::new(0, cx.entity_id().as_u64(), buffer_text));
        let (_, cx) = cx.add_window_view(|_| EmptyView {});

        let editor = cx.build_view(|cx| Editor::for_buffer(buffer.clone(), None, cx));

        let search_bar = cx.build_view(|cx| {
            let mut search_bar = BufferSearchBar::new(cx);
            search_bar.set_active_pane_item(Some(&editor), cx);
            search_bar.show(cx);
            search_bar
        });

        // Add 3 search items into the history.
        search_bar
            .update(cx, |search_bar, cx| search_bar.search("a", None, cx))
            .await
            .unwrap();
        search_bar
            .update(cx, |search_bar, cx| search_bar.search("b", None, cx))
            .await
            .unwrap();
        search_bar
            .update(cx, |search_bar, cx| {
                search_bar.search("c", Some(SearchOptions::CASE_SENSITIVE), cx)
            })
            .await
            .unwrap();
        // Ensure that the latest search is active.
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "c");
            assert_eq!(search_bar.search_options, SearchOptions::CASE_SENSITIVE);
        });

        // Next history query after the latest should set the query to the empty string.
        search_bar.update(cx, |search_bar, cx| {
            search_bar.next_history_query(&NextHistoryQuery, cx);
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "");
            assert_eq!(search_bar.search_options, SearchOptions::CASE_SENSITIVE);
        });
        search_bar.update(cx, |search_bar, cx| {
            search_bar.next_history_query(&NextHistoryQuery, cx);
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "");
            assert_eq!(search_bar.search_options, SearchOptions::CASE_SENSITIVE);
        });

        // First previous query for empty current query should set the query to the latest.
        search_bar.update(cx, |search_bar, cx| {
            search_bar.previous_history_query(&PreviousHistoryQuery, cx);
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "c");
            assert_eq!(search_bar.search_options, SearchOptions::CASE_SENSITIVE);
        });

        // Further previous items should go over the history in reverse order.
        search_bar.update(cx, |search_bar, cx| {
            search_bar.previous_history_query(&PreviousHistoryQuery, cx);
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "b");
            assert_eq!(search_bar.search_options, SearchOptions::CASE_SENSITIVE);
        });

        // Previous items should never go behind the first history item.
        search_bar.update(cx, |search_bar, cx| {
            search_bar.previous_history_query(&PreviousHistoryQuery, cx);
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "a");
            assert_eq!(search_bar.search_options, SearchOptions::CASE_SENSITIVE);
        });
        search_bar.update(cx, |search_bar, cx| {
            search_bar.previous_history_query(&PreviousHistoryQuery, cx);
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "a");
            assert_eq!(search_bar.search_options, SearchOptions::CASE_SENSITIVE);
        });

        // Next items should go over the history in the original order.
        search_bar.update(cx, |search_bar, cx| {
            search_bar.next_history_query(&NextHistoryQuery, cx);
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "b");
            assert_eq!(search_bar.search_options, SearchOptions::CASE_SENSITIVE);
        });

        search_bar
            .update(cx, |search_bar, cx| search_bar.search("ba", None, cx))
            .await
            .unwrap();
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "ba");
            assert_eq!(search_bar.search_options, SearchOptions::NONE);
        });

        // New search input should add another entry to history and move the selection to the end of the history.
        search_bar.update(cx, |search_bar, cx| {
            search_bar.previous_history_query(&PreviousHistoryQuery, cx);
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "c");
            assert_eq!(search_bar.search_options, SearchOptions::NONE);
        });
        search_bar.update(cx, |search_bar, cx| {
            search_bar.previous_history_query(&PreviousHistoryQuery, cx);
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "b");
            assert_eq!(search_bar.search_options, SearchOptions::NONE);
        });
        search_bar.update(cx, |search_bar, cx| {
            search_bar.next_history_query(&NextHistoryQuery, cx);
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "c");
            assert_eq!(search_bar.search_options, SearchOptions::NONE);
        });
        search_bar.update(cx, |search_bar, cx| {
            search_bar.next_history_query(&NextHistoryQuery, cx);
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "ba");
            assert_eq!(search_bar.search_options, SearchOptions::NONE);
        });
        search_bar.update(cx, |search_bar, cx| {
            search_bar.next_history_query(&NextHistoryQuery, cx);
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "");
            assert_eq!(search_bar.search_options, SearchOptions::NONE);
        });
    }
    #[gpui::test]
    async fn test_replace_simple(cx: &mut TestAppContext) {
        let (editor, search_bar, cx) = init_test(cx);

        search_bar
            .update(cx, |search_bar, cx| {
                search_bar.search("expression", None, cx)
            })
            .await
            .unwrap();

        search_bar.update(cx, |search_bar, cx| {
            search_bar.replacement_editor.update(cx, |editor, cx| {
                // We use $1 here as initially we should be in Text mode, where `$1` should be treated literally.
                editor.set_text("expr$1", cx);
            });
            search_bar.replace_all(&ReplaceAll, cx)
        });
        assert_eq!(
            editor.update(cx, |this, cx| { this.text(cx) }),
            r#"
        A regular expr$1 (shortened as regex or regexp;[1] also referred to as
        rational expr$1[2][3]) is a sequence of characters that specifies a search
        pattern in text. Usually such patterns are used by string-searching algorithms
        for "find" or "find and replace" operations on strings, or for input validation.
        "#
            .unindent()
        );

        // Search for word boundaries and replace just a single one.
        search_bar
            .update(cx, |search_bar, cx| {
                search_bar.search("or", Some(SearchOptions::WHOLE_WORD), cx)
            })
            .await
            .unwrap();

        search_bar.update(cx, |search_bar, cx| {
            search_bar.replacement_editor.update(cx, |editor, cx| {
                editor.set_text("banana", cx);
            });
            search_bar.replace_next(&ReplaceNext, cx)
        });
        // Notice how the first or in the text (shORtened) is not replaced. Neither are the remaining hits of `or` in the text.
        assert_eq!(
            editor.update(cx, |this, cx| { this.text(cx) }),
            r#"
        A regular expr$1 (shortened as regex banana regexp;[1] also referred to as
        rational expr$1[2][3]) is a sequence of characters that specifies a search
        pattern in text. Usually such patterns are used by string-searching algorithms
        for "find" or "find and replace" operations on strings, or for input validation.
        "#
            .unindent()
        );
        // Let's turn on regex mode.
        search_bar
            .update(cx, |search_bar, cx| {
                search_bar.activate_search_mode(SearchMode::Regex, cx);
                search_bar.search("\\[([^\\]]+)\\]", None, cx)
            })
            .await
            .unwrap();
        search_bar.update(cx, |search_bar, cx| {
            search_bar.replacement_editor.update(cx, |editor, cx| {
                editor.set_text("${1}number", cx);
            });
            search_bar.replace_all(&ReplaceAll, cx)
        });
        assert_eq!(
            editor.update(cx, |this, cx| { this.text(cx) }),
            r#"
        A regular expr$1 (shortened as regex banana regexp;1number also referred to as
        rational expr$12number3number) is a sequence of characters that specifies a search
        pattern in text. Usually such patterns are used by string-searching algorithms
        for "find" or "find and replace" operations on strings, or for input validation.
        "#
            .unindent()
        );
        // Now with a whole-word twist.
        search_bar
            .update(cx, |search_bar, cx| {
                search_bar.activate_search_mode(SearchMode::Regex, cx);
                search_bar.search("a\\w+s", Some(SearchOptions::WHOLE_WORD), cx)
            })
            .await
            .unwrap();
        search_bar.update(cx, |search_bar, cx| {
            search_bar.replacement_editor.update(cx, |editor, cx| {
                editor.set_text("things", cx);
            });
            search_bar.replace_all(&ReplaceAll, cx)
        });
        // The only word affected by this edit should be `algorithms`, even though there's a bunch
        // of words in this text that would match this regex if not for WHOLE_WORD.
        assert_eq!(
            editor.update(cx, |this, cx| { this.text(cx) }),
            r#"
        A regular expr$1 (shortened as regex banana regexp;1number also referred to as
        rational expr$12number3number) is a sequence of characters that specifies a search
        pattern in text. Usually such patterns are used by string-searching things
        for "find" or "find and replace" operations on strings, or for input validation.
        "#
            .unindent()
        );
    }
}
