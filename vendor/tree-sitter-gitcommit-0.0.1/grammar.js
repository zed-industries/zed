/// <reference types="./node_modules/tree-sitter-cli/dsl.d.ts">
/* glossary:
   - subject: the first line of a commit
   - message: the body of a commit
     - usually starts on the third line of the file (1-indexed)
     - may be interspersed with comments
   - item: an issue or PR
   - change: how a file will change with this commit
     - either 'new file', 'modified', 'renamed' or 'deleted'
 */

/**
 * Whitespace that is ignored by the parser: tab, form feed, vertical tab, and
 * space characters. Note that whitespace doesn't include newlines!
 */
const WHITE_SPACE = /[\t\f\v ]+/;
const ANYTHING = /[^\n\r]+/;

// see https://tree-sitter.github.io/tree-sitter/creating-parsers#lexical-precedence-vs-parse-precedence
const PARSE_PRECEDENCE = {
  NONSENSE: -1,
  BODY_LINE: 1,
  COMMENT: 2,
  TRAILERS: 3,
};
const LEXICAL_PRECEDENCE = {
  ANY_WORD: -3,
  NONSENSE: -2,
  NON_PUNCTUATED_WORD: -1,
  // string literals and RegExes not decorated with `prec(LEXICAL_PRECEDENCE._)`
  // have precedence 0.
  PATH: 1,
  PATH_SEPARATOR_ARROW: 2,
  ITEM: 3,
  USER: 4,
  SUBJECT_FIRST_CHAR: 5,
};

const SCISSORS =
  /# -+ >8 -+\r?\n# Do not modify or remove the line above.\r?\n# Everything below it will be ignored.\r?\n?/;

module.exports = grammar({
  name: "git_commit",

  extras: ($) => [WHITE_SPACE],

  rules: {
    source: ($) =>
      seq(
        optional(choice($.comment, $.subject)),
        repeat($._body_line),
        repeat($._trailer),
        repeat(seq($._newline, optional($.comment))),
        optional(
          seq(alias(SCISSORS, $.scissors), optional(alias($._rest, $.message)))
        )
      ),

    /**
     * The subject of the commit message: the first line.
     */
    subject: ($) => seq(/[^#\r\n]/, repeat(ANYTHING)),

    _body_line: ($) =>
      prec.right(
        PARSE_PRECEDENCE.BODY_LINE,
        seq($._newline, optional(choice($.message, $.comment)))
      ),

    _trailer: ($) =>
      prec(PARSE_PRECEDENCE.TRAILERS, seq($._newline, $.trailer)),

    /**
     * Non-comment body
     */
    message: ($) =>
      choice(
        // Lines starting with spaces are certainly messages and may start with any characters.
        seq(WHITE_SPACE, repeat($._text)),
        // Otherwise message lines must not start with '#'.
        seq(
          choice($.user, $.commit, $._non_punctuated_word, $._non_comment),
          repeat($._text)
        )
      ),

    _text: ($) => choice($.user, $.item, $.commit, $._word),

    comment: ($) => seq("#", optional($._comment_body)),

    _comment_body: ($) =>
      choice(
        alias($._rebase_summary, $.summary),
        $.summary,
        $._branch_declaration,
        // fallback to regular comment words if the words are nonsense
        repeat1($._word)
      ),

    /**
     * Trailers are "lines that look similar to RFC 822 e-mail headers at the
     * end of the otherwise free-form part of a commit message." For more details
     * on the format, see https://git-scm.com/docs/git-interpret-trailers
     */
    trailer: ($) =>
      seq(
        field("key", $.trailer_key),
        field("separator", choice(":", "=")),
        field("value", $.trailer_value)
      ),

    trailer_key: ($) => $._word,
    trailer_value: ($) => repeat1(choice($.user, $.item, $.commit, $._word)),

    _rebase_summary: ($) =>
      seq(
        seq(
          "interactive",
          "rebase",
          "in",
          "progress",
          ";",
          "onto",
          $.commit,
          $._newline
        ),
        seq("#", alias($._rebase_header, $.header), $._newline),
        repeat(seq("#", $.rebase_command, $._newline)),
        seq("#", alias($._rebase_header, $.header), $._newline),
        repeat(seq("#", $.rebase_command, $._newline)),
        seq(
          "#",
          "You",
          "are",
          "currently",
          repeat(/\S+/),
          "rebasing",
          "branch",
          "'",
          $.branch,
          "'",
          "on",
          "'",
          $.commit,
          "'",
          "."
        ),
        $._newline,
        optional("#")
      ),

    _rebase_header: ($) =>
      choice(
        seq(
          "Last",
          /commands?/,
          "done",
          "(",
          /\d+/,
          /commands?/,
          "done",
          ")",
          ":"
        ),
        seq(
          "Next",
          /commands?/,
          "to",
          "do",
          "(",
          /\d+/,
          "remaining",
          /commands?/,
          ")",
          ":"
        ),
        seq("No", "commands", "remaining", ".")
      ),

    summary: ($) =>
      choice(
        seq(
          alias($._change_header, $.header),
          $._newline,
          repeat1(seq("#", $.change, $._newline)),
          optional("#")
        ),
        seq(
          $.header,
          $._newline,
          repeat1(seq("#", $.path, $._newline)),
          optional("#")
        )
      ),

    _change_header: ($) =>
      choice(
        seq("Changes", "to", "be", "committed", ":"),
        seq("Changes", "not", "staged", "for", "commit", ":")
      ),

    _branch_declaration: ($) =>
      choice(
        seq("On", "branch", $.branch),
        seq(
          "Your",
          "branch",
          "is",
          "up",
          "to",
          "date",
          "with",
          "'",
          $.branch,
          "'."
        ),
        seq(
          "Your",
          "branch",
          "is",
          choice(seq("ahead", "of"), "behind"),
          "'",
          $.branch,
          "'",
          "by",
          /\d+/,
          /commits?/,
          "."
        ),
        seq(
          "Your",
          "branch",
          "and",
          "'",
          $.branch,
          "'",
          "have",
          "diverged",
          ","
        ),
        // # HEAD detached at upstream/gh-pages
        seq("HEAD", "detached", "at", choice($.commit, $.branch))
      ),

    header: ($) => seq(choice("Conflicts", seq("Untracked", "files")), ":"),

    change: ($) =>
      seq(
        field("kind", choice("new file", "modified", "renamed", "deleted")),
        ":",
        $.path,
        optional(
          seq(
            token(prec(LEXICAL_PRECEDENCE.PATH_SEPARATOR_ARROW, "->")),
            $.path
          )
        )
      ),

    commit: ($) => /[a-f0-9]{7,40}/,

    _non_punctuated_word: ($) =>
      token(prec(LEXICAL_PRECEDENCE.NON_PUNCTUATED_WORD, /[-\w]+/)),
    _non_comment: ($) => token(prec(LEXICAL_PRECEDENCE.NONSENSE, /[^#\s]+/)),
    _any_word: ($) => token(prec(LEXICAL_PRECEDENCE.ANY_WORD, /\S+/)),
    _word: ($) =>
      prec(
        PARSE_PRECEDENCE.NONSENSE,
        choice($._non_punctuated_word, $._non_comment, $._any_word)
      ),
    /**
     * For most of the details on branch name constraints, see https://git-scm.com/docs/git-check-ref-format
     */
    branch: ($) => /[^\.\s']+/,

    rebase_command: ($) =>
      seq(
        choice(
          "pick",
          "edit",
          "squash",
          "merge",
          "fixup",
          "drop",
          "reword",
          "exec",
          "label",
          "reset"
        ),
        repeat1(/\S+/)
      ),

    path: ($) => repeat1(token(prec(LEXICAL_PRECEDENCE.PATH, /\S+/))),
    /** a github-style issue or PR reference */
    item: ($) => token(prec(LEXICAL_PRECEDENCE.ITEM, /#\d+/)),

    user: ($) => token(prec(LEXICAL_PRECEDENCE.USER, /@[a-zA-Z0-9_-]+/)),

    _rest: ($) => repeat1(choice(/.*/, $._newline)),
    _newline: ($) => /\r?\n/,
  },
});
