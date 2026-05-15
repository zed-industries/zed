# lints

A [dylint](https://github.com/trailofbits/dylint) library that flags various bad patterns in our codebase.

## Current lints
- `shared_string_from_str_literal` — `SharedString::new/from` etc where `SharedString::from_static` should be used instead.
- `async_block_without_await` — `async { … }` blocks whose body contains no `.await` expression.
- `entity_update_in_render` — `Entity::update`/`WeakEntity::update` mutating an entity inside `Render::render`.
- `notify_in_render` — `Context::notify()` called inside `Render::render`.
- `owned_string_into_shared` — `String::from(<lit>).into()` / `<lit>.to_string().into()` / `<lit>.to_owned().into()` whose target is `SharedString`, `Arc<str>`, `Rc<str>`, or `Cow<'_, str>`.
- `wrapped_doc_comment` — `///` doc-comment paragraphs wrapped across multiple physical lines. Tool description doc comments are serialized verbatim into JSON tool schemas sent to language models; mid-paragraph line breaks add no semantic value and waste tokens.
- `blocking_io_on_foreground` - Catch blocking IO calls that are called on the main thread (but not on closures or background threads)

## How to run
Ideally you run this as part of the `clippy` script in the  `zed/scripts` directory since this will also run our other linters.

If you want to run only the dylints use:

```
cargo dylint --path tooling/lints -- --workspace
```

TODO!(yara) make it so the tooling is installed and the script is a litte more forgiving about missing tools.

### How to run on a single crate

```
cargo dylint --path tooling/lints -- -p project_panel
```

## Adding more lints with AI
All issues highlighted go through manual review therefore we feel comfortable vibe coding the lints. At worst we miss cases and have false positives.

This is how you can optimally prompt Claude 4.6 as of may 2026 to add a lint:

> We're trying to add another dylint lint to the `lint` crate in `zed/tooling` to catch <description of problem cases>.
>
> Come up with a plan for implementing this and unit tests for detecting it using `@LintRULES.md`.

### Examples of <description of problem cases>:
> async blocks without any `.await` in them


> `blocking_io_on_foreground` - Blocking IO calls that are called on the main thread (but not on closures or background threads)

Prompt 1:
```
Using `@LintRULES.md` , suggest a lint for catching blocking IO function calls that are called on the main thread, i.e., that are not called from background threads.

One example of this is this function [@telemetry.rs (127:142)](file:///Users/mrg/agents/zed-shared-string-lint/crates/client/src/telemetry.rs#L127:142)
```

Prompt 2:

```
Run ONLY that lint against the Zed codebase
```

Result:

```
Here are the two hits:

**1. `crates/snippets_ui/src/snippets_ui.rs:96`** — `fs::create_dir_all(snippets_dir())` called inside `open_folder`, which takes `cx: &mut Context<Workspace>`. This creates a directory synchronously on the UI thread.

*zed-shared-string-lint/crates/snippets_ui/src/snippets_ui.rs#L90-98:*

```rust
fn open_folder(
    workspace: &mut Workspace,
    _: &OpenFolder,
    _: &mut Window,
    cx: &mut Context<Workspace>,
) {
    fs::create_dir_all(snippets_dir()).notify_err(workspace, cx);
    cx.open_with_system(snippets_dir().borrow());
}
```

**2. `crates/zed/src/main.rs:1808`** — `std::fs::canonicalize(Path::new(&arg))` called inside `parse_url_arg`, which takes `cx: &App`. This resolves symlinks synchronously on the UI thread.

*zed-shared-string-lint/crates/zed/src/main.rs#L1807-1821*:
```rust
fn parse_url_arg(arg: &str, cx: &App) -> String {
    match std::fs::canonicalize(Path::new(&arg)) {
        Ok(path) => format!("file://{}", path.display()),
        Err(_) => {
            if arg.starts_with("file://")
                || arg.starts_with("zed://")
                || arg.starts_with("zed-cli://")
                || arg.starts_with("ssh://")
                || parse_zed_link(arg, cx).is_some()
            {
                arg.into()
            } else {
                format!("file://{arg}")
            }
```

Both are real foreground-blocking IO calls. The hit count is low (2), which suggests the codebase is generally disciplined about this, and the lint has a low false-positive rate.
```
