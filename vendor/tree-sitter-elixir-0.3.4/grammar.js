const PREC = {
  // See https://github.com/elixir-lang/elixir/blob/master/lib/elixir/src/elixir_parser.yrl
  IN_MATCH_OPS: 10,
  WHEN_OP: 20,
  TYPE_OP: 30,
  BAR_OP: 40,
  ASSOC_OP: 50,
  CAPTURE_OP: 60,
  MATCH_OP: 70,
  OR_OPS: 80,
  AND_OPS: 90,
  COMP_OPS: 100,
  REL_OPS: 110,
  ARROW_OPS: 120,
  IN_OPS: 130,
  XOR_OP: 140,
  TERNARY_OP: 150,
  CONCAT_OPS: 160,
  RANGE_OP: 160,
  ADD_OPS: 170,
  MULT_OPS: 180,
  POWER_OP: 190,
  UNARY_OPS: 200,
  ACCESS: 205,
  DOT_OP: 210,
  AT_OP: 220,
  CAPTURE_OPERAND: 235,
};

const IN_MATCH_OPS = ["<-", "\\\\"];
const OR_OPS = ["||", "|||", "or"];
const AND_OPS = ["&&", "&&&", "and"];
const COMP_OPS = ["==", "!=", "=~", "===", "!=="];
const REL_OPS = ["<", ">", "<=", ">="];
const ARROW_OPS = ["|>", "<<<", ">>>", "<<~", "~>>", "<~", "~>", "<~>", "<|>"];
const IN_OPS = ["in", "not in"];
const CONCAT_OPS = ["++", "--", "+++", "---", "<>"];
const ADD_OPS = ["+", "-"];
const MULT_OPS = ["*", "/"];
const UNARY_OPS = ["+", "-", "!", "^", "~~~", "not"];

const ALL_OPS = [
  ["->", "when", "::", "|", "=>", "&", "=", "^^^", "//", "..", "**", ".", "@"],
  IN_MATCH_OPS,
  OR_OPS,
  AND_OPS,
  COMP_OPS,
  REL_OPS,
  ARROW_OPS,
  IN_OPS,
  CONCAT_OPS,
  ADD_OPS,
  MULT_OPS,
  UNARY_OPS,
].flat();

// Ignore word literals and "=>" which is not a valid atom
const ATOM_OPERATOR_LITERALS = ALL_OPS.filter(
  (operator) => !/[a-z]/.test(operator) && operator !== "=>",
);

const ATOM_SPECIAL_LITERALS = ["...", "%{}", "{}", "%", "<<>>", "..//"];

// See Ref 6. in the docs
const ATOM_WORD_LITERAL = /[\p{ID_Start}_][\p{ID_Continue}@]*[?!]?/u;

// Word tokens used directly in the grammar
const RESERVED_WORD_TOKENS = [
  // Operators
  ["and", "in", "not", "or", "when"],
  // Literals
  ["true", "false", "nil"],
  // Other
  ["after", "catch", "do", "else", "end", "fn", "rescue"],
].flat();

const DIGITS = /[0-9]+/;
const BIN_DIGITS = /[0-1]+/;
const OCT_DIGITS = /[0-7]+/;
const HEX_DIGITS = /[0-9a-fA-F]+/;

const NUMBER_DEC = sep1(DIGITS, "_");
const NUMBER_BIN = seq("0b", sep1(BIN_DIGITS, "_"));
const NUMBER_OCT = seq("0o", sep1(OCT_DIGITS, "_"));
const NUMBER_HEX = seq("0x", sep1(HEX_DIGITS, "_"));

const INTEGER = choice(NUMBER_DEC, NUMBER_BIN, NUMBER_OCT, NUMBER_HEX);

const FLOAT_SCIENTIFIC_PART = seq(/[eE]/, optional(choice("-", "+")), INTEGER);
const FLOAT = seq(NUMBER_DEC, ".", NUMBER_DEC, optional(FLOAT_SCIENTIFIC_PART));

const NEWLINE = /\r?\n/;

module.exports = grammar({
  name: "elixir",

  externals: ($) => [
    // See Ref 1. in the docs
    $._quoted_content_i_single,
    $._quoted_content_i_double,
    $._quoted_content_i_heredoc_single,
    $._quoted_content_i_heredoc_double,
    $._quoted_content_i_parenthesis,
    $._quoted_content_i_curly,
    $._quoted_content_i_square,
    $._quoted_content_i_angle,
    $._quoted_content_i_bar,
    $._quoted_content_i_slash,
    $._quoted_content_single,
    $._quoted_content_double,
    $._quoted_content_heredoc_single,
    $._quoted_content_heredoc_double,
    $._quoted_content_parenthesis,
    $._quoted_content_curly,
    $._quoted_content_square,
    $._quoted_content_angle,
    $._quoted_content_bar,
    $._quoted_content_slash,

    // See Ref 2. in the docs
    $._newline_before_do,
    $._newline_before_binary_operator,
    $._newline_before_comment,

    // See Ref 3. in the docs
    $._before_unary_op,

    // See Ref 4. in the docs
    $._not_in,

    // See Ref 5. in the docs
    $._quoted_atom_start,
  ],

  extras: ($) => [
    NEWLINE,
    /[ \t]|\r?\n|\\\r?\n/,
    $.comment,
    $._newline_before_comment,
    // Placing this directly in the binary operator rule leads
    // to conflicts, but we can place it here without any drawbacks.
    // If we detect binary operator and the previous line is not a
    // valid expression, it's a syntax error either way
    $._newline_before_binary_operator,
  ],

  conflicts: ($) => [
    // Given `left • *`, `left` identifier can be either:
    //   * expression in `left * right`
    //   * call identifier in `left * / 2`
    [$._expression, $._local_call_without_parentheses],

    // Given `left • when`, `left` expression can be either:
    //   * binary operator operand in `left when right`
    //   * stab arguments item in `left when right ->`
    //
    // Given `arg1, left • when`, `left` expression can be either:
    //   * binary operator operand in `arg1, left when right, arg3`
    //   * stab arguments item in `arg1, left when right ->`
    [$.binary_operator, $._stab_clause_arguments_without_parentheses],

    // Given `((arg1, arg2 • ,`, `arg3` expression can be either:
    //   * stab parenthesised arguments item in `((arg1, arg2, arg3) ->)`
    //   * stab non-parenthesised arguments item in `((arg1, arg2, arg3 ->))`
    [
      $._stab_clause_arguments_without_parentheses,
      $._stab_clause_arguments_with_parentheses,
    ],

    // Given `(-> • /`, stab can be either:
    //   * stab clause operator in `(-> / / 2)`
    //   * operator identifier in `(-> / 2)`
    [$.operator_identifier, $.stab_clause],

    // Given `& • /`, ampersand can be either:
    //   * capture operator in `& / / 2`
    //   * operator identifier in `& / 1`
    [$.unary_operator, $.operator_identifier],

    // Given `(arg -> expression • \n`, the newline could be either:
    //   * terminator separating expressions in `(arg -> expression \n expression)`
    //   * terminator separating clauses in `(arg -> expression \n arg -> expression)`
    [$.body],
  ],

  rules: {
    source: ($) =>
      seq(
        optional($._terminator),
        optional(
          seq(sep1($._expression, $._terminator), optional($._terminator)),
        ),
      ),

    _terminator: ($) =>
      // Right precedence, because we want to consume `;` after newlines if present
      prec.right(choice(seq(repeat(NEWLINE), ";"), repeat1(NEWLINE))),

    _expression: ($) =>
      choice(
        $.block,
        $.identifier,
        $.alias,
        $.integer,
        $.float,
        $.char,
        $.boolean,
        $.nil,
        $._atom,
        $.string,
        $.charlist,
        $.sigil,
        $.list,
        $.tuple,
        $.bitstring,
        $.map,
        $._nullary_operator,
        $.unary_operator,
        $.binary_operator,
        $.dot,
        $.call,
        $.access_call,
        $.anonymous_function,
      ),

    block: ($) =>
      seq(
        "(",
        optional($._terminator),
        optional(
          choice(
            sep1(choice($.stab_clause), $._terminator),
            seq(
              sep1(choice($._expression), $._terminator),
              optional($._terminator),
            ),
          ),
        ),
        ")",
      ),

    identifier: ($) =>
      choice(
        // See Ref 6. in the docs
        /[_\p{Ll}\p{Lm}\p{Lo}\p{Nl}\u1885\u1886\u2118\u212E\u309B\u309C][\p{ID_Continue}]*[?!]?/u,
        "...",
      ),

    alias: ($) => token(sep1(/[A-Z][_a-zA-Z0-9]*/, /\s*\.\s*/)),

    integer: ($) => token(INTEGER),

    float: ($) => token(FLOAT),

    char: ($) => /\?(.|\\.)/,

    boolean: ($) => choice("true", "false"),

    nil: ($) => "nil",

    _atom: ($) => choice($.atom, $.quoted_atom),

    atom: ($) =>
      token(
        seq(
          ":",
          choice(
            ATOM_WORD_LITERAL,
            ...ATOM_OPERATOR_LITERALS,
            ...ATOM_SPECIAL_LITERALS,
          ),
        ),
      ),

    quoted_atom: ($) =>
      seq(
        alias($._quoted_atom_start, ":"),
        choice($._quoted_i_double, $._quoted_i_single),
      ),

    // Defines $._quoted_content_i_{name} and $._quoted_content_{name} rules,
    // content with and without interpolation respectively
    ...defineQuoted(`"`, `"`, "double"),
    ...defineQuoted(`'`, `'`, "single"),
    ...defineQuoted(`'''`, `'''`, "heredoc_single"),
    ...defineQuoted(`"""`, `"""`, "heredoc_double"),
    ...defineQuoted(`(`, `)`, "parenthesis"),
    ...defineQuoted(`{`, `}`, "curly"),
    ...defineQuoted(`[`, `]`, "square"),
    ...defineQuoted(`<`, `>`, "angle"),
    ...defineQuoted(`|`, `|`, "bar"),
    ...defineQuoted(`/`, `/`, "slash"),

    string: ($) => choice($._quoted_i_double, $._quoted_i_heredoc_double),

    charlist: ($) => choice($._quoted_i_single, $._quoted_i_heredoc_single),

    interpolation: ($) => seq("#{", optional($._expression), "}"),

    escape_sequence: ($) =>
      token(
        seq(
          "\\",
          choice(
            // Single escaped character
            /[^ux]/,
            // Hex byte
            /x[0-9a-fA-F]{1,2}/,
            /x\{[0-9a-fA-F]+\}/,
            // Unicode code point
            /u\{[0-9a-fA-F]+\}/,
            /u[0-9a-fA-F]{4}/,
          ),
        ),
      ),

    sigil: ($) =>
      seq(
        "~",
        choice(
          seq(
            alias(token.immediate(/[a-z]/), $.sigil_name),
            choice(
              $._quoted_i_double,
              $._quoted_i_single,
              $._quoted_i_heredoc_single,
              $._quoted_i_heredoc_double,
              $._quoted_i_parenthesis,
              $._quoted_i_curly,
              $._quoted_i_square,
              $._quoted_i_angle,
              $._quoted_i_bar,
              $._quoted_i_slash,
            ),
          ),
          seq(
            alias(token.immediate(/[A-Z][A-Z0-9]*/), $.sigil_name),
            choice(
              $._quoted_double,
              $._quoted_single,
              $._quoted_heredoc_single,
              $._quoted_heredoc_double,
              $._quoted_parenthesis,
              $._quoted_curly,
              $._quoted_square,
              $._quoted_angle,
              $._quoted_bar,
              $._quoted_slash,
            ),
          ),
        ),
        optional(alias(token.immediate(/[a-zA-Z0-9]+/), $.sigil_modifiers)),
      ),

    keywords: ($) =>
      // Right precedence, because we want to consume next items as long
      // as there is a comma ahead
      prec.right(sep1($.pair, ",")),

    _keywords_with_trailing_separator: ($) =>
      seq(sep1($.pair, ","), optional(",")),

    pair: ($) => seq(field("key", $._keyword), field("value", $._expression)),

    _keyword: ($) => choice($.keyword, $.quoted_keyword),

    keyword: ($) =>
      // See Ref 7. in the docs
      token(
        seq(
          choice(
            ATOM_WORD_LITERAL,
            ...ATOM_OPERATOR_LITERALS.filter((op) => op !== "::"),
            ...ATOM_SPECIAL_LITERALS,
          ),
          /:\s/,
        ),
      ),

    quoted_keyword: ($) =>
      seq(
        choice($._quoted_i_double, $._quoted_i_single),
        token.immediate(/:\s/),
      ),

    list: ($) => seq("[", optional($._items_with_trailing_separator), "]"),

    tuple: ($) => seq("{", optional($._items_with_trailing_separator), "}"),

    bitstring: ($) =>
      seq("<<", optional($._items_with_trailing_separator), ">>"),

    map: ($) =>
      // Precedence over tuple
      prec(
        1,
        seq(
          "%",
          optional($.struct),
          "{",
          optional(alias($._items_with_trailing_separator, $.map_content)),
          "}",
        ),
      ),

    struct: ($) =>
      // Left precedence, because if there is a conflict involving `{}`,
      // we want to treat it as map continuation rather than tuple
      prec.left(
        choice(
          $.alias,
          $._atom,
          $.identifier,
          $.unary_operator,
          $.dot,
          alias($._call_with_parentheses, $.call),
        ),
      ),

    _items_with_trailing_separator: ($) =>
      seq(
        choice(
          seq(sep1($._expression, ","), optional(",")),
          seq(
            optional(seq(sep1($._expression, ","), ",")),
            alias($._keywords_with_trailing_separator, $.keywords),
          ),
        ),
      ),

    _nullary_operator: ($) =>
      // Nullary operators don't have any child nodes, so we reuse the
      // operator_identifier node
      alias(prec(PREC.RANGE_OP, ".."), $.operator_identifier),

    unary_operator: ($) =>
      choice(
        unaryOp($, prec, PREC.CAPTURE_OP, "&", $._capture_expression),
        unaryOp($, prec, PREC.UNARY_OPS, choice(...UNARY_OPS)),
        unaryOp($, prec, PREC.AT_OP, "@"),
        // Capture operand like &1 is a special case with higher precedence
        unaryOp($, prec, PREC.CAPTURE_OPERAND, "&", $.integer),
      ),

    _capture_expression: ($) =>
      choice(
        // Note that block expression is not allowed as capture operand,
        // so we have an explicit sequence with the parentheses and higher
        // precedence
        prec(1, seq("(", $._expression, ")")),
        $._expression,
      ),

    binary_operator: ($) =>
      choice(
        binaryOp($, prec.left, PREC.IN_MATCH_OPS, choice(...IN_MATCH_OPS)),
        binaryOp(
          $,
          prec.right,
          PREC.WHEN_OP,
          "when",
          $._expression,
          choice($._expression, $.keywords),
        ),
        binaryOp($, prec.right, PREC.TYPE_OP, "::"),
        binaryOp(
          $,
          prec.right,
          PREC.BAR_OP,
          "|",
          $._expression,
          choice($._expression, $.keywords),
        ),
        binaryOp($, prec.right, PREC.ASSOC_OP, "=>"),
        binaryOp($, prec.right, PREC.MATCH_OP, "="),
        binaryOp($, prec.left, PREC.OR_OPS, choice(...OR_OPS)),
        binaryOp($, prec.left, PREC.AND_OPS, choice(...AND_OPS)),
        binaryOp($, prec.left, PREC.COMP_OPS, choice(...COMP_OPS)),
        binaryOp($, prec.left, PREC.REL_OPS, choice(...REL_OPS)),
        binaryOp($, prec.left, PREC.ARROW_OPS, choice(...ARROW_OPS)),
        binaryOp(
          $,
          prec.left,
          PREC.IN_OPS,
          choice("in", alias($._not_in, "not in")),
        ),
        binaryOp($, prec.left, PREC.XOR_OP, "^^^"),
        binaryOp($, prec.right, PREC.TERNARY_OP, "//"),
        binaryOp($, prec.right, PREC.CONCAT_OPS, choice(...CONCAT_OPS)),
        binaryOp($, prec.right, PREC.RANGE_OP, ".."),
        binaryOp($, prec.left, PREC.ADD_OPS, choice(...ADD_OPS)),
        binaryOp($, prec.left, PREC.MULT_OPS, choice(...MULT_OPS)),
        binaryOp($, prec.left, PREC.POWER_OP, "**"),
        // Operator with arity
        binaryOp(
          $,
          prec.left,
          PREC.MULT_OPS,
          "/",
          $.operator_identifier,
          $.integer,
        ),
      ),

    operator_identifier: ($) =>
      // Operators with the following changes:
      //
      //   * exclude "=>" since it's not a valid operator identifier
      //   * exclude // since it's only valid after ..
      //   * exclude binary "-" and "+" as they are handled as unary below
      //
      // For unary operator identifiers we use the same precedence as
      // operators, so that we get conflicts and resolve them dynamically
      // (see grammar.conflicts for more details)
      choice(
        // Unary operators
        prec(PREC.CAPTURE_OP, "&"),
        prec(PREC.UNARY_OPS, choice(...UNARY_OPS)),
        prec(PREC.AT_OP, "@"),
        // Binary operators
        ...IN_MATCH_OPS,
        "when",
        "::",
        "|",
        "=",
        ...OR_OPS,
        ...AND_OPS,
        ...COMP_OPS,
        ...REL_OPS,
        ...ARROW_OPS,
        "in",
        alias($._not_in, "not in"),
        "^^^",
        ...CONCAT_OPS,
        // The range operator has both a binary and a nullary version.
        // The nullary version is already parsed as operator_identifier,
        // so it covers this case
        // ".."
        ...MULT_OPS,
        "**",
        "->",
      ),

    dot: ($) =>
      prec(
        PREC.DOT_OP,
        seq(
          field("left", $._expression),
          field("operator", "."),
          field("right", choice($.alias, $.tuple)),
        ),
      ),

    call: ($) => choice($._call_without_parentheses, $._call_with_parentheses),

    _call_without_parentheses: ($) =>
      choice(
        $._local_call_without_parentheses,
        $._local_call_just_do_block,
        $._remote_call_without_parentheses,
      ),

    _call_with_parentheses: ($) =>
      choice(
        $._local_call_with_parentheses,
        $._remote_call_with_parentheses,
        $._anonymous_call,
        $._double_call,
      ),

    // Note, calls have left precedence, so that `do end` block sticks to
    // the outermost call

    _local_call_without_parentheses: ($) =>
      prec.left(
        seq(
          field("target", $.identifier),
          alias($._call_arguments_without_parentheses, $.arguments),
          optional(seq(optional($._newline_before_do), $.do_block)),
        ),
      ),

    _local_call_with_parentheses: ($) =>
      prec.left(
        seq(
          field("target", $.identifier),
          alias($._call_arguments_with_parentheses_immediate, $.arguments),
          optional(seq(optional($._newline_before_do), $.do_block)),
        ),
      ),

    _local_call_just_do_block: ($) =>
      // Lower precedence than identifier, because `foo bar do` is `foo(bar) do end`
      prec(-1, seq(field("target", $.identifier), $.do_block)),

    _remote_call_without_parentheses: ($) =>
      prec.left(
        seq(
          field("target", alias($._remote_dot, $.dot)),
          optional(alias($._call_arguments_without_parentheses, $.arguments)),
          optional(seq(optional($._newline_before_do), $.do_block)),
        ),
      ),

    _remote_call_with_parentheses: ($) =>
      prec.left(
        seq(
          field("target", alias($._remote_dot, $.dot)),
          alias($._call_arguments_with_parentheses_immediate, $.arguments),
          optional(seq(optional($._newline_before_do), $.do_block)),
        ),
      ),

    _remote_dot: ($) =>
      prec(
        PREC.DOT_OP,
        seq(
          field("left", $._expression),
          field("operator", "."),
          field(
            "right",
            choice(
              $.identifier,
              alias(choice(...RESERVED_WORD_TOKENS), $.identifier),
              $.operator_identifier,
              alias($._quoted_i_double, $.string),
              alias($._quoted_i_single, $.charlist),
            ),
          ),
        ),
      ),

    _anonymous_call: ($) =>
      seq(
        field("target", alias($._anonymous_dot, $.dot)),
        alias($._call_arguments_with_parentheses, $.arguments),
      ),

    _anonymous_dot: ($) =>
      prec(
        PREC.DOT_OP,
        seq(field("left", $._expression), field("operator", ".")),
      ),

    _double_call: ($) =>
      prec.left(
        seq(
          field(
            "target",
            alias(
              choice(
                $._local_call_with_parentheses,
                $._remote_call_with_parentheses,
                $._anonymous_call,
              ),
              $.call,
            ),
          ),
          alias($._call_arguments_with_parentheses, $.arguments),
          optional(seq(optional($._newline_before_do), $.do_block)),
        ),
      ),

    _call_arguments_with_parentheses: ($) =>
      seq("(", optional($._call_arguments_with_trailing_separator), ")"),

    _call_arguments_with_parentheses_immediate: ($) =>
      seq(
        token.immediate("("),
        optional($._call_arguments_with_trailing_separator),
        ")",
      ),

    _call_arguments_with_trailing_separator: ($) =>
      choice(
        seq(
          sep1($._expression, ","),
          optional(
            seq(",", alias($._keywords_with_trailing_separator, $.keywords)),
          ),
        ),
        alias($._keywords_with_trailing_separator, $.keywords),
      ),

    _call_arguments_without_parentheses: ($) =>
      // In stab clauses a newline can either separate multiple body expressions
      // or multiple stab clauses, this falls under the $.body conflict. Given a
      // multiline stab clause with trailing identifier like `1 -> 1 \n x \n 2 -> x`,
      // there are two matching interpretations:
      //   * `x` as identifier and `2` as stab argument
      //   * `x 2` call as stab argument
      // Similarly for `Mod.fun` or `mod.fun` the newline should terminate the call.
      // Consequently, we reject the second interpretation using dynamic precedence
      prec.dynamic(
        -1,
        // Right precedence, because `fun1 fun2 x, y` is `fun1(fun2(x, y))`
        prec.right(
          choice(
            seq(sep1($._expression, ","), optional(seq(",", $.keywords))),
            $.keywords,
          ),
        ),
      ),

    do_block: ($) =>
      seq(
        callKeywordBlock($, "do"),
        repeat(
          choice($.after_block, $.rescue_block, $.catch_block, $.else_block),
        ),
        "end",
      ),

    after_block: ($) => callKeywordBlock($, "after"),
    rescue_block: ($) => callKeywordBlock($, "rescue"),
    catch_block: ($) => callKeywordBlock($, "catch"),
    else_block: ($) => callKeywordBlock($, "else"),

    access_call: ($) =>
      prec(
        PREC.ACCESS,
        seq(
          field("target", $._expression),
          token.immediate("["),
          field("key", $._expression),
          "]",
        ),
      ),

    stab_clause: ($) =>
      // Right precedence, because we want to consume body if any
      prec.right(
        seq(
          optional(field("left", $._stab_clause_left)),
          field("operator", "->"),
          optional(field("right", $.body)),
        ),
      ),

    _stab_clause_left: ($) =>
      choice(
        alias($._stab_clause_arguments_with_parentheses, $.arguments),
        alias(
          $._stab_clause_arguments_with_parentheses_with_guard,
          $.binary_operator,
        ),
        alias($._stab_clause_arguments_without_parentheses, $.arguments),
        alias(
          $._stab_clause_arguments_without_parentheses_with_guard,
          $.binary_operator,
        ),
      ),

    _stab_clause_arguments_with_parentheses: ($) =>
      // Precedence over block expression
      prec(
        1,
        seq(
          "(",
          optional(
            choice(
              seq(
                // We need the same expression precedence as below, so that we don't
                // discard this rule in favour of the one below. We use right precedence,
                // because in this case we can consume expression until the next comma
                sep1(prec.right(PREC.WHEN_OP, $._expression), ","),
                optional(seq(",", $.keywords)),
              ),
              $.keywords,
            ),
          ),
          ")",
        ),
      ),

    _stab_clause_arguments_without_parentheses: ($) =>
      // We give the arguments and expression the same precedence as "when"
      // binary operator, so that we get conflicts and resolve them dynamically
      // (see the grammar.conflicts for more details)
      prec(
        PREC.WHEN_OP,
        choice(
          seq(
            sep1(prec(PREC.WHEN_OP, $._expression), ","),
            optional(seq(",", $.keywords)),
          ),
          $.keywords,
        ),
      ),

    _stab_clause_arguments_with_parentheses_with_guard: ($) =>
      seq(
        field(
          "left",
          alias($._stab_clause_arguments_with_parentheses, $.arguments),
        ),
        field("operator", "when"),
        field("right", $._expression),
      ),

    _stab_clause_arguments_without_parentheses_with_guard: ($) =>
      // Given `a when b ->`, the left stab operand can be interpreted either
      // as a single argument item, or as binary operator with arguments on
      // the left and guard expression on the right. Using dynamic precedence
      // we favour the latter interpretation during dynamic conflict resolution
      prec.dynamic(
        1,
        seq(
          field(
            "left",
            alias($._stab_clause_arguments_without_parentheses, $.arguments),
          ),
          field("operator", "when"),
          field("right", $._expression),
        ),
      ),

    body: ($) =>
      choice(
        $._terminator,
        seq(
          optional($._terminator),
          sep1($._expression, $._terminator),
          optional($._terminator),
        ),
      ),

    anonymous_function: ($) =>
      seq(
        "fn",
        optional($._terminator),
        // See Ref 8. in the docs
        optional(sep1($.stab_clause, $._terminator)),
        "end",
      ),

    // A comment may be anywhere, we give it a lower precedence,
    // so it doesn't intercept interpolation
    comment: ($) => token(prec(-1, seq("#", /.*/))),
  },
});

function sep1(rule, separator) {
  return seq(rule, repeat(seq(separator, rule)));
}

function unaryOp($, assoc, precedence, operator, right = null) {
  // Expression such as `x + y` falls under the "expression vs local call"
  // conflict that we already have. By using dynamic precedence we penalize
  // unary operator, so `x + y` is interpreted as binary operator (unless
  // _before_unary_op is tokenized and forces unary operator interpretation)
  return prec.dynamic(
    -1,
    assoc(
      precedence,
      seq(
        optional($._before_unary_op),
        field("operator", operator),
        field("operand", right || $._expression),
      ),
    ),
  );
}

function binaryOp($, assoc, precedence, operator, left = null, right = null) {
  return assoc(
    precedence,
    seq(
      field("left", left || $._expression),
      field("operator", operator),
      field("right", right || $._expression),
    ),
  );
}

function callKeywordBlock($, start) {
  return seq(
    start,
    optional($._terminator),
    optional(
      choice(
        sep1(choice($.stab_clause), $._terminator),
        seq(
          sep1(choice($._expression), $._terminator),
          optional($._terminator),
        ),
      ),
    ),
  );
}

function defineQuoted(start, end, name) {
  return {
    [`_quoted_i_${name}`]: ($) =>
      seq(
        field("quoted_start", start),
        optional(alias($[`_quoted_content_i_${name}`], $.quoted_content)),
        repeat(
          seq(
            choice($.interpolation, $.escape_sequence),
            optional(alias($[`_quoted_content_i_${name}`], $.quoted_content)),
          ),
        ),
        field("quoted_end", end),
      ),

    [`_quoted_${name}`]: ($) =>
      seq(
        field("quoted_start", start),
        optional(alias($[`_quoted_content_${name}`], $.quoted_content)),
        repeat(
          seq(
            // The end delimiter may be escaped in non-interpolating strings too
            $.escape_sequence,
            optional(alias($[`_quoted_content_${name}`], $.quoted_content)),
          ),
        ),
        field("quoted_end", end),
      ),
  };
}
