//! The action catalog — a searchable, functionally grouped list of every
//! action the workflow engine knows how to run, rendered from
//! `workflow_engine::registry::REGISTRY`.
//!
//! This is a view over the registry, not a second source of truth: labels,
//! group headings, and the not-yet-runnable flag all come from the same
//! `ActionDef` the engine executes, so a newly registered action shows up here
//! with no catalog edit. `CatalogGroup::label()` supplies the section
//! headings and `ActionDef::runnable` supplies the disabled state.
//!
//! The list is a `gpui_component` [`ListState`] over [`CatalogDelegate`], not
//! a hand-rolled `div` stack: that gets the search box, arrow-key/Enter
//! navigation, `Role::List`/`Role::ListItem` + `aria_*` wiring, and
//! virtualized scrolling from the shared widget, and it is the only
//! composition that satisfies this repo's rule that panel lists be built from
//! real `gpui_component` widgets (see `AGENTS.md`).
//!
//! Three ways to add an action, all sharing the same registry metadata:
//! clicking a row (or pressing Enter on it), dragging a row onto the canvas
//! ([`CatalogDrag`]), and the canvas right-click quick-add menu in
//! `designer_panel.rs`, which groups itself from the same
//! [`sections`](build_sections) this module produces.

use gpui::{
    App, AppContext as _, IntoElement, ParentElement as _, Pixels, Point, Render, SharedString,
    StatefulInteractiveElement as _, Styled as _, Task, Window, div, px,
};
use gpui_component::{
    ActiveTheme as _, Icon, IconName, IndexPath, Sizable as _, h_flex,
    list::{ListDelegate, ListItem, ListState},
    tag::Tag,
    tooltip::Tooltip,
    v_flex,
};
use workflow_engine::registry::{CatalogGroup, REGISTRY};

/// Section order for the catalog. Deliberately a fixed list rather than
/// `REGISTRY` order: the registry is a flat slice whose order is an
/// implementation detail of that module, while "what can I do to control
/// this flow" is a browsing order worth pinning. Flow control first because
/// it is what a new flow is mostly made of; stubs sink to the bottom of their
/// group because they are not usable yet.
pub const GROUP_ORDER: [CatalogGroup; 5] = [
    CatalogGroup::FlowControl,
    CatalogGroup::Data,
    CatalogGroup::Integrations,
    CatalogGroup::Processes,
    CatalogGroup::Utilities,
];

/// Why a not-yet-runnable action can't be added, shown on the disabled row
/// and in the quick-add menu. Kept here (rather than inlined at the call
/// sites) so the catalog pane and the right-click menu give the same
/// explanation for the same action.
pub fn not_ready_reason(type_id: &str) -> String {
    match type_id {
        // FUTURE FEATURE — neither type is registered right now (see
        // `workflow_engine::registry::REGISTRY`), so these two arms are
        // unreachable. They are kept for when the entries come back as
        // `runnable: false`.
        "DbMigration" => {
            "DB Migration isn't wired to an executor yet — the action is registered and \
             validated, but running it is blocked until a database executor lands."
                .to_string()
        }
        "AiCheck" => "AI Check isn't wired to an executor yet — the action is registered and \
             validated, but running it is blocked until an AI executor lands."
            .to_string(),
        other => format!("{other} has no executor yet, so it can't be added to a flow yet."),
    }
}

/// One catalog row: a single `REGISTRY` entry, plus the pre-lowercased text
/// the search filter matches against.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CatalogEntry {
    /// The `ActionDef::type_id` this row adds — the value callers hand back
    /// to `DesignerPanel::add_action_at`.
    pub type_id: &'static str,
    pub label: SharedString,
    /// The section heading, from `CatalogGroup::label()`.
    pub group: &'static str,
    /// `ActionDef::runnable`: `false` renders the row disabled and
    /// undraggable, since a run-blocked action has nothing useful to add.
    pub runnable: bool,
    /// Lowercased `label + type_id + group`, built once at construction.
    haystack: String,
}

impl CatalogEntry {
    fn new(def: &workflow_engine::registry::ActionDef) -> Self {
        let group = def.catalog_group.label();
        Self {
            type_id: def.type_id,
            label: SharedString::from(def.label),
            group,
            runnable: def.runnable,
            haystack: format!("{} {} {}", def.label, def.type_id, group).to_lowercase(),
        }
    }

    /// Case-insensitive substring match against the row's label, type id, and
    /// group heading. Multi-term queries are ANDed, so "flow if" narrows to
    /// `If` rather than matching everything containing "flow" *or* "if".
    pub fn matches(&self, query: &str) -> bool {
        let query = query.trim().to_lowercase();
        if query.is_empty() {
            return true;
        }
        query
            .split_whitespace()
            .all(|term| self.haystack.contains(term))
    }
}

/// A titled run of entries — one `ListDelegate` section, rendered as a sticky
/// group heading above its rows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CatalogSection {
    pub title: SharedString,
    pub entries: Vec<CatalogEntry>,
}

/// Every registered action, grouped and ordered by [`GROUP_ORDER`]. Groups
/// with no members are omitted, so the catalog never renders an empty
/// heading.
pub fn build_sections() -> Vec<CatalogSection> {
    GROUP_ORDER
        .iter()
        .map(|group| CatalogSection {
            title: SharedString::from(group.label()),
            entries: REGISTRY
                .iter()
                .filter(|def| def.catalog_group == *group)
                .map(CatalogEntry::new)
                .collect(),
        })
        .filter(|section| !section.entries.is_empty())
        .collect()
}

/// The payload carried by a drag from a catalog row to the canvas. The
/// `type_id` is the whole contract — the drop site only needs to know which
/// action to insert and where.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CatalogDrag {
    pub type_id: &'static str,
}

/// The floating chip that follows the cursor during a catalog drag.
#[derive(Clone)]
pub struct CatalogDragPreview {
    label: SharedString,
    type_id: &'static str,
}

impl CatalogDragPreview {
    pub fn new(entry: &CatalogEntry) -> Self {
        Self {
            label: entry.label.clone(),
            type_id: entry.type_id,
        }
    }
}

impl Render for CatalogDragPreview {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .items_center()
            .px_2()
            .py_1()
            .rounded_md()
            .border_1()
            .border_color(cx.theme().primary)
            .bg(cx.theme().popover)
            .shadow_lg()
            .child(
                Icon::new(IconName::Plus)
                    .xsmall()
                    .text_color(cx.theme().primary),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(cx.theme().foreground)
                    .child(self.label.clone()),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child(self.type_id),
            )
    }
}

/// [`ListDelegate`] over the registry. Keeps the unfiltered `all` sections
/// alongside the `shown` view the search box narrows, so clearing the query
/// restores the full catalog without another registry walk.
pub struct CatalogDelegate {
    all: Vec<CatalogSection>,
    shown: Vec<CatalogSection>,
    query: String,
    /// Last index path the list highlighted, so the panel can tell "the
    /// selection moved" from "the same row was confirmed again".
    selected: Option<IndexPath>,
}

impl CatalogDelegate {
    pub fn new() -> Self {
        let all = build_sections();
        Self {
            shown: all.clone(),
            all,
            query: String::new(),
            selected: None,
        }
    }

    /// The entry a `ListEvent::Confirm`/`IndexPath` refers to, if it still
    /// exists in the current filtered view. The panel resolves the click back
    /// to a `type_id` through here rather than trusting the index, so a
    /// re-filtered list can never insert a different action than the one the
    /// user saw.
    pub fn entry_at(&self, ix: IndexPath) -> Option<&CatalogEntry> {
        self.shown
            .get(ix.section)
            .and_then(|section| section.entries.get(ix.row))
    }

    /// The `type_id` a confirmed row should add, or `None` if the row can't be
    /// added.
    ///
    /// This is the one place the "not ready" rule is enforced for the list, and
    /// it has to live here rather than relying on the row's appearance:
    /// `ListState` knows nothing about disabled items — `ListItem::disabled`
    /// only styles the row, and the list still selects and confirms it on click
    /// or Enter. So a `runnable: false` action has to be rejected explicitly or
    /// it lands on the canvas with no executor behind it, despite looking
    /// disabled.
    ///
    /// The drag payload and the right-click menu filter on the same `runnable`
    /// flag, so all three entry points agree.
    pub fn addable_type_id(&self, ix: IndexPath) -> Option<&'static str> {
        self.entry_at(ix)
            .filter(|entry| entry.runnable)
            .map(|entry| entry.type_id)
    }

    /// Apply `query` to the unfiltered sections, in place.
    ///
    /// Group headings survive a query that matches the heading itself (typing
    /// "data" keeps the Data section as a labelled result), but a group whose
    /// entries are all filtered out is dropped rather than left as an empty
    /// heading.
    pub fn apply_query(&mut self, query: &str) {
        self.query = query.trim().to_string();
        let query = &self.query;
        self.shown = if query.is_empty() {
            self.all.clone()
        } else {
            let query_lower = query.to_lowercase();
            self.all
                .iter()
                .filter_map(|section| {
                    let entries: Vec<CatalogEntry> = section
                        .entries
                        .iter()
                        .filter(|entry| entry.matches(query))
                        .cloned()
                        .collect();
                    let heading_matches = section.title.to_lowercase().contains(&query_lower);
                    if entries.is_empty() && !heading_matches {
                        return None;
                    }
                    Some(CatalogSection {
                        title: section.title.clone(),
                        entries,
                    })
                })
                .collect()
        };
    }

    /// How many of the catalog's entries are currently listed. Excludes
    /// section headings, so the panel's "N actions" hint doesn't count them.
    pub fn shown_count(&self) -> usize {
        self.shown.iter().map(|section| section.entries.len()).sum()
    }

    /// How many entries exist before any filtering.
    pub fn total_count(&self) -> usize {
        self.all.iter().map(|section| section.entries.len()).sum()
    }
}

impl Default for CatalogDelegate {
    fn default() -> Self {
        Self::new()
    }
}

impl ListDelegate for CatalogDelegate {
    type Item = ListItem;

    fn sections_count(&self, _cx: &App) -> usize {
        // `ListState` clamps this to at least 1 and skips empty sections, but
        // returning 0 for an empty filtered view keeps the row cache from
        // reserving space for a phantom first section.
        self.shown.len()
    }

    fn items_count(&self, section: usize, _cx: &App) -> usize {
        self.shown
            .get(section)
            .map_or(0, |section| section.entries.len())
    }

    fn render_item(
        &mut self,
        ix: IndexPath,
        _window: &mut Window,
        cx: &mut gpui::Context<ListState<Self>>,
    ) -> Option<Self::Item> {
        let entry = self.entry_at(ix)?.clone();

        let mut item = ListItem::new(ix)
            .disabled(!entry.runnable)
            .gap_2()
            .child(
                h_flex()
                    .min_w_0()
                    .flex_1()
                    .gap_1()
                    .items_center()
                    .child(
                        div()
                            .text_sm()
                            .text_color(cx.theme().foreground)
                            .child(entry.label.clone()),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child(entry.type_id),
                    ),
            )
            // A second badge in its own trailing slot: the shared `ListItem`
            // reserves `w_5` for its own check icon, so the tag needs an
            // explicit `suffix` rather than a third child or it will sit
            // under that gap.
            .suffix(move |_window, _cx| {
                if entry.runnable {
                    div().into_any_element()
                } else {
                    Tag::secondary()
                        .xsmall()
                        .child("Not ready")
                        .into_any_element()
                }
            });

        // The "no executor yet" explanation is built lazily, on hover, rather
        // than per row per frame — the registry is small today, but the
        // catalog re-renders on every keystroke in the search box.
        if !entry.runnable {
            let reason = not_ready_reason(entry.type_id);
            item = item.tooltip(move |window, cx| Tooltip::new(reason.clone()).build(window, cx));
        }

        // Only runnable rows start a drag: a not-ready action has no
        // executor, so there is nothing useful to place on the canvas.
        if entry.runnable {
            let drag = CatalogDrag {
                type_id: entry.type_id,
            };
            let preview = CatalogDragPreview::new(&entry);
            item = item.on_drag(drag, move |_, _, _, cx| cx.new(|_| preview.clone()));
        }

        Some(item)
    }

    fn render_section_header(
        &mut self,
        section: usize,
        _window: &mut Window,
        cx: &mut gpui::Context<ListState<Self>>,
    ) -> Option<impl gpui::IntoElement> {
        let section = self.shown.get(section)?;
        Some(
            v_flex()
                .h(px(28.0))
                .justify_center()
                .px_3()
                .mt_1()
                .child(
                    div()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child(section.title.clone()),
                )
                .into_any_element(),
        )
    }

    fn render_empty(
        &mut self,
        _window: &mut Window,
        cx: &mut gpui::Context<ListState<Self>>,
    ) -> impl IntoElement {
        let message: SharedString = if self.query.is_empty() {
            "No actions available.".into()
        } else {
            format!("No actions match “{}”.", self.query).into()
        };
        v_flex()
            .flex_1()
            .justify_center()
            .items_center()
            .gap_2()
            .px_4()
            .child(
                Icon::new(IconName::Inbox)
                    .size_12()
                    .text_color(cx.theme().muted_foreground),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(cx.theme().muted_foreground)
                    .text_center()
                    .child(message),
            )
            .into_any_element()
    }

    fn perform_search(
        &mut self,
        query: &str,
        _window: &mut Window,
        _cx: &mut gpui::Context<ListState<Self>>,
    ) -> Task<()> {
        self.apply_query(query);
        Task::ready(())
    }

    fn set_selected_index(
        &mut self,
        ix: Option<IndexPath>,
        _window: &mut Window,
        _cx: &mut gpui::Context<ListState<Self>>,
    ) {
        self.selected = ix;
    }
}

/// The catalog pane's header, shown above the list. Split out so the panel
/// keeps its own toolbar row and the pane still reads as a titled surface.
pub fn header(cx: &App, count: usize, total: usize) -> impl IntoElement {
    gpui_component::panel_header::PanelHeader::new("Actions")
        .icon(Icon::new(IconName::LayoutDashboard))
        .action(
            h_flex()
                .text_xs()
                .text_color(cx.theme().muted_foreground)
                .child(if count == total {
                    format!("{total}")
                } else {
                    format!("{count} of {total}")
                })
                .into_any_element(),
        )
        .into_any_element()
}

/// Where a `CatalogDrag` should be inserted, in flow space.
///
/// Split out from the drop handler so the placement arithmetic is testable
/// without a live canvas: the drop site supplies a window-absolute point, and
/// this is the one place that turns it into a node position.
pub fn drop_position(
    window_point: Point<Pixels>,
    panel_bounds: gpui::Bounds<Pixels>,
    viewport: gpui_flow::Viewport,
) -> gpui_flow::FlowPoint {
    viewport.screen_to_flow(
        (window_point.x - panel_bounds.origin.x).as_f32(),
        (window_point.y - panel_bounds.origin.y).as_f32(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use workflow_engine::registry::ActionDef;

    fn entry(type_id: &str, label: &str, group: CatalogGroup, runnable: bool) -> CatalogEntry {
        let def = ActionDef {
            type_id: Box::leak(type_id.to_string().into_boxed_str()),
            label: Box::leak(label.to_string().into_boxed_str()),
            category: workflow_engine::registry::ActionCategory::Leaf,
            catalog_group: group,
            runnable,
            inputs: &[],
            outputs: &[],
        };
        CatalogEntry::new(&def)
    }

    #[test]
    fn sections_are_grouped_in_declared_order() {
        let sections = build_sections();
        let titles: Vec<&str> = sections.iter().map(|s| s.title.as_ref()).collect();
        assert_eq!(
            titles,
            vec![
                "Flow Control",
                "Data",
                "Integrations",
                "Processes",
                "Utilities"
            ]
        );
        // Every registered action lands in exactly one section.
        assert_eq!(
            sections.iter().map(|s| s.entries.len()).sum::<usize>(),
            REGISTRY.len()
        );
    }

    #[test]
    fn every_registry_entry_appears_once_with_its_own_group() {
        let sections = build_sections();
        for def in REGISTRY {
            let matches: Vec<&CatalogEntry> = sections
                .iter()
                .flat_map(|section| section.entries.iter())
                .filter(|entry| entry.type_id == def.type_id)
                .collect();
            assert_eq!(matches.len(), 1, "duplicate row for {}", def.type_id);
            assert_eq!(matches[0].label.as_ref(), def.label);
            assert_eq!(matches[0].group, def.catalog_group.label());
            assert_eq!(matches[0].runnable, def.runnable);
        }
    }

    #[test]
    fn stub_actions_are_marked_not_runnable_with_a_reason() {
        let sections = build_sections();
        for def in REGISTRY.iter().filter(|def| !def.runnable) {
            let entry = sections
                .iter()
                .flat_map(|section| section.entries.iter())
                .find(|entry| entry.type_id == def.type_id)
                .expect("stub action is listed in the catalog");
            assert!(!entry.runnable);
            let reason = not_ready_reason(def.type_id);
            assert!(
                reason.contains("executor"),
                "reason should name the missing executor: {reason}"
            );
        }
    }

    #[test]
    /// A not-ready row is visible but must never resolve to something
    /// addable, or `ListState` — which ignores `ListItem::disabled` when
    /// confirming — would insert an action with no executor behind it.
    // #[test]
    fn not_ready_rows_are_visible_but_never_addable() {
        let delegate = CatalogDelegate::new();
        let mut not_ready = 0;
        let mut runnable = 0;
        for (section_ix, section) in delegate.shown.iter().enumerate() {
            for (row_ix, entry) in section.entries.iter().enumerate() {
                let ix = IndexPath::new(row_ix).section(section_ix);
                assert_eq!(
                    delegate.entry_at(ix).map(|entry| entry.type_id),
                    Some(entry.type_id),
                    "every visible row resolves to itself"
                );
                if entry.runnable {
                    runnable += 1;
                    assert_eq!(delegate.addable_type_id(ix), Some(entry.type_id));
                } else {
                    not_ready += 1;
                    assert_eq!(
                        delegate.addable_type_id(ix),
                        None,
                        "{} is not ready and must not be addable",
                        entry.type_id
                    );
                }
            }
        }
        // FUTURE FEATURE — the registry has no stubs while `DbMigration` and
        // `AiCheck` are parked, so the not-ready half of this test is
        // vacuous. Restore the guard when a `runnable: false` entry returns.
        //
        // assert!(not_ready > 0, "expected some not-ready rows");
        let _ = not_ready;
        assert!(runnable > 0, "expected some runnable rows");
    }

    // FUTURE FEATURE — `DbMigration` and `AiCheck` are parked in
    // `workflow_engine::registry::REGISTRY`, so they are no longer listed at
    // all. Restore this test with the entries.
    //
    // /// The two actions Phase 3 calls out by name are the ones that must stay
    // /// visible-and-disabled, so pin them explicitly rather than relying on
    // /// "some stub exists".
    // #[test]
    // fn db_migration_and_ai_check_are_listed_but_not_addable() {
    //     for (type_id, label) in [("DbMigration", "DB Migration"), ("AiCheck", "AI Check")] {
    //         let delegate = CatalogDelegate::new();
    //         let (section_ix, row_ix) = delegate
    //             .shown
    //             .iter()
    //             .enumerate()
    //             .find_map(|(section_ix, section)| {
    //                 section
    //                     .entries
    //                     .iter()
    //                     .position(|entry| entry.type_id == type_id)
    //                     .map(|row_ix| (section_ix, row_ix))
    //             })
    //             .unwrap_or_else(|| panic!("{type_id} should be listed in the catalog"));
    //         let ix = IndexPath::new(row_ix).section(section_ix);
    //         let entry = delegate.entry_at(ix).expect("listed");
    //         // Pin the display name too: the requirement names the actions by
    //         // their labels, and a label drift should fail here rather than
    //         // silently make this test vacuous.
    //         assert_eq!(entry.label.as_ref(), label);
    //         assert_eq!(
    //             delegate.addable_type_id(ix),
    //             None,
    //             "{type_id} is not addable"
    //         );
    //         assert!(!not_ready_reason(type_id).is_empty());
    //     }
    // }

    #[test]
    fn parked_future_actions_are_not_listed() {
        let delegate = CatalogDelegate::new();
        for type_id in ["sendEmail", "DbMigration", "AiCheck"] {
            assert!(
                delegate
                    .shown
                    .iter()
                    .flat_map(|section| section.entries.iter())
                    .all(|entry| entry.type_id != type_id),
                "{type_id} is parked and should not be in the catalog"
            );
        }
    }

    /// A re-filter must not let a stale index name a different action than the
    /// one on screen, which is why the panel resolves through the delegate.
    #[test]
    fn indices_are_resolved_against_the_filtered_view_not_the_registry() {
        let mut delegate = CatalogDelegate::new();
        delegate.apply_query("http");
        assert!(delegate.shown_count() < delegate.total_count());
        for (section_ix, section) in delegate.shown.iter().enumerate() {
            for (row_ix, entry) in section.entries.iter().enumerate() {
                let ix = IndexPath::new(row_ix).section(section_ix);
                let resolved = delegate.entry_at(ix).expect("visible row resolves");
                assert_eq!(resolved.type_id, entry.type_id);
            }
        }
    }

    #[test]
    fn empty_query_keeps_everything() {
        let mut delegate = CatalogDelegate::new();
        delegate.apply_query("");
        assert_eq!(delegate.shown_count(), delegate.total_count());
        assert_eq!(delegate.total_count(), REGISTRY.len());
    }

    #[test]
    fn query_matches_label_type_id_and_group_case_insensitively() {
        let http = entry("http", "HTTP Request", CatalogGroup::Integrations, true);
        assert!(http.matches("http"));
        assert!(http.matches("HTTP"));
        assert!(http.matches("request"));
        assert!(http.matches("INTEGRATIONS"));
        assert!(http.matches("integrat"));
        assert!(!http.matches("nope"));
    }

    #[test]
    fn multiple_terms_are_anded() {
        let sections = build_sections();
        let delegate_all = sections
            .iter()
            .flat_map(|section| section.entries.iter())
            .collect::<Vec<_>>();
        let http = delegate_all
            .iter()
            .find(|e| e.type_id == "http")
            .expect("http is registered");
        // Both terms hit, so it matches...
        assert!(http.matches("http request"));
        assert!(http.matches("integrations request"));
        // ...but a term that doesn't excludes it, rather than the pair
        // matching on the strength of one term alone.
        assert!(!http.matches("http email"));
    }

    #[test]
    fn filtering_narrows_sections_and_drops_empties() {
        let mut delegate = CatalogDelegate::new();
        delegate.apply_query("http");
        assert!(delegate.shown_count() < delegate.total_count());
        assert!(delegate.shown_count() > 0);
        // "http" lives in Integrations, and nothing matches elsewhere, so
        // exactly one section survives.
        assert_eq!(delegate.shown.len(), 1);
        assert_eq!(delegate.shown[0].title.as_ref(), "Integrations");
    }

    #[test]
    fn a_group_heading_match_keeps_that_group_listed() {
        let mut delegate = CatalogDelegate::new();
        delegate.apply_query("utilities");
        assert_eq!(delegate.shown.len(), 1);
        assert_eq!(delegate.shown[0].title.as_ref(), "Utilities");
        // delay, log, SetEnv. It was 5 while `DbMigration` and `AiCheck` were
        // registered; they are parked as future features.
        assert_eq!(delegate.shown[0].entries.len(), 3);
    }

    #[test]
    fn a_no_match_query_empties_the_list() {
        let mut delegate = CatalogDelegate::new();
        delegate.apply_query("zzzz-no-such-action");
        assert_eq!(delegate.shown_count(), 0);
        assert!(delegate.shown.is_empty());
        // A blank query after a failed one restores the full catalog.
        delegate.apply_query("");
        assert_eq!(delegate.shown_count(), delegate.total_count());
    }

    #[test]
    fn entry_at_resolves_a_visible_index_path_to_a_type_id() {
        let delegate = CatalogDelegate::new();
        let ix = IndexPath::default().section(1).row(0);
        let resolved = delegate.entry_at(ix).map(|entry| entry.type_id);
        let expected = build_sections()[1].entries[0].type_id;
        assert_eq!(resolved, Some(expected));
        // Out-of-range paths resolve to nothing rather than panicking.
        assert!(
            delegate
                .entry_at(IndexPath::default().section(99))
                .is_none()
        );
        assert!(
            delegate
                .entry_at(IndexPath::default().section(0).row(99))
                .is_none()
        );
    }

    #[test]
    fn drop_position_maps_a_window_point_into_flow_space() {
        // Unpanned, unzoomed: the conversion is then just "window minus the
        // canvas origin", which is the case worth pinning exactly.
        let viewport = gpui_flow::Viewport {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
        };
        // A panel whose top-left sits 40/20 px into the window.
        let bounds = gpui::Bounds::new(
            Point::new(px(40.0), px(20.0)),
            gpui::size(px(600.0), px(400.0)),
        );
        // A point 200 px right and 100 px below the canvas origin.
        let window_point = Point::new(px(240.0), px(120.0));
        let flow = drop_position(window_point, bounds, viewport);
        assert_eq!(flow.x, 200.0);
        assert_eq!(flow.y, 100.0);
    }

    #[test]
    fn drop_position_accounts_for_pan_and_zoom() {
        // Panned and zoomed in, the same window point resolves to a smaller
        // flow coordinate: `screen_to_flow` removes the pan before dividing by
        // the zoom, exactly as every other canvas hit-test does.
        let viewport = gpui_flow::Viewport {
            x: 100.0,
            y: 50.0,
            zoom: 2.0,
        };
        let bounds = gpui::Bounds::new(
            Point::new(px(40.0), px(20.0)),
            gpui::size(px(600.0), px(400.0)),
        );
        // Canvas-local (400, 200) -> (400-100)/2 = 150, (200-50)/2 = 75.
        let window_point = Point::new(px(440.0), px(220.0));
        let flow = drop_position(window_point, bounds, viewport);
        assert_eq!(flow.x, 150.0);
        assert_eq!(flow.y, 75.0);
    }

    #[test]
    fn drop_position_matches_the_flow_graphs_own_conversion() {
        // The catalog's arithmetic and `FlowGraph::flow_point_at` must agree,
        // or a dropped node lands somewhere other than where the user let go
        // while every other interaction on the canvas is consistent.
        let viewport = gpui_flow::Viewport {
            x: -30.0,
            y: 70.0,
            zoom: 0.75,
        };
        let origin = Point::new(px(16.0), px(64.0));
        let window_point = Point::new(px(311.0), px(205.0));

        let from_catalog = drop_position(
            window_point,
            gpui::Bounds::new(origin, gpui::size(px(800.0), px(600.0))),
            viewport,
        );

        // Reproduce `FlowGraph::flow_point_at`, which subtracts the panel
        // origin and delegates to `Viewport::screen_to_flow`.
        let from_graph = viewport.screen_to_flow(
            (window_point.x - origin.x).as_f32(),
            (window_point.y - origin.y).as_f32(),
        );
        assert_eq!(from_catalog.x, from_graph.x);
        assert_eq!(from_catalog.y, from_graph.y);
    }
}
