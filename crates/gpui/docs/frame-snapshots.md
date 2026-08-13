# Frame snapshots and the frame oracle

`FrameSnapshot` is a test-support-only description of a completed GPUI frame.
It preserves the semantic order of every captured lane:

- scene paint operations
- hitboxes
- focus path and dispatch nodes, including key contexts and actions
- cursor requests
- tab-stop operations
- deferred-draw metadata
- the accessibility tree update when accessibility is active

Snapshot comparison is exact. Floating-point values are included through their
IEEE-754 bit representation; the oracle does not apply an epsilon. A mismatch
report labels the lane and index of each reported divergence.

## Atlas canonicalization

Oracle windows use the same `TextSystem` and the same test sprite atlas. The
test platform allocates one atlas per `TestAppContext` and shares it among that
context's windows. Consequently, glyph, image, and SVG tiles have identical
coordinates when both engines request the same semantic atlas keys in the same
order. The snapshot intentionally preserves scene operation order.

## Adding storybook coverage

The storybook is constructed in `frame_oracle::tests::storybook_script`.

1. Add the element to `Storybook::render`.
2. If the element owns independently invalidatable state, add its `AnyView` to
   `FrameScriptUi::new`'s notification-target list.
3. Add one or more `FrameStep`s that exercise the element's state transition.
4. Run the storybook test with multiple scheduler seeds and add a compact
   property-test case when the behavior can be generated safely.

Every `FrameStep` is applied to independent windows. A time advance is applied
once to the shared deterministic test clock before both windows draw.
