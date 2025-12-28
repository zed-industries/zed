mod registrar;

use crate::{
    FocusSearch, NextHistoryQuery, PreviousHistoryQuery, ReplaceAll, ReplaceNext, SearchOption,
    SearchOptions, SearchSource, SelectAllMatches, SelectNextMatch, SelectPreviousMatch,
    ToggleCaseSensitive, ToggleRegex, ToggleReplace, ToggleSelection, ToggleWholeWord,
    buffer_search::registrar::WithResultsOrExternalQuery,
    search_bar::{ActionButtonState, input_base_styles, render_action_button, render_text_input},
};
use any_vec::AnyVec;
use collections::HashMap;
use editor::{
    DisplayPoint, Editor, EditorSettings, MultiBufferOffset,
    actions::{Backtab, Tab},
};
use futures::channel::oneshot;
use gpui::{
    Action, App, ClickEvent, Context, Entity, EventEmitter, Focusable, InteractiveElement as _,
    IntoElement, KeyContext, ParentElement as _, Render, ScrollHandle, Styled, Subscription, Task,
    Window, actions, div,
};
use language::{Language, LanguageRegistry};
use project::{
    search::SearchQuery,
    search_history::{SearchHistory, SearchHistoryCursor},
};
use schemars::JsonSchema;
use serde::Deserialize;
use settings::Settings;
use std::sync::Arc;
use zed_actions::{outline::ToggleOutline, workspace::CopyPath, workspace::CopyRelativePath};

use ui::{
    BASE_REM_SIZE_IN_PX, IconButton, IconButtonShape, IconName, Tooltip, h_flex, prelude::*,
    utils::SearchInputWidth,
};
use util::{ResultExt, paths::PathMatcher};
use workspace::{
    ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace,
    item::ItemHandle,
    searchable::{
        Direction, FilteredSearchRange, SearchEvent, SearchableItemHandle, WeakSearchableItemHandle,
    },
};

pub use registrar::DivRegistrar;
use registrar::{ForDeployed, ForDismissed, SearchActionsRegistrar};

const MAX_BUFFER_SEARCH_HISTORY_SIZE: usize = 50;

/// Opens the buffer search interface with the specified configuration.
#[derive(PartialEq, Clone, Deserialize, JsonSchema, Action)]
#[action(namespace = buffer_search)]
#[serde(deny_unknown_fields)]
pub struct Deploy {
    #[serde(default = "util::serde::default_true")]
    pub focus: bool,
    #[serde(default)]
    pub replace_enabled: bool,
    #[serde(default)]
    pub selection_search_enabled: bool,
}

actions!(
    buffer_search,
    [
        /// Deploys the search and replace interface.
        DeployReplace,
        /// Dismisses the search bar.
        Dismiss,
        /// Focuses back on the editor.
        FocusEditor
    ]
);

impl Deploy {
    pub fn find() -> Self {
        Self {
            focus: true,
            replace_enabled: false,
            selection_search_enabled: false,
        }
    }

    pub fn replace() -> Self {
        Self {
            focus: true,
            replace_enabled: true,
            selection_search_enabled: false,
        }
    }
}

pub enum Event {
    UpdateLocation,
}

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| BufferSearchBar::register(workspace))
        .detach();
}

pub struct BufferSearchBar {
    query_editor: Entity<Editor>,
    query_editor_focused: bool,
    replacement_editor: Entity<Editor>,
    replacement_editor_focused: bool,
    active_searchable_item: Option<Box<dyn SearchableItemHandle>>,
    active_match_index: Option<usize>,
    #[cfg(target_os = "macos")]
    active_searchable_item_subscriptions: Option<[Subscription; 2]>,
    #[cfg(not(target_os = "macos"))]
    active_searchable_item_subscriptions: Option<Subscription>,
    #[cfg(target_os = "macos")]
    pending_external_query: Option<(String, SearchOptions)>,
    active_search: Option<Arc<SearchQuery>>,
    searchable_items_with_matches: HashMap<Box<dyn WeakSearchableItemHandle>, AnyVec<dyn Send>>,
    pending_search: Option<Task<()>>,
    search_options: SearchOptions,
    default_options: SearchOptions,
    configured_options: SearchOptions,
    query_error: Option<String>,
    dismissed: bool,
    search_history: SearchHistory,
    search_history_cursor: SearchHistoryCursor,
    replace_enabled: bool,
    selection_search_enabled: Option<FilteredSearchRange>,
    scroll_handle: ScrollHandle,
    editor_scroll_handle: ScrollHandle,
    editor_needed_width: Pixels,
    regex_language: Option<Arc<Language>>,
}

impl EventEmitter<Event> for BufferSearchBar {}
impl EventEmitter<workspace::ToolbarItemEvent> for BufferSearchBar {}
impl Render for BufferSearchBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.dismissed {
            return div().id("search_bar");
        }

        let focus_handle = self.focus_handle(cx);

        let narrow_mode =
            self.scroll_handle.bounds().size.width / window.rem_size() < 340. / BASE_REM_SIZE_IN_PX;
        let hide_inline_icons = self.editor_needed_width
            > self.editor_scroll_handle.bounds().size.width - window.rem_size() * 6.;

        let workspace::searchable::SearchOptions {
            case,
            word,
            regex,
            replacement,
            selection,
            find_in_results,
        } = self.supported_options(cx);

        self.query_editor.update(cx, |query_editor, cx| {
            if query_editor.placeholder_text(cx).is_none() {
                query_editor.set_placeholder_text("Search…", window, cx);
            }
        });

        self.replacement_editor.update(cx, |editor, cx| {
            editor.set_placeholder_text("Replace with…", window, cx);
        });

        let mut color_override = None;
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
                    color_override = Some(Color::Error); // No matches found
                    None
                }
            })
            .unwrap_or_else(|| "0/0".to_string());
        let should_show_replace_input = self.replace_enabled && replacement;
        let in_replace = self.replacement_editor.focus_handle(cx).is_focused(window);

        let theme_colors = cx.theme().colors();
        let query_border = if self.query_error.is_some() {
            Color::Error.color(cx)
        } else {
            theme_colors.border
        };
        let replacement_border = theme_colors.border;

        let container_width = window.viewport_size().width;
        let input_width = SearchInputWidth::calc_width(container_width);

        let input_base_styles =
            |border_color| input_base_styles(border_color, |div| div.w(input_width));

        let query_column = input_base_styles(query_border)
            .id("editor-scroll")
            .track_scroll(&self.editor_scroll_handle)
            .child(render_text_input(&self.query_editor, color_override, cx))
            .when(!hide_inline_icons, |div| {
                div.child(
                    h_flex()
                        .gap_1()
                        .when(case, |div| {
                            div.child(SearchOption::CaseSensitive.as_button(
                                self.search_options,
                                SearchSource::Buffer,
                                focus_handle.clone(),
                            ))
                        })
                        .when(word, |div| {
                            div.child(SearchOption::WholeWord.as_button(
                                self.search_options,
                                SearchSource::Buffer,
                                focus_handle.clone(),
                            ))
                        })
                        .when(regex, |div| {
                            div.child(SearchOption::Regex.as_button(
                                self.search_options,
                                SearchSource::Buffer,
                                focus_handle.clone(),
                            ))
                        }),
                )
            });

        let mode_column = h_flex()
            .gap_1()
            .min_w_64()
            .when(replacement, |this| {
                this.child(render_action_button(
                    "buffer-search-bar-toggle",
                    IconName::Replace,
                    self.replace_enabled.then_some(ActionButtonState::Toggled),
                    "Toggle Replace",
                    &ToggleReplace,
                    focus_handle.clone(),
                ))
            })
            .when(selection, |this| {
                this.child(
                    IconButton::new(
                        "buffer-search-bar-toggle-search-selection-button",
                        IconName::Quote,
                    )
                    .style(ButtonStyle::Subtle)
                    .shape(IconButtonShape::Square)
                    .when(self.selection_search_enabled.is_some(), |button| {
                        button.style(ButtonStyle::Filled)
                    })
                    .on_click(cx.listener(|this, _: &ClickEvent, window, cx| {
                        this.toggle_selection(&ToggleSelection, window, cx);
                    }))
                    .toggle_state(self.selection_search_enabled.is_some())
                    .tooltip({
                        let focus_handle = focus_handle.clone();
                        move |_window, cx| {
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
            .when(!find_in_results, |el| {
                let query_focus = self.query_editor.focus_handle(cx);
                let matches_column = h_flex()
                    .pl_2()
                    .ml_2()
                    .border_l_1()
                    .border_color(theme_colors.border_variant)
                    .child(render_action_button(
                        "buffer-search-nav-button",
                        ui::IconName::ChevronLeft,
                        self.active_match_index
                            .is_none()
                            .then_some(ActionButtonState::Disabled),
                        "Select Previous Match",
                        &SelectPreviousMatch,
                        query_focus.clone(),
                    ))
                    .child(render_action_button(
                        "buffer-search-nav-button",
                        ui::IconName::ChevronRight,
                        self.active_match_index
                            .is_none()
                            .then_some(ActionButtonState::Disabled),
                        "Select Next Match",
                        &SelectNextMatch,
                        query_focus.clone(),
                    ))
                    .when(!narrow_mode, |this| {
                        this.child(div().ml_2().min_w(rems_from_px(40.)).child(
                            Label::new(match_text).size(LabelSize::Small).color(
                                if self.active_match_index.is_some() {
                                    Color::Default
                                } else {
                                    Color::Disabled
                                },
                            ),
                        ))
                    });

                el.child(render_action_button(
                    "buffer-search-nav-button",
                    IconName::SelectAll,
                    Default::default(),
                    "Select All Matches",
                    &SelectAllMatches,
                    query_focus,
                ))
                .child(matches_column)
            })
            .when(find_in_results, |el| {
                el.child(render_action_button(
                    "buffer-search",
                    IconName::Close,
                    Default::default(),
                    "Close Search Bar",
                    &Dismiss,
                    focus_handle.clone(),
                ))
            });

        let search_line = h_flex()
            .w_full()
            .gap_2()
            .when(find_in_results, |el| {
                el.child(Label::new("Find in results").color(Color::Hint))
            })
            .child(query_column)
            .child(mode_column);

        let replace_line =
            should_show_replace_input.then(|| {
                let replace_column = input_base_styles(replacement_border)
                    .child(render_text_input(&self.replacement_editor, None, cx));
                let focus_handle = self.replacement_editor.read(cx).focus_handle(cx);

                let replace_actions = h_flex()
                    .min_w_64()
                    .gap_1()
                    .child(render_action_button(
                        "buffer-search-replace-button",
                        IconName::ReplaceNext,
                        Default::default(),
                        "Replace Next Match",
                        &ReplaceNext,
                        focus_handle.clone(),
                    ))
                    .child(render_action_button(
                        "buffer-search-replace-button",
                        IconName::ReplaceAll,
                        Default::default(),
                        "Replace All Matches",
                        &ReplaceAll,
                        focus_handle,
                    ));
                h_flex()
                    .w_full()
                    .gap_2()
                    .child(replace_column)
                    .child(replace_actions)
            });

        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("BufferSearchBar");
        if in_replace {
            key_context.add("in_replace");
        }

        let query_error_line = self.query_error.as_ref().map(|error| {
            Label::new(error)
                .size(LabelSize::Small)
                .color(Color::Error)
                .mt_neg_1()
                .ml_2()
        });

        let search_line =
            h_flex()
                .relative()
                .child(search_line)
                .when(!narrow_mode && !find_in_results, |div| {
                    div.child(h_flex().absolute().right_0().child(render_action_button(
                        "buffer-search",
                        IconName::Close,
                        Default::default(),
                        "Close Search Bar",
                        &Dismiss,
                        focus_handle.clone(),
                    )))
                    .w_full()
                });
        v_flex()
            .id("buffer_search")
            .gap_2()
            .py(px(1.0))
            .w_full()
            .track_scroll(&self.scroll_handle)
            .key_context(key_context)
            .capture_action(cx.listener(Self::tab))
            .capture_action(cx.listener(Self::backtab))
            .on_action(cx.listener(Self::previous_history_query))
            .on_action(cx.listener(Self::next_history_query))
            .on_action(cx.listener(Self::dismiss))
            .on_action(cx.listener(Self::select_next_match))
            .on_action(cx.listener(Self::select_prev_match))
            .on_action(cx.listener(|this, _: &ToggleOutline, window, cx| {
                if let Some(active_searchable_item) = &mut this.active_searchable_item {
                    active_searchable_item.relay_action(Box::new(ToggleOutline), window, cx);
                }
            }))
            .on_action(cx.listener(|this, _: &CopyPath, window, cx| {
                if let Some(active_searchable_item) = &mut this.active_searchable_item {
                    active_searchable_item.relay_action(Box::new(CopyPath), window, cx);
                }
            }))
            .on_action(cx.listener(|this, _: &CopyRelativePath, window, cx| {
                if let Some(active_searchable_item) = &mut this.active_searchable_item {
                    active_searchable_item.relay_action(Box::new(CopyRelativePath), window, cx);
                }
            }))
            .when(replacement, |this| {
                this.on_action(cx.listener(Self::toggle_replace))
                    .on_action(cx.listener(Self::replace_next))
                    .on_action(cx.listener(Self::replace_all))
            })
            .when(case, |this| {
                this.on_action(cx.listener(Self::toggle_case_sensitive))
            })
            .when(word, |this| {
                this.on_action(cx.listener(Self::toggle_whole_word))
            })
            .when(regex, |this| {
                this.on_action(cx.listener(Self::toggle_regex))
            })
            .when(selection, |this| {
                this.on_action(cx.listener(Self::toggle_selection))
            })
            .child(search_line)
            .children(query_error_line)
            .children(replace_line)
    }
}

impl Focusable for BufferSearchBar {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.query_editor.focus_handle(cx)
    }
}

impl ToolbarItemView for BufferSearchBar {
    fn contribute_context(&self, context: &mut KeyContext, _cx: &App) {
        if !self.dismissed {
            context.add("buffer_search_deployed");
        }
    }

    fn set_active_pane_item(
        &mut self,
        item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        cx.notify();
        self.active_searchable_item_subscriptions.take();
        self.active_searchable_item.take();

        self.pending_search.take();

        if let Some(searchable_item_handle) =
            item.and_then(|item| item.to_searchable_item_handle(cx))
        {
            let this = cx.entity().downgrade();

            let search_event_subscription = searchable_item_handle.subscribe_to_search_events(
                window,
                cx,
                Box::new(move |search_event, window, cx| {
                    if let Some(this) = this.upgrade() {
                        this.update(cx, |this, cx| {
                            this.on_active_searchable_item_event(search_event, window, cx)
                        });
                    }
                }),
            );

            #[cfg(target_os = "macos")]
            {
                let item_focus_handle = searchable_item_handle.item_focus_handle(cx);

                self.active_searchable_item_subscriptions = Some([
                    search_event_subscription,
                    cx.on_focus(&item_focus_handle, window, |this, window, cx| {
                        if this.query_editor_focused || this.replacement_editor_focused {
                            // no need to read pasteboard since focus came from toolbar
                            return;
                        }

                        cx.defer_in(window, |this, window, cx| {
                            let Some(item) = cx.read_from_find_pasteboard() else {
                                return;
                            };
                            let Some(text) = item.text() else {
                                return;
                            };

                            if this.query(cx) == text {
                                return;
                            }

                            let search_options = item
                                .metadata()
                                .and_then(|m| m.parse().ok())
                                .and_then(SearchOptions::from_bits)
                                .unwrap_or(this.search_options);

                            if this.dismissed {
                                this.pending_external_query = Some((text, search_options));
                            } else {
                                drop(this.search(&text, Some(search_options), true, window, cx));
                            }
                        });
                    }),
                ]);
            }
            #[cfg(not(target_os = "macos"))]
            {
                self.active_searchable_item_subscriptions = Some(search_event_subscription);
            }

            let is_project_search = searchable_item_handle.supported_options(cx).find_in_results;
            self.active_searchable_item = Some(searchable_item_handle);
            drop(self.update_matches(true, false, window, cx));
            if !self.dismissed {
                if is_project_search {
                    self.dismiss(&Default::default(), window, cx);
                } else {
                    return ToolbarItemLocation::Secondary;
                }
            }
        }
        ToolbarItemLocation::Hidden
    }
}

impl BufferSearchBar {
    pub fn query_editor_focused(&self) -> bool {
        self.query_editor_focused
    }

    pub fn register(registrar: &mut impl SearchActionsRegistrar) {
        registrar.register_handler(ForDeployed(|this, _: &FocusSearch, window, cx| {
            this.query_editor.focus_handle(cx).focus(window, cx);
            this.select_query(window, cx);
        }));
        registrar.register_handler(ForDeployed(
            |this, action: &ToggleCaseSensitive, window, cx| {
                if this.supported_options(cx).case {
                    this.toggle_case_sensitive(action, window, cx);
                }
            },
        ));
        registrar.register_handler(ForDeployed(|this, action: &ToggleWholeWord, window, cx| {
            if this.supported_options(cx).word {
                this.toggle_whole_word(action, window, cx);
            }
        }));
        registrar.register_handler(ForDeployed(|this, action: &ToggleRegex, window, cx| {
            if this.supported_options(cx).regex {
                this.toggle_regex(action, window, cx);
            }
        }));
        registrar.register_handler(ForDeployed(|this, action: &ToggleSelection, window, cx| {
            if this.supported_options(cx).selection {
                this.toggle_selection(action, window, cx);
            } else {
                cx.propagate();
            }
        }));
        registrar.register_handler(ForDeployed(|this, action: &ToggleReplace, window, cx| {
            if this.supported_options(cx).replacement {
                this.toggle_replace(action, window, cx);
            } else {
                cx.propagate();
            }
        }));
        registrar.register_handler(WithResultsOrExternalQuery(
            |this, action: &SelectNextMatch, window, cx| {
                if this.supported_options(cx).find_in_results {
                    cx.propagate();
                } else {
                    this.select_next_match(action, window, cx);
                }
            },
        ));
        registrar.register_handler(WithResultsOrExternalQuery(
            |this, action: &SelectPreviousMatch, window, cx| {
                if this.supported_options(cx).find_in_results {
                    cx.propagate();
                } else {
                    this.select_prev_match(action, window, cx);
                }
            },
        ));
        registrar.register_handler(WithResultsOrExternalQuery(
            |this, action: &SelectAllMatches, window, cx| {
                if this.supported_options(cx).find_in_results {
                    cx.propagate();
                } else {
                    this.select_all_matches(action, window, cx);
                }
            },
        ));
        registrar.register_handler(ForDeployed(
            |this, _: &editor::actions::Cancel, window, cx| {
                this.dismiss(&Dismiss, window, cx);
            },
        ));
        registrar.register_handler(ForDeployed(|this, _: &Dismiss, window, cx| {
            this.dismiss(&Dismiss, window, cx);
        }));

        // register deploy buffer search for both search bar states, since we want to focus into the search bar
        // when the deploy action is triggered in the buffer.
        registrar.register_handler(ForDeployed(|this, deploy, window, cx| {
            this.deploy(deploy, window, cx);
        }));
        registrar.register_handler(ForDismissed(|this, deploy, window, cx| {
            this.deploy(deploy, window, cx);
        }));
        registrar.register_handler(ForDeployed(|this, _: &DeployReplace, window, cx| {
            if this.supported_options(cx).find_in_results {
                cx.propagate();
            } else {
                this.deploy(&Deploy::replace(), window, cx);
            }
        }));
        registrar.register_handler(ForDismissed(|this, _: &DeployReplace, window, cx| {
            if this.supported_options(cx).find_in_results {
                cx.propagate();
            } else {
                this.deploy(&Deploy::replace(), window, cx);
            }
        }));
    }

    pub fn new(
        languages: Option<Arc<LanguageRegistry>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let query_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_use_autoclose(false);
            editor
        });
        cx.subscribe_in(&query_editor, window, Self::on_query_editor_event)
            .detach();
        let replacement_editor = cx.new(|cx| Editor::single_line(window, cx));
        cx.subscribe(&replacement_editor, Self::on_replacement_editor_event)
            .detach();

        let search_options = SearchOptions::from_settings(&EditorSettings::get_global(cx).search);
        if let Some(languages) = languages {
            let query_buffer = query_editor
                .read(cx)
                .buffer()
                .read(cx)
                .as_singleton()
                .expect("query editor should be backed by a singleton buffer");

            query_buffer
                .read(cx)
                .set_language_registry(languages.clone());

            cx.spawn(async move |buffer_search_bar, cx| {
                use anyhow::Context as _;

                let regex_language = languages
                    .language_for_name("regex")
                    .await
                    .context("loading regex language")?;

                buffer_search_bar
                    .update(cx, |buffer_search_bar, cx| {
                        buffer_search_bar.regex_language = Some(regex_language);
                        buffer_search_bar.adjust_query_regex_language(cx);
                    })
                    .ok();
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        }

        Self {
            query_editor,
            query_editor_focused: false,
            replacement_editor,
            replacement_editor_focused: false,
            active_searchable_item: None,
            active_searchable_item_subscriptions: None,
            #[cfg(target_os = "macos")]
            pending_external_query: None,
            active_match_index: None,
            searchable_items_with_matches: Default::default(),
            default_options: search_options,
            configured_options: search_options,
            search_options,
            pending_search: None,
            query_error: None,
            dismissed: true,
            search_history: SearchHistory::new(
                Some(MAX_BUFFER_SEARCH_HISTORY_SIZE),
                project::search_history::QueryInsertionBehavior::ReplacePreviousIfContains,
            ),
            search_history_cursor: Default::default(),
            active_search: None,
            replace_enabled: false,
            selection_search_enabled: None,
            scroll_handle: ScrollHandle::new(),
            editor_scroll_handle: ScrollHandle::new(),
            editor_needed_width: px(0.),
            regex_language: None,
        }
    }

    pub fn is_dismissed(&self) -> bool {
        self.dismissed
    }

    pub fn dismiss(&mut self, _: &Dismiss, window: &mut Window, cx: &mut Context<Self>) {
        self.dismissed = true;
        self.query_error = None;
        self.sync_select_next_case_sensitivity(cx);

        for searchable_item in self.searchable_items_with_matches.keys() {
            if let Some(searchable_item) =
                WeakSearchableItemHandle::upgrade(searchable_item.as_ref(), cx)
            {
                searchable_item.clear_matches(window, cx);
            }
        }
        if let Some(active_editor) = self.active_searchable_item.as_mut() {
            self.selection_search_enabled = None;
            self.replace_enabled = false;
            active_editor.search_bar_visibility_changed(false, window, cx);
            active_editor.toggle_filtered_search_ranges(None, window, cx);
            let handle = active_editor.item_focus_handle(cx);
            self.focus(&handle, window, cx);
        }

        cx.emit(Event::UpdateLocation);
        cx.emit(ToolbarItemEvent::ChangeLocation(
            ToolbarItemLocation::Hidden,
        ));
        cx.notify();
    }

    pub fn deploy(&mut self, deploy: &Deploy, window: &mut Window, cx: &mut Context<Self>) -> bool {
        let filtered_search_range = if deploy.selection_search_enabled {
            Some(FilteredSearchRange::Default)
        } else {
            None
        };
        if self.show(window, cx) {
            if let Some(active_item) = self.active_searchable_item.as_mut() {
                active_item.toggle_filtered_search_ranges(filtered_search_range, window, cx);
            }
            self.search_suggested(window, cx);
            self.smartcase(window, cx);
            self.sync_select_next_case_sensitivity(cx);
            self.replace_enabled |= deploy.replace_enabled;
            self.selection_search_enabled =
                self.selection_search_enabled
                    .or(if deploy.selection_search_enabled {
                        Some(FilteredSearchRange::Default)
                    } else {
                        None
                    });
            if deploy.focus {
                let mut handle = self.query_editor.focus_handle(cx);
                let mut select_query = true;
                if deploy.replace_enabled && handle.is_focused(window) {
                    handle = self.replacement_editor.focus_handle(cx);
                    select_query = false;
                };

                if select_query {
                    self.select_query(window, cx);
                }

                window.focus(&handle, cx);
            }
            return true;
        }

        cx.propagate();
        false
    }

    pub fn toggle(&mut self, action: &Deploy, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_dismissed() {
            self.deploy(action, window, cx);
        } else {
            self.dismiss(&Dismiss, window, cx);
        }
    }

    pub fn show(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        let Some(handle) = self.active_searchable_item.as_ref() else {
            return false;
        };

        let configured_options =
            SearchOptions::from_settings(&EditorSettings::get_global(cx).search);
        let settings_changed = configured_options != self.configured_options;

        if self.dismissed && settings_changed {
            // Only update configuration options when search bar is dismissed,
            // so we don't miss updates even after calling show twice
            self.configured_options = configured_options;
            self.search_options = configured_options;
            self.default_options = configured_options;
        }

        self.dismissed = false;
        self.adjust_query_regex_language(cx);
        handle.search_bar_visibility_changed(true, window, cx);
        cx.notify();
        cx.emit(Event::UpdateLocation);
        cx.emit(ToolbarItemEvent::ChangeLocation(
            ToolbarItemLocation::Secondary,
        ));
        true
    }

    fn supported_options(&self, cx: &mut Context<Self>) -> workspace::searchable::SearchOptions {
        self.active_searchable_item
            .as_ref()
            .map(|item| item.supported_options(cx))
            .unwrap_or_default()
    }

    pub fn search_suggested(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let search = self.query_suggestion(window, cx).map(|suggestion| {
            self.search(&suggestion, Some(self.default_options), true, window, cx)
        });

        #[cfg(target_os = "macos")]
        let search = search.or_else(|| {
            self.pending_external_query
                .take()
                .map(|(query, options)| self.search(&query, Some(options), true, window, cx))
        });

        if let Some(search) = search {
            cx.spawn_in(window, async move |this, cx| {
                if search.await.is_ok() {
                    this.update_in(cx, |this, window, cx| {
                        if !this.dismissed {
                            this.activate_current_match(window, cx)
                        }
                    })
                } else {
                    Ok(())
                }
            })
            .detach_and_log_err(cx);
        }
    }

    pub fn activate_current_match(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(match_ix) = self.active_match_index
            && let Some(active_searchable_item) = self.active_searchable_item.as_ref()
            && let Some(matches) = self
                .searchable_items_with_matches
                .get(&active_searchable_item.downgrade())
        {
            active_searchable_item.activate_match(match_ix, matches, window, cx)
        }
    }

    pub fn select_query(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.query_editor.update(cx, |query_editor, cx| {
            query_editor.select_all(&Default::default(), window, cx);
        });
    }

    pub fn query(&self, cx: &App) -> String {
        self.query_editor.read(cx).text(cx)
    }

    pub fn replacement(&self, cx: &mut App) -> String {
        self.replacement_editor.read(cx).text(cx)
    }

    pub fn query_suggestion(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<String> {
        self.active_searchable_item
            .as_ref()
            .map(|searchable_item| searchable_item.query_suggestion(window, cx))
            .filter(|suggestion| !suggestion.is_empty())
    }

    pub fn set_replacement(&mut self, replacement: Option<&str>, cx: &mut Context<Self>) {
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
                        replacement_buffer.edit(
                            [(MultiBufferOffset(0)..len, replacement.unwrap())],
                            None,
                            cx,
                        );
                    });
            });
    }

    pub fn focus_replace(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.focus(&self.replacement_editor.focus_handle(cx), window, cx);
        cx.notify();
    }

    pub fn search(
        &mut self,
        query: &str,
        options: Option<SearchOptions>,
        add_to_history: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> oneshot::Receiver<()> {
        let options = options.unwrap_or(self.default_options);
        let updated = query != self.query(cx) || self.search_options != options;
        if updated {
            self.query_editor.update(cx, |query_editor, cx| {
                query_editor.buffer().update(cx, |query_buffer, cx| {
                    let len = query_buffer.len(cx);
                    query_buffer.edit([(MultiBufferOffset(0)..len, query)], None, cx);
                });
            });
            self.set_search_options(options, cx);
            self.clear_matches(window, cx);
            #[cfg(target_os = "macos")]
            self.update_find_pasteboard(cx);
            cx.notify();
        }
        self.update_matches(!updated, add_to_history, window, cx)
    }

    #[cfg(target_os = "macos")]
    pub fn update_find_pasteboard(&mut self, cx: &mut App) {
        cx.write_to_find_pasteboard(gpui::ClipboardItem::new_string_with_metadata(
            self.query(cx),
            self.search_options.bits().to_string(),
        ));
    }

    pub fn focus_editor(&mut self, _: &FocusEditor, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(active_editor) = self.active_searchable_item.as_ref() {
            let handle = active_editor.item_focus_handle(cx);
            window.focus(&handle, cx);
        }
    }

    pub fn toggle_search_option(
        &mut self,
        search_option: SearchOptions,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.search_options.toggle(search_option);
        self.default_options = self.search_options;
        drop(self.update_matches(false, false, window, cx));
        self.adjust_query_regex_language(cx);
        self.sync_select_next_case_sensitivity(cx);
        cx.notify();
    }

    pub fn has_search_option(&mut self, search_option: SearchOptions) -> bool {
        self.search_options.contains(search_option)
    }

    pub fn enable_search_option(
        &mut self,
        search_option: SearchOptions,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.search_options.contains(search_option) {
            self.toggle_search_option(search_option, window, cx)
        }
    }

    pub fn set_search_within_selection(
        &mut self,
        search_within_selection: Option<FilteredSearchRange>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<oneshot::Receiver<()>> {
        let active_item = self.active_searchable_item.as_mut()?;
        self.selection_search_enabled = search_within_selection;
        active_item.toggle_filtered_search_ranges(self.selection_search_enabled, window, cx);
        cx.notify();
        Some(self.update_matches(false, false, window, cx))
    }

    pub fn set_search_options(&mut self, search_options: SearchOptions, cx: &mut Context<Self>) {
        self.search_options = search_options;
        self.adjust_query_regex_language(cx);
        self.sync_select_next_case_sensitivity(cx);
        cx.notify();
    }

    pub fn clear_search_within_ranges(
        &mut self,
        search_options: SearchOptions,
        cx: &mut Context<Self>,
    ) {
        self.search_options = search_options;
        self.adjust_query_regex_language(cx);
        cx.notify();
    }

    fn select_next_match(
        &mut self,
        _: &SelectNextMatch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_match(Direction::Next, 1, window, cx);
    }

    fn select_prev_match(
        &mut self,
        _: &SelectPreviousMatch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_match(Direction::Prev, 1, window, cx);
    }

    pub fn select_all_matches(
        &mut self,
        _: &SelectAllMatches,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.dismissed
            && self.active_match_index.is_some()
            && let Some(searchable_item) = self.active_searchable_item.as_ref()
            && let Some(matches) = self
                .searchable_items_with_matches
                .get(&searchable_item.downgrade())
        {
            searchable_item.select_matches(matches, window, cx);
            self.focus_editor(&FocusEditor, window, cx);
        }
    }

    pub fn select_match(
        &mut self,
        direction: Direction,
        count: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        #[cfg(target_os = "macos")]
        if let Some((query, options)) = self.pending_external_query.take() {
            let search_rx = self.search(&query, Some(options), true, window, cx);
            cx.spawn_in(window, async move |this, cx| {
                if search_rx.await.is_ok() {
                    this.update_in(cx, |this, window, cx| {
                        this.activate_current_match(window, cx);
                    })
                    .ok();
                }
            })
            .detach();

            return;
        }

        if let Some(index) = self.active_match_index
            && let Some(searchable_item) = self.active_searchable_item.as_ref()
            && let Some(matches) = self
                .searchable_items_with_matches
                .get(&searchable_item.downgrade())
                .filter(|matches| !matches.is_empty())
        {
            // If 'wrapscan' is disabled, searches do not wrap around the end of the file.
            if !EditorSettings::get_global(cx).search_wrap
                && ((direction == Direction::Next && index + count >= matches.len())
                    || (direction == Direction::Prev && index < count))
            {
                crate::show_no_more_matches(window, cx);
                return;
            }
            let new_match_index = searchable_item
                .match_index_for_direction(matches, index, direction, count, window, cx);

            searchable_item.update_matches(matches, Some(new_match_index), window, cx);
            searchable_item.activate_match(new_match_index, matches, window, cx);
        }
    }

    pub fn select_first_match(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(searchable_item) = self.active_searchable_item.as_ref()
            && let Some(matches) = self
                .searchable_items_with_matches
                .get(&searchable_item.downgrade())
        {
            if matches.is_empty() {
                return;
            }
            searchable_item.update_matches(matches, Some(0), window, cx);
            searchable_item.activate_match(0, matches, window, cx);
        }
    }

    pub fn select_last_match(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(searchable_item) = self.active_searchable_item.as_ref()
            && let Some(matches) = self
                .searchable_items_with_matches
                .get(&searchable_item.downgrade())
        {
            if matches.is_empty() {
                return;
            }
            let new_match_index = matches.len() - 1;
            searchable_item.update_matches(matches, Some(new_match_index), window, cx);
            searchable_item.activate_match(new_match_index, matches, window, cx);
        }
    }

    fn on_query_editor_event(
        &mut self,
        editor: &Entity<Editor>,
        event: &editor::EditorEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            editor::EditorEvent::Focused => self.query_editor_focused = true,
            editor::EditorEvent::Blurred => self.query_editor_focused = false,
            editor::EditorEvent::Edited { .. } => {
                self.smartcase(window, cx);
                self.clear_matches(window, cx);
                let search = self.update_matches(false, true, window, cx);

                let width = editor.update(cx, |editor, cx| {
                    let text_layout_details = editor.text_layout_details(window);
                    let snapshot = editor.snapshot(window, cx).display_snapshot;

                    snapshot.x_for_display_point(snapshot.max_point(), &text_layout_details)
                        - snapshot.x_for_display_point(DisplayPoint::zero(), &text_layout_details)
                });
                self.editor_needed_width = width;
                cx.notify();

                cx.spawn_in(window, async move |this, cx| {
                    if search.await.is_ok() {
                        this.update_in(cx, |this, window, cx| {
                            this.activate_current_match(window, cx);
                            #[cfg(target_os = "macos")]
                            this.update_find_pasteboard(cx);
                        })?;
                    }
                    anyhow::Ok(())
                })
                .detach_and_log_err(cx);
            }
            _ => {}
        }
    }

    fn on_replacement_editor_event(
        &mut self,
        _: Entity<Editor>,
        event: &editor::EditorEvent,
        _: &mut Context<Self>,
    ) {
        match event {
            editor::EditorEvent::Focused => self.replacement_editor_focused = true,
            editor::EditorEvent::Blurred => self.replacement_editor_focused = false,
            _ => {}
        }
    }

    fn on_active_searchable_item_event(
        &mut self,
        event: &SearchEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            SearchEvent::MatchesInvalidated => {
                drop(self.update_matches(false, false, window, cx));
            }
            SearchEvent::ActiveMatchChanged => self.update_match_index(window, cx),
        }
    }

    fn toggle_case_sensitive(
        &mut self,
        _: &ToggleCaseSensitive,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.toggle_search_option(SearchOptions::CASE_SENSITIVE, window, cx)
    }

    fn toggle_whole_word(
        &mut self,
        _: &ToggleWholeWord,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.toggle_search_option(SearchOptions::WHOLE_WORD, window, cx)
    }

    fn toggle_selection(
        &mut self,
        _: &ToggleSelection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_search_within_selection(
            if let Some(_) = self.selection_search_enabled {
                None
            } else {
                Some(FilteredSearchRange::Default)
            },
            window,
            cx,
        );
    }

    fn toggle_regex(&mut self, _: &ToggleRegex, window: &mut Window, cx: &mut Context<Self>) {
        self.toggle_search_option(SearchOptions::REGEX, window, cx)
    }

    fn clear_active_searchable_item_matches(&mut self, window: &mut Window, cx: &mut App) {
        if let Some(active_searchable_item) = self.active_searchable_item.as_ref() {
            self.active_match_index = None;
            self.searchable_items_with_matches
                .remove(&active_searchable_item.downgrade());
            active_searchable_item.clear_matches(window, cx);
        }
    }

    pub fn has_active_match(&self) -> bool {
        self.active_match_index.is_some()
    }

    fn clear_matches(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let mut active_item_matches = None;
        for (searchable_item, matches) in self.searchable_items_with_matches.drain() {
            if let Some(searchable_item) =
                WeakSearchableItemHandle::upgrade(searchable_item.as_ref(), cx)
            {
                if Some(&searchable_item) == self.active_searchable_item.as_ref() {
                    active_item_matches = Some((searchable_item.downgrade(), matches));
                } else {
                    searchable_item.clear_matches(window, cx);
                }
            }
        }

        self.searchable_items_with_matches
            .extend(active_item_matches);
    }

    fn update_matches(
        &mut self,
        reuse_existing_query: bool,
        add_to_history: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> oneshot::Receiver<()> {
        let (done_tx, done_rx) = oneshot::channel();
        let query = self.query(cx);
        self.pending_search.take();
        #[cfg(target_os = "macos")]
        self.pending_external_query.take();

        if let Some(active_searchable_item) = self.active_searchable_item.as_ref() {
            self.query_error = None;
            if query.is_empty() {
                self.clear_active_searchable_item_matches(window, cx);
                let _ = done_tx.send(());
                cx.notify();
            } else {
                let query: Arc<_> = if let Some(search) =
                    self.active_search.take().filter(|_| reuse_existing_query)
                {
                    search
                } else {
                    // Value doesn't matter, we only construct empty matchers with it

                    if self.search_options.contains(SearchOptions::REGEX) {
                        match SearchQuery::regex(
                            query,
                            self.search_options.contains(SearchOptions::WHOLE_WORD),
                            self.search_options.contains(SearchOptions::CASE_SENSITIVE),
                            false,
                            self.search_options
                                .contains(SearchOptions::ONE_MATCH_PER_LINE),
                            PathMatcher::default(),
                            PathMatcher::default(),
                            false,
                            None,
                        ) {
                            Ok(query) => query.with_replacement(self.replacement(cx)),
                            Err(e) => {
                                self.query_error = Some(e.to_string());
                                self.clear_active_searchable_item_matches(window, cx);
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
                            PathMatcher::default(),
                            PathMatcher::default(),
                            false,
                            None,
                        ) {
                            Ok(query) => query.with_replacement(self.replacement(cx)),
                            Err(e) => {
                                self.query_error = Some(e.to_string());
                                self.clear_active_searchable_item_matches(window, cx);
                                cx.notify();
                                return done_rx;
                            }
                        }
                    }
                    .into()
                };

                self.active_search = Some(query.clone());
                let query_text = query.as_str().to_string();

                let matches = active_searchable_item.find_matches(query, window, cx);

                let active_searchable_item = active_searchable_item.downgrade();
                self.pending_search = Some(cx.spawn_in(window, async move |this, cx| {
                    let matches = matches.await;

                    this.update_in(cx, |this, window, cx| {
                        if let Some(active_searchable_item) =
                            WeakSearchableItemHandle::upgrade(active_searchable_item.as_ref(), cx)
                        {
                            this.searchable_items_with_matches
                                .insert(active_searchable_item.downgrade(), matches);

                            this.update_match_index(window, cx);

                            if add_to_history {
                                this.search_history
                                    .add(&mut this.search_history_cursor, query_text);
                            }
                            if !this.dismissed {
                                let matches = this
                                    .searchable_items_with_matches
                                    .get(&active_searchable_item.downgrade())
                                    .unwrap();
                                if matches.is_empty() {
                                    active_searchable_item.clear_matches(window, cx);
                                } else {
                                    active_searchable_item.update_matches(
                                        matches,
                                        this.active_match_index,
                                        window,
                                        cx,
                                    );
                                }
                            }
                            let _ = done_tx.send(());
                            cx.notify();
                        }
                    })
                    .log_err();
                }));
            }
        }
        done_rx
    }

    fn reverse_direction_if_backwards(&self, direction: Direction) -> Direction {
        if self.search_options.contains(SearchOptions::BACKWARDS) {
            direction.opposite()
        } else {
            direction
        }
    }

    pub fn update_match_index(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let direction = self.reverse_direction_if_backwards(Direction::Next);
        let new_index = self
            .active_searchable_item
            .as_ref()
            .and_then(|searchable_item| {
                let matches = self
                    .searchable_items_with_matches
                    .get(&searchable_item.downgrade())?;
                searchable_item.active_match_index(direction, matches, window, cx)
            });
        if new_index != self.active_match_index {
            self.active_match_index = new_index;
            if !self.dismissed {
                if let Some(searchable_item) = self.active_searchable_item.as_ref() {
                    if let Some(matches) = self
                        .searchable_items_with_matches
                        .get(&searchable_item.downgrade())
                    {
                        if !matches.is_empty() {
                            searchable_item.update_matches(matches, new_index, window, cx);
                        }
                    }
                }
            }
            cx.notify();
        }
    }

    fn tab(&mut self, _: &Tab, window: &mut Window, cx: &mut Context<Self>) {
        self.cycle_field(Direction::Next, window, cx);
    }

    fn backtab(&mut self, _: &Backtab, window: &mut Window, cx: &mut Context<Self>) {
        self.cycle_field(Direction::Prev, window, cx);
    }
    fn cycle_field(&mut self, direction: Direction, window: &mut Window, cx: &mut Context<Self>) {
        let mut handles = vec![self.query_editor.focus_handle(cx)];
        if self.replace_enabled {
            handles.push(self.replacement_editor.focus_handle(cx));
        }
        if let Some(item) = self.active_searchable_item.as_ref() {
            handles.push(item.item_focus_handle(cx));
        }
        let current_index = match handles.iter().position(|focus| focus.is_focused(window)) {
            Some(index) => index,
            None => return,
        };

        let new_index = match direction {
            Direction::Next => (current_index + 1) % handles.len(),
            Direction::Prev if current_index == 0 => handles.len() - 1,
            Direction::Prev => (current_index - 1) % handles.len(),
        };
        let next_focus_handle = &handles[new_index];
        self.focus(next_focus_handle, window, cx);
        cx.stop_propagation();
    }

    fn next_history_query(
        &mut self,
        _: &NextHistoryQuery,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(new_query) = self
            .search_history
            .next(&mut self.search_history_cursor)
            .map(str::to_string)
        {
            drop(self.search(&new_query, Some(self.search_options), false, window, cx));
        } else {
            self.search_history_cursor.reset();
            drop(self.search("", Some(self.search_options), false, window, cx));
        }
    }

    fn previous_history_query(
        &mut self,
        _: &PreviousHistoryQuery,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.query(cx).is_empty()
            && let Some(new_query) = self
                .search_history
                .current(&self.search_history_cursor)
                .map(str::to_string)
        {
            drop(self.search(&new_query, Some(self.search_options), false, window, cx));
            return;
        }

        if let Some(new_query) = self
            .search_history
            .previous(&mut self.search_history_cursor)
            .map(str::to_string)
        {
            drop(self.search(&new_query, Some(self.search_options), false, window, cx));
        }
    }

    fn focus(&self, handle: &gpui::FocusHandle, window: &mut Window, cx: &mut App) {
        window.invalidate_character_coordinates();
        window.focus(handle, cx);
    }

    fn toggle_replace(&mut self, _: &ToggleReplace, window: &mut Window, cx: &mut Context<Self>) {
        if self.active_searchable_item.is_some() {
            self.replace_enabled = !self.replace_enabled;
            let handle = if self.replace_enabled {
                self.replacement_editor.focus_handle(cx)
            } else {
                self.query_editor.focus_handle(cx)
            };
            self.focus(&handle, window, cx);
            cx.notify();
        }
    }

    fn replace_next(&mut self, _: &ReplaceNext, window: &mut Window, cx: &mut Context<Self>) {
        let mut should_propagate = true;
        if !self.dismissed
            && self.active_search.is_some()
            && let Some(searchable_item) = self.active_searchable_item.as_ref()
            && let Some(query) = self.active_search.as_ref()
            && let Some(matches) = self
                .searchable_items_with_matches
                .get(&searchable_item.downgrade())
        {
            if let Some(active_index) = self.active_match_index {
                let query = query
                    .as_ref()
                    .clone()
                    .with_replacement(self.replacement(cx));
                searchable_item.replace(matches.at(active_index), &query, window, cx);
                self.select_next_match(&SelectNextMatch, window, cx);
            }
            should_propagate = false;
        }
        if !should_propagate {
            cx.stop_propagation();
        }
    }

    pub fn replace_all(&mut self, _: &ReplaceAll, window: &mut Window, cx: &mut Context<Self>) {
        if !self.dismissed
            && self.active_search.is_some()
            && let Some(searchable_item) = self.active_searchable_item.as_ref()
            && let Some(query) = self.active_search.as_ref()
            && let Some(matches) = self
                .searchable_items_with_matches
                .get(&searchable_item.downgrade())
        {
            let query = query
                .as_ref()
                .clone()
                .with_replacement(self.replacement(cx));
            searchable_item.replace_all(&mut matches.iter(), &query, window, cx);
        }
    }

    pub fn match_exists(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        self.update_match_index(window, cx);
        self.active_match_index.is_some()
    }

    pub fn should_use_smartcase_search(&mut self, cx: &mut Context<Self>) -> bool {
        EditorSettings::get_global(cx).use_smartcase_search
    }

    pub fn is_contains_uppercase(&mut self, str: &String) -> bool {
        str.chars().any(|c| c.is_uppercase())
    }

    fn smartcase(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.should_use_smartcase_search(cx) {
            let query = self.query(cx);
            if !query.is_empty() {
                let is_case = self.is_contains_uppercase(&query);
                if self.has_search_option(SearchOptions::CASE_SENSITIVE) != is_case {
                    self.toggle_search_option(SearchOptions::CASE_SENSITIVE, window, cx);
                }
            }
        }
    }

    fn adjust_query_regex_language(&self, cx: &mut App) {
        let enable = self.search_options.contains(SearchOptions::REGEX);
        let query_buffer = self
            .query_editor
            .read(cx)
            .buffer()
            .read(cx)
            .as_singleton()
            .expect("query editor should be backed by a singleton buffer");

        if enable {
            if let Some(regex_language) = self.regex_language.clone() {
                query_buffer.update(cx, |query_buffer, cx| {
                    query_buffer.set_language(Some(regex_language), cx);
                })
            }
        } else {
            query_buffer.update(cx, |query_buffer, cx| {
                query_buffer.set_language(None, cx);
            })
        }
    }

    /// Updates the searchable item's case sensitivity option to match the
    /// search bar's current case sensitivity setting. This ensures that
    /// editor's `select_next`/ `select_previous` operations respect the buffer
    /// search bar's search options.
    ///
    /// Clears the case sensitivity when the search bar is dismissed so that
    /// only the editor's settings are respected.
    fn sync_select_next_case_sensitivity(&self, cx: &mut Context<Self>) {
        let case_sensitive = match self.dismissed {
            true => None,
            false => Some(self.search_options.contains(SearchOptions::CASE_SENSITIVE)),
        };

        if let Some(active_searchable_item) = self.active_searchable_item.as_ref() {
            active_searchable_item.set_search_is_case_sensitive(case_sensitive, cx);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use super::*;
    use editor::{
        DisplayPoint, Editor, MultiBuffer, SearchSettings, SelectionEffects,
        display_map::DisplayRow, test::editor_test_context::EditorTestContext,
    };
    use gpui::{Hsla, TestAppContext, UpdateGlobal, VisualTestContext};
    use language::{Buffer, Point};
    use settings::{SearchSettingsContent, SettingsStore};
    use smol::stream::StreamExt as _;
    use unindent::Unindent as _;
    use util_macros::perf;

    fn init_globals(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
            editor::init(cx);

            theme::init(theme::LoadThemes::JustBase, cx);
            crate::init(cx);
        });
    }

    fn init_test(
        cx: &mut TestAppContext,
    ) -> (
        Entity<Editor>,
        Entity<BufferSearchBar>,
        &mut VisualTestContext,
    ) {
        init_globals(cx);
        let buffer = cx.new(|cx| {
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
        let mut editor = None;
        let window = cx.add_window(|window, cx| {
            let default_key_bindings = settings::KeymapFile::load_asset_allow_partial_failure(
                "keymaps/default-macos.json",
                cx,
            )
            .unwrap();
            cx.bind_keys(default_key_bindings);
            editor = Some(cx.new(|cx| Editor::for_buffer(buffer.clone(), None, window, cx)));
            let mut search_bar = BufferSearchBar::new(None, window, cx);
            search_bar.set_active_pane_item(Some(&editor.clone().unwrap()), window, cx);
            search_bar.show(window, cx);
            search_bar
        });
        let search_bar = window.root(cx).unwrap();

        let cx = VisualTestContext::from_window(*window, cx).into_mut();

        (editor.unwrap(), search_bar, cx)
    }

    #[perf]
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
            .update_in(cx, |search_bar, window, cx| {
                search_bar.search("us", None, true, window, cx)
            })
            .await
            .unwrap();
        editor.update_in(cx, |editor, window, cx| {
            assert_eq!(
                display_points_of(editor.all_text_background_highlights(window, cx)),
                &[
                    DisplayPoint::new(DisplayRow(2), 17)..DisplayPoint::new(DisplayRow(2), 19),
                    DisplayPoint::new(DisplayRow(2), 43)..DisplayPoint::new(DisplayRow(2), 45),
                ]
            );
        });

        // Switch to a case sensitive search.
        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.toggle_search_option(SearchOptions::CASE_SENSITIVE, window, cx);
        });
        let mut editor_notifications = cx.notifications(&editor);
        editor_notifications.next().await;
        editor.update_in(cx, |editor, window, cx| {
            assert_eq!(
                display_points_of(editor.all_text_background_highlights(window, cx)),
                &[DisplayPoint::new(DisplayRow(2), 43)..DisplayPoint::new(DisplayRow(2), 45),]
            );
        });

        // Search for a string that appears both as a whole word and
        // within other words. By default, all results are found.
        search_bar
            .update_in(cx, |search_bar, window, cx| {
                search_bar.search("or", None, true, window, cx)
            })
            .await
            .unwrap();
        editor.update_in(cx, |editor, window, cx| {
            assert_eq!(
                display_points_of(editor.all_text_background_highlights(window, cx)),
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
        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.toggle_search_option(SearchOptions::WHOLE_WORD, window, cx);
        });
        let mut editor_notifications = cx.notifications(&editor);
        editor_notifications.next().await;
        editor.update_in(cx, |editor, window, cx| {
            assert_eq!(
                display_points_of(editor.all_text_background_highlights(window, cx)),
                &[
                    DisplayPoint::new(DisplayRow(0), 41)..DisplayPoint::new(DisplayRow(0), 43),
                    DisplayPoint::new(DisplayRow(3), 11)..DisplayPoint::new(DisplayRow(3), 13),
                    DisplayPoint::new(DisplayRow(3), 56)..DisplayPoint::new(DisplayRow(3), 58),
                ]
            );
        });

        editor.update_in(cx, |editor, window, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(0), 0)
                ])
            });
        });
        search_bar.update_in(cx, |search_bar, window, cx| {
            assert_eq!(search_bar.active_match_index, Some(0));
            search_bar.select_next_match(&SelectNextMatch, window, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor
                    .selections
                    .display_ranges(&editor.display_snapshot(cx))),
                [DisplayPoint::new(DisplayRow(0), 41)..DisplayPoint::new(DisplayRow(0), 43)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.select_next_match(&SelectNextMatch, window, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor
                    .selections
                    .display_ranges(&editor.display_snapshot(cx))),
                [DisplayPoint::new(DisplayRow(3), 11)..DisplayPoint::new(DisplayRow(3), 13)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(1));
        });

        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.select_next_match(&SelectNextMatch, window, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor
                    .selections
                    .display_ranges(&editor.display_snapshot(cx))),
                [DisplayPoint::new(DisplayRow(3), 56)..DisplayPoint::new(DisplayRow(3), 58)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(2));
        });

        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.select_next_match(&SelectNextMatch, window, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor
                    .selections
                    .display_ranges(&editor.display_snapshot(cx))),
                [DisplayPoint::new(DisplayRow(0), 41)..DisplayPoint::new(DisplayRow(0), 43)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.select_prev_match(&SelectPreviousMatch, window, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor
                    .selections
                    .display_ranges(&editor.display_snapshot(cx))),
                [DisplayPoint::new(DisplayRow(3), 56)..DisplayPoint::new(DisplayRow(3), 58)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(2));
        });

        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.select_prev_match(&SelectPreviousMatch, window, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor
                    .selections
                    .display_ranges(&editor.display_snapshot(cx))),
                [DisplayPoint::new(DisplayRow(3), 11)..DisplayPoint::new(DisplayRow(3), 13)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(1));
        });

        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.select_prev_match(&SelectPreviousMatch, window, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor
                    .selections
                    .display_ranges(&editor.display_snapshot(cx))),
                [DisplayPoint::new(DisplayRow(0), 41)..DisplayPoint::new(DisplayRow(0), 43)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        // Park the cursor in between matches and ensure that going to the previous match selects
        // the closest match to the left.
        editor.update_in(cx, |editor, window, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(DisplayRow(1), 0)..DisplayPoint::new(DisplayRow(1), 0)
                ])
            });
        });
        search_bar.update_in(cx, |search_bar, window, cx| {
            assert_eq!(search_bar.active_match_index, Some(1));
            search_bar.select_prev_match(&SelectPreviousMatch, window, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor
                    .selections
                    .display_ranges(&editor.display_snapshot(cx))),
                [DisplayPoint::new(DisplayRow(0), 41)..DisplayPoint::new(DisplayRow(0), 43)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        // Park the cursor in between matches and ensure that going to the next match selects the
        // closest match to the right.
        editor.update_in(cx, |editor, window, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(DisplayRow(1), 0)..DisplayPoint::new(DisplayRow(1), 0)
                ])
            });
        });
        search_bar.update_in(cx, |search_bar, window, cx| {
            assert_eq!(search_bar.active_match_index, Some(1));
            search_bar.select_next_match(&SelectNextMatch, window, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor
                    .selections
                    .display_ranges(&editor.display_snapshot(cx))),
                [DisplayPoint::new(DisplayRow(3), 11)..DisplayPoint::new(DisplayRow(3), 13)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(1));
        });

        // Park the cursor after the last match and ensure that going to the previous match selects
        // the last match.
        editor.update_in(cx, |editor, window, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(DisplayRow(3), 60)..DisplayPoint::new(DisplayRow(3), 60)
                ])
            });
        });
        search_bar.update_in(cx, |search_bar, window, cx| {
            assert_eq!(search_bar.active_match_index, Some(2));
            search_bar.select_prev_match(&SelectPreviousMatch, window, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor
                    .selections
                    .display_ranges(&editor.display_snapshot(cx))),
                [DisplayPoint::new(DisplayRow(3), 56)..DisplayPoint::new(DisplayRow(3), 58)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(2));
        });

        // Park the cursor after the last match and ensure that going to the next match selects the
        // first match.
        editor.update_in(cx, |editor, window, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(DisplayRow(3), 60)..DisplayPoint::new(DisplayRow(3), 60)
                ])
            });
        });
        search_bar.update_in(cx, |search_bar, window, cx| {
            assert_eq!(search_bar.active_match_index, Some(2));
            search_bar.select_next_match(&SelectNextMatch, window, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor
                    .selections
                    .display_ranges(&editor.display_snapshot(cx))),
                [DisplayPoint::new(DisplayRow(0), 41)..DisplayPoint::new(DisplayRow(0), 43)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        // Park the cursor before the first match and ensure that going to the previous match
        // selects the last match.
        editor.update_in(cx, |editor, window, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(0), 0)
                ])
            });
        });
        search_bar.update_in(cx, |search_bar, window, cx| {
            assert_eq!(search_bar.active_match_index, Some(0));
            search_bar.select_prev_match(&SelectPreviousMatch, window, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor
                    .selections
                    .display_ranges(&editor.display_snapshot(cx))),
                [DisplayPoint::new(DisplayRow(3), 56)..DisplayPoint::new(DisplayRow(3), 58)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
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

    #[perf]
    #[gpui::test]
    async fn test_search_option_handling(cx: &mut TestAppContext) {
        let (editor, search_bar, cx) = init_test(cx);

        // show with options should make current search case sensitive
        search_bar
            .update_in(cx, |search_bar, window, cx| {
                search_bar.show(window, cx);
                search_bar.search("us", Some(SearchOptions::CASE_SENSITIVE), true, window, cx)
            })
            .await
            .unwrap();
        editor.update_in(cx, |editor, window, cx| {
            assert_eq!(
                display_points_of(editor.all_text_background_highlights(window, cx)),
                &[DisplayPoint::new(DisplayRow(2), 43)..DisplayPoint::new(DisplayRow(2), 45),]
            );
        });

        // search_suggested should restore default options
        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.search_suggested(window, cx);
            assert_eq!(search_bar.search_options, SearchOptions::NONE)
        });

        // toggling a search option should update the defaults
        search_bar
            .update_in(cx, |search_bar, window, cx| {
                search_bar.search(
                    "regex",
                    Some(SearchOptions::CASE_SENSITIVE),
                    true,
                    window,
                    cx,
                )
            })
            .await
            .unwrap();
        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.toggle_search_option(SearchOptions::WHOLE_WORD, window, cx)
        });
        let mut editor_notifications = cx.notifications(&editor);
        editor_notifications.next().await;
        editor.update_in(cx, |editor, window, cx| {
            assert_eq!(
                display_points_of(editor.all_text_background_highlights(window, cx)),
                &[DisplayPoint::new(DisplayRow(0), 35)..DisplayPoint::new(DisplayRow(0), 40),]
            );
        });

        // defaults should still include whole word
        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.search_suggested(window, cx);
            assert_eq!(
                search_bar.search_options,
                SearchOptions::CASE_SENSITIVE | SearchOptions::WHOLE_WORD
            )
        });
    }

    #[perf]
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
            .filter(|c| c.eq_ignore_ascii_case(&'a'))
            .count();
        assert!(
            expected_query_matches_count > 1,
            "Should pick a query with multiple results"
        );
        let buffer = cx.new(|cx| Buffer::local(buffer_text, cx));
        let window = cx.add_window(|_, _| gpui::Empty);

        let editor = window.build_entity(cx, |window, cx| {
            Editor::for_buffer(buffer.clone(), None, window, cx)
        });

        let search_bar = window.build_entity(cx, |window, cx| {
            let mut search_bar = BufferSearchBar::new(None, window, cx);
            search_bar.set_active_pane_item(Some(&editor), window, cx);
            search_bar.show(window, cx);
            search_bar
        });

        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.search("a", None, true, window, cx)
                })
            })
            .unwrap()
            .await
            .unwrap();
        let initial_selections = window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    let handle = search_bar.query_editor.focus_handle(cx);
                    window.focus(&handle, cx);
                    search_bar.activate_current_match(window, cx);
                });
                assert!(
                    !editor.read(cx).is_focused(window),
                    "Initially, the editor should not be focused"
                );
                let initial_selections = editor.update(cx, |editor, cx| {
                    let initial_selections = editor.selections.display_ranges(&editor.display_snapshot(cx));
                    assert_eq!(
                        initial_selections.len(), 1,
                        "Expected to have only one selection before adding carets to all matches, but got: {initial_selections:?}",
                    );
                    initial_selections
                });
                search_bar.update(cx, |search_bar, cx| {
                    assert_eq!(search_bar.active_match_index, Some(0));
                    let handle = search_bar.query_editor.focus_handle(cx);
                    window.focus(&handle, cx);
                    search_bar.select_all_matches(&SelectAllMatches, window, cx);
                });
                assert!(
                    editor.read(cx).is_focused(window),
                    "Should focus editor after successful SelectAllMatches"
                );
                search_bar.update(cx, |search_bar, cx| {
                    let all_selections =
                        editor.update(cx, |editor, cx| editor.selections.display_ranges(&editor.display_snapshot(cx)));
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

                search_bar.update(cx, |this, cx| this.select_next_match(&SelectNextMatch, window, cx));
                initial_selections
            }).unwrap();

        window
            .update(cx, |_, window, cx| {
                assert!(
                    editor.read(cx).is_focused(window),
                    "Should still have editor focused after SelectNextMatch"
                );
                search_bar.update(cx, |search_bar, cx| {
                    let all_selections = editor.update(cx, |editor, cx| {
                        editor
                            .selections
                            .display_ranges(&editor.display_snapshot(cx))
                    });
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
                    window.focus(&handle, cx);
                    search_bar.select_all_matches(&SelectAllMatches, window, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, window, cx| {
                assert!(
                    editor.read(cx).is_focused(window),
                    "Should focus editor after successful SelectAllMatches"
                );
                search_bar.update(cx, |search_bar, cx| {
                    let all_selections =
                        editor.update(cx, |editor, cx| editor.selections.display_ranges(&editor.display_snapshot(cx)));
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
                    search_bar.select_prev_match(&SelectPreviousMatch, window, cx);
                });
            })
            .unwrap();
        let last_match_selections = window
            .update(cx, |_, window, cx| {
                assert!(
                    editor.read(cx).is_focused(window),
                    "Should still have editor focused after SelectPreviousMatch"
                );

                search_bar.update(cx, |search_bar, cx| {
                    let all_selections = editor.update(cx, |editor, cx| {
                        editor
                            .selections
                            .display_ranges(&editor.display_snapshot(cx))
                    });
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
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    let handle = search_bar.query_editor.focus_handle(cx);
                    window.focus(&handle, cx);
                    search_bar.search("abas_nonexistent_match", None, true, window, cx)
                })
            })
            .unwrap()
            .await
            .unwrap();
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.select_all_matches(&SelectAllMatches, window, cx);
                });
                assert!(
                    editor.update(cx, |this, _cx| !this.is_focused(window)),
                    "Should not switch focus to editor if SelectAllMatches does not find any matches"
                );
                search_bar.update(cx, |search_bar, cx| {
                    let all_selections =
                        editor.update(cx, |editor, cx| editor.selections.display_ranges(&editor.display_snapshot(cx)));
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

    #[perf]
    #[gpui::test]
    async fn test_search_query_with_match_whole_word(cx: &mut TestAppContext) {
        init_globals(cx);
        let buffer_text = r#"
        self.buffer.update(cx, |buffer, cx| {
            buffer.edit(
                edits,
                Some(AutoindentMode::Block {
                    original_indent_columns,
                }),
                cx,
            )
        });

        this.buffer.update(cx, |buffer, cx| {
            buffer.edit([(end_of_line..start_of_next_line, replace)], None, cx)
        });
        "#
        .unindent();
        let buffer = cx.new(|cx| Buffer::local(buffer_text, cx));
        let cx = cx.add_empty_window();

        let editor =
            cx.new_window_entity(|window, cx| Editor::for_buffer(buffer.clone(), None, window, cx));

        let search_bar = cx.new_window_entity(|window, cx| {
            let mut search_bar = BufferSearchBar::new(None, window, cx);
            search_bar.set_active_pane_item(Some(&editor), window, cx);
            search_bar.show(window, cx);
            search_bar
        });

        search_bar
            .update_in(cx, |search_bar, window, cx| {
                search_bar.search(
                    "edit\\(",
                    Some(SearchOptions::WHOLE_WORD | SearchOptions::REGEX),
                    true,
                    window,
                    cx,
                )
            })
            .await
            .unwrap();

        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.select_all_matches(&SelectAllMatches, window, cx);
        });
        search_bar.update(cx, |_, cx| {
            let all_selections = editor.update(cx, |editor, cx| {
                editor
                    .selections
                    .display_ranges(&editor.display_snapshot(cx))
            });
            assert_eq!(
                all_selections.len(),
                2,
                "Should select all `edit(` in the buffer, but got: {all_selections:?}"
            );
        });

        search_bar
            .update_in(cx, |search_bar, window, cx| {
                search_bar.search(
                    "edit(",
                    Some(SearchOptions::WHOLE_WORD | SearchOptions::CASE_SENSITIVE),
                    true,
                    window,
                    cx,
                )
            })
            .await
            .unwrap();

        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.select_all_matches(&SelectAllMatches, window, cx);
        });
        search_bar.update(cx, |_, cx| {
            let all_selections = editor.update(cx, |editor, cx| {
                editor
                    .selections
                    .display_ranges(&editor.display_snapshot(cx))
            });
            assert_eq!(
                all_selections.len(),
                2,
                "Should select all `edit(` in the buffer, but got: {all_selections:?}"
            );
        });
    }

    #[perf]
    #[gpui::test]
    async fn test_search_query_history(cx: &mut TestAppContext) {
        let (_editor, search_bar, cx) = init_test(cx);

        // Add 3 search items into the history.
        search_bar
            .update_in(cx, |search_bar, window, cx| {
                search_bar.search("a", None, true, window, cx)
            })
            .await
            .unwrap();
        search_bar
            .update_in(cx, |search_bar, window, cx| {
                search_bar.search("b", None, true, window, cx)
            })
            .await
            .unwrap();
        search_bar
            .update_in(cx, |search_bar, window, cx| {
                search_bar.search("c", Some(SearchOptions::CASE_SENSITIVE), true, window, cx)
            })
            .await
            .unwrap();
        // Ensure that the latest search is active.
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "c");
            assert_eq!(search_bar.search_options, SearchOptions::CASE_SENSITIVE);
        });

        // Next history query after the latest should set the query to the empty string.
        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.next_history_query(&NextHistoryQuery, window, cx);
        });
        cx.background_executor.run_until_parked();
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "");
            assert_eq!(search_bar.search_options, SearchOptions::CASE_SENSITIVE);
        });
        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.next_history_query(&NextHistoryQuery, window, cx);
        });
        cx.background_executor.run_until_parked();
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "");
            assert_eq!(search_bar.search_options, SearchOptions::CASE_SENSITIVE);
        });

        // First previous query for empty current query should set the query to the latest.
        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.previous_history_query(&PreviousHistoryQuery, window, cx);
        });
        cx.background_executor.run_until_parked();
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "c");
            assert_eq!(search_bar.search_options, SearchOptions::CASE_SENSITIVE);
        });

        // Further previous items should go over the history in reverse order.
        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.previous_history_query(&PreviousHistoryQuery, window, cx);
        });
        cx.background_executor.run_until_parked();
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "b");
            assert_eq!(search_bar.search_options, SearchOptions::CASE_SENSITIVE);
        });

        // Previous items should never go behind the first history item.
        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.previous_history_query(&PreviousHistoryQuery, window, cx);
        });
        cx.background_executor.run_until_parked();
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "a");
            assert_eq!(search_bar.search_options, SearchOptions::CASE_SENSITIVE);
        });
        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.previous_history_query(&PreviousHistoryQuery, window, cx);
        });
        cx.background_executor.run_until_parked();
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "a");
            assert_eq!(search_bar.search_options, SearchOptions::CASE_SENSITIVE);
        });

        // Next items should go over the history in the original order.
        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.next_history_query(&NextHistoryQuery, window, cx);
        });
        cx.background_executor.run_until_parked();
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "b");
            assert_eq!(search_bar.search_options, SearchOptions::CASE_SENSITIVE);
        });

        search_bar
            .update_in(cx, |search_bar, window, cx| {
                search_bar.search("ba", None, true, window, cx)
            })
            .await
            .unwrap();
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "ba");
            assert_eq!(search_bar.search_options, SearchOptions::NONE);
        });

        // New search input should add another entry to history and move the selection to the end of the history.
        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.previous_history_query(&PreviousHistoryQuery, window, cx);
        });
        cx.background_executor.run_until_parked();
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "c");
            assert_eq!(search_bar.search_options, SearchOptions::NONE);
        });
        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.previous_history_query(&PreviousHistoryQuery, window, cx);
        });
        cx.background_executor.run_until_parked();
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "b");
            assert_eq!(search_bar.search_options, SearchOptions::NONE);
        });
        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.next_history_query(&NextHistoryQuery, window, cx);
        });
        cx.background_executor.run_until_parked();
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "c");
            assert_eq!(search_bar.search_options, SearchOptions::NONE);
        });
        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.next_history_query(&NextHistoryQuery, window, cx);
        });
        cx.background_executor.run_until_parked();
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "ba");
            assert_eq!(search_bar.search_options, SearchOptions::NONE);
        });
        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.next_history_query(&NextHistoryQuery, window, cx);
        });
        cx.background_executor.run_until_parked();
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "");
            assert_eq!(search_bar.search_options, SearchOptions::NONE);
        });
    }

    #[perf]
    #[gpui::test]
    async fn test_replace_simple(cx: &mut TestAppContext) {
        let (editor, search_bar, cx) = init_test(cx);

        search_bar
            .update_in(cx, |search_bar, window, cx| {
                search_bar.search("expression", None, true, window, cx)
            })
            .await
            .unwrap();

        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.replacement_editor.update(cx, |editor, cx| {
                // We use $1 here as initially we should be in Text mode, where `$1` should be treated literally.
                editor.set_text("expr$1", window, cx);
            });
            search_bar.replace_all(&ReplaceAll, window, cx)
        });
        assert_eq!(
            editor.read_with(cx, |this, cx| { this.text(cx) }),
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
            .update_in(cx, |search_bar, window, cx| {
                search_bar.search("or", Some(SearchOptions::WHOLE_WORD), true, window, cx)
            })
            .await
            .unwrap();

        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.replacement_editor.update(cx, |editor, cx| {
                editor.set_text("banana", window, cx);
            });
            search_bar.replace_next(&ReplaceNext, window, cx)
        });
        // Notice how the first or in the text (shORtened) is not replaced. Neither are the remaining hits of `or` in the text.
        assert_eq!(
            editor.read_with(cx, |this, cx| { this.text(cx) }),
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
            .update_in(cx, |search_bar, window, cx| {
                search_bar.search(
                    "\\[([^\\]]+)\\]",
                    Some(SearchOptions::REGEX),
                    true,
                    window,
                    cx,
                )
            })
            .await
            .unwrap();
        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.replacement_editor.update(cx, |editor, cx| {
                editor.set_text("${1}number", window, cx);
            });
            search_bar.replace_all(&ReplaceAll, window, cx)
        });
        assert_eq!(
            editor.read_with(cx, |this, cx| { this.text(cx) }),
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
            .update_in(cx, |search_bar, window, cx| {
                search_bar.search(
                    "a\\w+s",
                    Some(SearchOptions::REGEX | SearchOptions::WHOLE_WORD),
                    true,
                    window,
                    cx,
                )
            })
            .await
            .unwrap();
        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.replacement_editor.update(cx, |editor, cx| {
                editor.set_text("things", window, cx);
            });
            search_bar.replace_all(&ReplaceAll, window, cx)
        });
        // The only word affected by this edit should be `algorithms`, even though there's a bunch
        // of words in this text that would match this regex if not for WHOLE_WORD.
        assert_eq!(
            editor.read_with(cx, |this, cx| { this.text(cx) }),
            r#"
        A regular expr$1 (shortened as regex banana regexp;1number also referred to as
        rational expr$12number3number) is a sequence of characters that specifies a search
        pattern in text. Usually such patterns are used by string-searching things
        for "find" or "find and replace" operations on strings, or for input validation.
        "#
            .unindent()
        );
    }

    #[gpui::test]
    async fn test_replace_focus(cx: &mut TestAppContext) {
        let (editor, search_bar, cx) = init_test(cx);

        editor.update_in(cx, |editor, window, cx| {
            editor.set_text("What a bad day!", window, cx)
        });

        search_bar
            .update_in(cx, |search_bar, window, cx| {
                search_bar.search("bad", None, true, window, cx)
            })
            .await
            .unwrap();

        // Calling `toggle_replace` in the search bar ensures that the "Replace
        // *" buttons are rendered, so we can then simulate clicking the
        // buttons.
        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.toggle_replace(&ToggleReplace, window, cx)
        });

        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.replacement_editor.update(cx, |editor, cx| {
                editor.set_text("great", window, cx);
            });
        });

        // Focus on the editor instead of the search bar, as we want to ensure
        // that pressing the "Replace Next Match" button will work, even if the
        // search bar is not focused.
        cx.focus(&editor);

        // We'll not simulate clicking the "Replace Next Match " button, asserting that
        // the replacement was done.
        let button_bounds = cx
            .debug_bounds("ICON-ReplaceNext")
            .expect("'Replace Next Match' button should be visible");
        cx.simulate_click(button_bounds.center(), gpui::Modifiers::none());

        assert_eq!(
            editor.read_with(cx, |editor, cx| editor.text(cx)),
            "What a great day!"
        );
    }

    struct ReplacementTestParams<'a> {
        editor: &'a Entity<Editor>,
        search_bar: &'a Entity<BufferSearchBar>,
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
            .update_in(options.cx, |search_bar, window, cx| {
                if let Some(options) = options.search_options {
                    search_bar.set_search_options(options, cx);
                }
                search_bar.search(
                    options.search_text,
                    options.search_options,
                    true,
                    window,
                    cx,
                )
            })
            .await
            .unwrap();

        options
            .search_bar
            .update_in(options.cx, |search_bar, window, cx| {
                search_bar.replacement_editor.update(cx, |editor, cx| {
                    editor.set_text(options.replacement_text, window, cx);
                });

                if options.replace_all {
                    search_bar.replace_all(&ReplaceAll, window, cx)
                } else {
                    search_bar.replace_next(&ReplaceNext, window, cx)
                }
            });

        assert_eq!(
            options
                .editor
                .read_with(options.cx, |this, cx| { this.text(cx) }),
            options.expected_text
        );
    }

    #[perf]
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

    #[perf]
    #[gpui::test]
    async fn test_find_matches_in_selections_singleton_buffer_multiple_selections(
        cx: &mut TestAppContext,
    ) {
        init_globals(cx);
        let buffer = cx.new(|cx| {
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
        let editor =
            cx.new_window_entity(|window, cx| Editor::for_buffer(buffer.clone(), None, window, cx));

        let search_bar = cx.new_window_entity(|window, cx| {
            let mut search_bar = BufferSearchBar::new(None, window, cx);
            search_bar.set_active_pane_item(Some(&editor), window, cx);
            search_bar.show(window, cx);
            search_bar
        });

        editor.update_in(cx, |editor, window, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                s.select_ranges(vec![Point::new(1, 0)..Point::new(2, 4)])
            })
        });

        search_bar.update_in(cx, |search_bar, window, cx| {
            let deploy = Deploy {
                focus: true,
                replace_enabled: false,
                selection_search_enabled: true,
            };
            search_bar.deploy(&deploy, window, cx);
        });

        cx.run_until_parked();

        search_bar
            .update_in(cx, |search_bar, window, cx| {
                search_bar.search("aaa", None, true, window, cx)
            })
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

    #[perf]
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
        let editor = cx.new_window_entity(|window, cx| {
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
            Editor::for_multibuffer(multibuffer, None, window, cx)
        });

        let search_bar = cx.new_window_entity(|window, cx| {
            let mut search_bar = BufferSearchBar::new(None, window, cx);
            search_bar.set_active_pane_item(Some(&editor), window, cx);
            search_bar.show(window, cx);
            search_bar
        });

        editor.update_in(cx, |editor, window, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                s.select_ranges(vec![
                    Point::new(1, 0)..Point::new(1, 4),
                    Point::new(5, 3)..Point::new(6, 4),
                ])
            })
        });

        search_bar.update_in(cx, |search_bar, window, cx| {
            let deploy = Deploy {
                focus: true,
                replace_enabled: false,
                selection_search_enabled: true,
            };
            search_bar.deploy(&deploy, window, cx);
        });

        cx.run_until_parked();

        search_bar
            .update_in(cx, |search_bar, window, cx| {
                search_bar.search("aaa", None, true, window, cx)
            })
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

    #[perf]
    #[gpui::test]
    async fn test_invalid_regexp_search_after_valid(cx: &mut TestAppContext) {
        let (editor, search_bar, cx) = init_test(cx);
        // Search using valid regexp
        search_bar
            .update_in(cx, |search_bar, window, cx| {
                search_bar.enable_search_option(SearchOptions::REGEX, window, cx);
                search_bar.search("expression", None, true, window, cx)
            })
            .await
            .unwrap();
        editor.update_in(cx, |editor, window, cx| {
            assert_eq!(
                display_points_of(editor.all_text_background_highlights(window, cx)),
                &[
                    DisplayPoint::new(DisplayRow(0), 10)..DisplayPoint::new(DisplayRow(0), 20),
                    DisplayPoint::new(DisplayRow(1), 9)..DisplayPoint::new(DisplayRow(1), 19),
                ],
            );
        });

        // Now, the expression is invalid
        search_bar
            .update_in(cx, |search_bar, window, cx| {
                search_bar.search("expression (", None, true, window, cx)
            })
            .await
            .unwrap_err();
        editor.update_in(cx, |editor, window, cx| {
            assert!(
                display_points_of(editor.all_text_background_highlights(window, cx)).is_empty(),
            );
        });
    }

    #[perf]
    #[gpui::test]
    async fn test_search_options_changes(cx: &mut TestAppContext) {
        let (_editor, search_bar, cx) = init_test(cx);
        update_search_settings(
            SearchSettings {
                button: true,
                whole_word: false,
                case_sensitive: false,
                include_ignored: false,
                regex: false,
                center_on_match: false,
            },
            cx,
        );

        let deploy = Deploy {
            focus: true,
            replace_enabled: false,
            selection_search_enabled: true,
        };

        search_bar.update_in(cx, |search_bar, window, cx| {
            assert_eq!(
                search_bar.search_options,
                SearchOptions::NONE,
                "Should have no search options enabled by default"
            );
            search_bar.toggle_search_option(SearchOptions::WHOLE_WORD, window, cx);
            assert_eq!(
                search_bar.search_options,
                SearchOptions::WHOLE_WORD,
                "Should enable the option toggled"
            );
            assert!(
                !search_bar.dismissed,
                "Search bar should be present and visible"
            );
            search_bar.deploy(&deploy, window, cx);
            assert_eq!(
                search_bar.search_options,
                SearchOptions::WHOLE_WORD,
                "After (re)deploying, the option should still be enabled"
            );

            search_bar.dismiss(&Dismiss, window, cx);
            search_bar.deploy(&deploy, window, cx);
            assert_eq!(
                search_bar.search_options,
                SearchOptions::WHOLE_WORD,
                "After hiding and showing the search bar, search options should be preserved"
            );

            search_bar.toggle_search_option(SearchOptions::REGEX, window, cx);
            search_bar.toggle_search_option(SearchOptions::WHOLE_WORD, window, cx);
            assert_eq!(
                search_bar.search_options,
                SearchOptions::REGEX,
                "Should enable the options toggled"
            );
            assert!(
                !search_bar.dismissed,
                "Search bar should be present and visible"
            );
            search_bar.toggle_search_option(SearchOptions::WHOLE_WORD, window, cx);
        });

        update_search_settings(
            SearchSettings {
                button: true,
                whole_word: false,
                case_sensitive: true,
                include_ignored: false,
                regex: false,
                center_on_match: false,
            },
            cx,
        );
        search_bar.update_in(cx, |search_bar, window, cx| {
            assert_eq!(
                search_bar.search_options,
                SearchOptions::REGEX | SearchOptions::WHOLE_WORD,
                "Should have no search options enabled by default"
            );

            search_bar.deploy(&deploy, window, cx);
            assert_eq!(
                search_bar.search_options,
                SearchOptions::REGEX | SearchOptions::WHOLE_WORD,
                "Toggling a non-dismissed search bar with custom options should not change the default options"
            );
            search_bar.dismiss(&Dismiss, window, cx);
            search_bar.deploy(&deploy, window, cx);
            assert_eq!(
                search_bar.configured_options,
                SearchOptions::CASE_SENSITIVE,
                "After a settings update and toggling the search bar, configured options should be updated"
            );
            assert_eq!(
                search_bar.search_options,
                SearchOptions::CASE_SENSITIVE,
                "After a settings update and toggling the search bar, configured options should be used"
            );
        });

        update_search_settings(
            SearchSettings {
                button: true,
                whole_word: true,
                case_sensitive: true,
                include_ignored: false,
                regex: false,
                center_on_match: false,
            },
            cx,
        );

        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.deploy(&deploy, window, cx);
            search_bar.dismiss(&Dismiss, window, cx);
            search_bar.show(window, cx);
            assert_eq!(
                search_bar.search_options,
                SearchOptions::CASE_SENSITIVE | SearchOptions::WHOLE_WORD,
                "Calling deploy on an already deployed search bar should not prevent settings updates from being detected"
            );
        });
    }

    #[gpui::test]
    async fn test_select_occurrence_case_sensitivity(cx: &mut TestAppContext) {
        let (editor, search_bar, cx) = init_test(cx);
        let mut editor_cx = EditorTestContext::for_editor_in(editor, cx).await;

        // Start with case sensitive search settings.
        let mut search_settings = SearchSettings::default();
        search_settings.case_sensitive = true;
        update_search_settings(search_settings, cx);
        search_bar.update(cx, |search_bar, cx| {
            let mut search_options = search_bar.search_options;
            search_options.insert(SearchOptions::CASE_SENSITIVE);
            search_bar.set_search_options(search_options, cx);
        });

        editor_cx.set_state("«ˇfoo»\nFOO\nFoo\nfoo");
        editor_cx.update_editor(|e, window, cx| {
            e.select_next(&Default::default(), window, cx).unwrap();
        });
        editor_cx.assert_editor_state("«ˇfoo»\nFOO\nFoo\n«ˇfoo»");

        // Update the search bar's case sensitivite toggle, so we can later
        // confirm that `select_next` will now be case-insensitive.
        editor_cx.set_state("«ˇfoo»\nFOO\nFoo\nfoo");
        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.toggle_case_sensitive(&Default::default(), window, cx);
        });
        editor_cx.update_editor(|e, window, cx| {
            e.select_next(&Default::default(), window, cx).unwrap();
        });
        editor_cx.assert_editor_state("«ˇfoo»\n«ˇFOO»\nFoo\nfoo");

        // Confirm that, after dismissing the search bar, only the editor's
        // search settings actually affect the behavior of `select_next`.
        search_bar.update_in(cx, |search_bar, window, cx| {
            search_bar.dismiss(&Default::default(), window, cx);
        });
        editor_cx.set_state("«ˇfoo»\nFOO\nFoo\nfoo");
        editor_cx.update_editor(|e, window, cx| {
            e.select_next(&Default::default(), window, cx).unwrap();
        });
        editor_cx.assert_editor_state("«ˇfoo»\nFOO\nFoo\n«ˇfoo»");

        // Update the editor's search settings, disabling case sensitivity, to
        // check that the value is respected.
        let mut search_settings = SearchSettings::default();
        search_settings.case_sensitive = false;
        update_search_settings(search_settings, cx);
        editor_cx.set_state("«ˇfoo»\nFOO\nFoo\nfoo");
        editor_cx.update_editor(|e, window, cx| {
            e.select_next(&Default::default(), window, cx).unwrap();
        });
        editor_cx.assert_editor_state("«ˇfoo»\n«ˇFOO»\nFoo\nfoo");
    }

    fn update_search_settings(search_settings: SearchSettings, cx: &mut TestAppContext) {
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.editor.search = Some(SearchSettingsContent {
                        button: Some(search_settings.button),
                        whole_word: Some(search_settings.whole_word),
                        case_sensitive: Some(search_settings.case_sensitive),
                        include_ignored: Some(search_settings.include_ignored),
                        regex: Some(search_settings.regex),
                        center_on_match: Some(search_settings.center_on_match),
                    });
                });
            });
        });
    }
}
