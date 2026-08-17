/**
 * @file JSDoc grammar for tree-sitter
 * @author Max Brunsfeld <maxbrunsfeld@gmail.com>
 * @author Amaan Qureshi <amaanq12@gmail.com>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: 'jsdoc',

  externals: $ => [
    $.type,
  ],

  extras: _ => [
    token(choice(
      // Skip over stars at the beginnings of lines
      seq(/\n/, /[ \t]*/, repeat(seq('*', /[ \t]*/))),
      /\s/,
    )),
  ],

  rules: {
    document: $ => seq(
      $._begin,
      optional($.description),
      repeat($.tag),
      $._end,
    ),

    description: $ => seq(
      $._text,
      repeat(choice($._text, $.inline_tag, $._inline_tag_false_positive)),
    ),

    tag: $ => choice(
      // type, name, and description
      seq(
        alias($.tag_name_with_argument, $.tag_name),
        optional(seq('{', $.type, '}')),
        optional($._expression),
        optional($.description),
      ),

      // type and description
      seq(
        alias($.tag_name_with_type, $.tag_name),
        optional(seq('{', $.type, '}')),
        optional($.description),
      ),

      // description only
      seq(
        $.tag_name,
        optional($.description),
      ),
    ),

    inline_tag: $ => seq(
      '{',
      $.tag_name,
      $.description,
      '}',
    ),

    _inline_tag_false_positive: _ => token(prec.left(1, /\{[^@}]+\}?/)),

    tag_name_with_argument: _ => token(choice(
      '@access',
      '@alias',
      '@api',
      '@augments',
      '@borrows',
      '@callback',
      '@constructor',
      '@event',
      '@exports',
      '@external',
      '@extends',
      '@fires',
      '@function',
      '@mixes',
      '@name',
      '@namespace',
      '@param',
      '@property',
      '@prop',
      '@satisfies',
      '@typedef',
    )),

    tag_name_with_type: _ => token(choice(
      '@return',
      '@returns',
      '@throw',
      '@throws',
      '@type',
    )),

    tag_name: _ => /@[a-zA-Z_]+/,

    _expression: $ => choice(
      $.identifier,
      $.optional_identifier,
      $.member_expression,
      $.path_expression,
      $.qualified_expression,
    ),

    qualified_expression: $ => prec(1, seq(
      $.identifier,
      ':',
      $._expression,
    )),

    path_expression: $ => prec(2, seq(
      $.identifier,
      token.immediate('/'),
      $.identifier,
    )),

    member_expression: $ => seq(
      $._expression,
      choice(
        '.',
        '#',
        '~',
      ),
      choice(
        $.identifier,
        $.qualified_expression,
      ),
    ),

    optional_identifier: $ => seq('[', $.identifier, ']'),

    identifier: _ => /[a-zA-Z_$][a-zA-Z_$0-9]*/,

    _text: _ => token(prec(-1, /[^*{}@\s][^*{}\n]*([^*/{}\n][^*{}\n]*\*+)*/)),

    _begin: _ => token(seq('/', repeat('*'))),

    _end: _ => '/',
  },
});
