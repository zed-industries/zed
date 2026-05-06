// Phase 2: GPUI-aware async context detection.
//
// This module will detect GPUI spawn closures (cx.spawn, cx.background_spawn,
// etc.) and Entity::update closures inside spawn, marking them as async
// contexts that should be checked for blocking calls.
//
// For Phase 1, async context detection is handled inline in the AST visitor
// in analyzer.rs (plain `async fn` detection only).
