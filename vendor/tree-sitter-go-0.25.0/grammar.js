/**
 * @file Go grammar for tree-sitter
 * @author Max Brunsfeld <maxbrunsfeld@gmail.com>
 * @author Amaan Qureshi <amaanq12@gmail.com>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

const PREC = {
  primary: 7,
  unary: 6,
  multiplicative: 5,
  additive: 4,
  comparative: 3,
  and: 2,
  or: 1,
  composite_literal: -1,
};

const multiplicativeOperators = ['*', '/', '%', '<<', '>>', '&', '&^'];
const additiveOperators = ['+', '-', '|', '^'];
const comparativeOperators = ['==', '!=', '<', '<=', '>', '>='];
const assignmentOperators = multiplicativeOperators.concat(additiveOperators).map(operator => operator + '=').concat('=');


const newline = /\n/;
const terminator = choice(newline, ';', '\0');

const hexDigit = /[0-9a-fA-F]/;
const octalDigit = /[0-7]/;
const decimalDigit = /[0-9]/;
const binaryDigit = /[01]/;

const hexDigits = seq(hexDigit, repeat(seq(optional('_'), hexDigit)));
const octalDigits = seq(octalDigit, repeat(seq(optional('_'), octalDigit)));
const decimalDigits = seq(decimalDigit, repeat(seq(optional('_'), decimalDigit)));
const binaryDigits = seq(binaryDigit, repeat(seq(optional('_'), binaryDigit)));

const hexLiteral = seq('0', choice('x', 'X'), optional('_'), hexDigits);
const octalLiteral = seq('0', optional(choice('o', 'O')), optional('_'), octalDigits);
const decimalLiteral = choice('0', seq(/[1-9]/, optional(seq(optional('_'), decimalDigits))));
const binaryLiteral = seq('0', choice('b', 'B'), optional('_'), binaryDigits);

const intLiteral = choice(binaryLiteral, decimalLiteral, octalLiteral, hexLiteral);

const decimalExponent = seq(choice('e', 'E'), optional(choice('+', '-')), decimalDigits);
const decimalFloatLiteral = choice(
  seq(decimalDigits, '.', optional(decimalDigits), optional(decimalExponent)),
  seq(decimalDigits, decimalExponent),
  seq('.', decimalDigits, optional(decimalExponent)),
);

const hexExponent = seq(choice('p', 'P'), optional(choice('+', '-')), decimalDigits);
const hexMantissa = choice(
  seq(optional('_'), hexDigits, '.', optional(hexDigits)),
  seq(optional('_'), hexDigits),
  seq('.', hexDigits),
);
const hexFloatLiteral = seq('0', choice('x', 'X'), hexMantissa, hexExponent);

const floatLiteral = choice(decimalFloatLiteral, hexFloatLiteral);

const imaginaryLiteral = seq(choice(decimalDigits, intLiteral, floatLiteral), 'i');

module.exports = grammar({
  name: 'go',

  extras: $ => [
    $.comment,
    /\s/,
  ],

  inline: $ => [
    $._type,
    $._type_identifier,
    $._field_identifier,
    $._package_identifier,
    $._top_level_declaration,
    $._string_literal,
    $._interface_elem,
  ],

  word: $ => $.identifier,

  conflicts: $ => [
    [$._simple_type, $._expression],
    [$._simple_type, $.generic_type, $._expression],
    [$.qualified_type, $._expression],
    [$.generic_type, $._simple_type],
    [$.parameter_declaration, $._simple_type],
    [$.type_parameter_declaration, $._simple_type, $._expression],
    [$.type_parameter_declaration, $._expression],
    [$.type_parameter_declaration, $._simple_type, $.generic_type, $._expression],
  ],

  reserved: {
    global: $ => [
      // https://go.dev/ref/spec#Keywords
      'break', 'default', 'func', 'interface', 'select',
      'case', 'defer', 'go', 'map', 'struct',
      'chan', 'else', 'goto', 'package', 'switch',
      'const', 'fallthrough', 'if', 'range', 'type',
      'continue', 'for', 'import', 'return', 'var',
    ],
  },

  supertypes: $ => [
    $._expression,
    $._type,
    $._simple_type,
    $._statement,
    $._simple_statement,
  ],

  rules: {
    source_file: $ => seq(
      repeat(choice(
        // Unlike a Go compiler, we accept statements at top-level to enable
        // parsing of partial code snippets in documentation (see #63).
        seq($._statement, terminator),
        seq($._top_level_declaration, terminator),
      )),
      optional($._top_level_declaration),
    ),

    _top_level_declaration: $ => choice(
      $.package_clause,
      $.function_declaration,
      $.method_declaration,
      $.import_declaration,
    ),

    package_clause: $ => seq(
      'package',
      $._package_identifier,
    ),

    import_declaration: $ => seq(
      'import',
      choice(
        $.import_spec,
        $.import_spec_list,
      ),
    ),

    import_spec: $ => seq(
      optional(field('name', choice(
        $.dot,
        $.blank_identifier,
        $._package_identifier,
      ))),
      field('path', $._string_literal),
    ),
    dot: _ => '.',
    blank_identifier: _ => '_',

    import_spec_list: $ => seq(
      '(',
      optional(seq(
        $.import_spec,
        repeat(seq(terminator, $.import_spec)),
        optional(terminator),
      )),
      ')',
    ),

    _declaration: $ => choice(
      $.const_declaration,
      $.type_declaration,
      $.var_declaration,
    ),

    const_declaration: $ => seq(
      'const',
      choice(
        $.const_spec,
        seq(
          '(',
          repeat(seq($.const_spec, terminator)),
          ')',
        ),
      ),
    ),

    const_spec: $ => prec.left(seq(
      field('name', commaSep1($.identifier)),
      optional(seq(
        optional(field('type', $._type)),
        '=',
        field('value', $.expression_list),
      )),
    )),

    var_declaration: $ => seq(
      'var',
      choice(
        $.var_spec,
        $.var_spec_list,
      ),
    ),

    var_spec: $ => seq(
      commaSep1(field('name', $.identifier)),
      choice(
        seq(
          field('type', $._type),
          optional(seq('=', field('value', $.expression_list))),
        ),
        seq('=', field('value', $.expression_list)),
      ),
    ),

    var_spec_list: $ => seq(
      '(',
      repeat(seq($.var_spec, terminator)),
      ')',
    ),

    function_declaration: $ => prec.right(1, seq(
      'func',
      field('name', $.identifier),
      field('type_parameters', optional($.type_parameter_list)),
      field('parameters', $.parameter_list),
      field('result', optional(choice($.parameter_list, $._simple_type))),
      field('body', optional($.block)),
    )),

    method_declaration: $ => prec.right(1, seq(
      'func',
      field('receiver', $.parameter_list),
      field('name', $._field_identifier),
      field('parameters', $.parameter_list),
      field('result', optional(choice($.parameter_list, $._simple_type))),
      field('body', optional($.block)),
    )),

    type_parameter_list: $ => seq(
      '[',
      commaSep1($.type_parameter_declaration),
      optional(','),
      ']',
    ),

    type_parameter_declaration: $ => seq(
      commaSep1(field('name', $.identifier)),
      field('type', alias($.type_elem, $.type_constraint)),
    ),

    parameter_list: $ => seq(
      '(',
      optional(seq(
        commaSep(choice($.parameter_declaration, $.variadic_parameter_declaration)),
        optional(','),
      )),
      ')',
    ),

    parameter_declaration: $ => seq(
      commaSep(field('name', $.identifier)),
      field('type', $._type),
    ),

    variadic_parameter_declaration: $ => seq(
      field('name', optional($.identifier)),
      '...',
      field('type', $._type),
    ),

    type_alias: $ => seq(
      field('name', $._type_identifier),
      field('type_parameters', optional($.type_parameter_list)),
      '=',
      field('type', $._type),
    ),

    type_declaration: $ => seq(
      'type',
      choice(
        $.type_spec,
        $.type_alias,
        seq(
          '(',
          optionalTrailingSep(choice($.type_spec, $.type_alias), terminator),
          ')',
        ),
      ),
    ),

    type_spec: $ => seq(
      field('name', $._type_identifier),
      field('type_parameters', optional($.type_parameter_list)),
      field('type', $._type),
    ),

    field_name_list: $ => commaSep1($._field_identifier),

    expression_list: $ => commaSep1($._expression),

    _type: $ => choice(
      $._simple_type,
      $.parenthesized_type,
    ),

    parenthesized_type: $ => seq('(', $._type, ')'),

    _simple_type: $ => choice(
      prec.dynamic(-1, $._type_identifier),
      $.generic_type,
      $.qualified_type,
      $.pointer_type,
      $.struct_type,
      $.interface_type,
      $.array_type,
      $.slice_type,
      prec.dynamic(3, $.map_type),
      $.channel_type,
      $.function_type,
      $.negated_type,
    ),

    generic_type: $ => prec.dynamic(1, seq(
      field('type', choice($._type_identifier, $.qualified_type, $.negated_type)),
      field('type_arguments', $.type_arguments),
    )),

    type_arguments: $ => prec.dynamic(2, seq(
      '[',
      commaSep1($.type_elem),
      optional(','),
      ']',
    )),

    pointer_type: $ => prec(PREC.unary, seq('*', $._type)),

    array_type: $ => prec.right(seq(
      '[',
      field('length', $._expression),
      ']',
      field('element', $._type),
    )),

    implicit_length_array_type: $ => seq(
      '[',
      '...',
      ']',
      field('element', $._type),
    ),

    slice_type: $ => prec.right(seq(
      '[',
      ']',
      field('element', $._type),
    )),

    struct_type: $ => seq(
      'struct',
      $.field_declaration_list,
    ),

    negated_type: $ => prec.left(seq(
      '~',
      $._type,
    )),

    field_declaration_list: $ => seq(
      '{',
      optional(seq(
        $.field_declaration,
        repeat(seq(terminator, $.field_declaration)),
        optional(terminator),
      )),
      '}',
    ),

    field_declaration: $ => seq(
      choice(
        seq(
          commaSep1(field('name', $._field_identifier)),
          field('type', $._type),
        ),
        seq(
          optional('*'),
          field('type', choice(
            $._type_identifier,
            $.qualified_type,
            $.generic_type,
          )),
        ),
      ),
      field('tag', optional($._string_literal)),
    ),

    interface_type: $ => seq(
      'interface',
      '{',
      optional(seq(
        $._interface_elem,
        repeat(seq(terminator, $._interface_elem)),
        optional(terminator),
      )),
      '}',
    ),

    _interface_elem: $ => choice(
      $.method_elem,
      $.type_elem,
    ),

    method_elem: $ => seq(
      field('name', $._field_identifier),
      field('parameters', $.parameter_list),
      field('result', optional(choice($.parameter_list, $._simple_type))),
    ),

    type_elem: $ => sep1($._type, '|'),

    map_type: $ => prec.right(seq(
      'map',
      '[',
      field('key', $._type),
      ']',
      field('value', $._type),
    )),

    channel_type: $ => prec.left(choice(
      seq('chan', field('value', $._type)),
      seq('chan', '<-', field('value', $._type)),
      prec(PREC.unary, seq('<-', 'chan', field('value', $._type))),
    )),

    function_type: $ => prec.right(seq(
      'func',
      field('parameters', $.parameter_list),
      field('result', optional(choice($.parameter_list, $._simple_type))),
    )),

    block: $ => seq(
      '{',
      optional($.statement_list),
      '}',
    ),

    statement_list: $ => choice(
      seq(
        $._statement,
        repeat(seq(terminator, $._statement)),
        optional(seq(
          terminator,
          optional(alias($.empty_labeled_statement, $.labeled_statement)),
        )),
      ),
      alias($.empty_labeled_statement, $.labeled_statement),
    ),

    _statement: $ => choice(
      $._declaration,
      $._simple_statement,
      $.return_statement,
      $.go_statement,
      $.defer_statement,
      $.if_statement,
      $.for_statement,
      $.expression_switch_statement,
      $.type_switch_statement,
      $.select_statement,
      $.labeled_statement,
      $.fallthrough_statement,
      $.break_statement,
      $.continue_statement,
      $.goto_statement,
      $.block,
      $.empty_statement,
    ),

    empty_statement: _ => ';',

    _simple_statement: $ => choice(
      $.expression_statement,
      $.send_statement,
      $.inc_statement,
      $.dec_statement,
      $.assignment_statement,
      $.short_var_declaration,
    ),

    expression_statement: $ => $._expression,

    send_statement: $ => seq(
      field('channel', $._expression),
      '<-',
      field('value', $._expression),
    ),

    receive_statement: $ => seq(
      optional(seq(
        field('left', $.expression_list),
        choice('=', ':='),
      )),
      field('right', $._expression),
    ),

    inc_statement: $ => seq(
      $._expression,
      '++',
    ),

    dec_statement: $ => seq(
      $._expression,
      '--',
    ),

    assignment_statement: $ => seq(
      field('left', $.expression_list),
      field('operator', choice(...assignmentOperators)),
      field('right', $.expression_list),
    ),

    short_var_declaration: $ => seq(
      // TODO: this should really only allow identifier lists, but that causes
      // conflicts between identifiers as expressions vs identifiers here.
      field('left', $.expression_list),
      ':=',
      field('right', $.expression_list),
    ),

    labeled_statement: $ => seq(
      field('label', alias($.identifier, $.label_name)),
      ':',
      $._statement,
    ),

    empty_labeled_statement: $ => seq(
      field('label', alias($.identifier, $.label_name)),
      ':',
    ),

    // This is a hack to prevent `fallthrough_statement` from being parsed as
    // a single token. For consistency with `break_statement` etc it should
    // be parsed as a parent node that *contains* a `fallthrough` token.
    fallthrough_statement: _ => prec.left('fallthrough'),

    break_statement: $ => seq('break', optional(alias($.identifier, $.label_name))),

    continue_statement: $ => seq('continue', optional(alias($.identifier, $.label_name))),

    goto_statement: $ => seq('goto', alias($.identifier, $.label_name)),

    return_statement: $ => seq('return', optional($.expression_list)),

    go_statement: $ => seq('go', $._expression),

    defer_statement: $ => seq('defer', $._expression),

    if_statement: $ => seq(
      'if',
      optional(seq(
        field('initializer', $._simple_statement),
        ';',
      )),
      field('condition', $._expression),
      field('consequence', $.block),
      optional(seq(
        'else',
        field('alternative', choice($.block, $.if_statement)),
      )),
    ),

    for_statement: $ => seq(
      'for',
      optional(choice($._expression, $.for_clause, $.range_clause)),
      field('body', $.block),
    ),

    for_clause: $ => seq(
      field('initializer', optional($._simple_statement)),
      ';',
      field('condition', optional($._expression)),
      ';',
      field('update', optional($._simple_statement)),
    ),

    range_clause: $ => seq(
      optional(seq(
        field('left', $.expression_list),
        choice('=', ':='),
      )),
      'range',
      field('right', $._expression),
    ),

    expression_switch_statement: $ => seq(
      'switch',
      optional(seq(
        field('initializer', $._simple_statement),
        ';',
      )),
      field('value', optional($._expression)),
      '{',
      repeat(choice($.expression_case, $.default_case)),
      '}',
    ),

    expression_case: $ => seq(
      'case',
      field('value', $.expression_list),
      ':',
      optional($.statement_list),
    ),

    default_case: $ => seq(
      'default',
      ':',
      optional($.statement_list),
    ),

    type_switch_statement: $ => seq(
      'switch',
      $._type_switch_header,
      '{',
      repeat(choice($.type_case, $.default_case)),
      '}',
    ),

    _type_switch_header: $ => seq(
      optional(seq(
        field('initializer', $._simple_statement),
        ';',
      )),
      optional(seq(field('alias', $.expression_list), ':=')),
      field('value', $._expression),
      '.',
      '(',
      'type',
      ')',
    ),

    type_case: $ => seq(
      'case',
      field('type', commaSep1($._type)),
      ':',
      optional($.statement_list),
    ),

    select_statement: $ => seq(
      'select',
      '{',
      repeat(choice($.communication_case, $.default_case)),
      '}',
    ),

    communication_case: $ => seq(
      'case',
      field('communication', choice($.send_statement, $.receive_statement)),
      ':',
      optional($.statement_list),
    ),

    _expression: $ => choice(
      $.unary_expression,
      $.binary_expression,
      $.selector_expression,
      $.index_expression,
      $.slice_expression,
      $.call_expression,
      $.type_assertion_expression,
      $.type_conversion_expression,
      $.type_instantiation_expression,
      $.identifier,
      alias(choice('new', 'make'), $.identifier),
      $.composite_literal,
      $.func_literal,
      $._string_literal,
      $.int_literal,
      $.float_literal,
      $.imaginary_literal,
      $.rune_literal,
      $.nil,
      $.true,
      $.false,
      $.iota,
      $.parenthesized_expression,
    ),

    parenthesized_expression: $ => seq(
      '(',
      $._expression,
      ')',
    ),

    call_expression: $ => prec(PREC.primary, choice(
      seq(
        field('function', alias(choice('new', 'make'), $.identifier)),
        field('arguments', alias($.special_argument_list, $.argument_list)),
      ),
      seq(
        field('function', $._expression),
        field('type_arguments', optional($.type_arguments)),
        field('arguments', $.argument_list),
      ),
    )),

    variadic_argument: $ => prec.right(seq(
      $._expression,
      '...',
    )),

    special_argument_list: $ => seq(
      '(',
      optional(seq(
        $._type,
        repeat(seq(',', $._expression)),
        optional(','),
      )),
      ')',
    ),

    argument_list: $ => seq(
      '(',
      optional(seq(
        choice($._expression, $.variadic_argument),
        repeat(seq(',', choice($._expression, $.variadic_argument))),
        optional(','),
      )),
      ')',
    ),

    selector_expression: $ => prec(PREC.primary, seq(
      field('operand', $._expression),
      '.',
      field('field', $._field_identifier),
    )),

    index_expression: $ => prec(PREC.primary, prec.dynamic(1, seq(
      field('operand', $._expression),
      '[',
      field('index', $._expression),
      ']',
    ))),

    slice_expression: $ => prec(PREC.primary, seq(
      field('operand', $._expression),
      '[',
      choice(
        seq(
          field('start', optional($._expression)),
          ':',
          field('end', optional($._expression)),
        ),
        seq(
          field('start', optional($._expression)),
          ':',
          field('end', $._expression),
          ':',
          field('capacity', $._expression),
        ),
      ),
      ']',
    )),

    type_assertion_expression: $ => prec(PREC.primary, seq(
      field('operand', $._expression),
      '.',
      '(',
      field('type', $._type),
      ')',
    )),

    type_conversion_expression: $ => prec.dynamic(-1, seq(
      field('type', $._type),
      '(',
      field('operand', $._expression),
      optional(','),
      ')',
    )),

    type_instantiation_expression: $ => prec.dynamic(-1, seq(
      field('type', $._type),
      '[',
      commaSep1($._type),
      optional(','),
      ']',
    )),

    composite_literal: $ => prec(PREC.composite_literal, seq(
      field('type', choice(
        $.map_type,
        $.slice_type,
        $.array_type,
        $.implicit_length_array_type,
        $.struct_type,
        $._type_identifier,
        $.generic_type,
        $.qualified_type,
      )),
      field('body', $.literal_value),
    )),

    literal_value: $ => seq(
      '{',
      optional(
        seq(
          commaSep(choice($.literal_element, $.keyed_element)),
          optional(','))),
      '}',
    ),

    literal_element: $ => choice($._expression, $.literal_value),

    // In T{k: v}, the key k may be:
    // - any expression (when T is a map, slice or array),
    // - a field identifier (when T is a struct), or
    // - a literal_element (when T is an array).
    // The first two cases cannot be distinguished without type information.
    keyed_element: $ => seq(
      field('key', $.literal_element),
      ':',
      field('value', $.literal_element),
    ),

    func_literal: $ => seq(
      'func',
      field('parameters', $.parameter_list),
      field('result', optional(choice($.parameter_list, $._simple_type))),
      field('body', $.block),
    ),

    unary_expression: $ => prec(PREC.unary, seq(
      field('operator', choice('+', '-', '!', '^', '*', '&', '<-')),
      field('operand', $._expression),
    )),

    binary_expression: $ => {
      const table = [
        [PREC.multiplicative, choice(...multiplicativeOperators)],
        [PREC.additive, choice(...additiveOperators)],
        [PREC.comparative, choice(...comparativeOperators)],
        [PREC.and, '&&'],
        [PREC.or, '||'],
      ];

      return choice(...table.map(([precedence, operator]) =>
        // @ts-ignore
        prec.left(precedence, seq(
          field('left', $._expression),
          // @ts-ignore
          field('operator', operator),
          field('right', $._expression),
        )),
      ));
    },

    qualified_type: $ => seq(
      field('package', $._package_identifier),
      '.',
      field('name', $._type_identifier),
    ),

    identifier: _ => /[_\p{XID_Start}][_\p{XID_Continue}]*/,

    _type_identifier: $ => alias($.identifier, $.type_identifier),
    _field_identifier: $ => alias($.identifier, $.field_identifier),
    _package_identifier: $ => alias($.identifier, $.package_identifier),

    _string_literal: $ => choice(
      $.raw_string_literal,
      $.interpreted_string_literal,
    ),

    raw_string_literal: $ => seq(
      '`',
      alias(token(prec(1, /[^`]*/)), $.raw_string_literal_content),
      '`',
    ),

    interpreted_string_literal: $ => seq(
      '"',
      repeat(choice(
        alias(token.immediate(prec(1, /[^"\n\\]+/)), $.interpreted_string_literal_content),
        $.escape_sequence,
      )),
      token.immediate('"'),
    ),

    escape_sequence: _ => token.immediate(seq(
      '\\',
      choice(
        /[^xuU]/,
        /\d{2,3}/,
        /x[0-9a-fA-F]{2,}/,
        /u[0-9a-fA-F]{4}/,
        /U[0-9a-fA-F]{8}/,
      ),
    )),

    int_literal: _ => token(intLiteral),

    float_literal: _ => token(floatLiteral),

    imaginary_literal: _ => token(imaginaryLiteral),

    rune_literal: _ => token(seq(
      '\'',
      choice(
        /[^'\\]/,
        seq(
          '\\',
          choice(
            seq('x', hexDigit, hexDigit),
            seq(octalDigit, octalDigit, octalDigit),
            seq('u', hexDigit, hexDigit, hexDigit, hexDigit),
            seq('U', hexDigit, hexDigit, hexDigit, hexDigit, hexDigit, hexDigit, hexDigit, hexDigit),
            seq(choice('a', 'b', 'f', 'n', 'r', 't', 'v', '\\', '\'', '"')),
          ),
        ),
      ),
      '\'',
    )),

    nil: _ => 'nil',
    true: _ => 'true',
    false: _ => 'false',
    iota: _ => 'iota',

    // http://stackoverflow.com/questions/13014947/regex-to-match-a-c-style-multiline-comment/36328890#36328890
    comment: _ => token(choice(
      seq('//', /.*/),
      seq(
        '/*',
        /[^*]*\*+([^/*][^*]*\*+)*/,
        '/',
      ),
    )),
  },
});

/**
 * Creates a rule to match one or more occurrences of `rule` separated by `sep`
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
 * Creates a rule to match zero or more of the rules separated by {sep},
 * with an optional one at the end.
 *
 * @param {RuleOrLiteral} rule
 *
 * @param {RuleOrLiteral} separator
 */
function optionalTrailingSep(rule, separator) {
  return optional(seq(rule, repeat(seq(separator, rule)), optional(separator)));
}

/**
 * Creates a rule to match one or more of the rules separated by a comma
 *
 * @param {Rule} rule
 *
 * @returns {SeqRule}
 */
function commaSep1(rule) {
  return seq(rule, repeat(seq(',', rule)));
}

/**
 * Creates a rule to optionally match one or more of the rules separated by a comma
 *
 * @param {Rule} rule
 *
 * @returns {ChoiceRule}
 */
function commaSep(rule) {
  return optional(commaSep1(rule));
}
