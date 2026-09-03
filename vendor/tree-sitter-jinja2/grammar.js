function sep1(rule, separator) {
  return seq(rule, repeat(seq(separator, rule)));
}

function statement_start() {
  return alias(/\{\%[\+\-]?/, "statement_start");
}

function statement_end() {
  return alias(/[\+\-]?\%\}/, "statement_end");
}

/**
 * Matches something like `{% <kw> ...rest %}`
 */
function statement(kw, ...rest) {
  return seq(statement_start(), keyword(kw), ...rest, statement_end());
}

function expression_in_statement($) {
  return alias($._expression_in_statement, $.expression);
}

function keyword(kw) {
  return alias(token(prec(1, kw)), "_keyword");
}

function context_specifier() {
  return choice(keyword("with context"), keyword("without context"));
}

export default grammar({
  name: "jinja2",
  word: ($) => $.identifier,
  conflicts: ($) => [[$.elif_statement]],
  rules: {
    template: ($) => repeat($._item),

    _item: ($) => choice($._statement, $.output, $.comment, $.text),

    text: (_) =>
      prec.right(
        repeat1(choice(token(prec(-1, /\{/)), token(prec(1, /[^\{]+/)))),
      ),

    comment: () => seq("{#", repeat(/[^\#]+|[\#]/), "#}"),

    output: ($) =>
      seq("{{", alias(optional($._output_code), $.expression), "}}"),
    _output_code: () => prec.right(repeat1(/[^\s\}\-\+]+|[\}\-\+]/)),

    _expression_in_statement: () => repeat1(/[^\s\%\-\+]+|[\%\-\+]/),

    _statement: ($) =>
      choice(
        $.for_statement,
        $.if_statement,
        $.macro_statement,
        $.call_statement,
        $.filter_statement,
        $.assignment_statement,
        $.end_assignment_statement,
        $.extends_statement,
        $.block_statement,
        $.include_statement,
        $.import_statement,
        $.with_statement,
        $.raw_statement,
        $.custom_statement,
      ),

    for_statement: ($) =>
      seq(
        statement("for", field("iteration", expression_in_statement($))),
        field("body", repeat($._item)),
        choice($.for_else_statement, statement("endfor")),
      ),
    for_else_statement: ($) =>
      seq(
        statement("else"),
        field("body", repeat($._item)),
        statement("endfor"),
      ),

    if_statement: ($) =>
      seq(
        statement("if", field("condition", expression_in_statement($))),
        field("body", repeat($._item)),
        field("elif", repeat($.elif_statement)),
        choice(field("else", $.else_statement), statement("endif")),
      ),

    elif_statement: ($) =>
      seq(
        statement("elif", field("condition", expression_in_statement($))),
        repeat($._item),
      ),

    else_statement: ($) =>
      seq(
        statement("else"),
        field("body", repeat($._item)),
        statement("endif"),
      ),

    macro_statement: ($) =>
      seq(
        statement("macro", field("signature", expression_in_statement($))),
        repeat($._item),
        statement("endmacro"),
      ),

    call_statement: ($) =>
      seq(
        statement("call", field("call", expression_in_statement($))),
        repeat($._item),
        statement("endcall"),
      ),

    filter_statement: ($) =>
      seq(
        statement("filter", field("code", expression_in_statement($))),
        repeat($._item),
        statement("endfilter"),
      ),

    assignment_statement: ($) =>
      statement("set", field("code", expression_in_statement($))),

    end_assignment_statement: ($) => statement("endset"),

    extends_statement: ($) => statement("extends", expression_in_statement($)),

    block_statement: ($) =>
      seq(
        statement(
          "block",
          field("id", $.identifier),
          optional(keyword("scoped")),
          optional(keyword("required")),
        ),
        repeat($._item),
        statement("endblock", optional($.identifier)),
      ),

    include_statement: ($) =>
      statement(
        "include",
        choice($.string, $.identifier),
        optional(alias("ignore missing", "_keyword")),
        optional(context_specifier()),
      ),
    import_statement: ($) =>
      choice(
        statement(
          "import",
          field("id", $.string),
          alias("as", "_keyword"),
          $.identifier,
          optional(context_specifier()),
        ),
        statement(
          "from",
          field("id", $.string),
          keyword("import"),
          sep1(
            choice(
              $.identifier,
              seq($.identifier, keyword("as"), $.identifier),
            ),
            ",",
          ),
          optional(context_specifier()),
        ),
      ),

    with_statement: ($) =>
      seq(
        statement(
          "with",
          optional(field("assignment", expression_in_statement($))),
        ),
        repeat($._item),
        statement("endwith"),
      ),

    raw_statement: ($) =>
      seq(
        alias(
          token(seq(statement_start(), /\s*raw\s*/, statement_end())),
          "raw_start",
        ),
        $.text,
        alias(
          token(seq(statement_start(), /\s*endraw\s*/, statement_end())),
          "raw_end",
        ),
      ),

    custom_statement: ($) =>
      prec.dynamic(
        -1,
        seq(
          statement_start(),
          alias($._expression_in_statement, $.custom_tag),
          statement_end(),
        ),
      ),

    identifier: () => /[\w]+/,

    string: () => choice(seq(`"`, /[^\"]+/, `"`), seq(`'`, /[^\']+/, `'`)),
  },
});
