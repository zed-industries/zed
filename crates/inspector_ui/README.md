# Inspector

This is a tool for inspecting and manipulating rendered elements in Zed. It is
only available in debug builds. Use the `dev::ToggleInspector` action to toggle
inspector mode and click on UI elements to inspect them.

# Current features

* Picking of elements via the mouse, with scroll wheel to inspect occluded elements.

* Temporary manipulation of the selected element.

* Layout info and JSON-based style manipulation for `Div`.

* Navigation to code that constructed the element.

# Known bugs

* The style inspector buffer will leak memory over time due to building up
history on each change of inspected element. Instead of using `Project` to
create it, should just directly build the `Buffer` and `File` each time the inspected element changes.

# Future features

* Info and manipulation of element types other than `Div`.

* Ability to highlight current element after it's been picked.

* Indicate when the picked element has disappeared.

* Hierarchy view?

## Better manipulation than JSON

The current approach is not easy to move back to the code. Possibilities:

* Editable list of style attributes to apply.

* Rust buffer of code that does a very lenient parse to get the style attributes. Some options:

  - Take all the identifier-like tokens and use them if they are the name of an attribute. A custom completion provider in a buffer could be used.

  - Use TreeSitter to parse out the fluent style method chain. With this approach the buffer could even be the actual code file. Tricky part of this is LSP - ideally the LSP already being used by the developer's Zed would be used.

## Source locations

* Mode to navigate to source code on every element change while picking.

* Tracking of more source locations - currently the source location is often in a ui compoenent. Ideally this would have a way for the components to indicate that they are probably not the source location the user is looking for.

## Persistent modification

Currently, element modifications disappear when picker mode is started. Handling this well is tricky. Potential features:

* Support modifying multiple elements at once. This requires a way to specify which elements are modified - possibly wildcards in a match of the `InspectorElementId` path. This might default to ignoring all numeric parts and just matching on the names.

* Show a list of active modifications in the UI.

* Support for modifications being partial overrides instead of snapshots. A trickiness here is that multiple modifications may apply to the same element.

* The code should probably distinguish the data that is provided by the element and the modifications from the inspector. Currently these are conflated in element states.

# Code cleanups

## Remove special side pane rendering

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
