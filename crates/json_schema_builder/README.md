# JSON Schema Builder

Generates a `keymap.schema.json` for linting Zed's `keymap.json` file.

## Keymap.schema.json

1. From root run the following command.

```sh
$ cargo run -p json-schema-builder --bin keymap -- ./crates
```

2. It'll update a key in `schemas/keymap.schema.json`. Specifically the key `$defs.action.oneOf[0].enum`.
