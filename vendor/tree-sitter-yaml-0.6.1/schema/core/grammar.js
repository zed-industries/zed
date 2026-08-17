/** @see {@link Core Schema|https://yaml.org/spec/1.2.2/#103-core-schema} */

/// <reference types="tree-sitter-cli/dsl" />

module.exports = grammar({
  name: "core_schema",

  extras: _ => [],

  rules: {
    scalar: $ => choice($.null, $.bool, $.int, $.float),

    null: _ => token(choice("~", "null", "Null", "NULL")),

    bool: _ => token(choice("true", "True", "TRUE", "false", "False", "FALSE")),

    int: _ => token(choice(
      /[-+]?[0-9]+/, // base 10
      /0o[0-7]+/,  // base 8
      /0x[0-9a-fA-F]+/, // base 12
    )),

    float: _ => token(choice(
      /[-+]?(\.\d+|\d+(\.\d*)?)([eE][-+]?\d+)?/, // number
      seq(
        optional(choice("-", "+")),
        choice(".inf", ".Inf", ".INF")
      ),  // infinity
      choice(".nan", ".NaN", ".NAN"),  // not a number
    )),
  },
});
