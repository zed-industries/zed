# lints

A [dylint](https://github.com/trailofbits/dylint) library that flags various bad patterns in our codebase.

## Current lints
- `shared_string_from_str_literal` — `SharedString::new/from` etc where `SharedString::from_static` should be used instead.
- `async_block_without_await` — `async { … }` blocks whose body contains no `.await` expression.
- `entity_update_in_render` — `Entity::update`/`WeakEntity::update` mutating an entity inside `Render::render`.
- `notify_in_render` — `Context::notify()` called inside `Render::render`.
- `owned_string_into_shared` — `String::from(<lit>).into()` / `<lit>.to_string().into()` / `<lit>.to_owned().into()` whose target is `SharedString`, `Arc<str>`, `Rc<str>`, or `Cow<'_, str>`.

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
All issues highlighed go through manual review therefore we feel comfortable vibe coding the lints. At worst we miss cases and have false positives.

This is how you can optimally prompt Claude 4.6 as of may 2026 to add a lint:

> We're trying to add another dylint lint to the `lint` crate in `zed/tooling` to catch <description of problem cases>.
>
> Come up with a plan for implementing this and unit tests for detecting it using `@LintRULES.md`.

### Examples of <description of problem cases>:
> async blocks without any `.await` in them

TODO!(raz)
- setting state as you render, specifically (the `set_text` on the editor there)
https://github.com/zed-industries/zed/blob/fb3218e01e22d5dcc2791fd6b94d22cf37d8e42f/crates/settings_ui/src/components/input_field.rs#L188-L208
