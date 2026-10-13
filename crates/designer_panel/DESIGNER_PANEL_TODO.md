# Designer Panel UX and Script Editing TODO

Track the agreed work to make workflow authoring more discoverable and
user-friendly. Update this checklist as work lands; do not mark a phase
complete until its acceptance criteria are met.

## Agreed product requirements

- Keep the canvas and properties inspector, and add a third resizable pane for
  the action catalog.
- Make the catalog searchable and group actions by function (for example,
  Flow Control, Data, Integrations, Processes, and Utilities).
- Clicking a catalog action adds it near the selected node, or near the canvas
  center when nothing is selected; select the new node and show its inspector.
- Support dragging catalog actions onto the canvas for explicit placement.
- Preserve canvas right-click quick-add.
- Generate inspector fields from the workflow action registry. Use controls
  appropriate to the field type, show required status/help, and validate
  inputs. New actions start with required inputs blank and flagged; optional
  fields remain unset.
- Allow incomplete workflows to be saved as drafts. Show validation problems
  on affected nodes and in the inspector, and prevent running while blocking
  validation errors remain.
- Start with an assisted JSONLogic editor for expression inputs, with
  validation, examples, and completion for other actions' outputs. A guided
  expression builder is a later enhancement, not a prerequisite.
- Keep DB Migration and AI Check visible in the catalog but disabled with a
  clear explanation until their executors are implemented.
- Treat file scripts as workspace-only. Edit the selected workspace file
  directly, infer the runtime from its extension, and allow an override.
- Keep the current runtime scope: inline Python, Node, and PowerShell; .NET is
  file-only.
- Switching Inline to File transfers the code into a new workspace file;
  switching File to Inline loads the file contents. Never delete the original
  file as a side effect of switching modes.
- First prove that inline scripts can use project-aware LSP and reliably
  synchronize/save/reopen with the workflow. If inline scratch documents
  cannot support this, use visible, committable companion files in a sibling
  `<flow-name>.flow-scripts/<action-id>.<extension>` folder while retaining
  the Inline authoring mode. Nested action scopes must remain collision-free.
- Validate required fields, input types/JSON, JSONLogic expressions, script
  runtime/path, `runAfter` references, and actions without executors. Save is
  still allowed for drafts; Run is blocked until blocking errors are fixed.

### Changed since: scripts are file-only

The Inline/File requirements above are superseded. A script action runs a
file; the Designer no longer creates or edits inline code, and has no
Inline/File switch. Keeping the code in two places (the flow and a companion)
needed a two-way sync with conflict rules, and inline code ran from a
temporary copy, where relative imports and installed packages can't be found.

- A new Script gets its own file in `<flow>.flow-scripts/`, from a template.
- A flow that still has inline code is converted when opened: the code goes
  to its companion file and the action refers to that file.
- The engine still runs inline code, so a flow that hasn't been opened in the
  Designer, or is run from the Flows panel, keeps working. That support can
  be removed in a later version.
- Script paths are relative to the project root, as before. Moving a flow
  means moving its `.flow-scripts` folder and updating those paths.

## Phases

### Phase 1 findings

Initial source trace:

- Zed's `Editor::for_buffer` / `Editor::for_multibuffer` accepts a Project,
  and the Editor registers visible buffers with the Project's language servers.
- `Project::create_local_buffer` creates a language-tagged buffer without a
  file. `LspStore::register_buffer_with_language_servers` returns early when a
  buffer has no local `File`; language-server selection and document URI
  creation are also derived from the file/worktree path. Therefore a normal
  in-memory scratch buffer cannot receive full project LSP through the current
  registration path.
- The existing Designer "View Raw" path opens a workspace file with
  `Project::open_buffer` and wraps it in `Editor::for_buffer(..., Some(project),
  ...)`, which is the right existing integration to reuse for file-backed
  script editing. This source trace establishes the route, but has not yet
  verified live completions/diagnostics for each runtime.
- **Working direction:** pursue the agreed visible companion-file fallback for
  Inline mode unless a prototype proves a stable virtual-file URI can be
  registered safely. The agreed layout is a sibling folder such as
  `order.flow-scripts/` beside `order.flow.json`, with a normal project file
  buffer/editor for LSP. Inline code remains serialized in the flow; a saved
  companion is the latest source and hydrates the action on flow open, save,
  and run. Nested scopes/branches map to subfolders so repeated nested action
  IDs stay distinct. The prototype is in place, but complete in-application
  save/close/reopen and live-LSP verification is still outstanding.

### 1. Prove inline-script editor and LSP behavior

- [x] Trace Zed's existing editor and language-server plumbing; do not
  implement a new LSP client. Current in-memory buffers have no file identity
  and are intentionally skipped for LSP registration.
- [ ] Verify language selection for inline Python, Node, and PowerShell, plus
  file-mode runtime inference/override and file-only .NET.
- [x] Verify whether an ordinary inline scratch buffer has a stable,
  project-aware identity and can receive LSP. It does not with the current
  API; any virtual-file alternative still needs a prototype.
- [ ] Verify editor changes synchronize to the action and survive workflow
  save, close, and reopen without data loss.
- [x] Prototype workflow-managed sibling companion files; the Script
  inspector opens them through `Project::open_buffer` and `Editor::for_buffer`,
  and path/inline-code round-trip tests cover nested branches and reopen
  hydration.
- [ ] Verify in Zed that editing and saving a companion updates the flow on
  Save/Run and survives closing/reopening the flow; confirm a configured
  language server provides completions/diagnostics for the detected runtime.
- [x] Record the current finding and working storage/document-identity
  direction here before building the Script editor UX. Finalize it after the
  companion-file prototype and round-trip checks.

### 2. Define registry-backed validation metadata

- [x] Review the action registry as the source of truth for labels, input
  kinds, required status, help/examples, categories, and runnable status.
  Added `catalog_group: CatalogGroup`, `runnable: bool`, and `help: &'static
  str` on `InputField` to `ActionDef`; all REGISTRY entries updated.
- [x] Add only the metadata needed to generate clear catalog entries and typed
  inspector fields; avoid duplicating action definitions elsewhere.
  `CatalogGroup` (FlowControl / Data / Integrations / Processes / Utilities)
  and `runnable` live solely in `registry.rs`; `help` text lives on each
  `InputField`. No duplication of action definitions.
- [x] Implement validation for required fields, typed values/JSON, expressions,
  script runtime/path, `runAfter` references, and unavailable executors.
  `workflow_engine::validation::validate_definition` covers all of these;
  21 tests in `validation::tests` cover every check path including nested
  container paths.
- [x] Represent validation results so they can be displayed both in the
  inspector and on canvas nodes. `ValidationIssue { node_path, field,
  severity, message }` matches the layout-path key convention; inspector
  shows node-level issues as a banner and field-level issues inline.
  `DesignerPanel::validation_issues` is repopulated on every load, save,
  reload, and property edit via `revalidate()`.
- [x] Ensure draft save remains possible while blocking Run on validation
  errors. Save is always available; the Run button is disabled (with an
  error count badge and tooltip) when any `ValidationSeverity::Error` issue
  is present.

### 3. Build the three-pane Designer and action catalog

- [x] Add a searchable, functionally grouped catalog beside the canvas and
  inspector, using resizable panes.
- [x] Add an action by click near the selection/canvas center; select it and
  open its inspector.
- [x] Support drag-and-drop from the catalog to a canvas position.
- [x] Preserve right-click quick-add and keep it consistent with the catalog.
- [x] Show DB Migration and AI Check as disabled with a not-ready explanation.
  - **Changed since:** both, and Send Email, are now parked as future
    features — commented out of the registry, so they are not listed at all.
    The disabled-row behaviour remains for any `runnable: false` entry.
- [x] Check narrow-window behavior, pane resizing, keyboard navigation, and
  accessibility.
  - Code-level only: panes use the standard `resizable_panel` sizing (which
    scales the fixed panes proportionally when the window is too narrow, so
    the canvas is never starved to zero), and keyboard navigation plus ARIA
    come from the shared searchable `List` widget. **Still needs a live
    visual pass** — dragging the splitters at a few window widths, and
    tabbing through the catalog — before this can be called verified.

### 4. Replace generic properties with typed editors

- [x] Generate fields from registry metadata with type-appropriate text,
  number, boolean, JSON, and expression controls.
  - Rows are enumerated from `ActionDef::inputs` rather than from whatever
    happens to be in `FlowNode::properties`, so the inspector, the runtime
    validator, and the action catalog all read the same metadata. Declared
    fields render in registry order; undeclared extras follow.
  - `FieldKind::Bool` gets a `Switch`, `Number` a `NumberInput`, `Json` and
    `Expression` a multi-line `Textarea` (a JSON body or a JSONLogic
    expression on one line is unreadable), everything else a single-line
    `Input`. A `Json`/`Expression` row keeps its input and textarea in sync
    through one shared commit closure, so either can be read back.
- [x] Mark required fields, show concise guidance/examples, and provide
  field-level validation.
  - A `*` marker next to the name, the registry's `help` text underneath, and
    a type badge (`Text`/`Number`/`Bool`/`JSON`/`Expr`) so the expected shape
    is legible before typing.
  - **A missing required field now gets a row at all.** It previously produced
    no editor — the only signal was a message in the toolbar popover with
    nowhere to type a value. An empty required value is also reported inline
    ("✗ This field is required") rather than only in the popover.
  - Every `ActionDef` input is now asserted to carry non-empty `help` text, so
    the guidance line can't silently disappear from under a new field.
- [x] Preserve unknown/unrecognized action inputs when loading and saving
  existing workflows.
  - Undeclared input keys round-trip through the property layer unchanged.
  - **Fixed: flow-level `outputs` was silently wiped on every save.** `load`
    folds a definition down to *nodes*, so the flow-level `outputs` map had
    nowhere to live in `FlowState`; `save` hardcoded
    `outputs: Default::default()`. It is now captured in
    `workflow_json::FlowPreserved` and written back verbatim.
  - **Fixed: silent type retyping.** The property layer is stringly-typed, so
    `string_to_value` unconditionally re-parsed inspector text as JSON — a
    `FieldKind::String` field holding the literal text `true` round-tripped
    into the JSON boolean `true`, changing the file's type on every open/save
    cycle. `string_to_value_as` now coerces per the declared `FieldKind`:
    scalars stay literal text for `String`, while an object/array is kept
    structured (those same fields accept a JSONLogic expression, which the
    runtime only evaluates if it stays a non-string).
  - Action- and flow-level keys *outside* the schema are still dropped, which
    matches `flow.schema.json`'s deliberate `additionalProperties: false` —
    it exists to catch typo'd field names.
- [x] Add assisted JSONLogic editing with validation, examples, and
  autocomplete for action outputs.
  - Expression validation was already in `workflow_engine::validation` and is
    surfaced per field.
  - Added a reference picker under every `Expression` field: a clickable chip
    per upstream output, built from `compute_levels` over the node's scope plus
    each action's declared `ActionDef::outputs`. Clicking inserts the whole
    `{"var": "…"}` object **at the cursor**, so it composes with what's already
    typed. Only level-ordered predecessors are offered — a `var` against an
    action that may not have run yet resolves to null and silently changes
    behavior.
- [x] Add Script Inline/File mode controls and the editor/storage design proven
  in phase 1.
  - Already implemented (`ScriptRuntime` picker, the `.NET` inline guard, and
    path materialization); this item was stale rather than outstanding.
- [x] Make container bodies and If/Try branches understandable and editable
  without exposing synthetic branch wrappers as actions.
  - Selecting a branch wrapper now explains what it is, names the container
    that owns it, and counts the actions inside, instead of rendering a
    property editor whose values were discarded on save.
  - **Fixed: deleting a branch wrapper destroyed the whole branch body.** The
    handler deleted all descendants, and since a wrapper exists only to
    *display* `If`/`Try`'s nested `actions`/`else`/`catch` maps, there was no
    JSON to record the removal in — real saved actions were lost with no undo
    and no confirmation. Delete is now refused for a wrapper (button disabled
    with an explanation, handler guarded too, since the button isn't the only
    path to it), and the status line says what to delete instead.

### 5. Wire validation to canvas, inspector, and Run

- [x] Show actionable validation errors on their corresponding canvas nodes
  and in the inspector.
  - Nodes with a blocking error get a theme-driven danger ring
    (`FlowNode::validation_error`, deliberately a *separate* field from
    `accent_border` so run status and validation can't clear each other).
    The ring outranks both selection and run status, since it's the only one
    that blocks the user.
  - The toolbar badge is now a popover trigger listing **every** issue, errors
    first. This is the fix for the previous state, where the count was shown
    but the messages only rendered in the inspector for the selected node — so
    flow-level issues (`node_path == ""`: a `runAfter` cycle or dangling
    reference) matched no node and were displayed nowhere at all.
  - Clicking a row selects the node and centers the viewport on it. A
    flow-level row is rendered flat rather than clickable, since there's no
    node to jump to.
  - Container ancestors light up too, so an error buried in a body isn't
    invisible. `issue_targets_node` is unit-tested for top-level, nested,
    ancestor, middle-container, and flow-level cases.
- [x] Keep Save available for incomplete drafts.
- [x] Disable/block Run while blocking errors remain and explain what needs
  fixing.
- [x] Ensure unavailable action types cannot be added through alternate
  paths, or are reported as not runnable if loaded from an existing file.
  - `ListItem::disabled` turned out to be style-only — `ListState` still
    selects and confirms a disabled row on click/Enter — so the rule is
    enforced in `CatalogDelegate::addable_type_id`, which the pane, the drag
    payload, and the right-click menu all filter through.

### 6. HTTP request-body editor (companion file + JSON schemas)

`http`'s `body` is a `FieldKind::Json` input, so it gets the same treatment
`script`'s `source` already had: a real file in a real editor tab, rather than a
single-line inspector row.

- [x] Store each body in a companion file under `<flow>.flow-http/`, preserving
  the action's nesting scope as subdirectories, escaped via
  `script_companion::companion_component` so an id containing a separator or
  `..` cannot escape the directory.
  - `<flow>.flow-http/<scope>/<action_id>.body.json`; a `.json` suffix is what
    gives the file a language at all.
- [x] Open it in a workspace editor tab through a `ViewHttpBody` action, with
  an "Edit Body" button on `http` nodes in the inspector.
- [x] Two-way sync, mirroring `script_companion`'s rules: whichever side changed
  since the last sync wins, and *both* changed is a reported conflict rather
  than a silently discarded edit. A body without a `body` input gets no
  companion and none is invented for it.
  - Comparison is **semantic, not byte-wise** (`http_companion::canonical_body`):
    a user's editor reindenting the file, or reordering keys, must not read as a
    conflict. The canonicalizer is hand-rolled rather than `to_string_pretty`
    because the workspace enables `serde_json`'s `preserve_order`, so the
    built-in serializer would preserve key order and defeat the comparison.
  - A body that isn't valid JSON round-trips **verbatim** rather than being
    normalized or dropped, so a half-typed body is never lost.
- [x] Schema validation, two sources:
  - **Registry-derived, always on.** `registry::body_json_schema("http")` is
    generated from the `ActionDef`, so it cannot drift from the registry, and
    is served as `zed://schemas/flow_http_body` for the
    `**/*.flow-http/**/*.body.json` association. Its root is deliberately
    *unconstrained* — `FieldKind::Json` is an escape hatch that accepts any JSON
    value, so asserting an object shape would flag every legitimate array or
    scalar body. What the registry contributes is the field catalog (inputs,
    outputs, kinds, requiredness) as hover documentation, the same
    deliberate-permissiveness trade-off `flow.schema.json` documents for
    `inputs`.
  - **User-authored sidecar, when present.** Dropping
    `<action_id>.body.schema.json` next to the body validates it against your
    own schema, in addition to the above. Chosen over a `$schema` key inside the
    body because JSON LS would meta-validate the body *as a schema*, and the key
    would ship in the actual request payload. The association is only emitted
    when the file exists — a `url` the server can't load is reported as an
    error, so speculatively listing a sidecar nobody wrote would put an error in
    the editor for every body the user opens.
- [x] Wire the sync into load, `reload_from_disk`, Save, and Run, with a
  `body_sync` baseline separate from `script_sync` (different files, different
  key space). Hydrations are pushed back into the live canvas state and the
  inspector inputs, or the next sync would write the stale inline body over the
  user's external edit.

### 7. Verify the authoring workflow end to end

- [ ] Test catalog search, categories, click-to-add, drag-to-place, and
  right-click quick-add.
- [ ] Test typed input editing, draft save, validation display, and Run
  blocking.
- [ ] Test nested containers and If/Try branches through save/reopen.
- [ ] Test Inline and File scripts, mode switching, runtime selection,
  workspace boundaries, and LSP completion/diagnostics.
- [ ] Test companion-file visibility and save/reopen behavior if that fallback
  is selected.
- [ ] Test successful and failing execution paths for actions that have
  executors; clearly identify registered but unimplemented actions.
- [ ] Update this document with completed phases, deferred work, and any
  intentional changes to these requirements.
