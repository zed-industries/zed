//! The Designer — a workspace tab that renders a `.flow.json` workflow as
//! a `gpui_flow` canvas: view, edit node properties, save, and run.
//!
//! Ported from `E:\Forge_GPUI\src\forge_shell\panels\designer_panel\`,
//! narrowed to the workflow-JSON half only (`.fdgn`/legacy ForgeFlow was
//! never vendored into this tree — see `gpui_flow`/`workflow_engine`'s own
//! module docs). Unlike `flows_panel` (a `workspace::dock::Panel`), this
//! is a per-file `workspace::item::Item` — opened once per flow, not a
//! persistent sidebar.
//!
//! Positions round-trip through the `.flow.layout.json` sidecar
//! (`workflow_engine::layout`), never through `Action::pos` — see
//! `workflow_json`'s module doc. A flow with no sidecar yet gets a
//! layered auto-layout (`workflow_json::auto_layout`, built on
//! `workflow_engine::compute_levels`) instead of the placeholder stack.
//!
//! Property editing is deliberately plain-text (every `FlowNode.property`
//! value is a string, parsed back to JSON on save via `string_to_value`)
//! rather than a typed widget per `registry::FieldKind` — matches
//! `flows_panel`'s own "everything as text for now" pragmatism; a typed
//! form is a natural follow-up, not required to make editing usable.

mod catalog;
mod http_companion;
mod persistence;
mod project_item;
mod run_results;
mod run_state;
mod script_companion;
mod script_helpers;
mod workflow_json;

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::Context as _;
use editor::Editor;
use gpui::{
    Action, App, AppContext as _, Context, DismissEvent, Entity, EventEmitter, FocusHandle,
    Focusable, InteractiveElement as _, IntoElement, ParentElement as _, Point, Render,
    ScrollHandle, SharedString, StatefulInteractiveElement as _, Styled as _, Subscription,
    TaskExt as _, WeakEntity, Window, anchored, deferred, div, prelude::FluentBuilder as _, px,
};
use gpui_component::{
    ActiveTheme as _, Disableable as _, Icon, IconName, IndexPath, Sizable as _, Size, Theme,
    button::{Button, ButtonVariants as _},
    h_flex,
    input::{Input, InputEvent, InputState, NumberInput, Textarea, TextareaState},
    list::{List, ListEvent, ListState},
    menu::{ContextMenuExt as _, PopupMenu, PopupMenuItem},
    popover::Popover,
    resizable::{h_resizable, resizable_panel},
    scroll::Scrollbar,
    select::{Select, SelectEvent, SelectState},
    spinner::Spinner,
    switch::Switch,
    tag::Tag,
    v_flex,
};
use gpui_flow::{
    Controls, FlowEdge, FlowGraph, FlowNode, FlowPoint, FlowState, HandleDef, HandlePosition,
    Minimap, NodeId,
};
use notifications::status_toast::StatusToast;
use project::Project;
use run_state::{ActionRun, RunLogLine, RunPhase, RunState, SharedRunState};
use workflow_engine::{
    ActionHistoryRecord, FieldKind, InputField, RunHistoryEntry, RunOutcome, RunStatus,
    ScriptRuntime, ScriptSource, StatusSink, ValidationIssue, ValidationSeverity,
    WorkflowDefinition, registry::REGISTRY, run_workflow, validate_definition,
};
use workspace::{
    ItemId, Pane, ProjectItem, SerializableItem, Workspace, WorkspaceId,
    item::{Item, ItemEvent, SaveOptions},
};

pub use project_item::FlowFile;
pub use workflow_json::{apply_saved_layout, auto_layout, load, save};

const CANVAS_SIZE: (f32, f32) = (1200.0, 800.0);

/// How far an action moves down when another is inserted above it: a leaf's
/// height plus the gap `below_node` leaves between consecutive steps.
const INSERT_SHIFT: f32 = workflow_json::LEAF_SIZE.1 + 30.0;

#[derive(Clone)]
struct ContextTarget {
    node_id: Option<NodeId>,
    position: FlowPoint,
}

/// Where a newly added action goes, resolved by the caller before any node
/// exists. See [`DesignerPanel::push_action_node`], the one consumer.
#[derive(Clone, Debug)]
struct ActionAnchor {
    /// Absolute flow-space position. `FlowState::set_parent` re-expresses it
    /// relative to `parent_id` after insertion, so callers can always compute
    /// it in the same absolute frame.
    position: FlowPoint,
    /// Container to nest the new node inside.
    parent_id: Option<NodeId>,
    /// Node to draw the new node's incoming edge from. Absent for an
    /// explicitly placed node — a deliberate drop has no "previous step".
    source_id: Option<NodeId>,
    /// Position inside `parent_id`, in the parent's own frame, used instead
    /// of converting `position`. Set when the spot is known relative to the
    /// container: below a nested sibling, or the next free slot of a body the
    /// cursor wasn't actually over.
    local_position: Option<FlowPoint>,
}

/// `FlowGraph`'s `bg_color`/`grid_color`/`node_bg_color`/`node_border_color`
/// take a raw `0xRRGGBB` `u32`, not an `Hsla` — this converts a theme color
/// at the point the canvas is built, so it opens matching Zed's active
/// theme instead of a fixed dark palette. These four are set once at
/// construction (not re-read on every render like the node renderers
/// below), so a flow tab left open across a live theme switch keeps the
/// canvas chrome it opened with — same one-shot-at-construction tradeoff
/// `gpui_flow`'s own demos make; a full live-resync would need observing
/// `SettingsStore` and rebuilding `FlowGraph`, not just its labels.
fn hex(color: gpui::Hsla) -> u32 {
    u32::from(color.to_rgb()) >> 8
}

/// The fixed set of container node types the canvas ever renders a header
/// for — `Foreach`/`Until`/`If`/`Try` from `registry.rs` (a closed set by
/// that module's own design), plus `workflow_json`'s four synthetic branch
/// wrappers. Leaf types are open-ended (any `REGISTRY` entry), so they go
/// through `default_renderer` instead of a per-type registration.
const CONTAINER_TYPES: &[(&str, &str)] = &[
    ("Foreach", "\u{21bb} Foreach"),
    ("Until", "\u{27f3} Until"),
    ("If", "\u{2442} If"),
    ("Try", "\u{26a0} Try/Catch"),
    (workflow_json::BRANCH_THEN, "True"),
    (workflow_json::BRANCH_ELSE, "False"),
    (workflow_json::BRANCH_TRY, "Try"),
    (workflow_json::BRANCH_CATCH, "Catch"),
];

/// The four synthetic branch nodes `workflow_json::load` synthesizes to give
/// an `If`/`Try`'s nested bodies a canvas container. `set` here is the
/// authoritative list — `workflow_json::is_branch_wrapper` is the same set and
/// the two must stay in sync.
const BRANCH_WRAPPER_TYPES: &[&str] = &[
    workflow_json::BRANCH_THEN,
    workflow_json::BRANCH_ELSE,
    workflow_json::BRANCH_TRY,
    workflow_json::BRANCH_CATCH,
];

fn is_branch_wrapper_type(type_id: impl AsRef<str>) -> bool {
    BRANCH_WRAPPER_TYPES.contains(&type_id.as_ref())
}

/// Human-readable role of a branch wrapper, for the inspector's explanation.
fn branch_wrapper_label(type_id: &str) -> &'static str {
    match type_id {
        workflow_json::BRANCH_THEN => "the True branch",
        workflow_json::BRANCH_ELSE => "the False branch",
        workflow_json::BRANCH_TRY => "the Try body",
        _ => "the Catch handler",
    }
}

/// "View Raw" tab-context-menu action — carries the flow's own abs path so
/// the handler (registered once, workspace-wide, in [`init`]) doesn't need
/// to know which `DesignerPanel` instance the click came from. Opens the
/// same plain-JSON tab `flows_panel`'s "Open" button does.
#[derive(Action, Clone, PartialEq, Eq, serde::Deserialize)]
#[action(namespace = designer_panel, no_json)]
pub struct ViewRaw(PathBuf);

/// Opens the workspace-visible Inline script companion as a normal editor
/// buffer, giving it the file URI and worktree context required for LSP.
#[derive(Action, Clone, PartialEq, Eq, serde::Deserialize)]
#[action(namespace = designer_panel, no_json)]
pub struct ViewScriptCompanion(PathBuf);

/// Opens an `http` action's request-body companion (`<flow>.flow-http/…`) as
/// a normal editor buffer, giving it the file URI and worktree context the
/// JSON language server needs to apply the body's schemas.
///
/// Same shape and reason as [`ViewScriptCompanion`]: dispatched to a
/// workspace-wide handler so it works for designers opened via
/// `ProjectItem::for_project_item`, which have no workspace handle of their own.
#[derive(Action, Clone, PartialEq, Eq, serde::Deserialize)]
#[action(namespace = designer_panel, no_json)]
pub struct ViewHttpBody(PathBuf);

/// "Results" action — opens (or reactivates) the `RunResults` tab bound to
/// the flow at `path`. Dispatched from the designer toolbar through the
/// workspace-wide handler below, so it works even for designers opened via
/// `ProjectItem::for_project_item`, which have no `WeakEntity<Workspace>`
/// of their own to call `add_item_to_active_pane` through.
#[derive(Action, Clone, PartialEq, Eq, serde::Deserialize)]
#[action(namespace = designer_panel, no_json)]
pub struct ViewRunResults(PathBuf);

/// Fired by `DesignerPanel::build` when a flow fails to parse. Carries just
/// the flow's display name — dispatched via `window.dispatch_action` rather
/// than resolved through `self.workspace` (which is `None` for designers
/// opened via `ProjectItem::for_project_item`, e.g. double-click/quick-open
/// — exactly the case that needs this toast most) so it reaches the
/// workspace-wide handler below regardless of how the designer was opened.
#[derive(Action, Clone, PartialEq, Eq, serde::Deserialize)]
#[action(namespace = designer_panel, no_json)]
pub struct ShowInvalidFlowToast(String);

/// Registers the "View Raw", "Results", and invalid-flow-toast handlers on
/// every workspace. Call once at app startup, alongside the other panels'
/// `init` functions — and see `DesignerPanel::for_project_item`'s doc
/// comment for why `.flow.json` opening as the Designer doesn't need a
/// matching registration here (it's wired via
/// `workspace::register_project_item` instead).
pub fn init(cx: &mut gpui::App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, action: &ViewRaw, window, cx| {
            // Deliberately not `workspace.open_abs_path` — that still routes
            // through `ProjectItemRegistry`, which re-claims `.flow.json`
            // via `FlowFile::try_open` and just reopens the Designer canvas
            // again. Building the plain-text `Editor` directly and adding
            // it to the pane is what actually bypasses that routing.
            let abs_path = action.0.clone();
            let project = workspace.project().clone();
            let Some(project_path) = project
                .read(cx)
                .project_path_for_absolute_path(&abs_path, cx)
            else {
                log::warn!("View Raw: {abs_path:?} isn't inside a worktree of this project");
                return;
            };

            cx.spawn_in(window, async move |workspace, cx| {
                let buffer = project
                    .update(cx, |project, cx| project.open_buffer(project_path, cx))
                    .await?;
                workspace.update_in(cx, |workspace, window, cx| {
                    let editor = cx.new(|cx| Editor::for_buffer(buffer, Some(project), window, cx));
                    workspace.add_item_to_active_pane(Box::new(editor), None, true, window, cx);
                })
            })
            .detach_and_log_err(cx);
        });
        workspace.register_action(|workspace, action: &ViewScriptCompanion, window, cx| {
            let abs_path = action.0.clone();
            let project = workspace.project().clone();
            let Some(project_path) = project
                .read(cx)
                .project_path_for_absolute_path(&abs_path, cx)
            else {
                log::warn!("Open Script: {abs_path:?} isn't inside a worktree of this project");
                return;
            };

            cx.spawn_in(window, async move |workspace, cx| {
                let buffer = project
                    .update(cx, |project, cx| project.open_buffer(project_path, cx))
                    .await?;
                workspace.update_in(cx, |workspace, window, cx| {
                    let editor = cx.new(|cx| Editor::for_buffer(buffer, Some(project), window, cx));
                    workspace.add_item_to_active_pane(Box::new(editor), None, true, window, cx);
                })
            })
            .detach_and_log_err(cx);
        });
        workspace.register_action(|workspace, action: &ViewHttpBody, window, cx| {
            let abs_path = action.0.clone();
            let project = workspace.project().clone();
            let Some(project_path) = project
                .read(cx)
                .project_path_for_absolute_path(&abs_path, cx)
            else {
                log::warn!("Edit Body: {abs_path:?} isn't inside a worktree of this project");
                return;
            };

            cx.spawn_in(window, async move |workspace, cx| {
                let buffer = project
                    .update(cx, |project, cx| project.open_buffer(project_path, cx))
                    .await?;
                workspace.update_in(cx, |workspace, window, cx| {
                    let editor = cx.new(|cx| Editor::for_buffer(buffer, Some(project), window, cx));
                    workspace.add_item_to_active_pane(Box::new(editor), None, true, window, cx);
                })
            })
            .detach_and_log_err(cx);
        });
        workspace.register_action(|workspace, action: &ViewRunResults, window, cx| {
            run_results::RunResults::open(workspace, &action.0, window, cx);
        });
        workspace.register_action(|workspace, action: &ShowInvalidFlowToast, _window, cx| {
            let flow_name = action.0.clone();
            let status_toast = StatusToast::new(
                format!(
                    "{flow_name}\nInvalid flow configuration — please fix the JSON and try again"
                ),
                cx,
                |this, _cx| this.icon(ui::Icon::new(ui::IconName::Warning).color(ui::Color::Error)),
            );
            workspace.toggle_status_toast(status_toast, cx);
        });
    })
    .detach();
    workspace::register_project_item::<DesignerPanel>(cx);
    workspace::register_serializable_item::<DesignerPanel>(cx);
}

/// The runtime a `File`-mode script at `absolute` will run under: the
/// flow's explicit override when it has one, otherwise inferred from the
/// extension by the same `ScriptRuntime::from_path` the executor uses.
///
/// Checked before opening the file so a script the engine couldn't run is
/// reported here - naming the path and the fix - rather than as a spawn
/// failure on the next Run.
fn inferred_runtime(
    absolute: &Path,
    override_runtime: Option<ScriptRuntime>,
) -> Result<ScriptRuntime, String> {
    match override_runtime {
        Some(runtime) => Ok(runtime),
        None => ScriptRuntime::from_path(absolute).ok_or_else(|| {
            format!(
                "Couldn't infer a runtime from {} - give the script an explicit \"runtime\" \
                 override, or use a known extension (py, js, mjs, cjs, ts, mts, cts, ps1, sh, \
                 bash, csproj)",
                absolute.display()
            )
        }),
    }
}

fn node_label(node: &FlowNode) -> String {
    if node.label.is_empty() {
        node.id.to_string()
    } else {
        node.label.to_string()
    }
}

/// Deterministic FNV-1a over `type_id`, so the same action type always
/// lands on the same one of the theme's 5 categorical `chart_*` colors —
/// stable across renders and across runs, unlike a `HashMap`'s default
/// per-process-random hasher.
fn accent_for_type(type_id: &str, cx: &App) -> gpui::Hsla {
    let mut hash: u32 = 0x811c9dc5;
    for byte in type_id.bytes() {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(0x01000193);
    }
    let theme = cx.theme();
    let palette = [
        theme.chart_1,
        theme.chart_2,
        theme.chart_3,
        theme.chart_4,
        theme.chart_5,
    ];
    palette[(hash as usize) % palette.len()]
}

fn render_container_header(
    node: &FlowNode,
    title: &str,
    _w: &mut gpui::Window,
    cx: &mut App,
) -> gpui::AnyElement {
    div()
        .text_sm()
        .font_weight(gpui::FontWeight::MEDIUM)
        .text_color(cx.theme().popover_foreground)
        .child(format!("{title} \u{2014} {}", node_label(node)))
        .into_any_element()
}

fn render_leaf(node: &FlowNode, _w: &mut gpui::Window, cx: &mut App) -> gpui::AnyElement {
    let type_label = node.node_type.clone().unwrap_or_else(|| "action".into());
    let accent = accent_for_type(&type_label, cx);
    let text_color = cx.theme().popover_foreground;

    // Left accent bar — the one place a node's `type_id` gets a stable,
    // distinct color; everything else is theme-neutral so different
    // action types still read apart from each other at a glance, matching
    // the original Forge canvas's per-type accent bars (see `render_leaf`
    // in the vendored kitchen-sink demo).
    let accent_bar = div()
        .w(gpui::px(3.0))
        .rounded_l_sm()
        .bg(accent)
        .my(gpui::px(-8.0))
        .ml(gpui::px(-16.0))
        .mr(gpui::px(12.0));

    let type_row = div()
        .text_xs()
        .text_color(text_color)
        .font_weight(gpui::FontWeight::MEDIUM)
        .child(type_label);

    let label_row = div()
        .text_sm()
        .text_color(text_color)
        .font_weight(gpui::FontWeight::MEDIUM)
        .child(node_label(node));

    // Same subtle tint `gpui_flow`'s own container-header strip sits on
    // (`graph.rs`'s `container_header_strip`, `bg(0x00000008)`) — without
    // it, identically-colored text reads dimmer here than in a header
    // simply because it has less local contrast against plain `popover`.
    // Leaf nodes have no header/body split to hang a real strip off, so
    // this wraps just the text instead of reproducing the header's
    // viewport-relative full-bleed geometry (not available here anyway —
    // this renderer never sees the canvas's `Viewport`/zoom).
    let text_block = div()
        .flex()
        .flex_col()
        .gap_0p5()
        .rounded_sm()
        .px_1()
        .py_0p5()
        .bg(gpui::rgba(0x00000008))
        .child(type_row)
        .child(label_row);

    div()
        .flex()
        .flex_row()
        .items_center()
        .child(accent_bar)
        .child(text_block)
        .into_any_element()
}

/// One editable field in the inspector.
///
/// `kind`/`required`/`help` come from the `ActionDef` registry entry for the
/// node's `type_id` (see `workflow_json::declared_field`), so the editor the
/// user gets — and the guidance under it — is generated from the same
/// metadata the runtime validates against, rather than restated here.
///
/// A `None` `kind` means the key is **undeclared**: an extra the registry has
/// no schema for (a leftover `strategy` key on an old transform flow, or a
/// container meta row like `__limit_timeout`). Those still round-trip
/// (`workflow_json`'s `string_to_value`) but get a plain text editor and no
/// required/help treatment, because there's nothing truthful to assert.
struct PropertyRow {
    key: SharedString,
    value: Entity<InputState>,
    /// Backing state for the multi-line editor used by `Json`/`Expression`
    /// fields. `None` renders the single-line `value` input instead. Both are
    /// always kept in sync, so either can be read back.
    textarea: Option<Entity<TextareaState>>,
    /// Dropdown for a field the registry restricts to a closed value set
    /// (`InputField::options`). `None` for free input.
    select: Option<Entity<SelectState<Vec<SharedString>>>>,
    /// Declared `FieldKind`, or `None` for an undeclared extra.
    kind: Option<FieldKind>,
    required: bool,
    help: SharedString,
    /// A short type badge (`Text`, `Number`, `Bool`, `JSON`, `Expr`) shown
    /// next to the label, so the shape of a field is legible before editing.
    type_badge: Option<&'static str>,
    /// Programmatic write path, shared with this row's widgets.
    ///
    /// The `Switch` and the expression-picker chips *must* go through this
    /// rather than relying on an `InputEvent::Change`: both `set_value` and
    /// `insert` set `emit_events = false` (they're silent by design, so a
    /// programmatic sync doesn't look like a user edit), so a toggle that only
    /// wrote to the input would repaint without ever reaching `FlowNode` — and
    /// a save would then persist the old value.
    commit: Commit,
    _sub: Subscription,
}

/// The action menu shown after clicking the "+" on a connection.
struct InsertMenu {
    menu: Entity<PopupMenu>,
    /// Window position of the click, where the menu opens.
    position: Point<gpui::Pixels>,
    _dismissed: Subscription,
}

/// Fills `menu` with every addable action type, grouped the way the catalog
/// groups them, calling `on_pick` with the chosen type.
fn action_type_menu(
    mut menu: PopupMenu,
    window: &mut Window,
    cx: &mut Context<PopupMenu>,
    on_pick: Rc<dyn Fn(&'static str, &mut App)>,
) -> PopupMenu {
    for section in catalog::build_sections() {
        let on_pick = on_pick.clone();
        let group = PopupMenu::build(window, cx, move |mut group, _window, _cx| {
            for entry in section.entries.iter().filter(|entry| entry.runnable) {
                let type_id = entry.type_id;
                let on_pick = on_pick.clone();
                group = group.item(
                    PopupMenuItem::new(entry.label.clone())
                        .on_click(move |_, _, cx| on_pick(type_id, cx)),
                );
            }
            group
        });
        menu = menu.item(PopupMenuItem::submenu(section.title.clone(), group));
    }
    menu
}

/// What an expression on one node can refer to. Built by
/// `DesignerPanel::reference_scope`.
#[derive(Default)]
struct ReferenceScope {
    /// Actions whose outputs are recorded by the time the expression runs.
    actions: Vec<NodeId>,
    /// `Try` actions whose catch branch the node sits in; their `error`
    /// output is set there.
    catch_owners: Vec<NodeId>,
    /// Enclosing `Foreach` actions, innermost first. Non-empty means `$item`
    /// is bound.
    loops: Vec<NodeId>,
}

/// A reference found in what an action returned on the last run.
#[derive(Debug, Clone, PartialEq)]
struct SampledRef {
    /// The JSONLogic `var` path, e.g. `fetch.outputs.body.customer.id`.
    path: String,
    /// `value`, `object` or `list`.
    kind: &'static str,
    /// The sampled value, shortened, for the chip's tooltip.
    preview: String,
}

/// How far into a sampled output [`sample_paths`] descends.
const MAX_SAMPLE_DEPTH: usize = 4;

/// How many sampled references one picker lists, so a large response can't
/// bury the field under chips.
const MAX_SAMPLED_REFS: usize = 60;

/// Lists the `var` paths under `value`, an object reached at `prefix`.
///
/// A list is offered as a whole and not descended into: an element has no
/// name to put in the path, and which index is wanted isn't something a
/// sample can say. A key containing a `.` is skipped, since `var` would read
/// it as two path segments.
fn sample_paths(prefix: &str, value: &serde_json::Value, depth: usize, out: &mut Vec<SampledRef>) {
    let Some(object) = value.as_object() else {
        return;
    };
    for (key, value) in object {
        if key.is_empty() || key.contains('.') {
            continue;
        }
        let path = format!("{prefix}.{key}");
        let (kind, preview) = match value {
            serde_json::Value::Object(_) => ("object", "an object".to_string()),
            serde_json::Value::Array(items) => ("list", format!("a list of {}", items.len())),
            scalar => {
                let text = scalar.to_string();
                let preview = if text.chars().count() > 40 {
                    format!("{}…", text.chars().take(40).collect::<String>())
                } else {
                    text
                };
                ("value", preview)
            }
        };
        out.push(SampledRef {
            path: path.clone(),
            kind,
            preview,
        });
        if depth < MAX_SAMPLE_DEPTH {
            sample_paths(&path, value, depth + 1, out);
        }
    }
}

/// Short label for a declared `FieldKind`, for the inspector's type badge.
fn field_kind_badge(kind: FieldKind) -> &'static str {
    match kind {
        FieldKind::String => "Text",
        FieldKind::Number => "Number",
        FieldKind::Bool => "Bool",
        FieldKind::Json => "JSON",
        FieldKind::Expression => "Expr",
    }
}

/// Collects the `<action_id>.<output>` paths an expression on `node_id` could
/// legally reference: every output of every action that is guaranteed to have
/// run first.
///
/// "Guaranteed to have run first" is the real constraint — a JSONLogic `var`
/// against an action that may not have run yet resolves to null and silently
/// changes behavior, so offering only level-ordered predecessors keeps the
/// picker honest. Ordering comes from `compute_levels` over the node's own
/// scope's reconstructed `runAfter` graph, and output names from each action's
/// `ActionDef::outputs` (the same metadata the runtime fills them from).
///
/// Actions with no declared outputs (the four container types) contribute
/// `sync_selection` uses to decide whether to render a row for a missing
/// required field.
/// Writes a property row's new text back onto its `FlowNode` and revalidates.
///
/// Shared by a row's two editors (`Input` and, for `Json`/`Expression`, a
/// `Textarea`) so a toggle and a typed edit are indistinguishable downstream.
type Commit = Rc<dyn Fn(&mut DesignerPanel, String, &mut Context<DesignerPanel>)>;

/// The `FlowNode::properties` key a registry input of `type_id` is stored
/// under.
///
/// A container's own fields (`foreach`/`until`/`limit`/`expression`) live on
/// the `Action` struct rather than in `inputs`, so `workflow_json` keeps them
/// under reserved `__` keys. Only containers get that mapping: `transform`
/// also declares an `expression`, and that one is an ordinary input.
fn storage_key<'a>(type_id: &str, name: &'a str) -> &'a str {
    if !workflow_engine::registry::is_container(type_id) {
        return name;
    }
    match name {
        "expression" => "__expression",
        "foreach" => "__foreach",
        "until" => "__until",
        // `limit` is one `Json` object in the registry but two canvas rows; the
        // count row is the one that carries the field's presence.
        "limit" => "__limit_count",
        _ => name,
    }
}

fn meta_property(
    type_id: &str,
    properties: &[(SharedString, SharedString)],
    name: &str,
) -> Option<(SharedString, SharedString)> {
    let storage_key = storage_key(type_id, name);
    properties
        .iter()
        .find(|(k, _)| k.as_ref() == storage_key)
        .cloned()
}

/// The content-dependent half of `DesignerPanel`'s state — everything that
/// comes from parsing and laying out the flow's own JSON, as opposed to
/// panel-lifetime state (run history, theme subscription, property-editor
/// inputs) that should survive a reload untouched. Built once by
/// `DesignerPanel::load` at construction, and rebuilt by the same function
/// whenever the file-watcher (`watch_file`) detects the file changed on
/// disk out from under an already-open tab.
struct LoadedFlow {
    error: Option<String>,
    flow_id: String,
    flow_name: String,
    /// Flow-level content the canvas can't represent (today just the
    /// flow-level `outputs` map), carried so `save` can write it back
    /// verbatim instead of silently dropping it.
    preserved: workflow_json::FlowPreserved,
    state: Entity<FlowState>,
    context_target: Rc<RefCell<ContextTarget>>,
    flow: Entity<FlowGraph>,
    minimap: Entity<Minimap>,
    controls: Entity<Controls>,
    state_sub: Subscription,
    /// The companion content last synced per script action, as of the moment
    /// this state was built - the baseline `script_companion`'s two-way sync
    /// compares against to tell a panel edit from an external one.
    script_sync: script_companion::SyncState,
    /// The same baseline for `http` request bodies, which sync through their
    /// own companions and their own key space (an action id can be a script
    /// and an `http` body in different flows without either baseline knowing).
    body_sync: script_companion::SyncState,
}

pub struct DesignerPanel {
    focus_handle: FocusHandle,
    /// Not read yet — kept for workspace-level wiring. Which flows are open
    /// is already persisted per item by `persistence::DesignerDb`.
    /// `None` when opened via `ProjectItem::for_project_item` (a bare
    /// `Entity<Project>`/`Option<&Pane>` don't expose the owning
    /// workspace — `Pane::workspace` is crate-private to `workspace`);
    /// `Some` when opened via `flows_panel`'s "Graph" button, which has it.
    #[allow(dead_code)]
    workspace: Option<WeakEntity<Workspace>>,
    /// Used to resolve this flow's worktree for `watch_file`, and to build
    /// the "Raw" tab's `Editor` — see `ViewRaw`'s handler in `init`.
    project: Entity<Project>,
    /// The exact file content last used to (re)build the canvas — compared
    /// against on every file-watcher event so a reload triggered by this
    /// panel's own "Save" (or a redundant duplicate filesystem event for one
    /// logical write) is a no-op instead of a rebuild/flicker. Updated both
    /// on load and immediately after every successful save.
    last_raw: String,
    _file_watch: Option<Subscription>,
    /// Debounce handle for `watch_file`'s reload — assigning a new `Task`
    /// here drops (cancels) any still-pending one, which is what coalesces
    /// several rapid filesystem events for one logical external write into
    /// a single `reload_from_disk` call.
    _reload_debounce: gpui::Task<()>,
    path: PathBuf,
    root: PathBuf,
    flow_id: String,
    flow_name: String,
    /// Baseline for `script_companion`'s two-way inline/companion sync: the
    /// companion content each script action was last seen to hold. Reset by
    /// `reload_from_disk` (which re-derives it from disk) and never assumed
    /// for an action it doesn't know about.
    script_sync: script_companion::SyncState,
    /// Baseline for `http_companion`'s two-way inline/companion sync, kept
    /// separate from `script_sync` because it tracks a different set of files
    /// (request bodies rather than script sources).
    body_sync: script_companion::SyncState,
    /// Flow-level content the canvas can't represent (see
    /// `workflow_json::FlowPreserved`). Re-read on every load and handed to
    /// `workflow_json::save`, so a hand-authored flow-level `outputs` map
    /// survives editing instead of being wiped on the first save.
    flow_preserved: workflow_json::FlowPreserved,
    state: Entity<FlowState>,
    context_target: Rc<RefCell<ContextTarget>>,
    flow: Entity<FlowGraph>,
    minimap: Entity<Minimap>,
    controls: Entity<Controls>,
    /// The action catalog pane: a `gpui_component` list over
    /// `catalog::CatalogDelegate`, so search, group headings, arrow-key
    /// navigation, and `Role::List` semantics come from the shared widget
    /// rather than being re-implemented here. Its contents are derived from
    /// `REGISTRY` and never invalidated, which is why `reload_from_disk`
    /// leaves it alone.
    catalog: Entity<ListState<catalog::CatalogDelegate>>,
    /// Turns a confirmed catalog row into a node insertion. Clicking a row
    /// and pressing Enter on it both arrive here as `ListEvent::Confirm`.
    _catalog_sub: Subscription,
    label_input: Option<Entity<InputState>>,
    _label_sub: Option<Subscription>,
    property_rows: Vec<PropertyRow>,
    selected_node_id: Option<NodeId>,
    script_file_input: Option<Entity<InputState>>,
    _script_file_sub: Option<Subscription>,
    new_prop_key: Entity<InputState>,
    new_prop_value: Entity<InputState>,
    running: bool,
    status: Option<String>,
    error: Option<String>,
    /// Validation issues from the last `revalidate()` call, keyed by the
    /// layout path of the affected node (same convention as
    /// `WorkflowLayout::positions`). Populated after every load, save, and
    /// property edit. A node with no entry has no known issues. The full
    /// list drives the toolbar error badge and the per-node inspector hints.
    validation_issues: Vec<ValidationIssue>,
    /// [`Self::structure_fingerprint`] of the graph as of the last
    /// [`Self::on_state_changed`], so a change to which nodes and edges exist
    /// can be told from a drag or a selection.
    structure: u64,
    /// [`Self::content_fingerprint`] of the canvas as last loaded or saved.
    saved_content: u64,
    /// Whether the canvas differs from `saved_content`: what the tab's
    /// unsaved marker and the close prompt go by.
    dirty: bool,
    /// Whether the file changed on disk while `dirty`. The change is not
    /// loaded over the unsaved edits; the user picks which to keep.
    conflict: bool,
    /// Whether the toolbar error badge's issues popover is open. A plain
    /// bool rather than a `Popover` handle: the badge only needs "is it
    /// showing", and `Popover::on_open_change` keeps the two in sync for
    /// outside-click and Escape.
    show_issues: bool,
    /// The live run state shared with the `StatusSink` (written off-thread)
    /// and with the "Run Results" tab. Snapshot into `run_snapshot` by the
    /// poll task a few times a second while a run is active.
    run_state: SharedRunState,
    /// UI-side snapshot of `run_state`, driven by [`Self::sync_run`]. All
    /// rendering (toolbar "running" line, per-node results, transcript)
    /// reads this instead of locking the shared state from render.
    run_snapshot: RunState,
    /// Last `RunState::revision` that was merged into `run_snapshot` — a
    /// cheap "did the run make progress since my last paint" check.
    run_revision: u64,
    /// Whether the under-canvas run transcript is expanded.
    show_log: bool,
    log_scroll: ScrollHandle,
    /// Scroll position of the inspector pane, whose rows and reference chips
    /// routinely run past the bottom of the tab.
    inspector_scroll: ScrollHandle,
    /// The action menu opened from the "+" on a connection, while it is up.
    insert_menu: Option<InsertMenu>,
    /// Which iteration of each `Foreach` (by node id) the inspector is
    /// showing. A loop with no entry shows its first.
    loop_view: HashMap<String, usize>,
    /// Ids of actions whose last-run outputs no longer describe the flow,
    /// because they or something upstream of them was edited since. The
    /// expression picker doesn't sample these. Cleared by the next run.
    stale_samples: std::collections::HashSet<String>,
    _state_sub: Subscription,
    _theme_sub: Subscription,
}

impl DesignerPanel {
    /// Opens `path` (a `.flow.json` file) as a new canvas tab. `root` is
    /// the project root (used as `run_workflow`'s `solution_root` and the
    /// run-history namespace — same convention `flows_panel` uses). Used
    /// by `flows_panel`'s "Graph" button, which has a real
    /// `WeakEntity<Workspace>` to pass; opening via the project-item path
    /// (double-click, quick-open, ...) goes through `for_project_item`
    /// instead, which doesn't.
    pub fn open(
        path: PathBuf,
        root: PathBuf,
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        cx.new(|cx| Self::build(path, root, Some(workspace), project, window, cx))
    }

    /// Reads and builds the canvas state for `path`, applying its
    /// `.flow.layout.json` sidecar if present and auto-layouting anything
    /// still unpositioned. Infallible by design: a missing/unreadable/
    /// invalid file still produces an open (empty) canvas tab, with the
    /// failure recorded in `self.error` and rendered as this tab's own
    /// inline banner — `ProjectItem::for_project_item` has no `Result` to
    /// return through, so this can't bail out partway the way the old
    /// `open`'s `?`-based version did.
    fn build(
        path: PathBuf,
        root: PathBuf,
        workspace: Option<WeakEntity<Workspace>>,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let (last_raw, loaded) = Self::load_from_raw(
            &path,
            &root,
            script_companion::SyncState::default(),
            script_companion::SyncState::default(),
            window,
            cx,
        );

        if loaded.error.is_some() {
            window.dispatch_action(Box::new(ShowInvalidFlowToast(loaded.flow_name.clone())), cx);
        }

        let _theme_sub = cx.observe_global::<Theme>(|this: &mut Self, cx| {
            this.sync_canvas_colors(cx);
        });
        let new_prop_key = cx.new(|cx| InputState::new(window, cx).placeholder("property"));
        let new_prop_value = cx.new(|cx| InputState::new(window, cx).placeholder("value"));

        // `searchable(true)` puts the shared list widget's own search input at
        // the top of the pane and routes each keystroke into the delegate's
        // `perform_search`, so there is no panel-side query state to keep in
        // sync.
        let catalog = cx
            .new(|cx| ListState::new(catalog::CatalogDelegate::new(), window, cx).searchable(true));
        let _catalog_sub = cx.subscribe_in(&catalog, window, |this, _list, event, _window, cx| {
            let ListEvent::Confirm(ix) = event else {
                return;
            };
            // Resolve the index path back to a `type_id` through the delegate
            // rather than trusting the index: the visible rows are a filtered
            // view of the registry, so an index captured before a keystroke
            // could otherwise name a different action than the one shown.
            // `addable_type_id` also drops not-ready rows — `ListState` has no
            // concept of a disabled item, so the styling alone wouldn't stop a
            // "not ready" action being added by click or Enter.
            let type_id = this.catalog.read(cx).delegate().addable_type_id(*ix);
            if let Some(type_id) = type_id {
                this.add_action_at(type_id, None, cx);
            }
        });

        let run_state: SharedRunState = Arc::new(Mutex::new(RunState::default()));
        run_state::register_run_state(path.clone(), run_state.clone());

        let mut this = Self {
            focus_handle: cx.focus_handle(),
            workspace,
            project,
            last_raw,
            _file_watch: None,
            _reload_debounce: gpui::Task::ready(()),
            path,
            root,
            flow_id: loaded.flow_id,
            flow_name: loaded.flow_name,
            script_sync: loaded.script_sync,
            body_sync: loaded.body_sync,
            flow_preserved: loaded.preserved,
            state: loaded.state,
            context_target: loaded.context_target,
            flow: loaded.flow,
            minimap: loaded.minimap,
            controls: loaded.controls,
            catalog,
            _catalog_sub,
            label_input: None,
            _label_sub: None,
            property_rows: Vec::new(),
            selected_node_id: None,
            script_file_input: None,
            _script_file_sub: None,
            new_prop_key,
            new_prop_value,
            running: false,
            status: None,
            error: loaded.error,
            validation_issues: Vec::new(),
            structure: 0,
            saved_content: 0,
            dirty: false,
            conflict: false,
            show_issues: false,
            run_state,
            run_snapshot: RunState::default(),
            run_revision: 0,
            show_log: false,
            log_scroll: ScrollHandle::default(),
            inspector_scroll: ScrollHandle::default(),
            insert_menu: None,
            loop_view: HashMap::new(),
            stale_samples: Default::default(),
            _state_sub: loaded.state_sub,
            _theme_sub,
        };
        this.watch_file(window, cx);
        this.saved_content = Self::content_fingerprint(this.state.read(cx));
        this.revalidate(cx);
        this
    }

    /// Reads `path` and builds the content-dependent half of the panel's
    /// state — everything `build` needs for a fresh tab, and everything
    /// `reload_from_disk` (the file-watcher's callback) needs to refresh an
    /// already-open one. Same infallible-by-design contract as `build`
    /// itself: a missing/unreadable/invalid file still produces a `LoadedFlow`
    /// (empty canvas, `error` set), never a `Result` the caller has to
    /// unwind through. Returns the exact raw file content alongside it (or
    /// `""` if the file couldn't be read) so callers can track `last_raw`.
    ///
    /// `script_baseline`/`body_baseline` are the companion sync baselines to
    /// reconcile against: empty for a tab being opened, the panel's own for a
    /// reload. Carrying them across a reload is what lets a body or script
    /// edited in the raw JSON win over its unchanged companion, where an
    /// empty baseline would hand the edit back to the companion.
    fn load_from_raw(
        path: &Path,
        root: &Path,
        script_baseline: script_companion::SyncState,
        body_baseline: script_companion::SyncState,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> (String, LoadedFlow) {
        let raw_result = std::fs::read_to_string(path);
        let raw = raw_result.as_deref().unwrap_or("").to_string();

        let (mut def, mut error) = match raw_result
            .map_err(|e| format!("couldn't read {}: {e}", path.display()))
            .and_then(|raw| {
                serde_json::from_str::<WorkflowDefinition>(&raw)
                    .map_err(|e| format!("{} isn't a valid flow: {e}", path.display()))
            }) {
            Ok(def) => (def, None),
            Err(e) => {
                let id = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .and_then(|n| n.strip_suffix(".flow.json"))
                    .unwrap_or("flow")
                    .to_string();
                (
                    WorkflowDefinition {
                        id: id.clone(),
                        name: id,
                        actions: Default::default(),
                        outputs: Default::default(),
                    },
                    Some(e),
                )
            }
        };

        // Populated by the companion sync below; stays empty (i.e. "unknown"
        // for every action) when the flow itself failed to load, which is
        // exactly the right baseline - the next sync pass then falls back to
        // the documented "a saved companion is the latest source" rule.
        let mut loaded_sync = script_companion::SyncState::default();
        // Same deal for `http` request bodies, which hydrate from their own
        // companions. Reported separately so a body conflict doesn't get
        // blamed on scripts.
        let mut loaded_body_sync = script_companion::SyncState::default();
        if error.is_none() {
            let mut script_sync = script_baseline;
            if let Err(sync_error) =
                script_companion::sync_inline_scripts(&mut def, path, &mut script_sync)
            {
                error = Some(format!("Couldn't load script companion: {sync_error}"));
            }
            loaded_sync = script_sync;
        }
        // Scripts are files. The sync above has just written each inline
        // script's code to its companion, so the canvas refers to that file
        // from here on, and the next save writes the flow that way. The flow
        // file itself is untouched until then.
        if error.is_none()
            && let Err(convert_error) =
                script_companion::convert_inline_to_file(&mut def, path, root)
        {
            error = Some(format!(
                "Couldn't move inline scripts to files: {convert_error}"
            ));
        }
        if error.is_none() {
            let mut body_sync = body_baseline;
            if let Err(sync_error) =
                http_companion::sync_inline_bodies(&mut def, path, &mut body_sync)
            {
                error = Some(format!(
                    "Couldn't load request body companion: {sync_error}"
                ));
            }
            loaded_body_sync = body_sync;
        }

        if error.is_none() {
            if let Some(layout) = workflow_engine::layout::load(path) {
                workflow_json::apply_saved_layout(&mut def, &layout);
            }
            let newly_placed = workflow_json::auto_layout(&mut def);
            if !newly_placed.positions.is_empty() {
                let mut layout = workflow_engine::layout::load(path).unwrap_or_default();
                layout.positions.extend(newly_placed.positions);
                let _ = workflow_engine::layout::save(path, &layout);
            }
        }

        // Snapshot the flow-level fields the canvas has no node for *before*
        // `def` is consumed, so `save` can round-trip them.
        let preserved = workflow_json::preserved_from(&def);

        let mut flow_state = workflow_json::load(
            &def,
            hex(cx.theme().muted_foreground),
            hex(cx.theme().danger),
        );
        flow_state.fit_view(60.0, CANVAS_SIZE.0, CANVAS_SIZE.1);
        let state = cx.new(|_| flow_state);
        let context_target = Rc::new(RefCell::new(ContextTarget {
            node_id: None,
            position: FlowPoint::new(60.0, 60.0),
        }));

        let panel = cx.weak_entity();
        let flow = cx.new(|cx| {
            let mut graph = FlowGraph::new(state.clone(), cx)
                // `sidebar`/`popover` rather than `background`/`secondary`:
                // in some themes those two are nearly the same value, which
                // reads as a flat, washed-out canvas with no depth (the
                // node cards need to visibly sit "on" the canvas, the way
                // the fixed dark palette this replaced did on purpose).
                .bg_color(hex(cx.theme().sidebar))
                .grid_color(hex(cx.theme().sidebar_border))
                .node_bg_color(hex(cx.theme().popover))
                .node_border_color(hex(cx.theme().border))
                .accent_color(hex(cx.theme().primary))
                .default_renderer(render_leaf);
            let context_target_for_graph = context_target.clone();
            graph = graph.on_canvas_context_menu(move |_, position, node_id, _, _| {
                let mut target = context_target_for_graph.borrow_mut();
                target.position = position;
                target.node_id = node_id;
            });
            let context_target_for_node = context_target.clone();
            graph = graph.on_node_context_menu(move |node_id, _, _, _| {
                context_target_for_node.borrow_mut().node_id = Some(node_id);
            });
            // The "+" on a hovered connection: the canvas reports the click,
            // the panel asks which action to put there.
            let panel = panel.clone();
            graph = graph.on_edge_insert(move |edge_id, position, window, cx| {
                panel
                    .update(cx, |panel, cx| {
                        panel.open_insert_menu(edge_id, position, window, cx)
                    })
                    .ok();
            });
            for (type_id, title) in CONTAINER_TYPES {
                graph = graph
                    .node_renderer(*type_id, move |n, w, cx| {
                        render_container_header(n, title, w, cx)
                    })
                    .container_header(*type_id, move |n, w, cx| {
                        render_container_header(n, title, w, cx)
                    });
            }
            graph
        });
        let minimap =
            cx.new(|_| Minimap::new(state.clone()).container_bounds(CANVAS_SIZE.0, CANVAS_SIZE.1));
        let controls =
            cx.new(|_| Controls::new(state.clone()).container_size(CANVAS_SIZE.0, CANVAS_SIZE.1));

        let flow_id = def.id.clone();
        let flow_name = def.name.clone();
        let state_sub = cx.observe(&state, |this, state, cx| this.on_state_changed(&state, cx));

        (
            raw,
            LoadedFlow {
                error,
                flow_id,
                flow_name,
                preserved,
                state,
                context_target,
                flow,
                minimap,
                controls,
                state_sub,
                script_sync: loaded_sync,
                body_sync: loaded_body_sync,
            },
        )
    }

    /// Runs whenever the canvas state changes. The canvas edits the graph on
    /// its own (Delete, undo, dragging a new edge), so this is where the panel
    /// catches up: when the set of nodes and edges is different from last
    /// time, it removes anything left without a parent and revalidates.
    fn on_state_changed(&mut self, state: &Entity<FlowState>, cx: &mut Context<Self>) {
        // A reload swaps the state; ignore one this panel no longer shows.
        if *state == self.state {
            let structure = Self::structure_fingerprint(state.read(cx));
            if structure != self.structure {
                if state.update(cx, |state, _| Self::prune_orphans(state)) {
                    self.structure = Self::structure_fingerprint(state.read(cx));
                } else {
                    self.structure = structure;
                }
                self.revalidate(cx);
            }
            let dirty = Self::content_fingerprint(state.read(cx)) != self.saved_content;
            if dirty != self.dirty {
                self.dirty = dirty;
                // The tab's unsaved marker follows this.
                cx.emit(());
            }
        }
        cx.notify();
    }

    /// A hash of what the graph is made of (which nodes, nested where, joined
    /// by which edges), ignoring positions, selection and property values.
    fn structure_fingerprint(state: &FlowState) -> u64 {
        use std::hash::{Hash as _, Hasher as _};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        for node in &state.nodes {
            node.id.hash(&mut hasher);
            node.parent_id.hash(&mut hasher);
        }
        for edge in &state.edges {
            edge.source.hash(&mut hasher);
            edge.target.hash(&mut hasher);
            edge.label.hash(&mut hasher);
        }
        hasher.finish()
    }

    /// Removes every node whose container is gone, with its edges, and
    /// returns whether anything was removed.
    ///
    /// The canvas's own Delete key removes only the selected node. Left
    /// alone, a deleted container's body would stay in the state pointing at
    /// a parent that no longer exists, where save never reaches it: the
    /// actions would vanish from the file without ever leaving the screen.
    /// Removing them here makes Delete on a container take its body with it,
    /// the same as the inspector's Delete Node, and the canvas has already
    /// pushed the undo entry that brings all of it back.
    fn prune_orphans(state: &mut FlowState) -> bool {
        let mut removed = false;
        loop {
            let orphans: Vec<NodeId> = state
                .nodes
                .iter()
                .filter(|node| {
                    node.parent_id
                        .as_ref()
                        .is_some_and(|parent| !state.nodes.iter().any(|n| n.id == *parent))
                })
                .map(|node| node.id.clone())
                .collect();
            if orphans.is_empty() {
                break;
            }
            state.nodes.retain(|node| !orphans.contains(&node.id));
            state
                .edges
                .retain(|edge| !orphans.contains(&edge.source) && !orphans.contains(&edge.target));
            removed = true;
        }
        if removed {
            state.rebuild_lookup();
            state.refit_all_containers();
        }
        removed
    }

    /// Re-reads `self.path` and, if its content genuinely differs from
    /// `self.last_raw`, replaces the content-dependent half of this panel's
    /// state with a freshly parsed/laid-out one. Called by `watch_file`'s
    /// debounced file-change handler. The `last_raw` comparison is what
    /// keeps this from re-triggering on this panel's own "Save" (which
    /// updates `last_raw` immediately after writing, before the filesystem
    /// event even arrives) or on duplicate events for one logical write —
    /// see `LoadedFlow`'s doc comment.
    ///
    /// A flow with unsaved edits is not reloaded over: the change is recorded
    /// as a conflict instead, for the user to settle by saving or reloading.
    /// `discard_edits` is that explicit reload.
    fn reload_from_disk(
        &mut self,
        discard_edits: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Compare before loading: a load reconciles the companions, which
        // writes files and moves the baselines, and none of that should
        // happen for this panel's own save or a duplicate event.
        let unchanged = std::fs::read_to_string(&self.path).unwrap_or_default() == self.last_raw;
        if unchanged && !discard_edits {
            return;
        }
        if self.dirty && !discard_edits {
            if !self.conflict {
                self.conflict = true;
                cx.emit(());
                cx.notify();
            }
            return;
        }
        let (raw, loaded) = Self::load_from_raw(
            &self.path,
            &self.root,
            self.script_sync.clone(),
            self.body_sync.clone(),
            window,
            cx,
        );
        self.last_raw = raw;
        self.error = loaded.error;
        self.flow_id = loaded.flow_id;
        self.flow_name = loaded.flow_name;
        self.state = loaded.state;
        self.context_target = loaded.context_target;
        self.flow = loaded.flow;
        self.minimap = loaded.minimap;
        self.controls = loaded.controls;
        self._state_sub = loaded.state_sub;
        // The baselines the load reconciled against, moved on by whatever it
        // synced. A flow that failed to load synced nothing and comes back
        // with empty ones, so the panel's own are kept for the next good load.
        if self.error.is_none() {
            self.script_sync = loaded.script_sync;
            self.body_sync = loaded.body_sync;
        }
        self.flow_preserved = loaded.preserved;
        if self.error.is_some() {
            window.dispatch_action(Box::new(ShowInvalidFlowToast(self.flow_name.clone())), cx);
        }
        self.stale_samples.clear();
        self.mark_saved(cx);
        self.revalidate(cx);
    }

    /// A hash of everything Save writes: each node's identity, nesting,
    /// label, properties and position, and each edge. Run status, validation
    /// rings, selection and the viewport are left out, so running a flow or
    /// panning the canvas doesn't count as an edit.
    fn content_fingerprint(state: &FlowState) -> u64 {
        use std::hash::{Hash as _, Hasher as _};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        for node in &state.nodes {
            node.id.hash(&mut hasher);
            node.parent_id.hash(&mut hasher);
            node.node_type.hash(&mut hasher);
            node.label.hash(&mut hasher);
            node.properties.hash(&mut hasher);
            node.position.x.to_bits().hash(&mut hasher);
            node.position.y.to_bits().hash(&mut hasher);
        }
        for edge in &state.edges {
            edge.source.hash(&mut hasher);
            edge.target.hash(&mut hasher);
            edge.label.hash(&mut hasher);
        }
        hasher.finish()
    }

    /// Records the canvas as it stands as what is on disk: called after a
    /// load and after a successful save.
    fn mark_saved(&mut self, cx: &mut Context<Self>) {
        self.saved_content = Self::content_fingerprint(self.state.read(cx));
        if self.dirty || self.conflict {
            self.dirty = false;
            self.conflict = false;
            cx.emit(());
        }
        cx.notify();
    }

    /// Subscribes to this flow's worktree so external edits (the "Raw" tab,
    /// or any tool outside Zed) refresh the canvas automatically instead of
    /// requiring a manual close/reopen. Debounces briefly to coalesce a
    /// multi-step external write into one reload, then re-reads and applies
    /// via `reload_from_disk` (whose `last_raw` check is what prevents this
    /// panel's own "Save" from re-triggering itself). No-ops quietly if
    /// `self.path` isn't inside a worktree of `self.project` (e.g. a flow
    /// opened from outside any open project).
    fn watch_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(project_path) = self.project.read(cx).find_project_path(&self.path, cx) else {
            return;
        };
        let Some(worktree) = self
            .project
            .read(cx)
            .worktree_for_id(project_path.worktree_id, cx)
        else {
            return;
        };

        self._file_watch = Some(cx.subscribe_in(&worktree, window, {
            let watched_path = project_path.path.clone();
            move |this, _worktree, event, window, cx| {
                let worktree::Event::UpdatedEntries(changes) = event else {
                    return;
                };
                let matched = changes.iter().any(|(entry_path, _, change)| {
                    *entry_path == watched_path && !matches!(change, worktree::PathChange::Removed)
                });
                if !matched {
                    return;
                }

                this._reload_debounce = cx.spawn_in(window, async move |this, cx| {
                    cx.background_executor()
                        .timer(std::time::Duration::from_millis(150))
                        .await;
                    this.update_in(cx, |this, window, cx| {
                        this.reload_from_disk(false, window, cx)
                    })
                    .ok();
                });
            }
        }));
    }

    /// Re-applies the canvas chrome from the active theme. The node renderers
    /// already read `cx.theme()` on every paint, but `FlowGraph`'s
    /// `bg_color`/`grid_color`/`node_bg_color`/`node_border_color`/`accent_color`/
    /// `node_error_color` are one-shot builder values — this keeps them
    /// tracking a live theme switch instead of freezing at open time.
    fn sync_canvas_colors(&mut self, cx: &mut Context<Self>) {
        self.flow.update(cx, |graph, cx| {
            graph.set_theme_colors(
                hex(cx.theme().sidebar),
                hex(cx.theme().sidebar_border),
                hex(cx.theme().popover),
                hex(cx.theme().border),
                hex(cx.theme().primary),
            );
            // The validation ring is theme-driven too, so an invalid node reads
            // as "danger" in light and dark rather than a fixed red that only
            // works on one of them.
            graph.set_node_error_color(hex(cx.theme().danger));
            cx.notify();
        });
    }

    fn selected_node(&self, cx: &App) -> Option<FlowNode> {
        self.state
            .read(cx)
            .nodes
            .iter()
            .find(|n| n.selected)
            .cloned()
    }

    /// The node type of the current selection, if any.
    fn selected_node_type(&self, cx: &Context<Self>) -> Option<SharedString> {
        self.state
            .read(cx)
            .get_node(self.selected_node_id.as_ref()?)
            .and_then(|n| n.node_type.clone())
    }

    /// True when the current selection is one of the synthetic branch nodes
    /// `workflow_json::load` creates. Those are canvas-only scaffolding for an
    /// `If`/`Try`'s nested bodies — they have no `Action` of their own, so
    /// editing or deleting one can't be expressed in `flow.json`.
    fn selected_is_branch_wrapper(&self, cx: &Context<Self>) -> bool {
        self.selected_node_type(cx)
            .as_deref()
            .is_some_and(is_branch_wrapper_type)
    }

    fn selected_branch_label(&self, cx: &Context<Self>) -> &'static str {
        match self.selected_node_type(cx).as_deref() {
            Some(t) => branch_wrapper_label(t),
            None => "A branch container",
        }
    }

    /// Rebuilds the property panel's input entities when the selected node
    /// changes — called at the top of `render` since selection is driven
    /// entirely by `FlowGraph`'s own mouse handling on `self.state`, which
    /// this panel only observes, not owns.
    fn sync_selection(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let selected = self.selected_node(cx);
        let selected_id = selected.as_ref().map(|n| n.id.clone());
        if selected_id == self.selected_node_id {
            return;
        }
        self.selected_node_id = selected_id;
        self.property_rows.clear();
        self.label_input = None;
        self._label_sub = None;
        self.script_file_input = None;
        self._script_file_sub = None;

        let Some(node) = selected else { return };

        let label_input =
            cx.new(|cx| InputState::new(window, cx).default_value(node.label.clone()));
        let node_id = node.id.clone();
        let state = self.state.clone();
        let _label_sub = cx.subscribe(&label_input, move |this, input, event, cx| {
            if matches!(event, InputEvent::Change) {
                let value = input.read(cx).value().to_string();
                this.clear_run_marker(&node_id, cx);
                state.update(cx, |state, cx| {
                    if let Some(n) = state.get_node_mut(&node_id) {
                        n.label = value.into();
                    }
                    cx.notify();
                });
            }
        });
        self.label_input = Some(label_input);
        self._label_sub = Some(_label_sub);

        let is_script = node.node_type.as_deref() == Some("script");
        if is_script {
            if let Some((_, source_str)) =
                node.properties.iter().find(|(k, _)| k.as_ref() == "source")
            {
                if let Ok(ScriptSource::File { ref path, .. }) =
                    serde_json::from_str::<ScriptSource>(source_str.as_ref())
                {
                    let file_path = path.clone();
                    let file_input = cx.new(|cx| {
                        InputState::new(window, cx)
                            .default_value(file_path)
                            .placeholder("path/to/script.py")
                    });
                    let nid = node.id.clone();
                    let state = self.state.clone();
                    let _script_file_sub = cx.subscribe(&file_input, move |this, input, event, cx| {
                        if matches!(event, InputEvent::Change) {
                            let new_path = input.read(cx).value().trim().to_string();
                            this.clear_run_marker(&nid, cx);
                            state.update(cx, |state, cx| {
                                if let Some(n) = state.get_node_mut(&nid) {
                                    if let Some(prop) = n.properties.iter_mut().find(|(k, _)| k.as_ref() == "source") {
                                        if let Ok(mut src) = serde_json::from_str::<ScriptSource>(prop.1.as_ref()) {
                                            if let ScriptSource::File { ref mut path, .. } = src {
                                                *path = new_path;
                                                if let Ok(val) = serde_json::to_string(&src) {
                                                    prop.1 = val.into();
                                                }
                                            }
                                        }
                                    }
                                }
                                cx.notify();
                            });
                            this.revalidate(cx);
                        }
                    });
                    self.script_file_input = Some(file_input);
                    self._script_file_sub = Some(_script_file_sub);
                }
            }
        }

        // Generate rows from the registry so a *missing required* field still
        // gets a row — previously a field absent from `properties` produced no
        // editor at all, so the only signal it was required was a message in
        // the toolbar popover with nothing to type into. Declared fields come
        // first in registry order, then undeclared extras, so the common case
        // reads top-down in the order the runtime expects.
        let node_type = node.node_type.as_deref().unwrap_or_default().to_string();
        let mut emitted: Vec<SharedString> = Vec::new();
        if let Some(def) = workflow_engine::registry::find(&node_type) {
            for field in def.inputs {
                // A script's `source` is edited through the script section
                // (file path, runtime), not as a row of raw JSON.
                if is_script && field.name == "source" {
                    emitted.push("source".into());
                    continue;
                }
                let Some((key, value)) = meta_property(&node_type, &node.properties, field.name)
                else {
                    // Not set. Show the editor anyway when it's required, so
                    // there's somewhere to enter a value; otherwise stay quiet
                    // rather than showing a row for something the flow doesn't
                    // use. The row is keyed by the storage key, so the first
                    // edit creates the property where save looks for it.
                    if field.required {
                        let key: SharedString = storage_key(&node_type, field.name).into();
                        self.push_property_row(key.clone(), "".into(), Some(*field), window, cx);
                        emitted.push(key);
                    }
                    continue;
                };
                self.push_property_row(key.clone(), value, Some(*field), window, cx);
                emitted.push(key);
            }
        }

        for (key, value) in &node.properties {
            if is_script && key.as_ref() == "source" {
                continue;
            }
            // Already emitted above as a declared field.
            if emitted.contains(key) {
                continue;
            }
            // Container meta rows are declared under a different input name
            // (`__limit_count` for `limit`, etc.); resolve those too so they
            // get the right editor instead of falling through as extras.
            let field = workflow_json::declared_field_for_meta(&node_type, key);
            self.push_property_row(key.clone(), value.clone(), field, window, cx);
        }
    }

    /// Builds one inspector row, typed from `field` when the registry
    /// declares it.
    ///
    /// The `FieldKind` decides the control: a `Bool` gets a `Switch` (with
    /// the backing `InputState` kept in step so the property text stays the
    /// single source of truth for save), a `Number` a `NumberInput`, and
    /// `Json`/`Expression` a multi-line `Textarea` — a JSON body or a
    /// JSONLogic expression on one line is unreadable. Everything else is a
    /// plain single-line `Input`.
    ///
    /// `field` is `None` for an undeclared extra, which keeps the old
    /// text-input behavior.
    fn push_property_row(
        &mut self,
        key: SharedString,
        value: SharedString,
        field: Option<InputField>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(node_id) = self.selected_node_id.clone() else {
            return;
        };
        let kind = field.map(|f| f.kind);
        let input = cx.new(|cx| InputState::new(window, cx).default_value(value.clone()));
        // `Json`/`Expression` get the tall editor; see the doc comment.
        let multiline = matches!(kind, Some(FieldKind::Json | FieldKind::Expression));
        let textarea = multiline.then(|| {
            cx.new(|cx| {
                TextareaState::new(window, cx)
                    .rows(4)
                    .default_value(value.to_string())
            })
        });

        let state = self.state.clone();
        let row_key = key.clone();
        // Both widgets write through the same closure so a `Switch` toggle and
        // a typed edit are indistinguishable downstream. `Rc` because a
        // `Json`/`Expression` row has two subscriptions (input + textarea).
        let commit: Commit = Rc::new(
            move |this: &mut Self, value: String, cx: &mut Context<Self>| {
                let row_key = row_key.clone();
                this.clear_run_marker(&node_id, cx);
                state.update(cx, |state, cx| {
                    if let Some(n) = state.get_node_mut(&node_id) {
                        match n.properties.iter().position(|(k, _)| *k == row_key) {
                            Some(ix) => n.properties[ix].1 = value.into(),
                            // A row for a missing required field has no
                            // property behind it until the first edit.
                            None => n.properties.push((row_key, value.into())),
                        }
                    }
                    cx.notify();
                });
                this.revalidate(cx);
            },
        );
        let commit_input = commit.clone();
        let sub = cx.subscribe(&input, move |this, input, event, cx| {
            if matches!(event, InputEvent::Change) {
                let value = input.read(cx).value().to_string();
                commit_input(this, value, cx);
            }
        });

        let mut subs = vec![sub];

        // A field with a closed value set gets a dropdown over it. A saved
        // value outside the set is appended rather than dropped, so the row
        // still shows what the file holds while validation flags it.
        let options = field.map(|f| f.options).unwrap_or_default();
        let select = (!options.is_empty()).then(|| {
            let mut items: Vec<SharedString> = options.iter().map(|o| (*o).into()).collect();
            let current = value.trim();
            let selected = items
                .iter()
                .position(|item| item.eq_ignore_ascii_case(current))
                .or_else(|| {
                    (!current.is_empty()).then(|| {
                        items.push(current.to_string().into());
                        items.len() - 1
                    })
                });
            cx.new(|cx| SelectState::new(items, selected.map(IndexPath::new), window, cx))
        });
        if let Some(select) = &select {
            let commit = commit.clone();
            let input = input.clone();
            subs.push(cx.subscribe_in(
                select,
                window,
                move |this, _, event: &SelectEvent<Vec<SharedString>>, window, cx| {
                    let SelectEvent::Confirm(Some(choice)) = event else {
                        return;
                    };
                    // Keep the backing input in step, as the `Switch` does.
                    input.update(cx, |input, cx| {
                        input.set_value(choice.clone(), window, cx);
                    });
                    commit(this, choice.to_string(), cx);
                },
            ));
        }

        if let Some(area) = &textarea {
            let commit = commit.clone();
            subs.push(cx.subscribe(area, move |this, area, event, cx| {
                if matches!(event, InputEvent::Change) {
                    let value = area.read(cx).value().to_string();
                    commit(this, value, cx);
                }
            }));
        }

        self.property_rows.push(PropertyRow {
            key,
            value: input,
            textarea,
            select,
            kind,
            required: field.is_some_and(|f| f.required),
            help: field.map(|f| f.help).unwrap_or_default().into(),
            type_badge: kind.map(field_kind_badge),
            commit,
            _sub: subs
                .into_iter()
                .reduce(Subscription::join)
                .expect("at least the input subscription"),
        });
    }

    fn on_add_property_click(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(node_id) = self.selected_node_id.clone() else {
            return;
        };
        let key = self.new_prop_key.read(cx).value().trim().to_string();
        if key.is_empty() {
            return;
        }
        let value = self.new_prop_value.read(cx).value().to_string();
        self.clear_run_marker(&node_id, cx);
        self.state.update(cx, |state, cx| {
            if let Some(n) = state.get_node_mut(&node_id) {
                if let Some(prop) = n.properties.iter_mut().find(|(k, _)| k.as_ref() == key) {
                    prop.1 = value.clone().into();
                } else {
                    n.properties
                        .push((key.clone().into(), value.clone().into()));
                }
            }
            cx.notify();
        });
        // A hand-added key that the registry *does* declare still gets the
        // declared editor/required/help treatment — this path is how a user
        // fills in a required field the canvas didn't create a row for.
        let field = workflow_engine::registry::find(
            self.state
                .read(cx)
                .get_node(&node_id)
                .and_then(|n| n.node_type.clone())
                .unwrap_or_default()
                .as_ref(),
        )
        .and_then(|def| def.inputs.iter().copied().find(|f| f.name == key));
        self.push_property_row(key.into(), value.into(), field, window, cx);
        self.new_prop_key
            .update(cx, |input, cx| input.set_value("", window, cx));
        self.new_prop_value
            .update(cx, |input, cx| input.set_value("", window, cx));
        self.revalidate(cx);
    }

    /// Gives the selected node an empty `name` input, so the inspector shows
    /// a row for an optional field the action didn't use yet. A blank
    /// optional input isn't written to the file (see
    /// `workflow_json::unflatten_action_meta`), so adding one and leaving it
    /// empty changes nothing on save.
    fn on_add_optional_field_click(&mut self, name: &'static str, cx: &mut Context<Self>) {
        let Some(node_id) = self.selected_node_id.clone() else {
            return;
        };
        self.state.update(cx, |state, cx| {
            if let Some(n) = state.get_node_mut(&node_id) {
                let key = storage_key(n.node_type.as_deref().unwrap_or_default(), name);
                if !n.properties.iter().any(|(k, _)| k.as_ref() == key) {
                    n.properties.push((key.into(), "".into()));
                }
            }
            cx.notify();
        });
        self.rebuild_property_rows(cx);
    }

    /// Takes `key` out of the selected node's properties altogether.
    fn on_remove_property_click(&mut self, key: SharedString, cx: &mut Context<Self>) {
        let Some(node_id) = self.selected_node_id.clone() else {
            return;
        };
        self.clear_run_marker(&node_id, cx);
        self.state.update(cx, |state, cx| {
            if let Some(n) = state.get_node_mut(&node_id) {
                n.properties.retain(|(k, _)| *k != key);
            }
            cx.notify();
        });
        self.rebuild_property_rows(cx);
    }

    /// Rebuilds the inspector's rows from the node's properties after one was
    /// added or removed. `sync_selection` regenerates them on the next render
    /// when the tracked id differs from the canvas selection, which is the
    /// same route [`Self::select_node`] takes.
    fn rebuild_property_rows(&mut self, cx: &mut Context<Self>) {
        self.selected_node_id = None;
        self.revalidate(cx);
    }

    /// Deletes the selected node, every descendant (a container's own
    /// children), and every edge touching any of them.
    ///
    /// A synthetic branch wrapper is refused rather than deleted. Deleting one
    /// used to recursively remove the whole branch body it stood for, and
    /// since the wrapper exists only to *display* `If`/`Try`'s nested
    /// `actions`/`else`/`catch` maps, there is no JSON to write the removal
    /// to — the user would lose real, saved actions with no undo and no
    /// confirmation. Removing a branch means deleting the action that owns it
    /// (or an action inside it), which is what this still allows.
    fn on_delete_node_click(&mut self, cx: &mut Context<Self>) {
        let Some(node_id) = self.selected_node_id.clone() else {
            return;
        };
        if self.selected_is_branch_wrapper(cx) {
            self.status = Some(format!(
                "{} can't be deleted on its own — it only groups the actions \
                 inside it. Delete one of those actions, or the whole \
                 container, instead.",
                self.selected_branch_label(cx)
            ));
            cx.notify();
            return;
        }
        self.state.update(cx, |state, cx| {
            // The canvas's own Delete/Backspace key already pushes undo
            // before removing selected nodes — this button is the other
            // path to the same destructive operation and was missing it,
            // making a misclick here unrecoverable where the keyboard path
            // wasn't. No confirmation dialog either (see the doc comment
            // above), so undo is the only safety net; it has to actually be
            // there.
            state.push_undo();
            let mut doomed = vec![node_id.clone()];
            let mut frontier = vec![node_id];
            while let Some(id) = frontier.pop() {
                for child in state.children_of(&id) {
                    doomed.push(child.id.clone());
                    frontier.push(child.id.clone());
                }
            }
            state.nodes.retain(|n| !doomed.contains(&n.id));
            state
                .edges
                .retain(|e| !doomed.contains(&e.source) && !doomed.contains(&e.target));
            state.rebuild_lookup();
            state.refit_all_containers();
            cx.notify();
        });
        self.selected_node_id = None;
        self.property_rows.clear();
        self.label_input = None;
        self.script_file_input = None;
        self._script_file_sub = None;
    }

    /// Reconciles `def`'s inline scripts with their companion files, then
    /// pushes any code that came *back* from a companion into the live canvas
    /// state (and the inspector row showing it).
    ///
    /// The push-back is load-bearing, not a convenience: the live state is
    /// what the next Save/Run rebuilds `def` from, so a hydration left only
    /// in the throwaway `def` would let the next sync pass see "inline code
    /// changed since the last sync" and write the stale inline code straight
    /// back over the user's external edit. After this returns `Ok`, the three
    /// representations - canvas state, `def`, companion file - agree for
    /// every inline script.
    fn sync_scripts(
        &mut self,
        def: &mut WorkflowDefinition,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<(), String> {
        let report = script_companion::sync_inline_scripts(def, &self.path, &mut self.script_sync)?;
        let hydrated: Vec<String> = report
            .iter()
            .filter(|(_, outcome)| *outcome == script_companion::Sync::Hydrated)
            .map(|(key, _)| key.clone())
            .collect();
        if hydrated.is_empty() {
            return Ok(());
        }
        self.state.update(cx, |state, cx| {
            for key in &hydrated {
                let action_path: Vec<String> = key.split('/').map(str::to_string).collect();
                let Some(node_id) = action_path.last().map(|id| SharedString::from(id.as_str()))
                else {
                    continue;
                };
                let Some(text) = script_companion::source_text_for_action_path(def, &action_path)
                else {
                    continue;
                };
                if let Some(node) = state.get_node_mut(&node_id)
                    && let Some(property) = node
                        .properties
                        .iter_mut()
                        .find(|(name, _)| name.as_ref() == "source")
                {
                    property.1 = text.into();
                }
            }
            cx.notify();
        });
        // The inspector's `InputState` is a separate entity from the property
        // it edits, so it still holds the pre-hydration text; without this
        // the next keystroke would write that stale text back over the
        // companion edit we just accepted.
        self.refresh_property_rows(window, cx);
        Ok(())
    }

    /// [`Self::sync_scripts`] for `http` request bodies: reconciles `def`'s
    /// bodies with their `<flow>.flow-http/…` companions, then pushes any body
    /// that came *back* from a companion into the live canvas state.
    ///
    /// The push-back matters for the same reason it does for scripts: the live
    /// state is what the next Save rebuilds `def` from, so a body hydrated only
    /// into the throwaway `def` would be seen as "changed since the last sync"
    /// on the next pass and written straight back over the user's external edit.
    fn sync_bodies(
        &mut self,
        def: &mut WorkflowDefinition,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<(), String> {
        let report = http_companion::sync_inline_bodies(def, &self.path, &mut self.body_sync)?;
        let hydrated: Vec<String> = report
            .iter()
            .filter(|(_, outcome)| *outcome == script_companion::Sync::Hydrated)
            .map(|(key, _)| key.clone())
            .collect();
        if hydrated.is_empty() {
            return Ok(());
        }
        self.state.update(cx, |state, cx| {
            for key in &hydrated {
                let action_path: Vec<String> = key.split('/').map(str::to_string).collect();
                let Some(node_id) = action_path.last().map(|id| SharedString::from(id.as_str()))
                else {
                    continue;
                };
                let Some(text) = http_companion::body_text_for_action_path(def, &action_path)
                else {
                    continue;
                };
                if let Some(node) = state.get_node_mut(&node_id)
                    && let Some(property) = node
                        .properties
                        .iter_mut()
                        .find(|(name, _)| name.as_ref() == "body")
                {
                    property.1 = text.into();
                }
            }
            cx.notify();
        });
        self.refresh_property_rows(window, cx);
        Ok(())
    }

    /// Re-seeds every property input from the live state, for when the state
    /// was changed programmatically (a companion hydration) rather than
    /// through the inputs themselves.
    fn refresh_property_rows(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(node_id) = self.selected_node_id.clone() else {
            return;
        };
        // Read the current values out and drop the state borrow before
        // touching the input entities: `InputState::update` needs `&mut cx`,
        // which a live `state.read(cx)` guard would still be holding.
        let values: Vec<(Entity<InputState>, Option<Entity<TextareaState>>, String)> = {
            let state = self.state.read(cx);
            let Some(node) = state.nodes.iter().find(|node| node.id == node_id) else {
                return;
            };
            self.property_rows
                .iter()
                .map(|row| {
                    let value = node
                        .properties
                        .iter()
                        .find(|(name, _)| *name == row.key)
                        .map(|(_, value)| value.to_string())
                        .unwrap_or_default();
                    (row.value.clone(), row.textarea.clone(), value)
                })
                .collect()
        };
        for (input, textarea, value) in values {
            // A `Json`/`Expression` row shows its textarea, so that is the
            // editor that must not be left holding the old text.
            if let Some(textarea) = textarea {
                textarea.update(cx, |area, cx| area.set_value(value.clone(), window, cx));
            }
            input.update(cx, |input, cx| input.set_value(value, window, cx));
        }
        if let Some(file_input) = &self.script_file_input {
            let state = self.state.read(cx);
            if let Some(node) = state.nodes.iter().find(|node| node.id == node_id) {
                if let Some((_, source_str)) =
                    node.properties.iter().find(|(k, _)| k.as_ref() == "source")
                {
                    if let Ok(ScriptSource::File { ref path, .. }) =
                        serde_json::from_str::<ScriptSource>(source_str.as_ref())
                    {
                        let path_val = path.clone();
                        file_input.update(cx, |input, cx| input.set_value(path_val, window, cx));
                    }
                }
            }
        }
    }

    fn on_open_script_click(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(node_id) = self.selected_node_id.as_ref().map(|id| id.to_string()) else {
            return;
        };
        let (action_path, source) = {
            let state = self.state.read(cx);
            let (definition, _) = workflow_json::save(
                &state,
                self.flow_id.clone(),
                self.flow_name.clone(),
                &self.flow_preserved,
            );
            let Some(action_path) =
                script_companion::action_path_for_node(&definition, &state, &node_id)
            else {
                self.error = Some(format!("Couldn't resolve Script action {node_id}"));
                cx.notify();
                return;
            };
            let Some(source) = script_companion::source_for_action_path(&definition, &action_path)
            else {
                self.error = Some(format!("Script action {node_id} has no valid source input"));
                cx.notify();
                return;
            };
            (action_path, source)
        };

        // A `File` source names a workspace file the executor reads directly;
        // there is no companion to materialize, so this just opens it. The
        // runtime is the executor's own inference, checked here so the panel
        // can say *why* a run would fail before the run is attempted.
        let path = match source {
            ScriptSource::File {
                path: script_path,
                runtime,
            } => {
                // `solution_root.join(path)`, the executor's own resolution
                // (executors.rs), so this opens the file that would run.
                let absolute = self.root.join(&script_path);
                if let Err(error) = inferred_runtime(&absolute, runtime) {
                    self.error = Some(error);
                    cx.notify();
                    return;
                }
                if !absolute.is_file() {
                    self.error = Some(format!(
                        "Script file {} doesn't exist (resolved to {})",
                        script_path,
                        absolute.display()
                    ));
                    cx.notify();
                    return;
                }
                absolute
            }
            ScriptSource::Inline { runtime, .. } => {
                if runtime == ScriptRuntime::Dotnet {
                    self.error =
                        Some("Inline .NET scripts aren't supported; use File mode".to_string());
                    cx.notify();
                    return;
                }
                let (definition, _) = {
                    let state = self.state.read(cx);
                    workflow_json::save(
                        &state,
                        self.flow_id.clone(),
                        self.flow_name.clone(),
                        &self.flow_preserved,
                    )
                };
                let path = match script_companion::companion_path(&self.path, &action_path, runtime)
                {
                    Ok(path) => path,
                    Err(error) => {
                        self.error = Some(error);
                        cx.notify();
                        return;
                    }
                };
                // Write the inspector's code through to the companion rather
                // than creating it once and never touching it again, which is
                // what made an inline edit vanish on the next save. A
                // companion edited outside this panel in the meantime is a
                // conflict, not something to overwrite.
                if let Err(error) = script_companion::write_through(
                    &definition,
                    &self.path,
                    &action_path,
                    &mut self.script_sync,
                ) {
                    self.error = Some(format!("Couldn't sync script companion: {error}"));
                    cx.notify();
                    return;
                }
                path
            }
        };

        if self
            .project
            .read(cx)
            .project_path_for_absolute_path(&path, cx)
            .is_none()
        {
            self.error = Some(format!(
                "{} isn't inside an open project, so it can't be opened in an editor",
                path.display()
            ));
            cx.notify();
            return;
        }

        self.error = None;
        self.status = Some(format!("Opening {}", path.display()));
        window.dispatch_action(Box::new(ViewScriptCompanion(path)), cx);
        cx.notify();
    }

    /// Opens the selected `http` action's request body as its own JSON editor
    /// tab, materializing the companion on first use.
    ///
    /// The inspector's `body` row stays editable and stays the source of truth;
    /// this button is how you get a real JSON editor for it, with the body and
    /// action schemas applied. Mirrors [`Self::on_open_script_click`]: the
    /// inspector's current body is written through first, so a companion
    /// edited out of band since the last sync is reported as a conflict rather
    /// than silently overwritten.
    fn on_edit_body_click(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(selected) = self.selected_node_id.clone() else {
            return;
        };
        let node_id = selected.to_string();

        // `body` is optional (it's meaningless for GET/DELETE), so a request
        // may not have one yet, and a blank one isn't saved. Asking to edit
        // the body is asking for one, so start it as an empty object rather
        // than refusing.
        let has_body = self.state.read(cx).get_node(&selected).is_some_and(|n| {
            n.properties
                .iter()
                .any(|(k, v)| k.as_ref() == "body" && !v.trim().is_empty())
        });
        if !has_body {
            self.state.update(cx, |state, cx| {
                if let Some(n) = state.get_node_mut(&selected) {
                    match n.properties.iter().position(|(k, _)| k.as_ref() == "body") {
                        Some(ix) => n.properties[ix].1 = "{}".into(),
                        None => n.properties.push(("body".into(), "{}".into())),
                    }
                }
                cx.notify();
            });
            self.rebuild_property_rows(cx);
        }

        let action_path = {
            let state = self.state.read(cx);
            let (definition, _) = workflow_json::save(
                &state,
                self.flow_id.clone(),
                self.flow_name.clone(),
                &self.flow_preserved,
            );
            let Some(action_path) =
                script_companion::action_path_for_node(&definition, &state, &node_id)
            else {
                self.error = Some(format!("Couldn't resolve HTTP action {node_id}"));
                cx.notify();
                return;
            };
            action_path
        };

        let path = match http_companion::companion_path(&self.path, &action_path) {
            Ok(path) => path,
            Err(error) => {
                self.error = Some(error);
                cx.notify();
                return;
            }
        };

        // A body that isn't valid JSON has no canonical form, but it is still
        // the user's work. `write_through` round-trips malformed text verbatim,
        // so writing it unconditionally hands it over as-is instead of losing
        // it to a parse failure (and leaves a real file behind to open).
        let (definition, _) = {
            let state = self.state.read(cx);
            workflow_json::save(
                &state,
                self.flow_id.clone(),
                self.flow_name.clone(),
                &self.flow_preserved,
            )
        };
        if let Err(error) = http_companion::write_through(
            &definition,
            &self.path,
            &action_path,
            &mut self.body_sync,
        ) {
            self.error = Some(format!("Couldn't sync request body companion: {error}"));
            cx.notify();
            return;
        }

        if self
            .project
            .read(cx)
            .project_path_for_absolute_path(&path, cx)
            .is_none()
        {
            self.error = Some(format!(
                "{} isn't inside an open project, so it can't be opened in an editor",
                path.display()
            ));
            cx.notify();
            return;
        }

        self.error = None;
        self.status = Some(format!("Opening {}", path.display()));
        window.dispatch_action(Box::new(ViewHttpBody(path)), cx);
        cx.notify();
    }

    /// Re-runs `validate_definition` over the current canvas state and
    /// stores the result in `self.validation_issues`. Called after every
    /// load, save, and property edit — cheap enough to call synchronously
    /// (it is a pure in-memory traversal of the action tree).
    fn revalidate(&mut self, cx: &mut Context<Self>) {
        let (def, _) = {
            let state = self.state.read(cx);
            workflow_json::save(
                &state,
                self.flow_id.clone(),
                self.flow_name.clone(),
                &self.flow_preserved,
            )
        };
        self.validation_issues = validate_definition(&def);
        self.apply_validation_borders(cx);
        cx.notify();
    }

    /// Mirrors `self.validation_issues` onto the canvas as
    /// `FlowNode::validation_error`, so a broken node is visibly broken
    /// instead of only being discoverable by selecting it and reading the
    /// inspector.
    ///
    /// A whole-list rewrite rather than a diff, because the issue list is
    /// recomputed from scratch on every keystroke anyway and a node's
    /// *un*-flagging is the case that matters most — a stale red ring the user
    /// can't clear is worse than one they have to wait a frame for.
    fn apply_validation_borders(&mut self, cx: &mut Context<Self>) {
        let flagged: Vec<String> = self
            .validation_issues
            .iter()
            .filter(|issue| issue.severity == ValidationSeverity::Error)
            .map(|issue| issue.node_path.clone())
            .collect();
        self.state.update(cx, |state, cx| {
            for node in &mut state.nodes {
                // `node_path` uses the same prefix convention as the node ids
                // `WorkflowLayout` is keyed by, so a node is flagged when its
                // own id is a path prefix of the issue's path — that's what
                // makes a *container* light up for an error on something
                // nested three levels inside it.
                node.validation_error = flagged
                    .iter()
                    .any(|path| Self::issue_targets_node(path, node.id.as_ref()));
            }
            cx.notify();
        });
    }

    /// Whether an issue at `issue_path` should mark the node `node_id`.
    ///
    /// The two strings are *not* in the same frame, which is the whole subtlety
    /// here. A node's own id is bare (`a1`), while the path an issue is
    /// reported at carries the full ancestor chain (`foreach-1/if-1/then/a1`) —
    /// the same `{prefix}{id}` convention `workflow_json::build_actions_map`
    /// uses for its layout keys. So equality never works for a nested action,
    /// and the match has to be "does this id appear *anywhere* along the
    /// chain", which covers all three positions at once:
    ///
    /// - last segment → the issue is *on* this action;
    /// - first segment → the issue is inside this container;
    /// - a middle segment → the issue is inside a container nested in this one.
    ///
    /// Flagging ancestors is deliberate: an error buried in a container body
    /// isn't visible from the canvas, so the nearest enclosing container the
    /// user can actually see has to light up too.
    ///
    /// Matching a bare segment is safe against false positives because node ids
    /// are globally unique on the canvas and contain no `/` —
    /// `push_action_node` suffixes `type_id-N` until `FlowState::get_node`
    /// misses, so no id can straddle or imitate a path boundary.
    ///
    /// A flow-level issue (empty `node_path` — a `runAfter` cycle or dangling
    /// reference, which `validate_definition` attributes to the definition
    /// root) flags nothing, because there is no node to flag. It stays
    /// reachable through the issues overlay instead.
    fn issue_targets_node(issue_path: &str, node_id: &str) -> bool {
        if issue_path.is_empty() {
            return false;
        }
        issue_path.split('/').any(|segment| segment == node_id)
    }

    /// The canvas node an issue at `issue_path` is reported on, or `None`
    /// for a flow-level issue.
    ///
    /// The path carries the ancestor chain (`foreach-1/if-1/then/a1`) while a
    /// node's id is bare, so the node is the last segment, not the whole
    /// path. See [`Self::issue_targets_node`] for the same distinction.
    fn issue_node_id(issue_path: &str) -> Option<NodeId> {
        let id = issue_path.rsplit('/').next()?;
        (!id.is_empty()).then(|| id.to_string().into())
    }

    fn on_save_click(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let (mut def, layout) = {
            let state = self.state.read(cx);
            workflow_json::save(
                state,
                self.flow_id.clone(),
                self.flow_name.clone(),
                &self.flow_preserved,
            )
        };
        if let Err(error) = self.sync_scripts(&mut def, window, cx) {
            self.error = Some(format!("Couldn't sync script companion: {error}"));
            cx.notify();
            return;
        }
        if let Err(error) = self.sync_bodies(&mut def, window, cx) {
            self.error = Some(format!("Couldn't sync request body companion: {error}"));
            cx.notify();
            return;
        }
        let json = match serde_json::to_string_pretty(&def) {
            Ok(json) => json,
            Err(e) => {
                self.error = Some(format!("Couldn't serialize flow: {e}"));
                cx.notify();
                return;
            }
        };
        if let Err(e) = std::fs::write(&self.path, &json) {
            self.error = Some(format!("Couldn't write {}: {e}", self.path.display()));
            cx.notify();
            return;
        }
        // Update before the file-watcher's event even arrives — that's what
        // makes `reload_from_disk`'s `last_raw` comparison treat this save
        // as a no-op instead of an unwanted rebuild/flicker right after
        // saving. See `LoadedFlow`'s doc comment.
        self.last_raw = json;
        if let Err(e) = workflow_engine::layout::save(&self.path, &layout) {
            self.error = Some(format!("Saved the flow, but couldn't save its layout: {e}"));
            cx.notify();
            return;
        }
        self.error = None;
        self.status = Some("Saved".to_string());
        self.write_script_support(cx);
        self.mark_saved(cx);
        self.revalidate(cx);
    }

    fn on_run_click(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.running {
            return;
        }
        let (mut def, _layout) = {
            let state = self.state.read(cx);
            workflow_json::save(
                state,
                self.flow_id.clone(),
                self.flow_name.clone(),
                &self.flow_preserved,
            )
        };
        if let Err(error) = self.sync_scripts(&mut def, window, cx) {
            self.error = Some(format!("Couldn't sync script companion: {error}"));
            cx.notify();
            return;
        }
        if let Err(error) = self.sync_bodies(&mut def, window, cx) {
            self.error = Some(format!("Couldn't sync request body companion: {error}"));
            cx.notify();
            return;
        }

        // A script written against the helpers needs them there to start.
        self.write_script_support(cx);

        let mut action_meta = HashMap::new();
        collect_action_meta(&def.actions, &mut action_meta);

        // Reset the shared run state, then let the sink replay into it
        // off-thread as `run_workflow` progresses.
        let run_state = self.run_state.clone();
        let started_at = Instant::now();
        {
            let mut st = run_state.lock().unwrap();
            *st = RunState::default();
            st.started_at = Some(started_at);
        }
        self.run_snapshot = RunState::default();
        self.run_revision = 0;
        // A new run replaces every sample, so nothing is stale any more.
        self.stale_samples.clear();
        self.state.update(cx, |state, cx| {
            for node in &mut state.nodes {
                node.accent_border = None;
            }
            cx.notify();
        });

        // `records` mirrors final outcomes for the persistent `flows_panel`
        // run history (`flows_panel_history_append` below); the shared
        // `RunState` carries the same events plus the `Running` phase and
        // the live transcript.
        let records: Arc<Mutex<Vec<ActionHistoryRecord>>> = Arc::new(Mutex::new(Vec::new()));
        let records_for_sink = records.clone();
        let sink_state = run_state.clone();
        let sink_meta = action_meta.clone();
        let status = StatusSink::with_scope(move |scope, action_id, status, detail| {
            let phase = match status {
                RunStatus::Running => RunPhase::Running,
                RunStatus::Succeeded => RunPhase::Succeeded,
                RunStatus::Failed => RunPhase::Failed,
                RunStatus::Skipped => RunPhase::Skipped,
            };
            let (name, type_id) = sink_meta
                .get(action_id)
                .cloned()
                .unwrap_or_else(|| (action_id.to_string(), "unknown".to_string()));
            sink_state.lock().unwrap().record(
                started_at,
                scope,
                action_id,
                phase,
                detail.clone(),
                name.clone(),
                type_id.clone(),
            );
            if matches!(phase, RunPhase::Running) {
                return;
            }
            let outcome = match phase {
                RunPhase::Succeeded => RunOutcome::Succeeded,
                RunPhase::Failed => RunOutcome::Failed,
                RunPhase::Skipped => RunOutcome::Skipped,
                RunPhase::Running => return,
            };
            records_for_sink.lock().unwrap().push(ActionHistoryRecord {
                id: action_id.to_string(),
                name,
                type_id,
                outcome,
                message: detail,
                response: None,
            });
        });

        self.error = None;
        self.running = true;
        self.status = Some("Running\u{2026}".to_string());
        self.start_run_poll(window, cx);
        cx.notify();

        let root = self.root.clone();
        let namespace = root.to_string_lossy().to_string();
        let flow_id = self.flow_id.clone();
        let db = cx.global::<db::AppDatabase>();
        let store = db::kvp::KeyValueStore::from_app_db(db);
        let run_state_for_finish = run_state.clone();

        cx.spawn_in(window, async move |this, cx| {
            let result = run_workflow(def, root, status).await;
            let outcome = if result.is_ok() {
                RunOutcome::Succeeded
            } else {
                RunOutcome::Failed
            };
            {
                let mut st = run_state_for_finish.lock().unwrap();
                st.outcome = Some(outcome);
                st.error = result.as_ref().err().cloned();
                st.finished_at = Some(Instant::now());
                st.revision += 1;
            }

            let history_entry = RunHistoryEntry {
                id: format!("run-{}", chrono::Utc::now().timestamp_millis()),
                time: chrono::Utc::now().to_rfc3339(),
                outcome,
                actions: records.lock().unwrap().clone(),
            };
            let _ = flows_panel_history_append(&store, &namespace, &flow_id, history_entry).await;

            let _ = this.update_in(cx, |this, _window, cx| {
                this.running = false;
                this.status = Some(match &result {
                    Ok(()) => "Run succeeded".to_string(),
                    Err(e) => format!("Run failed: {e}"),
                });
                this.sync_run(cx);
                // The run's results are what the type declarations for the
                // scripts are drawn from.
                this.write_script_support(cx);
                cx.notify();
            });
        })
        .detach();
    }

    /// Merges the shared `RunState` into `self.run_snapshot` and repaints the
    /// nodes' status borders. Returns whether anything new had arrived since
    /// the last snapshot (drives the poll loop's re-render decision).
    fn sync_run(&mut self, cx: &mut Context<Self>) -> bool {
        let (st, revision) = {
            let locked = self.run_state.lock().unwrap();
            (locked.clone(), locked.revision)
        };
        if revision == self.run_revision {
            return false;
        }
        self.apply_status_colors(&st, cx);
        self.run_snapshot = st;
        self.run_revision = revision;
        true
    }

    /// Applies phase-colored `accent_border`s (Running = primary,
    /// Succeeded = success, Failed = danger, Skipped = none) onto the live
    /// `FlowState` nodes — `gpui_flow::FlowNode::accent_border` is the
    /// documented hook for exactly this ("a running/succeeded/failed node in
    /// a workflow executor sets a raw color; this crate just draws it").
    fn apply_status_colors(&mut self, st: &RunState, cx: &mut Context<Self>) {
        let running_hex = hex(cx.theme().primary);
        let success_hex = hex(cx.theme().success);
        let danger_hex = hex(cx.theme().danger);
        self.state.update(cx, |state, cx| {
            for (id, run) in &st.nodes {
                let color = match run.phase {
                    RunPhase::Running => Some(running_hex),
                    RunPhase::Succeeded => Some(success_hex),
                    RunPhase::Failed => Some(danger_hex),
                    RunPhase::Skipped => None,
                };
                if let Some(node) = state
                    .nodes
                    .iter_mut()
                    .find(|n| n.id.as_ref() == id.as_str())
                {
                    if node.accent_border != color {
                        node.accent_border = color;
                    }
                }
            }
            cx.notify();
        });
    }

    /// Drives the UI off the shared run state while a run is in flight — a
    /// ~10 Hz poll that merges new events into [`Self::run_snapshot`] and
    /// re-renders, exiting once the run has reported `finished_at`.
    fn start_run_poll(&self, window: &mut Window, cx: &mut Context<Self>) {
        let run_state = self.run_state.clone();
        cx.spawn_in(window, async move |this, cx| {
            loop {
                let done = run_state.lock().unwrap().finished_at.is_some();
                if this
                    .update_in(cx, |this, _window, cx| {
                        if this.sync_run(cx) {
                            cx.notify();
                        }
                    })
                    .is_err()
                {
                    break;
                }
                if done {
                    break;
                }
                cx.background_executor()
                    .timer(Duration::from_millis(100))
                    .await;
            }
        })
        .detach();
    }

    /// The most recent action still `Running` (parallel leaves can be
    /// active at the same time) — the toolbar's live "now running" line.
    fn running_action(&self) -> Option<ActionRun> {
        self.run_snapshot
            .nodes
            .values()
            .filter(|r| r.phase == RunPhase::Running)
            .max_by_key(|r| r.started_at)
            .cloned()
    }

    /// Editing a node invalidates its run result: the user chose to keep
    /// phase colors "until you edit", so an edited node drops its marker
    /// (and its accent border) rather than showing stale result data.
    fn clear_run_marker(&mut self, node_id: &NodeId, cx: &mut Context<Self>) {
        self.mark_samples_stale(node_id, cx);
        let id = node_id.to_string();
        if self.run_state.lock().unwrap().nodes.remove(&id).is_none() {
            return;
        }
        {
            let mut run_state = self.run_state.lock().unwrap();
            // Its per-iteration results describe the old version too.
            for results in run_state.iterations.values_mut() {
                results.remove(&id);
            }
            run_state.revision += 1;
        }
        self.state.update(cx, |state, cx| {
            if let Some(node) = state.get_node_mut(node_id) {
                node.accent_border = None;
            }
            cx.notify();
        });
        self.sync_run(cx);
        cx.notify();
    }

    fn on_clear_results_click(&mut self, cx: &mut Context<Self>) {
        *self.run_state.lock().unwrap() = RunState::default();
        self.run_snapshot = RunState::default();
        self.run_revision = 0;
        self.stale_samples.clear();
        self.status = None;
        self.state.update(cx, |state, cx| {
            for node in &mut state.nodes {
                node.accent_border = None;
            }
            cx.notify();
        });
        cx.notify();
    }

    fn on_toggle_log_click(&mut self, cx: &mut Context<Self>) {
        self.show_log = !self.show_log;
        if self.show_log {
            self.log_scroll.scroll_to_bottom();
        }
        cx.notify();
    }

    /// Selects a node on the canvas (used by clicking a transcript row) by
    /// writing `selected` straight onto `FlowState` — the same field
    /// `FlowGraph`'s mouse handling drives, so the selection ring and the
    /// property pane pick it up on the next render.
    fn select_node(&mut self, node_id: NodeId, cx: &mut Context<Self>) {
        self.state.update(cx, |state, cx| {
            for node in &mut state.nodes {
                node.selected = node.id == node_id;
            }
            cx.notify();
        });
        self.selected_node_id = None;
        cx.notify();
    }

    fn on_fit_view_click(&mut self, cx: &mut Context<Self>) {
        self.state.update(cx, |state, cx| {
            state.fit_view(60.0, CANVAS_SIZE.0, CANVAS_SIZE.1);
            cx.notify();
        });
    }

    /// Brings the node an issue was reported against into view: selects it
    /// (which the inspector follows) and centers the viewport on it.
    ///
    /// Selecting alone isn't enough — the offending node may well be scrolled
    /// or panned off-screen, in which case "click the error" appears to do
    /// nothing. The centering uses the *laid-out* canvas box rather than
    /// `CANVAS_SIZE` for the same reason [`Self::canvas_center`] does: a
    /// viewport centered on coordinates that assume a canvas which isn't the
    /// size on screen lands the node in the wrong place.
    fn reveal_node(&mut self, node_id: NodeId, cx: &mut Context<Self>) {
        self.select_node(node_id.clone(), cx);
        let bounds = self.flow.read(cx).panel_bounds();
        let width = bounds.size.width;
        let height = bounds.size.height;
        let container = (width > px(0.0) && height > px(0.0))
            .then(|| (width.as_f32(), height.as_f32()))
            .unwrap_or(CANVAS_SIZE);
        self.state.update(cx, |state, cx| {
            // `absolute_position` resolves any parent chain, so a node nested
            // inside a container is centered where it's actually drawn rather
            // than at its parent-relative coordinates.
            let point = state.absolute_position(&node_id).unwrap_or_default();
            state.set_center(point.x, point.y, container.0, container.1);
            cx.notify();
        });
    }

    /// Adds `type_id` at the last right-click's spot: below the node that was
    /// clicked if there was one (also connecting from it), otherwise at the
    /// click's own canvas position. This is the pre-existing quick-add
    /// gesture, preserved unchanged; it now shares
    /// [`Self::push_action_node`] with the catalog so all three entry points
    /// build identical nodes.
    fn add_action(&mut self, type_id: &'static str, cx: &mut Context<Self>) {
        let context = self.context_target.borrow().clone();
        let anchor = {
            let state = self.state.read(cx);
            context
                .node_id
                .as_ref()
                .and_then(|id| state.get_node(id))
                .map(|source| Self::beside_node(state, source))
        };
        // No node was reported for the click: it may still have landed inside
        // a container's box, in which case the action belongs in that body.
        let anchor = anchor
            .or_else(|| self.container_at(context.position, cx))
            .unwrap_or(ActionAnchor {
                position: context.position,
                parent_id: None,
                source_id: None,
                local_position: None,
            });
        self.push_action_node(type_id, anchor, cx);
    }

    /// Adds `type_id` from the catalog, where there is no right-click
    /// position to work from.
    ///
    /// `at` is `Some` for a drag-and-drop and places the node exactly there,
    /// nesting into whatever container the user dropped onto. `at` is `None`
    /// for click-to-add and Enter-to-add, which anchor below the current
    /// selection and fall back to the canvas' center, so a flow's very first
    /// action lands in view rather than off at the world origin.
    ///
    /// Either way the new node ends up selected, so its inspector opens
    /// ready for the required inputs the registry declares.
    fn add_action_at(
        &mut self,
        type_id: &'static str,
        at: Option<FlowPoint>,
        cx: &mut Context<Self>,
    ) {
        let anchor = match at {
            Some(position) => self.container_at(position, cx).unwrap_or(ActionAnchor {
                position,
                parent_id: None,
                source_id: None,
                local_position: None,
            }),
            None => {
                let below_selection = {
                    let state = self.state.read(cx);
                    state
                        .nodes
                        .iter()
                        .find(|node| node.selected)
                        .map(|source| Self::beside_node(state, source))
                };
                below_selection.unwrap_or(ActionAnchor {
                    position: self.canvas_center(cx),
                    parent_id: None,
                    source_id: None,
                    local_position: None,
                })
            }
        };
        self.push_action_node(type_id, anchor, cx);
    }

    /// The flow-space spot, owning container, and upstream node for a new
    /// action added below `source`: directly beneath its footprint, inside
    /// the same container, and connected from it.
    fn below_node(state: &FlowState, source: &FlowNode) -> ActionAnchor {
        let height = if state.is_container(&source.id) {
            source
                .container_size
                .map(|(_, height)| height)
                .unwrap_or(220.0)
        } else {
            workflow_json::LEAF_SIZE.1
        };
        // `source.position` is in its parent's frame when it is nested, so the
        // spot below it is too.
        let below = FlowPoint::new(source.position.x, source.position.y + height + 30.0);
        ActionAnchor {
            position: below,
            parent_id: source.parent_id.clone(),
            source_id: Some(source.id.clone()),
            local_position: source.parent_id.is_some().then_some(below),
        }
    }

    /// Where an action added "at" `source` goes when there is no cursor
    /// position to place it by: a catalog click with `source` selected, or
    /// Add Action from `source`'s right-click menu.
    ///
    /// For a loop, a branch, or an `If`/`Try`, that is inside it, since
    /// filling the body is what selecting a container is for and a branch has
    /// no "below" at all. For an ordinary action it is the next step after
    /// it.
    fn beside_node(state: &FlowState, source: &FlowNode) -> ActionAnchor {
        Self::inside_node(state, source).unwrap_or_else(|| Self::below_node(state, source))
    }

    /// The end of `node`'s body, or `None` when `node` has no body.
    ///
    /// The new action is chained after the body's last one when there is
    /// exactly one action nothing else follows. Otherwise it is left
    /// unconnected, because there is no single "previous step" to pick.
    fn inside_node(state: &FlowState, node: &FlowNode) -> Option<ActionAnchor> {
        let body = Self::body_container(state, node)?;
        let children = state.children_of(&body.id);
        let mut tails = children.iter().filter(|child| {
            !state.edges.iter().any(|edge| edge.source == child.id)
        });
        let source_id = match (tails.next(), tails.next()) {
            (Some(tail), None) => Some(tail.id.clone()),
            _ => None,
        };
        Some(ActionAnchor {
            position: FlowPoint::default(),
            parent_id: Some(body.id.clone()),
            source_id,
            local_position: Some(Self::next_slot(state, &body.id)),
        })
    }

    /// The node whose children are `node`'s body: the node itself for a loop
    /// or a branch wrapper, the True/Try wrapper for an `If`/`Try`, and `None`
    /// for a leaf. Decided by type, not by whether the node has children yet,
    /// so an empty container can take its first action.
    fn body_container<'a>(state: &'a FlowState, node: &'a FlowNode) -> Option<&'a FlowNode> {
        let node_type = node.node_type.as_deref()?;
        if matches!(node_type, "Foreach" | "Until") || is_branch_wrapper_type(node_type) {
            return Some(node);
        }
        let [body, _] = workflow_json::branch_types(node_type)?;
        state.get_node(&format!("{}{body}", node.id).into())
    }

    /// Where the next action goes inside `container` when the cursor isn't a
    /// usable position: under its lowest child, or at the body's first slot.
    fn next_slot(state: &FlowState, container: &NodeId) -> FlowPoint {
        let lowest = state
            .children_of(container)
            .iter()
            .map(|child| child.position.y)
            .fold(None, |lowest: Option<f32>, y| {
                Some(lowest.map_or(y, |lowest| lowest.max(y)))
            });
        FlowPoint::new(
            workflow_json::CHILD_X,
            lowest.map_or(workflow_json::CHILD_Y0, |y| {
                y + workflow_json::LEAF_SIZE.1 + 30.0
            }),
        )
    }

    /// The flow-space center of the visible canvas, the default insertion
    /// point when nothing is selected. Read back off `FlowGraph`'s laid-out
    /// box so it tracks the pane's real size — the hardcoded `CANVAS_SIZE`
    /// that `fit_view` and auto-layout use is a layout hint, not a claim
    /// about how much canvas is actually on screen.
    fn canvas_center(&self, cx: &App) -> FlowPoint {
        let bounds = self.flow.read(cx).panel_bounds();
        if bounds.size.width <= px(0.0) || bounds.size.height <= px(0.0) {
            // Not laid out yet (first frame) — the origin beats dividing by an
            // empty rect.
            return FlowPoint::new(0.0, 0.0);
        }
        let viewport = self.state.read(cx).viewport;
        self.flow.read(cx).flow_point_at(
            Point::new(
                bounds.origin.x + bounds.size.width / 2.0,
                bounds.origin.y + bounds.size.height / 2.0,
            ),
            viewport,
        )
    }

    /// The deepest container enclosing `position`, so a drop lands *inside*
    /// the container the user aimed at rather than on top of it. `None` for
    /// open canvas. Walks up from whatever node is under the point, so
    /// dropping onto a child of a container still nests in that container.
    fn container_at(&self, position: FlowPoint, cx: &App) -> Option<ActionAnchor> {
        let state = self.state.read(cx);
        let under = state.node_at_flow((position.x, position.y))?;
        let mut current = state.get_node(&under);
        while let Some(node) = current {
            if let Some(body) = Self::body_container(state, node) {
                // The cursor is a real position inside the body only when it
                // was over that body and the body already has a drawn box.
                // An empty one is drawn as a small node, and an `If`/`Try`
                // hands the drop on to its first branch.
                let over_body = body.id == node.id && state.is_container(&body.id);
                return Some(ActionAnchor {
                    position,
                    parent_id: Some(body.id.clone()),
                    source_id: None,
                    local_position: (!over_body).then(|| Self::next_slot(state, &body.id)),
                });
            }
            current = node.parent_id.as_ref().and_then(|id| state.get_node(id));
        }
        None
    }

    /// Opens the action menu for the "+" on the connection `edge_id`, at the
    /// click. Picking an action puts it on that connection.
    fn open_insert_menu(
        &mut self,
        edge_id: gpui_flow::EdgeId,
        position: Point<gpui::Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let panel = cx.weak_entity();
        let menu = PopupMenu::build(window, cx, move |menu, window, cx| {
            action_type_menu(
                menu.label("Insert action"),
                window,
                cx,
                Rc::new(move |type_id, cx| {
                    panel
                        .update(cx, |panel, cx| panel.insert_on_edge(type_id, &edge_id, cx))
                        .ok();
                }),
            )
        });
        let _dismissed = cx.subscribe(&menu, |this, _, _: &DismissEvent, cx| {
            this.insert_menu = None;
            cx.notify();
        });
        // Focused so it takes the keyboard and closes on Escape.
        menu.focus_handle(cx).focus(window, cx);
        self.insert_menu = Some(InsertMenu {
            menu,
            position,
            _dismissed,
        });
        cx.notify();
    }

    /// Puts a new `type_id` action on the connection `edge_id`: the
    /// connection is replaced by one into the new action and one out of it,
    /// and the action it used to lead to moves down, with everything after
    /// it, to make room.
    ///
    /// The connection into the new action keeps the old one's label, since
    /// "run after this failed" is a statement about the action upstream. The
    /// one out of it is plain: the action below now waits on the new one.
    fn insert_on_edge(
        &mut self,
        type_id: &'static str,
        edge_id: &gpui_flow::EdgeId,
        cx: &mut Context<Self>,
    ) {
        let Some((edge, slot, parent_id)) = ({
            let state = self.state.read(cx);
            state
                .edges
                .iter()
                .find(|edge| edge.id == *edge_id)
                .and_then(|edge| {
                    let target = state.get_node(&edge.target)?;
                    Some((edge.clone(), target.position, target.parent_id.clone()))
                })
        }) else {
            return;
        };

        // Created where the downstream action sits, in the same body.
        let node_id = self.push_action_node(
            type_id,
            ActionAnchor {
                position: slot,
                local_position: parent_id.is_some().then_some(slot),
                parent_id,
                source_id: None,
            },
            cx,
        );
        self.state.update(cx, |state, cx| {
            for id in Self::shifted_by_insert(state, &edge.target) {
                if let Some(node) = state.get_node_mut(&id) {
                    node.position.y += INSERT_SHIFT;
                }
            }
            state.edges.retain(|existing| existing.id != edge.id);
            let mut into = FlowEdge::new(
                format!("e_{}_{node_id}", edge.source),
                edge.source.clone(),
                node_id.clone(),
            );
            into.label = edge.label.clone();
            into.color = edge.color;
            let mut out_of = FlowEdge::new(
                format!("e_{node_id}_{}", edge.target),
                node_id.clone(),
                edge.target.clone(),
            );
            // The theme's ordinary edge colour, not the "Failed" one the
            // replaced connection may have carried.
            out_of.color = state
                .edges
                .iter()
                .find(|other| other.label.is_none())
                .and_then(|other| other.color)
                .or(edge.color.filter(|_| edge.label.is_none()));
            state.edges.push(into);
            state.edges.push(out_of);
            state.refit_all_containers();
            cx.notify();
        });
    }

    /// The action an insert displaces and everything that runs after it in
    /// the same body: what has to move down to make room.
    fn shifted_by_insert(state: &FlowState, target: &NodeId) -> Vec<NodeId> {
        let parent = state.get_node(target).and_then(|n| n.parent_id.clone());
        let mut shifted = vec![target.clone()];
        let mut frontier = vec![target.clone()];
        while let Some(id) = frontier.pop() {
            for edge in state.edges.iter().filter(|edge| edge.source == id) {
                let same_body = state
                    .get_node(&edge.target)
                    .is_some_and(|node| node.parent_id == parent);
                if same_body && !shifted.contains(&edge.target) {
                    shifted.push(edge.target.clone());
                    frontier.push(edge.target.clone());
                }
            }
        }
        shifted
    }

    /// The single place a node is actually created, shared by right-click
    /// quick-add, catalog click, and catalog drag-and-drop. Selects the new
    /// node so its inspector opens, and returns its id.
    fn push_action_node(
        &mut self,
        type_id: &'static str,
        anchor: ActionAnchor,
        cx: &mut Context<Self>,
    ) -> NodeId {
        let ActionAnchor {
            position,
            parent_id,
            source_id,
            local_position,
        } = anchor;
        let id = {
            let state = self.state.read(cx);
            let base = type_id.to_ascii_lowercase();
            let mut index = 1;
            loop {
                let candidate = format!("{base}-{index}");
                if state.get_node(&candidate.clone().into()).is_none() {
                    break candidate;
                }
                index += 1;
            }
        };
        let def = REGISTRY.iter().find(|def| def.type_id == type_id);
        let label = def.map(|def| def.label).unwrap_or(type_id);
        let is_container = def
            .is_some_and(|def| matches!(def.category, workflow_engine::ActionCategory::Container));
        let mut node = FlowNode::new(id.clone(), position.x, position.y)
            .node_type(type_id)
            .label(label)
            .context_menu(true)
            .handles(vec![
                HandleDef::target(HandlePosition::Top),
                HandleDef::source(HandlePosition::Bottom),
            ]);
        if !is_container {
            node = node.size(workflow_json::LEAF_SIZE.0, workflow_json::LEAF_SIZE.1);
        }
        node.properties = workflow_json::default_properties_for_type(type_id);
        if type_id == "script" {
            // A script is a file from the start: its own, in the flow's
            // scripts folder, holding a template that already runs.
            let runtime = ScriptRuntime::Python;
            let created = script_companion::new_script_path(&self.path, &id, runtime).and_then(
                |path| {
                    script_companion::ensure_script_file(&path, runtime)?;
                    Ok(path)
                },
            );
            match created {
                Ok(path) => {
                    let source = ScriptSource::File {
                        path: script_companion::root_relative(&path, &self.root),
                        runtime: None,
                    };
                    if let Ok(source) = serde_json::to_string(&source) {
                        node.properties.push(("source".into(), source.into()));
                    }
                }
                // The node is still added; validation reports the missing
                // source, and the script section can create the file later.
                Err(error) => {
                    self.error = Some(format!("Couldn't create the script file: {error}"));
                }
            }
        }
        let node_id: NodeId = id.clone().into();
        self.state.update(cx, |state, cx| {
            state.push_undo();
            state.nodes.push(node);
            if let Some(source_id) = source_id {
                state.edges.push(FlowEdge::new(
                    format!("e_{source_id}_{id}"),
                    source_id,
                    id.clone(),
                ));
            }
            // An `If`/`Try` is nothing but its two branches on the canvas, so
            // it is created with both: they are what actions get dropped into.
            for branch_type in workflow_json::branch_types(type_id).into_iter().flatten() {
                state
                    .nodes
                    .push(workflow_json::branch_node(&id, branch_type, false));
            }
            // `set_parent` resolves nodes through the lookup, which doesn't
            // know the nodes pushed above until it is rebuilt.
            state.rebuild_lookup();
            // Nest *after* insertion, via `set_parent`, rather than building
            // the node with a parent: `position` was computed in absolute
            // flow space (a cursor position), and `set_parent` is what
            // re-expresses it relative to the new parent. Building the node
            // pre-parented would misplace it by the container's own origin —
            // the same jump `set_parent`'s doc comment exists to prevent.
            if let Some(parent_id) = parent_id {
                state.set_parent(&node_id, Some(parent_id));
                if let Some(local) = local_position
                    && let Some(node) = state.get_node_mut(&node_id)
                {
                    node.position = local;
                }
            }
            state.refit_all_containers();
            cx.notify();
        });
        if type_id == "script" {
            // Its template reads the run through the helper beside it.
            self.write_script_support(cx);
        }
        self.select_node(node_id.clone(), cx);
        node_id
    }

    fn render_toolbar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let run_state_has_results =
            !self.run_snapshot.log.is_empty() || !self.run_snapshot.nodes.is_empty();
        h_flex()
            .w_full()
            .flex_shrink_0()
            .items_center()
            .justify_between()
            .gap_2()
            .px_3()
            .py_2()
            .border_b_1()
            .border_color(cx.theme().border)
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(Icon::new(IconName::Network).text_color(cx.theme().foreground))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(cx.theme().foreground)
                            .child(self.flow_name.clone()),
                    ),
            )
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .when(run_state_has_results, |el| {
                        el.child(
                            Button::new("designer-logs")
                                .ghost()
                                .xsmall()
                                .icon(if self.show_log {
                                    IconName::ChevronDown
                                } else {
                                    IconName::SquareTerminal
                                })
                                .tooltip(if self.show_log {
                                    "Close the run log"
                                } else {
                                    "Show the run log"
                                })
                                .on_click(
                                    cx.listener(|this, _, _, cx| this.on_toggle_log_click(cx)),
                                ),
                        )
                        .child(
                            Button::new("designer-open-results")
                                .ghost()
                                .xsmall()
                                .icon(IconName::ChartPie)
                                .tooltip("Open the run results in a new tab")
                                .on_click(cx.listener(|this, _, window, cx| {
                                    window.dispatch_action(
                                        Box::new(ViewRunResults(this.path.clone())),
                                        cx,
                                    );
                                })),
                        )
                        .child(
                            Button::new("designer-clear-results")
                                .ghost()
                                .xsmall()
                                .icon(IconName::Close)
                                .tooltip("Clear run results and status colors")
                                .on_click(
                                    cx.listener(|this, _, _, cx| this.on_clear_results_click(cx)),
                                ),
                        )
                    })
                    .when_some(self.running_action(), |el, run| {
                        el.child(
                            h_flex()
                                .gap_1p5()
                                .items_center()
                                .child(Spinner::new().xsmall())
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(gpui::FontWeight::MEDIUM)
                                        .text_color(cx.theme().primary)
                                        .child(format!("{} \u{00b7} {}", run.name, run.type_id)),
                                ),
                        )
                    })
                    .when_some(self.status.clone(), |el, status| {
                        el.child(
                            div()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child(status),
                        )
                    })
                    .child(
                        Button::new("designer-fit-view")
                            .ghost()
                            .xsmall()
                            .label("Fit View")
                            .on_click(cx.listener(|this, _, _, cx| this.on_fit_view_click(cx))),
                    )
                    .child(
                        Button::new("designer-save")
                            .ghost()
                            .xsmall()
                            .label("Save")
                            .on_click(
                                cx.listener(|this, _, window, cx| this.on_save_click(window, cx)),
                            ),
                    )
                    .child(if self.running {
                        Spinner::new().xsmall().into_any_element()
                    } else {
                        let error_count = self
                            .validation_issues
                            .iter()
                            .filter(|i| i.severity == ValidationSeverity::Error)
                            .count();
                        let blocked = error_count > 0;
                        let run_btn = Button::new("designer-run")
                            .primary()
                            .xsmall()
                            .icon(IconName::Play)
                            .label("Run")
                            .disabled(blocked)
                            .tooltip(if blocked {
                                "Fix validation errors before running"
                            } else {
                                "Run this workflow"
                            })
                            .on_click(
                                cx.listener(|this, _, window, cx| this.on_run_click(window, cx)),
                            );
                        if blocked {
                            h_flex()
                                .gap_1()
                                .items_center()
                                .child(run_btn)
                                .child(self.render_issues_badge(error_count, cx))
                                .into_any_element()
                        } else {
                            run_btn.into_any_element()
                        }
                    })
                    .child(
                        Button::new("designer-view-raw")
                            .ghost()
                            .xsmall()
                            .label("Raw")
                            .tooltip("Open the flow's underlying JSON as text")
                            .on_click(cx.listener(|this, _, window, cx| {
                                window.dispatch_action(Box::new(ViewRaw(this.path.clone())), cx);
                            })),
                    ),
            )
    }

    /// The clickable "N errors" badge, and the popover it opens.
    ///
    /// This exists because the count alone was unactionable: Run is disabled,
    /// the badge says "2 errors", and the only place the messages were rendered
    /// was the inspector for whichever node happened to be selected — so
    /// flow-level issues (`node_path == ""`: a `runAfter` cycle or a dangling
    /// reference) matched no node and were shown *nowhere at all*.
    ///
    /// Every issue gets a row here, so the overlay is the one place the full
    /// list is guaranteed visible. A row that names a node selects it and pans
    /// it into view; a flow-level row says so instead of pretending to point
    /// somewhere.
    fn render_issues_badge(&self, error_count: usize, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity();
        Popover::new("designer-issues-popover")
            .open(self.show_issues)
            .on_open_change(move |open, _, cx| {
                // Covers the dismissals the trigger click doesn't drive:
                // clicking outside, or Escape.
                entity.update(cx, |this, cx| {
                    this.show_issues = *open;
                    cx.notify();
                });
            })
            .trigger(
                Button::new("designer-issues-badge")
                    .danger()
                    .outline()
                    .xsmall()
                    .label(format!(
                        "{error_count} error{} \u{25be}",
                        if error_count == 1 { "" } else { "s" }
                    ))
                    .tooltip("Show what's blocking Run")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.show_issues = !this.show_issues;
                        cx.notify();
                    })),
            )
            .child(self.render_issues_overlay(cx))
    }

    /// The popover body: one row per issue, errors first, each naming the node
    /// it belongs to and clickable to reveal it.
    fn render_issues_overlay(&self, cx: &mut Context<Self>) -> impl IntoElement {
        // Errors before warnings, then by node path so the order is stable
        // across re-renders (issue order follows the action tree, which the
        // user can reshuffle by editing, and a list that reorders under the
        // cursor while they're clicking through it is unusable).
        let mut issues: Vec<&ValidationIssue> = self.validation_issues.iter().collect();
        issues.sort_by_key(|issue| {
            (
                match issue.severity {
                    ValidationSeverity::Error => 0,
                    ValidationSeverity::Warning => 1,
                },
                issue.node_path.clone(),
                issue.field.clone(),
            )
        });

        if issues.is_empty() {
            return div()
                .p_3()
                .text_xs()
                .text_color(cx.theme().muted_foreground)
                .child("No validation issues.")
                .into_any_element();
        }

        v_flex()
            .id("designer-issues-list")
            .w(px(420.))
            .max_h(px(360.))
            .overflow_y_scroll()
            .gap_0p5()
            .p_1()
            .children(issues.into_iter().map(|issue| {
                let is_error = issue.severity == ValidationSeverity::Error;
                let color = if is_error {
                    cx.theme().danger
                } else {
                    cx.theme().warning
                };
                let (icon, subject) = if issue.node_path.is_empty() {
                    ("\u{26a0}", "Workflow".to_string())
                } else {
                    ("\u{25cf}", issue.node_path.clone())
                };
                // A flow-level issue has no node to jump to, so its row is
                // rendered flat rather than clickable — an affordance that
                // silently does nothing is worse than no affordance.
                let target = Self::issue_node_id(&issue.node_path);
                let row_target = target.clone();
                // The row id has to be a 2-tuple at most (`ElementId` has no
                // 3-tuple impl), so the field is folded into the string key
                // rather than kept as a separate tuple element.
                let row_id = format!(
                    "designer-issue:{}:{}",
                    issue.node_path,
                    issue.field.as_deref().unwrap_or("")
                );
                // `Div::hover` takes only a `StyleRefinement` (no `cx`), so the
                // theme has to be sampled out here rather than read inside.
                let hover_bg = cx.theme().accent;
                let hover_border = cx.theme().border;
                h_flex()
                    .id(row_id)
                    .gap_2()
                    .px_2()
                    .py_1p5()
                    .rounded_sm()
                    .when(target.is_some(), |el| {
                        el.cursor_pointer().hover(move |style| {
                            style.bg(hover_bg).border_1().border_color(hover_border)
                        })
                    })
                    .when(target.is_some(), |el| {
                        el.on_click(cx.listener(move |this, _, _, cx| {
                            if let Some(node_id) = &row_target {
                                this.reveal_node(node_id.clone(), cx);
                            }
                            this.show_issues = false;
                            cx.notify();
                        }))
                    })
                    .child(
                        div()
                            .text_xs()
                            .text_color(color)
                            .child(format!("{icon} {subject}")),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .text_xs()
                            .text_color(cx.theme().foreground)
                            // `field` is already folded into `message` by the
                            // validator, so it isn't repeated here.
                            .child(issue.message.clone()),
                    )
                    .into_any_element()
            }))
            .into_any_element()
    }

    fn render_properties(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(node_id) = &self.selected_node_id else {
            let selected_edge = self
                .state
                .read(cx)
                .edges
                .iter()
                .find(|edge| edge.selected)
                .cloned();
            if let Some(edge) = selected_edge {
                return self.render_edge_properties(&edge, cx).into_any_element();
            }
            return div()
                .p_4()
                .text_xs()
                .text_color(cx.theme().muted_foreground)
                .child("Select a node or a connection to edit it.")
                .into_any_element();
        };
        let type_id = self
            .state
            .read(cx)
            .get_node(node_id)
            .and_then(|n| n.node_type.clone())
            .unwrap_or_default();

        // Collect issues belonging to this node (any nesting depth: the leaf
        // id segment is always the node_id, so we match on suffix).
        let node_id_str = node_id.as_ref();
        let node_issues: Vec<&ValidationIssue> = self
            .validation_issues
            .iter()
            .filter(|i| {
                i.node_path == node_id_str || i.node_path.ends_with(&format!("/{node_id_str}"))
            })
            .collect();

        let node_level_issues: Vec<&ValidationIssue> = node_issues
            .iter()
            .copied()
            .filter(|i| i.field.is_none())
            .collect();

        let loop_pages = self.loop_pages(node_id, cx);
        let is_loop = type_id == "Foreach";

        // Registry-declared optional inputs with no property behind them.
        let unset_optional: Vec<InputField> = {
            let state = self.state.read(cx);
            let properties = state
                .get_node(node_id)
                .map(|n| n.properties.as_slice())
                .unwrap_or_default();
            workflow_engine::registry::find(type_id.as_ref())
                .map(|def| def.inputs)
                .unwrap_or_default()
                .iter()
                .filter(|field| {
                    !field.required
                        && meta_property(type_id.as_ref(), properties, field.name).is_none()
                })
                .copied()
                .collect()
        };

        // Synthetic branch node: no editable properties, no Delete. Resolve the
        // owning action's label so the explanation names something the user
        // can actually find on the canvas.
        let is_branch_wrapper = is_branch_wrapper_type(type_id.as_ref());
        let (label, owning_label, child_count) = if is_branch_wrapper {
            let node_id = self
                .selected_node_id
                .as_ref()
                .expect("rendered node has an id");
            let state = self.state.read(cx);
            // A wrapper's id is always `<owning action id><wrapper type id>`.
            let owner_id = node_id
                .as_ref()
                .strip_suffix(type_id.as_ref())
                .map(SharedString::from);
            let owning = owner_id
                .as_ref()
                .and_then(|id| state.get_node(id))
                .map(|n| n.label.to_string())
                .unwrap_or_else(|| "container".to_string());
            let child_count = state.children_of(node_id).len();
            (branch_wrapper_label(type_id.as_ref()), owning, child_count)
        } else {
            ("", String::new(), 0)
        };

        // Full width but natural height: the pane around it scrolls, and a
        // height pinned to the pane would clip the rows instead.
        v_flex()
            .w_full()
            .gap_2()
            .p_3()
            .child(
                div()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child(format!("{type_id} \u{00b7} {node_id}")),
            )
            // Node-level validation issues (unregistered type, unrunnable, cycle, etc.)
            .when(!node_level_issues.is_empty(), |el| {
                el.child(
                    v_flex()
                        .gap_1()
                        .children(node_level_issues.iter().map(|issue| {
                            let (color, prefix) = if issue.severity == ValidationSeverity::Error {
                                (cx.theme().danger, "\u{2717} ") // ✗
                            } else {
                                (cx.theme().warning, "\u{26a0} ") // ⚠
                            };
                            div()
                                .text_xs()
                                .text_color(color)
                                .child(format!("{prefix}{}", issue.message))
                        })),
                )
            })
            // Inside a loop the body ran once per item, so say which run of
            // it is being shown and let the user step through them.
            .children(
                loop_pages
                    .iter()
                    .map(|page| self.render_loop_page(page, cx).into_any_element())
                    .collect::<Vec<_>>(),
            )
            .when_some(self.run_result_for(node_id, cx), |el, run| {
                el.child(self.render_run_result(&run, cx).into_any_element())
            })
            .when(
                !loop_pages.is_empty() && !is_loop && self.run_result_for(node_id, cx).is_none(),
                |el| {
                    el.child(
                        div()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child("This action didn't run in this iteration."),
                    )
                },
            )
            .when_some(self.label_input.clone(), |el, input| {
                el.child(labeled_field(
                    "Label",
                    cx.theme().muted_foreground,
                    Input::new(&input),
                ))
            })
            .when_some(self.get_selected_script_source(cx), |el, src| {
                el.child(self.render_script_properties(&src, cx))
            })
            .when(type_id == "http", |el| {
                el.child(
                    v_flex()
                        .gap_1()
                        .child(
                            div()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child("Request body"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child(
                                    "Edit the body as JSON in its own tab. The registry-derived \
                                     schema and any .body.schema.json sidecar are applied there.",
                                ),
                        )
                        .child(
                            Button::new("designer-edit-body")
                                .ghost()
                                .xsmall()
                                .label("Edit Body")
                                .tooltip(
                                    "Open this request body as a JSON file, with schema validation.",
                                )
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.on_edit_body_click(window, cx)
                                })),
                        ),
                )
            })
            .child(div().h(gpui::px(1.0)).w_full().bg(cx.theme().border))
            // A synthetic branch node has no `Action` to edit: its properties
            // would be discarded on the next save, and Delete would wipe the
            // real actions it groups. Show what it is and why instead.
            .when(is_branch_wrapper, |el| {
                el.child(
                    v_flex()
                        .gap_1()
                        .child(
                            div()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child(format!(
                                    "This box is {label}, a canvas grouping for the actions inside it. \
                                     It isn't a separate step in the flow, so it has no properties of \
                                     its own and can't be deleted on its own. Edit or delete the \
                                     actions inside it, or the {} that owns it.",
                                    owning_label,
                                )),
                        )
                        .child(
                            div().text_xs().text_color(cx.theme().accent).child(format!(
                                "{} action(s) inside",
                                child_count
                            )),
                        ),
                )
            })
            .when(!is_branch_wrapper, |el| {
                el.children(self.property_rows.iter().map(|row| {
                    // Collect field-level issues for this property key.
                    let field_issues: Vec<&ValidationIssue> = node_issues
                        .iter()
                        .copied()
                        .filter(|i| i.field.as_deref() == Some(row.key.as_ref()))
                        .collect();
                    let muted = cx.theme().muted_foreground;
                    // A required field left empty is a field-level error in its
                    // own right, so the user gets it next to the input rather
                    // than only in the toolbar popover.
                    // Read the editor the user actually types into: a
                    // `Json`/`Expression` row edits its textarea, which the
                    // single-line input doesn't follow.
                    let missing_required = row.required
                        && match &row.textarea {
                            Some(area) => area.read(cx).value().trim().is_empty(),
                            None => row.value.read(cx).value().trim().is_empty(),
                        };
                    let field_el = v_flex()
                        .gap_0p5()
                        .child(self.render_property_label(row, muted, cx))
                        .child(self.render_property_editor(row, cx))
                        .when(!row.help.is_empty(), |el| {
                            el.child(
                                div()
                                    .text_xs()
                                    .text_color(muted)
                                    .child(row.help.to_string()),
                            )
                        })
                        // JSONLogic assistance: offer the outputs that are
                        // actually in scope, so a `var` can be written without
                        // guessing an action id.
                        .when(row.kind == Some(FieldKind::Expression), |el| {
                            el.child(self.render_expression_picker(row, cx))
                        })
                        .when(missing_required, |el| {
                            el.child(
                                div()
                                    .text_xs()
                                    .text_color(cx.theme().danger)
                                    .child("\u{2717} This field is required"),
                            )
                        })
                        .children(field_issues.iter().map(|issue| {
                            let (color, prefix) = if issue.severity == ValidationSeverity::Error {
                                (cx.theme().danger, "\u{2717} ")
                            } else {
                                (cx.theme().warning, "\u{26a0} ")
                            };
                            div()
                                .text_xs()
                                .text_color(color)
                                .child(format!("{prefix}{}", issue.message))
                        }));
                    field_el.into_any_element()
                }))
            })
            // Optional inputs the action doesn't use yet. They get no row until
            // asked for, so this is where they are discoverable.
            .when(!unset_optional.is_empty(), |el| {
                el.child(
                    v_flex()
                        .gap_1()
                        .child(
                            div()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child("Add an optional input:"),
                        )
                        .child(div().flex().flex_wrap().gap_1().children(
                            unset_optional.iter().map(|field| {
                                let name = field.name;
                                Button::new(SharedString::from(format!(
                                    "designer-add-optional-{name}"
                                )))
                                .ghost()
                                .xsmall()
                                .icon(IconName::Plus)
                                .label(name)
                                .tooltip(field.help)
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.on_add_optional_field_click(name, cx)
                                }))
                            }),
                        )),
                )
            })
            .when(!is_branch_wrapper, |el| {
                el.child(
                    h_flex()
                        .gap_1()
                        .child(Input::new(&self.new_prop_key).xsmall().w_24())
                        .child(Input::new(&self.new_prop_value).xsmall().flex_1())
                        .child(
                            Button::new("designer-add-property")
                                .ghost()
                                .xsmall()
                                .icon(IconName::Plus)
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.on_add_property_click(window, cx)
                                })),
                        ),
                )
            })
            .child(
                Button::new("designer-delete-node")
                    .ghost()
                    .xsmall()
                    .icon(IconName::Delete)
                    .label("Delete Node")
                    .when(is_branch_wrapper, |b| {
                        b.disabled(true).tooltip(
                            "This box only groups the actions inside it — delete one of those, \
                             or the container that owns it.",
                        )
                    })
                    .on_click(cx.listener(|this, _, _, cx| this.on_delete_node_click(cx))),
            )
            .into_any_element()
    }

    /// Sets when the action a connection leads to runs: after the one above
    /// it succeeded (`None`), failed (`"Failed"`), or either (`"Always"`).
    ///
    /// The condition is kept as the edge's label, which is what
    /// `workflow_json` reads back into `runAfter` on save, and the edge is
    /// recoloured to match the way a loaded one is.
    fn on_edge_condition_click(
        &mut self,
        edge_id: gpui_flow::EdgeId,
        condition: Option<&'static str>,
        cx: &mut Context<Self>,
    ) {
        let normal = hex(cx.theme().muted_foreground);
        let failed = hex(cx.theme().danger);
        self.state.update(cx, |state, cx| {
            state.push_undo();
            if let Some(edge) = state.edges.iter_mut().find(|edge| edge.id == edge_id) {
                edge.label = condition.map(Into::into);
                edge.color = Some(if condition == Some("Failed") {
                    failed
                } else {
                    normal
                });
            }
            cx.notify();
        });
    }

    fn on_delete_edge_click(&mut self, edge_id: gpui_flow::EdgeId, cx: &mut Context<Self>) {
        self.state.update(cx, |state, cx| {
            state.push_undo();
            state.edges.retain(|edge| edge.id != edge_id);
            cx.notify();
        });
    }

    /// The inspector for a selected connection: what it joins, and when the
    /// action it leads to runs.
    fn render_edge_properties(&self, edge: &FlowEdge, cx: &mut Context<Self>) -> impl IntoElement {
        let muted = cx.theme().muted_foreground;
        let (source, target) = {
            let state = self.state.read(cx);
            let label = |id: &NodeId| {
                state
                    .get_node(id)
                    .map(node_label)
                    .unwrap_or_else(|| id.to_string())
            };
            (label(&edge.source), label(&edge.target))
        };
        let current = edge.label.as_deref();
        let conditions: [(Option<&'static str>, &'static str, &'static str); 3] = [
            (None, "Succeeded", "Run only if the action above succeeded."),
            (
                Some("Failed"),
                "Failed",
                "Run only if the action above failed: an error path.",
            ),
            (
                Some("Always"),
                "Either",
                "Run whether the action above succeeded or failed.",
            ),
        ];

        v_flex()
            .w_full()
            .gap_2()
            .p_3()
            .child(div().text_xs().text_color(muted).child("Connection"))
            .child(
                div()
                    .text_sm()
                    .text_color(cx.theme().foreground)
                    .child(format!("{source} \u{2192} {target}")),
            )
            .child(div().h(gpui::px(1.0)).w_full().bg(cx.theme().border))
            .child(
                div()
                    .text_xs()
                    .text_color(muted)
                    .child(format!("Run {target} when {source} has:")),
            )
            .child(h_flex().gap_1().flex_wrap().children(
                conditions.into_iter().map(|(condition, label, tooltip)| {
                    let is_active = current == condition;
                    let edge_id = edge.id.clone();
                    Button::new(format!("designer-edge-{}", label.to_ascii_lowercase()))
                        .xsmall()
                        .when(!is_active, |button| button.ghost())
                        .when(is_active, |button| button.outline())
                        .label(label)
                        .tooltip(tooltip)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.on_edge_condition_click(edge_id.clone(), condition, cx);
                        }))
                        .into_any_element()
                }),
            ))
            .child({
                let edge_id = edge.id.clone();
                Button::new("designer-delete-edge")
                    .ghost()
                    .xsmall()
                    .icon(IconName::Delete)
                    .label("Delete Connection")
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.on_delete_edge_click(edge_id.clone(), cx);
                    }))
            })
    }

    /// A property row's label: the field name, a `*` when the registry marks
    /// it required, and a muted type badge so the expected shape is legible
    /// before the user starts typing.
    ///
    /// A row that isn't required also gets a remove button, which takes the
    /// input out of the action altogether rather than leaving it blank.
    fn render_property_label(
        &self,
        row: &PropertyRow,
        muted: gpui::Hsla,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let danger = cx.theme().danger;
        let key = row.key.clone();
        h_flex()
            .gap_1()
            .items_center()
            .child(
                div()
                    .text_xs()
                    .text_color(muted)
                    .child(row.key.to_string())
                    .when(row.required, |el| {
                        el.child(div().text_xs().text_color(danger).child("*"))
                    }),
            )
            .when_some(row.type_badge, |el, badge| {
                el.child(div().text_xs().text_color(muted).opacity(0.7).child(badge))
            })
            .when(!row.required, |el| {
                el.child(div().flex_1()).child(
                    Button::new(SharedString::from(format!(
                        "designer-remove-prop-{}",
                        row.key
                    )))
                    .ghost()
                    .xsmall()
                    .icon(IconName::Close)
                    .tooltip("Remove this input from the action")
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.on_remove_property_click(key.clone(), cx)
                    })),
                )
            })
    }

    /// The control for a property row, chosen by its declared `FieldKind`:
    /// a `Switch` for `Bool`, a `NumberInput` for `Number`, a multi-line
    /// `Textarea` for `Json`/`Expression`, and a single-line `Input`
    /// otherwise (including undeclared extras).
    ///
    /// The `Switch` writes through [`PropertyRow::commit`] rather than
    /// through an input event, because `set_value` deliberately doesn't emit
    /// one — see `commit`'s doc comment.
    fn render_property_editor(
        &self,
        row: &PropertyRow,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        if let Some(select) = &row.select {
            return Select::new(select).small().into_any_element();
        }
        match row.kind {
            Some(FieldKind::Bool) => {
                let checked = row.value.read(cx).value().trim() == "true";
                let commit = row.commit.clone();
                let input = row.value.clone();
                Switch::new(SharedString::from(format!("designer-prop-{}", row.key)))
                    .checked(checked)
                    .label(if checked { "Enabled" } else { "Disabled" })
                    .on_click(cx.listener(move |this, next: &bool, window, cx| {
                        let text = next.to_string();
                        // Keep the backing input in step so a later read of the
                        // row agrees with the toggle and with what we persist.
                        input.update(cx, |input, cx| {
                            input.set_value(text.clone(), window, cx);
                        });
                        commit(this, text, cx);
                    }))
                    .into_any_element()
            }
            Some(FieldKind::Number) => NumberInput::new(&row.value).xsmall().into_any_element(),
            Some(FieldKind::Json) | Some(FieldKind::Expression) => {
                match &row.textarea {
                    Some(area) => Textarea::new(area).h(gpui::px(72.0)).into_any_element(),
                    // Unreachable in practice: the builder always pairs a
                    // `Json`/`Expression` row with a textarea. Fall back rather
                    // than panic.
                    None => Input::new(&row.value).into_any_element(),
                }
            }
            _ => Input::new(&row.value).into_any_element(),
        }
    }

    /// Collects the JSONLogic `var` paths an expression on `node_id` can
    /// reference: `<action_id>.outputs.<output>` for every action that has
    /// run by the time the expression is evaluated, in the shape
    /// `RunContext::record_outputs` stores them.
    ///
    /// A `var` against an action that hasn't run resolves to null and
    /// silently changes behavior, so only these are offered:
    ///
    /// - the node's predecessors, followed back through its incoming edges
    ///   (the canvas draws `runAfter` as an arrow into the dependent node);
    /// - the same for every container the node sits inside, since a body
    ///   reads its enclosing scopes' results;
    /// - `$item` inside a `Foreach`, and the owning `Try`'s `error` inside
    ///   its catch branch;
    /// - for an `Until`, its own body, which runs before `until` is checked.
    ///
    /// An action reached only over a "Failed" edge is followed but not
    /// offered: it failed, so it recorded no outputs.
    fn expression_references(state: &FlowState, node_id: &NodeId) -> Vec<String> {
        let scope = Self::reference_scope(state, node_id);
        let mut refs = Vec::new();
        for id in scope.actions.iter().chain(&scope.catch_owners) {
            let outputs = state
                .get_node(id)
                .and_then(|n| n.node_type.as_deref())
                .and_then(workflow_engine::registry::find)
                .map(|def| def.outputs)
                .unwrap_or_default();
            for output in outputs {
                refs.push(format!("{id}.outputs.{}", output.name));
            }
        }
        // A variable is read by its own name, whichever action set it last.
        for id in &scope.actions {
            let Some(node) = state.get_node(id) else {
                continue;
            };
            if node.node_type.as_deref() != Some("InitVariable") {
                continue;
            }
            let name = node
                .properties
                .iter()
                .find(|(key, _)| key.as_ref() == "name")
                .map(|(_, name)| name.trim())
                .filter(|name| !name.is_empty() && !name.contains('.'));
            if let Some(name) = name {
                refs.push(format!("variables.{name}"));
            }
        }
        if !scope.loops.is_empty() {
            refs.push("$item".to_string());
        }

        // Stable order, no duplicates.
        refs.sort();
        refs.dedup();
        refs
    }

    /// What an expression on `node_id` can see, by the rules
    /// [`Self::expression_references`] documents.
    fn reference_scope(state: &FlowState, node_id: &NodeId) -> ReferenceScope {
        let mut scope = ReferenceScope::default();
        let Some(node) = state.get_node(node_id) else {
            return scope;
        };
        let node_type = node.node_type.as_deref().unwrap_or_default();
        if node_type.is_empty() || is_branch_wrapper_type(node_type) {
            return scope;
        }

        if node_type == "Until" {
            for child in state.children_of(node_id) {
                scope.actions.push(child.id.clone());
            }
        }

        // Walk outward one scope at a time: the node, then the container it
        // sits in, then that container's own container.
        let mut current = node;
        loop {
            let mut seen: Vec<NodeId> = vec![current.id.clone()];
            let mut frontier = vec![current.id.clone()];
            while let Some(id) = frontier.pop() {
                for edge in state.edges.iter().filter(|edge| edge.target == id) {
                    if seen.contains(&edge.source) {
                        continue;
                    }
                    seen.push(edge.source.clone());
                    frontier.push(edge.source.clone());
                    if edge.label.as_deref() != Some("Failed") {
                        scope.actions.push(edge.source.clone());
                    }
                }
            }

            let Some(parent) = current.parent_id.as_ref().and_then(|id| state.get_node(id)) else {
                break;
            };
            // A branch wrapper is canvas scaffolding; the action that owns
            // the scope is the wrapper's own parent.
            let owner = match parent.node_type.as_deref() {
                Some(wrapper) if is_branch_wrapper_type(wrapper) => {
                    let owner = parent.parent_id.as_ref().and_then(|id| state.get_node(id));
                    if wrapper == workflow_json::BRANCH_CATCH
                        && let Some(owner) = owner
                    {
                        scope.catch_owners.push(owner.id.clone());
                    }
                    owner
                }
                _ => Some(parent),
            };
            let Some(owner) = owner else {
                break;
            };
            if owner.node_type.as_deref() == Some("Foreach") {
                scope.loops.push(owner.id.clone());
            }
            current = owner;
        }
        scope
    }

    /// References read off what the visible actions actually returned on the
    /// last run, which is how the picker can offer what is *inside* an output
    /// (`fetch.outputs.body.customer.id`) when the registry only knows the
    /// output's name.
    ///
    /// `samples` maps an action id to the outputs it last produced. `known`
    /// is what [`Self::expression_references`] already offers, so nothing is
    /// listed twice.
    ///
    /// Inside a `Foreach` that loops over a plain reference to a sampled
    /// list, the list's first element stands in for `$item`.
    fn sampled_references(
        state: &FlowState,
        samples: &HashMap<String, serde_json::Value>,
        known: &[String],
        node_id: &NodeId,
    ) -> Vec<SampledRef> {
        let scope = Self::reference_scope(state, node_id);
        let mut refs = Vec::new();
        for id in &scope.actions {
            if let Some(outputs) = samples.get::<str>(id.as_ref()) {
                sample_paths(&format!("{id}.outputs"), outputs, 1, &mut refs);
            }
        }
        if let Some(item) = scope
            .loops
            .first()
            .and_then(|foreach| Self::sampled_item(state, samples, foreach))
        {
            sample_paths("$item", &item, 1, &mut refs);
        }
        refs.retain(|sampled| !known.contains(&sampled.path));
        refs.sort_by(|a, b| a.path.cmp(&b.path));
        refs.dedup_by(|a, b| a.path == b.path);
        refs.truncate(MAX_SAMPLED_REFS);
        refs
    }

    /// One element of the list `foreach` loops over, when its expression is
    /// a plain `{"var": "<id>.outputs.<path>"}` whose target was sampled. A
    /// computed expression has no list to look up, so it yields nothing.
    fn sampled_item(
        state: &FlowState,
        samples: &HashMap<String, serde_json::Value>,
        foreach: &NodeId,
    ) -> Option<serde_json::Value> {
        let expression = state
            .get_node(foreach)?
            .properties
            .iter()
            .find(|(key, _)| key.as_ref() == "__foreach")
            .map(|(_, value)| value.to_string())?;
        let expression: serde_json::Value = serde_json::from_str(&expression).ok()?;
        let path = expression.as_object()?.get("var")?.as_str()?;
        let mut segments = path.split('.');
        let outputs = samples.get(segments.next()?)?;
        if segments.next()? != "outputs" {
            return None;
        }
        let list = segments.try_fold(outputs, |value, key| value.get(key))?;
        list.as_array()?.first().cloned()
    }

    /// The last run's outputs per action, minus any whose inputs have been
    /// edited since (see [`Self::mark_samples_stale`]).
    fn run_samples(&self) -> HashMap<String, serde_json::Value> {
        self.run_snapshot
            .nodes
            .iter()
            .filter(|(id, run)| {
                run.phase == RunPhase::Succeeded && !self.stale_samples.contains(id.as_str())
            })
            .filter_map(|(id, run)| {
                let outputs: serde_json::Value = serde_json::from_str(run.detail.as_deref()?).ok()?;
                outputs.is_object().then(|| (id.clone(), outputs))
            })
            .collect()
    }

    /// Writes what this flow's scripts read the run through: the helper per
    /// language, and type declarations for the JavaScript ones. A flow with
    /// no script action gets neither.
    ///
    /// A failure here is logged, not shown: the helpers are a convenience,
    /// and a script that reads the context file itself runs without them.
    fn write_script_support(&self, cx: &App) {
        let state = self.state.read(cx);
        let has_script = state
            .nodes
            .iter()
            .any(|node| node.node_type.as_deref() == Some("script"));
        if !has_script {
            return;
        }
        let result = script_companion::scripts_dir(&self.path).and_then(|dir| {
            script_helpers::ensure_helpers(&dir)?;
            let samples = self.run_samples();
            let (variables, actions) = Self::script_types(state, &samples);
            script_helpers::write_declarations(&dir, &variables, &actions)
        });
        if let Err(error) = result {
            log::warn!("Designer: couldn't write the script helpers: {error}");
        }
    }

    /// The TypeScript types for the flow's variables and its actions'
    /// outputs, as `(name, type)`.
    ///
    /// A variable starts from the type its `InitVariable` declares. Where
    /// the last run recorded a value for it, the value's own shape is used
    /// instead, with a `SetVariable`'s taking precedence, since that is what
    /// the variable went on to hold (a list that started empty has elements
    /// by then). An action is typed from its sampled outputs.
    fn script_types(
        state: &FlowState,
        samples: &HashMap<String, serde_json::Value>,
    ) -> (Vec<(String, String)>, Vec<(String, String)>) {
        let property = |node: &FlowNode, key: &str| {
            node.properties
                .iter()
                .find(|(name, _)| name.as_ref() == key)
                .map(|(_, value)| value.trim().to_string())
                .filter(|value| !value.is_empty())
        };
        let mut variables: HashMap<String, String> = HashMap::new();
        for kind in ["InitVariable", "SetVariable"] {
            for node in state
                .nodes
                .iter()
                .filter(|node| node.node_type.as_deref() == Some(kind))
            {
                let Some(name) = property(node, "name") else {
                    continue;
                };
                if kind == "InitVariable" {
                    let declared = property(node, "type").unwrap_or_default();
                    variables.insert(
                        name.clone(),
                        script_helpers::declared_ts_type(&declared).to_string(),
                    );
                }
                let sampled = samples
                    .get::<str>(node.id.as_ref())
                    .and_then(|outputs| outputs.get("value"))
                    .map(script_helpers::ts_type)
                    // A null or an empty list says less than the declaration.
                    .filter(|ty| !ty.starts_with("unknown"));
                if let Some(sampled) = sampled
                    && variables.contains_key(&name)
                {
                    variables.insert(name, sampled);
                }
            }
        }
        let actions = samples
            .iter()
            .map(|(id, outputs)| (id.clone(), script_helpers::ts_type(outputs)))
            .collect();
        (variables.into_iter().collect(), actions)
    }

    /// Every node whose result can change when `node_id` does: what runs
    /// after it, what runs after each container it sits in, and everything
    /// inside any of those.
    fn downstream_of(state: &FlowState, node_id: &NodeId) -> Vec<NodeId> {
        // The flag is whether a node's own body is affected too. It isn't
        // for a container the edited node merely sits in: the siblings that
        // ran before the edit are unchanged.
        let mut affected: Vec<NodeId> = vec![node_id.clone()];
        let mut frontier: Vec<(NodeId, bool)> = vec![(node_id.clone(), true)];
        let mut ancestor = state.get_node(node_id).and_then(|n| n.parent_id.clone());
        while let Some(id) = ancestor {
            ancestor = state.get_node(&id).and_then(|n| n.parent_id.clone());
            affected.push(id.clone());
            frontier.push((id, false));
        }
        while let Some((id, with_body)) = frontier.pop() {
            let after = state
                .edges
                .iter()
                .filter(|edge| edge.source == id)
                .map(|edge| &edge.target);
            let inside = state
                .nodes
                .iter()
                .filter(|node| with_body && node.parent_id.as_ref() == Some(&id))
                .map(|node| &node.id);
            for next in after.chain(inside) {
                if !affected.contains(next) {
                    affected.push(next.clone());
                    frontier.push((next.clone(), true));
                }
            }
        }
        affected
    }

    /// Stops offering last-run samples for `node_id` and everything
    /// downstream of it. An edit there means those results describe a flow
    /// that no longer exists; the run's status colors are left alone.
    fn mark_samples_stale(&mut self, node_id: &NodeId, cx: &App) {
        let affected = Self::downstream_of(self.state.read(cx), node_id);
        self.stale_samples
            .extend(affected.into_iter().map(|id| id.to_string()));
    }

    /// A row of clickable `{"var": "..."}` snippets for an `Expression`
    /// field, built from the outputs of the actions that run before the
    /// selected node (see [`Self::expression_references`]).
    ///
    /// Clicking one inserts the whole JSONLogic object at the cursor, so it
    /// composes with whatever the user has already typed instead of replacing
    /// it, and writes through [`PropertyRow::commit`] — `insert` is silent by
    /// design, so without that the node's stored value would never change.
    /// Both of a row's editors are updated, so this works whether the field is
    /// being edited in the textarea or (after a re-selection) the single-line
    /// input.
    fn render_expression_picker(
        &self,
        row: &PropertyRow,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let muted = cx.theme().muted_foreground;
        let Some(node_id) = self.selected_node_id.clone() else {
            return div().into_any_element();
        };
        let refs = Self::expression_references(self.state.read(cx), &node_id);
        let sampled = Self::sampled_references(
            self.state.read(cx),
            &self.run_samples(),
            &refs,
            &node_id,
        );
        if refs.is_empty() && sampled.is_empty() {
            return v_flex()
                .gap_0p5()
                .child(
                    div()
                        .text_xs()
                        .text_color(muted)
                        .child(
                            "Nothing to reference yet. Connect this action after one that \
                             produces an output.",
                        ),
                )
                .into_any_element();
        }

        v_flex()
            .gap_1()
            .when(!refs.is_empty(), |el| {
                el.child(
                    div()
                        .text_xs()
                        .text_color(muted)
                        .child("Insert a reference:"),
                )
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap_1()
                        .children(refs.iter().map(|reference| {
                            let snippet = format!("{{\"var\": \"{reference}\"}}");
                            self.reference_chip(row, reference, snippet, cx)
                        })),
                )
            })
            // What the actions before this one actually returned last time.
            // These are observations, not guarantees: the next response may
            // be shaped differently, and a path that isn't there reads null.
            .when(!sampled.is_empty(), |el| {
                el.child(
                    div()
                        .text_xs()
                        .text_color(muted)
                        .child("From the last run:"),
                )
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap_1()
                        .children(sampled.iter().map(|sampled| {
                            let tooltip = format!(
                                "{{\"var\": \"{}\"}} — {}, last seen as {}",
                                sampled.path, sampled.kind, sampled.preview
                            );
                            self.reference_chip(row, &sampled.path, tooltip, cx)
                        })),
                )
            })
            .into_any_element()
    }

    /// One clickable reference for `row`'s expression. The chip names the
    /// action and output; `tooltip` says what is inserted.
    fn reference_chip(
        &self,
        row: &PropertyRow,
        path: &str,
        tooltip: String,
        cx: &mut Context<Self>,
    ) -> Button {
        let snippet = format!("{{\"var\": \"{path}\"}}");
        let input = row.value.clone();
        let textarea = row.textarea.clone();
        let commit = row.commit.clone();
        Button::new(SharedString::from(format!(
            "designer-var-{}-{path}",
            row.key
        )))
        .ghost()
        .xsmall()
        .label(path.replace(".outputs.", "."))
        .tooltip(tooltip)
        .on_click(cx.listener(move |this, _, window, cx| {
            // Insert into whichever editor is live, then mirror into the
            // other so a later read agrees.
            if let Some(area) = &textarea {
                area.update(cx, |area, cx| {
                    area.insert(snippet.clone(), window, cx);
                });
            }
            input.update(cx, |input, cx| {
                input.insert(snippet.clone(), window, cx);
            });
            // `insert` doesn't emit, so persist explicitly from the
            // resulting text.
            let new_text = match &textarea {
                Some(area) => area.read(cx).value().to_string(),
                None => input.read(cx).value().to_string(),
            };
            commit(this, new_text, cx);
        }))
    }

    fn get_selected_script_source(&self, cx: &App) -> Option<ScriptSource> {
        let node_id = self.selected_node_id.as_ref()?;
        let state = self.state.read(cx);
        let node = state.get_node(node_id)?;
        if node.node_type.as_deref() != Some("script") {
            return None;
        }
        let (_, source_val) = node
            .properties
            .iter()
            .find(|(k, _)| k.as_ref() == "source")?;
        serde_json::from_str(source_val.as_ref()).ok()
    }

    fn set_selected_script_source(
        &mut self,
        source: ScriptSource,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(node_id) = self.selected_node_id.clone() else {
            return;
        };
        let serialized: SharedString = match serde_json::to_string(&source) {
            Ok(s) => s.into(),
            Err(e) => {
                self.error = Some(format!("Failed to serialize script source: {e}"));
                cx.notify();
                return;
            }
        };
        self.clear_run_marker(&node_id, cx);
        self.state.update(cx, |state, cx| {
            if let Some(n) = state.get_node_mut(&node_id) {
                if let Some(prop) = n
                    .properties
                    .iter_mut()
                    .find(|(k, _)| k.as_ref() == "source")
                {
                    prop.1 = serialized.clone();
                } else {
                    n.properties.push(("source".into(), serialized.clone()));
                }
            }
            cx.notify();
        });

        match &source {
            ScriptSource::File { path, .. } => {
                let path_str = path.clone();
                let file_input = cx.new(|cx| {
                    InputState::new(window, cx)
                        .default_value(path_str)
                        .placeholder("path/to/script.py")
                });
                let nid = node_id.clone();
                let state = self.state.clone();
                let _sub = cx.subscribe(&file_input, move |this, input, event, cx| {
                    if matches!(event, InputEvent::Change) {
                        let new_path = input.read(cx).value().trim().to_string();
                        this.clear_run_marker(&nid, cx);
                        state.update(cx, |state, cx| {
                            if let Some(n) = state.get_node_mut(&nid) {
                                if let Some(prop) = n
                                    .properties
                                    .iter_mut()
                                    .find(|(k, _)| k.as_ref() == "source")
                                {
                                    if let Ok(mut src) =
                                        serde_json::from_str::<ScriptSource>(prop.1.as_ref())
                                    {
                                        if let ScriptSource::File { ref mut path, .. } = src {
                                            *path = new_path;
                                            if let Ok(val) = serde_json::to_string(&src) {
                                                prop.1 = val.into();
                                            }
                                        }
                                    }
                                }
                            }
                            cx.notify();
                        });
                        this.revalidate(cx);
                    }
                });
                self.script_file_input = Some(file_input);
                self._script_file_sub = Some(_sub);
            }
            ScriptSource::Inline { .. } => {
                self.script_file_input = None;
                self._script_file_sub = None;
            }
        }
        // Which file, and which runtime, is what the script checks look at.
        self.revalidate(cx);
    }

    /// Points the selected script at a file of its own for `runtime`, in the
    /// flow's `.flow-scripts` folder, creating it from a template if it
    /// isn't there yet. An existing file is reused untouched, and whatever
    /// the script pointed at before is left on disk.
    fn on_create_script_file(
        &mut self,
        runtime: ScriptRuntime,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(node_id) = self.selected_node_id.as_ref().map(|id| id.to_string()) else {
            return;
        };
        let created = script_companion::new_script_path(&self.path, &node_id, runtime)
            .and_then(|path| {
                script_companion::ensure_script_file(&path, runtime)?;
                Ok(path)
            });
        match created {
            Ok(path) => {
                self.error = None;
                self.status = Some(format!("Script file: {}", path.display()));
                self.set_selected_script_source(
                    ScriptSource::File {
                        path: script_companion::root_relative(&path, &self.root),
                        runtime: None,
                    },
                    window,
                    cx,
                );
                self.write_script_support(cx);
            }
            Err(error) => {
                self.error = Some(format!("Couldn't create the script file: {error}"));
                cx.notify();
            }
        }
    }

    /// Sets the runtime override on the selected script's file. An inline
    /// source has no file to override the runtime of; it is replaced by
    /// creating a script file instead.
    fn on_script_runtime_change(
        &mut self,
        new_runtime: ScriptRuntime,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(ScriptSource::File { path, .. }) = self.get_selected_script_source(cx) else {
            return;
        };
        self.error = None;
        self.set_selected_script_source(
            ScriptSource::File {
                path,
                runtime: Some(new_runtime),
            },
            window,
            cx,
        );
    }

    fn on_script_runtime_override_clear(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(current_source) = self.get_selected_script_source(cx) else {
            return;
        };
        if let ScriptSource::File { path, .. } = current_source {
            self.error = None;
            self.set_selected_script_source(
                ScriptSource::File {
                    path,
                    runtime: None,
                },
                window,
                cx,
            );
        }
    }

    fn render_script_properties(
        &self,
        source: &ScriptSource,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_inline = matches!(source, ScriptSource::Inline { .. });
        let muted = cx.theme().muted_foreground;
        let languages = [
            (ScriptRuntime::Python, "Python"),
            (ScriptRuntime::Node, "Node"),
            (ScriptRuntime::Powershell, "PowerShell"),
            (ScriptRuntime::Bash, "Bash"),
        ];

        v_flex()
            .gap_2()
            // A script is a file. Inline code is only still here when a flow
            // written before that couldn't be moved to one when it was
            // opened, and the way forward is the same: give it a file.
            .when(is_inline, |el| {
                el.child(div().text_xs().text_color(cx.theme().warning).child(
                    "This script holds inline code, which is no longer edited or \
                     created here. Create a script file below to replace it.",
                ))
            })
            .child(
                v_flex()
                    .gap_1()
                    .child(
                        div()
                            .text_xs()
                            .text_color(muted)
                            .child("Create a script file"),
                    )
                    .child(h_flex().gap_1().flex_wrap().children(
                        languages.into_iter().map(|(runtime, label)| {
                            Button::new(format!("script-new-{}", label.to_ascii_lowercase()))
                                .ghost()
                                .xsmall()
                                .icon(IconName::Plus)
                                .label(label)
                                .tooltip(
                                    "Point this action at its own script file for this \
                                     language, creating it from a template if needed.",
                                )
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    this.on_create_script_file(runtime, window, cx);
                                }))
                                .into_any_element()
                        }),
                    )),
            )
            .when(!is_inline, |el| {
                let (path_str, override_rt) = match source {
                    ScriptSource::File { path, runtime } => (path.as_str(), *runtime),
                    _ => ("", None),
                };
                let inferred_text = {
                    let abs = self.root.join(path_str);
                    match inferred_runtime(&abs, override_rt) {
                        Ok(rt) => match override_rt {
                            Some(_) => format!("Override: {:?}", rt),
                            None => format!("Inferred from extension: {:?}", rt),
                        },
                        Err(err) => err,
                    }
                };

                let override_options = [
                    (None, "Auto"),
                    (Some(ScriptRuntime::Python), "Python"),
                    (Some(ScriptRuntime::Node), "Node"),
                    (Some(ScriptRuntime::Powershell), "PowerShell"),
                    (Some(ScriptRuntime::Bash), "Bash"),
                    (Some(ScriptRuntime::Dotnet), ".NET"),
                ];

                el.child(
                    v_flex()
                        .gap_2()
                        .when_some(self.script_file_input.clone(), |el, input| {
                            el.child(labeled_field("File Path", muted, Input::new(&input)))
                        })
                        .child(
                            v_flex()
                                .gap_1()
                                .child(div().text_xs().text_color(muted).child("Runtime"))
                                .child(
                                    h_flex()
                                        .gap_1()
                                        .flex_wrap()
                                        .children(override_options.into_iter().map(|(opt, label)| {
                                            let is_active = override_rt == opt;
                                            Button::new(format!("script-ovr-{}", label.to_ascii_lowercase()))
                                                .xsmall()
                                                .when(!is_active, |btn| btn.ghost())
                                                .when(is_active, |btn| btn.outline())
                                                .label(label)
                                                .on_click(cx.listener(move |this, _, window, cx| {
                                                    if let Some(rt) = opt {
                                                        this.on_script_runtime_change(rt, window, cx);
                                                    } else {
                                                        this.on_script_runtime_override_clear(window, cx);
                                                    }
                                                }))
                                                .into_any_element()
                                        })),
                                ),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(muted)
                                .child(inferred_text),
                        ),
                )
            })
            .child(
                Button::new("designer-open-script")
                    .ghost()
                    .xsmall()
                    .label("Open Script")
                    .tooltip("Open this action's script file in an editor tab.")
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.on_open_script_click(window, cx)
                    })),
            )
    }
}

impl Drop for DesignerPanel {
    fn drop(&mut self) {
        run_state::unregister_run_state(&self.path);
    }
}

fn labeled_field(
    label: impl Into<SharedString>,
    muted: gpui::Hsla,
    input: Input,
) -> impl IntoElement {
    v_flex()
        .gap_1()
        .child(div().text_xs().text_color(muted).child(label.into()))
        .child(input)
}

/// Walks a flow's (possibly nested) action tree, collecting each action
/// id's display name and type — used to fill in
/// [`ActionHistoryRecord`]'s `name`/`type` fields from a `StatusSink`
/// callback, which only gets the bare id. Mirrors `flows_panel`'s own
/// helper of the same shape.
fn collect_action_meta(
    actions: &workflow_engine::ActionMap,
    out: &mut HashMap<String, (String, String)>,
) {
    for (id, action) in actions {
        let name = action.label.clone().unwrap_or_else(|| id.clone());
        out.insert(id.clone(), (name, action.type_id.clone()));
        if let Some(nested) = &action.actions {
            collect_action_meta(nested, out);
        }
        if let Some(branch) = &action.else_branch {
            collect_action_meta(&branch.actions, out);
        }
        if let Some(branch) = &action.catch {
            collect_action_meta(&branch.actions, out);
        }
    }
}

/// Appends `entry` to the scoped-kv run history — the same
/// `flows_panel::persistence` key convention (`history:<flow_id>` under
/// the project-root namespace), duplicated here rather than depending on
/// `flows_panel` from this crate (that dependency would run the wrong way
/// — `flows_panel`'s "Graph" button depends on `designer_panel`, not the
/// reverse).
async fn flows_panel_history_append(
    store: &db::kvp::KeyValueStore,
    namespace: &str,
    flow_id: &str,
    entry: RunHistoryEntry,
) -> anyhow::Result<()> {
    let key = format!("history:{flow_id}");
    let raw = store.scoped(namespace).read(&key)?;
    let mut entries: Vec<RunHistoryEntry> = raw
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_default();
    entries.push(entry);
    let max = workflow_engine::history::MAX_HISTORY_ENTRIES;
    if entries.len() > max {
        let drop = entries.len() - max;
        entries.drain(0..drop);
    }
    let json = serde_json::to_string(&entries)?;
    store.scoped(namespace).write(key, json).await
}

/// The under-canvas / results-pane display helper shared with `run_results`:
/// a `Succeeded` action's `detail` is serialized `outputs` JSON, so it gets
/// pretty-printed; a `Failed` action's detail is an engine error string and
/// is kept as-is.
pub(crate) fn format_detail(detail: &str) -> String {
    if detail.trim().starts_with('{')
        && let Ok(value) = serde_json::from_str::<serde_json::Value>(detail)
    {
        serde_json::to_string_pretty(&value).unwrap_or_else(|_| detail.to_string())
    } else {
        detail.to_string()
    }
}

/// Compact duration text, e.g. `42ms` / `3.2s`.
pub(crate) fn format_ms(d: Duration) -> String {
    if d.as_secs() >= 1 {
        format!("{:.1}s", d.as_secs_f32())
    } else {
        format!("{}ms", d.as_millis())
    }
}

/// Phase → theme color, shared by the transcript rows and the per-node
/// result card (Skipped gets the muted foreground so it reads as "not run").
pub(crate) fn phase_color(phase: RunPhase, theme: &Theme) -> gpui::Hsla {
    match phase {
        RunPhase::Running => theme.primary,
        RunPhase::Succeeded => theme.success,
        RunPhase::Failed => theme.danger,
        RunPhase::Skipped => theme.muted,
    }
}

impl Focusable for DesignerPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for DesignerPanel {}

impl Item for DesignerPanel {
    type Event = ();

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        format!("{}.flow.json", self.flow_name).into()
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<ui::Icon> {
        Some(ui::Icon::new(ui::IconName::GitBranch))
    }

    fn tab_extra_context_menu_actions(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Vec<(SharedString, Box<dyn Action>)> {
        vec![("View Raw".into(), Box::new(ViewRaw(self.path.clone())))]
    }

    /// The panel emits `()` when its unsaved or conflict state flips, which
    /// is when the tab needs redrawing.
    fn to_item_events(_event: &Self::Event, f: &mut dyn FnMut(ItemEvent)) {
        f(ItemEvent::UpdateTab);
        f(ItemEvent::Edit);
    }

    fn is_dirty(&self, _cx: &App) -> bool {
        self.dirty
    }

    fn has_conflict(&self, _cx: &App) -> bool {
        self.conflict
    }

    fn can_save(&self, _cx: &App) -> bool {
        true
    }

    /// The workspace's own save (the keybinding, and "Save" in the prompt
    /// shown when closing a tab with unsaved edits), routed to the same code
    /// as the toolbar button.
    fn save(
        &mut self,
        _options: SaveOptions,
        _project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> gpui::Task<anyhow::Result<()>> {
        self.on_save_click(window, cx);
        gpui::Task::ready(match &self.error {
            Some(error) => Err(anyhow::anyhow!("{error}")),
            None => Ok(()),
        })
    }

    /// Discards unsaved edits and loads the file as it is on disk.
    fn reload(
        &mut self,
        _project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> gpui::Task<anyhow::Result<()>> {
        self.reload_from_disk(true, window, cx);
        gpui::Task::ready(Ok(()))
    }
}

impl ProjectItem for DesignerPanel {
    type Item = FlowFile;

    /// Resolves the flow's abs path and project root from `item`/`project`
    /// and defers to [`DesignerPanel::build`] — the same infallible
    /// load path `open` uses, so a broken `.flow.json` opened by
    /// double-click still opens a tab (with its own error banner) rather
    /// than silently failing or falling back to `for_broken_project_item`.
    fn for_project_item(
        project: Entity<Project>,
        _pane: Option<&Pane>,
        item: Entity<FlowFile>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let flow_file = item.read(cx);
        let path = flow_file.abs_path().unwrap_or_default();
        let worktree_id = flow_file.worktree_id();
        let root = project
            .read(cx)
            .worktree_for_id(worktree_id, cx)
            .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
            .unwrap_or_default();

        Self::build(path, root, None, project, window, cx)
    }
}

impl SerializableItem for DesignerPanel {
    fn serialized_item_kind() -> &'static str {
        "designer_flow"
    }

    fn deserialize(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        workspace_id: WorkspaceId,
        item_id: ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> gpui::Task<anyhow::Result<Entity<Self>>> {
        let db = persistence::DesignerDb::global(cx);
        window.spawn(cx, async move |cx| {
            let path = db
                .flow_path(item_id, workspace_id)?
                .ok_or_else(|| anyhow::anyhow!("No Designer flow path found for item"))?;
            let path = PathBuf::from(path);
            let (worktree, _) = project
                .update(cx, |project, cx| {
                    project.find_or_create_worktree(path.clone(), false, cx)
                })
                .await
                .context("Flow path is not inside a project worktree")?;
            let root = worktree.read_with(cx, |worktree, _| worktree.abs_path().to_path_buf());

            cx.update(|window, cx| {
                let workspace = workspace
                    .upgrade()
                    .ok_or_else(|| anyhow::anyhow!("Workspace released before Designer restore"))?;
                Ok(cx.new(|cx| {
                    Self::build(path, root, Some(workspace.downgrade()), project, window, cx)
                }))
            })?
        })
    }

    fn cleanup(
        workspace_id: WorkspaceId,
        alive_items: Vec<ItemId>,
        _window: &mut Window,
        cx: &mut App,
    ) -> gpui::Task<anyhow::Result<()>> {
        let db = persistence::DesignerDb::global(cx);
        cx.background_spawn(async move { db.delete_unloaded(workspace_id, alive_items).await })
    }

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: ItemId,
        _closing: bool,
        cx: &mut Context<Self>,
    ) -> Option<gpui::Task<anyhow::Result<()>>> {
        let workspace_id = workspace.database_id()?;
        let path = self.path.to_string_lossy().into_owned();
        let db = persistence::DesignerDb::global(cx);
        Some(cx.background_spawn(async move { db.save_flow(item_id, workspace_id, path).await }))
    }

    fn should_serialize(&self, _event: &Self::Event) -> bool {
        false
    }
}

impl Render for DesignerPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.sync_selection(window, cx);

        v_flex()
            .id("designer-panel")
            .track_focus(&self.focus_handle(cx))
            .size_full()
            .bg(cx.theme().background)
            .child(self.render_toolbar(cx))
            .when_some(self.error.clone(), |el, error| {
                el.child(
                    div()
                        .px_3()
                        .py_1()
                        .text_xs()
                        .text_color(cx.theme().danger)
                        .child(error),
                )
            })
            // The action menu for the "+" on a connection, drawn over the
            // whole tab at the point that was clicked.
            .when_some(self.insert_menu.as_ref(), |el, insert| {
                el.child(
                    deferred(
                        anchored()
                            .position(insert.position)
                            .snap_to_window_with_margin(px(8.))
                            .child(insert.menu.clone()),
                    )
                    .with_priority(1),
                )
            })
            .when(self.conflict, |el| {
                el.child(
                    h_flex()
                        .px_3()
                        .py_1()
                        .gap_2()
                        .items_center()
                        .child(
                            div()
                                .text_xs()
                                .text_color(cx.theme().warning)
                                .child(
                                    "This flow changed on disk while it has unsaved edits. \
                                     Save to keep your edits, or reload to discard them.",
                                ),
                        )
                        .child(
                            Button::new("designer-reload-from-disk")
                                .ghost()
                                .xsmall()
                                .label("Reload from disk")
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.reload_from_disk(true, window, cx)
                                })),
                        ),
                )
            })
            .child(
                v_flex()
                    .flex_1()
                    .min_h_0()
                    .child(
                        div().flex_1().min_h_0().child(
                            // Tuple id, not a bare string: the split's saved
                            // sizes are keyed per-entity, and every flow tab is
                            // its own `DesignerPanel`, so a shared `"split"`
                            // id would have the tabs overwriting each other's
                            // pane widths. Same convention `database_panel`
                            // uses for its three-pane body split.
                            h_resizable(("designer-body", cx.entity().entity_id()))
                                // Catalog — fixed-ish, collapsible by dragging
                                // its handle to the very edge.
                                .child(
                                    resizable_panel()
                                        .size(px(240.0))
                                        .size_range(px(180.0)..px(420.0))
                                        .flex_none()
                                        .child(self.render_catalog(cx)),
                                )
                                // Canvas — the flexible middle pane, bare
                                // `resizable_panel()` so it absorbs the
                                // remaining width.
                                .child(resizable_panel().child(self.render_canvas(cx)))
                                .child(
                                    resizable_panel()
                                        .size(px(280.0))
                                        .size_range(px(220.0)..px(420.0))
                                        .flex_none()
                                        .child(
                                            // Scrolls on its own: the rows,
                                            // the optional-input list and the
                                            // reference chips easily outgrow
                                            // the pane.
                                            div()
                                                .relative()
                                                .size_full()
                                                .border_l_1()
                                                .border_color(cx.theme().border)
                                                .child(
                                                    div()
                                                        .id("designer-inspector-scroll")
                                                        .size_full()
                                                        .overflow_y_scroll()
                                                        .track_scroll(&self.inspector_scroll)
                                                        .child(self.render_properties(cx)),
                                                )
                                                .child(Scrollbar::vertical(
                                                    &self.inspector_scroll,
                                                )),
                                        ),
                                ),
                        ),
                    )
                    .when(self.show_log && !self.run_snapshot.log.is_empty(), |el| {
                        el.child(self.render_run_log(cx).into_any_element())
                    }),
            )
    }
}

impl DesignerPanel {
    /// The left pane: a titled action catalog over
    /// `catalog::CatalogDelegate`. Search, group headings, keyboard
    /// navigation, and list semantics all come from the shared
    /// `gpui_component` list widget.
    fn render_catalog(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let (shown, total) = {
            let delegate = self.catalog.read(cx).delegate();
            (delegate.shown_count(), delegate.total_count())
        };
        v_flex()
            .size_full()
            .min_w_0()
            .border_r_1()
            .border_color(cx.theme().border)
            .child(catalog::header(cx, shown, total))
            .child(
                // `min_h_0` so the virtual list can shrink below its content
                // height in a short window instead of pushing the canvas
                // pane out of the split.
                div().flex_1().min_h_0().child(
                    List::new(&self.catalog)
                        .search_placeholder("Search actions")
                        .with_size(Size::Small),
                ),
            )
            .into_any_element()
    }

    /// The middle pane: the `gpui_flow` canvas, its zoom controls and minimap,
    /// and the drag-and-drop target for catalog rows.
    fn render_canvas(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .min_w_0()
            .child(
                div()
                    .id("designer-flow-context")
                    .size_full()
                    // Bubble-phase mouse listener: `FlowGraph` never stops
                    // propagation, so this fires for a drop anywhere over the
                    // canvas even though the graph itself is the element under
                    // the cursor.
                    .on_drop::<catalog::CatalogDrag>(cx.listener(
                        |this, drag: &catalog::CatalogDrag, window, cx| {
                            let bounds = this.flow.read(cx).panel_bounds();
                            let viewport = this.state.read(cx).viewport;
                            // The drop point has to be read from the *window*,
                            // since drag payloads carry no position of their
                            // own, and the graph's own bounds don't include
                            // the catalog/inspector panes around it.
                            let position =
                                catalog::drop_position(window.mouse_position(), bounds, viewport);
                            this.add_action_at(drag.type_id, Some(position), cx);
                        },
                    ))
                    .drag_over::<catalog::CatalogDrag>(|style, _drag, _window, cx| {
                        // Outline the whole canvas while a catalog row is
                        // over it: the drop lands wherever the cursor is, so
                        // the feedback has to cover the whole surface rather
                        // than one box.
                        style.border_2().border_color(cx.theme().primary)
                    })
                    .context_menu({
                        let panel = cx.entity().downgrade();
                        move |menu, window, cx| {
                            // Groups are built with `PopupMenu::build` rather
                            // than `PopupMenu::submenu`'s builder sugar: that
                            // builder is an `Fn`, so nesting one inside
                            // another would have to move the outer builder's
                            // borrowed `window`/`cx` into a closure that may
                            // run more than once. `build` takes `FnOnce`, so
                            // the grouping is buildable — just not through the
                            // nested sugar. Plain `for` loops keep the
                            // `window` reborrows in statement position.
                            let mut groups = Vec::new();
                            for section in catalog::build_sections() {
                                let group = PopupMenu::build(window, cx, |mut group, _w, _c| {
                                    for entry in section.entries {
                                        let type_id = entry.type_id;
                                        let runnable = entry.runnable;
                                        // Not-runnable actions stay *visible*
                                        // here, matching the catalog pane, but
                                        // are disabled and annotated with the
                                        // same `not_ready_reason` the pane's
                                        // tooltip explains. A menu that silently
                                        // omits them reads as "this Designer
                                        // can't do that" rather than "this
                                        // isn't built yet".
                                        let label: SharedString = if runnable {
                                            entry.label.clone()
                                        } else {
                                            format!(
                                                "{} ({})",
                                                entry.label,
                                                catalog::not_ready_reason(type_id)
                                            )
                                            .into()
                                        };
                                        let panel = panel.clone();
                                        group = group.item(
                                            PopupMenuItem::new(label).disabled(!runnable).when(
                                                runnable,
                                                |item| {
                                                    item.on_click(move |_, _, cx| {
                                                        let _ = panel.update(cx, |panel, cx| {
                                                            panel.add_action(type_id, cx)
                                                        });
                                                    })
                                                },
                                            ),
                                        );
                                    }
                                    group
                                });
                                groups.push(PopupMenuItem::submenu(section.title.clone(), group));
                            }
                            let types = PopupMenu::build(window, cx, |mut types, _w, _c| {
                                for group in groups {
                                    types = types.item(group);
                                }
                                types
                            });
                            menu.label("Add Action")
                                .item(PopupMenuItem::submenu("Choose action type", types))
                        }
                    })
                    .child(self.flow.clone()),
            )
            .child(
                div()
                    .absolute()
                    .bottom(px(16.0))
                    .left(px(16.0))
                    .child(self.controls.clone()),
            )
            .child(
                div()
                    .absolute()
                    .bottom(px(16.0))
                    .right(px(16.0))
                    .child(self.minimap.clone()),
            )
            .into_any_element()
    }
}

/// One `Foreach` pager in the inspector: which loop, which of its iterations
/// is being looked at, and what the run recorded for it.
struct LoopPage {
    loop_id: NodeId,
    label: String,
    /// Zero-based iteration shown.
    index: usize,
    total: usize,
    /// The item that iteration was bound to, when the run reached it.
    item: Option<serde_json::Value>,
}

impl DesignerPanel {
    /// The `Foreach` actions `node_id` runs inside, outermost first, each
    /// with the iteration the inspector is showing for it.
    fn loop_path(&self, node_id: &NodeId, cx: &App) -> Vec<(String, usize)> {
        let mut loops = Self::reference_scope(self.state.read(cx), node_id).loops;
        loops.reverse();
        let mut path = Vec::new();
        for id in loops {
            let index = self.loop_index(&path, &id);
            path.push((id.to_string(), index));
        }
        path
    }

    /// The iteration being shown for `loop_id`, which runs inside `path`.
    /// Held to the loop's size on the last run, so a selection made against
    /// a longer list doesn't point past the end of a shorter one.
    fn loop_index(&self, path: &[(String, usize)], loop_id: &NodeId) -> usize {
        let wanted = self
            .loop_view
            .get::<str>(loop_id.as_ref())
            .copied()
            .unwrap_or(0);
        let key = format!("{}/{loop_id}", run_state::scope_key(path));
        match self.run_snapshot.loops.get(&key) {
            Some(run) if run.total > 0 => wanted.min(run.total - 1),
            _ => wanted,
        }
    }

    /// The pagers to show for `node_id`: one per `Foreach` it runs inside,
    /// and one for itself when it is a `Foreach`. Loops the last run never
    /// entered (no run yet, or an empty list) get none.
    fn loop_pages(&self, node_id: &NodeId, cx: &App) -> Vec<LoopPage> {
        let state = self.state.read(cx);
        let mut path = self.loop_path(node_id, cx);
        let is_loop = state
            .get_node(node_id)
            .is_some_and(|node| node.node_type.as_deref() == Some("Foreach"));
        if is_loop {
            let index = self.loop_index(&path, node_id);
            path.push((node_id.to_string(), index));
        }

        let mut pages = Vec::new();
        for (depth, (loop_id, index)) in path.iter().enumerate() {
            let key = format!("{}/{loop_id}", run_state::scope_key(&path[..depth]));
            let Some(run) = self.run_snapshot.loops.get(&key).filter(|run| run.total > 0) else {
                // Without this loop's items the inner ones can't be placed.
                break;
            };
            let index = (*index).min(run.total - 1);
            let loop_id: NodeId = loop_id.clone().into();
            pages.push(LoopPage {
                label: state
                    .get_node(&loop_id)
                    .map(node_label)
                    .unwrap_or_else(|| loop_id.to_string()),
                loop_id,
                index,
                total: run.total,
                item: run.items.get(&index).cloned(),
            });
        }
        pages
    }

    /// What the last run recorded for `node_id`: for a loop body, the
    /// iteration the pagers are on; otherwise its one result.
    fn run_result_for(&self, node_id: &NodeId, cx: &App) -> Option<ActionRun> {
        let path = self.loop_path(node_id, cx);
        if path.is_empty() {
            return self.run_snapshot.nodes.get(node_id.as_ref()).cloned();
        }
        self.run_snapshot
            .iterations
            .get(&run_state::scope_key(&path))
            .and_then(|results| results.get(node_id.as_ref()))
            .cloned()
    }

    fn on_loop_page_click(&mut self, loop_id: &NodeId, index: usize, cx: &mut Context<Self>) {
        self.loop_view.insert(loop_id.to_string(), index);
        cx.notify();
    }

    /// `For each · <loop>   ‹ 2 / 12 ›`, and under it the item that
    /// iteration ran for. Stepping it changes which iteration's results the
    /// inspector shows for every action in that loop's body.
    fn render_loop_page(&self, page: &LoopPage, cx: &mut Context<Self>) -> impl IntoElement {
        let muted = cx.theme().muted_foreground;
        let step = |id: &'static str, icon: IconName, target: Option<usize>| {
            let loop_id = page.loop_id.clone();
            Button::new(SharedString::from(format!("{id}-{}", page.loop_id)))
                .ghost()
                .xsmall()
                .icon(icon)
                .disabled(target.is_none())
                .on_click(cx.listener(move |this, _, _, cx| {
                    if let Some(target) = target {
                        this.on_loop_page_click(&loop_id, target, cx);
                    }
                }))
        };
        let previous = step(
            "designer-loop-prev",
            IconName::ChevronLeft,
            page.index.checked_sub(1),
        );
        let next = step(
            "designer-loop-next",
            IconName::ChevronRight,
            (page.index + 1 < page.total).then_some(page.index + 1),
        );

        v_flex()
            .gap_1()
            .p_2()
            .rounded_sm()
            .border_1()
            .border_color(cx.theme().border)
            .child(
                h_flex()
                    .gap_1()
                    .items_center()
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .text_xs()
                            .text_color(muted)
                            .child(format!("For each \u{00b7} {}", page.label)),
                    )
                    .child(previous)
                    .child(
                        div()
                            .text_xs()
                            .font_weight(gpui::FontWeight::MEDIUM)
                            .text_color(cx.theme().foreground)
                            .child(format!("{} / {}", page.index + 1, page.total)),
                    )
                    .child(next),
            )
            .child(match &page.item {
                Some(item) => div()
                    .id(SharedString::from(format!(
                        "designer-loop-item-{}",
                        page.loop_id
                    )))
                    .max_h(gpui::px(120.0))
                    .overflow_y_scroll()
                    .text_xs()
                    .font_family("monospace")
                    .text_color(cx.theme().foreground)
                    .child(
                        serde_json::to_string_pretty(item).unwrap_or_else(|_| item.to_string()),
                    )
                    .into_any_element(),
                None => div()
                    .text_xs()
                    .text_color(muted)
                    .child("This iteration hasn't started.")
                    .into_any_element(),
            })
    }

    fn render_run_result(&self, run: &ActionRun, cx: &mut Context<Self>) -> impl IntoElement {
        let tag = match run.phase {
            RunPhase::Running => Tag::primary().outline(),
            RunPhase::Succeeded => Tag::success().outline(),
            RunPhase::Failed => Tag::danger().outline(),
            RunPhase::Skipped => Tag::secondary().outline(),
        };
        let detail_color = match run.phase {
            RunPhase::Failed => cx.theme().danger,
            _ => cx.theme().foreground,
        };
        let detail = run.detail.as_deref().filter(|d| !d.is_empty());

        v_flex()
            .gap_1()
            .p_2()
            .rounded_sm()
            .border_1()
            .border_color(if run.phase == RunPhase::Failed {
                cx.theme().danger
            } else {
                cx.theme().border
            })
            .child(
                h_flex()
                    .gap_1p5()
                    .items_center()
                    .child(tag.child(run.phase.label()))
                    .when(run.phase == RunPhase::Running, |el| {
                        el.child(Spinner::new().xsmall())
                    })
                    .when_some(run.elapsed(), |el, d| {
                        el.child(
                            div()
                                .ml_auto()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child(format_ms(d)),
                        )
                    }),
            )
            .child(
                div()
                    .mt_1()
                    .text_sm()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_color(cx.theme().foreground)
                    .child(run.name.clone()),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child(run.type_id.clone()),
            )
            .when_some(detail, |el, d| {
                el.child(
                    div()
                        .id("designer-run-detail")
                        .mt_1()
                        .max_h(gpui::px(140.0))
                        .overflow_y_scroll()
                        .text_xs()
                        .font_family("monospace")
                        .text_color(detail_color)
                        .child(format_detail(d)),
                )
            })
    }

    /// The under-canvas transcript pane (`show_log` toggle in the toolbar):
    /// a chronological, color-coded trace of every status event the run
    /// emitted, with its own idle-state totals and a scrollbar.
    fn render_run_log(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let (succeeded, failed, skipped) = self.run_snapshot.totals();
        v_flex()
            .id("designer-run-log")
            .flex_shrink_0()
            .h(gpui::px(180.0))
            .border_t_1()
            .border_color(cx.theme().border)
            .child(
                h_flex()
                    .px_3()
                    .py_1()
                    .gap_2()
                    .items_center()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .child(
                        Icon::new(IconName::SquareTerminal)
                            .xsmall()
                            .text_color(cx.theme().muted_foreground),
                    )
                    .child(
                        div()
                            .text_xs()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(cx.theme().foreground)
                            .child("Run log"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child(format!(
                                "{succeeded} ok \u{00b7} {failed} failed \u{00b7} {skipped} skipped"
                            )),
                    )
                    .child(
                        Button::new("designer-log-collapse")
                            .ghost()
                            .xsmall()
                            .icon(IconName::ChevronDown)
                            .tooltip("Close the run log")
                            .on_click(cx.listener(|this, _, _, cx| this.on_toggle_log_click(cx))),
                    ),
            )
            .child(
                div()
                    .relative()
                    .flex_1()
                    .size_full()
                    .child(
                        div()
                            .id("designer-run-log-scroll")
                            .size_full()
                            .overflow_y_scroll()
                            .track_scroll(&mut self.log_scroll)
                            .children(self.run_snapshot.log.iter().enumerate().map(|(i, line)| {
                                self.render_log_row(i, line, cx).into_any_element()
                            })),
                    )
                    .child(Scrollbar::vertical(&self.log_scroll)),
            )
    }

    fn render_log_row(
        &self,
        index: usize,
        line: &RunLogLine,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let color = phase_color(line.phase, cx.theme());
        let detail = line.detail.as_deref().unwrap_or("");
        let phase_label = if line.phase == RunPhase::Running {
            "running\u{2026}".to_string()
        } else {
            format!(
                "{} in {:.1}s",
                line.phase.label(),
                (line.time_ms as f64) / 1000.0
            )
        };
        let action_id = line.action_id.clone();
        h_flex()
            .id(SharedString::from(format!("designer-log-row-{index}")))
            .w_full()
            .gap_2()
            .items_center()
            .px_3()
            .py_1()
            .hover(|d| d.bg(cx.theme().muted.opacity(0.2)))
            .on_click(cx.listener(move |this, _event, _window, cx| {
                this.select_node(action_id.clone().into(), cx);
            }))
            .child(div().size_1p5().rounded_full().flex_shrink_0().bg(color))
            .child(
                div()
                    .text_xs()
                    .font_weight(gpui::FontWeight::MEDIUM)
                    .text_color(cx.theme().foreground)
                    .child(line.name.clone()),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child(line.type_id.clone()),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .truncate()
                    .text_xs()
                    .font_family("monospace")
                    .text_color(cx.theme().muted_foreground)
                    .child(if detail.is_empty() {
                        phase_label
                    } else {
                        detail.to_string()
                    }),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child(format!("{}ms", line.time_ms)),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The four synthetic branch types must stay in lockstep with
    /// `workflow_json`'s own set — a wrapper that this list misses becomes
    /// deletable again, which destroys the branch body it stands for.
    #[test]
    fn branch_wrapper_detection_matches_the_canonical_set() {
        for wrapper in [
            workflow_json::BRANCH_THEN,
            workflow_json::BRANCH_ELSE,
            workflow_json::BRANCH_TRY,
            workflow_json::BRANCH_CATCH,
        ] {
            assert!(is_branch_wrapper_type(wrapper), "{wrapper}");
            assert!(workflow_json::is_branch_wrapper(wrapper), "{wrapper}");
        }
        // Real action and container types must not be mistaken for wrappers.
        for real in ["log", "http", "script", "If", "Foreach", "Until", "Try"] {
            assert!(!is_branch_wrapper_type(real), "{real}");
        }
    }

    fn typed(id: &'static str, node_type: &'static str) -> FlowNode {
        FlowNode::new(id, 0.0, 0.0).node_type(node_type)
    }

    fn edge(source: &'static str, target: &'static str) -> FlowEdge {
        FlowEdge::new(format!("e_{source}_{target}"), source, target)
    }

    fn references(state: &FlowState, node_id: &'static str) -> Vec<String> {
        DesignerPanel::expression_references(state, &node_id.into())
    }

    /// The paths are the ones `RunContext::record_outputs` stores, and they
    /// follow the chain back rather than stopping at the direct dependency.
    #[test]
    fn references_follow_predecessors_and_use_the_outputs_path() {
        let state = FlowState::new(
            vec![
                typed("fetch", "http"),
                typed("shape", "transform"),
                typed("gate", "If"),
                typed("later", "http"),
                typed("unrelated", "http"),
            ],
            vec![
                edge("fetch", "shape"),
                edge("shape", "gate"),
                edge("gate", "later"),
            ],
        );
        assert_eq!(
            references(&state, "gate"),
            vec![
                "fetch.outputs.body".to_string(),
                "fetch.outputs.status".to_string(),
                "shape.outputs.result".to_string(),
            ]
        );
        // Nothing upstream, nothing to offer: not a successor, not a sibling.
        assert!(references(&state, "fetch").is_empty());
    }

    /// An initialized variable upstream is offered by its own name.
    #[test]
    fn an_upstream_variable_is_offered_by_name() {
        let mut declare = typed("declare", "InitVariable");
        declare.properties.push(("name".into(), "count".into()));
        let state = FlowState::new(
            vec![declare, typed("use", "transform")],
            vec![edge("declare", "use")],
        );
        assert_eq!(
            references(&state, "use"),
            vec![
                "declare.outputs.name".to_string(),
                "declare.outputs.value".to_string(),
                "variables.count".to_string(),
            ]
        );
    }

    #[test]
    fn a_failed_predecessor_is_followed_but_not_offered() {
        let state = FlowState::new(
            vec![
                typed("fetch", "http"),
                typed("charge", "http"),
                typed("report", "transform"),
            ],
            vec![
                edge("fetch", "charge"),
                edge("charge", "report").label("Failed"),
            ],
        );
        assert_eq!(
            references(&state, "report"),
            vec![
                "fetch.outputs.body".to_string(),
                "fetch.outputs.status".to_string(),
            ]
        );
    }

    #[test]
    fn a_nested_action_sees_its_enclosing_scopes() {
        let catch = format!("guard{}", workflow_json::BRANCH_CATCH);
        let state = FlowState::new(
            vec![
                typed("fetch", "http"),
                typed("each", "Foreach"),
                typed("guard", "Try").parent("each"),
                FlowNode::new(catch.clone(), 0.0, 0.0)
                    .node_type(workflow_json::BRANCH_CATCH)
                    .parent("guard"),
                typed("recover", "transform").parent(catch),
            ],
            vec![edge("fetch", "each")],
        );
        assert_eq!(
            references(&state, "recover"),
            vec![
                "$item".to_string(),
                "fetch.outputs.body".to_string(),
                "fetch.outputs.status".to_string(),
                "guard.outputs.error".to_string(),
            ]
        );
    }

    #[test]
    fn an_until_condition_sees_its_own_body() {
        let state = FlowState::new(
            vec![
                typed("poll", "Until"),
                typed("status", "http").parent("poll"),
            ],
            Vec::new(),
        );
        assert_eq!(
            references(&state, "poll"),
            vec![
                "status.outputs.body".to_string(),
                "status.outputs.status".to_string(),
            ]
        );
    }

    /// Deleting a container from the keyboard removes just that node; its
    /// body, and the body of anything nested in it, has to go with it.
    #[test]
    fn pruning_removes_a_deleted_containers_whole_body() {
        let then = format!("cond{}", workflow_json::BRANCH_THEN);
        let mut state = FlowState::new(
            vec![
                typed("keep", "http"),
                typed("each", "Foreach"),
                typed("cond", "If").parent("each"),
                FlowNode::new(then.clone(), 0.0, 0.0)
                    .node_type(workflow_json::BRANCH_THEN)
                    .parent("cond"),
                typed("inner", "log").parent(then),
            ],
            vec![edge("keep", "each"), edge("keep", "inner")],
        );
        assert!(!DesignerPanel::prune_orphans(&mut state), "nothing orphaned yet");

        // What the canvas's Delete key does to a selected container.
        state.nodes.retain(|node| node.id.as_ref() != "each");
        state.edges.retain(|edge| edge.target.as_ref() != "each");
        state.rebuild_lookup();

        assert!(DesignerPanel::prune_orphans(&mut state));
        let left: Vec<&str> = state.nodes.iter().map(|node| node.id.as_ref()).collect();
        assert_eq!(left, vec!["keep"]);
        assert!(state.edges.is_empty());
    }

    fn sample_map(entries: &[(&str, serde_json::Value)]) -> HashMap<String, serde_json::Value> {
        entries
            .iter()
            .map(|(id, outputs)| (id.to_string(), outputs.clone()))
            .collect()
    }

    fn sampled_paths(
        state: &FlowState,
        samples: &HashMap<String, serde_json::Value>,
        node_id: &'static str,
    ) -> Vec<String> {
        let known = references(state, node_id);
        DesignerPanel::sampled_references(state, samples, &known, &node_id.into())
            .into_iter()
            .map(|sampled| sampled.path)
            .collect()
    }

    /// A sampled result is walked into, a list is offered whole, and what
    /// the registry already offers isn't repeated.
    #[test]
    fn sampled_references_walk_what_an_action_returned() {
        let state = FlowState::new(
            vec![typed("a", "script"), typed("use", "transform")],
            vec![edge("a", "use")],
        );
        let samples = sample_map(&[(
            "a",
            serde_json::json!({
                "id": 7,
                "title": "T",
                "object": {"id": 1, "title": "O"},
                "items": [{"id": 1, "title": "x"}, {"id": 2, "title": "y"}],
            }),
        )]);
        assert_eq!(
            sampled_paths(&state, &samples, "use"),
            vec![
                "a.outputs.id",
                "a.outputs.items",
                "a.outputs.object",
                "a.outputs.object.id",
                "a.outputs.object.title",
                "a.outputs.title",
            ]
        );

        let sampled =
            DesignerPanel::sampled_references(&state, &samples, &[], &NodeId::from("use"));
        let items = sampled.iter().find(|s| s.path == "a.outputs.items").unwrap();
        assert_eq!((items.kind, items.preview.as_str()), ("list", "a list of 2"));

        // An `http` response sits under the declared `body` output, which the
        // registry chip already covers, so only what is inside it is new.
        let state = FlowState::new(
            vec![typed("fetch", "http"), typed("use", "transform")],
            vec![edge("fetch", "use")],
        );
        let samples = sample_map(&[(
            "fetch",
            serde_json::json!({"status": 200, "body": {"customer": {"id": 3}}}),
        )]);
        assert_eq!(
            sampled_paths(&state, &samples, "use"),
            vec![
                "fetch.outputs.body.customer",
                "fetch.outputs.body.customer.id",
            ]
        );
        // Nothing sampled for an action that isn't upstream.
        assert!(sampled_paths(&state, &samples, "fetch").is_empty());
    }

    /// Inside a `Foreach` over a plain reference, the sampled list's first
    /// element describes `$item`.
    #[test]
    fn a_foreach_body_gets_the_shape_of_its_item() {
        let mut each = typed("each", "Foreach");
        each.properties
            .push(("__foreach".into(), r#"{"var": "a.outputs.items"}"#.into()));
        let state = FlowState::new(
            vec![
                typed("a", "script"),
                each,
                typed("body", "transform").parent("each"),
            ],
            vec![edge("a", "each")],
        );
        let samples = sample_map(&[(
            "a",
            serde_json::json!({"items": [{"id": 1, "title": "x"}]}),
        )]);
        assert_eq!(
            sampled_paths(&state, &samples, "body"),
            vec!["$item.id", "$item.title", "a.outputs.items"]
        );

        // A computed list has nothing to look up, so `$item` stays bare.
        let mut each = typed("each", "Foreach");
        each.properties.push((
            "__foreach".into(),
            r#"{"filter": [{"var": "a.outputs.items"}, true]}"#.into(),
        ));
        let state = FlowState::new(
            vec![
                typed("a", "script"),
                each,
                typed("body", "transform").parent("each"),
            ],
            vec![edge("a", "each")],
        );
        assert_eq!(
            sampled_paths(&state, &samples, "body"),
            vec!["a.outputs.items"]
        );
    }

    /// An edit makes its own result and everything after it stale, but not
    /// what ran before it, including earlier steps of the same loop body.
    #[test]
    fn an_edit_is_downstream_of_itself_and_what_follows() {
        let state = FlowState::new(
            vec![
                typed("first", "http"),
                typed("each", "Foreach"),
                typed("early", "http").parent("each"),
                typed("edited", "transform").parent("each"),
                typed("late", "http").parent("each"),
                typed("after", "Until"),
                typed("inner", "http").parent("after"),
            ],
            vec![
                edge("first", "each"),
                edge("early", "edited"),
                edge("edited", "late"),
                edge("each", "after"),
            ],
        );
        let mut affected: Vec<String> =
            DesignerPanel::downstream_of(&state, &NodeId::from("edited"))
                .into_iter()
                .map(|id| id.to_string())
                .collect();
        affected.sort();
        assert_eq!(affected, vec!["after", "each", "edited", "inner", "late"]);
    }

    /// Clicking an issue selects the node it is on, which for a nested action
    /// is the last path segment rather than the path.
    #[test]
    fn an_issue_resolves_to_the_node_it_is_on() {
        assert_eq!(
            DesignerPanel::issue_node_id("a1"),
            Some(NodeId::from("a1"))
        );
        assert_eq!(
            DesignerPanel::issue_node_id("foreach-1/if-1/then/a1"),
            Some(NodeId::from("a1"))
        );
        assert_eq!(DesignerPanel::issue_node_id(""), None);
    }

    /// A catalog click or a right-click Add Action on a container fills its
    /// body; on an ordinary action it adds the next step.
    #[test]
    fn adding_at_a_container_goes_inside_it() {
        let then = format!("cond{}", workflow_json::BRANCH_THEN);
        let state = FlowState::new(
            vec![
                typed("each", "Foreach"),
                typed("one", "http").parent("each"),
                typed("two", "transform").parent("each"),
                typed("empty", "Until"),
                typed("cond", "If"),
                workflow_json::branch_node("cond", workflow_json::BRANCH_THEN, false),
                workflow_json::branch_node("cond", workflow_json::BRANCH_ELSE, false),
            ],
            vec![edge("one", "two")],
        );
        let at = |id: &'static str| {
            DesignerPanel::beside_node(&state, state.get_node(&id.into()).unwrap())
        };

        // A body with one last action: inside, chained after it.
        let anchor = at("each");
        assert_eq!(anchor.parent_id, Some(NodeId::from("each")));
        assert_eq!(anchor.source_id, Some(NodeId::from("two")));
        assert!(anchor.local_position.is_some());

        // An empty body: inside, with nothing to chain from.
        let anchor = at("empty");
        assert_eq!(anchor.parent_id, Some(NodeId::from("empty")));
        assert_eq!(anchor.source_id, None);

        // An If hands it to its True branch.
        assert_eq!(at("cond").parent_id, Some(NodeId::from(then)));

        // An ordinary action: the next step, in the same body.
        let anchor = at("two");
        assert_eq!(anchor.parent_id, Some(NodeId::from("each")));
        assert_eq!(anchor.source_id, Some(NodeId::from("two")));
    }

    /// A variable is typed by its declaration until a run shows what it
    /// holds; what a Set Variable left in it wins over where it started.
    #[test]
    fn script_types_start_declared_and_follow_the_last_run() {
        let variable = |id: &'static str, kind: &'static str, name: &str, ty: Option<&str>| {
            let mut node = typed(id, kind);
            node.properties.push(("name".into(), name.to_string().into()));
            if let Some(ty) = ty {
                node.properties.push(("type".into(), ty.to_string().into()));
            }
            node
        };
        let state = FlowState::new(
            vec![
                variable("init-ids", "InitVariable", "ids", Some("array")),
                variable("init-name", "InitVariable", "name", Some("string")),
                variable("set-ids", "SetVariable", "ids", None),
                // Sets a variable nothing declares: not a variable to type.
                variable("set-stray", "SetVariable", "stray", None),
                typed("fetch", "http"),
            ],
            Vec::new(),
        );

        let sorted = |mut entries: Vec<(String, String)>| {
            entries.sort();
            entries
        };
        let (variables, actions) = DesignerPanel::script_types(&state, &HashMap::new());
        assert_eq!(
            sorted(variables),
            vec![
                ("ids".to_string(), "unknown[]".to_string()),
                ("name".to_string(), "string".to_string()),
            ]
        );
        assert!(actions.is_empty());

        let samples = sample_map(&[
            ("init-ids", serde_json::json!({"name": "ids", "value": []})),
            ("set-ids", serde_json::json!({"name": "ids", "value": [4, 5]})),
            ("set-stray", serde_json::json!({"name": "stray", "value": 1})),
            ("fetch", serde_json::json!({"status": 200})),
        ]);
        let (variables, actions) = DesignerPanel::script_types(&state, &samples);
        assert_eq!(
            sorted(variables),
            vec![
                ("ids".to_string(), "number[]".to_string()),
                ("name".to_string(), "string".to_string()),
            ]
        );
        assert!(actions.contains(&("fetch".to_string(), r#"{ "status": number }"#.to_string())));
    }

    /// Inserting above an action moves it and what follows it in the same
    /// body, and nothing else.
    #[test]
    fn an_insert_shifts_the_displaced_action_and_what_follows() {
        let state = FlowState::new(
            vec![
                typed("first", "http"),
                typed("second", "transform"),
                typed("third", "log"),
                typed("side", "log"),
                typed("each", "Foreach"),
                typed("inner", "log").parent("each"),
            ],
            vec![
                edge("first", "second"),
                edge("second", "third"),
                edge("first", "side"),
                edge("third", "each"),
            ],
        );
        let mut shifted: Vec<String> =
            DesignerPanel::shifted_by_insert(&state, &NodeId::from("second"))
                .into_iter()
                .map(|id| id.to_string())
                .collect();
        shifted.sort();
        // `each` moves and carries `inner` with it, since a child's position
        // is relative to its container; `side` is a separate branch.
        assert_eq!(shifted, vec!["each", "second", "third"]);
    }

    /// Unsaved-edit tracking counts what Save writes and nothing else.
    #[test]
    fn content_fingerprint_tracks_saved_content_only() {
        let build = || {
            FlowState::new(
                vec![typed("fetch", "http"), typed("shape", "transform")],
                vec![edge("fetch", "shape")],
            )
        };
        let saved = DesignerPanel::content_fingerprint(&build());

        // Not edits: selecting, a run's status border, a validation ring.
        let mut state = build();
        state.nodes[0].selected = true;
        state.nodes[0].accent_border = Some(0x00ff00);
        state.nodes[1].validation_error = true;
        assert_eq!(DesignerPanel::content_fingerprint(&state), saved);

        // Edits: a property, a label, a move, a new edge label.
        let mut state = build();
        state.nodes[0].properties.push(("url".into(), "x".into()));
        assert_ne!(DesignerPanel::content_fingerprint(&state), saved);

        let mut state = build();
        state.nodes[0].label = "Fetch".into();
        assert_ne!(DesignerPanel::content_fingerprint(&state), saved);

        let mut state = build();
        state.nodes[1].position.y += 10.0;
        assert_ne!(DesignerPanel::content_fingerprint(&state), saved);

        let mut state = build();
        state.edges[0].label = Some("Failed".into());
        assert_ne!(DesignerPanel::content_fingerprint(&state), saved);
    }

    /// An empty loop or branch is still a body, and an `If`/`Try` hands a
    /// drop on to its first branch.
    #[test]
    fn body_container_is_decided_by_type_not_by_having_children() {
        let then = format!("cond{}", workflow_json::BRANCH_THEN);
        let state = FlowState::new(
            vec![
                typed("each", "Foreach"),
                typed("cond", "If"),
                workflow_json::branch_node("cond", workflow_json::BRANCH_THEN, false),
                workflow_json::branch_node("cond", workflow_json::BRANCH_ELSE, false),
                typed("leaf", "http"),
            ],
            Vec::new(),
        );
        let body = |id: &'static str| {
            DesignerPanel::body_container(&state, state.get_node(&id.into()).unwrap())
                .map(|node| node.id.to_string())
        };
        assert_eq!(body("each"), Some("each".to_string()));
        assert_eq!(body("cond"), Some(then.clone()));
        assert_eq!(
            DesignerPanel::body_container(&state, state.get_node(&then.clone().into()).unwrap())
                .map(|node| node.id.to_string()),
            Some(then)
        );
        assert_eq!(body("leaf"), None);

        // First slot of an empty body, then under its lowest child.
        let first = DesignerPanel::next_slot(&state, &"each".into());
        assert_eq!((first.x, first.y), (workflow_json::CHILD_X, workflow_json::CHILD_Y0));
    }

    #[test]
    fn every_branch_wrapper_has_a_role_label() {
        assert_eq!(
            branch_wrapper_label(workflow_json::BRANCH_THEN),
            "the True branch"
        );
        assert_eq!(
            branch_wrapper_label(workflow_json::BRANCH_CATCH),
            "the Catch handler"
        );
    }

    /// A registry input is found under its own name; a container-only input
    /// under its reserved `__` key. This is what lets `sync_selection` render
    /// a typed row for `Foreach`'s `foreach` expression.
    #[test]
    fn meta_property_resolves_registry_and_container_keys() {
        let props: Vec<(SharedString, SharedString)> = vec![
            ("level".into(), "info".into()),
            ("__foreach".into(), r#"{"var": "items"}"#.into()),
            ("__limit_count".into(), "10".into()),
        ];
        assert_eq!(
            meta_property("log", &props, "level").map(|(_, v)| v.to_string()),
            Some("info".to_string())
        );
        assert_eq!(
            meta_property("Foreach", &props, "foreach").map(|(_, v)| v.to_string()),
            Some(r#"{"var": "items"}"#.to_string())
        );
        assert_eq!(
            meta_property("Until", &props, "limit").map(|(_, v)| v.to_string()),
            Some("10".to_string())
        );
        // Absent is the signal to render a row for a missing required field.
        assert!(meta_property("log", &props, "message").is_none());
    }

    /// Only a container's struct-level fields move to a reserved key.
    /// `transform` declares an `expression` too, and that one is a plain
    /// input: mapping it would hide the saved value and show an empty row.
    #[test]
    fn storage_key_reserves_keys_for_containers_only() {
        assert_eq!(storage_key("If", "expression"), "__expression");
        assert_eq!(storage_key("Foreach", "foreach"), "__foreach");
        assert_eq!(storage_key("Until", "until"), "__until");
        assert_eq!(storage_key("Until", "limit"), "__limit_count");
        assert_eq!(storage_key("transform", "expression"), "expression");
        assert_eq!(storage_key("http", "url"), "url");

        let props: Vec<(SharedString, SharedString)> =
            vec![("expression".into(), r#"{"var": "a.outputs.b"}"#.into())];
        assert_eq!(
            meta_property("transform", &props, "expression").map(|(_, v)| v.to_string()),
            Some(r#"{"var": "a.outputs.b"}"#.to_string())
        );
    }

    #[test]
    fn type_badges_are_distinct_per_kind() {
        let badges: Vec<_> = [
            FieldKind::String,
            FieldKind::Number,
            FieldKind::Bool,
            FieldKind::Json,
            FieldKind::Expression,
        ]
        .into_iter()
        .map(field_kind_badge)
        .collect();
        assert_eq!(badges, ["Text", "Number", "Bool", "JSON", "Expr"]);
    }

    /// `ActionDef`s must line up with what the inspector assumes: a declared
    /// field with no help text would silently lose its guidance line, and a
    /// required field is what drives the `*` marker and the empty-value error.
    #[test]
    fn the_registry_declares_the_metadata_the_inspector_renders() {
        for def in workflow_engine::registry::REGISTRY {
            for field in def.inputs {
                assert!(
                    !field.help.is_empty(),
                    "{}.{} has no help text",
                    def.type_id,
                    field.name
                );
            }
        }
    }

    /// A top-level action's issue path *is* its node id.
    #[test]
    fn issue_on_a_top_level_action_flags_that_node() {
        assert!(DesignerPanel::issue_targets_node("http-1", "http-1"));
        assert!(!DesignerPanel::issue_targets_node("http-1", "http-2"));
    }

    /// The case a naive `path == id` test gets wrong: a nested action's node
    /// id is bare while its issue path carries the whole container chain.
    #[test]
    fn issue_on_a_nested_action_flags_that_node() {
        assert!(DesignerPanel::issue_targets_node("foreach-1/a1", "a1"));
        // Branch wrappers contribute a path segment but are not themselves
        // nodes on the canvas.
        assert!(DesignerPanel::issue_targets_node("if-1/then/a1", "a1"));
        assert!(DesignerPanel::issue_targets_node(
            "foreach-1/if-1/then/a1",
            "a1"
        ));
    }

    /// Flagging ancestors as well as the offending action: an error buried in a
    /// container body has to light up the container the user can actually see.
    #[test]
    fn issue_inside_a_container_also_flags_the_container() {
        assert!(DesignerPanel::issue_targets_node(
            "foreach-1/a1",
            "foreach-1"
        ));
        assert!(DesignerPanel::issue_targets_node(
            "foreach-1/if-1/then/a1",
            "foreach-1"
        ));
        assert!(DesignerPanel::issue_targets_node(
            "foreach-1/if-1/then/a1",
            "if-1"
        ));
        // ...but not an unrelated sibling.
        assert!(!DesignerPanel::issue_targets_node(
            "foreach-1/a1",
            "foreach-2"
        ));
        assert!(!DesignerPanel::issue_targets_node("foreach-1/a1", "a2"));
    }

    /// `validate_definition` attributes a `runAfter` cycle or dangling
    /// reference to the definition root (`node_path == ""`). There is no node
    /// to flag, so no node may be flagged - the issue stays reachable through
    /// the overlay's "Workflow" row instead.
    #[test]
    fn a_flow_level_issue_flags_no_node() {
        for node_id in ["http-1", "foreach-1", "a1", ""] {
            assert!(
                !DesignerPanel::issue_targets_node("", node_id),
                "flow-level issue must not flag {node_id:?}"
            );
        }
    }
}
