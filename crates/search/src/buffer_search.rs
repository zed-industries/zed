mod registrar;

use crate::{
    search_bar::render_nav_button, FocusSearch, NextHistoryQuery, PreviousHistoryQuery, ReplaceAll,
    ReplaceNext, SearchOptions, SelectAllMatches, SelectNextMatch, SelectPrevMatch,
    ToggleCaseSensitive, ToggleRegex, ToggleReplace, ToggleSelection, ToggleWholeWord,
};
use any_vec::AnyVec;
use collections::HashMap;
use editor::{
    actions::{Tab, TabPrev},
    DisplayPoint, Editor, EditorElement, EditorSettings, EditorStyle,
};
use futures::channel::oneshot;
use gpui::{
    actions, div, impl_actions, Action, AppContext, ClickEvent, EventEmitter, FocusHandle,
    FocusableView, Hsla, InteractiveElement as _, IntoElement, KeyContext, ParentElement as _,
    Render, ScrollHandle, Styled, Subscription, Task, TextStyle, View, ViewContext,
    VisualContext as _, WindowContext,
};
use project::{
    search::SearchQuery,
    search_history::{SearchHistory, SearchHistoryCursor},
};
use serde::Deserialize;
use settings::Settings;
use std::sync::Arc;
use theme::ThemeSettings;

use ui::{h_flex, prelude::*, IconButton, IconButtonShape, IconName, Tooltip, BASE_REM_SIZE_IN_PX};
use util::ResultExt;
use workspace::{
    item::ItemHandle,
    searchable::{Direction, SearchEvent, SearchableItemHandle, WeakSearchableItemHandle},
    ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace,
};

pub use registrar::DivRegistrar;
use registrar::{ForDeployed, ForDismissed, SearchActionsRegistrar, WithResults};

const MIN_INPUT_WIDTH_REMS: f32 = 10.;
const MAX_INPUT_WIDTH_REMS: f32 = 30.;
const MAX_BUFFER_SEARCH_HISTORY_SIZE: usize = 50;

#[derive(PartialEq, Clone, Deserialize)]
pub struct Deploy {
    #[serde(default = "util::serde::default_true")]
    pub focus: bool,
    #[serde(default)]
    pub replace_enabled: bool,
    #[serde(default)]
    pub selection_search_enabled: bool,
}

impl_actions!(buffer_search, [Deploy]);

actions!(buffer_search, [Dismiss, FocusEditor]);

impl Deploy {
    pub fn find() -> Self {
        Self {
            focus: true,
            replace_enabled: false,
            selection_search_enabled: false,
        }
    }
}

pub enum Event {
    UpdateLocation,
}

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, _| BufferSearchBar::register(workspace))
        .detach();
}

pub struct BufferSearchBar {
    query_editor: View<Editor>,
    query_editor_focused: bool,
    replacement_editor: View<Editor>,
    replacement_editor_focused: bool,
    active_searchable_item: Option<Box<dyn SearchableItemHandle>>,
    active_match_index: Option<usize>,
    active_searchable_item_subscription: Option<Subscription>,
    active_search: Option<Arc<SearchQuery>>,
    searchable_items_with_matches: HashMap<Box<dyn WeakSearchableItemHandle>, AnyVec<dyn Send>>,
    pending_search: Option<Task<()>>,
    search_options: SearchOptions,
    default_options: SearchOptions,
    configured_options: SearchOptions,
    query_contains_error: bool,
    dismissed: bool,
    search_history: SearchHistory,
    search_history_cursor: SearchHistoryCursor,
    replace_enabled: bool,
    selection_search_enabled: bool,
    scroll_handle: ScrollHandle,
    editor_scroll_handle: ScrollHandle,
    editor_needed_width: Pixels,
}

impl BufferSearchBar {
    fn render_text_input(
        &self,
        editor: &View<Editor>,
        color: Hsla,
        cx: &ViewContext<Self>,
    ) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: if editor.read(cx).read_only(cx) {
                cx.theme().colors().text_disabled
            } else {
                color
            },
            font_family: settings.buffer_font.family.clone(),
            font_features: settings.buffer_font.features.clone(),
            font_fallbacks: settings.buffer_font.fallbacks.clone(),
            font_size: rems(0.875).into(),
            font_weight: settings.buffer_font.weight,
            line_height: relative(1.3),
            ..Default::default()
        };

        EditorElement::new(
            editor,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }

    pub fn query_editor_focused(&self) -> bool {
        self.query_editor_focused
    }
}

impl EventEmitter<Event> for BufferSearchBar {}
impl EventEmitter<workspace::ToolbarItemEvent> for BufferSearchBar {}
impl Render for BufferSearchBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        if self.dismissed {
            return div().id("search_bar");
        }

        let focus_handle = self.focus_handle(cx);

        let narrow_mode =
            self.scroll_handle.bounds().size.width / cx.rem_size() < 340. / BASE_REM_SIZE_IN_PX;
        let hide_inline_icons = self.editor_needed_width
            > self.editor_scroll_handle.bounds().size.width - cx.rem_size() * 6.;

        let supported_options = self.supported_options();

        if self.query_editor.update(cx, |query_editor, cx| {
            query_editor.placeholder_text(cx).is_none()
        }) {
            self.query_editor.update(cx, |editor, cx| {
                editor.set_placeholder_text("Search", cx);
            });
        }

        self.replacement_editor.update(cx, |editor, cx| {
            editor.set_placeholder_text("Replace with...", cx);
        });

        let mut text_color = Color::Default;
        let match_text = self
            .active_searchable_item
            .as_ref()
            .and_then(|searchable_item| {
                if self.query(cx).is_empty() {
                    return None;
                }
                let matches_count = self
                    .searchable_items_with_matches
                    .get(&searchable_item.downgrade())
                    .map(AnyVec::len)
                    .unwrap_or(0);
                if let Some(match_ix) = self.active_match_index {
                    Some(format!("{}/{}", match_ix + 1, matches_count))
                } else {
                    text_color = Color::Error; // No matches found
                    None
                }
            })
            .unwrap_or_else(|| "0/0".to_string());
        let should_show_replace_input = self.replace_enabled && supported_options.replacement;
        let in_replace = self.replacement_editor.focus_handle(cx).is_focused(cx);

        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("BufferSearchBar");
        if in_replace {
            key_context.add("in_replace");
        }
        let editor_border = if self.query_contains_error {
            Color::Error.color(cx)
        } else {
            cx.theme().colors().border
        };

        let search_line = h_flex()
            .gap_2()
            .child(
                h_flex()
                    .id("editor-scroll")
                    .track_scroll(&self.editor_scroll_handle)
                    .flex_1()
                    .h_8()
                    .px_2()
                    .py_1()
                    .border_1()
                    .border_color(editor_border)
                    .min_w(rems(MIN_INPUT_WIDTH_REMS))
                    .max_w(rems(MAX_INPUT_WIDTH_REMS))
                    .rounded_lg()
                    .child(self.render_text_input(&self.query_editor, text_color.color(cx), cx))
                    .when(!hide_inline_icons, |div| {
                        div.children(supported_options.case.then(|| {
                            self.render_search_option_button(
                                SearchOptions::CASE_SENSITIVE,
                                focus_handle.clone(),
                                cx.listener(|this, _, cx| {
                                    this.toggle_case_sensitive(&ToggleCaseSensitive, cx)
                                }),
                            )
                        }))
                        .children(supported_options.word.then(|| {
                            self.render_search_option_button(
                                SearchOptions::WHOLE_WORD,
                                focus_handle.clone(),
                                cx.listener(|this, _, cx| {
                                    this.toggle_whole_word(&ToggleWholeWord, cx)
                                }),
                            )
                        }))
                        .children(supported_options.regex.then(|| {
                            self.render_search_option_button(
                                SearchOptions::REGEX,
                                focus_handle.clone(),
                                cx.listener(|this, _, cx| this.toggle_regex(&ToggleRegex, cx)),
                            )
                        }))
                    }),
            )
            .child(
                h_flex()
                    .flex_none()
                    .gap_0p5()
                    .when(supported_options.replacement, |this| {
                        this.child(
                            IconButton::new(
                                "buffer-search-bar-toggle-replace-button",
                                IconName::Replace,
                            )
                            .style(ButtonStyle::Subtle)
                            .shape(IconButtonShape::Square)
                            .when(self.replace_enabled, |button| {
                                button.style(ButtonStyle::Filled)
                            })
                            .on_click(cx.listener(|this, _: &ClickEvent, cx| {
                                this.toggle_replace(&ToggleReplace, cx);
                            }))
                            .selected(self.replace_enabled)
                            .tooltip({
                                let focus_handle = focus_handle.clone();
                                move |cx| {
                                    Tooltip::for_action_in(
                                        "Toggle Replace",
                                        &ToggleReplace,
                                        &focus_handle,
                                        cx,
                                    )
                                }
                            }),
                        )
                    })
                    .when(supported_options.selection, |this| {
                        this.child(
                            IconButton::new(
                                "buffer-search-bar-toggle-search-selection-button",
                                IconName::SearchSelection,
                            )
                            .style(ButtonStyle::Subtle)
                            .shape(IconButtonShape::Square)
                            .when(self.selection_search_enabled, |button| {
                                button.style(ButtonStyle::Filled)
                            })
                            .on_click(cx.listener(|this, _: &ClickEvent, cx| {
                                this.toggle_selection(&ToggleSelection, cx);
                            }))
                            .selected(self.selection_search_enabled)
                            .tooltip({
                                let focus_handle = focus_handle.clone();
                                move |cx| {
                                    Tooltip::for_action_in(
                                        "Toggle Search Selection",
                                        &ToggleSelection,
                                        &focus_handle,
                                        cx,
                                    )
                                }
                            }),
                        )
                    })
                    .child(
                        IconButton::new("select-all", ui::IconName::SelectAll)
                            .on_click(|_, cx| cx.dispatch_action(SelectAllMatches.boxed_clone()))
                            .shape(IconButtonShape::Square)
                            .tooltip({
                                let focus_handle = focus_handle.clone();
                                move |cx| {
                                    Tooltip::for_action_in(
                                        "Select All Matches",
                                        &SelectAllMatches,
                                        &focus_handle,
                                        cx,
                                    )
                                }
                            }),
                    )
                    .child(render_nav_button(
                        ui::IconName::ChevronLeft,
                        self.active_match_index.is_some(),
                        "Select Previous Match",
                        &SelectPrevMatch,
                        focus_handle.clone(),
                    ))
                    .child(render_nav_button(
                        ui::IconName::ChevronRight,
                        self.active_match_index.is_some(),
                        "Select Next Match",
                        &SelectNextMatch,
                        focus_handle.clone(),
                    ))
                    .when(!narrow_mode, |this| {
                        this.child(h_flex().ml_2().min_w(rems_from_px(40.)).child(
                            Label::new(match_text).size(LabelSize::Small).color(
                                if self.active_match_index.is_some() {
                                    Color::Default
                                } else {
                                    Color::Disabled
                                },
                            ),
                        ))
                    }),
            );

        let replace_line = should_show_replace_input.then(|| {
            h_flex()
                .gap_2()
                .flex_1()
                .child(
                    h_flex()
                        .flex_1()
                        // We're giving this a fixed height to match the height of the search input,
                        // which has an icon inside that is increasing its height.
                        .h_8()
                        .px_2()
                        .py_1()
                        .border_1()
                        .border_color(cx.theme().colors().border)
                        .rounded_lg()
                        .min_w(rems(MIN_INPUT_WIDTH_REMS))
                        .max_w(rems(MAX_INPUT_WIDTH_REMS))
                        .child(self.render_text_input(
                            &self.replacement_editor,
                            cx.theme().colors().text,
                            cx,
                        )),
                )
                .child(
                    h_flex()
                        .flex_none()
                        .gap_0p5()
                        .child(
                            IconButton::new("search-replace-next", ui::IconName::ReplaceNext)
                                .shape(IconButtonShape::Square)
                                .tooltip({
                                    let focus_handle = focus_handle.clone();
                                    move |cx| {
                                        Tooltip::for_action_in(
                                            "Replace Next Match",
                                            &ReplaceNext,
                                            &focus_handle,
                                            cx,
                                        )
                                    }
                                })
                                .on_click(
                                    cx.listener(|this, _, cx| this.replace_next(&ReplaceNext, cx)),
                                ),
                        )
                        .child(
                            IconButton::new("search-replace-all", ui::IconName::ReplaceAll)
                                .shape(IconButtonShape::Square)
                                .tooltip({
                                    let focus_handle = focus_handle.clone();
                                    move |cx| {
                                        Tooltip::for_action_in(
                                            "Replace All Matches",
                                            &ReplaceAll,
                                            &focus_handle,
                                            cx,
                                        )
                                    }
                                })
                                .on_click(
                                    cx.listener(|this, _, cx| this.replace_all(&ReplaceAll, cx)),
                                ),
                        ),
                )
        });

        v_flex()
            .id("buffer_search")
            .track_scroll(&self.scroll_handle)
            .key_context(key_context)
            .capture_action(cx.listener(Self::tab))
            .capture_action(cx.listener(Self::tab_prev))
            .on_action(cx.listener(Self::previous_history_query))
            .on_action(cx.listener(Self::next_history_query))
            .on_action(cx.listener(Self::dismiss))
            .on_action(cx.listener(Self::select_next_match))
            .on_action(cx.listener(Self::select_prev_match))
            .when(self.supported_options().replacement, |this| {
                this.on_action(cx.listener(Self::toggle_replace))
                    .when(in_replace, |this| {
                        this.on_action(cx.listener(Self::replace_next))
                            .on_action(cx.listener(Self::replace_all))
                    })
            })
            .when(self.supported_options().case, |this| {
                this.on_action(cx.listener(Self::toggle_case_sensitive))
            })
            .when(self.supported_options().word, |this| {
                this.on_action(cx.listener(Self::toggle_whole_word))
            })
            .when(self.supported_options().regex, |this| {
                this.on_action(cx.listener(Self::toggle_regex))
            })
            .when(self.supported_options().selection, |this| {
                this.on_action(cx.listener(Self::toggle_selection))
            })
            .gap_2()
            .child(
                h_flex()
                    .child(search_line.w_full())
                    .when(!narrow_mode, |div| {
                        div.child(
                            IconButton::new(SharedString::from("Close"), IconName::Close)
                                .shape(IconButtonShape::Square)
                                .tooltip(move |cx| {
                                    Tooltip::for_action("Close Search Bar", &Dismiss, cx)
                                })
                                .on_click(cx.listener(|this, _: &ClickEvent, cx| {
                                    this.dismiss(&Dismiss, cx)
                                })),
                        )
                    }),
            )
            .children(replace_line)
    }
}

impl FocusableView for BufferSearchBar {
    fn focus_handle(&self, cx: &AppContext) -> gpui::FocusHandle {
        self.query_editor.focus_handle(cx)
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

            self.active_searchable_item_subscription =
                Some(searchable_item_handle.subscribe_to_search_events(
                    cx,
                    Box::new(move |search_event, cx| {
                        if let Some(this) = this.upgrade() {
                            this.update(cx, |this, cx| {
                                this.on_active_searchable_item_event(search_event, cx)
                            });
                        }
                    }),
                ));

            self.active_searchable_item = Some(searchable_item_handle);
            drop(self.update_matches(true, cx));
            if !self.dismissed {
                return ToolbarItemLocation::Secondary;
            }
        }
        ToolbarItemLocation::Hidden
    }
}

impl BufferSearchBar {
    pub fn register(registrar: &mut impl SearchActionsRegistrar) {
        registrar.register_handler(ForDeployed(|this, _: &FocusSearch, cx| {
            this.query_editor.focus_handle(cx).focus(cx);
            this.select_query(cx);
        }));
        registrar.register_handler(ForDeployed(|this, action: &ToggleCaseSensitive, cx| {
            if this.supported_options().case {
                this.toggle_case_sensitive(action, cx);
            }
        }));
        registrar.register_handler(ForDeployed(|this, action: &ToggleWholeWord, cx| {
            if this.supported_options().word {
                this.toggle_whole_word(action, cx);
            }
        }));
        registrar.register_handler(ForDeployed(|this, action: &ToggleSelection, cx| {
            if this.supported_options().selection {
                this.toggle_selection(action, cx);
            }
        }));
        registrar.register_handler(ForDeployed(|this, action: &ToggleReplace, cx| {
            if this.supported_options().replacement {
                this.toggle_replace(action, cx);
            }
        }));
        registrar.register_handler(WithResults(|this, action: &SelectNextMatch, cx| {
            this.select_next_match(action, cx);
        }));
        registrar.register_handler(WithResults(|this, action: &SelectPrevMatch, cx| {
            this.select_prev_match(action, cx);
        }));
        registrar.register_handler(WithResults(|this, action: &SelectAllMatches, cx| {
            this.select_all_matches(action, cx);
        }));
        registrar.register_handler(ForDeployed(|this, _: &editor::actions::Cancel, cx| {
            this.dismiss(&Dismiss, cx);
        }));
        registrar.register_handler(ForDeployed(|this, _: &Dismiss, cx| {
            this.dismiss(&Dismiss, cx);
        }));

        // register deploy buffer search for both search bar states, since we want to focus into the search bar
        // when the deploy action is triggered in the buffer.
        registrar.register_handler(ForDeployed(|this, deploy, cx| {
            this.deploy(deploy, cx);
        }));
        registrar.register_handler(ForDismissed(|this, deploy, cx| {
            this.deploy(deploy, cx);
        }))
    }

    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let query_editor = cx.new_view(Editor::single_line);
        cx.subscribe(&query_editor, Self::on_query_editor_event)
            .detach();
        let replacement_editor = cx.new_view(Editor::single_line);
        cx.subscribe(&replacement_editor, Self::on_replacement_editor_event)
            .detach();

        let search_options = SearchOptions::from_settings(&EditorSettings::get_global(cx).search);

        Self {
            query_editor,
            query_editor_focused: false,
            replacement_editor,
            replacement_editor_focused: false,
            active_searchable_item: None,
            active_searchable_item_subscription: None,
            active_match_index: None,
            searchable_items_with_matches: Default::default(),
            default_options: search_options,
            configured_options: search_options,
            search_options,
            pending_search: None,
            query_contains_error: false,
            dismissed: true,
            search_history: SearchHistory::new(
                Some(MAX_BUFFER_SEARCH_HISTORY_SIZE),
                project::search_history::QueryInsertionBehavior::ReplacePreviousIfContains,
            ),
            search_history_cursor: Default::default(),
            active_search: None,
            replace_enabled: false,
            selection_search_enabled: false,
            scroll_handle: ScrollHandle::new(),
            editor_scroll_handle: ScrollHandle::new(),
            editor_needed_width: px(0.),
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
        if let Some(active_editor) = self.active_searchable_item.as_mut() {
            self.selection_search_enabled = false;
            self.replace_enabled = false;
            active_editor.search_bar_visibility_changed(false, cx);
            active_editor.toggle_filtered_search_ranges(false, cx);
            let handle = active_editor.focus_handle(cx);
            self.focus(&handle, cx);
        }
        cx.emit(Event::UpdateLocation);
        cx.emit(ToolbarItemEvent::ChangeLocation(
            ToolbarItemLocation::Hidden,
        ));
        cx.notify();
    }

    pub fn deploy(&mut self, deploy: &Deploy, cx: &mut ViewContext<Self>) -> bool {
        if self.show(cx) {
            if let Some(active_item) = self.active_searchable_item.as_mut() {
                active_item.toggle_filtered_search_ranges(deploy.selection_search_enabled, cx);
            }
            self.search_suggested(cx);
            self.smartcase(cx);
            self.replace_enabled = deploy.replace_enabled;
            self.selection_search_enabled = deploy.selection_search_enabled;
            if deploy.focus {
                let mut handle = self.query_editor.focus_handle(cx).clone();
                let mut select_query = true;
                if deploy.replace_enabled && handle.is_focused(cx) {
                    handle = self.replacement_editor.focus_handle(cx).clone();
                    select_query = false;
                };

                if select_query {
                    self.select_query(cx);
                }

                cx.focus(&handle);
            }
            return true;
        }

        false
    }

    pub fn toggle(&mut self, action: &Deploy, cx: &mut ViewContext<Self>) {
        if self.is_dismissed() {
            self.deploy(action, cx);
        } else {
            self.dismiss(&Dismiss, cx);
        }
    }

    pub fn show(&mut self, cx: &mut ViewContext<Self>) -> bool {
        let Some(handle) = self.active_searchable_item.as_ref() else {
            return false;
        };

        self.configured_options =
            SearchOptions::from_settings(&EditorSettings::get_global(cx).search);
        if self.dismissed && self.configured_options != self.default_options {
            self.search_options = self.configured_options;
            self.default_options = self.configured_options;
        }

        self.dismissed = false;
        handle.search_bar_visibility_changed(true, cx);
        cx.notify();
        cx.emit(Event::UpdateLocation);
        cx.emit(ToolbarItemEvent::ChangeLocation(
            ToolbarItemLocation::Secondary,
        ));
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
        let updated = query != self.query(cx) || self.search_options != options;
        if updated {
            self.query_editor.update(cx, |query_editor, cx| {
                query_editor.buffer().update(cx, |query_buffer, cx| {
                    let len = query_buffer.len(cx);
                    query_buffer.edit([(0..len, query)], None, cx);
                });
            });
            self.search_options = options;
            self.clear_matches(cx);
            cx.notify();
        }
        self.update_matches(!updated, cx)
    }

    fn render_search_option_button(
        &self,
        option: SearchOptions,
        focus_handle: FocusHandle,
        action: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
    ) -> impl IntoElement {
        let is_active = self.search_options.contains(option);
        option.as_button(is_active, focus_handle, action)
    }

    pub fn focus_editor(&mut self, _: &FocusEditor, cx: &mut ViewContext<Self>) {
        if let Some(active_editor) = self.active_searchable_item.as_ref() {
            let handle = active_editor.focus_handle(cx);
            cx.focus(&handle);
        }
    }

    pub fn toggle_search_option(
        &mut self,
        search_option: SearchOptions,
        cx: &mut ViewContext<Self>,
    ) {
        self.search_options.toggle(search_option);
        self.default_options = self.search_options;
        drop(self.update_matches(false, cx));
        cx.notify();
    }

    pub fn has_search_option(&mut self, search_option: SearchOptions) -> bool {
        self.search_options.contains(search_option)
    }

    pub fn enable_search_option(
        &mut self,
        search_option: SearchOptions,
        cx: &mut ViewContext<Self>,
    ) {
        if !self.search_options.contains(search_option) {
            self.toggle_search_option(search_option, cx)
        }
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
                    .filter(|matches| !matches.is_empty())
                {
                    // If 'wrapscan' is disabled, searches do not wrap around the end of the file.
                    if !EditorSettings::get_global(cx).search_wrap
                        && ((direction == Direction::Next && index + count >= matches.len())
                            || (direction == Direction::Prev && index < count))
                    {
                        crate::show_no_more_matches(cx);
                        return;
                    }
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
                if matches.is_empty() {
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
        editor: View<Editor>,
        event: &editor::EditorEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            editor::EditorEvent::Focused => self.query_editor_focused = true,
            editor::EditorEvent::Blurred => self.query_editor_focused = false,
            editor::EditorEvent::Edited { .. } => {
                self.smartcase(cx);
                self.clear_matches(cx);
                let search = self.update_matches(false, cx);

                let width = editor.update(cx, |editor, cx| {
                    let text_layout_details = editor.text_layout_details(cx);
                    let snapshot = editor.snapshot(cx).display_snapshot;

                    snapshot.x_for_display_point(snapshot.max_point(), &text_layout_details)
                        - snapshot.x_for_display_point(DisplayPoint::zero(), &text_layout_details)
                });
                self.editor_needed_width = width;
                cx.notify();

                cx.spawn(|this, mut cx| async move {
                    search.await?;
                    this.update(&mut cx, |this, cx| this.activate_current_match(cx))
                })
                .detach_and_log_err(cx);
            }
            _ => {}
        }
    }

    fn on_replacement_editor_event(
        &mut self,
        _: View<Editor>,
        event: &editor::EditorEvent,
        _: &mut ViewContext<Self>,
    ) {
        match event {
            editor::EditorEvent::Focused => self.replacement_editor_focused = true,
            editor::EditorEvent::Blurred => self.replacement_editor_focused = false,
            _ => {}
        }
    }

    fn on_active_searchable_item_event(&mut self, event: &SearchEvent, cx: &mut ViewContext<Self>) {
        match event {
            SearchEvent::MatchesInvalidated => {
                drop(self.update_matches(false, cx));
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

    fn toggle_selection(&mut self, _: &ToggleSelection, cx: &mut ViewContext<Self>) {
        if let Some(active_item) = self.active_searchable_item.as_mut() {
            self.selection_search_enabled = !self.selection_search_enabled;
            active_item.toggle_filtered_search_ranges(self.selection_search_enabled, cx);
            drop(self.update_matches(false, cx));
            cx.notify();
        }
    }

    fn toggle_regex(&mut self, _: &ToggleRegex, cx: &mut ViewContext<Self>) {
        self.toggle_search_option(SearchOptions::REGEX, cx)
    }

    fn clear_active_searchable_item_matches(&mut self, cx: &mut WindowContext) {
        if let Some(active_searchable_item) = self.active_searchable_item.as_ref() {
            self.active_match_index = None;
            self.searchable_items_with_matches
                .remove(&active_searchable_item.downgrade());
            active_searchable_item.clear_matches(cx);
        }
    }

    pub fn has_active_match(&self) -> bool {
        self.active_match_index.is_some()
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

    fn update_matches(
        &mut self,
        reuse_existing_query: bool,
        cx: &mut ViewContext<Self>,
    ) -> oneshot::Receiver<()> {
        let (done_tx, done_rx) = oneshot::channel();
        let query = self.query(cx);
        self.pending_search.take();

        if let Some(active_searchable_item) = self.active_searchable_item.as_ref() {
            self.query_contains_error = false;
            if query.is_empty() {
                self.clear_active_searchable_item_matches(cx);
                let _ = done_tx.send(());
                cx.notify();
            } else {
                let query: Arc<_> = if let Some(search) =
                    self.active_search.take().filter(|_| reuse_existing_query)
                {
                    search
                } else {
                    if self.search_options.contains(SearchOptions::REGEX) {
                        match SearchQuery::regex(
                            query,
                            self.search_options.contains(SearchOptions::WHOLE_WORD),
                            self.search_options.contains(SearchOptions::CASE_SENSITIVE),
                            false,
                            Default::default(),
                            Default::default(),
                            None,
                        ) {
                            Ok(query) => query.with_replacement(self.replacement(cx)),
                            Err(_) => {
                                self.query_contains_error = true;
                                self.clear_active_searchable_item_matches(cx);
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
                            Default::default(),
                            Default::default(),
                            None,
                        ) {
                            Ok(query) => query.with_replacement(self.replacement(cx)),
                            Err(_) => {
                                self.query_contains_error = true;
                                self.clear_active_searchable_item_matches(cx);
                                cx.notify();
                                return done_rx;
                            }
                        }
                    }
                    .into()
                };

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
                            this.search_history
                                .add(&mut this.search_history_cursor, query_text);
                            if !this.dismissed {
                                let matches = this
                                    .searchable_items_with_matches
                                    .get(&active_searchable_item.downgrade())
                                    .unwrap();
                                if matches.is_empty() {
                                    active_searchable_item.clear_matches(cx);
                                } else {
                                    active_searchable_item.update_matches(matches, cx);
                                }
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

    pub fn update_match_index(&mut self, cx: &mut ViewContext<Self>) {
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

    fn tab(&mut self, _: &Tab, cx: &mut ViewContext<Self>) {
        // Search -> Replace -> Editor
        let focus_handle = if self.replace_enabled && self.query_editor_focused {
            self.replacement_editor.focus_handle(cx)
        } else if let Some(item) = self.active_searchable_item.as_ref() {
            item.focus_handle(cx)
        } else {
            return;
        };
        self.focus(&focus_handle, cx);
        cx.stop_propagation();
    }

    fn tab_prev(&mut self, _: &TabPrev, cx: &mut ViewContext<Self>) {
        // Search -> Replace -> Search
        let focus_handle = if self.replace_enabled && self.query_editor_focused {
            self.replacement_editor.focus_handle(cx)
        } else if self.replacement_editor_focused {
            self.query_editor.focus_handle(cx)
        } else {
            return;
        };
        self.focus(&focus_handle, cx);
        cx.stop_propagation();
    }

    fn next_history_query(&mut self, _: &NextHistoryQuery, cx: &mut ViewContext<Self>) {
        if let Some(new_query) = self
            .search_history
            .next(&mut self.search_history_cursor)
            .map(str::to_string)
        {
            drop(self.search(&new_query, Some(self.search_options), cx));
        } else {
            self.search_history_cursor.reset();
            drop(self.search("", Some(self.search_options), cx));
        }
    }

    fn previous_history_query(&mut self, _: &PreviousHistoryQuery, cx: &mut ViewContext<Self>) {
        if self.query(cx).is_empty() {
            if let Some(new_query) = self
                .search_history
                .current(&mut self.search_history_cursor)
                .map(str::to_string)
            {
                drop(self.search(&new_query, Some(self.search_options), cx));
                return;
            }
        }

        if let Some(new_query) = self
            .search_history
            .previous(&mut self.search_history_cursor)
            .map(str::to_string)
        {
            drop(self.search(&new_query, Some(self.search_options), cx));
        }
    }

    fn focus(&self, handle: &gpui::FocusHandle, cx: &mut ViewContext<Self>) {
        cx.on_next_frame(|_, cx| {
            cx.invalidate_character_coordinates();
        });
        cx.focus(handle);
    }

    fn toggle_replace(&mut self, _: &ToggleReplace, cx: &mut ViewContext<Self>) {
        if self.active_searchable_item.is_some() {
            self.replace_enabled = !self.replace_enabled;
            let handle = if self.replace_enabled {
                self.replacement_editor.focus_handle(cx)
            } else {
                self.query_editor.focus_handle(cx)
            };
            self.focus(&handle, cx);
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
                            searchable_item.replace(matches.at(active_index), &query, cx);
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
                        searchable_item.replace_all(&mut matches.iter(), &query, cx);
                    }
                }
            }
        }
    }

    pub fn match_exists(&mut self, cx: &mut ViewContext<Self>) -> bool {
        self.update_match_index(cx);
        self.active_match_index.is_some()
    }

    pub fn should_use_smartcase_search(&mut self, cx: &mut ViewContext<Self>) -> bool {
        EditorSettings::get_global(cx).use_smartcase_search
    }

    pub fn is_contains_uppercase(&mut self, str: &String) -> bool {
        str.chars().any(|c| c.is_uppercase())
    }

    fn smartcase(&mut self, cx: &mut ViewContext<Self>) {
        if self.should_use_smartcase_search(cx) {
            let query = self.query(cx);
            if !query.is_empty() {
                let is_case = self.is_contains_uppercase(&query);
                if self.has_search_option(SearchOptions::CASE_SENSITIVE) != is_case {
                    self.toggle_search_option(SearchOptions::CASE_SENSITIVE, cx);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use super::*;
    use editor::{display_map::DisplayRow, DisplayPoint, Editor, MultiBuffer, SearchSettings};
    use gpui::{Context, Hsla, TestAppContext, UpdateGlobal, VisualTestContext};
    use language::{Buffer, Point};
    use project::Project;
    use settings::SettingsStore;
    use smol::stream::StreamExt as _;
    use unindent::Unindent as _;

    fn init_globals(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
            editor::init(cx);

            language::init(cx);
            Project::init_settings(cx);
            theme::init(theme::LoadThemes::JustBase, cx);
            crate::init(cx);
        });
    }

    fn init_test(
        cx: &mut TestAppContext,
    ) -> (View<Editor>, View<BufferSearchBar>, &mut VisualTestContext) {
        init_globals(cx);
        let buffer = cx.new_model(|cx| {
            Buffer::local(
                r#"
                A regular expression (shortened as regex or regexp;[1] also referred to as
                rational expression[2][3]) is a sequence of characters that specifies a search
                pattern in text. Usually such patterns are used by string-searching algorithms
                for "find" or "find and replace" operations on strings, or for input validation.
                "#
                .unindent(),
                cx,
            )
        });
        let cx = cx.add_empty_window();
        let editor = cx.new_view(|cx| Editor::for_buffer(buffer.clone(), None, cx));

        let search_bar = cx.new_view(|cx| {
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
                    DisplayPoint::new(DisplayRow(2), 17)..DisplayPoint::new(DisplayRow(2), 19),
                    DisplayPoint::new(DisplayRow(2), 43)..DisplayPoint::new(DisplayRow(2), 45),
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
                &[DisplayPoint::new(DisplayRow(2), 43)..DisplayPoint::new(DisplayRow(2), 45),]
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
                    DisplayPoint::new(DisplayRow(0), 24)..DisplayPoint::new(DisplayRow(0), 26),
                    DisplayPoint::new(DisplayRow(0), 41)..DisplayPoint::new(DisplayRow(0), 43),
                    DisplayPoint::new(DisplayRow(2), 71)..DisplayPoint::new(DisplayRow(2), 73),
                    DisplayPoint::new(DisplayRow(3), 1)..DisplayPoint::new(DisplayRow(3), 3),
                    DisplayPoint::new(DisplayRow(3), 11)..DisplayPoint::new(DisplayRow(3), 13),
                    DisplayPoint::new(DisplayRow(3), 56)..DisplayPoint::new(DisplayRow(3), 58),
                    DisplayPoint::new(DisplayRow(3), 60)..DisplayPoint::new(DisplayRow(3), 62),
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
                    DisplayPoint::new(DisplayRow(0), 41)..DisplayPoint::new(DisplayRow(0), 43),
                    DisplayPoint::new(DisplayRow(3), 11)..DisplayPoint::new(DisplayRow(3), 13),
                    DisplayPoint::new(DisplayRow(3), 56)..DisplayPoint::new(DisplayRow(3), 58),
                ]
            );
        });

        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(0), 0)
                ])
            });
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.active_match_index, Some(0));
            search_bar.select_next_match(&SelectNextMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(DisplayRow(0), 41)..DisplayPoint::new(DisplayRow(0), 43)]
            );
        });
        search_bar.update(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_next_match(&SelectNextMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(DisplayRow(3), 11)..DisplayPoint::new(DisplayRow(3), 13)]
            );
        });
        search_bar.update(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(1));
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_next_match(&SelectNextMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(DisplayRow(3), 56)..DisplayPoint::new(DisplayRow(3), 58)]
            );
        });
        search_bar.update(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(2));
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_next_match(&SelectNextMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(DisplayRow(0), 41)..DisplayPoint::new(DisplayRow(0), 43)]
            );
        });
        search_bar.update(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_prev_match(&SelectPrevMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(DisplayRow(3), 56)..DisplayPoint::new(DisplayRow(3), 58)]
            );
        });
        search_bar.update(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(2));
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_prev_match(&SelectPrevMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(DisplayRow(3), 11)..DisplayPoint::new(DisplayRow(3), 13)]
            );
        });
        search_bar.update(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(1));
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_prev_match(&SelectPrevMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(DisplayRow(0), 41)..DisplayPoint::new(DisplayRow(0), 43)]
            );
        });
        search_bar.update(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        // Park the cursor in between matches and ensure that going to the previous match selects
        // the closest match to the left.
        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(DisplayRow(1), 0)..DisplayPoint::new(DisplayRow(1), 0)
                ])
            });
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.active_match_index, Some(1));
            search_bar.select_prev_match(&SelectPrevMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(DisplayRow(0), 41)..DisplayPoint::new(DisplayRow(0), 43)]
            );
        });
        search_bar.update(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        // Park the cursor in between matches and ensure that going to the next match selects the
        // closest match to the right.
        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(DisplayRow(1), 0)..DisplayPoint::new(DisplayRow(1), 0)
                ])
            });
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.active_match_index, Some(1));
            search_bar.select_next_match(&SelectNextMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(DisplayRow(3), 11)..DisplayPoint::new(DisplayRow(3), 13)]
            );
        });
        search_bar.update(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(1));
        });

        // Park the cursor after the last match and ensure that going to the previous match selects
        // the last match.
        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(DisplayRow(3), 60)..DisplayPoint::new(DisplayRow(3), 60)
                ])
            });
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.active_match_index, Some(2));
            search_bar.select_prev_match(&SelectPrevMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(DisplayRow(3), 56)..DisplayPoint::new(DisplayRow(3), 58)]
            );
        });
        search_bar.update(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(2));
        });

        // Park the cursor after the last match and ensure that going to the next match selects the
        // first match.
        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(DisplayRow(3), 60)..DisplayPoint::new(DisplayRow(3), 60)
                ])
            });
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.active_match_index, Some(2));
            search_bar.select_next_match(&SelectNextMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(DisplayRow(0), 41)..DisplayPoint::new(DisplayRow(0), 43)]
            );
        });
        search_bar.update(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        // Park the cursor before the first match and ensure that going to the previous match
        // selects the last match.
        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(0), 0)
                ])
            });
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.active_match_index, Some(0));
            search_bar.select_prev_match(&SelectPrevMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(DisplayRow(3), 56)..DisplayPoint::new(DisplayRow(3), 58)]
            );
        });
        search_bar.update(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(2));
        });
    }

    fn display_points_of(
        background_highlights: Vec<(Range<DisplayPoint>, Hsla)>,
    ) -> Vec<Range<DisplayPoint>> {
        background_highlights
            .into_iter()
            .map(|(range, _)| range)
            .collect::<Vec<_>>()
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
        editor.update(cx, |editor, cx| {
            assert_eq!(
                display_points_of(editor.all_text_background_highlights(cx)),
                &[DisplayPoint::new(DisplayRow(2), 43)..DisplayPoint::new(DisplayRow(2), 45),]
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
                &[DisplayPoint::new(DisplayRow(0), 35)..DisplayPoint::new(DisplayRow(0), 40),]
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
        let buffer = cx.new_model(|cx| Buffer::local(buffer_text, cx));
        let window = cx.add_window(|_| gpui::Empty);

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
                    editor.read(cx).is_focused(cx),
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
        init_globals(cx);
        let buffer_text = r#"
        A regular expression (shortened as regex or regexp;[1] also referred to as
        rational expression[2][3]) is a sequence of characters that specifies a search
        pattern in text. Usually such patterns are used by string-searching algorithms
        for "find" or "find and replace" operations on strings, or for input validation.
        "#
        .unindent();
        let buffer = cx.new_model(|cx| Buffer::local(buffer_text, cx));
        let cx = cx.add_empty_window();

        let editor = cx.new_view(|cx| Editor::for_buffer(buffer.clone(), None, cx));

        let search_bar = cx.new_view(|cx| {
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
                search_bar.search("\\[([^\\]]+)\\]", Some(SearchOptions::REGEX), cx)
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
                search_bar.search(
                    "a\\w+s",
                    Some(SearchOptions::REGEX | SearchOptions::WHOLE_WORD),
                    cx,
                )
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

    struct ReplacementTestParams<'a> {
        editor: &'a View<Editor>,
        search_bar: &'a View<BufferSearchBar>,
        cx: &'a mut VisualTestContext,
        search_text: &'static str,
        search_options: Option<SearchOptions>,
        replacement_text: &'static str,
        replace_all: bool,
        expected_text: String,
    }

    async fn run_replacement_test(options: ReplacementTestParams<'_>) {
        options
            .search_bar
            .update(options.cx, |search_bar, cx| {
                if let Some(options) = options.search_options {
                    search_bar.set_search_options(options, cx);
                }
                search_bar.search(options.search_text, options.search_options, cx)
            })
            .await
            .unwrap();

        options.search_bar.update(options.cx, |search_bar, cx| {
            search_bar.replacement_editor.update(cx, |editor, cx| {
                editor.set_text(options.replacement_text, cx);
            });

            if options.replace_all {
                search_bar.replace_all(&ReplaceAll, cx)
            } else {
                search_bar.replace_next(&ReplaceNext, cx)
            }
        });

        assert_eq!(
            options
                .editor
                .update(options.cx, |this, cx| { this.text(cx) }),
            options.expected_text
        );
    }

    #[gpui::test]
    async fn test_replace_special_characters(cx: &mut TestAppContext) {
        let (editor, search_bar, cx) = init_test(cx);

        run_replacement_test(ReplacementTestParams {
            editor: &editor,
            search_bar: &search_bar,
            cx,
            search_text: "expression",
            search_options: None,
            replacement_text: r"\n",
            replace_all: true,
            expected_text: r#"
            A regular \n (shortened as regex or regexp;[1] also referred to as
            rational \n[2][3]) is a sequence of characters that specifies a search
            pattern in text. Usually such patterns are used by string-searching algorithms
            for "find" or "find and replace" operations on strings, or for input validation.
            "#
            .unindent(),
        })
        .await;

        run_replacement_test(ReplacementTestParams {
            editor: &editor,
            search_bar: &search_bar,
            cx,
            search_text: "or",
            search_options: Some(SearchOptions::WHOLE_WORD | SearchOptions::REGEX),
            replacement_text: r"\\\n\\\\",
            replace_all: false,
            expected_text: r#"
            A regular \n (shortened as regex \
            \\ regexp;[1] also referred to as
            rational \n[2][3]) is a sequence of characters that specifies a search
            pattern in text. Usually such patterns are used by string-searching algorithms
            for "find" or "find and replace" operations on strings, or for input validation.
            "#
            .unindent(),
        })
        .await;

        run_replacement_test(ReplacementTestParams {
            editor: &editor,
            search_bar: &search_bar,
            cx,
            search_text: r"(that|used) ",
            search_options: Some(SearchOptions::REGEX),
            replacement_text: r"$1\n",
            replace_all: true,
            expected_text: r#"
            A regular \n (shortened as regex \
            \\ regexp;[1] also referred to as
            rational \n[2][3]) is a sequence of characters that
            specifies a search
            pattern in text. Usually such patterns are used
            by string-searching algorithms
            for "find" or "find and replace" operations on strings, or for input validation.
            "#
            .unindent(),
        })
        .await;
    }

    #[gpui::test]
    async fn test_find_matches_in_selections_singleton_buffer_multiple_selections(
        cx: &mut TestAppContext,
    ) {
        init_globals(cx);
        let buffer = cx.new_model(|cx| {
            Buffer::local(
                r#"
                aaa bbb aaa ccc
                aaa bbb aaa ccc
                aaa bbb aaa ccc
                aaa bbb aaa ccc
                aaa bbb aaa ccc
                aaa bbb aaa ccc
                "#
                .unindent(),
                cx,
            )
        });
        let cx = cx.add_empty_window();
        let editor = cx.new_view(|cx| Editor::for_buffer(buffer.clone(), None, cx));

        let search_bar = cx.new_view(|cx| {
            let mut search_bar = BufferSearchBar::new(cx);
            search_bar.set_active_pane_item(Some(&editor), cx);
            search_bar.show(cx);
            search_bar
        });

        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.select_ranges(vec![Point::new(1, 0)..Point::new(2, 4)])
            })
        });

        search_bar.update(cx, |search_bar, cx| {
            let deploy = Deploy {
                focus: true,
                replace_enabled: false,
                selection_search_enabled: true,
            };
            search_bar.deploy(&deploy, cx);
        });

        cx.run_until_parked();

        search_bar
            .update(cx, |search_bar, cx| search_bar.search("aaa", None, cx))
            .await
            .unwrap();

        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.search_background_highlights(cx),
                &[
                    Point::new(1, 0)..Point::new(1, 3),
                    Point::new(1, 8)..Point::new(1, 11),
                    Point::new(2, 0)..Point::new(2, 3),
                ]
            );
        });
    }

    #[gpui::test]
    async fn test_find_matches_in_selections_multiple_excerpts_buffer_multiple_selections(
        cx: &mut TestAppContext,
    ) {
        init_globals(cx);
        let text = r#"
            aaa bbb aaa ccc
            aaa bbb aaa ccc
            aaa bbb aaa ccc
            aaa bbb aaa ccc
            aaa bbb aaa ccc
            aaa bbb aaa ccc

            aaa bbb aaa ccc
            aaa bbb aaa ccc
            aaa bbb aaa ccc
            aaa bbb aaa ccc
            aaa bbb aaa ccc
            aaa bbb aaa ccc
            "#
        .unindent();

        let cx = cx.add_empty_window();
        let editor = cx.new_view(|cx| {
            let multibuffer = MultiBuffer::build_multi(
                [
                    (
                        &text,
                        vec![
                            Point::new(0, 0)..Point::new(2, 0),
                            Point::new(4, 0)..Point::new(5, 0),
                        ],
                    ),
                    (&text, vec![Point::new(9, 0)..Point::new(11, 0)]),
                ],
                cx,
            );
            Editor::for_multibuffer(multibuffer, None, false, cx)
        });

        let search_bar = cx.new_view(|cx| {
            let mut search_bar = BufferSearchBar::new(cx);
            search_bar.set_active_pane_item(Some(&editor), cx);
            search_bar.show(cx);
            search_bar
        });

        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.select_ranges(vec![
                    Point::new(1, 0)..Point::new(1, 4),
                    Point::new(5, 3)..Point::new(6, 4),
                ])
            })
        });

        search_bar.update(cx, |search_bar, cx| {
            let deploy = Deploy {
                focus: true,
                replace_enabled: false,
                selection_search_enabled: true,
            };
            search_bar.deploy(&deploy, cx);
        });

        cx.run_until_parked();

        search_bar
            .update(cx, |search_bar, cx| search_bar.search("aaa", None, cx))
            .await
            .unwrap();

        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.search_background_highlights(cx),
                &[
                    Point::new(1, 0)..Point::new(1, 3),
                    Point::new(5, 8)..Point::new(5, 11),
                    Point::new(6, 0)..Point::new(6, 3),
                ]
            );
        });
    }

    #[gpui::test]
    async fn test_invalid_regexp_search_after_valid(cx: &mut TestAppContext) {
        let (editor, search_bar, cx) = init_test(cx);
        // Search using valid regexp
        search_bar
            .update(cx, |search_bar, cx| {
                search_bar.enable_search_option(SearchOptions::REGEX, cx);
                search_bar.search("expression", None, cx)
            })
            .await
            .unwrap();
        editor.update(cx, |editor, cx| {
            assert_eq!(
                display_points_of(editor.all_text_background_highlights(cx)),
                &[
                    DisplayPoint::new(DisplayRow(0), 10)..DisplayPoint::new(DisplayRow(0), 20),
                    DisplayPoint::new(DisplayRow(1), 9)..DisplayPoint::new(DisplayRow(1), 19),
                ],
            );
        });

        // Now, the expression is invalid
        search_bar
            .update(cx, |search_bar, cx| {
                search_bar.search("expression (", None, cx)
            })
            .await
            .unwrap_err();
        editor.update(cx, |editor, cx| {
            assert!(display_points_of(editor.all_text_background_highlights(cx)).is_empty(),);
        });
    }

    #[gpui::test]
    async fn test_search_options_changes(cx: &mut TestAppContext) {
        let (_editor, search_bar, cx) = init_test(cx);
        update_search_settings(
            SearchSettings {
                whole_word: false,
                case_sensitive: false,
                include_ignored: false,
                regex: false,
            },
            cx,
        );

        let deploy = Deploy {
            focus: true,
            replace_enabled: false,
            selection_search_enabled: true,
        };

        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(
                search_bar.search_options,
                SearchOptions::NONE,
                "Should have no search options enabled by default"
            );
            search_bar.toggle_search_option(SearchOptions::WHOLE_WORD, cx);
            assert_eq!(
                search_bar.search_options,
                SearchOptions::WHOLE_WORD,
                "Should enable the option toggled"
            );
            assert!(
                !search_bar.dismissed,
                "Search bar should be present and visible"
            );
            search_bar.deploy(&deploy, cx);
            assert_eq!(
                search_bar.configured_options,
                SearchOptions::NONE,
                "Should have configured search options matching the settings"
            );
            assert_eq!(
                search_bar.search_options,
                SearchOptions::WHOLE_WORD,
                "After (re)deploying, the option should still be enabled"
            );

            search_bar.dismiss(&Dismiss, cx);
            search_bar.deploy(&deploy, cx);
            assert_eq!(
                search_bar.search_options,
                SearchOptions::NONE,
                "After hiding and showing the search bar, default options should be used"
            );

            search_bar.toggle_search_option(SearchOptions::REGEX, cx);
            search_bar.toggle_search_option(SearchOptions::WHOLE_WORD, cx);
            assert_eq!(
                search_bar.search_options,
                SearchOptions::REGEX | SearchOptions::WHOLE_WORD,
                "Should enable the options toggled"
            );
            assert!(
                !search_bar.dismissed,
                "Search bar should be present and visible"
            );
        });

        update_search_settings(
            SearchSettings {
                whole_word: false,
                case_sensitive: true,
                include_ignored: false,
                regex: false,
            },
            cx,
        );
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(
                search_bar.search_options,
                SearchOptions::REGEX | SearchOptions::WHOLE_WORD,
                "Should have no search options enabled by default"
            );

            search_bar.deploy(&deploy, cx);
            assert_eq!(
                search_bar.configured_options,
                SearchOptions::CASE_SENSITIVE,
                "Should have configured search options matching the settings"
            );
            assert_eq!(
                search_bar.search_options,
                SearchOptions::REGEX | SearchOptions::WHOLE_WORD,
                "Toggling a non-dismissed search bar with custom options should not change the default options"
            );
            search_bar.dismiss(&Dismiss, cx);
            search_bar.deploy(&deploy, cx);
            assert_eq!(
                search_bar.search_options,
                SearchOptions::CASE_SENSITIVE,
                "After hiding and showing the search bar, default options should be used"
            );
        });
    }

    fn update_search_settings(search_settings: SearchSettings, cx: &mut TestAppContext) {
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings::<EditorSettings>(cx, |settings| {
                    settings.search = Some(search_settings);
                });
            });
        });
    }
}
