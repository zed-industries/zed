use super::*;

impl Editor {
    pub fn rewrap(&mut self, options: RewrapOptions, cx: &mut Context<Self>) {
        if self.read_only(cx) || self.mode.is_single_line() {
            return;
        }
        let buffer = self.buffer.read(cx).snapshot(cx);
        let selections = self.selections.all::<Point>(&self.display_snapshot(cx));

        #[derive(Clone, Debug, PartialEq)]
        enum CommentFormat {
            /// single line comment, with prefix for line
            Line(String),
            /// single line within a block comment, with prefix for line
            BlockLine(String),
            /// a single line of a block comment that includes the initial delimiter
            BlockCommentWithStart(BlockCommentConfig),
            /// a single line of a block comment that includes the ending delimiter
            BlockCommentWithEnd(BlockCommentConfig),
        }

        // Split selections to respect paragraph, indent, and comment prefix boundaries.
        let wrap_ranges = selections.into_iter().flat_map(|selection| {
            let language_settings = buffer.language_settings_at(selection.head(), cx);
            let language_scope = buffer.language_scope_at(selection.head());

            let indent_and_prefix_for_row =
                |row: u32| -> (IndentSize, Option<CommentFormat>, Option<String>) {
                    let indent = buffer.indent_size_for_line(MultiBufferRow(row));
                    let (comment_prefix, rewrap_prefix) = if let Some(language_scope) =
                        &language_scope
                    {
                        let indent_end = Point::new(row, indent.len);
                        let line_end = Point::new(row, buffer.line_len(MultiBufferRow(row)));
                        let line_text_after_indent = buffer
                            .text_for_range(indent_end..line_end)
                            .collect::<String>();

                        let is_within_comment_override = buffer
                            .language_scope_at(indent_end)
                            .is_some_and(|scope| scope.override_name() == Some("comment"));
                        let comment_delimiters = if is_within_comment_override {
                            // we are within a comment syntax node, but we don't
                            // yet know what kind of comment: block, doc or line
                            match (
                                language_scope.documentation_comment(),
                                language_scope.block_comment(),
                            ) {
                                (Some(config), _) | (_, Some(config))
                                    if buffer.contains_str_at(indent_end, &config.start) =>
                                {
                                    Some(CommentFormat::BlockCommentWithStart(config.clone()))
                                }
                                (Some(config), _) | (_, Some(config))
                                    if line_text_after_indent.ends_with(config.end.as_ref()) =>
                                {
                                    Some(CommentFormat::BlockCommentWithEnd(config.clone()))
                                }
                                (Some(config), _) | (_, Some(config))
                                    if !config.prefix.is_empty()
                                        && buffer.contains_str_at(indent_end, &config.prefix) =>
                                {
                                    Some(CommentFormat::BlockLine(config.prefix.to_string()))
                                }
                                (_, _) => language_scope
                                    .line_comment_prefixes()
                                    .iter()
                                    .find(|prefix| buffer.contains_str_at(indent_end, prefix))
                                    .map(|prefix| CommentFormat::Line(prefix.to_string())),
                            }
                        } else {
                            // we not in an overridden comment node, but we may
                            // be within a non-overridden line comment node
                            language_scope
                                .line_comment_prefixes()
                                .iter()
                                .find(|prefix| buffer.contains_str_at(indent_end, prefix))
                                .map(|prefix| CommentFormat::Line(prefix.to_string()))
                        };

                        let rewrap_prefix = language_scope
                            .rewrap_prefixes()
                            .iter()
                            .find_map(|prefix_regex| {
                                prefix_regex.find(&line_text_after_indent).map(|mat| {
                                    if mat.start() == 0 {
                                        Some(mat.as_str().to_string())
                                    } else {
                                        None
                                    }
                                })
                            })
                            .flatten();
                        (comment_delimiters, rewrap_prefix)
                    } else {
                        (None, None)
                    };
                    (indent, comment_prefix, rewrap_prefix)
                };

            let mut start_row = selection.start.row;
            let mut end_row = selection.end.row;

            if selection.is_empty() {
                let cursor_row = selection.start.row;

                let (mut indent_size, comment_prefix, _) = indent_and_prefix_for_row(cursor_row);
                let line_prefix = match &comment_prefix {
                    Some(CommentFormat::Line(prefix) | CommentFormat::BlockLine(prefix)) => {
                        Some(prefix.as_str())
                    }
                    Some(CommentFormat::BlockCommentWithEnd(BlockCommentConfig {
                        prefix, ..
                    })) => Some(prefix.as_ref()),
                    Some(CommentFormat::BlockCommentWithStart(BlockCommentConfig {
                        start: _,
                        end: _,
                        prefix,
                        tab_size,
                    })) => {
                        indent_size.len += tab_size;
                        Some(prefix.as_ref())
                    }
                    None => None,
                };
                let indent_prefix = indent_size.chars().collect::<String>();
                let line_prefix = format!("{indent_prefix}{}", line_prefix.unwrap_or(""));

                'expand_upwards: while start_row > 0 {
                    let prev_row = start_row - 1;
                    if buffer.contains_str_at(Point::new(prev_row, 0), &line_prefix)
                        && buffer.line_len(MultiBufferRow(prev_row)) as usize > line_prefix.len()
                        && !buffer.is_line_blank(MultiBufferRow(prev_row))
                    {
                        start_row = prev_row;
                    } else {
                        break 'expand_upwards;
                    }
                }

                'expand_downwards: while end_row < buffer.max_point().row {
                    let next_row = end_row + 1;
                    if buffer.contains_str_at(Point::new(next_row, 0), &line_prefix)
                        && buffer.line_len(MultiBufferRow(next_row)) as usize > line_prefix.len()
                        && !buffer.is_line_blank(MultiBufferRow(next_row))
                    {
                        end_row = next_row;
                    } else {
                        break 'expand_downwards;
                    }
                }
            }

            let mut non_blank_rows_iter = (start_row..=end_row)
                .filter(|row| !buffer.is_line_blank(MultiBufferRow(*row)))
                .peekable();

            let first_row = if let Some(&row) = non_blank_rows_iter.peek() {
                row
            } else {
                return Vec::new();
            };

            let mut ranges = Vec::new();

            let mut current_range_start = first_row;
            let mut prev_row = first_row;
            let (
                mut current_range_indent,
                mut current_range_comment_delimiters,
                mut current_range_rewrap_prefix,
            ) = indent_and_prefix_for_row(first_row);

            for row in non_blank_rows_iter.skip(1) {
                let has_paragraph_break = row > prev_row + 1;

                let (row_indent, row_comment_delimiters, row_rewrap_prefix) =
                    indent_and_prefix_for_row(row);

                let has_indent_change = row_indent != current_range_indent;
                let has_comment_change = row_comment_delimiters != current_range_comment_delimiters;

                let has_boundary_change = has_comment_change
                    || row_rewrap_prefix.is_some()
                    || (has_indent_change && current_range_comment_delimiters.is_some());

                if has_paragraph_break || has_boundary_change {
                    ranges.push((
                        language_settings.clone(),
                        Point::new(current_range_start, 0)
                            ..Point::new(prev_row, buffer.line_len(MultiBufferRow(prev_row))),
                        current_range_indent,
                        current_range_comment_delimiters.clone(),
                        current_range_rewrap_prefix.clone(),
                    ));
                    current_range_start = row;
                    current_range_indent = row_indent;
                    current_range_comment_delimiters = row_comment_delimiters;
                    current_range_rewrap_prefix = row_rewrap_prefix;
                }
                prev_row = row;
            }

            ranges.push((
                language_settings.clone(),
                Point::new(current_range_start, 0)
                    ..Point::new(prev_row, buffer.line_len(MultiBufferRow(prev_row))),
                current_range_indent,
                current_range_comment_delimiters,
                current_range_rewrap_prefix,
            ));

            ranges
        });

        let mut edits = Vec::new();
        let mut rewrapped_row_ranges = Vec::<RangeInclusive<u32>>::new();

        for (language_settings, wrap_range, mut indent_size, comment_prefix, rewrap_prefix) in
            wrap_ranges
        {
            let start_row = wrap_range.start.row;
            let end_row = wrap_range.end.row;

            // Skip selections that overlap with a range that has already been rewrapped.
            let selection_range = start_row..end_row;
            if rewrapped_row_ranges
                .iter()
                .any(|range| range.overlaps(&selection_range))
            {
                continue;
            }

            let tab_size = language_settings.tab_size;

            let (line_prefix, inside_comment) = match &comment_prefix {
                Some(CommentFormat::Line(prefix) | CommentFormat::BlockLine(prefix)) => {
                    (Some(prefix.as_str()), true)
                }
                Some(CommentFormat::BlockCommentWithEnd(BlockCommentConfig { prefix, .. })) => {
                    (Some(prefix.as_ref()), true)
                }
                Some(CommentFormat::BlockCommentWithStart(BlockCommentConfig {
                    start: _,
                    end: _,
                    prefix,
                    tab_size,
                })) => {
                    indent_size.len += tab_size;
                    (Some(prefix.as_ref()), true)
                }
                None => (None, false),
            };
            let indent_prefix = indent_size.chars().collect::<String>();
            let line_prefix = format!("{indent_prefix}{}", line_prefix.unwrap_or(""));

            let allow_rewrap_based_on_language = match language_settings.allow_rewrap {
                RewrapBehavior::InComments => inside_comment,
                RewrapBehavior::InSelections => !wrap_range.is_empty(),
                RewrapBehavior::Anywhere => true,
            };

            let should_rewrap = options.override_language_settings
                || allow_rewrap_based_on_language
                || self.hard_wrap.is_some();
            if !should_rewrap {
                continue;
            }

            let start = Point::new(start_row, 0);
            let start_offset = ToOffset::to_offset(&start, &buffer);
            let end = Point::new(end_row, buffer.line_len(MultiBufferRow(end_row)));
            let selection_text = buffer.text_for_range(start..end).collect::<String>();
            let mut first_line_delimiter = None;
            let mut last_line_delimiter = None;
            let Some(lines_without_prefixes) = selection_text
                .lines()
                .enumerate()
                .map(|(ix, line)| {
                    let line_trimmed = line.trim_start();
                    if rewrap_prefix.is_some() && ix > 0 {
                        Ok(line_trimmed)
                    } else if let Some(
                        CommentFormat::BlockCommentWithStart(BlockCommentConfig {
                            start,
                            prefix,
                            end,
                            tab_size,
                        })
                        | CommentFormat::BlockCommentWithEnd(BlockCommentConfig {
                            start,
                            prefix,
                            end,
                            tab_size,
                        }),
                    ) = &comment_prefix
                    {
                        let line_trimmed = line_trimmed
                            .strip_prefix(start.as_ref())
                            .map(|s| {
                                let mut indent_size = indent_size;
                                indent_size.len -= tab_size;
                                let indent_prefix: String = indent_size.chars().collect();
                                first_line_delimiter = Some((indent_prefix, start));
                                s.trim_start()
                            })
                            .unwrap_or(line_trimmed);
                        let line_trimmed = line_trimmed
                            .strip_suffix(end.as_ref())
                            .map(|s| {
                                last_line_delimiter = Some(end);
                                s.trim_end()
                            })
                            .unwrap_or(line_trimmed);
                        let line_trimmed = line_trimmed
                            .strip_prefix(prefix.as_ref())
                            .unwrap_or(line_trimmed);
                        Ok(line_trimmed)
                    } else if let Some(CommentFormat::BlockLine(prefix)) = &comment_prefix {
                        line_trimmed.strip_prefix(prefix).with_context(|| {
                            format!("line did not start with prefix {prefix:?}: {line:?}")
                        })
                    } else {
                        line_trimmed
                            .strip_prefix(&line_prefix.trim_start())
                            .with_context(|| {
                                format!("line did not start with prefix {line_prefix:?}: {line:?}")
                            })
                    }
                })
                .collect::<Result<Vec<_>, _>>()
                .log_err()
            else {
                continue;
            };

            let wrap_column = options.line_length.or(self.hard_wrap).unwrap_or_else(|| {
                buffer
                    .language_settings_at(Point::new(start_row, 0), cx)
                    .preferred_line_length as usize
            });

            let subsequent_lines_prefix = if let Some(rewrap_prefix_str) = &rewrap_prefix {
                format!("{}{}", indent_prefix, " ".repeat(rewrap_prefix_str.len()))
            } else {
                line_prefix.clone()
            };

            let wrapped_text = {
                let mut wrapped_text = wrap_with_prefix(
                    line_prefix,
                    subsequent_lines_prefix,
                    lines_without_prefixes.join("\n"),
                    wrap_column,
                    tab_size,
                    options.preserve_existing_whitespace,
                );

                if let Some((indent, delimiter)) = first_line_delimiter {
                    wrapped_text = format!("{indent}{delimiter}\n{wrapped_text}");
                }
                if let Some(last_line) = last_line_delimiter {
                    wrapped_text = format!("{wrapped_text}\n{indent_prefix}{last_line}");
                }

                wrapped_text
            };

            // TODO: should always use char-based diff while still supporting cursor behavior that
            // matches vim.
            let mut diff_options = DiffOptions::default();
            if options.override_language_settings {
                diff_options.max_word_diff_len = 0;
                diff_options.max_word_diff_line_count = 0;
            } else {
                diff_options.max_word_diff_len = usize::MAX;
                diff_options.max_word_diff_line_count = usize::MAX;
            }

            for (old_range, new_text) in
                text_diff_with_options(&selection_text, &wrapped_text, diff_options)
            {
                let edit_start = buffer.anchor_after(start_offset + old_range.start);
                let edit_end = buffer.anchor_after(start_offset + old_range.end);
                edits.push((edit_start..edit_end, new_text));
            }

            rewrapped_row_ranges.push(start_row..=end_row);
        }

        self.buffer
            .update(cx, |buffer, cx| buffer.edit(edits, None, cx));
    }
}

fn char_len_with_expanded_tabs(offset: usize, text: &str, tab_size: NonZeroU32) -> usize {
    let tab_size = tab_size.get() as usize;
    let mut width = offset;

    for ch in text.chars() {
        width += if ch == '\t' {
            tab_size - (width % tab_size)
        } else {
            1
        };
    }

    width - offset
}

/// Tokenizes a string into runs of text that should stick together, or that is whitespace.
struct WordBreakingTokenizer<'a> {
    input: &'a str,
}

impl<'a> WordBreakingTokenizer<'a> {
    fn new(input: &'a str) -> Self {
        Self { input }
    }
}

fn is_char_ideographic(ch: char) -> bool {
    use unicode_script::Script::*;
    use unicode_script::UnicodeScript;
    matches!(ch.script(), Han | Tangut | Yi)
}

fn is_grapheme_ideographic(text: &str) -> bool {
    text.chars().any(is_char_ideographic)
}

fn is_grapheme_whitespace(text: &str) -> bool {
    text.chars().any(|x| x.is_whitespace())
}

fn should_stay_with_preceding_ideograph(text: &str) -> bool {
    text.chars()
        .next()
        .is_some_and(|ch| matches!(ch, '。' | '、' | '，' | '？' | '！' | '：' | '；' | '…'))
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum WordBreakToken<'a> {
    Word { token: &'a str, grapheme_len: usize },
    InlineWhitespace { token: &'a str, grapheme_len: usize },
    Newline,
}

impl<'a> Iterator for WordBreakingTokenizer<'a> {
    /// Yields a span, the count of graphemes in the token, and whether it was
    /// whitespace. Note that it also breaks at word boundaries.
    type Item = WordBreakToken<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        use unicode_segmentation::UnicodeSegmentation;
        if self.input.is_empty() {
            return None;
        }

        let mut iter = self.input.graphemes(true).peekable();
        let mut offset = 0;
        let mut grapheme_len = 0;
        if let Some(first_grapheme) = iter.next() {
            let is_newline = first_grapheme == "\n";
            let is_whitespace = is_grapheme_whitespace(first_grapheme);
            offset += first_grapheme.len();
            grapheme_len += 1;
            if is_grapheme_ideographic(first_grapheme) && !is_whitespace {
                if let Some(grapheme) = iter.peek().copied()
                    && should_stay_with_preceding_ideograph(grapheme)
                {
                    offset += grapheme.len();
                    grapheme_len += 1;
                }
            } else {
                let mut words = self.input[offset..].split_word_bound_indices().peekable();
                let mut next_word_bound = words.peek().copied();
                if next_word_bound.is_some_and(|(i, _)| i == 0) {
                    next_word_bound = words.next();
                }
                while let Some(grapheme) = iter.peek().copied() {
                    if next_word_bound.is_some_and(|(i, _)| i == offset) {
                        break;
                    };
                    if is_grapheme_whitespace(grapheme) != is_whitespace
                        || (grapheme == "\n") != is_newline
                    {
                        break;
                    };
                    offset += grapheme.len();
                    grapheme_len += 1;
                    iter.next();
                }
            }
            let token = &self.input[..offset];
            self.input = &self.input[offset..];
            if token == "\n" {
                Some(WordBreakToken::Newline)
            } else if is_whitespace {
                Some(WordBreakToken::InlineWhitespace {
                    token,
                    grapheme_len,
                })
            } else {
                Some(WordBreakToken::Word {
                    token,
                    grapheme_len,
                })
            }
        } else {
            None
        }
    }
}

fn wrap_with_prefix(
    first_line_prefix: String,
    subsequent_lines_prefix: String,
    unwrapped_text: String,
    wrap_column: usize,
    tab_size: NonZeroU32,
    preserve_existing_whitespace: bool,
) -> String {
    let first_line_prefix_len = char_len_with_expanded_tabs(0, &first_line_prefix, tab_size);
    let subsequent_lines_prefix_len =
        char_len_with_expanded_tabs(0, &subsequent_lines_prefix, tab_size);
    let mut wrapped_text = String::new();
    let mut current_line = first_line_prefix;
    let mut is_first_line = true;

    let tokenizer = WordBreakingTokenizer::new(&unwrapped_text);
    let mut current_line_len = first_line_prefix_len;
    let mut in_whitespace = false;
    for token in tokenizer {
        let have_preceding_whitespace = in_whitespace;
        match token {
            WordBreakToken::Word {
                token,
                grapheme_len,
            } => {
                in_whitespace = false;
                let current_prefix_len = if is_first_line {
                    first_line_prefix_len
                } else {
                    subsequent_lines_prefix_len
                };
                if current_line_len + grapheme_len > wrap_column
                    && current_line_len != current_prefix_len
                {
                    wrapped_text.push_str(current_line.trim_end());
                    wrapped_text.push('\n');
                    is_first_line = false;
                    current_line = subsequent_lines_prefix.clone();
                    current_line_len = subsequent_lines_prefix_len;
                }
                current_line.push_str(token);
                current_line_len += grapheme_len;
            }
            WordBreakToken::InlineWhitespace {
                mut token,
                mut grapheme_len,
            } => {
                in_whitespace = true;
                if have_preceding_whitespace && !preserve_existing_whitespace {
                    continue;
                }
                if !preserve_existing_whitespace {
                    // Keep a single whitespace grapheme as-is
                    if let Some(first) =
                        unicode_segmentation::UnicodeSegmentation::graphemes(token, true).next()
                    {
                        token = first;
                    } else {
                        token = " ";
                    }
                    grapheme_len = 1;
                }
                let current_prefix_len = if is_first_line {
                    first_line_prefix_len
                } else {
                    subsequent_lines_prefix_len
                };
                if current_line_len + grapheme_len > wrap_column {
                    wrapped_text.push_str(current_line.trim_end());
                    wrapped_text.push('\n');
                    is_first_line = false;
                    current_line = subsequent_lines_prefix.clone();
                    current_line_len = subsequent_lines_prefix_len;
                } else if current_line_len != current_prefix_len || preserve_existing_whitespace {
                    current_line.push_str(token);
                    current_line_len += grapheme_len;
                }
            }
            WordBreakToken::Newline => {
                in_whitespace = true;
                let current_prefix_len = if is_first_line {
                    first_line_prefix_len
                } else {
                    subsequent_lines_prefix_len
                };
                if preserve_existing_whitespace {
                    wrapped_text.push_str(current_line.trim_end());
                    wrapped_text.push('\n');
                    is_first_line = false;
                    current_line = subsequent_lines_prefix.clone();
                    current_line_len = subsequent_lines_prefix_len;
                } else if have_preceding_whitespace {
                    continue;
                } else if current_line_len + 1 > wrap_column
                    && current_line_len != current_prefix_len
                {
                    wrapped_text.push_str(current_line.trim_end());
                    wrapped_text.push('\n');
                    is_first_line = false;
                    current_line = subsequent_lines_prefix.clone();
                    current_line_len = subsequent_lines_prefix_len;
                } else if current_line_len != current_prefix_len {
                    current_line.push(' ');
                    current_line_len += 1;
                }
            }
        }
    }

    if !current_line.is_empty() {
        wrapped_text.push_str(&current_line);
    }
    wrapped_text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_size_with_expanded_tabs() {
        let nz = |val| NonZeroU32::new(val).unwrap();
        assert_eq!(char_len_with_expanded_tabs(0, "", nz(4)), 0);
        assert_eq!(char_len_with_expanded_tabs(0, "hello", nz(4)), 5);
        assert_eq!(char_len_with_expanded_tabs(0, "\thello", nz(4)), 9);
        assert_eq!(char_len_with_expanded_tabs(0, "abc\tab", nz(4)), 6);
        assert_eq!(char_len_with_expanded_tabs(0, "hello\t", nz(4)), 8);
        assert_eq!(char_len_with_expanded_tabs(0, "\t\t", nz(8)), 16);
        assert_eq!(char_len_with_expanded_tabs(0, "x\t", nz(8)), 8);
        assert_eq!(char_len_with_expanded_tabs(7, "x\t", nz(8)), 9);
    }

    #[test]
    fn test_word_breaking_tokenizer() {
        let tests: &[(&str, &[WordBreakToken<'static>])] = &[
            ("", &[]),
            ("  ", &[whitespace("  ", 2)]),
            ("Ʒ", &[word("Ʒ", 1)]),
            ("Ǽ", &[word("Ǽ", 1)]),
            ("⋑", &[word("⋑", 1)]),
            ("⋑⋑", &[word("⋑⋑", 2)]),
            (
                "原理，进而",
                &[word("原", 1), word("理，", 2), word("进", 1), word("而", 1)],
            ),
            (
                "hello world",
                &[word("hello", 5), whitespace(" ", 1), word("world", 5)],
            ),
            (
                "hello, world",
                &[word("hello,", 6), whitespace(" ", 1), word("world", 5)],
            ),
            (
                "  hello world",
                &[
                    whitespace("  ", 2),
                    word("hello", 5),
                    whitespace(" ", 1),
                    word("world", 5),
                ],
            ),
            (
                "这是什么 \n 钢笔",
                &[
                    word("这", 1),
                    word("是", 1),
                    word("什", 1),
                    word("么", 1),
                    whitespace(" ", 1),
                    newline(),
                    whitespace(" ", 1),
                    word("钢", 1),
                    word("笔", 1),
                ],
            ),
            (" mutton", &[whitespace(" ", 1), word("mutton", 6)]),
        ];

        fn word(token: &'static str, grapheme_len: usize) -> WordBreakToken<'static> {
            WordBreakToken::Word {
                token,
                grapheme_len,
            }
        }

        fn whitespace(token: &'static str, grapheme_len: usize) -> WordBreakToken<'static> {
            WordBreakToken::InlineWhitespace {
                token,
                grapheme_len,
            }
        }

        fn newline() -> WordBreakToken<'static> {
            WordBreakToken::Newline
        }

        for (input, result) in tests {
            assert_eq!(
                WordBreakingTokenizer::new(input)
                    .collect::<Vec<_>>()
                    .as_slice(),
                *result,
            );
        }
    }

    #[test]
    fn test_wrap_with_prefix() {
        assert_eq!(
            wrap_with_prefix(
                "# ".to_string(),
                "# ".to_string(),
                "abcdefg".to_string(),
                4,
                NonZeroU32::new(4).unwrap(),
                false,
            ),
            "# abcdefg"
        );
        assert_eq!(
            wrap_with_prefix(
                "".to_string(),
                "".to_string(),
                "\thello world".to_string(),
                8,
                NonZeroU32::new(4).unwrap(),
                false,
            ),
            "hello\nworld"
        );
        assert_eq!(
            wrap_with_prefix(
                "// ".to_string(),
                "// ".to_string(),
                "xx \nyy zz aa bb cc".to_string(),
                12,
                NonZeroU32::new(4).unwrap(),
                false,
            ),
            "// xx yy zz\n// aa bb cc"
        );
        assert_eq!(
            wrap_with_prefix(
                String::new(),
                String::new(),
                "这是什么 \n 钢笔".to_string(),
                3,
                NonZeroU32::new(4).unwrap(),
                false,
            ),
            "这是什\n么 钢\n笔"
        );
        assert_eq!(
            wrap_with_prefix(
                String::new(),
                String::new(),
                format!("foo{}bar", '\u{2009}'), // thin space
                80,
                NonZeroU32::new(4).unwrap(),
                false,
            ),
            format!("foo{}bar", '\u{2009}')
        );
    }
}
