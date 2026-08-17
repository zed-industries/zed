/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

const PREC = {
  COMMENT: -2,
  CURLY_BLOCK: 1,
  DO_BLOCK: -1,

  AND: -2,
  OR: -2,
  NOT: 5,
  DEFINED: 10,
  ALIAS: 11,
  ASSIGN: 15,
  RESCUE: 16,
  CONDITIONAL: 20,
  RANGE: 25,
  BOOLEAN_OR: 30,
  BOOLEAN_AND: 35,
  RELATIONAL: 40,
  COMPARISON: 45,
  BITWISE_OR: 50,
  BITWISE_AND: 55,
  CALL: 56,
  SHIFT: 60,
  ADDITIVE: 65,
  MULTIPLICATIVE: 70,
  UNARY_MINUS: 75,
  EXPONENTIAL: 80,
  COMPLEMENT: 85,
};

const IDENTIFIER_CHARS = /[^\x00-\x1F\s:;`"'@$#.,|^&<=>+\-*/\\%?!~()\[\]{}]*/;
const LOWER_ALPHA_CHAR = /[^\x00-\x1F\sA-Z0-9:;`"'@$#.,|^&<=>+\-*/\\%?!~()\[\]{}]/;
const ALPHA_CHAR = /[^\x00-\x1F\s0-9:;`"'@$#.,|^&<=>+\-*/\\%?!~()\[\]{}]/;

module.exports = grammar({
  name: 'ruby',
  inline: $ => [$._arg_rhs, $._call_operator],
  externals: $ => [
    $._line_break,
    $._no_line_break,

    // Delimited literals
    $.simple_symbol,
    $._string_start,
    $._symbol_start,
    $._subshell_start,
    $._regex_start,
    $._string_array_start,
    $._symbol_array_start,
    $._heredoc_body_start,
    $.string_content,
    $.heredoc_content,
    $._string_end,
    $.heredoc_end,
    $.heredoc_beginning,

    // Tokens that require lookahead
    '/',
    $._block_ampersand,
    $._splat_star,
    $._unary_minus,
    $._unary_minus_num,
    $._binary_minus,
    $._binary_star,
    $._singleton_class_left_angle_left_langle,
    $.hash_key_symbol,
    $._identifier_suffix,
    $._constant_suffix,
    $._hash_splat_star_star,
    $._binary_star_star,
    $._element_reference_bracket,
    $._short_interpolation,
  ],

  extras: $ => [
    $.comment,
    $.heredoc_body,
    /\s/,
    /\\\r?\n/,
  ],

  word: $ => $.identifier,

  supertypes: $ => [
    $._statement,
    $._arg,
    $._call_operator,
    $._method_name,
    $._expression,
    $._variable,
    $._primary,
    $._simple_numeric,
    $._lhs,
    $._nonlocal_variable,
    $._pattern_top_expr_body,
    $._pattern_expr,
    $._pattern_expr_basic,
    $._pattern_primitive,
    $._pattern_constant,
  ],

  rules: {
    program: $ => seq(
      optional($._statements),
      optional(
        choice(
          seq(/__END__/, $.uninterpreted),
        ),
      ),
    ),

    uninterpreted: _ => /(.|\s)*/,

    block_body: $ => $._statements,

    _statements: $ => choice(
      seq(
        repeat1(choice(
          seq($._statement, $._terminator),
          $.empty_statement,
        )),
        optional($._statement),
      ),
      $._statement,
    ),

    begin_block: $ => seq('BEGIN', '{', optional($._statements), '}'),
    end_block: $ => seq('END', '{', optional($._statements), '}'),

    _statement: $ => choice(
      $.undef,
      $.alias,
      $.if_modifier,
      $.unless_modifier,
      $.while_modifier,
      $.until_modifier,
      $.rescue_modifier,
      $.begin_block,
      $.end_block,
      $._expression,
    ),

    method: $ => seq('def', $._method_rest),

    singleton_method: $ => seq(
      'def',
      seq(
        choice(
          field('object', $._variable),
          seq('(', field('object', $._arg), ')'),
        ),
        choice('.', '::'),
      ),
      $._method_rest,
    ),

    _method_rest: $ => seq(
      field('name', $._method_name),
      choice(
        $._body_expr,
        seq(
          field('parameters', alias($.parameters, $.method_parameters)),
          choice(
            seq(optional($._terminator), optional(field('body', $.body_statement)), 'end'),
            $._body_expr,
          ),

        ),
        seq(
          optional(
            field('parameters', alias($.bare_parameters, $.method_parameters)),
          ),
          $._terminator,
          optional(field('body', $.body_statement)),
          'end',
        ),
      ),
    ),

    rescue_modifier_arg: $ => prec(PREC.RESCUE,
      seq(
        field('body', $._arg),
        'rescue',
        field('handler', $._arg),
      ),
    ),

    rescue_modifier_expression: $ => prec(PREC.RESCUE,
      seq(
        field('body', $._expression),
        'rescue',
        field('handler', $._arg),
      ),
    ),

    _body_expr: $ =>
      seq(
        '=',
        field('body',
          choice(
            $._arg,
            alias($.rescue_modifier_arg, $.rescue_modifier),
          )),
      ),


    parameters: $ => seq(
      '(',
      commaSep($._formal_parameter),
      ')',
    ),

    bare_parameters: $ => seq(
      $._simple_formal_parameter,
      repeat(seq(',', $._formal_parameter)),
    ),

    block_parameters: $ => seq(
      '|',
      seq(commaSep($._formal_parameter), optional(',')),
      optional(seq(';', sep1(field('locals', $.identifier), ','))), // Block shadow args e.g. {|; a, b| ...}
      '|',
    ),

    _formal_parameter: $ => choice(
      $._simple_formal_parameter,
      alias($.parameters, $.destructured_parameter),
    ),

    _simple_formal_parameter: $ => choice(
      $.identifier,
      $.splat_parameter,
      $.hash_splat_parameter,
      $.hash_splat_nil,
      $.forward_parameter,
      $.block_parameter,
      $.keyword_parameter,
      $.optional_parameter,
    ),

    forward_parameter: _ => '...',

    splat_parameter: $ => prec.right(-2, seq(
      '*',
      field('name', optional($.identifier)),
    )),
    hash_splat_parameter: $ => seq(
      '**',
      field('name', optional($.identifier)),
    ),
    hash_splat_nil: _ => seq('**', 'nil'),
    block_parameter: $ => seq(
      '&',
      field('name', optional($.identifier)),
    ),
    keyword_parameter: $ => prec.right(PREC.BITWISE_OR + 1, seq(
      field('name', $.identifier),
      token.immediate(':'),
      field('value', optional($._arg)),
    )),
    optional_parameter: $ => prec(PREC.BITWISE_OR + 1, seq(
      field('name', $.identifier),
      '=',
      field('value', $._arg),
    )),

    class: $ => seq(
      'class',
      field('name', choice($.constant, $.scope_resolution)),
      choice(
        seq(field('superclass', $.superclass), $._terminator),
        optional($._terminator),
      ),
      optional(field('body', $.body_statement)),
      'end',
    ),

    superclass: $ => seq('<', $._expression),

    singleton_class: $ => seq(
      'class',
      alias($._singleton_class_left_angle_left_langle, '<<'),
      field('value', $._arg),
      $._terminator,
      optional(field('body', $.body_statement)),
      'end',
    ),

    module: $ => seq(
      'module',
      field('name', choice($.constant, $.scope_resolution)),
      optional($._terminator),
      optional(field('body', $.body_statement)),
      'end',
    ),

    return_command: $ => prec.left(seq('return', alias($.command_argument_list, $.argument_list))),
    yield_command: $ => prec.left(seq('yield', alias($.command_argument_list, $.argument_list))),
    break_command: $ => prec.left(seq('break', alias($.command_argument_list, $.argument_list))),
    next_command: $ => prec.left(seq('next', alias($.command_argument_list, $.argument_list))),
    return: $ => prec.left(seq('return', optional($.argument_list))),
    yield: $ => prec.left(seq('yield', optional($.argument_list))),
    break: $ => prec.left(seq('break', optional($.argument_list))),
    next: $ => prec.left(seq('next', optional($.argument_list))),
    redo: $ => prec.left(seq('redo', optional($.argument_list))),
    retry: $ => prec.left(seq('retry', optional($.argument_list))),

    if_modifier: $ => prec(PREC.RESCUE, seq(
      field('body', $._statement),
      'if',
      field('condition', $._expression),
    )),

    unless_modifier: $ => prec(PREC.RESCUE, seq(
      field('body', $._statement),
      'unless',
      field('condition', $._expression),
    )),

    while_modifier: $ => prec(PREC.RESCUE, seq(
      field('body', $._statement),
      'while',
      field('condition', $._expression),
    )),

    until_modifier: $ => prec(PREC.RESCUE, seq(
      field('body', $._statement),
      'until',
      field('condition', $._expression),
    )),

    rescue_modifier: $ => prec(PREC.RESCUE, seq(
      field('body', $._statement),
      'rescue',
      field('handler', $._expression),
    )),

    while: $ => seq(
      'while',
      field('condition', $._statement),
      field('body', $.do),
    ),

    until: $ => seq(
      'until',
      field('condition', $._statement),
      field('body', $.do),
    ),

    for: $ => seq(
      'for',
      field('pattern', choice($._lhs, $.left_assignment_list)),
      field('value', $.in),
      field('body', $.do),
    ),

    in: $ => seq('in', $._arg),
    do: $ => seq(
      choice('do', $._terminator),
      optional($._statements),
      'end',
    ),

    case: $ => seq(
      'case',
      optional(seq(optional($._line_break), field('value', $._statement))),
      optional($._terminator),
      repeat($.when),
      optional($.else),
      'end',
    ),

    case_match: $ => seq(
      'case',
      seq(optional($._line_break), field('value', $._statement)),
      optional($._terminator),
      repeat1(field('clauses', $.in_clause)),
      optional(field('else', $.else)),
      'end',
    ),

    when: $ => seq(
      'when',
      commaSep1(field('pattern', $.pattern)),
      choice($._terminator, field('body', $.then)),
    ),

    in_clause: $ => seq(
      'in',
      field('pattern', $._pattern_top_expr_body),
      field('guard', optional($._guard)),
      choice($._terminator, field('body', $.then)),
    ),

    pattern: $ => choice($._arg, $.splat_argument),

    _guard: $ => choice(
      $.if_guard,
      $.unless_guard,
    ),

    if_guard: $ => seq(
      'if',
      field('condition', $._expression),
    ),

    unless_guard: $ => seq(
      'unless',
      field('condition', $._expression),
    ),

    _pattern_top_expr_body: $ => prec(-1, choice(
      $._pattern_expr,
      alias($._array_pattern_n, $.array_pattern),
      alias($._find_pattern_body, $.find_pattern),
      alias($._hash_pattern_body, $.hash_pattern),
    )),

    _array_pattern_n: $ => prec.right(choice(
      seq($._pattern_expr, alias(',', $.splat_parameter)),
      seq($._pattern_expr, ',', choice($._pattern_expr, $._array_pattern_n)),
      seq($.splat_parameter, repeat(seq(',', $._pattern_expr))),
    )),

    _pattern_expr: $ => choice(
      $.as_pattern,
      $._pattern_expr_alt,
    ),

    as_pattern: $ => seq(field('value', $._pattern_expr), '=>', field('name', $.identifier)),

    _pattern_expr_alt: $ => choice(
      $.alternative_pattern,
      $._pattern_expr_basic,
    ),

    alternative_pattern: $ => seq(field('alternatives', $._pattern_expr_basic), repeat1(seq('|', field('alternatives', $._pattern_expr_basic)))),

    _array_pattern_body: $ => choice(
      $._pattern_expr,
      $._array_pattern_n,
    ),

    array_pattern: $ => prec.right(-1, choice(
      seq('[', optional($._array_pattern_body), ']'),
      seq(field('class', $._pattern_constant), token.immediate('['), optional($._array_pattern_body), ']'),
      seq(field('class', $._pattern_constant), token.immediate('('), optional($._array_pattern_body), ')'),
    )),

    _find_pattern_body: $ => seq($.splat_parameter, repeat1(seq(',', $._pattern_expr)), ',', $.splat_parameter),
    find_pattern: $ => choice(
      seq('[', $._find_pattern_body, ']'),
      seq(field('class', $._pattern_constant), token.immediate('['), $._find_pattern_body, ']'),
      seq(field('class', $._pattern_constant), token.immediate('('), $._find_pattern_body, ')'),
    ),

    _hash_pattern_body: $ => prec.right(choice(
      seq(commaSep1($.keyword_pattern), optional(',')),
      seq(commaSep1($.keyword_pattern), ',', $._hash_pattern_any_rest),
      $._hash_pattern_any_rest,
    )),

    keyword_pattern: $ => prec.right(-1, seq(
      field('key',
        choice(
          alias($.identifier, $.hash_key_symbol),
          alias($.constant, $.hash_key_symbol),
          alias($.identifier_suffix, $.hash_key_symbol),
          alias($.constant_suffix, $.hash_key_symbol),
          $.string,
        ),
      ),
      token.immediate(':'),
      optional(field('value', $._pattern_expr)),
    )),

    _hash_pattern_any_rest: $ => choice($.hash_splat_parameter, $.hash_splat_nil),

    hash_pattern: $ => prec.right(-1, choice(
      seq('{', optional($._hash_pattern_body), '}'),
      seq(field('class', $._pattern_constant), token.immediate('['), $._hash_pattern_body, ']'),
      seq(field('class', $._pattern_constant), token.immediate('('), $._hash_pattern_body, ')'),
    )),

    _pattern_expr_basic: $ => prec.right(-1, choice(
      $._pattern_value,
      $.identifier,
      $.array_pattern,
      $.find_pattern,
      $.hash_pattern,
      $.parenthesized_pattern,
    )),

    parenthesized_pattern: $ => seq('(', $._pattern_expr, ')'),

    _pattern_value: $ => prec.right(-1, choice(
      $._pattern_primitive,
      alias($._pattern_range, $.range),
      $.variable_reference_pattern,
      $.expression_reference_pattern,
      $._pattern_constant,
    )),

    _pattern_range: $ => {
      const begin = field('begin', $._pattern_primitive);
      const end = field('end', $._pattern_primitive);
      const operator = field('operator', choice('..', '...'));
      return choice(
        seq(begin, operator, end),
        seq(operator, end),
        seq(begin, operator),
      );
    },

    _pattern_primitive: $ => choice(
      $._pattern_literal,
      $._pattern_lambda,
    ),

    _pattern_lambda: $ => prec.right(-1, $.lambda),

    _pattern_literal: $ => prec.right(-1, choice(
      $._literal,
      $.string,
      $.subshell,
      $.heredoc_beginning,
      $.regex,
      $.string_array,
      $.symbol_array,
      $._keyword_variable,
    )),

    _keyword_variable: $ => prec.right(-1, choice(
      $.nil,
      $.self,
      $.true,
      $.false,
      $.line,
      $.file,
      $.encoding,
    )),

    line: _ => '__LINE__',
    file: _ => '__FILE__',
    encoding: _ => '__ENCODING__',

    variable_reference_pattern: $ => seq('^', field('name', choice($.identifier, $._nonlocal_variable))),

    expression_reference_pattern: $ => seq('^', '(', field('value', $._expression), ')'),

    _pattern_constant: $ => prec.right(-1, choice(
      $.constant,
      alias($._pattern_constant_resolution, $.scope_resolution),
    )),

    _pattern_constant_resolution: $ => seq(
      optional(field('scope', $._pattern_constant)),
      '::',
      field('name', $.constant),
    ),

    if: $ => seq(
      'if',
      field('condition', $._statement),
      choice($._terminator, field('consequence', $.then)),
      field('alternative', optional(choice($.else, $.elsif))),
      'end',
    ),

    unless: $ => seq(
      'unless',
      field('condition', $._statement),
      choice($._terminator, field('consequence', $.then)),
      field('alternative', optional(choice($.else, $.elsif))),
      'end',
    ),

    elsif: $ => seq(
      'elsif',
      field('condition', $._statement),
      choice($._terminator, field('consequence', $.then)),
      field('alternative', optional(choice($.else, $.elsif))),
    ),

    else: $ => seq(
      'else',
      optional($._terminator),
      optional($._statements),
    ),

    then: $ => choice(
      seq(
        $._terminator,
        $._statements,
      ),
      seq(
        optional($._terminator),
        'then',
        optional($._statements),
      ),
    ),

    begin: $ => seq('begin', optional($._terminator), optional($._body_statement), 'end'),

    ensure: $ => seq('ensure', optional($._statements)),

    rescue: $ => seq(
      'rescue',
      field('exceptions', optional($.exceptions)),
      field('variable', optional($.exception_variable)),
      choice(
        $._terminator,
        field('body', $.then),
      ),
    ),

    exceptions: $ => commaSep1(choice($._arg, $.splat_argument)),

    exception_variable: $ => seq('=>', $._lhs),

    body_statement: $ => $._body_statement,

    _body_statement: $ => choice(
      seq($._statements, repeat(choice($.rescue, $.else, $.ensure))),
      seq(optional($._statements), repeat1(choice($.rescue, $.else, $.ensure))),
    ),

    // Method calls without parentheses (aka "command calls") are only allowed
    // in certain positions, like the top-level of a statement, the condition
    // of a postfix control-flow operator like `if`, or as the value of a
    // control-flow statement like `return`. In many other places, they're not
    // allowed.
    //
    // Because of this distinction, a lot of rules have two variants: the
    // normal variant, which can appear anywhere that an expression is valid,
    // and the "command" varaint, which is only valid in a more limited set of
    // positions, because it can contain "command calls".
    //
    // The `_expression` rule can appear in relatively few places, but can
    // contain command calls. The `_arg` rule can appear in many more places,
    // but cannot contain command calls (unless they are wrapped in parens).
    // This naming convention is based on Ruby's standard grammar.
    _expression: $ => choice(
      alias($.command_binary, $.binary),
      alias($.command_unary, $.unary),
      alias($.command_assignment, $.assignment),
      alias($.command_operator_assignment, $.operator_assignment),
      alias($.command_call, $.call),
      alias($.command_call_with_block, $.call),
      prec.left(alias($._chained_command_call, $.call)),
      alias($.return_command, $.return),
      alias($.yield_command, $.yield),
      alias($.break_command, $.break),
      alias($.next_command, $.next),
      $.match_pattern,
      $.test_pattern,
      $._arg,
    ),

    match_pattern: $ => prec(100, seq(field('value', $._arg), '=>', field('pattern', $._pattern_top_expr_body))),

    test_pattern: $ => prec(100, seq(field('value', $._arg), 'in', field('pattern', $._pattern_top_expr_body))),

    _arg: $ => choice(
      alias($._unary_minus_pow, $.unary),
      $._primary,
      $.assignment,
      $.operator_assignment,
      $.conditional,
      $.range,
      $.binary,
      $.unary,
    ),

    _unary_minus_pow: $ => seq(field('operator', alias($._unary_minus_num, '-')), field('operand', alias($._pow, $.binary))),
    _pow: $ => prec.right(PREC.EXPONENTIAL, seq(field('left', $._simple_numeric), field('operator', alias($._binary_star_star, '**')), field('right', $._arg))),

    _primary: $ => choice(
      $.parenthesized_statements,
      $._lhs,
      alias($._function_identifier_call, $.call),
      $.call,
      $.array,
      $.string_array,
      $.symbol_array,
      $.hash,
      $.subshell,
      $._literal,
      $.string,
      $.character,
      $.chained_string,
      $.regex,
      $.lambda,
      $.method,
      $.singleton_method,
      $.class,
      $.singleton_class,
      $.module,
      $.begin,
      $.while,
      $.until,
      $.if,
      $.unless,
      $.for,
      $.case,
      $.case_match,
      $.return,
      $.yield,
      $.break,
      $.next,
      $.redo,
      $.retry,
      alias($.parenthesized_unary, $.unary),
      $.heredoc_beginning,
    ),

    parenthesized_statements: $ => seq('(', optional($._statements), ')'),

    element_reference: $ => prec.left(1, seq(
      field('object', $._primary),
      alias($._element_reference_bracket, '['),
      optional($._argument_list_with_trailing_comma),
      ']',
      optional(field('block', choice($.block, $.do_block))),
    )),

    scope_resolution: $ => prec.left(PREC.CALL + 1, seq(
      choice(
        '::',
        seq(field('scope', $._primary), token.immediate('::')),
      ),
      field('name', $.constant),
    )),

    _call_operator: _ => choice('.', '&.', token.immediate('::')),
    _call: $ => prec.left(PREC.CALL, seq(
      field('receiver', $._primary),
      field('operator', $._call_operator),
      field('method', choice($.identifier, $.operator, $.constant, $._function_identifier)),
    )),

    command_call: $ => seq(
      choice(
        $._call,
        $._chained_command_call,
        field('method', choice(
          $._variable,
          $._function_identifier,
        )),
      ),
      field('arguments', alias($.command_argument_list, $.argument_list)),
    ),

    command_call_with_block: $ => {
      const receiver = choice(
        $._call,
        field('method', choice($._variable, $._function_identifier)),
      );
      const args = field('arguments', alias($.command_argument_list, $.argument_list));
      const block = field('block', $.block);
      const doBlock = field('block', $.do_block);
      return choice(
        seq(receiver, prec(PREC.CURLY_BLOCK, seq(args, block))),
        seq(receiver, prec(PREC.DO_BLOCK, seq(args, doBlock))),
      );
    },

    _chained_command_call: $ => seq(
      field('receiver', alias($.command_call_with_block, $.call)),
      field('operator', $._call_operator),
      field('method', choice($.identifier, $._function_identifier, $.operator, $.constant)),
    ),

    call: $ => {
      const receiver = choice(
        $._call,
        field('method', choice(
          $._variable, $._function_identifier,
        )),
      );

      const args = field('arguments', $.argument_list);
      const receiverArguments =
        seq(
          choice(
            receiver,
            prec.left(PREC.CALL, seq(
              field('receiver', $._primary),
              field('operator', $._call_operator),
            )),
          ),
          args,
        );

      const block = field('block', $.block);
      const doBlock = field('block', $.do_block);
      return choice(
        receiverArguments,
        prec(PREC.CURLY_BLOCK, seq(receiverArguments, block)),
        prec(PREC.DO_BLOCK, seq(receiverArguments, doBlock)),
        prec(PREC.CURLY_BLOCK, seq(receiver, block)),
        prec(PREC.DO_BLOCK, seq(receiver, doBlock)),
      );
    },

    command_argument_list: $ => prec.right(commaSep1($._argument)),

    argument_list: $ => prec.right(seq(
      token.immediate('('),
      optional($._argument_list_with_trailing_comma),
      ')',
    )),

    _argument_list_with_trailing_comma: $ => prec.right(seq(
      commaSep1($._argument),
      optional(','),
    )),

    _argument: $ => prec.left(choice(
      $._expression,
      $.splat_argument,
      $.hash_splat_argument,
      $.forward_argument,
      $.block_argument,
      $.pair,
    )),

    forward_argument: _ => '...',
    splat_argument: $ => prec.right(seq(alias($._splat_star, '*'), optional($._arg))),
    hash_splat_argument: $ => prec.right(seq(alias($._hash_splat_star_star, '**'), optional($._arg))),
    block_argument: $ => prec.right(seq(alias($._block_ampersand, '&'), optional($._arg))),

    do_block: $ => seq(
      'do',
      optional($._terminator),
      optional(seq(
        field('parameters', $.block_parameters),
        optional($._terminator),
      )),
      optional(field('body', $.body_statement)),
      'end',
    ),

    block: $ => prec(PREC.CURLY_BLOCK, seq(
      '{',
      field('parameters', optional($.block_parameters)),
      optional(field('body', $.block_body)),
      '}',
    )),

    _arg_rhs: $ => choice($._arg, alias($.rescue_modifier_arg, $.rescue_modifier)),
    assignment: $ => prec.right(PREC.ASSIGN, choice(
      seq(
        field('left', choice($._lhs, $.left_assignment_list)),
        '=',
        field('right', choice(
          $._arg_rhs,
          $.splat_argument,
          $.right_assignment_list,
        )),
      ),
    )),

    command_assignment: $ => prec.right(PREC.ASSIGN,
      seq(
        field('left', choice($._lhs, $.left_assignment_list)),
        '=',
        field('right', choice($._expression, alias($.rescue_modifier_expression, $.rescue_modifier))),
      ),
    ),

    operator_assignment: $ => prec.right(PREC.ASSIGN, seq(
      field('left', $._lhs),
      field('operator', choice('+=', '-=', '*=', '**=', '/=', '||=', '|=', '&&=', '&=', '%=', '>>=', '<<=', '^=')),
      field('right', $._arg_rhs),
    )),

    command_operator_assignment: $ => prec.right(PREC.ASSIGN, seq(
      field('left', $._lhs),
      field('operator', choice('+=', '-=', '*=', '**=', '/=', '||=', '|=', '&&=', '&=', '%=', '>>=', '<<=', '^=')),
      field('right', choice($._expression, alias($.rescue_modifier_expression, $.rescue_modifier))),
    )),

    conditional: $ => prec.right(PREC.CONDITIONAL, seq(
      field('condition', $._arg),
      '?',
      field('consequence', $._arg),
      ':',
      field('alternative', $._arg),
    )),

    range: $ => {
      const begin = field('begin', $._arg);
      const end = field('end', $._arg);
      const operator = field('operator', choice('..', '...'));
      return prec.right(PREC.RANGE, choice(
        seq(begin, operator, end),
        seq(operator, end),
        seq(begin, operator),
      ));
    },

    binary: $ => {
      const operators = [
        [prec.left, PREC.AND, 'and'],
        [prec.left, PREC.OR, 'or'],
        [prec.left, PREC.BOOLEAN_OR, '||'],
        [prec.left, PREC.BOOLEAN_AND, '&&'],
        [prec.left, PREC.SHIFT, choice('<<', '>>')],
        [prec.left, PREC.COMPARISON, choice('<', '<=', '>', '>=')],
        [prec.left, PREC.BITWISE_AND, '&'],
        [prec.left, PREC.BITWISE_OR, choice('^', '|')],
        [prec.left, PREC.ADDITIVE, choice('+', alias($._binary_minus, '-'))],
        [prec.left, PREC.MULTIPLICATIVE, choice('/', '%', alias($._binary_star, '*'))],
        [prec.right, PREC.RELATIONAL, choice('==', '!=', '===', '<=>', '=~', '!~')],
        [prec.right, PREC.EXPONENTIAL, alias($._binary_star_star, '**')],
      ];

      // @ts-ignore
      return choice(...operators.map(([fn, precedence, operator]) => fn(precedence, seq(
        field('left', $._arg),
        // @ts-ignore
        field('operator', operator),
        field('right', $._arg),
      ))));
    },

    command_binary: $ => prec.left(seq(
      field('left', $._expression),
      field('operator', choice('or', 'and')),
      field('right', $._expression),
    )),

    unary: $ => {
      const operators = [
        [prec, PREC.DEFINED, 'defined?'],
        [prec.right, PREC.NOT, 'not'],
        [prec.right, PREC.UNARY_MINUS, choice(alias($._unary_minus, '-'), alias($._binary_minus, '-'), '+')],
        [prec.right, PREC.COMPLEMENT, choice('!', '~')],
      ];
      // @ts-ignore
      return choice(...operators.map(([fn, precedence, operator]) => fn(precedence, seq(
        // @ts-ignore
        field('operator', operator),
        field('operand', $._arg),
      ))));
    },

    command_unary: $ => {
      const operators = [
        [prec, PREC.DEFINED, 'defined?'],
        [prec.right, PREC.NOT, 'not'],
        [prec.right, PREC.UNARY_MINUS, choice(alias($._unary_minus, '-'), '+')],
        [prec.right, PREC.COMPLEMENT, choice('!', '~')],
      ];
      // @ts-ignore
      return choice(...operators.map(([fn, precedence, operator]) => fn(precedence, seq(
        // @ts-ignore
        field('operator', operator),
        field('operand', $._expression),
      ))));
    },

    parenthesized_unary: $ => prec(PREC.CALL, seq(
      field('operator', choice('defined?', 'not')),
      field('operand', $.parenthesized_statements),
    )),

    unary_literal: $ => prec.right(PREC.UNARY_MINUS, seq(
      field('operator', choice(alias($._unary_minus_num, '-'), '+')),
      field('operand', $._simple_numeric),
    )),

    _literal: $ => choice(
      $.simple_symbol,
      $.delimited_symbol,
      $._numeric,
    ),

    _numeric: $ => choice(
      $._simple_numeric,
      alias($.unary_literal, $.unary),
    ),

    _simple_numeric: $ =>
      choice(
        $.integer,
        $.float,
        $.complex,
        $.rational,
      ),

    right_assignment_list: $ => prec(-1, commaSep1(choice($._arg, $.splat_argument))),

    left_assignment_list: $ => $._mlhs,
    _mlhs: $ => prec.left(-1, seq(
      commaSep1(choice($._lhs, $.rest_assignment, $.destructured_left_assignment)),
      optional(','),
    )),
    destructured_left_assignment: $ => prec(-1, seq('(', $._mlhs, ')')),

    rest_assignment: $ => prec(-1, seq('*', optional($._lhs))),

    _function_identifier: $ => choice(alias($.identifier_suffix, $.identifier), alias($.constant_suffix, $.constant)),
    _function_identifier_call: $ => prec.left(field('method', $._function_identifier)),
    _lhs: $ => prec.left(choice(
      $._variable,
      $.true,
      $.false,
      $.nil,
      $.scope_resolution,
      $.element_reference,
      alias($._call, $.call),
    )),

    _variable: $ => prec.right(choice(
      $.self,
      $.super,
      $._nonlocal_variable,
      $.identifier,
      $.constant,
    )),

    operator: _ => choice(
      '..', '|', '^', '&', '<=>', '==', '===', '=~', '>', '>=', '<', '<=', '+', '!=',
      '-', '*', '/', '%', '!', '!~', '**', '<<', '>>', '~', '+@', '-@', '~@', '[]', '[]=', '`',
    ),

    _method_name: $ => choice(
      $.identifier,
      $._function_identifier,
      $.constant,
      $.setter,
      $.simple_symbol,
      $.delimited_symbol,
      $.operator,
      $._nonlocal_variable,
    ),

    _nonlocal_variable: $ => choice(
      $.instance_variable,
      $.class_variable,
      $.global_variable,
    ),

    setter: $ => seq(field('name', $.identifier), token.immediate('=')),

    undef: $ => seq('undef', commaSep1($._method_name)),
    alias: $ => seq(
      'alias',
      field('name', $._method_name),
      field('alias', $._method_name),
    ),

    comment: _ => token(prec(PREC.COMMENT, choice(
      seq('#', /.*/),
      seq(
        /=begin.*\r?\n/,
        repeat(choice(
          /[^=]/,
          /=[^e]/,
          /=e[^n]/,
          /=en[^d]/,
        )),
        /[\s*]*=end.*/,
      ),
    ))),

    integer: _ => /0[bB][01](_?[01])*|0[oO]?[0-7](_?[0-7])*|(0[dD])?\d(_?\d)*|0[xX][0-9a-fA-F](_?[0-9a-fA-F])*/,
    _int_or_float: $ => choice($.integer, $.float),
    float: _ => /\d(_?\d)*(\.\d)?(_?\d)*([eE][\+-]?\d(_?\d)*)?/,
    complex: $ => choice(
      seq($._int_or_float, token.immediate('i')),
      seq(alias($._int_or_float, $.rational), token.immediate('ri')),
    ),
    rational: $ => seq($._int_or_float, token.immediate('r')),
    super: _ => 'super',
    self: _ => 'self',
    true: _ => 'true',
    false: _ => 'false',
    nil: _ => 'nil',

    constant: _ => token(seq(/[A-Z]/, IDENTIFIER_CHARS)),
    constant_suffix: $ => choice(token(seq(/[A-Z]/, IDENTIFIER_CHARS, /[?]/)), $._constant_suffix),
    identifier: _ => token(seq(LOWER_ALPHA_CHAR, IDENTIFIER_CHARS)),
    identifier_suffix: $ => choice(token(seq(LOWER_ALPHA_CHAR, IDENTIFIER_CHARS, /[?]/)), $._identifier_suffix),
    instance_variable: _ => token(seq('@', ALPHA_CHAR, IDENTIFIER_CHARS)),
    class_variable: _ => token(seq('@@', ALPHA_CHAR, IDENTIFIER_CHARS)),

    global_variable: _ => /\$(-[a-zA-Z0-9_]|[!@&`'+~=/\\,;.<>*$?:"]|[0-9]+|[a-zA-Z_][a-zA-Z0-9_]*)/,

    chained_string: $ => seq($.string, repeat1($.string)),

    character: _ => /\?(\\\S(\{[0-9A-Fa-f]*\}|[0-9A-Fa-f]*|-\S([MC]-\S)?)?|\S)/,

    interpolation: $ => choice(
      seq('#{', optional($._statements), '}'),
      seq($._short_interpolation, $._nonlocal_variable),
    ),

    string: $ => seq(
      alias($._string_start, '"'),
      optional($._literal_contents),
      alias($._string_end, '"'),
    ),

    subshell: $ => seq(
      alias($._subshell_start, '`'),
      optional($._literal_contents),
      alias($._string_end, '`'),
    ),

    string_array: $ => seq(
      alias($._string_array_start, '%w('),
      optional(/\s+/),
      sep(alias($._literal_contents, $.bare_string), /\s+/),
      optional(/\s+/),
      alias($._string_end, ')'),
    ),

    symbol_array: $ => seq(
      alias($._symbol_array_start, '%i('),
      optional(/\s+/),
      sep(alias($._literal_contents, $.bare_symbol), /\s+/),
      optional(/\s+/),
      alias($._string_end, ')'),
    ),

    delimited_symbol: $ => seq(
      alias($._symbol_start, ':"'),
      optional($._literal_contents),
      alias($._string_end, '"'),
    ),

    regex: $ => seq(
      alias($._regex_start, '/'),
      optional($._literal_contents),
      alias($._string_end, '/'),
    ),

    heredoc_body: $ => seq(
      $._heredoc_body_start,
      repeat(choice(
        $.heredoc_content,
        $.interpolation,
        $.escape_sequence,
      )),
      $.heredoc_end,
    ),

    _literal_contents: $ => repeat1(choice(
      $.string_content,
      $.interpolation,
      $.escape_sequence,
    )),

    // https://ruby-doc.org/core-2.5.0/doc/syntax/literals_rdoc.html#label-Strings
    escape_sequence: _ => token(seq(
      '\\',
      choice(
        /[^ux0-7]/, // single character
        /x[0-9a-fA-F]{1,2}/, // hex code
        /[0-7]{1,3}/, // octal
        /u[0-9a-fA-F]{4}/, // single unicode
        /u\{[0-9a-fA-F ]+\}/, // multiple unicode
      ),
    )),

    array: $ => seq(
      '[',
      optional($._argument_list_with_trailing_comma),
      ']',
    ),

    hash: $ => seq(
      '{',
      optional(seq(
        commaSep1(choice($.pair, $.hash_splat_argument)),
        optional(','),
      )),
      '}',
    ),

    pair: $ => prec.right(choice(
      seq(
        field('key', $._arg),
        '=>',
        field('value', $._arg),
      ),
      seq(
        field('key', choice(
          $.string,
        )),
        token.immediate(':'),
        field('value', $._arg),
      ),
      seq(
        field('key', choice(
          $.hash_key_symbol,
          alias($.identifier, $.hash_key_symbol),
          alias($.constant, $.hash_key_symbol),
          alias($.identifier_suffix, $.hash_key_symbol),
          alias($.constant_suffix, $.hash_key_symbol),
        )),
        token.immediate(':'),
        choice(
          field('value', optional($._arg)),
          // This alternative never matches, because '_no_line_break' tokens do not exist.
          // The purpose is give a hint to the scanner that it should not produce any line-break
          // terminators at this point.
          $._no_line_break),
      ),
    )),

    lambda: $ => seq(
      '->',
      field('parameters', optional(choice(
        alias($.parameters, $.lambda_parameters),
        alias($.bare_parameters, $.lambda_parameters),
      ))),
      field('body', choice($.block, $.do_block)),
    ),

    empty_statement: _ => prec(-1, ';'),

    _terminator: $ => choice(
      $._line_break,
      ';',
    ),
  },
});

/**
 * Creates a rule to optionally match one or more of the rules separated by `separator`
 *
 * @param {RuleOrLiteral} rule
 *
 * @param {RuleOrLiteral} separator
 *
 * @returns {ChoiceRule}
 */
function sep(rule, separator) {
  return optional(sep1(rule, separator));
}

/**
 * Creates a rule to match one or more of the rules separated by `separator`
 *
 * @param {RuleOrLiteral} rule
 *
 * @param {RuleOrLiteral} separator
 *
 * @returns {SeqRule}
 */
function sep1(rule, separator) {
  return seq(rule, repeat(seq(separator, rule)));
}

/**
 * Creates a rule to match one or more of the rules separated by a comma
 *
 * @param {RuleOrLiteral} rule
 *
 * @returns {SeqRule}
 */
function commaSep1(rule) {
  return sep1(rule, ',');
}

/**
 * Creates a rule to optionally match one or more of the rules separated by a comma
 *
 * @param {RuleOrLiteral} rule
 *
 * @returns {ChoiceRule}
 */
function commaSep(rule) {
  return optional(commaSep1(rule));
}
