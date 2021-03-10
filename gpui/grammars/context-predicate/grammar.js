module.exports = grammar({
  name: 'context_predicate',

  rules: {
    source: $ => $._expression,

    _expression: $ => choice(
      $.identifier,
      $.not,
      $.and,
      $.or,
      $.equal,
      $.not_equal,
      $.parenthesized,
    ),

    identifier: $ => /[A-Za-z0-9_-]+/,

    not: $ => prec(3, seq("!", field("expression", $._expression))),

    and: $ => prec.left(2, seq(field("left", $._expression), "&&", field("right", $._expression))),

    or: $ => prec.left(1, seq(field("left", $._expression), "||", field("right", $._expression))),

    equal: $ => seq(field("left", $.identifier), "==", field("right", $.identifier)),

    not_equal: $ => seq(field("left", $.identifier), "!=", field("right", $.identifier)),

    parenthesized: $ => seq("(", field("expression", $._expression), ")"),
  }
});
