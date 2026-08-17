// This grammar only concerns the block structure according to the CommonMark Spec
// (https://spec.commonmark.org/0.30/#blocks-and-inlines)
// For more information see README.md

/// <reference types="tree-sitter-cli/dsl" />

const common = require('../common/common');

const PRECEDENCE_LEVEL_LINK = common.PRECEDENCE_LEVEL_LINK;

const PUNCTUATION_CHARACTERS_REGEX = '!-/:-@\\[-`\\{-~';

module.exports = grammar({
    name: 'markdown',

    rules: {
        document: $ => seq(
            optional(choice(
                common.EXTENSION_MINUS_METADATA ? $.minus_metadata : choice(),
                common.EXTENSION_PLUS_METADATA ? $.plus_metadata : choice(),
            )),
            alias(prec.right(repeat($._block_not_section)), $.section),
            repeat($.section),
        ),

        ...common.rules,
        _last_token_punctuation: $ => choice(), // needed for compatability with common rules

        // BLOCK STRUCTURE

        // All blocks. Every block contains a trailing newline.
        _block: $ => choice(
            $._block_not_section,
            $.section,
        ),
        _block_not_section: $ => choice(
            alias($._setext_heading1, $.setext_heading),
            alias($._setext_heading2, $.setext_heading),
            $.paragraph,
            $.indented_code_block,
            $.block_quote,
            $.thematic_break,
            $.list,
            $.fenced_code_block,
            $._blank_line,
            $.html_block,
            $.link_reference_definition,
            common.EXTENSION_PIPE_TABLE ? $.pipe_table : choice(),
        ),
        section: $ => choice($._section1, $._section2, $._section3, $._section4, $._section5, $._section6),
        _section1: $ => prec.right(seq(
            alias($._atx_heading1, $.atx_heading),
            repeat(choice(
                alias(choice($._section6, $._section5, $._section4, $._section3, $._section2), $.section),
                $._block_not_section
            ))
        )),
        _section2: $ => prec.right(seq(
            alias($._atx_heading2, $.atx_heading),
            repeat(choice(
                alias(choice($._section6, $._section5, $._section4, $._section3), $.section),
                $._block_not_section
            ))
        )),
        _section3: $ => prec.right(seq(
            alias($._atx_heading3, $.atx_heading),
            repeat(choice(
                alias(choice($._section6, $._section5, $._section4), $.section),
                $._block_not_section
            ))
        )),
        _section4: $ => prec.right(seq(
            alias($._atx_heading4, $.atx_heading),
            repeat(choice(
                alias(choice($._section6, $._section5), $.section),
                $._block_not_section
            ))
        )),
        _section5: $ => prec.right(seq(
            alias($._atx_heading5, $.atx_heading),
            repeat(choice(
                alias($._section6, $.section),
                $._block_not_section
            ))
        )),
        _section6: $ => prec.right(seq(
            alias($._atx_heading6, $.atx_heading),
            repeat($._block_not_section)
        )),

        // LEAF BLOCKS

        // A thematic break. This is currently handled by the external scanner but maybe could be
        // parsed using normal tree-sitter rules.
        //
        // https://github.github.com/gfm/#thematic-breaks
        thematic_break: $ => seq($._thematic_break, choice($._newline, $._eof)),

        // An ATX heading. This is currently handled by the external scanner but maybe could be
        // parsed using normal tree-sitter rules.
        //
        // https://github.github.com/gfm/#atx-headings
        _atx_heading1: $ => prec(1, seq(
            $.atx_h1_marker,
            optional($._atx_heading_content),
            $._newline
        )),
        _atx_heading2: $ => prec(1, seq(
            $.atx_h2_marker,
            optional($._atx_heading_content),
            $._newline
        )),
        _atx_heading3: $ => prec(1, seq(
            $.atx_h3_marker,
            optional($._atx_heading_content),
            $._newline
        )),
        _atx_heading4: $ => prec(1, seq(
            $.atx_h4_marker,
            optional($._atx_heading_content),
            $._newline
        )),
        _atx_heading5: $ => prec(1, seq(
            $.atx_h5_marker,
            optional($._atx_heading_content),
            $._newline
        )),
        _atx_heading6: $ => prec(1, seq(
            $.atx_h6_marker,
            optional($._atx_heading_content),
            $._newline
        )),
        _atx_heading_content: $ => prec(1, seq(
            optional($._whitespace),
            field('heading_content', alias($._line, $.inline))
        )),

        // A setext heading. The underlines are currently handled by the external scanner but maybe
        // could be parsed using normal tree-sitter rules.
        //
        // https://github.github.com/gfm/#setext-headings
        _setext_heading1: $ => seq(
            field('heading_content', $.paragraph),
            $.setext_h1_underline,
            choice($._newline, $._eof),
        ),
        _setext_heading2: $ => seq(
            field('heading_content', $.paragraph),
            $.setext_h2_underline,
            choice($._newline, $._eof),
        ),

        // An indented code block. An indented code block is made up of indented chunks and blank
        // lines. The indented chunks are handeled by the external scanner.
        //
        // https://github.github.com/gfm/#indented-code-blocks
        indented_code_block: $ => prec.right(seq($._indented_chunk, repeat(choice($._indented_chunk, $._blank_line)))),
        _indented_chunk: $ => seq($._indented_chunk_start, repeat(choice($._line, $._newline)), $._block_close, optional($.block_continuation)),

        // A fenced code block. Fenced code blocks are mainly handled by the external scanner. In
        // case of backtick code blocks the external scanner also checks that the info string is
        // proper.
        //
        // https://github.github.com/gfm/#fenced-code-blocks
        fenced_code_block: $ => prec.right(choice(
            seq(
                alias($._fenced_code_block_start_backtick, $.fenced_code_block_delimiter),
                optional($._whitespace),
                optional($.info_string),
                $._newline,
                optional($.code_fence_content),
                optional(seq(alias($._fenced_code_block_end_backtick, $.fenced_code_block_delimiter), $._close_block, $._newline)),
                $._block_close,
            ),
            seq(
                alias($._fenced_code_block_start_tilde, $.fenced_code_block_delimiter),
                optional($._whitespace),
                optional($.info_string),
                $._newline,
                optional($.code_fence_content),
                optional(seq(alias($._fenced_code_block_end_tilde, $.fenced_code_block_delimiter), $._close_block, $._newline)),
                $._block_close,
            ),
        )),
        code_fence_content: $ => repeat1(choice($._newline, $._line)),
        info_string: $ => choice(
            seq($.language, repeat(choice($._line, $.backslash_escape, $.entity_reference, $.numeric_character_reference))),
            seq(
                repeat1(choice('{', '}')),
                optional(choice(
                    seq($.language, repeat(choice($._line, $.backslash_escape, $.entity_reference, $.numeric_character_reference))),
                    seq($._whitespace, repeat(choice($._line, $.backslash_escape, $.entity_reference, $.numeric_character_reference))),
                ))
            )
        ),
        language: $ => prec.right(repeat1(choice($._word, common.punctuation_without($, ['{', '}', ',']), $.backslash_escape, $.entity_reference, $.numeric_character_reference))),

        // An HTML block. We do not emit addition nodes relating to the kind or structure or of the
        // html block as this is best done using language injections and a proper html parsers.
        //
        // See the `build_html_block` function for more information.
        // See the spec for the different kinds of html blocks.
        //
        // https://github.github.com/gfm/#html-blocks
        html_block: $ => prec(1, seq(optional($._whitespace), choice(
            $._html_block_1,
            $._html_block_2,
            $._html_block_3,
            $._html_block_4,
            $._html_block_5,
            $._html_block_6,
            $._html_block_7,
        ))),
        _html_block_1: $ => build_html_block($,
            // new RegExp(
            //     '[ \t]*<' + regex_case_insensitive_list(HTML_TAG_NAMES_RULE_1) + '([\\r\\n]|[ \\t>][^<\\r\\n]*(\\n|\\r\\n?)?)'
            // ),
            $._html_block_1_start,
            $._html_block_1_end,
            true
        ),
        _html_block_2: $ => build_html_block($, $._html_block_2_start, '-->', true),
        _html_block_3: $ => build_html_block($, $._html_block_3_start, '?>', true),
        _html_block_4: $ => build_html_block($, $._html_block_4_start, '>', true),
        _html_block_5: $ => build_html_block($, $._html_block_5_start, ']]>', true),
        _html_block_6: $ => build_html_block(
            $,
            $._html_block_6_start,
            seq($._newline, $._blank_line),
            true
        ),
        _html_block_7: $ => build_html_block(
            $,
            $._html_block_7_start,
            seq($._newline, $._blank_line),
            false
        ),

        // A link reference definition. We need to make sure that this is not mistaken for a
        // paragraph or indented chunk. The `$._no_indented_chunk` token is used to tell the
        // external scanner not to allow indented chunks when the `$.link_title` of the link
        // reference definition would be valid.
        //
        // https://github.github.com/gfm/#link-reference-definitions
        link_reference_definition: $ => prec.dynamic(PRECEDENCE_LEVEL_LINK, seq(
            optional($._whitespace),
            $.link_label,
            ':',
            optional(seq(optional($._whitespace), optional(seq($._soft_line_break, optional($._whitespace))))),
            $.link_destination,
            optional(prec.dynamic(2 * PRECEDENCE_LEVEL_LINK, seq(
                choice(
                    seq($._whitespace, optional(seq($._soft_line_break, optional($._whitespace)))),
                    seq($._soft_line_break, optional($._whitespace)),
                ),
                optional($._no_indented_chunk),
                $.link_title
            ))),
            choice($._newline, $._soft_line_break, $._eof),
        )),
        _text_inline_no_link: $ => choice($._word, $._whitespace, common.punctuation_without($, ['[', ']'])),

        // A paragraph. The parsing tactic for deciding when a paragraph ends is as follows:
        // on every newline inside a paragraph a conflict is triggered manually using
        // `$._split_token` to split the parse state into two branches.
        //
        // One of them - the one that also contains a `$._soft_line_break_marker` will try to
        // continue the paragraph, but we make sure that the beginning of a new block that can
        // interrupt a paragraph can also be parsed. If this is the case we know that the paragraph
        // should have been closed and the external parser will emit an `$._error` to kill the parse
        // branch.
        //
        // The other parse branch consideres the paragraph to be over. It will be killed if no valid new
        // block is detected before the next newline. (For example it will also be killed if a indented
        // code block is detected, which cannot interrupt paragraphs).
        //
        // Either way, after the next newline only one branch will exist, so the ammount of branches
        // related to paragraphs ending does not grow.
        //
        // https://github.github.com/gfm/#paragraphs
        paragraph: $ => seq(alias(repeat1(choice($._line, $._soft_line_break)), $.inline), choice($._newline, $._eof)),

        // A blank line including the following newline.
        //
        // https://github.github.com/gfm/#blank-lines
        _blank_line: $ => seq($._blank_line_start, choice($._newline, $._eof)),


        // CONTAINER BLOCKS

        // A block quote. This is the most basic example of a container block handled by the
        // external scanner.
        //
        // https://github.github.com/gfm/#block-quotes
        block_quote: $ => seq(
            alias($._block_quote_start, $.block_quote_marker),
            optional($.block_continuation),
            repeat($._block),
            $._block_close,
            optional($.block_continuation)
        ),

        // A list. This grammar does not differentiate between loose and tight lists for efficiency
        // reasons.
        //
        // Lists can only contain list items with list markers of the same type. List items are
        // handled by the external scanner.
        //
        // https://github.github.com/gfm/#lists
        list: $ => prec.right(choice(
            $._list_plus,
            $._list_minus,
            $._list_star,
            $._list_dot,
            $._list_parenthesis
        )),
        _list_plus: $ => prec.right(repeat1(alias($._list_item_plus, $.list_item))),
        _list_minus: $ => prec.right(repeat1(alias($._list_item_minus, $.list_item))),
        _list_star: $ => prec.right(repeat1(alias($._list_item_star, $.list_item))),
        _list_dot: $ => prec.right(repeat1(alias($._list_item_dot, $.list_item))),
        _list_parenthesis: $ => prec.right(repeat1(alias($._list_item_parenthesis, $.list_item))),
        // Some list items can not interrupt a paragraph and are marked as such by the external
        // scanner.
        list_marker_plus: $ => choice($._list_marker_plus, $._list_marker_plus_dont_interrupt),
        list_marker_minus: $ => choice($._list_marker_minus, $._list_marker_minus_dont_interrupt),
        list_marker_star: $ => choice($._list_marker_star, $._list_marker_star_dont_interrupt),
        list_marker_dot: $ => choice($._list_marker_dot, $._list_marker_dot_dont_interrupt),
        list_marker_parenthesis: $ => choice($._list_marker_parenthesis, $._list_marker_parenthesis_dont_interrupt),
        _list_item_plus: $ => seq(
            $.list_marker_plus,
            optional($.block_continuation),
            $._list_item_content,
            $._block_close,
            optional($.block_continuation)
        ),
        _list_item_minus: $ => seq(
            $.list_marker_minus,
            optional($.block_continuation),
            $._list_item_content,
            $._block_close,
            optional($.block_continuation)
        ),
        _list_item_star: $ => seq(
            $.list_marker_star,
            optional($.block_continuation),
            $._list_item_content,
            $._block_close,
            optional($.block_continuation)
        ),
        _list_item_dot: $ => seq(
            $.list_marker_dot,
            optional($.block_continuation),
            $._list_item_content,
            $._block_close,
            optional($.block_continuation)
        ),
        _list_item_parenthesis: $ => seq(
            $.list_marker_parenthesis,
            optional($.block_continuation),
            $._list_item_content,
            $._block_close,
            optional($.block_continuation)
        ),
        // List items are closed after two consecutive blank lines
        _list_item_content: $ => choice(
            prec(1, seq(
                $._blank_line,
                $._blank_line,
                $._close_block,
                optional($.block_continuation)
            )),
            repeat1($._block),
            common.EXTENSION_TASK_LIST ? prec(1, seq(
                choice($.task_list_marker_checked, $.task_list_marker_unchecked),
                $._whitespace,
                $.paragraph,
                repeat($._block)
            )) : choice()
        ),

        // Newlines as in the spec. Parsing a newline triggers the matching process by making
        // the external parser emit a `$._line_ending`.
        _newline: $ => seq(
            $._line_ending,
            optional($.block_continuation)
        ),
        _soft_line_break: $ => seq(
            $._soft_line_ending,
            optional($.block_continuation)
        ),
        // Some symbols get parsed as single tokens so that html blocks get detected properly
        _line: $ => prec.right(repeat1(choice($._word, $._whitespace, common.punctuation_without($, [])))),
        _word: $ => choice(
            new RegExp('[^' + PUNCTUATION_CHARACTERS_REGEX + ' \\t\\n\\r]+'),
            common.EXTENSION_TASK_LIST ? choice(
                /\[[xX]\]/,
                /\[[ \t]\]/,
            ) : choice()
        ),
        // The external scanner emits some characters that should just be ignored.
        _whitespace: $ => /[ \t]+/,

        ...(common.EXTENSION_TASK_LIST ? {
            task_list_marker_checked: $ => prec(1, /\[[xX]\]/),
            task_list_marker_unchecked: $ => prec(1, /\[[ \t]\]/),
        } : {}),

        ...(common.EXTENSION_PIPE_TABLE ? {
            pipe_table: $ => prec.right(seq(
                $._pipe_table_start,
                alias($.pipe_table_row, $.pipe_table_header),
                $._newline,
                $.pipe_table_delimiter_row,
                repeat(seq($._pipe_table_newline, optional($.pipe_table_row))),
                choice($._newline, $._eof),
            )),

            _pipe_table_newline: $ => seq(
                $._pipe_table_line_ending,
                optional($.block_continuation)
            ),

            pipe_table_delimiter_row: $ => seq(
                optional(seq(
                    optional($._whitespace),
                    '|',
                )),
                repeat1(prec.right(seq(
                    optional($._whitespace),
                    $.pipe_table_delimiter_cell,
                    optional($._whitespace),
                    '|',
                ))),
                optional($._whitespace),
                optional(seq(
                    $.pipe_table_delimiter_cell,
                    optional($._whitespace)
                )),
            ),

            pipe_table_delimiter_cell: $ => seq(
                optional(alias(':', $.pipe_table_align_left)),
                repeat1('-'),
                optional(alias(':', $.pipe_table_align_right)),
            ),

            pipe_table_row: $ => seq(
                optional(seq(
                    optional($._whitespace),
                    '|',
                )),
                choice(
                    seq(
                        repeat1(prec.right(seq(
                            choice(
                                seq(
                                    optional($._whitespace),
                                    $.pipe_table_cell,
                                    optional($._whitespace)
                                ),
                                alias($._whitespace, $.pipe_table_cell)
                            ),
                            '|',
                        ))),
                        optional($._whitespace),
                        optional(seq(
                            $.pipe_table_cell,
                            optional($._whitespace)
                        )),
                    ),
                    seq(
                        optional($._whitespace),
                        $.pipe_table_cell,
                        optional($._whitespace)
                    )
                ),
            ),

            pipe_table_cell: $ => prec.right(seq(
                choice(
                    $._word,
                    $._backslash_escape,
                    common.punctuation_without($, ['|']),
                ),
                repeat(choice(
                    $._word,
                    $._whitespace,
                    $._backslash_escape,
                    common.punctuation_without($, ['|']),
                )),
            )),
        } : {}),
    },

    externals: $ => [
        // Quite a few of these tokens could maybe be implemented without use of the external parser.
        // For this the `$._open_block` and `$._close_block` tokens should be used to tell the external
        // parser to put a new anonymous block on the block stack.

        // Block structure gets parsed as follows: After every newline (`$._line_ending`) we try to match
        // as many open blocks as possible. For example if the last line was part of a block quote we look
        // for a `>` at the beginning of the next line. We emit a `$.block_continuation` for each matched
        // block. For this process the external scanner keeps a stack of currently open blocks.
        //
        // If we are not able to match all blocks that does not necessarily mean that all unmatched blocks
        // have to be closed. It could also mean that the line is a lazy continuation line
        // (https://github.github.com/gfm/#lazy-continuation-line, see also `$._split_token` and
        // `$._soft_line_break_marker` below)
        //
        // If a block does get closed (because it was not matched or because some closing token was
        // encountered) we emit a `$._block_close` token

        $._line_ending, // this token does not contain the actual newline characters. see `$._newline`
        $._soft_line_ending,
        $._block_close,
        $.block_continuation,

        // Tokens signifying the start of a block. Blocks that do not need a `$._block_close` because they
        // always span one line are marked as such.

        $._block_quote_start,
        $._indented_chunk_start,
        $.atx_h1_marker, // atx headings do not need a `$._block_close`
        $.atx_h2_marker,
        $.atx_h3_marker,
        $.atx_h4_marker,
        $.atx_h5_marker,
        $.atx_h6_marker,
        $.setext_h1_underline, // setext headings do not need a `$._block_close`
        $.setext_h2_underline,
        $._thematic_break, // thematic breaks do not need a `$._block_close`
        $._list_marker_minus,
        $._list_marker_plus,
        $._list_marker_star,
        $._list_marker_parenthesis,
        $._list_marker_dot,
        $._list_marker_minus_dont_interrupt, // list items that do not interrupt an ongoing paragraph
        $._list_marker_plus_dont_interrupt,
        $._list_marker_star_dont_interrupt,
        $._list_marker_parenthesis_dont_interrupt,
        $._list_marker_dot_dont_interrupt,
        $._fenced_code_block_start_backtick,
        $._fenced_code_block_start_tilde,
        $._blank_line_start, // Does not contain the newline characters. Blank lines do not need a `$._block_close`

        // Special tokens for block structure

        // Closing backticks or tildas for a fenced code block. They are used to trigger a `$._close_block`
        // which in turn will trigger a `$._block_close` at the beginning the following line.
        $._fenced_code_block_end_backtick,
        $._fenced_code_block_end_tilde,

        $._html_block_1_start,
        $._html_block_1_end,
        $._html_block_2_start,
        $._html_block_3_start,
        $._html_block_4_start,
        $._html_block_5_start,
        $._html_block_6_start,
        $._html_block_7_start,

        // Similarly this is used if the closing of a block is not decided by the external parser.
        // A `$._block_close` will be emitted at the beginning of the next line. Notice that a
        // `$._block_close` can also get emitted if the parent block closes.
        $._close_block,

        // This is a workaround so the external parser does not try to open indented blocks when
        // parsing a link reference definition.
        $._no_indented_chunk,

        // An `$._error` token is never valid  and gets emmited to kill invalid parse branches. Concretely
        // this is used to decide wether a newline closes a paragraph and together and it gets emitted
        // when trying to parse the `$._trigger_error` token in `$.link_title`.
        $._error,
        $._trigger_error,
        $._eof,

        $.minus_metadata,
        $.plus_metadata,

        $._pipe_table_start,
        $._pipe_table_line_ending,
    ],
    precedences: $ => [
        [$._setext_heading1, $._block],
        [$._setext_heading2, $._block],
        [$.indented_code_block, $._block],
    ],
    conflicts: $ => [
        [$.link_reference_definition],
        [$.link_label, $._line],
        [$.link_reference_definition, $._line],
    ],
    extras: $ => [],
});

// General purpose structure for html blocks. The different kinds mostly work the same but have
// different openling and closing conditions. Some html blocks may not interrupt a paragraph and
// have to be marked as such.
function build_html_block($, open, close, interrupt_paragraph) {
    return seq(
        open,
        repeat(choice(
            $._line,
            $._newline,
            seq(close, $._close_block),
        )),
        $._block_close,
        optional($.block_continuation),
    );
}
