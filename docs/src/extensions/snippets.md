---
title: Snippets
description: "Snippets for Zed extensions."
---

# Snippets

Extensions may provide snippets for one or more languages.

Each snippet can be specified in the `snippets` field of the `extensions.toml` file.

The path referencing the snippet must be relative to the `extensions.toml`.

## Defining Snippets

A given extension may provide one or more snippets. Each snippet must be registered in the `extension.toml`.

For example, here is an extension that provides two snippets for `rust` and `typescript`:

```toml
snippets = ["./snippets/rust.json", "./snippets/typescript.json"]
```

You can refer to the [Snippets](../snippets.md) page for writing them.
