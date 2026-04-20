---
title: Yarn
description: "Configure Yarn language support in Zed, including language servers, formatting, and debugging."
---

# Yarn

[Yarn](https://yarnpkg.com/) is a JavaScript package manager that provides deterministic dependency resolution and offline caching.

## Setup

1. Run `yarn dlx @yarnpkg/sdks base` to generate a `.yarn/sdks` directory.
2. Set your language server (e.g. VTSLS) to use TypeScript SDK from `.yarn/sdks/typescript/lib` directory in [LSP initialization options](../reference/all-settings.md#lsp). The actual setting depends on your language server; for example, for VTSLS set [`typescript.tsdk`](https://github.com/yioneko/vtsls/blob/6adfb5d3889ad4b82c5e238446b27ae3ee1e3767/packages/service/configuration.schema.json#L5).

After configuration, language server features (Go to Definition, completions, hover documentation) should work correctly.
