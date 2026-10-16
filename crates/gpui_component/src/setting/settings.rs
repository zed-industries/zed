use std::ops::Range;

use crate::{
    IconName, Sizable, Size, StyledExt,
    group_box::GroupBoxVariant,
    h_resizable,
    input::{Input, InputState},
    resizable_panel,
    setting::{SettingGroup, SettingPage},
    sidebar::{Sidebar, SidebarMenu, SidebarMenuItem},
};
use gpui::{
    App, AppContext as _, Axis, ElementId, Entity, IntoElement, ParentElement as _, Pixels,
    RenderOnce, SharedString, StyleRefinement, Styled, Window, container_query, div,
    prelude::FluentBuilder as _, px, relative,
};
use rust_i18n::t;

const STACKED_LAYOUT_MAX_WIDTH: Pixels = px(480.);

/// The settings structure containing multiple pages for app settings.
///
/// The hierarchy of settings is as follows:
///
/// ```ignore
/// Settings
///   SettingPage     <- The single active page displayed
///     SettingGroup
///       SettingItem
///         Label
///         SettingField (e.g., Switch, Dropdown, Input)
/// ```
#[derive(IntoElement)]
pub struct Settings {
    id: ElementId,
    pages: Vec<SettingPage>,
    group_variant: GroupBoxVariant,
    size: Size,
    sidebar_width: Pixels,
    sidebar_size_range: Range<Pixels>,
    sidebar_style: StyleRefinement,
    default_selected_index: SelectIndex,
    /// A request to show page `page_ix`, as `(request_number, page_ix)`:
    /// applied once per request number, so the sidebar still works afterwards.
    page_request: Option<(usize, usize)>,
    header_style: StyleRefinement,
}

impl Settings {
    /// Create a new settings with the given ID.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            pages: vec![],
            group_variant: GroupBoxVariant::default(),
            size: Size::default(),
            sidebar_width: px(250.0),
            sidebar_size_range: px(160.0)..px(360.0),
            sidebar_style: StyleRefinement::default(),
            default_selected_index: SelectIndex::default(),
            header_style: StyleRefinement::default(),
            page_request: None,
        }
    }

    /// Set the width of the sidebar, default is `250px`.
    pub fn sidebar_width(mut self, width: impl Into<Pixels>) -> Self {
        self.sidebar_width = width.into();
        self
    }

    /// Set the resize range of the sidebar, default is `160px..360px`.
    pub fn sidebar_size_range(mut self, range: impl Into<Range<Pixels>>) -> Self {
        self.sidebar_size_range = range.into();
        self
    }

    /// Add a page to the settings.
    pub fn page(mut self, page: SettingPage) -> Self {
        self.pages.push(page);
        self
    }

    /// Add pages to the settings.
    pub fn pages(mut self, pages: impl IntoIterator<Item = SettingPage>) -> Self {
        self.pages.extend(pages);
        self
    }

    /// Set the default variant for all setting groups.
    ///
    /// All setting groups will use this variant unless overridden individually.
    pub fn with_group_variant(mut self, variant: GroupBoxVariant) -> Self {
        self.group_variant = variant;
        self
    }

    /// Set the style refinement for the sidebar.
    pub fn sidebar_style(mut self, style: &StyleRefinement) -> Self {
        self.sidebar_style = style.clone();
        self
    }

    /// Set the default index of the page to be selected.
    pub fn default_selected_index(mut self, index: SelectIndex) -> Self {
        self.default_selected_index = index;
        self
    }
    /// Shows page `page_ix` when `request` carries a request number not
    /// applied before. Pass the same request again on every render: it is
    /// applied once, and the sidebar keeps working afterwards.
    pub fn select_page(mut self, request: Option<(usize, usize)>) -> Self {
        self.page_request = request;
        self
    }

    /// Set the style refinement for the header.
    pub fn header_style(mut self, style: &StyleRefinement) -> Self {
        self.header_style = style.clone();
        self
    }

    fn filtered_pages(&self, query: &str, cx: &App) -> Vec<SettingPage> {
        self.pages
            .iter()
            .filter_map(|page| {
                let filtered_groups: Vec<SettingGroup> = page
                    .groups
                    .iter()
                    .filter_map(|group| {
                        let mut group = group.clone();
                        group.items = group
                            .items
                            .iter()
                            .filter(|item| item.is_match(&query, cx))
                            .cloned()
                            .collect();
                        if group.items.is_empty() {
                            None
                        } else {
                            Some(group)
                        }
                    })
                    .collect();
                let mut page = page.clone();
                page.groups = filtered_groups;
                if page.groups.is_empty() {
                    None
                } else {
                    Some(page)
                }
            })
            .collect()
    }

    fn render_active_page(
        &self,
        state: &Entity<SettingsState>,
        pages: &Vec<SettingPage>,
        options: &RenderOptions,
        window: &mut Window,
        cx: &mut App,
    ) -> gpui::AnyElement {
        let selected_index = state.read(cx).selected_index;

        for (ix, page) in pages.into_iter().enumerate() {
            if selected_index.page_ix == ix {
                return page
                    .render(ix, state, &options, window, cx)
                    .into_any_element();
            }
        }

        return div().into_any_element();
    }

    fn render_sidebar(
        &self,
        state: &Entity<SettingsState>,
        pages: &Vec<SettingPage>,
        _: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        let selected_index = state.read(cx).selected_index;
        let search_input = state.read(cx).search_input.clone();

        Sidebar::new("settings-sidebar")
            .w(relative(1.))
            .border_0()
            .refine_style(&self.sidebar_style)
            .collapsible(false)
            .collapsed(false)
            .header(
                div()
                    .w_full()
                    .refine_style(&self.header_style)
                    .child(Input::new(&search_input).prefix(IconName::Search)),
            )
            .child(
                SidebarMenu::new().children(pages.iter().enumerate().map(|(page_ix, page)| {
                    let is_page_active =
                        selected_index.page_ix == page_ix && selected_index.group_ix.is_none();
                    SidebarMenuItem::new(page.title.clone())
                        .click_to_open(true)
                        .when_some(page.icon.clone(), |this, icon| this.icon(icon))
                        .default_open(page.default_open)
                        .when_some(page.sidebar_suffix.clone(), |this, suffix| {
                            this.suffix(move |window, cx| suffix(window, cx))
                        })
                        .active(is_page_active)
                        .on_click({
                            let state = state.clone();
                            move |_, _, cx| {
                                state.update(cx, |state, cx| {
                                    state.selected_index = SelectIndex {
                                        page_ix,
                                        ..Default::default()
                                    };
                                    cx.notify();
                                })
                            }
                        })
                        .when(page.groups.len() > 1, |this| {
                            this.children(
                                page.groups
                                    .iter()
                                    .filter(|g| g.title.is_some())
                                    .enumerate()
                                    .map(|(group_ix, group)| {
                                        let is_active = selected_index.page_ix == page_ix
                                            && selected_index.group_ix == Some(group_ix);
                                        let title = group.title.clone().unwrap_or_default();

                                        SidebarMenuItem::new(title).active(is_active).on_click({
                                            let state = state.clone();
                                            move |_, _, cx| {
                                                state.update(cx, |state, cx| {
                                                    state.selected_index = SelectIndex {
                                                        page_ix,
                                                        group_ix: Some(group_ix),
                                                    };
                                                    state.deferred_scroll_group_ix = Some(group_ix);
                                                    cx.notify();
                                                })
                                            }
                                        })
                                    }),
                            )
                        })
                })),
            )
    }
}

impl Sizable for Settings {
    fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }
}

pub(super) struct SettingsState {
    pub(super) selected_index: SelectIndex,
    /// If set, defer scrolling to this group index after rendering.
    pub(super) deferred_scroll_group_ix: Option<usize>,
    pub(super) search_input: Entity<InputState>,
    /// The request number of the last `select_page` request applied.
    pub(super) applied_page_request: Option<usize>,
}

/// Options for rendering setting item.
///
/// The fields are private and reached through the methods below, so that a new
/// one can be added without breaking the item renderers. The setters take
/// `self` by value, so a nested renderer narrows a copy of its parent options:
///
/// ```ignore
/// item.render_item(&options.with_item_ix(item_ix), window, cx)
/// ```
#[derive(Clone)]
pub struct RenderOptions {
    page_ix: usize,
    group_ix: usize,
    item_ix: usize,
    /// A stable identity for the current item, distinct from its positional
    /// `item_ix`. Fields use this (falling back to `item_ix` when empty) to
    /// key per-item render state such as a cached `InputState`. Positional
    /// indices alone are unsafe here: a `SettingGroup` built conditionally
    /// (e.g. different items depending on some selected type) can place
    /// unrelated fields at the same index across renders, which would
    /// otherwise make them share cached state and silently cross-wire their
    /// change handlers.
    item_key: SharedString,
    size: Size,
    group_variant: GroupBoxVariant,
    layout: Axis,
    disabled: bool,
}

impl RenderOptions {
    pub fn new() -> Self {
        Self {
            page_ix: 0,
            group_ix: 0,
            item_ix: 0,
            item_key: SharedString::default(),
            size: Size::default(),
            group_variant: GroupBoxVariant::default(),
            layout: Axis::Horizontal,
            disabled: false,
        }
    }

    pub fn with_page_ix(mut self, page_ix: usize) -> Self {
        self.page_ix = page_ix;
        self
    }

    pub fn with_group_ix(mut self, group_ix: usize) -> Self {
        self.group_ix = group_ix;
        self
    }

    pub fn with_item_ix(mut self, item_ix: usize) -> Self {
        self.item_ix = item_ix;
        self
    }

    /// Set the stable per-item key used for keying cached render state.
    pub fn with_item_key(mut self, item_key: impl Into<SharedString>) -> Self {
        self.item_key = item_key.into();
        self
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn with_group_variant(mut self, group_variant: GroupBoxVariant) -> Self {
        self.group_variant = group_variant;
        self
    }

    pub fn with_layout(mut self, layout: Axis) -> Self {
        self.layout = layout;
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn page_ix(&self) -> usize {
        self.page_ix
    }

    pub fn group_ix(&self) -> usize {
        self.group_ix
    }

    pub fn item_ix(&self) -> usize {
        self.item_ix
    }

    pub fn item_key(&self) -> &SharedString {
        &self.item_key
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn group_variant(&self) -> GroupBoxVariant {
        self.group_variant
    }

    pub fn layout(&self) -> Axis {
        self.layout
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled
    }
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Default)]
pub struct SelectIndex {
    pub page_ix: usize,
    pub group_ix: Option<usize>,
}

impl RenderOnce for Settings {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(self.id.clone(), cx, |window, cx| {
            let search_input = cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder(t!("Settings.search_placeholder"))
                    .default_value("")
            });

            SettingsState {
                search_input,
                selected_index: self.default_selected_index,
                deferred_scroll_group_ix: None,
                applied_page_request: None,
            }
        });

        if let Some((request, page_ix)) = self.page_request {
            state.update(cx, |state, _cx| {
                if state.applied_page_request != Some(request) {
                    state.applied_page_request = Some(request);
                    state.selected_index = SelectIndex {
                        page_ix,
                        group_ix: None,
                    };
                }
            });
        }

        let query = state.read(cx).search_input.read(cx).value();
        let filtered_pages = self.filtered_pages(&query, cx);
        let options = RenderOptions::new()
            .with_size(self.size)
            .with_group_variant(self.group_variant);
        let sidebar_size_range = self.sidebar_size_range.clone();
        let sidebar = self
            .render_sidebar(&state, &filtered_pages, window, cx)
            .into_any_element();

        h_resizable(self.id.clone())
            .child(
                resizable_panel()
                    .size(self.sidebar_width)
                    .size_range(sidebar_size_range)
                    .child(sidebar),
            )
            .child(
                resizable_panel().child(container_query(move |size, window, cx| {
                    let options = options.clone().with_layout(if size.width <= STACKED_LAYOUT_MAX_WIDTH {
                        Axis::Vertical
                    } else {
                        Axis::Horizontal
                    });
                    self.render_active_page(&state, &filtered_pages, &options, window, cx)
                })),
            )
    }
}
