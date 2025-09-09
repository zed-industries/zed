This crate contains Zed's builtin languages. Eventually these will be migrated to the extension system.

When updating the `*.scm` TreeSitter query files, the following can be helpful:

```sh
HOT_RELOAD_BUILTIN_TREE_SITTER_QUERIES=. cargo run
```

The implementation of this is fairly inefficient - on every file change it reloads all builtin languages and resets the language on all buffers.

This also works with release builds of Zed, and when run from some other directory it can be set to the path of the Zed repository that has the queries.
