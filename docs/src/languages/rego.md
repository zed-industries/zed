# Rego

Rego language support in Zed is provided by the community-maintained [Rego extension](https://github.com/StyraInc/zed-rego).

- Tree-sitter: [FallenAngel97/tree-sitter-rego](https://github.com/FallenAngel97/tree-sitter-rego)
- Language Server: [StyraInc/regal](https://github.com/StyraInc/regal)

## Installation

The extension is largely based on the [Regal](https://docs.styra.com/regal/language-server) language server which should be installed to make use of the extension. Read the [getting started](https://docs.styra.com/regal#getting-started) instructions for more information.

## Configuration

The extension's behavior is configured in the `.regal/config.yaml` file. The following is an example configuration which disables the `todo-comment` rule, customizes the `line-length` rule, and ignores test files for the `opa-fmt` rule:

```yaml
rules:
  style:
    todo-comment:
      # don't report on todo comments
      level: ignore
    line-length:
      # custom rule configuration
      max-line-length: 100
      # warn on too long lines, but don't fail
      level: warning
    opa-fmt:
      # not needed as error is the default, but
      # being explicit won't hurt
      level: error
      # files can be ignored for any individual rule
      # in this example, test files are ignored
      ignore:
        files:
          - "*_test.rego"
```

Read Regal's [configuration documentation](https://docs.styra.com/regal#configuration) for more information.
