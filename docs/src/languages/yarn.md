# Yarn

[Yarn](https://yarnpkg.com/) is a versatile package manager that improves dependency management and workflow efficiency for JavaScript and other languages. It ensures a deterministic dependency tree, offers offline support, and enhances security for reliable builds.

## Setup

1. Run `yarn dlx @yarnpkg/sdks base` to generate a `.yarn/sdks` directory.
2. Set your language server (e.g. VTSLS) to use TypeScript SDK from `.yarn/sdks/typescript/lib` directory in [LSP initialization options](../reference/all-settings.md#lsp). The actual setting for that depends on language server; for example, for VTSLS you should set [`typescript.tsdk`](https://github.com/yioneko/vtsls/blob/6adfb5d3889ad4b82c5e238446b27ae3ee1e3767/packages/service/configuration.schema.json#L5).
3. Voilla! Language server functionalities such as Go to Definition, Code Completions and On Hover documentation should work.
