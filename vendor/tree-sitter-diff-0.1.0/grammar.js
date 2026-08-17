const NEWLINE = /\r?\n/;
const WHITE_SPACE = /[\t\f\v ]+/;
const ANYTHING = /[^\r\n]+/;

module.exports = grammar({
  name: "diff",

  extras: ($) => [WHITE_SPACE],

  rules: {
    source: ($) =>
      seq(
        repeat(choice($.block, seq(optional($._line), NEWLINE))),
        optional($._line)
      ),

    _line: ($) =>
      choice(
        $.file_change,
        $.binary_change,
        $.index,
        $.similarity,
        $.old_file,
        $.new_file,
        $.location,
        $.addition,
        $.deletion,
        $.context,
        $.comment
      ),

    block: ($) =>
      prec.right(
        seq(
          $.command,
          NEWLINE,
          repeat(
            seq(
              choice($.file_change, $.binary_change, $.index, $.similarity),
              NEWLINE
            )
          ),
          optional(seq($.old_file, NEWLINE, $.new_file, NEWLINE, $.hunks))
        )
      ),

    hunks: ($) => prec.right(repeat1($.hunk)),

    hunk: ($) =>
      prec.right(
        seq(
          field("location", $.location),
          NEWLINE,
          optional(field("changes", $.changes))
        )
      ),

    changes: ($) =>
      prec.right(
        repeat1(
          seq(
            choice($.addition, $.deletion, $.context),
            prec.right(repeat1(NEWLINE))
          )
        )
      ),

    command: ($) => iseq("diff", alias(/[-\w]+/, $.argument), $.filename),

    file_change: ($) =>
      choice(
        seq(choice("new", "deleted"), "file", "mode", $.mode),
        seq(choice("new", "old"), "mode", $.mode),
        seq("rename", choice("from", "to"), $.filename)
      ),

    binary_change: ($) =>
      iseq("Binary", "files", $.filename, "and", $.filename, "differ"),

    index: ($) => iseq("index", $.commit, "..", $.commit, optional($.mode)),

    similarity: ($) => iseq("similarity", "index", alias(/\d+/, $.score), "%"),

    old_file: ($) => iseq("---", $.filename),
    new_file: ($) => iseq("+++", $.filename),

    location: ($) =>
      iseq("@@", $.linerange, $.linerange, "@@", optional(ANYTHING)),

    addition: ($) =>
      choice(
        iseq("+", optional(ANYTHING)),
        iseq("++", optional(ANYTHING)),
        iseq("+++"),
        iseq("++++", optional(ANYTHING))
      ),
    deletion: ($) =>
      choice(
        iseq("-", optional(ANYTHING)),
        iseq("--", optional(ANYTHING)),
        iseq("---"),
        iseq("----", optional(ANYTHING))
      ),

    context: ($) => token(prec(-1, ANYTHING)),
    comment: ($) => iseq("#", optional(ANYTHING)),

    linerange: ($) => /[-\+]\d+(,\d+)?/,
    filename: ($) => repeat1(/\S+/),
    commit: ($) => /[a-f0-9]{7,40}/,
    mode: ($) => /\d+/,
  },
});

function iseq(start_token, ...tokens) {
  return seq(token.immediate(start_token), ...tokens);
}
