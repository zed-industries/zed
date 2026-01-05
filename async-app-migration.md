# Project: AsyncApp Result Removal - Codebase Migration

## Overview

Phase 1-3 of the AsyncApp Result removal project are complete. The GPUI framework changes are in place:
- Executor trampoline checks ensure foreground tasks only run when app is alive
- `AppContext` trait no longer has `type Result<T>` associated type
- `AsyncApp` methods return values directly (panicking if app gone)
- `Flatten` trait has been removed

This brief covers the codebase-wide migration to update all callsites.

## Scope

**Estimated callsites:** ~500+ across the codebase

**Crates affected:** All crates that use `AsyncApp`, `AsyncWindowContext`, or implement `AppContext`

## Breaking Changes

### 1. `AppContext` Trait Signature Change

```rust
// Before
pub trait AppContext {
    type Result<T>;
    fn new<T>(...) -> Self::Result<Entity<T>>;
    fn update_entity<T, R>(...) -> Self::Result<R>;
    // etc.
}

// After
pub trait AppContext {
    fn new<T>(...) -> Entity<T>;
    fn update_entity<T, R>(...) -> R;
    // etc.
}
```

**Action required:** Any crate implementing `AppContext` must update its impl.

**Known implementors:**
- `crates/eval/src/example.rs` - `ExampleContext`

### 2. `AsyncApp` Method Return Types

All `AsyncApp` methods that previously returned `Result<T>` now return `T` directly:

| Method | Before | After |
|--------|--------|-------|
| `update()` | `Result<R>` | `R` |
| `read_entity()` | `Result<R>` | `R` |
| `update_entity()` | `Result<R>` | `R` |
| `read_global()` | `Result<R>` | `R` |
| `update_global()` | `Result<R>` | `R` |
| `new()` | `Result<Entity<T>>` | `Entity<T>` |
| `reserve_entity()` | `Result<Reservation<T>>` | `Reservation<T>` |
| `insert_entity()` | `Result<Entity<T>>` | `Entity<T>` |
| `has_global()` | `Result<bool>` | `bool` |
| `refresh()` | `Result<()>` | `()` |
| `open_window()` | `Result<WindowHandle<V>>` | `WindowHandle<V>` |
| `subscribe()` | `Result<Subscription>` | `Subscription` |

**Note:** `read_window()` and `update_window()` still return `Result<T>` because windows can be closed independently.

## Transformation Patterns

### Pattern 1: Remove trailing `?` (Most Common)

```rust
// Before
let result = this.update(cx, |this, cx| { ... })?;

// After
let result = this.update(cx, |this, cx| { ... });
```

**ast-grep rule:**
```yaml
id: remove-update-question-mark
language: rust
rule:
  pattern: $ENTITY.update($CX, $$$CLOSURE)?
fix: $ENTITY.update($CX, $$$CLOSURE)
```

### Pattern 2: Remove `.unwrap()` calls

```rust
// Before
let result = cx.new(|cx| MyStruct::new(cx)).unwrap();

// After
let result = cx.new(|cx| MyStruct::new(cx));
```

### Pattern 3: Remove `?` from `read_with` calls

```rust
// Before
let value = this.read_with(cx, |this, _| this.some_field.clone())?;

// After
let value = this.read_with(cx, |this, _| this.some_field.clone());
```

### Pattern 4: Double `??` to single `?`

This occurs when an async closure returns `Result<T>` and the `update()` call also returned `Result`:

```rust
// Before
this.update(cx, |this, cx| {
    // ... code that returns Result<T>
    anyhow::Ok(())
})??;

// After
this.update(cx, |this, cx| {
    // ... code that returns Result<T>
    anyhow::Ok(())
})?;
```

### Pattern 5: Chained `?.await?` to `.await?`

```rust
// Before
let transaction = buffer.update(cx, |buffer, cx| buffer.reload(cx))?.await?;

// After
let transaction = buffer.update(cx, |buffer, cx| buffer.reload(cx)).await?;
```

### Pattern 6: `.ok()?` patterns (Requires Manual Review)

```rust
// Before - the .ok() might be intentional for control flow
let t = this.update(cx, |this, cx| this.process(&p, cx)).ok()?;

// After - depends on context
// Option A: If .ok() was just to suppress the Result
let t = Some(this.update(cx, |this, cx| this.process(&p, cx)))?;

// Option B: If the closure itself returns Option and .ok()? was for short-circuiting
let t = this.update(cx, |this, cx| this.process(&p, cx))?;
```

### Pattern 7: `cx.new()` in async contexts

```rust
// Before
let buffer = cx.new(|cx| Buffer::local("", cx))?;

// After
let buffer = cx.new(|cx| Buffer::local("", cx));
```

### Pattern 8: `reserve_entity` / `insert_entity`

```rust
// Before
let reservation = cx.reserve_entity::<Buffer>()?;
let entity = cx.insert_entity(reservation, |_| buffer)?;

// After
let reservation = cx.reserve_entity::<Buffer>();
let entity = cx.insert_entity(reservation, |_| buffer);
```

## ast-grep Rules

Create these rules in a `rules/` directory:

### `rules/remove-update-question-mark.yml`
```yaml
id: remove-update-question-mark
language: rust
rule:
  any:
    - pattern: $E.update($CX, |$$$PARAMS| $BODY)?
    - pattern: $E.update($CX, |$$$PARAMS| { $$$BODY })?
fix: $E.update($CX, |$$$PARAMS| { $$$BODY })
```

### `rules/remove-read-with-question-mark.yml`
```yaml
id: remove-read-with-question-mark
language: rust
rule:
  any:
    - pattern: $E.read_with($CX, |$$$PARAMS| $BODY)?
    - pattern: $E.read_with($CX, |$$$PARAMS| { $$$BODY })?
fix: $E.read_with($CX, |$$$PARAMS| { $$$BODY })
```

### `rules/remove-cx-new-question-mark.yml`
```yaml
id: remove-cx-new-question-mark
language: rust
rule:
  pattern: $CX.new(|$$$PARAMS| $BODY)?
fix: $CX.new(|$$$PARAMS| $BODY)
```

### `rules/double-question-to-single.yml`
```yaml
id: double-question-to-single
language: rust
rule:
  pattern: $EXPR)??
fix: $EXPR)?
```

## Migration Strategy

### Phase 1: Update External AppContext Implementations

1. **`crates/eval/src/example.rs`** - Update `ExampleContext`:
   - Remove `type Result<T> = anyhow::Result<T>;`
   - Update all method signatures to return `T` directly instead of `Self::Result<T>`
   - The inner calls to `self.app.*` already return the correct types

### Phase 2: Automated Transformations

Run ast-grep rules iteratively:

```bash
# Install ast-grep if needed
cargo install ast-grep

# Run rules (dry-run first)
ast-grep scan -r rules/remove-update-question-mark.yml
ast-grep scan -r rules/remove-read-with-question-mark.yml
ast-grep scan -r rules/remove-cx-new-question-mark.yml

# Apply changes
ast-grep scan -r rules/remove-update-question-mark.yml --update-all
```

### Phase 3: Compile-Driven Fixes

After automated transforms:

```bash
cargo check 2>&1 | head -100
```

Common remaining errors and fixes:

| Error Pattern | Fix |
|--------------|-----|
| `expected X, found Result<X>` | Remove `?` from the call |
| `the ? operator can only be used on Result` | The call no longer returns Result, remove `?` |
| `cannot use the ? operator in a function that returns ()` | Remove `?`, call now returns `()` directly |
| `type mismatch... Result<Result<X>>` | Change `??` to `?` |

### Phase 4: Manual Review

Some patterns require human judgment:

1. **`.ok()` patterns** - Determine if `.ok()` was for error suppression or control flow
2. **`.log_err()` patterns** - Many of these are on other `Result` types, not `AsyncApp`
3. **Return type changes** - Functions returning `Result<T>` may need signature updates if the only error source was `AsyncApp`

## Crate Priority Order

Based on dependency graph, migrate in this order:

1. **gpui** - ✅ Already done
2. **eval** - Has `ExampleContext` AppContext impl
3. **project** - Heavy async usage
4. **editor** - Heavy async usage
5. **assistant** - Heavy async usage
6. **workspace** - Heavy async usage
7. **collab** - Heavy async usage
8. **zed** - Main application crate
9. **Other crates** - Alphabetically or by compile errors

## Testing Strategy

After each batch of changes:

```bash
# Quick check
cargo check -p <crate>

# Full test
cargo test -p <crate>

# Before final merge
./script/clippy
cargo test --workspace
```

## Rollback Plan

All changes are mechanical and git-reversible:

```bash
git checkout -- <files>
```

If issues are found post-merge:
1. The executor changes are independent of callsite changes
2. Can revert callsite changes while keeping executor improvements
3. Can add `try_*` variants to `AsyncApp` if panics are found in practice

## Success Criteria

1. `cargo check --workspace` passes
2. `cargo test --workspace` passes
3. `./script/clippy` passes
4. No remaining uses of:
   - `.update(cx, ...)?.` (except on `WeakEntity` or window operations)
   - `.read_with(cx, ...)?` (except on `WeakEntity`)
   - `cx.new(...)?` in async contexts
   - `??` double unwrap on update calls

## Notes

### Window Operations Still Return Result

`read_window()` and `update_window()` still return `Result<T>` because windows can be closed independently of the app being alive. These `?` operators should NOT be removed.

### WeakEntity Still Returns Result

`WeakEntity::update()`, `WeakEntity::read_with()`, etc. still return `Result<T>` because the entity may have been dropped. These `?` operators should NOT be removed.

### Background Tasks Awaiting Foreground Tasks

If a background task awaits a foreground task and the app dies, the foreground task is cancelled and the await will panic. If this becomes a problem in practice, we can add `try_*` variants to `AsyncApp`. For now, this is considered acceptable since:
1. Background tasks shouldn't typically await foreground tasks
2. If the app is gone, the background task's work is likely irrelevant anyway