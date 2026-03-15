# View Example — Plan

## Done

- Introduced `View` trait, `ComponentView` trait, and `ViewElement` struct as a unification of `Component`, `RenderOnce`, `AnyView`, and `Render`
- Initialized example of the composition this can achieve with the editor

## Next

- Add a render log showing coarse-grained caching (rather than existing spot-caching)
- Add tab index support so that the demo doesn't need mouse movement
- Move focus handles out to the input and textarea, and stop blinking when not focused
- De-fluff LLM generated code (remove excessive comments, simplify implementations, etc.)
