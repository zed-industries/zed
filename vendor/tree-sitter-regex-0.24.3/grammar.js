/**
 * @file Regex grammar for tree-sitter
 * @author Max Brunsfeld <maxbrunsfeld@gmail.com>
 * @author Amaan Qureshi <amaanq12@gmail.com>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

/**
 *
 * @param {RuleBuilder<string>} prefix - The rule builder
 *
 * @returns {RuleBuilder<string>}
 */
const quantifierRule = prefix => $ => seq(
  prefix($),
  optional(alias('?', $.lazy)),
);

const SYNTAX_CHARS = [
  ...'^$\\.*+?()[]|',
];

const SYNTAX_CHARS_ESCAPED = SYNTAX_CHARS.map(
  char => `\\${char}`,
).join('');

module.exports = grammar({
  name: 'regex',

  extras: _ => [/\r?\n/],

  inline: $ => [
    $._character_escape,
    $._class_atom,
  ],

  conflicts: $ => [[$.class_range, $.character_class]],

  rules: {
    pattern: $ => choice(
      $.alternation,
      $.term,
    ),

    alternation: $ => seq(
      optional($.term),
      repeat1(seq('|', optional($.term))),
    ),

    term: $ => repeat1(seq(
      choice(
        $.start_assertion,
        $.end_assertion,
        $.boundary_assertion,
        $.non_boundary_assertion,
        $.lookaround_assertion,
        $.pattern_character,
        $.character_class,
        $.posix_character_class,
        $.any_character,
        $.decimal_escape,
        $.character_class_escape,
        $._character_escape,
        $.backreference_escape,
        $.named_group_backreference,
        $.anonymous_capturing_group,
        $.named_capturing_group,
        $.non_capturing_group,
        $.inline_flags_group,
      ),
      optional(choice(
        $.zero_or_more,
        $.one_or_more,
        $.optional,
        $.count_quantifier,
      )),
    )),

    any_character: _ => '.',

    start_assertion: _ => '^',
    end_assertion: _ => '$',
    boundary_assertion: _ => '\\b',
    non_boundary_assertion: _ => '\\B',
    lookaround_assertion: $ => choice(
      $._lookahead_assertion,
      $._lookbehind_assertion,
    ),
    _lookahead_assertion: $ => seq(
      '(?',
      choice('=', '!'),
      $.pattern,
      ')',
    ),
    _lookbehind_assertion: $ => seq(
      '(?<',
      choice('=', '!'),
      $.pattern,
      ')',
    ),

    pattern_character: _ => new RegExp(`[^${SYNTAX_CHARS_ESCAPED}\\r?\\n]`),

    character_class: $ => seq(
      '[',
      optional('^'),
      optional(alias('-', $.class_character)),
      repeat($._class_atom),
      optional(alias('-', $.class_character)),
      ']',
    ),

    posix_character_class: $ => seq(
      '[:',
      $.posix_class_name,
      ':]',
    ),

    posix_class_name: _ => /[a-zA-Z]+/,

    class_range: $ => prec.right(seq(
      choice(
        $.class_character,
        $.character_class_escape,
        $.control_escape,
        alias('-', $.class_character),
      ),
      '-',
      choice(
        $.class_character,
        $.character_class_escape,
        $.control_escape,
        alias('-', $.class_character),
      ),
    )),

    _class_atom: $ => choice(
      $.class_character,
      alias('\\-', $.identity_escape),
      $.character_class_escape,
      $._character_escape,
      $.posix_character_class,
      $.class_range,
    ),

    class_character: _ => // NOT: \ ] or -
      /[^\\\]\-]/,

    anonymous_capturing_group: $ => seq('(', $.pattern, ')'),

    named_capturing_group: $ => seq(choice('(?<', '(?P<'), $.group_name, '>', $.pattern, ')'),

    non_capturing_group: $ => seq('(?:', $.pattern, ')'),

    inline_flags_group: $ => seq(
      '(?',
      choice(
        $.flags,
        seq($.flags, '-', $.flags),
        seq('-', $.flags),
      ),
      optional(seq(':', $.pattern)),
      ')',
    ),

    flags: _ => /[a-zA-Z]+/,

    zero_or_more: quantifierRule(_ => '*'),
    one_or_more: quantifierRule(_ => '+'),
    optional: quantifierRule(_ => '?'),
    count_quantifier: quantifierRule($ => seq(
      '{',
      choice(
        seq(
          $.decimal_digits,
          optional(seq(',', optional($.decimal_digits))),
        ),
        seq(',', $.decimal_digits),
      ),
      '}',
    )),

    backreference_escape: $ => seq('\\k', '<', $.group_name, '>'),

    named_group_backreference: $ => seq('(?P=', $.group_name, ')'),

    decimal_escape: _ => /\\[1-9][0-9]*/,

    character_class_escape: $ => choice(
      /\\[dDsSwW]/,
      seq(/\\[pP]/, '{', $.unicode_property_value_expression, '}'),
      $.unicode_character_escape,
    ),

    unicode_character_escape: _ => choice(
      /\\u[0-9a-fA-F]{4}/,
      // NOTE: The following is a valid syntax only if the "u" flag is set.
      // However, this is unlikely that "u" would be encountered
      /\\u\{[0-9a-fA-F]{1,6}\}/,
    ),

    unicode_property_value_expression: $ => seq(
      optional(seq(alias($.unicode_property, $.unicode_property_name), '=')),
      alias($.unicode_property, $.unicode_property_value),
    ),

    unicode_property: _ => /[a-zA-Z_0-9]+/,

    _character_escape: $ => choice(
      $.control_escape,
      $.control_letter_escape,
      $.identity_escape,
    ),

    // TODO: We should technically not accept \0 unless the
    // lookahead is not also a digit.
    // I think this has little bearing on the highlighting of
    // correct regexes.
    control_escape: _ => choice(
      /\\[bfnrtv0]/,
      /\\x[0-9a-fA-F]{2}/,
    ),

    control_letter_escape: _ => /\\c[a-zA-Z]/,

    identity_escape: _ => token(seq('\\', /[^kdDsSpPwWbfnrtv0-9]/)),

    // TODO: This is an approximation of RegExpIdentifierName in the
    // formal grammar, which allows for Unicode names through
    // the following mechanism:
    //
    // RegExpIdentifierName[U]::
    //   RegExpIdentifierStart[?U]
    //   RegExpIdentifierName[?U]RegExpIdentifierPart[?U]
    //
    // RegExpIdentifierStart[U]::
    //   UnicodeIDStart
    //   $
    //   _
    //   \RegExpUnicodeEscapeSequence[?U]
    //
    // RegExpIdentifierPart[U]::
    //   UnicodeIDContinue
    //   $
    //   \RegExpUnicodeEscapeSequence[?U]
    //   <ZWNJ> <ZWJ>
    // RegExpUnicodeEscapeSequence[U]::
    //   [+U]uLeadSurrogate\uTrailSurrogate
    //   [+U]uLeadSurrogate
    //   [+U]uTrailSurrogate
    //   [+U]uNonSurrogate
    //   [~U]uHex4Digits
    //   [+U]u{CodePoint}
    group_name: _ => /[A-Za-z_][A-Za-z0-9_]*/,

    decimal_digits: _ => /\d+/,
  },
});
