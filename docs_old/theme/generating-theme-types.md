[⬅ Back to Index](../index.md)

# Generating Theme Types


## How to generate theme types:

Run a script

```bash
./script/build-theme-types
```

Types are generated in `styles/src/types/zed.ts`


## How it works:

1. Rust types

    The `crates/theme` contains theme types.
    Crate `schemars` used to generate a JSON schema from the theme structs.
    Every struct that represent theme type has a `#[derive(JsonSchema)]` attribute.

    Task lotaked at `crates/xtask/src/main.rs` generates a JSON schema from the theme structs.

2. TypeScript types

    Script `npm run build-types` from `styles` package generates TypeScript types from the JSON schema and saves them to `styles/src/types/zed.ts`.
