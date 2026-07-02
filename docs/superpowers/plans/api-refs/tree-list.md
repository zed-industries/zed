# Rendering a collapsible tree list in a Zed panel + panel chrome (uniform_list, ui::ListItem, expanded-state tracking, header/toolbar IconButtons, right-click ContextMenu, error colors, Tooltip)

# Zed panel tree-list + chrome API reference

All paths absolute; line numbers from current `main` (bb48a42983).

---

## 1. Scrollable list: `uniform_list`

### Exact signature — /Users/user/zed/crates/gpui/src/elements/uniform_list.rs:22

```rust
#[track_caller]
pub fn uniform_list<R>(
    id: impl Into<ElementId>,
    item_count: usize,
    f: impl 'static + Fn(Range<usize>, &mut Window, &mut App) -> Vec<R>,
) -> UniformList
where
    R: IntoElement,
```

Builder methods on `UniformList` (same file):
- `.track_scroll(&UniformListScrollHandle) -> Self` (line 683)
- `.with_sizing_behavior(ListSizingBehavior) -> Self` (line 628) — e.g. `ListSizingBehavior::Infer`
- `.with_horizontal_sizing_behavior(ListHorizontalSizingBehavior) -> Self` (line 636) — `Unconstrained` / `FitList`
- `.with_decoration(impl UniformListDecoration + 'static) -> Self` (line 653) — used for `ui::indent_guides(...)` and `ui::sticky_items(...)`
- `.with_width_from_item(Option<usize>) -> Self` (line 622)

`UniformListScrollHandle` (line ~101, `#[derive(Clone, Debug, Default)] pub struct UniformListScrollHandle(pub Rc<RefCell<UniformListScrollState>>)`), methods: `scroll_to_item(ix, ScrollStrategy)` (150), `scroll_to_item_strict(ix, ScrollStrategy)` (163), `scroll_to_item_strict_with_offset(ix, strategy, offset)` (201). Store as a field: `scroll_handle: UniformListScrollHandle` (project_panel.rs:141).

### The `cx.processor` helper for the render closure — /Users/user/zed/crates/gpui/src/app/context.rs:264

```rust
pub fn processor<E, R>(
    &self,
    f: impl Fn(&mut T, E, &mut Window, &mut Context<T>) -> R + 'static,
) -> impl Fn(E, &mut Window, &mut App) -> R + 'static
```

### Real usage A — project_panel — /Users/user/zed/crates/project_panel/src/project_panel.rs:6996

```rust
uniform_list("entries", item_count, {
    cx.processor(|this, range: Range<usize>, window, cx| {
        this.rendered_entries_len = range.end - range.start;
        let mut items = Vec::with_capacity(this.rendered_entries_len);
        let marked_selections: Arc<[SelectedEntry]> = Arc::from(this.marked_entries.clone());
        this.for_each_visible_entry(range, window, cx, &mut |id, details, window, cx| {
            items.push(this.render_entry(id, details, Arc::clone(&marked_selections), window, cx));
        });
        items
    })
})
.when(show_indent_guides, |list| {
    list.with_decoration(
        ui::indent_guides(px(indent_size), IndentGuideColors::panel(cx))
            .with_compute_indents_fn(cx.entity(), |this, range, window, cx| { /* SmallVec<usize> of depths */ })
            .with_render_fn(cx.entity(), move |this, params, _, cx| { /* Vec<ui::RenderedIndentGuide> */ }),
    )
})
.with_sizing_behavior(ListSizingBehavior::Infer)
.with_horizontal_sizing_behavior(if horizontal_scroll {
    ListHorizontalSizingBehavior::Unconstrained
} else {
    ListHorizontalSizingBehavior::FitList
})
.when(horizontal_scroll, |list| list.with_width_from_item(self.state.max_width_item_index))
.track_scroll(&self.scroll_handle),
```
(full indent-guide render fn: lines 7019–7135; sticky-header decoration `ui::sticky_items(...)`: 7137–7209)

### Real usage B — git_panel (tree + flat modes) — /Users/user/zed/crates/git_ui/src/git_panel.rs:6275

```rust
uniform_list(
    "entries",
    entry_count,
    cx.processor(move |this, range: Range<usize>, window, cx| {
        let Some(repo) = repo.upgrade() else { return Vec::new(); };
        let repo = repo.read(cx);
        let mut items = Vec::with_capacity(range.end - range.start);
        for ix in range.into_iter().map(|ix| match &this.view_mode {
            GitPanelViewMode::Tree(state) => state.logical_indices[ix],
            GitPanelViewMode::Flat => ix,
        }) {
            match &this.entries.get(ix) {
                Some(GitListEntry::Status(entry)) => items.push(this.render_status_entry(ix, entry, 0, has_write_access, repo, window, cx)),
                Some(GitListEntry::TreeStatus(entry)) => items.push(this.render_status_entry(ix, &entry.entry, entry.depth, has_write_access, repo, window, cx)),
                Some(GitListEntry::Directory(entry)) => items.push(this.render_directory_entry(ix, entry, has_write_access, window, cx)),
                Some(GitListEntry::Header(header)) => items.push(this.render_list_header(ix, header, has_write_access, window, cx)),
                None => {}
            }
        }
        items
    }),
)
.when(is_tree_view, |list| {
    let indent_size = px(TREE_INDENT);
    list.with_decoration(
        ui::indent_guides(indent_size, IndentGuideColors::panel(cx))
            .with_compute_indents_fn(cx.entity(), |this, range, _window, _cx| this.compute_visible_depths(range))
            .with_render_fn(cx.entity(), |_, params, _, _| { /* ... */ }),
    )
})
```

Key pattern: the tree is **flattened into a Vec of visible entries** before render; `item_count` is the number of *visible* rows, and the closure renders only the requested `range`.

---

## Expanded/collapsed state data structures

**project_panel** (/Users/user/zed/crates/project_panel/src/project_panel.rs:101–113):
```rust
struct State {
    last_worktree_root_id: Option<ProjectEntryId>,
    ancestors: HashMap<ProjectEntryId, FoldedAncestors>,
    visible_entries: Vec<VisibleEntriesForWorktree>,   // flattened list fed to uniform_list
    max_width_item_index: Option<usize>,
    edit_state: Option<EditState>,
    temporarily_unfolded_pending_state: Option<TemporaryUnfoldedPendingState>,
    unfolded_dir_ids: HashSet<ProjectEntryId>,
    expanded_dir_ids: HashMap<WorktreeId, Vec<ProjectEntryId>>, // SORTED Vec, binary_search
}
```

Toggle (project_panel.rs:1567–1591):
```rust
fn toggle_expanded(&mut self, entry_id: ProjectEntryId, window: &mut Window, cx: &mut Context<Self>) {
    if let Some(worktree_id) = self.project.read(cx).worktree_id_for_entry(entry_id, cx)
        && let Some(expanded_dir_ids) = self.state.expanded_dir_ids.get_mut(&worktree_id)
    {
        self.project.update(cx, |project, cx| {
            match expanded_dir_ids.binary_search(&entry_id) {
                Ok(ix) => { expanded_dir_ids.remove(ix); }
                Err(ix) => {
                    project.expand_entry(worktree_id, entry_id, cx);
                    expanded_dir_ids.insert(ix, entry_id);
                }
            }
        });
        self.update_visible_entries(Some((worktree_id, entry_id)), false, false, window, cx); // re-flatten
        window.focus(&self.focus_handle, cx);
        cx.notify();
    }
}
```

**outline_panel** uses the inverse: `collapsed_entries: HashSet<CollapsedEntry>` (outline_panel.rs:127), where (outline_panel.rs:338):
```rust
enum CollapsedEntry {
    Dir(WorktreeId, ProjectEntryId),
    File(WorktreeId, BufferId),
    ExternalFile(BufferId),
    Excerpt(ExcerptRange<Anchor>),
    Outline(Range<Anchor>),
}
```
Expanded check: `let is_expanded = !self.collapsed_entries.contains(&CollapsedEntry::Dir(worktree_id, entry.id));` (outline_panel.rs:2381). Toggle = `if !self.collapsed_entries.remove(&e) { self.collapsed_entries.insert(e); }` (e.g. outline_panel.rs:1860–1869).

**git_panel** keeps `expanded: bool` on each `GitTreeDirEntry` plus `state.logical_indices: Vec<usize>` mapping visible row -> entry index.

---

## Entry rendering with `ui::ListItem`

### API — /Users/user/zed/crates/ui/src/components/list/list_item.rs

`pub struct ListItem` (line 26), `#[derive(IntoElement)]`. Constructor and builders (lines 62–245):
```rust
pub fn new(id: impl Into<ElementId>) -> Self                 // defaults: indent_step_size: px(12.), spacing: ListItemSpacing::Dense, selectable: true
pub fn spacing(mut self, spacing: ListItemSpacing) -> Self   // Dense | ExtraDense | Sparse
pub fn selectable(mut self, has_hover: bool) -> Self
pub fn always_show_disclosure_icon(mut self, show: bool) -> Self
pub fn on_click(...)                                          // Fn(&ClickEvent, &mut Window, &mut App)
pub fn on_secondary_mouse_down(...)                           // Fn(&MouseDownEvent, &mut Window, &mut App)
pub fn tooltip(mut self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self
pub fn inset(mut self, inset: bool) -> Self
pub fn indent_level(mut self, indent_level: usize) -> Self
pub fn indent_step_size(mut self, indent_step_size: Pixels) -> Self
pub fn toggle(mut self, toggle: impl Into<Option<bool>>) -> Self   // Some(true/false) shows disclosure chevron (open/closed); None = no chevron
pub fn on_toggle(...)                                         // chevron click handler: Fn(&ClickEvent, &mut Window, &mut App)
pub fn start_slot<E: IntoElement>(mut self, start_slot: impl Into<Option<E>>) -> Self
pub fn end_slot<E: IntoElement>(mut self, end_slot: impl Into<Option<E>>) -> Self
pub fn end_slot_on_hover<E: IntoElement>(...) / pub fn show_end_slot_on_hover(mut self) -> Self
pub fn height(mut self, height: impl Into<DefiniteLength>) -> Self
```
Also `impl Toggleable for ListItem { fn toggle_state(mut self, selected: bool) -> Self }` (line 257) — `toggle_state` = *selected/active highlight*, `toggle` = *disclosure chevron state*. The chevron is `ui::Disclosure` internally: `Disclosure::new(id, is_open)` with `.on_toggle_expanded(...)`, `.opened_icon(IconName)`, `.closed_icon(IconName)` (/Users/user/zed/crates/ui/src/components/disclosure.rs:22–56).

### Canonical tree row with chevron — collab_panel channel row — /Users/user/zed/crates/collab_ui/src/collab_panel.rs:3328

```rust
ListItem::new(ix)
    .height(height)
    // Add one level of depth for the disclosure arrow.
    .indent_level(depth + 1)
    .indent_step_size(px(20.))
    .toggle_state(is_selected || is_active)
    .toggle(disclosed)                       // Option<bool>: Some(expanded?)
    .on_toggle(cx.listener(move |this, _, window, cx| {
        this.toggle_channel_collapsed(channel_id, window, cx)
    }))
    .on_click(cx.listener(move |this, _, window, cx| { /* open/join */ }))
    .on_secondary_mouse_down(cx.listener(
        move |this, event: &MouseDownEvent, window, cx| {
            this.deploy_channel_context_menu(event.position, channel_id, ix, window, cx)
        },
    ))
    .child(h_flex()/* icon + label */)
```

### project_panel row (indent + icon + label + end_slot badges) — project_panel.rs:5544 `fn render_entry(&self, entry_id: ProjectEntryId, details: EntryDetails, marked_selections: Arc<[SelectedEntry]>, window: &mut Window, cx: &mut Context<Self>) -> Stateful<Div>`

Row = outer `div().id(id).group(GROUP_NAME).bg(...).border_1().border_r_2().hover(...)` wrapping a `ListItem` (project_panel.rs:6012):
```rust
ListItem::new(id)
    .indent_level(depth)
    .indent_step_size(px(settings.indent_size))
    .spacing(match settings.entry_spacing {
        ProjectPanelEntrySpacing::Comfortable => ListItemSpacing::Dense,
        ProjectPanelEntrySpacing::Standard => ListItemSpacing::ExtraDense,
    })
    .selectable(false)
    .end_slot::<AnyElement>(h_flex().gap_1().flex_none().pr_3()/* diagnostics + git badges */.into_any_element())
    .child(/* file/folder Icon::from_path(icon).color(Color::Muted) */)
    .child(h_flex().h_6().child(Label::new(file_name).single_line().color(filename_text_color)))
    .on_secondary_mouse_down(cx.listener(move |this, event: &MouseDownEvent, window, cx| {
        cx.stop_propagation();
        // ... this.deploy_context_menu(event.position, entry_id, window, cx)
    }))
```
Directory expand-on-click lives on the outer div's `.on_click` (project_panel.rs:5970–6000): `if kind.is_dir() { if event.modifiers().alt { project_panel.toggle_expand_all(entry_id, window, cx); } else { project_panel.toggle_expanded(entry_id, window, cx); } } else { project_panel.open_entry(entry_id, focus_opened_item, allow_preview, cx); }`

Folder icon flip (expanded vs collapsed) — git_panel.rs:6864:
```rust
let folder_icon = if settings.folder_icons {
    FileIcons::get_folder_icon(entry.expanded, entry.key.path.as_std_path(), cx)
} else {
    FileIcons::get_chevron_icon(entry.expanded, cx)   // ChevronDown / ChevronRight paths
};
```

### outline_panel row wrapper — outline_panel.rs:2620 `fn entry_element(&self, rendered_entry: PanelEntry, item_id: ElementId, depth: usize, icon_element: AnyElement, is_active: bool, label_element: gpui::AnyElement, window: &mut Window, cx: &mut Context<OutlinePanel>) -> Stateful<Div>`

```rust
div().text_ui(cx).id(item_id.clone())
    .on_click({
        let clicked_entry = rendered_entry.clone();
        cx.listener(move |outline_panel, event: &gpui::ClickEvent, window, cx| {
            if event.is_right_click() || event.first_focus() { return; }
            outline_panel.toggle_expanded(&clicked_entry, window, cx);
            // ...
        })
    })
    .cursor_pointer()
    .child(
        ListItem::new(item_id)
            .indent_level(depth)
            .indent_step_size(px(settings.indent_size))
            .toggle_state(is_active)
            .child(h_flex()
                .child(h_flex().w(px(16.)).justify_center().child(icon_element))
                .child(h_flex().h_6().child(label_element).ml_1()))
            .on_secondary_mouse_down(cx.listener(move |outline_panel, event: &MouseDownEvent, window, cx| {
                cx.stop_propagation();
                outline_panel.deploy_context_menu(event.position, rendered_entry.clone(), window, cx)
            })),
    )
    .border_1().border_r_2().rounded_none()
    .hover(|style| if is_active { style } else {
        let hover_color = cx.theme().colors().ghost_element_hover;
        style.bg(hover_color).border_color(hover_color)
    })
    .when(is_active && self.focus_handle.contains_focused(window, cx),
        |div| div.border_color(cx.theme().colors().panel_focused_border))
```

---

## 2. Panel header/toolbar patterns

`panel::PanelHeader` is just a marker: `pub trait PanelHeader: workspace::Panel {}` (/Users/user/zed/crates/panel/src/panel.rs:14; `impl PanelHeader for GitPanel {}` git_panel.rs:7414). Real chrome is hand-rolled `h_flex` rows.

### git_panel header row — /Users/user/zed/crates/git_ui/src/git_panel.rs:5059 `fn render_changes_header(&self, _window: &mut Window, cx: &mut Context<Self>) -> Option<impl IntoElement>`

```rust
h_flex()
    .min_h(Tab::container_height(cx))
    .w_full().pl_1().pr_2().flex_none().flex_wrap().gap_1()
    .justify_between()
    .child(
        ButtonLike::new("diff-button")
            .child(h_flex().gap_1()
                .child(Icon::new(IconName::Diff).size(IconSize::Small).color(Color::Muted))
                .child(Label::new("View Diff").size(LabelSize::Small).color(Color::Muted)))
            .tooltip(Tooltip::for_action_title_in("View Diff", &Diff, &self.focus_handle))
            .on_click(|_, _, cx| cx.defer(|cx| cx.dispatch_action(&Diff))),
    )
    .child(h_flex().gap_1()
        .child(self.render_view_options_menu("view_options_menu"))
        .child(self.render_git_changes_actions_button(cx)))
```

Header IconButton-with-dropdown — git_panel.rs:4718:
```rust
PopoverMenu::new(id.into())
    .trigger_with_tooltip(
        IconButton::new("view-options-menu-trigger", IconName::Sliders).icon_size(IconSize::Small),
        Tooltip::text("View Options"),
    )
    .menu(move |window, cx| Some(git_panel_view_options_menu(focus_handle.clone(), window, cx)))
    .anchor(Anchor::TopRight)
```

### terminal_panel toolbar (+ button, split, zoom) — /Users/user/zed/crates/terminal_view/src/terminal_panel.rs:155–228

```rust
let right_children = h_flex()
    .gap(DynamicSpacing::Base02.rems(cx))
    .child(
        PopoverMenu::new("terminal-tab-bar-popover-menu")
            .trigger_with_tooltip(
                IconButton::new("plus", IconName::Plus).icon_size(IconSize::Small),
                Tooltip::text("New…"),
            )
            .anchor(Anchor::TopRight)
            .with_handle(pane.new_item_context_menu_handle.clone())
            .menu(move |window, cx| {
                let focus_handle = focus_handle.clone();
                let menu = ContextMenu::build(window, cx, |menu, _, _| {
                    menu.context(focus_handle.clone())
                        .action("New Terminal", workspace::NewTerminal::default().boxed_clone())
                        .action("Spawn Task", zed_actions::Spawn::modal().boxed_clone())
                });
                Some(menu)
            }),
    )
    .child({
        let zoomed = pane.is_zoomed();
        IconButton::new("toggle_zoom", IconName::Maximize)
            .icon_size(IconSize::Small)
            .toggle_state(zoomed)
            .selected_icon(IconName::Minimize)
            .on_click(cx.listener(|pane, _, window, cx| {
                pane.toggle_zoom(&workspace::ToggleZoom, window, cx);
            }))
            .tooltip(move |_window, cx| {
                Tooltip::for_action(if zoomed { "Zoom Out" } else { "Zoom In" }, &ToggleZoom, cx)
            })
    })
    .into_any_element()
```
`IconButton::new(id: impl Into<ElementId>, icon: IconName) -> Self` (/Users/user/zed/crates/ui/src/components/button/icon_button.rs:33); common builders seen above: `.icon_size(IconSize)`, `.icon_color(Color)`, `.toggle_state(bool)`, `.selected_icon(IconName)`, `.on_click(...)`, `.tooltip(...)`.

### Panel root element pattern (git_panel Render, git_panel.rs:7187–7296)

```rust
v_flex()
    .id("git_panel")
    .key_context(self.dispatch_context(window, cx))
    .track_focus(&self.focus_handle)
    .on_action(cx.listener(Self::select_next))     // dozens of .on_action(...)
    .size_full()
    .overflow_hidden()
    .bg(cx.theme().colors().panel_background)
    .child(v_flex().size_full()
        .children(self.render_changes_header(window, cx))
        .child(self.render_entries(has_write_access, repo, window, cx))
        .children(self.render_footer(window, cx)))
```
The list itself is wrapped: `v_flex().flex_1().size_full().overflow_hidden().relative().child(h_flex().flex_1().size_full().relative().overflow_hidden().child(uniform_list(...)))` (git_panel.rs:6263–6274).

---

## 3. Right-click ContextMenu on a list entry

`ContextMenu::build` signature — /Users/user/zed/crates/ui/src/components/context_menu.rs:329:
```rust
pub fn build(
    window: &mut Window,
    cx: &mut App,
    f: impl FnOnce(Self, &mut Window, &mut Context<Self>) -> Self,
) -> Entity<Self>
```

State field: `context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>` (project_panel.rs:150).

### Deploy (condensed) — project_panel.rs:1059 `fn deploy_context_menu(&mut self, position: Point<Pixels>, entry_id: ProjectEntryId, window: &mut Window, cx: &mut Context<Self>)`

```rust
let context_menu = ContextMenu::build(window, cx, |menu, _, cx| {
    menu.context(self.focus_handle.clone())
        .action("New File", Box::new(NewFile))
        .action("New Folder", Box::new(NewDirectory))
        .separator()
        .when(is_dir, |menu| menu.separator().action("Find in Folder…", Box::new(NewSearchInDirectory)))
        .action_disabled_when(!has_pasteable_content, "Paste", Box::new(Paste))
        .when(!should_hide_rename, |menu| menu.separator().action("Rename", Box::new(Rename)))
        .when(!is_root, |menu| menu.action("Delete", Box::new(Delete { skip_prompt: false })))
});

window.focus(&context_menu.focus_handle(cx), cx);
let subscription = cx.subscribe(&context_menu, |this, _, _: &DismissEvent, cx| {
    this.context_menu.take();
    cx.notify();
});
self.context_menu = Some((context_menu, position, subscription));
cx.notify();
```
(full menu with all `.when` branches: lines 1116–1232)

Triggered from the row via `.on_secondary_mouse_down(cx.listener(move |this, event: &MouseDownEvent, window, cx| { cx.stop_propagation(); this.deploy_context_menu(event.position, entry_id, window, cx); }))` (project_panel.rs:6171; outline_panel.rs:2665).

### Rendering the open menu (child of panel root) — project_panel.rs:7394

```rust
.children(self.context_menu.as_ref().map(|(menu, position, _)| {
    deferred(
        anchored()
            .position(*position)
            .anchor(gpui::Anchor::TopLeft)
            .child(menu.clone()),
    )
    .with_priority(3)
}))
```
(`deferred`, `anchored` are gpui imports — project_panel.rs:30 area imports.)

---

## 4. Error-colored Label/Icon + Tooltip

`ui::Color` enum — /Users/user/zed/crates/ui/src/styles/color.rs:19: variants include `Default, Accent, Conflict, Created, Custom(Hsla), Debugger, Deleted, Disabled, Error, Hidden, Hint, Ignored, Info, Modified, Muted, Warning, Success, ...`. Resolve to `Hsla` with `Color::Error.color(cx)`.

Error label (project_panel.rs:6048–6054, diagnostics badge in end_slot):
```rust
Label::new(count.capped_error_count())
    .size(LabelSize::Small)
    .color(Color::Error)
```
Warning variant right below uses `.color(Color::Warning)` (6059–6062). Raw Hsla for borders: `ValidationState::Error(msg) => Some((Color::Error.color(cx), msg))` (project_panel.rs:5611).

Icon with color + size: `Icon::new(IconName::ArrowUpRight).size(IconSize::Indicator).color(filename_text_color)` (project_panel.rs:6037); `Icon::new(icon_name).color(color).size(IconSize::Small)` (project_panel.rs:6128); from file-type path: `Icon::from_path(icon.clone()).color(Color::Muted)` (project_panel.rs:6094).

### `ui::Tooltip` constructors — /Users/user/zed/crates/ui/src/components/tooltip.rs:37–150

```rust
pub fn simple(title: impl Into<SharedString>, cx: &mut App) -> AnyView
pub fn text(title: impl Into<SharedString>) -> impl Fn(&mut Window, &mut App) -> AnyView   // pass directly to .tooltip(...)
pub fn for_action_title<T: Into<SharedString>>(title: T, action: &dyn Action) -> impl Fn(&mut Window, &mut App) -> AnyView
pub fn for_action_title_in<Str: Into<SharedString>>(title: Str, action: &dyn Action, focus_handle: &FocusHandle) -> impl Fn(&mut Window, &mut App) -> AnyView
pub fn for_action(title: impl Into<SharedString>, action: &dyn Action, cx: &mut App) -> AnyView
pub fn for_action_in(title: impl Into<SharedString>, action: &dyn Action, focus_handle: &FocusHandle, cx: &mut App) -> AnyView
pub fn with_meta(title: impl Into<SharedString>, action: Option<&dyn Action>, meta: impl Into<SharedString>, cx: &mut App) -> AnyView
pub fn with_meta_in(title, action: Option<&dyn Action>, meta, focus_handle: &FocusHandle, cx: &mut App) -> AnyView
```

Usage patterns:
- Closure form: `.tooltip(Tooltip::text("View Options"))` (git_panel.rs:4725) or `.tooltip(move |_window, cx| Tooltip::simple(format!("{action} folder"), cx))` (git_panel.rs:6964–6970).
- With keybinding: `.tooltip(move |_window, cx| Tooltip::for_action("Zoom In", &ToggleZoom, cx))` (terminal_panel.rs:220–226); scoped to panel focus: `.tooltip(Tooltip::for_action_title_in("View Diff", &Diff, &self.focus_handle))` (git_panel.rs:5109).
- With meta line: element `div().id("symlink_icon").tooltip(move |_window, cx| Tooltip::with_meta(path.to_string_lossy().into_owned(), None, "Symbolic Link", cx)).child(Icon::new(...))` (project_panel.rs:6026–6041).

---

## Misc gotchas

- Row `ElementId` must be unique and stable: project_panel uses `(entry_id.to_proto() as usize).into()`; git_panel uses `ElementId::Name(format!("dir_{}_{}", entry.name, ix).into())` (git_panel.rs:6837).
- Selected/focused row border: `.border_1().border_r_2()` + `.when(selected && self.focus_handle.is_focused(window, cx), |el| el.border_color(cx.theme().colors().panel_focused_border))` (git_panel.rs:6926–6930).
- Hover colors come from theme: `cx.theme().colors().ghost_element_background / ghost_element_hover / ghost_element_active` (git_panel.rs:6856–6860).
- After any expand/collapse mutation: rebuild the flattened visible-entries Vec, then `cx.notify()`.
- Manual depth indent alternative to `ListItem::indent_level`: `.pl(px(entry.depth as f32 * TREE_INDENT))` (git_panel.rs:6901).