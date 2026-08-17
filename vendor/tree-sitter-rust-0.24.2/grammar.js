/**
 * @file Rust grammar for tree-sitter
 * @author Maxim Sokolov <maxim0xff@gmail.com>
 * @author Max Brunsfeld <maxbrunsfeld@gmail.com>
 * @author Amaan Qureshi <amaanq12@gmail.com>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

// https://doc.rust-lang.org/reference/expressions.html#expression-precedence
const PREC = {
  call: 15,
  field: 14,
  try: 13,
  unary: 12,
  cast: 11,
  multiplicative: 10,
  additive: 9,
  shift: 8,
  bitand: 7,
  bitxor: 6,
  bitor: 5,
  comparative: 4,
  and: 3,
  or: 2,
  range: 1,
  assign: 0,
  closure: -1,
};

const numericTypes = [
  'u8',
  'i8',
  'u16',
  'i16',
  'u32',
  'i32',
  'u64',
  'i64',
  'u128',
  'i128',
  'isize',
  'usize',
  'f32',
  'f64',
];

// https://doc.rust-lang.org/reference/tokens.html#punctuation
const TOKEN_TREE_NON_SPECIAL_PUNCTUATION = [
  '+', '-', '*', '/', '%', '^', '!', '&', '|', '&&', '||', '<<',
  '>>', '+=', '-=', '*=', '/=', '%=', '^=', '&=', '|=', '<<=',
  '>>=', '=', '==', '!=', '>', '<', '>=', '<=', '@', '_', '.',
  '..', '...', '..=', ',', ';', ':', '::', '->', '=>', '#', '?',
];

const primitiveTypes = numericTypes.concat(['bool', 'str', 'char']);

module.exports = grammar({
  name: 'rust',

  extras: $ => [
    /\s/,
    $.line_comment,
    $.block_comment,
  ],

  externals: $ => [
    $.string_content,
    $.string_close,
    $._raw_string_literal_start,
    $.raw_string_literal_content,
    $._raw_string_literal_end,
    $.float_literal,
    $._outer_block_doc_comment_marker,
    $._inner_block_doc_comment_marker,
    $._block_comment_content,
    $._line_doc_content,
    $._error_sentinel,
  ],

  supertypes: $ => [
    $._expression,
    $._type,
    $._literal,
    $._literal_pattern,
    $._declaration_statement,
    $._pattern,
  ],

  inline: $ => [
    $._path,
    $._type_identifier,
    $._tokens,
    $._field_identifier,
    $._non_special_token,
    $._declaration_statement,
    $._reserved_identifier,
    $._expression_ending_with_block,
  ],

  conflicts: $ => [
    // Local ambiguity due to anonymous types:
    // See https://internals.rust-lang.org/t/pre-rfc-deprecating-anonymous-parameters/3710
    [$._type, $._pattern],
    [$.unit_type, $.tuple_pattern],
    [$.scoped_identifier, $.scoped_type_identifier],
    [$.parameters, $._pattern],
    [$.parameters, $.tuple_struct_pattern],
    [$.array_expression],
    [$.visibility_modifier],
    [$.visibility_modifier, $.scoped_identifier, $.scoped_type_identifier],
    [$.foreign_mod_item, $.function_modifiers],
  ],

  word: $ => $.identifier,

  rules: {
    source_file: $ => seq(
      optional($.shebang),
      repeat($._statement),
    ),

    _statement: $ => choice(
      $.expression_statement,
      $._declaration_statement,
    ),

    empty_statement: _ => ';',

    expression_statement: $ => choice(
      seq($._expression, ';'),
      prec(1, $._expression_ending_with_block),
    ),

    _declaration_statement: $ => choice(
      $.const_item,
      $.macro_invocation,
      $.macro_definition,
      $.empty_statement,
      $.attribute_item,
      $.inner_attribute_item,
      $.mod_item,
      $.foreign_mod_item,
      $.struct_item,
      $.union_item,
      $.enum_item,
      $.type_item,
      $.function_item,
      $.function_signature_item,
      $.impl_item,
      $.trait_item,
      $.associated_type,
      $.let_declaration,
      $.use_declaration,
      $.extern_crate_declaration,
      $.static_item,
    ),

    // Section - Macro definitions

    macro_definition: $ => {
      const rules = seq(
        repeat(seq($.macro_rule, ';')),
        optional($.macro_rule),
      );

      return seq(
        'macro_rules!',
        field('name', choice(
          $.identifier,
          $._reserved_identifier,
        )),
        choice(
          seq('(', rules, ')', ';'),
          seq('[', rules, ']', ';'),
          seq('{', rules, '}'),
        ),
      );
    },

    macro_rule: $ => seq(
      field('left', $.token_tree_pattern),
      '=>',
      field('right', $.token_tree),
    ),

    _token_pattern: $ => choice(
      $.token_tree_pattern,
      $.token_repetition_pattern,
      $.token_binding_pattern,
      $.metavariable,
      $._non_special_token,
    ),

    token_tree_pattern: $ => choice(
      seq('(', repeat($._token_pattern), ')'),
      seq('[', repeat($._token_pattern), ']'),
      seq('{', repeat($._token_pattern), '}'),
    ),

    token_binding_pattern: $ => prec(1, seq(
      field('name', $.metavariable),
      ':',
      field('type', $.fragment_specifier),
    )),

    token_repetition_pattern: $ => seq(
      '$', '(', repeat($._token_pattern), ')', optional(/[^+*?]+/), choice('+', '*', '?'),
    ),

    fragment_specifier: _ => choice(
      'block', 'expr', 'expr_2021', 'ident', 'item', 'lifetime', 'literal', 'meta', 'pat',
      'pat_param', 'path', 'stmt', 'tt', 'ty', 'vis',
    ),

    _tokens: $ => choice(
      $.token_tree,
      $.token_repetition,
      $.metavariable,
      $._non_special_token,
    ),

    token_tree: $ => choice(
      seq('(', repeat($._tokens), ')'),
      seq('[', repeat($._tokens), ']'),
      seq('{', repeat($._tokens), '}'),
    ),

    token_repetition: $ => seq(
      '$', '(', repeat($._tokens), ')', optional(/[^+*?]+/), choice('+', '*', '?'),
    ),

    // Matches non-delimiter tokens common to both macro invocations and
    // definitions. This is everything except $ and metavariables (which begin
    // with $).
    _non_special_token: $ => choice(
      $._literal, $.identifier, $.mutable_specifier, $.self, $.super, $.crate,
      alias(choice(...primitiveTypes), $.primitive_type),
      prec.right(repeat1(choice(...TOKEN_TREE_NON_SPECIAL_PUNCTUATION))),
      '\'',
      'as', 'async', 'await', 'break', 'const', 'continue', 'default', 'enum', 'fn', 'for', 'gen',
      'if', 'impl', 'let', 'loop', 'match', 'mod', 'pub', 'return', 'static', 'struct', 'trait',
      'type', 'union', 'unsafe', 'use', 'where', 'while',
    ),

    // Section - Declarations

    attribute_item: $ => seq(
      '#',
      '[',
      $.attribute,
      ']',
    ),

    inner_attribute_item: $ => seq(
      '#',
      '!',
      '[',
      $.attribute,
      ']',
    ),

    attribute: $ => seq(
      $._path,
      optional(choice(
        seq('=', field('value', $._expression)),
        field('arguments', alias($.delim_token_tree, $.token_tree)),
      )),
    ),

    mod_item: $ => seq(
      optional($.visibility_modifier),
      'mod',
      field('name', $.identifier),
      choice(
        ';',
        field('body', $.declaration_list),
      ),
    ),

    foreign_mod_item: $ => seq(
      optional('unsafe'),
      $.extern_modifier,
      choice(
        ';',
        field('body', $.declaration_list),
      ),
    ),

    declaration_list: $ => seq(
      '{',
      repeat($._declaration_statement),
      '}',
    ),

    struct_item: $ => seq(
      optional($.visibility_modifier),
      'struct',
      field('name', $._type_identifier),
      field('type_parameters', optional($.type_parameters)),
      choice(
        seq(
          optional($.where_clause),
          field('body', $.field_declaration_list),
        ),
        seq(
          field('body', $.ordered_field_declaration_list),
          optional($.where_clause),
          ';',
        ),
        ';',
      ),
    ),

    union_item: $ => seq(
      optional($.visibility_modifier),
      'union',
      field('name', $._type_identifier),
      field('type_parameters', optional($.type_parameters)),
      optional($.where_clause),
      field('body', $.field_declaration_list),
    ),

    enum_item: $ => seq(
      optional($.visibility_modifier),
      'enum',
      field('name', $._type_identifier),
      field('type_parameters', optional($.type_parameters)),
      optional($.where_clause),
      field('body', $.enum_variant_list),
    ),

    enum_variant_list: $ => seq(
      '{',
      sepBy(',', seq(repeat($.attribute_item), $.enum_variant)),
      optional(','),
      '}',
    ),

    enum_variant: $ => seq(
      optional($.visibility_modifier),
      field('name', $.identifier),
      field('body', optional(choice(
        $.field_declaration_list,
        $.ordered_field_declaration_list,
      ))),
      optional(seq(
        '=',
        field('value', $._expression),
      )),
    ),

    field_declaration_list: $ => seq(
      '{',
      sepBy(',', seq(repeat($.attribute_item), $.field_declaration)),
      optional(','),
      '}',
    ),

    field_declaration: $ => seq(
      optional($.visibility_modifier),
      field('name', $._field_identifier),
      ':',
      field('type', $._type),
    ),

    ordered_field_declaration_list: $ => seq(
      '(',
      sepBy(',', seq(
        repeat($.attribute_item),
        optional($.visibility_modifier),
        field('type', $._type),
      )),
      optional(','),
      ')',
    ),

    extern_crate_declaration: $ => seq(
      optional($.visibility_modifier),
      'extern',
      $.crate,
      field('name', $.identifier),
      optional(seq(
        'as',
        field('alias', $.identifier),
      )),
      ';',
    ),

    const_item: $ => seq(
      optional($.visibility_modifier),
      'const',
      field('name', $.identifier),
      ':',
      field('type', $._type),
      optional(
        seq(
          '=',
          field('value', $._expression),
        ),
      ),
      ';',
    ),

    static_item: $ => seq(
      optional($.visibility_modifier),
      'static',

      // Not actual rust syntax, but made popular by the lazy_static crate.
      optional('ref'),

      optional($.mutable_specifier),
      field('name', $.identifier),
      ':',
      field('type', $._type),
      optional(seq(
        '=',
        field('value', $._expression),
      )),
      ';',
    ),

    type_item: $ => seq(
      optional($.visibility_modifier),
      'type',
      field('name', $._type_identifier),
      field('type_parameters', optional($.type_parameters)),
      optional($.where_clause),
      '=',
      field('type', $._type),
      optional($.where_clause),
      ';',
    ),

    function_item: $ => seq(
      optional($.visibility_modifier),
      optional($.function_modifiers),
      'fn',
      field('name', choice($.identifier, $.metavariable)),
      field('type_parameters', optional($.type_parameters)),
      field('parameters', $.parameters),
      optional(seq('->', field('return_type', $._type))),
      optional($.where_clause),
      field('body', $.block),
    ),

    function_signature_item: $ => seq(
      optional($.visibility_modifier),
      optional($.function_modifiers),
      'fn',
      field('name', choice($.identifier, $.metavariable)),
      field('type_parameters', optional($.type_parameters)),
      field('parameters', $.parameters),
      optional(seq('->', field('return_type', $._type))),
      optional($.where_clause),
      ';',
    ),

    function_modifiers: $ => repeat1(choice(
      'async',
      'default',
      'const',
      'unsafe',
      $.extern_modifier,
    )),

    where_clause: $ => prec.right(seq(
      'where',
      optional(seq(
        sepBy1(',', $.where_predicate),
        optional(','),
      )),
    )),

    where_predicate: $ => seq(
      field('left', choice(
        $.lifetime,
        $._type_identifier,
        $.scoped_type_identifier,
        $.generic_type,
        $.reference_type,
        $.pointer_type,
        $.tuple_type,
        $.array_type,
        $.higher_ranked_trait_bound,
        alias(choice(...primitiveTypes), $.primitive_type),
      )),
      field('bounds', $.trait_bounds),
    ),

    impl_item: $ => seq(
      optional('unsafe'),
      'impl',
      field('type_parameters', optional($.type_parameters)),
      optional(seq(
        optional('!'),
        field('trait', choice(
          $._type_identifier,
          $.scoped_type_identifier,
          $.generic_type,
        )),
        'for',
      )),
      field('type', $._type),
      optional($.where_clause),
      choice(field('body', $.declaration_list), ';'),
    ),

    trait_item: $ => seq(
      optional($.visibility_modifier),
      optional('unsafe'),
      'trait',
      field('name', $._type_identifier),
      field('type_parameters', optional($.type_parameters)),
      field('bounds', optional($.trait_bounds)),
      optional($.where_clause),
      field('body', $.declaration_list),
    ),

    associated_type: $ => seq(
      'type',
      field('name', $._type_identifier),
      field('type_parameters', optional($.type_parameters)),
      field('bounds', optional($.trait_bounds)),
      optional($.where_clause),
      ';',
    ),

    trait_bounds: $ => seq(
      ':',
      sepBy1('+', choice(
        $._type,
        $.lifetime,
        $.higher_ranked_trait_bound,
      )),
    ),

    higher_ranked_trait_bound: $ => seq(
      'for',
      field('type_parameters', $.type_parameters),
      field('type', $._type),
    ),

    removed_trait_bound: $ => seq(
      '?',
      $._type,
    ),

    type_parameters: $ => prec(1, seq(
      '<',
      sepBy1(',', seq(
        repeat($.attribute_item),
        choice(
          $.metavariable,
          $.type_parameter,
          $.lifetime_parameter,
          $.const_parameter,
        ),
      )),
      optional(','),
      '>',
    )),

    const_parameter: $ => seq(
      'const',
      field('name', $.identifier),
      ':',
      field('type', $._type),
      optional(
        seq(
          '=',
          field('value',
            choice(
              $.block,
              $.identifier,
              $._literal,
              $.negative_literal,
            ),
          ),
        ),
      ),
    ),

    type_parameter: $ => prec(1, seq(
      field('name', $._type_identifier),
      optional(field('bounds', $.trait_bounds)),
      optional(
        seq(
          '=',
          field('default_type', $._type),
        ),
      ),
    )),

    lifetime_parameter: $ => prec(1, seq(
      field('name', $.lifetime),
      optional(field('bounds', $.trait_bounds)),
    )),

    let_declaration: $ => seq(
      'let',
      optional($.mutable_specifier),
      field('pattern', $._pattern),
      optional(seq(
        ':',
        field('type', $._type),
      )),
      optional(seq(
        '=',
        field('value', $._expression),
      )),
      optional(seq(
        'else',
        field('alternative', $.block),
      )),
      ';',
    ),

    use_declaration: $ => seq(
      optional($.visibility_modifier),
      'use',
      field('argument', $._use_clause),
      ';',
    ),

    _use_clause: $ => choice(
      $._path,
      $.use_as_clause,
      $.use_list,
      $.scoped_use_list,
      $.use_wildcard,
    ),

    scoped_use_list: $ => seq(
      field('path', optional($._path)),
      '::',
      field('list', $.use_list),
    ),

    use_list: $ => seq(
      '{',
      sepBy(',', choice(
        $._use_clause,
      )),
      optional(','),
      '}',
    ),

    use_as_clause: $ => seq(
      field('path', $._path),
      'as',
      field('alias', $.identifier),
    ),

    use_wildcard: $ => seq(
      optional(seq(optional($._path), '::')),
      '*',
    ),

    parameters: $ => seq(
      '(',
      sepBy(',', seq(
        optional($.attribute_item),
        choice(
          $.parameter,
          $.self_parameter,
          $.variadic_parameter,
          '_',
          $._type,
        ))),
      optional(','),
      ')',
    ),

    self_parameter: $ => seq(
      optional('&'),
      optional($.lifetime),
      optional($.mutable_specifier),
      $.self,
    ),

    variadic_parameter: $ => seq(
      optional($.mutable_specifier),
      optional(seq(
        field('pattern', $._pattern),
        ':',
      )),
      '...',
    ),

    parameter: $ => seq(
      optional($.mutable_specifier),
      field('pattern', choice(
        $._pattern,
        $.self,
      )),
      ':',
      field('type', $._type),
    ),

    extern_modifier: $ => seq(
      'extern',
      optional($.string_literal),
    ),

    visibility_modifier: $ => choice(
      $.crate,
      seq(
        'pub',
        optional(seq(
          '(',
          choice(
            $.self,
            $.super,
            $.crate,
            seq('in', $._path),
          ),
          ')',
        )),
      ),
    ),

    // Section - Types

    _type: $ => choice(
      $.abstract_type,
      $.reference_type,
      $.metavariable,
      $.pointer_type,
      $.generic_type,
      $.scoped_type_identifier,
      $.tuple_type,
      $.unit_type,
      $.array_type,
      $.function_type,
      $._type_identifier,
      $.macro_invocation,
      $.never_type,
      $.dynamic_type,
      $.bounded_type,
      $.removed_trait_bound,
      alias(choice(...primitiveTypes), $.primitive_type),
    ),

    bracketed_type: $ => seq(
      '<',
      choice(
        $._type,
        $.qualified_type,
      ),
      '>',
    ),

    qualified_type: $ => seq(
      field('type', $._type),
      'as',
      field('alias', $._type),
    ),

    lifetime: $ => prec(1, seq('\'', $.identifier)),

    array_type: $ => seq(
      '[',
      field('element', $._type),
      optional(seq(
        ';',
        field('length', $._expression),
      )),
      ']',
    ),

    for_lifetimes: $ => seq(
      'for',
      '<',
      sepBy1(',', $.lifetime),
      optional(','),
      '>',
    ),

    function_type: $ => seq(
      optional($.for_lifetimes),
      prec(PREC.call, seq(
        choice(
          field('trait', choice(
            $._type_identifier,
            $.scoped_type_identifier,
          )),
          seq(
            optional($.function_modifiers),
            'fn',
          ),
        ),
        field('parameters', $.parameters),
      )),
      optional(seq('->', field('return_type', $._type))),
    ),

    tuple_type: $ => seq(
      '(',
      sepBy1(',', $._type),
      optional(','),
      ')',
    ),

    unit_type: _ => seq('(', ')'),

    generic_function: $ => prec(1, seq(
      field('function', choice(
        $.identifier,
        $.scoped_identifier,
        $.field_expression,
      )),
      '::',
      field('type_arguments', $.type_arguments),
    )),

    generic_type: $ => prec(1, seq(
      field('type', choice(
        $._type_identifier,
        $._reserved_identifier,
        $.scoped_type_identifier,
      )),
      field('type_arguments', $.type_arguments),
    )),

    generic_type_with_turbofish: $ => seq(
      field('type', choice(
        $._type_identifier,
        $.scoped_identifier,
      )),
      '::',
      field('type_arguments', $.type_arguments),
    ),

    bounded_type: $ => prec.left(-1, seq(
      choice($.lifetime, $._type, $.use_bounds),
      '+',
      choice($.lifetime, $._type, $.use_bounds),
    )),

    use_bounds: $ => seq(
      'use',
      token(prec(1, '<')),
      sepBy(
        ',',
        choice(
          $.lifetime,
          $._type_identifier,
        ),
      ),
      optional(','),
      '>',
    ),

    type_arguments: $ => seq(
      token(prec(1, '<')),
      sepBy1(',', seq(
        choice(
          $._type,
          $.type_binding,
          $.lifetime,
          $._literal,
          $.block,
        ),
        optional($.trait_bounds),
      )),
      optional(','),
      '>',
    ),

    type_binding: $ => seq(
      field('name', $._type_identifier),
      field('type_arguments', optional($.type_arguments)),
      '=',
      field('type', $._type),
    ),

    reference_type: $ => seq(
      '&',
      optional($.lifetime),
      optional($.mutable_specifier),
      field('type', $._type),
    ),

    pointer_type: $ => seq(
      '*',
      choice('const', $.mutable_specifier),
      field('type', $._type),
    ),

    never_type: _ => '!',

    abstract_type: $ => seq(
      'impl',
      optional(seq('for', $.type_parameters)),
      field('trait', prec(1, choice(
        $._type_identifier,
        $.scoped_type_identifier,
        $.removed_trait_bound,
        $.generic_type,
        $.function_type,
        $.tuple_type,
        $.bounded_type,
      ))),
    ),

    dynamic_type: $ => seq(
      'dyn',
      field('trait', choice(
        $.higher_ranked_trait_bound,
        $._type_identifier,
        $.scoped_type_identifier,
        $.generic_type,
        $.function_type,
        $.tuple_type,
      )),
    ),

    mutable_specifier: _ => 'mut',

    // Section - Expressions

    _expression_except_range: $ => choice(
      $.unary_expression,
      $.reference_expression,
      $.try_expression,
      $.binary_expression,
      $.assignment_expression,
      $.compound_assignment_expr,
      $.type_cast_expression,
      $.call_expression,
      $.return_expression,
      $.yield_expression,
      $._literal,
      prec.left($.identifier),
      alias(choice(...primitiveTypes), $.identifier),
      prec.left($._reserved_identifier),
      $.self,
      $.scoped_identifier,
      $.generic_function,
      $.await_expression,
      $.field_expression,
      $.array_expression,
      $.tuple_expression,
      prec(1, $.macro_invocation),
      $.unit_expression,
      $.break_expression,
      $.continue_expression,
      $.index_expression,
      $.metavariable,
      $.closure_expression,
      $.parenthesized_expression,
      $.struct_expression,
      $._expression_ending_with_block,
    ),

    _expression: $ => choice(
      $._expression_except_range,
      $.range_expression,
    ),

    _expression_ending_with_block: $ => choice(
      $.unsafe_block,
      $.async_block,
      $.gen_block,
      $.try_block,
      $.block,
      $.if_expression,
      $.match_expression,
      $.while_expression,
      $.loop_expression,
      $.for_expression,
      $.const_block,
    ),

    macro_invocation: $ => seq(
      field('macro', choice(
        $.scoped_identifier,
        $.identifier,
        $._reserved_identifier,
      )),
      '!',
      alias($.delim_token_tree, $.token_tree),
    ),

    delim_token_tree: $ => choice(
      seq('(', repeat($._delim_tokens), ')'),
      seq('[', repeat($._delim_tokens), ']'),
      seq('{', repeat($._delim_tokens), '}'),
    ),

    _delim_tokens: $ => choice(
      $._non_delim_token,
      alias($.delim_token_tree, $.token_tree),
    ),

    // Should match any token other than a delimiter.
    _non_delim_token: $ => choice(
      $._non_special_token,
      '$',
    ),

    scoped_identifier: $ => seq(
      field('path', optional(choice(
        $._path,
        $.bracketed_type,
        alias($.generic_type_with_turbofish, $.generic_type),
      ))),
      '::',
      field('name', choice($.identifier, $.super)),
    ),

    scoped_type_identifier_in_expression_position: $ => prec(-2, seq(
      field('path', optional(choice(
        $._path,
        alias($.generic_type_with_turbofish, $.generic_type),
      ))),
      '::',
      field('name', $._type_identifier),
    )),

    scoped_type_identifier: $ => seq(
      field('path', optional(choice(
        $._path,
        alias($.generic_type_with_turbofish, $.generic_type),
        $.bracketed_type,
        $.generic_type,
      ))),
      '::',
      field('name', $._type_identifier),
    ),

    range_expression: $ => prec.left(PREC.range, choice(
      seq($._expression, choice('..', '...', '..='), $._expression),
      seq($._expression, '..'),
      seq('..', $._expression),
      '..',
    )),

    unary_expression: $ => prec(PREC.unary, seq(
      choice('-', '*', '!'),
      $._expression,
    )),

    try_expression: $ => prec(PREC.try, seq(
      $._expression,
      '?',
    )),

    reference_expression: $ => prec(PREC.unary, seq(
      '&',
      choice(
        seq('raw', choice('const', $.mutable_specifier)),
        optional($.mutable_specifier),
      ),
      field('value', $._expression),
    )),

    binary_expression: $ => {
      const table = [
        [PREC.and, '&&'],
        [PREC.or, '||'],
        [PREC.bitand, '&'],
        [PREC.bitor, '|'],
        [PREC.bitxor, '^'],
        [PREC.comparative, choice('==', '!=', '<', '<=', '>', '>=')],
        [PREC.shift, choice('<<', '>>')],
        [PREC.additive, choice('+', '-')],
        [PREC.multiplicative, choice('*', '/', '%')],
      ];

      // @ts-ignore
      return choice(...table.map(([precedence, operator]) => prec.left(precedence, seq(
        field('left', $._expression),
        // @ts-ignore
        field('operator', operator),
        field('right', $._expression),
      ))));
    },

    assignment_expression: $ => prec.left(PREC.assign, seq(
      field('left', $._expression),
      '=',
      field('right', $._expression),
    )),

    compound_assignment_expr: $ => prec.left(PREC.assign, seq(
      field('left', $._expression),
      field('operator', choice('+=', '-=', '*=', '/=', '%=', '&=', '|=', '^=', '<<=', '>>=')),
      field('right', $._expression),
    )),

    type_cast_expression: $ => prec.left(PREC.cast, seq(
      field('value', $._expression),
      'as',
      field('type', $._type),
    )),

    return_expression: $ => choice(
      prec.left(seq('return', $._expression)),
      prec(-1, 'return'),
    ),

    yield_expression: $ => choice(
      prec.left(seq('yield', $._expression)),
      prec(-1, 'yield'),
    ),

    call_expression: $ => prec(PREC.call, seq(
      field('function', $._expression_except_range),
      field('arguments', $.arguments),
    )),

    arguments: $ => seq(
      '(',
      sepBy(',', seq(repeat($.attribute_item), $._expression)),
      optional(','),
      ')',
    ),

    array_expression: $ => seq(
      '[',
      repeat($.attribute_item),
      choice(
        seq(
          $._expression,
          ';',
          field('length', $._expression),
        ),
        seq(
          sepBy(',', seq(repeat($.attribute_item), $._expression)),
          optional(','),
        ),
      ),
      ']',
    ),

    parenthesized_expression: $ => seq(
      '(',
      $._expression,
      ')',
    ),

    tuple_expression: $ => seq(
      '(',
      repeat($.attribute_item),
      seq($._expression, ','),
      repeat(seq($._expression, ',')),
      optional($._expression),
      ')',
    ),

    unit_expression: _ => seq('(', ')'),

    struct_expression: $ => seq(
      field('name', choice(
        $._type_identifier,
        alias($.scoped_type_identifier_in_expression_position, $.scoped_type_identifier),
        $.generic_type_with_turbofish,
      )),
      field('body', $.field_initializer_list),
    ),

    field_initializer_list: $ => seq(
      '{',
      sepBy(',', choice(
        $.shorthand_field_initializer,
        $.field_initializer,
        $.base_field_initializer,
      )),
      optional(','),
      '}',
    ),

    shorthand_field_initializer: $ => seq(
      repeat($.attribute_item),
      $.identifier,
    ),

    field_initializer: $ => seq(
      repeat($.attribute_item),
      field('field', choice($._field_identifier, $.integer_literal)),
      ':',
      field('value', $._expression),
    ),

    base_field_initializer: $ => seq(
      '..',
      $._expression,
    ),

    if_expression: $ => prec.right(seq(
      'if',
      field('condition', $._condition),
      field('consequence', $.block),
      optional(field('alternative', $.else_clause)),
    )),

    let_condition: $ => seq(
      'let',
      field('pattern', $._pattern),
      '=',
      field('value', prec.left(PREC.and, $._expression)),
    ),

    _let_chain: $ => prec.left(PREC.and, choice(
      seq($._let_chain, '&&', $.let_condition),
      seq($._let_chain, '&&', $._expression),
      seq($.let_condition, '&&', $._expression),
      seq($.let_condition, '&&', $.let_condition),
      seq($._expression, '&&', $.let_condition),
    )),

    _condition: $ => choice(
      $._expression,
      $.let_condition,
      alias($._let_chain, $.let_chain),
    ),

    else_clause: $ => seq(
      'else',
      choice(
        $.block,
        $.if_expression,
      ),
    ),

    match_expression: $ => seq(
      'match',
      field('value', $._expression),
      field('body', $.match_block),
    ),

    match_block: $ => seq(
      '{',
      optional(seq(
        repeat($.match_arm),
        alias($.last_match_arm, $.match_arm),
      )),
      '}',
    ),

    match_arm: $ => prec.right(seq(
      repeat(choice($.attribute_item, $.inner_attribute_item)),
      field('pattern', $.match_pattern),
      '=>',
      choice(
        seq(field('value', $._expression), ','),
        field('value', prec(1, $._expression_ending_with_block)),
      ),
    )),

    last_match_arm: $ => seq(
      repeat(choice($.attribute_item, $.inner_attribute_item)),
      field('pattern', $.match_pattern),
      '=>',
      field('value', $._expression),
      optional(','),
    ),

    match_pattern: $ => seq(
      $._pattern,
      optional(seq('if', field('condition', $._condition))),
    ),

    while_expression: $ => seq(
      optional(seq($.label, ':')),
      'while',
      field('condition', $._condition),
      field('body', $.block),
    ),

    loop_expression: $ => seq(
      optional(seq($.label, ':')),
      'loop',
      field('body', $.block),
    ),

    for_expression: $ => seq(
      optional(seq($.label, ':')),
      'for',
      field('pattern', $._pattern),
      'in',
      field('value', $._expression),
      field('body', $.block),
    ),

    const_block: $ => seq(
      'const',
      field('body', $.block),
    ),

    closure_expression: $ => prec(PREC.closure, seq(
      optional('static'),
      optional('async'),
      optional('move'),
      field('parameters', $.closure_parameters),
      choice(
        seq(
          optional(seq('->', field('return_type', $._type))),
          field('body', $.block),
        ),
        field('body', choice($._expression, '_')),
      ),
    )),

    closure_parameters: $ => seq(
      '|',
      sepBy(',', choice(
        $._pattern,
        $.parameter,
      )),
      '|',
    ),

    label: $ => seq('\'', $.identifier),

    break_expression: $ => prec.left(seq('break', optional($.label), optional($._expression))),

    continue_expression: $ => prec.left(seq('continue', optional($.label))),

    index_expression: $ => prec(PREC.call, seq($._expression, '[', $._expression, ']')),

    await_expression: $ => prec(PREC.field, seq(
      $._expression,
      '.',
      'await',
    )),

    field_expression: $ => prec(PREC.field, seq(
      field('value', $._expression),
      '.',
      field('field', choice(
        $._field_identifier,
        $.integer_literal,
      )),
    )),

    unsafe_block: $ => seq(
      'unsafe',
      $.block,
    ),

    async_block: $ => seq(
      'async',
      optional('move'),
      $.block,
    ),

    gen_block: $ => seq(
      'gen',
      optional('move'),
      $.block,
    ),

    try_block: $ => seq(
      'try',
      $.block,
    ),

    block: $ => seq(
      optional(seq($.label, ':')),
      '{',
      repeat($._statement),
      optional($._expression),
      '}',
    ),

    // Section - Patterns

    _pattern: $ => choice(
      $._literal_pattern,
      alias(choice(...primitiveTypes), $.identifier),
      $.identifier,
      $.scoped_identifier,
      $.generic_pattern,
      $.tuple_pattern,
      $.tuple_struct_pattern,
      $.struct_pattern,
      $._reserved_identifier,
      $.ref_pattern,
      $.slice_pattern,
      $.captured_pattern,
      $.reference_pattern,
      $.remaining_field_pattern,
      $.mut_pattern,
      $.range_pattern,
      $.or_pattern,
      $.const_block,
      $.macro_invocation,
      '_',
    ),

    generic_pattern: $ => seq(
      choice(
        $.identifier,
        $.scoped_identifier,
      ),
      '::',
      field('type_arguments', $.type_arguments),
    ),

    tuple_pattern: $ => seq(
      '(',
      sepBy(',', choice($._pattern, $.closure_expression)),
      optional(','),
      ')',
    ),

    slice_pattern: $ => seq(
      '[',
      sepBy(',', $._pattern),
      optional(','),
      ']',
    ),

    tuple_struct_pattern: $ => seq(
      field('type', choice(
        $.identifier,
        $.scoped_identifier,
        alias($.generic_type_with_turbofish, $.generic_type),
      )),
      '(',
      sepBy(',', $._pattern),
      optional(','),
      ')',
    ),

    struct_pattern: $ => seq(
      field('type', choice(
        $._type_identifier,
        $.scoped_type_identifier,
      )),
      '{',
      sepBy(',', choice($.field_pattern, $.remaining_field_pattern)),
      optional(','),
      '}',
    ),

    field_pattern: $ => seq(
      optional('ref'),
      optional($.mutable_specifier),
      choice(
        field('name', alias($.identifier, $.shorthand_field_identifier)),
        seq(
          field('name', $._field_identifier),
          ':',
          field('pattern', $._pattern),
        ),
      ),
    ),

    remaining_field_pattern: _ => '..',

    mut_pattern: $ => prec(-1, seq(
      $.mutable_specifier,
      $._pattern,
    )),

    range_pattern: $ => choice(
      seq(
        field('left', choice(
          $._literal_pattern,
          $._path,
        )),
        choice(
          seq(
            choice('...', '..=', '..'),
            field('right', choice(
              $._literal_pattern,
              $._path,
            )),
          ),
          '..',
        ),
      ),
      seq(
        choice('..=', '..'),
        field('right', choice(
          $._literal_pattern,
          $._path,
        )),
      ),
    ),

    ref_pattern: $ => seq(
      'ref',
      $._pattern,
    ),

    captured_pattern: $ => seq(
      $.identifier,
      '@',
      $._pattern,
    ),

    reference_pattern: $ => seq(
      '&',
      optional($.mutable_specifier),
      $._pattern,
    ),

    or_pattern: $ => prec.left(-2, choice(
      seq($._pattern, '|', $._pattern),
      seq('|', $._pattern),
    )),

    // Section - Literals

    _literal: $ => choice(
      $.string_literal,
      $.raw_string_literal,
      $.char_literal,
      $.boolean_literal,
      $.integer_literal,
      $.float_literal,
    ),

    _literal_pattern: $ => choice(
      $.string_literal,
      $.raw_string_literal,
      $.char_literal,
      $.boolean_literal,
      $.integer_literal,
      $.float_literal,
      $.negative_literal,
    ),

    negative_literal: $ => seq('-', choice($.integer_literal, $.float_literal)),

    integer_literal: _ => token(seq(
      choice(
        /[0-9][0-9_]*/,
        /0x[0-9a-fA-F_]+/,
        /0b[01_]+/,
        /0o[0-7_]+/,
      ),
      optional(choice(...numericTypes)),
    )),

    string_literal: $ => seq(
      alias(/[bc]?"/, '"'),
      repeat(choice(
        $.escape_sequence,
        $.string_content,
      )),
      alias($.string_close, '"'),
    ),

    raw_string_literal: $ => seq(
      $._raw_string_literal_start,
      alias($.raw_string_literal_content, $.string_content),
      $._raw_string_literal_end,
    ),

    char_literal: _ => token(seq(
      optional('b'),
      '\'',
      optional(choice(
        seq('\\', choice(
          /[^xu]/,
          /u[0-9a-fA-F]{4}/,
          /u\{[0-9a-fA-F]+\}/,
          /x[0-9a-fA-F]{2}/,
        )),
        /[^\\']/,
      )),
      '\'',
    )),

    escape_sequence: _ => token.immediate(
      seq('\\',
        choice(
          /[^xu]/,
          /u[0-9a-fA-F]{4}/,
          /u\{[0-9a-fA-F]+\}/,
          /x[0-9a-fA-F]{2}/,
        ),
      )),

    boolean_literal: _ => choice('true', 'false'),

    comment: $ => choice(
      $.line_comment,
      $.block_comment,
    ),

    line_comment: $ => seq(
      // All line comments start with two //
      '//',
      // Then are followed by:
      // - 2 or more slashes making it a regular comment
      // - 1 slash or 1 or more bang operators making it a doc comment
      // - or just content for the comment
      choice(
        // A tricky edge case where what looks like a doc comment is not
        seq(token.immediate(prec(2, /\/\//)), /.*/),
        // A regular doc comment
        seq($._line_doc_comment_marker, field('doc', alias($._line_doc_content, $.doc_comment))),
        token.immediate(prec(1, /.*/)),
      ),
    ),

    _line_doc_comment_marker: $ => choice(
      // An outer line doc comment applies to the element that it is outside of
      field('outer', alias($._outer_line_doc_comment_marker, $.outer_doc_comment_marker)),
      // An inner line doc comment applies to the element it is inside of
      field('inner', alias($._inner_line_doc_comment_marker, $.inner_doc_comment_marker)),
    ),

    _inner_line_doc_comment_marker: _ => token.immediate(prec(2, '!')),
    _outer_line_doc_comment_marker: _ => token.immediate(prec(2, '/')),

    block_comment: $ => seq(
      '/*',
      optional(
        choice(
          // Documentation block comments: /** docs */ or /*! docs */
          seq(
            $._block_doc_comment_marker,
            optional(field('doc', alias($._block_comment_content, $.doc_comment))),
          ),
          // Non-doc block comments
          $._block_comment_content,
        ),
      ),
      '*/',
    ),

    _block_doc_comment_marker: $ => choice(
      field('outer', alias($._outer_block_doc_comment_marker, $.outer_doc_comment_marker)),
      field('inner', alias($._inner_block_doc_comment_marker, $.inner_doc_comment_marker)),
    ),

    _path: $ => choice(
      $.self,
      alias(choice(...primitiveTypes), $.identifier),
      $.metavariable,
      $.super,
      $.crate,
      $.identifier,
      $.scoped_identifier,
      $._reserved_identifier,
    ),

    identifier: _ => /(r#)?[_\p{XID_Start}][_\p{XID_Continue}]*/,

    shebang: _ => /#![\r\f\t\v ]*([^\[\n].*)?\n/,

    _reserved_identifier: $ => alias(choice(
      'default',
      'union',
      'gen',
      'raw',
    ), $.identifier),

    _type_identifier: $ => alias($.identifier, $.type_identifier),
    _field_identifier: $ => alias($.identifier, $.field_identifier),

    self: _ => 'self',
    super: _ => 'super',
    crate: _ => 'crate',

    metavariable: _ => /\$[a-zA-Z_]\w*/,
  },
});

/**
 * Creates a rule to match one or more of the rules separated by the separator.
 *
 * @param {RuleOrLiteral} sep - The separator to use.
 * @param {RuleOrLiteral} rule
 *
 * @returns {SeqRule}
 */
function sepBy1(sep, rule) {
  return seq(rule, repeat(seq(sep, rule)));
}


/**
 * Creates a rule to optionally match one or more of the rules separated by the separator.
 *
 * @param {RuleOrLiteral} sep - The separator to use.
 * @param {RuleOrLiteral} rule
 *
 * @returns {ChoiceRule}
 */
function sepBy(sep, rule) {
  return optional(sepBy1(sep, rule));
}
