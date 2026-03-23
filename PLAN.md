# DB Test Isolation Plan

## Problem

`static_connection!` creates a single shared `LazyLock` per DB type. All tests in a binary share one in-memory SQLite database. Tests that write to the DB can interfere with each other when running in parallel.

## Solution

Three pieces working together:

### 1. `AppDatabase` (new type in `db` crate)

- Wraps a `ThreadSafeConnection`, implements `Global`
- `AppDatabase::new()`: opens a fresh in-memory DB with a unique name, runs all inventory-registered migrations, returns `Self`
- `AppDatabase::global(cx: &App) -> &ThreadSafeConnection`: checks `cx.try_global::<AppDatabase>()` first, falls back to the existing `LazyLock`
- Production doesn't need to call anything — the `LazyLock` fallback handles it
- Tests call `cx.set_global(AppDatabase::new())` in `init_test` for isolation

### 2. `inventory` migration registration

- Each `static_connection!` invocation registers its `Domain` migrations via `inventory::submit!`
- `AppDatabase::new()` collects all registered migrations and runs them
- Migrations are already idempotent (`connection.migrate()` skips applied ones)

### 3. `static_connection!` macro changes

- Remove the `LazyLock` static (the fallback lives on `AppDatabase`, not per-type)
- Add `Clone` impl for wrapper type
- Add `fn global(cx: &App) -> Self` that clones from `AppDatabase::global(cx)`
- Register migrations via `inventory::submit!`

## Call site migration

Every `DB.foo()` becomes `WorkspaceDb::global(cx).foo()`. For async blocks:

```rust
// Before
window.spawn(cx, async move |_| {
    persistence::DB.save_workspace(x).await;
})

// After
let db = WorkspaceDb::global(cx);
window.spawn(cx, async move |_| {
    db.save_workspace(x).await;
})
```

Compiler-guided: removing the static import makes every old call site fail to compile.

## Test behavior

- Tests that call `cx.set_global(AppDatabase::new())` in their setup get a fresh isolated DB
- Tests that don't set it fall back to the shared `LazyLock` (today's behavior, unchanged)
- No test is broken by this change — isolation is opt-in

## Order of operations

1. Add `inventory` dependency to `db` crate
2. Add `AppDatabase` type with `new()` and `global(cx)`
3. Modify `static_connection!` macro: add `Clone`, `global(cx)`, inventory registration
4. Migrate call sites (`DB.foo()` → `Type::global(cx).foo()`) — compiler-guided
5. Add `cx.set_global(AppDatabase::new())` to workspace `init_test`
6. Workspace persistence tests get isolation
