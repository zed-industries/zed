# Inspector

This is a tool for inspecting and manipulating rendered elements in Zed. It is only available in debug builds. Use the `dev::ToggleInspector` action to toggle inspector mode and click on UI elements to inspect them.

# Current features

* Picking of elements via the mouse, with scroll wheel to inspect occluded elements.

* Temporary manipulation of the selected element.

* Layout info for `Div`.

* Both Rust and JSON-based style manipulation of `Div` style. The rust style editor only supports argumentless `Styled` and `StyledExt` method calls.

* Navigation to code that constructed the element.

# Known bugs

## JSON style editor undo history doesn't get reset

The JSON style editor appends to its undo stack on every change of the active inspected element.

I attempted to fix it by creating a new buffer and setting the buffer associated with the `json_style_buffer` entity. Unfortunately this doesn't work because the language server uses the `version: clock::Global` to figure out the changes, so would need some way to start the new buffer's text at that version.

```
        json_style_buffer.update(cx, |json_style_buffer, cx| {
            let language = json_style_buffer.language().cloned();
            let file = json_style_buffer.file().cloned();

            *json_style_buffer = Buffer::local("", cx);

            json_style_buffer.set_language(language, cx);
            if let Some(file) = file {
                json_style_buffer.file_updated(file, cx);
            }
        });
```

# Future features

* Action and keybinding for entering pick mode.

* Ability to highlight current element after it's been picked.

* Info and manipulation of element types other than `Div`.

* Indicate when the picked element has disappeared.

* To inspect elements that disappear, it would be helpful to be able to pause the UI.

* Hierarchy view?

## Methods that take arguments in Rust style editor

Could use TreeSitter to parse out the fluent style method chain and arguments. Tricky part of this is completions - ideally the Rust Analyzer already being used by the developer's Zed would be used.

## Edit original code in Rust style editor

Two approaches:

1. Open an excerpt of the original file.

2. Communicate with the Zed process that has the repo open - it would send the code for the element. This seems like a lot of work, but would be very nice for rapid development, and it would allow use of rust analyzer.

With both approaches, would need to record the buffer version and use that when referring to source locations, since editing elements can cause code layout shift.

## Source location UI improvements

* Mode to navigate to source code on every element change while picking.

* Tracking of more source locations - currently the source location is often in a ui component. Ideally this would have a way for the components to indicate that they are probably not the source location the user is looking for.

  - Could have `InspectorElementId` be `Vec<(ElementId, Option<Location>)>`, but if there are multiple code paths that construct the same element this would cause them to be considered different.

  - Probably better to have a separate `Vec<Option<Location>>` that uses the same indices as `GlobalElementId`.

## Persistent modification

Currently, element modifications disappear when picker mode is started. Handling this well is tricky. Potential features:

* Support modifying multiple elements at once. This requires a way to specify which elements are modified - possibly wildcards in a match of the `InspectorElementId` path. This might default to ignoring all numeric parts and just matching on the names.

* Show a list of active modifications in the UI.

* Support for modifications being partial overrides instead of snapshots. A trickiness here is that multiple modifications may apply to the same element.

* The code should probably distinguish the data that is provided by the element and the modifications from the inspector. Currently these are conflated in element states.

If support is added for editing original code, then the logical selector in this case would be just matches of the source path.

# Code cleanups

## Consider removing special side pane rendering

Currently the inspector has special rendering in the UI, but maybe it could just be a workspace item.

## Pull more inspector logic out of GPUI

Currently `crates/gpui/inspector.rs` and `crates/inspector_ui/inspector.rs` are quite entangled.  It seems cleaner to pull as much logic a possible out of GPUI.

## Cleaner lifecycle for inspector state viewers / editors

Currently element state inspectors are just called on render. Ideally instead they would be implementors of some trait like:

```
trait StateInspector: Render {
    fn new(cx: &mut App) -> Task<Self>;
    fn element_changed(inspector_id: &InspectorElementId, window: &mut Window, cx: &mut App);
}
```

See `div_inspector.rs` - it needs to initialize itself, keep track of its own loading state, and keep track of the last inspected ID in its render function.
