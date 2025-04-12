1. The `Canvas` struct now includes a `theme` field of type `Theme`. The `new` method for `Canvas` has been updated to accept a `theme` parameter and initialize the field with it. The `Theme` struct has been marked as `#[derive(Debug, Clone)]`.
2. The `paint_draw_rectangle` method has been simplified to take a `NodeId` directly instead of a `usize`. It now reads the canvas and app state separately to avoid borrow issues. The method no longer requires a `GlobalState` parameter.
3. The `paint_nodes` method has been significantly refactored to:
   - Collect all rendering data upfront in a `NodeRenderInfo` struct
   - Handle node transformations and bounds more efficiently
   - Use the canvas's theme directly instead of the global theme
   - Simplify the rendering loop by pre-computing all necessary data
4. The `Element` implementation for `CanvasElement` has been updated to:
   - Remove commented-out key context code
   - Simplify the painting logic by reading canvas data once
   - Use the canvas's theme directly
   - Streamline the active element drawing logic
5. The `Luna` struct's initialization now creates a `Theme` instance and passes it to the `Canvas` constructor.
6. The `Theme` struct has been marked with `#[derive(Debug, Clone)]` to support cloning, which is needed for storing it in the `Canvas` struct.
