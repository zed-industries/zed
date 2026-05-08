# `owned_string_into_shared` — findings on the Zed workspace

The lint flags any expression that allocates an owned `String` from a
string literal and immediately converts it with `.into()` into one of
`gpui::SharedString`, `Arc<str>`, `Rc<str>`, or `Cow<'_, str>`.

The detected shapes are:

* `String::from(<literal>).into()`
* `<literal>.to_string().into()`
* `<literal>.to_owned().into()`

## Run

The findings below were produced from the workspace root with:

```
RUSTFLAGS="-A async_block_without_await -A shared_string_from_str_literal -A entity_update_in_render -A notify_in_render" \
    cargo dylint --path tooling/lints -- --workspace
```

The other lints in the dylint library are silenced via `RUSTFLAGS` so
that the output contains only `owned_string_into_shared` diagnostics.

## Summary

| Crate                | Sites |
|----------------------|------:|
| `editor`             |     1 |
| `git_ui`             |     2 |
| `toolchain_selector` |     1 |
| **Total**            | **4** |

## Findings

### 1. `crates/editor/src/editor.rs:4995`

```crates/editor/src/editor.rs#L4992-4996
                edits.push((
                    emoji_shortcode_start..selection.start,
                    "".to_string().into(),
                ));
```

The literal is the empty string. The `edits` vector is consumed by
`MultiBuffer::edit`, whose second tuple element is a refcounted string
buffer; the `.into()` is converting an empty `String` into that
refcounted destination after first allocating the `String`.

### 2. `crates/toolchain_selector/src/toolchain_selector.rs:867`

```crates/toolchain_selector/src/toolchain_selector.rs#L867-L867
        let placeholder_text = "Select a toolchain…".to_string().into();
```

`placeholder_text` is then stored on the picker as a refcounted shared
string. The literal `"Select a toolchain…"` is `'static`, so the
intermediate `String` is pure overhead.

### 3. `crates/git_ui/src/worktree_picker.rs:708`

```crates/git_ui/src/worktree_picker.rs#L707-L712
                let item = create_new_list_item(
                    "create-from-current".to_string().into(),
                    label.into(),
                    self.creation_blocked_reason(cx),
                    selected,
                );
```

The first parameter of `create_new_list_item` is `id: SharedString`
(see `worktree_picker.rs:1003-1008`). `"create-from-current"` is a
`'static` literal, so the call materialises a `String` only to feed it
into `SharedString::from(String)`.

### 4. `crates/git_ui/src/worktree_picker.rs:722`

```crates/git_ui/src/worktree_picker.rs#L721-L726
                let item = create_new_list_item(
                    "create-from-main".to_string().into(),
                    label.into(),
                    self.creation_blocked_reason(cx),
                    selected,
                );
```

The same shape as finding 3, with the literal `"create-from-main"`.

## Notes on signal

* All four findings are direct conversions of `'static` string
  literals into a refcounted destination type. None require runtime
  data. They are concrete cases of the spec's stated cost: one
  allocation for the `String` plus a second for the refcounted buffer
  where one allocation (or none, for `Cow::Borrowed`) is sufficient.
* No false positives were observed in the workspace run. The lint
  declines to fire when the receiver of `.into()` is a non-literal
  `String`, when the destination is `Box<str>` or another type, or when
  the literal is converted directly without a `String` intermediate.
