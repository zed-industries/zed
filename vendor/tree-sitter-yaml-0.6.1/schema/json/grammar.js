/** @see {@link JSON Schema|https://yaml.org/spec/1.2.2/#102-json-schema} */

/// <reference types="tree-sitter-cli/dsl" />

module.exports = grammar({
  name: "json_schema",

  extras: _ => [],

  rules: {
    scalar: $ => choice($.null, $.bool, $.int, $.float),

    null: _ => token("null"),

    bool: _ => token(choice("true", "false")),

    int: _ => token(/-?(0|[1-9][0-9]*)/),

    float: _ => token(/-?(0|[1-9][0-9]*)(\.[0-9]*)?([eE][-+]?[0-9]+)?/),
  },
});
