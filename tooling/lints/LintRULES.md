# Lint RULES

1. Every lint MUST have accompanying `ui` tests
2. `ui` tests MUST be in the `ui` folder
3. Every lint MUST be in a separate module
4. Every lint MUST have negative `ui` tests
5. Lints should be as simple as possible.
6. Reporting is fine if it's simple, it does not need to be elaborate or lengthy code.
7. Do NOT suggest how to fix the lint, only flag it.
8. Do NOT make lints machine applicable.
9. You can exclude running specific lints by putting them into the `RUSTFLAGS` env var, like so:

```
RUSTFLAGS="-A entity_update_in_render" cargo dylint --path tooling/lints -- --manifest-path /Users/$USER_NAME/OTHER_PROJECT/Cargo.toml --workspace
```
10. Also use the above command to run `lints` on different repos
