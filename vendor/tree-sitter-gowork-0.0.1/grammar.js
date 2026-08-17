module.exports = grammar({
  name: "gowork",

  extras: ($) => [$.comment, /\s/],

  rules: {
    source_file: ($) => repeat($._directive),

    _directive: ($) =>
      choice(
        $.go_directive,
        $.use_directive,
        $.replace_directive,
      ),

    _string_literal: ($) =>
      choice($.raw_string_literal, $.interpreted_string_literal),

    raw_string_literal: ($) => token(seq("`", repeat(/[^`]/), "`")),

    interpreted_string_literal: ($) =>
      seq(
        '"',
        repeat(
          choice(token.immediate(prec(1, /[^"\n\\]+/)), $.escape_sequence)
        ),
        '"'
      ),

    escape_sequence: ($) =>
      token.immediate(
        seq(
          "\\",
          choice(
            /[^xuU]/,
            /\d{2,3}/,
            /x[0-9a-fA-F]{2,}/,
            /u[0-9a-fA-F]{4}/,
            /U[0-9a-fA-F]{8}/
          )
        )
      ),

    _identifier: ($) => token(/[^\s,\[\]]+/),

    _string_or_ident: ($) => choice($._string_literal, $._identifier),

    module_path: ($) => $._string_or_ident,
    go_version: ($) => $._string_or_ident,
    version: ($) => $._string_or_ident,

    go_directive: ($) => seq("go", $.go_version, "\n"),

    replace_directive: ($) =>
      seq(
        "replace",
        choice(
          $.replace_spec,
          seq("(", "\n", repeat($.replace_spec), ")", "\n")
        )
      ),

    replace_spec: ($) =>
      choice(
        seq($.module_path, optional($.version), "=>", $.file_path, "\n"),
        seq(
          $.module_path,
          optional($.version),
          "=>",
          $.module_path,
          $.version,
          "\n"
        )
      ),

    use_directive: ($) =>
      seq(
        "use",
        choice(
          $.use_spec,
          seq("(", "\n", repeat($.use_spec), ")", "\n")
        )
      ),

    use_spec: ($) =>
      seq($.file_path, "\n"),

    file_path: ($) => $._identifier,

    comment: ($) => seq("//", /.*/),
  },
});
