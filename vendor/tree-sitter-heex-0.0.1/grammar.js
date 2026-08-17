module.exports = grammar({
  name: "heex",

  rules: {
    fragment: ($) => repeat($._node),

    _node: ($) =>
      choice($.doctype, $.tag, $.component, $.text, $.comment, $.directive),

    doctype: ($) => seq("<!", "DOCTYPE", "html", ">"),

    tag: ($) =>
      choice(seq($.start_tag, repeat($._node), $.end_tag), $.self_closing_tag),

    component: ($) =>
      choice(
        seq(
          $.start_component,
          repeat(choice($._node, $.slot)),
          $.end_component
        ),
        $.self_closing_component
      ),

    slot: ($) =>
      choice(
        seq($.start_slot, repeat($._node), $.end_slot),
        $.self_closing_slot
      ),

    start_tag: ($) =>
      seq(
        "<",
        $.tag_name,
        repeat(choice($.attribute, $.expression, $.special_attribute)),
        ">"
      ),

    end_tag: ($) => seq("</", $.tag_name, ">"),

    self_closing_tag: ($) =>
      seq(
        "<",
        $.tag_name,
        repeat(choice($.attribute, $.expression, $.special_attribute)),
        "/>"
      ),

    start_component: ($) =>
      seq(
        "<",
        $.component_name,
        repeat(choice($.attribute, $.expression, $.special_attribute)),
        ">"
      ),

    end_component: ($) => seq("</", $.component_name, ">"),

    self_closing_component: ($) =>
      seq(
        "<",
        $.component_name,
        repeat(choice($.attribute, $.expression, $.special_attribute)),
        "/>"
      ),

    start_slot: ($) =>
      seq(
        "<:",
        alias($.tag_name, $.slot_name),
        repeat(choice($.attribute, $.expression, $.special_attribute)),
        ">"
      ),

    end_slot: ($) => seq("</:", alias($.tag_name, $.slot_name), ">"),

    self_closing_slot: ($) =>
      seq(
        "<:",
        alias($.tag_name, $.slot_name),
        repeat(choice($.attribute, $.expression, $.special_attribute)),
        "/>"
      ),

    expression: ($) =>
      seq(
        "{",
        prec.left(
          seq(alias(repeat($._expression_value), $.expression_value), "}")
        )
      ),

    _expression_value: ($) =>
      choice(/[^{}]+/, seq("{", optional($._expression_value), "}")),

    special_attribute: ($) => seq($.special_attribute_name, "=", $.expression),

    special_attribute_name: ($) => choice(":let", ":for", ":stream", ":if"),

    attribute: ($) =>
      seq(
        $.attribute_name,
        optional(
          seq(
            "=",
            choice($.quoted_attribute_value, $.attribute_value, $.expression)
          )
        )
      ),

    attribute_value: ($) => /[^<>{}"'=\s]+/,

    quoted_attribute_value: ($) =>
      choice(
        seq("'", optional(alias(/[^']+/, $.attribute_value)), "'"),
        seq('"', optional(alias(/[^"]+/, $.attribute_value)), '"')
      ),

    directive: ($) =>
      seq(
        choice("<%", "<%=", "<%%", "<%%="),
        prec.left(
          seq(
            choice(
              $.partial_expression_value,
              $.ending_expression_value,
              $.expression_value
            ),
            "%>"
          )
        )
      ),

    comment: ($) => choice($._html_comment, $._bang_comment, $._hash_comment),

    _html_comment: ($) => seq("<!--", prec.left(seq(repeat(/[^-]+|-/), "-->"))),

    _bang_comment: ($) =>
      seq("<%!--", prec.left(seq(repeat(/[^-]+|-/), "--%>"))),

    _hash_comment: ($) => seq("<%#", prec.left(seq(repeat($._code), "%>"))),

    expression_value: ($) => repeat1($._code),

    partial_expression_value: ($) =>
      seq(
        repeat($._code),
        choice("do", "->"),
        optional(seq("#", repeat($._code)))
      ),

    ending_expression_value: ($) => seq(/end[\)\]\}]*/, repeat($._code)),

    component_name: ($) =>
      choice(seq(optional($.module), seq(".", $.function)), $.module),

    module: ($) => /([A-Z][^\-<>{}!"'/=\s\.]*)(\.[A-Z][^\-<>{}!"'/=\s\.]*)*/,

    function: ($) => /[a-z][^\-<>{}!"'/=\s\.]*/,

    _code: ($) => /[^%\s]+|[%\s]/,

    tag_name: ($) => /[a-z]+[^<>{}!"'/=\s]*/,

    attribute_name: ($) => token(prec(-1, /[^:<>{}"'/=\s][^<>{}"'/=\s]*/)),

    text: ($) => /[^<>{}\s]([^<>{}]*[^<>{}\s])?/,
  },
});
