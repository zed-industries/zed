---
title: Standard ML
description: "Configure Standard ML language support in Zed, including language servers, formatting, and debugging."
---

# Standard ML

Standard ML support is available through the community-maintained [Standard ML extension](https://github.com/omarjatoi/zed-sml).

- Tree-sitter: [MatthewFluet/tree-sitter-sml](https://github.com/MatthewFluet/tree-sitter-sml)
- Language Server: [Millet](https://github.com/azdavis/millet)

## Setup

1. Install a Standard ML implementation such as [SML/NJ](https://www.smlnj.org/) or [MLton](http://mlton.org/) to compile and run your code.
2. [Install Millet](https://github.com/azdavis/millet#install) and ensure `millet-ls` is on your `$PATH`.

## Project setup

For projects with more than one source file, Millet expects a single root group file. Create a `millet.toml` in the directory you open in Zed:

```toml
version = 1
[workspace]
root = "sources.mlb"
```

The root must be either a [ML Basis (MLB)](http://mlton.org/MLBasis) file (`.mlb`, used with MLton) or a [SML/NJ Compilation Manager (CM)](https://www.smlnj.org/doc/CM/new.pdf) file (`.cm`, used with SML/NJ). Files not transitively reachable from the root are not analyzed. See the [Millet manual](https://github.com/azdavis/millet/blob/main/docs/manual.md) for more options.
