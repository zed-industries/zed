// Most of this file is adapted from https://github.com/rust-lang/rust/blob/160905b6253f42967ed4aef4b98002944c7df24c/compiler/rustc_errors/src/emitter.rs

use alloc::borrow::{Cow, ToOwned};
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::{format, vec, vec::Vec};
use core::cmp::{max, min, Ordering, Reverse};
use core::fmt;

use anstyle::Style;

use super::margin::Margin;
use super::stylesheet::Stylesheet;
use super::DecorStyle;
use super::Renderer;
use crate::level::{Level, LevelInner};
use crate::renderer::source_map::{
    AnnotatedLineInfo, LineInfo, Loc, SourceMap, SplicedLines, SubstitutionHighlight, TrimmedPatch,
};
use crate::renderer::styled_buffer::StyledBuffer;
use crate::snippet::Id;
use crate::{
    Annotation, AnnotationKind, Element, Group, Message, Origin, Padding, Patch, Report, Snippet,
    Title,
};

const ANONYMIZED_LINE_NUM: &str = "LL";

pub(crate) fn render(renderer: &Renderer, groups: Report<'_>) -> String {
    if renderer.short_message {
        render_short_message(renderer, groups).unwrap()
    } else {
        let (max_line_num, og_primary_path, groups) = pre_process(groups);
        let max_line_num_len = if renderer.anonymized_line_numbers {
            ANONYMIZED_LINE_NUM.len()
        } else {
            num_decimal_digits(max_line_num)
        };
        let mut out_string = String::new();
        let group_len = groups.len();
        for (
            g,
            PreProcessedGroup {
                group,
                elements,
                primary_path,
                max_depth,
            },
        ) in groups.into_iter().enumerate()
        {
            let mut buffer = StyledBuffer::new();
            let level = group.primary_level.clone();
            let mut message_iter = elements.into_iter().enumerate().peekable();
            if let Some(title) = &group.title {
                let peek = message_iter.peek().map(|(_, s)| s);
                let title_style = if title.allows_styling {
                    TitleStyle::Header
                } else {
                    TitleStyle::MainHeader
                };
                let buffer_msg_line_offset = buffer.num_lines();
                render_title(
                    renderer,
                    &mut buffer,
                    title,
                    max_line_num_len,
                    title_style,
                    matches!(peek, Some(PreProcessedElement::Message(_))),
                    buffer_msg_line_offset,
                );
                let buffer_msg_line_offset = buffer.num_lines();

                if matches!(peek, Some(PreProcessedElement::Message(_))) {
                    draw_col_separator_no_space(
                        renderer,
                        &mut buffer,
                        buffer_msg_line_offset,
                        max_line_num_len + 1,
                    );
                }
                if peek.is_none()
                    && title_style == TitleStyle::MainHeader
                    && g == 0
                    && group_len > 1
                {
                    draw_col_separator_end(
                        renderer,
                        &mut buffer,
                        buffer_msg_line_offset,
                        max_line_num_len + 1,
                    );
                }
            }
            let mut seen_primary = false;
            let mut last_suggestion_path = None;
            while let Some((i, section)) = message_iter.next() {
                let peek = message_iter.peek().map(|(_, s)| s);
                let is_first = i == 0;
                match section {
                    PreProcessedElement::Message(title) => {
                        let title_style = TitleStyle::Secondary;
                        let buffer_msg_line_offset = buffer.num_lines();
                        render_title(
                            renderer,
                            &mut buffer,
                            title,
                            max_line_num_len,
                            title_style,
                            peek.is_some(),
                            buffer_msg_line_offset,
                        );
                    }
                    PreProcessedElement::Cause((cause, source_map, annotated_lines)) => {
                        let is_primary = primary_path == cause.path.as_ref() && !seen_primary;
                        seen_primary |= is_primary;
                        render_snippet_annotations(
                            renderer,
                            &mut buffer,
                            max_line_num_len,
                            cause,
                            is_primary,
                            &source_map,
                            &annotated_lines,
                            max_depth,
                            peek.is_some() || (g == 0 && group_len > 1),
                            is_first,
                        );

                        if g == 0 {
                            let current_line = buffer.num_lines();
                            match peek {
                                Some(PreProcessedElement::Message(_)) => {
                                    draw_col_separator_no_space(
                                        renderer,
                                        &mut buffer,
                                        current_line,
                                        max_line_num_len + 1,
                                    );
                                }
                                None if group_len > 1 => draw_col_separator_end(
                                    renderer,
                                    &mut buffer,
                                    current_line,
                                    max_line_num_len + 1,
                                ),
                                _ => {}
                            }
                        }
                    }
                    PreProcessedElement::Suggestion((
                        suggestion,
                        source_map,
                        spliced_lines,
                        display_suggestion,
                    )) => {
                        let matches_previous_suggestion =
                            last_suggestion_path == Some(suggestion.path.as_ref());
                        emit_suggestion_default(
                            renderer,
                            &mut buffer,
                            suggestion,
                            spliced_lines,
                            display_suggestion,
                            max_line_num_len,
                            &source_map,
                            primary_path.or(og_primary_path),
                            matches_previous_suggestion,
                            is_first,
                            //matches!(peek, Some(Element::Message(_) | Element::Padding(_))),
                            peek.is_some(),
                        );

                        if matches!(peek, Some(PreProcessedElement::Suggestion(_))) {
                            last_suggestion_path = Some(suggestion.path.as_ref());
                        } else {
                            last_suggestion_path = None;
                        }
                    }

                    PreProcessedElement::Origin(origin) => {
                        let buffer_msg_line_offset = buffer.num_lines();
                        let is_primary = primary_path == Some(&origin.path) && !seen_primary;
                        seen_primary |= is_primary;
                        render_origin(
                            renderer,
                            &mut buffer,
                            max_line_num_len,
                            origin,
                            is_primary,
                            is_first,
                            peek.is_none(),
                            buffer_msg_line_offset,
                        );
                        let current_line = buffer.num_lines();
                        if g == 0 && peek.is_none() && group_len > 1 {
                            draw_col_separator_end(
                                renderer,
                                &mut buffer,
                                current_line,
                                max_line_num_len + 1,
                            );
                        }
                    }
                    PreProcessedElement::Padding(_) => {
                        let current_line = buffer.num_lines();
                        if peek.is_none() {
                            draw_col_separator_end(
                                renderer,
                                &mut buffer,
                                current_line,
                                max_line_num_len + 1,
                            );
                        } else {
                            draw_col_separator_no_space(
                                renderer,
                                &mut buffer,
                                current_line,
                                max_line_num_len + 1,
                            );
                        }
                    }
                }
            }
            buffer
                .render(&level, &renderer.stylesheet, &mut out_string)
                .unwrap();
            if g != group_len - 1 {
                use core::fmt::Write;

                writeln!(out_string).unwrap();
            }
        }
        out_string
    }
}

fn render_short_message(renderer: &Renderer, groups: &[Group<'_>]) -> Result<String, fmt::Error> {
    let mut buffer = StyledBuffer::new();
    let mut labels = None;
    let group = groups.first().expect("Expected at least one group");

    let Some(title) = &group.title else {
        panic!("Expected a Title");
    };

    if let Some(Element::Cause(cause)) = group
        .elements
        .iter()
        .find(|e| matches!(e, Element::Cause(_)))
    {
        let labels_inner = cause
            .markers
            .iter()
            .filter_map(|ann| match &ann.label {
                Some(msg) if ann.kind.is_primary() => {
                    if !msg.trim().is_empty() {
                        Some(msg.to_string())
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(", ");
        if !labels_inner.is_empty() {
            labels = Some(labels_inner);
        }

        if let Some(path) = &cause.path {
            let mut origin = Origin::path(path.as_ref());

            let source_map = SourceMap::new(&cause.source, cause.line_start);
            let (_depth, annotated_lines) =
                source_map.annotated_lines(cause.markers.clone(), cause.fold);

            if let Some(primary_line) = annotated_lines
                .iter()
                .find(|l| l.annotations.iter().any(LineAnnotation::is_primary))
                .or(annotated_lines.iter().find(|l| !l.annotations.is_empty()))
            {
                origin.line = Some(primary_line.line_index);
                if let Some(first_annotation) = primary_line
                    .annotations
                    .iter()
                    .min_by_key(|a| (Reverse(a.is_primary()), a.start.char))
                {
                    origin.char_column = Some(first_annotation.start.char + 1);
                }
            }

            render_origin(renderer, &mut buffer, 0, &origin, true, true, true, 0);
            buffer.append(0, ": ", ElementStyle::LineAndColumn);
        }
    }

    render_title(
        renderer,
        &mut buffer,
        title,
        0, // No line numbers in short messages
        TitleStyle::MainHeader,
        false,
        0,
    );

    if let Some(labels) = labels {
        buffer.append(0, &format!(": {labels}"), ElementStyle::NoStyle);
    }

    let mut out_string = String::new();
    buffer.render(&title.level, &renderer.stylesheet, &mut out_string)?;

    Ok(out_string)
}

#[allow(clippy::too_many_arguments)]
fn render_title(
    renderer: &Renderer,
    buffer: &mut StyledBuffer,
    title: &dyn MessageOrTitle,
    max_line_num_len: usize,
    title_style: TitleStyle,
    is_cont: bool,
    buffer_msg_line_offset: usize,
) {
    let (label_style, title_element_style) = match title_style {
        TitleStyle::MainHeader => (
            ElementStyle::Level(title.level().level),
            if renderer.short_message {
                ElementStyle::NoStyle
            } else {
                ElementStyle::MainHeaderMsg
            },
        ),
        TitleStyle::Header => (
            ElementStyle::Level(title.level().level),
            ElementStyle::HeaderMsg,
        ),
        TitleStyle::Secondary => {
            for _ in 0..max_line_num_len {
                buffer.prepend(buffer_msg_line_offset, " ", ElementStyle::NoStyle);
            }

            draw_note_separator(
                renderer,
                buffer,
                buffer_msg_line_offset,
                max_line_num_len + 1,
                is_cont,
            );
            (ElementStyle::MainHeaderMsg, ElementStyle::NoStyle)
        }
    };
    let mut label_width = 0;

    if title.level().name != Some(None) {
        buffer.append(buffer_msg_line_offset, title.level().as_str(), label_style);
        label_width += title.level().as_str().len();
        if let Some(Id { id: Some(id), url }) = &title.id() {
            buffer.append(buffer_msg_line_offset, "[", label_style);
            if let Some(url) = url.as_ref() {
                buffer.append(
                    buffer_msg_line_offset,
                    &format!("\x1B]8;;{url}\x1B\\"),
                    label_style,
                );
            }
            buffer.append(buffer_msg_line_offset, id, label_style);
            if url.is_some() {
                buffer.append(buffer_msg_line_offset, "\x1B]8;;\x1B\\", label_style);
            }
            buffer.append(buffer_msg_line_offset, "]", label_style);
            label_width += 2 + id.len();
        }
        buffer.append(buffer_msg_line_offset, ": ", title_element_style);
        label_width += 2;
    }

    let padding = " ".repeat(if title_style == TitleStyle::Secondary {
        // The extra 3 ` ` is padding that's always needed to align to the
        // label i.e. `note: `:
        //
        //   error: message
        //     --> file.rs:13:20
        //      |
        //   13 |     <CODE>
        //      |      ^^^^
        //      |
        //      = note: multiline
        //              message
        //   ++^^^------
        //    |  |     |
        //    |  |     |
        //    |  |     width of label
        //    |  magic `3`
        //    `max_line_num_len`
        max_line_num_len + 3 + label_width
    } else {
        label_width
    });

    let (title_str, style) = if title.allows_styling() {
        (title.text().to_owned(), ElementStyle::NoStyle)
    } else {
        (normalize_whitespace(title.text()), title_element_style)
    };
    for (i, text) in title_str.split('\n').enumerate() {
        if i != 0 {
            buffer.append(buffer_msg_line_offset + i, &padding, ElementStyle::NoStyle);
            if title_style == TitleStyle::Secondary
                && is_cont
                && matches!(renderer.decor_style, DecorStyle::Unicode)
            {
                // There's another note after this one, associated to the subwindow above.
                // We write additional vertical lines to join them:
                //   ╭▸ test.rs:3:3
                //   │
                // 3 │   code
                //   │   ━━━━
                //   │
                //   ├ note: foo
                //   │       bar
                //   ╰ note: foo
                //           bar
                draw_col_separator_no_space(
                    renderer,
                    buffer,
                    buffer_msg_line_offset + i,
                    max_line_num_len + 1,
                );
            }
        }
        buffer.append(buffer_msg_line_offset + i, text, style);
    }
}

#[allow(clippy::too_many_arguments)]
fn render_origin(
    renderer: &Renderer,
    buffer: &mut StyledBuffer,
    max_line_num_len: usize,
    origin: &Origin<'_>,
    is_primary: bool,
    is_first: bool,
    alone: bool,
    buffer_msg_line_offset: usize,
) {
    if is_primary && !renderer.short_message {
        buffer.prepend(
            buffer_msg_line_offset,
            renderer.decor_style.file_start(is_first, alone),
            ElementStyle::LineNumber,
        );
    } else if !renderer.short_message {
        // if !origin.standalone {
        //     // Add spacing line, as shown:
        //     //   --> $DIR/file:54:15
        //     //    |
        //     // LL |         code
        //     //    |         ^^^^
        //     //    | (<- It prints *this* line)
        //     //   ::: $DIR/other_file.rs:15:5
        //     //    |
        //     // LL |     code
        //     //    |     ----
        //     draw_col_separator_no_space(renderer,
        //         buffer,
        //         buffer_msg_line_offset,
        //         max_line_num_len + 1,
        //     );
        //
        //     buffer_msg_line_offset += 1;
        // }
        // Then, the secondary file indicator
        buffer.prepend(
            buffer_msg_line_offset,
            renderer.decor_style.secondary_file_start(),
            ElementStyle::LineNumber,
        );
    }

    let str = match (&origin.line, &origin.char_column) {
        (Some(line), Some(col)) => {
            format!("{}:{}:{}", origin.path, line, col)
        }
        (Some(line), None) => format!("{}:{}", origin.path, line),
        _ => origin.path.to_string(),
    };

    buffer.append(buffer_msg_line_offset, &str, ElementStyle::LineAndColumn);
    if !renderer.short_message {
        for _ in 0..max_line_num_len {
            buffer.prepend(buffer_msg_line_offset, " ", ElementStyle::NoStyle);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn render_snippet_annotations(
    renderer: &Renderer,
    buffer: &mut StyledBuffer,
    max_line_num_len: usize,
    snippet: &Snippet<'_, Annotation<'_>>,
    is_primary: bool,
    sm: &SourceMap<'_>,
    annotated_lines: &[AnnotatedLineInfo<'_>],
    multiline_depth: usize,
    is_cont: bool,
    is_first: bool,
) {
    if let Some(path) = &snippet.path {
        let mut origin = Origin::path(path.as_ref());
        // print out the span location and spacer before we print the annotated source
        // to do this, we need to know if this span will be primary
        //let is_primary = primary_path == Some(&origin.path);

        if is_primary {
            if let Some(primary_line) = annotated_lines
                .iter()
                .find(|l| l.annotations.iter().any(LineAnnotation::is_primary))
                .or(annotated_lines.iter().find(|l| !l.annotations.is_empty()))
            {
                origin.line = Some(primary_line.line_index);
                if let Some(first_annotation) = primary_line
                    .annotations
                    .iter()
                    .min_by_key(|a| (Reverse(a.is_primary()), a.start.char))
                {
                    origin.char_column = Some(first_annotation.start.char + 1);
                }
            }
        } else {
            let buffer_msg_line_offset = buffer.num_lines();
            // Add spacing line, as shown:
            //   --> $DIR/file:54:15
            //    |
            // LL |         code
            //    |         ^^^^
            //    | (<- It prints *this* line)
            //   ::: $DIR/other_file.rs:15:5
            //    |
            // LL |     code
            //    |     ----
            draw_col_separator_no_space(
                renderer,
                buffer,
                buffer_msg_line_offset,
                max_line_num_len + 1,
            );
            if let Some(first_line) = annotated_lines.first() {
                origin.line = Some(first_line.line_index);
                if let Some(first_annotation) = first_line.annotations.first() {
                    origin.char_column = Some(first_annotation.start.char + 1);
                }
            }
        }
        let buffer_msg_line_offset = buffer.num_lines();
        render_origin(
            renderer,
            buffer,
            max_line_num_len,
            &origin,
            is_primary,
            is_first,
            false,
            buffer_msg_line_offset,
        );
        // Put in the spacer between the location and annotated source
        draw_col_separator_no_space(
            renderer,
            buffer,
            buffer_msg_line_offset + 1,
            max_line_num_len + 1,
        );
    } else {
        let buffer_msg_line_offset = buffer.num_lines();
        if is_primary {
            if renderer.decor_style == DecorStyle::Unicode {
                buffer.puts(
                    buffer_msg_line_offset,
                    max_line_num_len,
                    renderer.decor_style.file_start(is_first, false),
                    ElementStyle::LineNumber,
                );
            } else {
                draw_col_separator_no_space(
                    renderer,
                    buffer,
                    buffer_msg_line_offset,
                    max_line_num_len + 1,
                );
            }
        } else {
            // Add spacing line, as shown:
            //   --> $DIR/file:54:15
            //    |
            // LL |         code
            //    |         ^^^^
            //    | (<- It prints *this* line)
            //   ::: $DIR/other_file.rs:15:5
            //    |
            // LL |     code
            //    |     ----
            draw_col_separator_no_space(
                renderer,
                buffer,
                buffer_msg_line_offset,
                max_line_num_len + 1,
            );

            buffer.puts(
                buffer_msg_line_offset + 1,
                max_line_num_len,
                renderer.decor_style.secondary_file_start(),
                ElementStyle::LineNumber,
            );
        }
    }

    // Contains the vertical lines' positions for active multiline annotations
    let mut multilines = Vec::new();

    // Get the left-side margin to remove it
    let mut whitespace_margin = usize::MAX;
    for line_info in annotated_lines {
        let leading_whitespace = line_info
            .line
            .chars()
            .take_while(|c| c.is_whitespace())
            .map(|c| {
                match c {
                    // Tabs are displayed as 4 spaces
                    '\t' => 4,
                    _ => 1,
                }
            })
            .sum();
        if line_info.line.chars().any(|c| !c.is_whitespace()) {
            whitespace_margin = min(whitespace_margin, leading_whitespace);
        }
    }
    if whitespace_margin == usize::MAX {
        whitespace_margin = 0;
    }

    // Left-most column any visible span points at.
    let mut span_left_margin = usize::MAX;
    for line_info in annotated_lines {
        for ann in &line_info.annotations {
            span_left_margin = min(span_left_margin, ann.start.display);
            span_left_margin = min(span_left_margin, ann.end.display);
        }
    }
    if span_left_margin == usize::MAX {
        span_left_margin = 0;
    }

    // Right-most column any visible span points at.
    let mut span_right_margin = 0;
    let mut label_right_margin = 0;
    let mut max_line_len = 0;
    for line_info in annotated_lines {
        max_line_len = max(max_line_len, str_width(line_info.line));
        for ann in &line_info.annotations {
            span_right_margin = max(span_right_margin, ann.start.display);
            span_right_margin = max(span_right_margin, ann.end.display);
            // FIXME: account for labels not in the same line
            let label_right = ann.label.as_ref().map_or(0, |l| str_width(l) + 1);
            label_right_margin = max(label_right_margin, ann.end.display + label_right);
        }
    }
    let width_offset = 3 + max_line_num_len;
    let code_offset = if multiline_depth == 0 {
        width_offset
    } else {
        width_offset + multiline_depth + 1
    };

    let column_width = renderer.term_width.saturating_sub(code_offset);

    let margin = Margin::new(
        whitespace_margin,
        span_left_margin,
        span_right_margin,
        label_right_margin,
        column_width,
        max_line_len,
    );

    // Next, output the annotate source for this file
    for annotated_line_idx in 0..annotated_lines.len() {
        let previous_buffer_line = buffer.num_lines();

        let depths = render_source_line(
            renderer,
            &annotated_lines[annotated_line_idx],
            buffer,
            width_offset,
            code_offset,
            max_line_num_len,
            margin,
            !is_cont && annotated_line_idx + 1 == annotated_lines.len(),
        );

        let mut to_add = BTreeMap::new();

        for (depth, style) in depths {
            if let Some(index) = multilines.iter().position(|(d, _)| d == &depth) {
                multilines.swap_remove(index);
            } else {
                to_add.insert(depth, style);
            }
        }

        // Set the multiline annotation vertical lines to the left of
        // the code in this line.
        for (depth, style) in &multilines {
            for line in previous_buffer_line..buffer.num_lines() {
                draw_multiline_line(renderer, buffer, line, width_offset, *depth, *style);
            }
        }
        // check to see if we need to print out or elide lines that come between
        // this annotated line and the next one.
        if annotated_line_idx < (annotated_lines.len() - 1) {
            let line_idx_delta = annotated_lines[annotated_line_idx + 1].line_index
                - annotated_lines[annotated_line_idx].line_index;
            match line_idx_delta.cmp(&2) {
                Ordering::Greater => {
                    let last_buffer_line_num = buffer.num_lines();

                    draw_line_separator(renderer, buffer, last_buffer_line_num, width_offset);

                    // Set the multiline annotation vertical lines on `...` bridging line.
                    for (depth, style) in &multilines {
                        draw_multiline_line(
                            renderer,
                            buffer,
                            last_buffer_line_num,
                            width_offset,
                            *depth,
                            *style,
                        );
                    }
                    if let Some(line) = annotated_lines.get(annotated_line_idx) {
                        for ann in &line.annotations {
                            if let LineAnnotationType::MultilineStart(pos) = ann.annotation_type {
                                // In the case where we have elided the entire start of the
                                // multispan because those lines were empty, we still need
                                // to draw the `|`s across the `...`.
                                draw_multiline_line(
                                    renderer,
                                    buffer,
                                    last_buffer_line_num,
                                    width_offset,
                                    pos,
                                    if ann.is_primary() {
                                        ElementStyle::UnderlinePrimary
                                    } else {
                                        ElementStyle::UnderlineSecondary
                                    },
                                );
                            }
                        }
                    }
                }

                Ordering::Equal => {
                    let unannotated_line = sm
                        .get_line(annotated_lines[annotated_line_idx].line_index + 1)
                        .unwrap_or("");

                    let last_buffer_line_num = buffer.num_lines();

                    draw_line(
                        renderer,
                        buffer,
                        &normalize_whitespace(unannotated_line),
                        annotated_lines[annotated_line_idx + 1].line_index - 1,
                        last_buffer_line_num,
                        width_offset,
                        code_offset,
                        max_line_num_len,
                        margin,
                    );

                    for (depth, style) in &multilines {
                        draw_multiline_line(
                            renderer,
                            buffer,
                            last_buffer_line_num,
                            width_offset,
                            *depth,
                            *style,
                        );
                    }
                    if let Some(line) = annotated_lines.get(annotated_line_idx) {
                        for ann in &line.annotations {
                            if let LineAnnotationType::MultilineStart(pos) = ann.annotation_type {
                                draw_multiline_line(
                                    renderer,
                                    buffer,
                                    last_buffer_line_num,
                                    width_offset,
                                    pos,
                                    if ann.is_primary() {
                                        ElementStyle::UnderlinePrimary
                                    } else {
                                        ElementStyle::UnderlineSecondary
                                    },
                                );
                            }
                        }
                    }
                }
                Ordering::Less => {}
            }
        }

        multilines.extend(to_add);
    }
}

#[allow(clippy::too_many_arguments)]
fn render_source_line(
    renderer: &Renderer,
    line_info: &AnnotatedLineInfo<'_>,
    buffer: &mut StyledBuffer,
    width_offset: usize,
    code_offset: usize,
    max_line_num_len: usize,
    margin: Margin,
    close_window: bool,
) -> Vec<(usize, ElementStyle)> {
    // Draw:
    //
    //   LL | ... code ...
    //      |     ^^-^ span label
    //      |       |
    //      |       secondary span label
    //
    //   ^^ ^ ^^^ ^^^^ ^^^ we don't care about code too far to the right of a span, we trim it
    //   |  | |   |
    //   |  | |   actual code found in your source code and the spans we use to mark it
    //   |  | when there's too much wasted space to the left, trim it
    //   |  vertical divider between the column number and the code
    //   column number

    let source_string = normalize_whitespace(line_info.line);

    let line_offset = buffer.num_lines();

    let left = draw_line(
        renderer,
        buffer,
        &source_string,
        line_info.line_index,
        line_offset,
        width_offset,
        code_offset,
        max_line_num_len,
        margin,
    );

    // If there are no annotations, we are done
    if line_info.annotations.is_empty() {
        // `close_window` normally gets handled later, but we are early
        // returning, so it needs to be handled here
        if close_window {
            draw_col_separator_end(renderer, buffer, line_offset + 1, width_offset - 2);
        }
        return vec![];
    }

    // Special case when there's only one annotation involved, it is the start of a multiline
    // span and there's no text at the beginning of the code line. Instead of doing the whole
    // graph:
    //
    // 2 |   fn foo() {
    //   |  _^
    // 3 | |
    // 4 | | }
    //   | |_^ test
    //
    // we simplify the output to:
    //
    // 2 | / fn foo() {
    // 3 | |
    // 4 | | }
    //   | |_^ test
    let mut buffer_ops = vec![];
    let mut annotations = vec![];
    let mut short_start = true;
    for ann in &line_info.annotations {
        if let LineAnnotationType::MultilineStart(depth) = ann.annotation_type {
            if source_string
                .chars()
                .take(ann.start.display)
                .all(char::is_whitespace)
            {
                let uline = renderer.decor_style.underline(ann.is_primary());
                let chr = uline.multiline_whole_line;
                annotations.push((depth, uline.style));
                buffer_ops.push((line_offset, width_offset + depth - 1, chr, uline.style));
            } else {
                short_start = false;
                break;
            }
        } else if let LineAnnotationType::MultilineLine(_) = ann.annotation_type {
        } else {
            short_start = false;
            break;
        }
    }
    if short_start {
        for (y, x, c, s) in buffer_ops {
            buffer.putc(y, x, c, s);
        }
        return annotations;
    }

    // We want to display like this:
    //
    //      vec.push(vec.pop().unwrap());
    //      ---      ^^^               - previous borrow ends here
    //      |        |
    //      |        error occurs here
    //      previous borrow of `vec` occurs here
    //
    // But there are some weird edge cases to be aware of:
    //
    //      vec.push(vec.pop().unwrap());
    //      --------                    - previous borrow ends here
    //      ||
    //      |this makes no sense
    //      previous borrow of `vec` occurs here
    //
    // For this reason, we group the lines into "highlight lines"
    // and "annotations lines", where the highlight lines have the `^`.

    // Sort the annotations by (start, end col)
    // The labels are reversed, sort and then reversed again.
    // Consider a list of annotations (A1, A2, C1, C2, B1, B2) where
    // the letter signifies the span. Here we are only sorting by the
    // span and hence, the order of the elements with the same span will
    // not change. On reversing the ordering (|a, b| but b.cmp(a)), you get
    // (C1, C2, B1, B2, A1, A2). All the elements with the same span are
    // still ordered first to last, but all the elements with different
    // spans are ordered by their spans in last to first order. Last to
    // first order is important, because the jiggly lines and | are on
    // the left, so the rightmost span needs to be rendered first,
    // otherwise the lines would end up needing to go over a message.

    let mut annotations = line_info.annotations.clone();
    annotations.sort_by_key(|a| Reverse((a.start.display, a.start.char)));

    // First, figure out where each label will be positioned.
    //
    // In the case where you have the following annotations:
    //
    //      vec.push(vec.pop().unwrap());
    //      --------                    - previous borrow ends here [C]
    //      ||
    //      |this makes no sense [B]
    //      previous borrow of `vec` occurs here [A]
    //
    // `annotations_position` will hold [(2, A), (1, B), (0, C)].
    //
    // We try, when possible, to stick the rightmost annotation at the end
    // of the highlight line:
    //
    //      vec.push(vec.pop().unwrap());
    //      ---      ---               - previous borrow ends here
    //
    // But sometimes that's not possible because one of the other
    // annotations overlaps it. For example, from the test
    // `span_overlap_label`, we have the following annotations
    // (written on distinct lines for clarity):
    //
    //      fn foo(x: u32) {
    //      --------------
    //             -
    //
    // In this case, we can't stick the rightmost-most label on
    // the highlight line, or we would get:
    //
    //      fn foo(x: u32) {
    //      -------- x_span
    //      |
    //      fn_span
    //
    // which is totally weird. Instead we want:
    //
    //      fn foo(x: u32) {
    //      --------------
    //      |      |
    //      |      x_span
    //      fn_span
    //
    // which is...less weird, at least. In fact, in general, if
    // the rightmost span overlaps with any other span, we should
    // use the "hang below" version, so we can at least make it
    // clear where the span *starts*. There's an exception for this
    // logic, when the labels do not have a message:
    //
    //      fn foo(x: u32) {
    //      --------------
    //             |
    //             x_span
    //
    // instead of:
    //
    //      fn foo(x: u32) {
    //      --------------
    //      |      |
    //      |      x_span
    //      <EMPTY LINE>
    //
    let mut overlap = vec![false; annotations.len()];
    let mut annotations_position = vec![];
    let mut line_len: usize = 0;
    let mut p = 0;
    for (i, annotation) in annotations.iter().enumerate() {
        for (j, next) in annotations.iter().enumerate() {
            if overlaps(next, annotation, 0) && j > 1 {
                overlap[i] = true;
                overlap[j] = true;
            }
            if overlaps(next, annotation, 0)  // This label overlaps with another one and both
                    && annotation.has_label()     // take space (they have text and are not
                    && j > i                      // multiline lines).
                    && p == 0
            // We're currently on the first line, move the label one line down
            {
                // If we're overlapping with an un-labelled annotation with the same span
                // we can just merge them in the output
                if next.start.display == annotation.start.display
                    && next.start.char == annotation.start.char
                    && next.end.display == annotation.end.display
                    && next.end.char == annotation.end.char
                    && !next.has_label()
                {
                    continue;
                }

                // This annotation needs a new line in the output.
                p += 1;
                break;
            }
        }
        annotations_position.push((p, annotation));
        for (j, next) in annotations.iter().enumerate() {
            if j > i {
                let l = next.label.as_ref().map_or(0, |label| label.len() + 2);
                if (overlaps(next, annotation, l) // Do not allow two labels to be in the same
                        // line if they overlap including padding, to
                        // avoid situations like:
                        //
                        //      fn foo(x: u32) {
                        //      -------^------
                        //      |      |
                        //      fn_spanx_span
                        //
                        && annotation.has_label()    // Both labels must have some text, otherwise
                        && next.has_label())         // they are not overlapping.
                        // Do not add a new line if this annotation
                        // or the next are vertical line placeholders.
                        || (annotation.takes_space() // If either this or the next annotation is
                        && next.has_label())     // multiline start/end, move it to a new line
                        || (annotation.has_label()   // so as not to overlap the horizontal lines.
                        && next.takes_space())
                        || (annotation.takes_space() && next.takes_space())
                        || (overlaps(next, annotation, l)
                        && (next.end.display, next.end.char) <= (annotation.end.display, annotation.end.char)
                        && next.has_label()
                        && p == 0)
                // Avoid #42595.
                {
                    // This annotation needs a new line in the output.
                    p += 1;
                    break;
                }
            }
        }
        line_len = max(line_len, p);
    }

    if line_len != 0 {
        line_len += 1;
    }

    // If there are no annotations or the only annotations on this line are
    // MultilineLine, then there's only code being shown, stop processing.
    if line_info.annotations.iter().all(LineAnnotation::is_line) {
        return vec![];
    }

    if annotations_position
        .iter()
        .all(|(_, ann)| matches!(ann.annotation_type, LineAnnotationType::MultilineStart(_)))
    {
        if let Some(max_pos) = annotations_position.iter().map(|(pos, _)| *pos).max() {
            // Special case the following, so that we minimize overlapping multiline spans.
            //
            // 3 │       X0 Y0 Z0
            //   │ ┏━━━━━┛  │  │     < We are writing these lines
            //   │ ┃┌───────┘  │     < by reverting the "depth" of
            //   │ ┃│┌─────────┘     < their multiline spans.
            // 4 │ ┃││   X1 Y1 Z1
            // 5 │ ┃││   X2 Y2 Z2
            //   │ ┃│└────╿──│──┘ `Z` label
            //   │ ┃└─────│──┤
            //   │ ┗━━━━━━┥  `Y` is a good letter too
            //   ╰╴       `X` is a good letter
            for (pos, _) in &mut annotations_position {
                *pos = max_pos - *pos;
            }
            // We know then that we don't need an additional line for the span label, saving us
            // one line of vertical space.
            line_len = line_len.saturating_sub(1);
        }
    }

    // Write the column separator.
    //
    // After this we will have:
    //
    // 2 |   fn foo() {
    //   |
    //   |
    //   |
    // 3 |
    // 4 |   }
    //   |
    for pos in 0..=line_len {
        draw_col_separator_no_space(renderer, buffer, line_offset + pos + 1, width_offset - 2);
    }
    if close_window {
        draw_col_separator_end(
            renderer,
            buffer,
            line_offset + line_len + 1,
            width_offset - 2,
        );
    }
    // Write the horizontal lines for multiline annotations
    // (only the first and last lines need this).
    //
    // After this we will have:
    //
    // 2 |   fn foo() {
    //   |  __________
    //   |
    //   |
    // 3 |
    // 4 |   }
    //   |  _
    for &(pos, annotation) in &annotations_position {
        let underline = renderer.decor_style.underline(annotation.is_primary());
        let pos = pos + 1;
        match annotation.annotation_type {
            LineAnnotationType::MultilineStart(depth) | LineAnnotationType::MultilineEnd(depth) => {
                draw_range(
                    buffer,
                    underline.multiline_horizontal,
                    line_offset + pos,
                    width_offset + depth,
                    (code_offset + annotation.start.display).saturating_sub(left),
                    underline.style,
                );
            }
            _ if annotation.highlight_source => {
                buffer.set_style_range(
                    line_offset,
                    (code_offset + annotation.start.display).saturating_sub(left),
                    (code_offset + annotation.end.display).saturating_sub(left),
                    underline.style,
                    annotation.is_primary(),
                );
            }
            _ => {}
        }
    }

    // Write the vertical lines for labels that are on a different line as the underline.
    //
    // After this we will have:
    //
    // 2 |   fn foo() {
    //   |  __________
    //   | |    |
    //   | |
    // 3 | |
    // 4 | | }
    //   | |_
    for &(pos, annotation) in &annotations_position {
        let underline = renderer.decor_style.underline(annotation.is_primary());
        let pos = pos + 1;

        if pos > 1 && (annotation.has_label() || annotation.takes_space()) {
            for p in line_offset + 1..=line_offset + pos {
                buffer.putc(
                    p,
                    (code_offset + annotation.start.display).saturating_sub(left),
                    match annotation.annotation_type {
                        LineAnnotationType::MultilineLine(_) => underline.multiline_vertical,
                        _ => underline.vertical_text_line,
                    },
                    underline.style,
                );
            }
            if let LineAnnotationType::MultilineStart(_) = annotation.annotation_type {
                buffer.putc(
                    line_offset + pos,
                    (code_offset + annotation.start.display).saturating_sub(left),
                    underline.bottom_right,
                    underline.style,
                );
            }
            if matches!(
                annotation.annotation_type,
                LineAnnotationType::MultilineEnd(_)
            ) && annotation.has_label()
            {
                buffer.putc(
                    line_offset + pos,
                    (code_offset + annotation.start.display).saturating_sub(left),
                    underline.multiline_bottom_right_with_text,
                    underline.style,
                );
            }
        }
        match annotation.annotation_type {
            LineAnnotationType::MultilineStart(depth) => {
                buffer.putc(
                    line_offset + pos,
                    width_offset + depth - 1,
                    underline.top_left,
                    underline.style,
                );
                for p in line_offset + pos + 1..line_offset + line_len + 2 {
                    buffer.putc(
                        p,
                        width_offset + depth - 1,
                        underline.multiline_vertical,
                        underline.style,
                    );
                }
            }
            LineAnnotationType::MultilineEnd(depth) => {
                for p in line_offset..line_offset + pos {
                    buffer.putc(
                        p,
                        width_offset + depth - 1,
                        underline.multiline_vertical,
                        underline.style,
                    );
                }
                buffer.putc(
                    line_offset + pos,
                    width_offset + depth - 1,
                    underline.bottom_left,
                    underline.style,
                );
            }
            _ => (),
        }
    }

    // Write the labels on the annotations that actually have a label.
    //
    // After this we will have:
    //
    // 2 |   fn foo() {
    //   |  __________
    //   |      |
    //   |      something about `foo`
    // 3 |
    // 4 |   }
    //   |  _  test
    for &(pos, annotation) in &annotations_position {
        let style = if annotation.is_primary() {
            ElementStyle::LabelPrimary
        } else {
            ElementStyle::LabelSecondary
        };
        let (pos, col) = if pos == 0 {
            if annotation.end.display == 0 {
                (pos + 1, (annotation.end.display + 2).saturating_sub(left))
            } else {
                (pos + 1, (annotation.end.display + 1).saturating_sub(left))
            }
        } else {
            (pos + 2, annotation.start.display.saturating_sub(left))
        };
        if let Some(label) = &annotation.label {
            buffer.puts(line_offset + pos, code_offset + col, label, style);
        }
    }

    // Sort from biggest span to smallest span so that smaller spans are
    // represented in the output:
    //
    // x | fn foo()
    //   | ^^^---^^
    //   | |  |
    //   | |  something about `foo`
    //   | something about `fn foo()`
    annotations_position.sort_by_key(|(_, ann)| {
        // Decreasing order. When annotations share the same length, prefer `Primary`.
        (Reverse(ann.len()), ann.is_primary())
    });

    // Write the underlines.
    //
    // After this we will have:
    //
    // 2 |   fn foo() {
    //   |  ____-_____^
    //   |      |
    //   |      something about `foo`
    // 3 |
    // 4 |   }
    //   |  _^  test
    for &(pos, annotation) in &annotations_position {
        let uline = renderer.decor_style.underline(annotation.is_primary());
        for p in annotation.start.display..annotation.end.display {
            // The default span label underline.
            buffer.putc(
                line_offset + 1,
                (code_offset + p).saturating_sub(left),
                uline.underline,
                uline.style,
            );
        }

        if pos == 0
            && matches!(
                annotation.annotation_type,
                LineAnnotationType::MultilineStart(_) | LineAnnotationType::MultilineEnd(_)
            )
        {
            // The beginning of a multiline span with its leftward moving line on the same line.
            buffer.putc(
                line_offset + 1,
                (code_offset + annotation.start.display).saturating_sub(left),
                match annotation.annotation_type {
                    LineAnnotationType::MultilineStart(_) => uline.top_right_flat,
                    LineAnnotationType::MultilineEnd(_) => uline.multiline_end_same_line,
                    _ => panic!("unexpected annotation type: {annotation:?}"),
                },
                uline.style,
            );
        } else if pos != 0
            && matches!(
                annotation.annotation_type,
                LineAnnotationType::MultilineStart(_) | LineAnnotationType::MultilineEnd(_)
            )
        {
            // The beginning of a multiline span with its leftward moving line on another line,
            // so we start going down first.
            buffer.putc(
                line_offset + 1,
                (code_offset + annotation.start.display).saturating_sub(left),
                match annotation.annotation_type {
                    LineAnnotationType::MultilineStart(_) => uline.multiline_start_down,
                    LineAnnotationType::MultilineEnd(_) => uline.multiline_end_up,
                    _ => panic!("unexpected annotation type: {annotation:?}"),
                },
                uline.style,
            );
        } else if pos != 0 && annotation.has_label() {
            // The beginning of a span label with an actual label, we'll point down.
            buffer.putc(
                line_offset + 1,
                (code_offset + annotation.start.display).saturating_sub(left),
                uline.label_start,
                uline.style,
            );
        }
    }

    // We look for individual *long* spans, and we trim the *middle*, so that we render
    // LL | ...= [0, 0, 0, ..., 0, 0];
    //    |      ^^^^^^^^^^...^^^^^^^ expected `&[u8]`, found `[{integer}; 1680]`
    for (i, (_pos, annotation)) in annotations_position.iter().enumerate() {
        // Skip cases where multiple spans overlap eachother.
        if overlap[i] {
            continue;
        };
        let LineAnnotationType::Singleline = annotation.annotation_type else {
            continue;
        };
        let width = annotation.end.display - annotation.start.display;
        if width > margin.term_width * 2 && width > 10 {
            // If the terminal is *too* small, we keep at least a tiny bit of the span for
            // display.
            let pad = max(margin.term_width / 3, 5);
            // Code line
            buffer.replace(
                line_offset,
                annotation.start.display + pad,
                annotation.end.display - pad,
                renderer.decor_style.margin(),
            );
            // Underline line
            buffer.replace(
                line_offset + 1,
                annotation.start.display + pad,
                annotation.end.display - pad,
                renderer.decor_style.margin(),
            );
        }
    }
    annotations_position
        .iter()
        .filter_map(|&(_, annotation)| match annotation.annotation_type {
            LineAnnotationType::MultilineStart(p) | LineAnnotationType::MultilineEnd(p) => {
                let style = if annotation.is_primary() {
                    ElementStyle::LabelPrimary
                } else {
                    ElementStyle::LabelSecondary
                };
                Some((p, style))
            }
            _ => None,
        })
        .collect::<Vec<_>>()
}

#[allow(clippy::too_many_arguments)]
fn emit_suggestion_default(
    renderer: &Renderer,
    buffer: &mut StyledBuffer,
    suggestion: &Snippet<'_, Patch<'_>>,
    spliced_lines: SplicedLines<'_>,
    show_code_change: DisplaySuggestion,
    max_line_num_len: usize,
    sm: &SourceMap<'_>,
    primary_path: Option<&Cow<'_, str>>,
    matches_previous_suggestion: bool,
    is_first: bool,
    is_cont: bool,
) {
    let buffer_offset = buffer.num_lines();
    let mut row_num = buffer_offset + usize::from(!matches_previous_suggestion);
    let (complete, parts, highlights) = spliced_lines;
    let is_multiline = complete.lines().count() > 1;

    if matches_previous_suggestion {
        buffer.puts(
            row_num - 1,
            max_line_num_len + 1,
            renderer.decor_style.multi_suggestion_separator(),
            ElementStyle::LineNumber,
        );
    } else {
        draw_col_separator_start(renderer, buffer, row_num - 1, max_line_num_len + 1);
    }
    if suggestion.path.as_ref() != primary_path {
        if let Some(path) = suggestion.path.as_ref() {
            if !matches_previous_suggestion {
                let (loc, _) = sm.span_to_locations(parts[0].span.clone());
                // --> file.rs:line:col
                //  |
                let arrow = renderer.decor_style.file_start(is_first, false);
                buffer.puts(row_num - 1, 0, arrow, ElementStyle::LineNumber);
                let message = format!("{}:{}:{}", path, loc.line, loc.char + 1);
                let col = usize::max(max_line_num_len + 1, arrow.len());
                buffer.puts(row_num - 1, col, &message, ElementStyle::LineAndColumn);
                for _ in 0..max_line_num_len {
                    buffer.prepend(row_num - 1, " ", ElementStyle::NoStyle);
                }
                draw_col_separator_no_space(renderer, buffer, row_num, max_line_num_len + 1);
                row_num += 1;
            }
        }
    }

    if let DisplaySuggestion::Diff = show_code_change {
        row_num += 1;
    }

    let lo = parts.iter().map(|p| p.span.start).min().unwrap();
    let hi = parts.iter().map(|p| p.span.end).max().unwrap();

    let file_lines = sm.span_to_lines(lo..hi);
    let (line_start, line_end) = if suggestion.fold {
        // We use the original span to get original line_start
        sm.span_to_locations(parts[0].original_span.clone())
    } else {
        sm.span_to_locations(0..sm.source.len())
    };
    let mut lines = complete.lines();
    if lines.clone().next().is_none() {
        // Account for a suggestion to completely remove a line(s) with whitespace (#94192).
        for line in line_start.line..=line_end.line {
            buffer.puts(
                row_num - 1 + line - line_start.line,
                0,
                &maybe_anonymized(renderer, line, max_line_num_len),
                ElementStyle::LineNumber,
            );
            buffer.puts(
                row_num - 1 + line - line_start.line,
                max_line_num_len + 1,
                "- ",
                ElementStyle::Removal,
            );
            buffer.puts(
                row_num - 1 + line - line_start.line,
                max_line_num_len + 3,
                &normalize_whitespace(sm.get_line(line).unwrap()),
                ElementStyle::Removal,
            );
        }
        row_num += line_end.line - line_start.line;
    }
    let mut unhighlighted_lines = Vec::new();
    for (line_pos, (line, highlight_parts)) in lines.by_ref().zip(highlights).enumerate() {
        // Remember lines that are not highlighted to hide them if needed
        if highlight_parts.is_empty() && suggestion.fold {
            unhighlighted_lines.push((line_pos, line));
            continue;
        }

        match unhighlighted_lines.len() {
            0 => (),
            // Since we show first line, "..." line and last line,
            // There is no reason to hide if there are 3 or less lines
            // (because then we just replace a line with ... which is
            // not helpful)
            n if n <= 3 => unhighlighted_lines.drain(..).for_each(|(p, l)| {
                draw_code_line(
                    renderer,
                    buffer,
                    &mut row_num,
                    &[],
                    p + line_start.line,
                    l,
                    show_code_change,
                    max_line_num_len,
                    &file_lines,
                    is_multiline,
                );
            }),
            // Print first unhighlighted line, "..." and last unhighlighted line, like so:
            //
            // LL | this line was highlighted
            // LL | this line is just for context
            // ...
            // LL | this line is just for context
            // LL | this line was highlighted
            _ => {
                let last_line = unhighlighted_lines.pop();
                let first_line = unhighlighted_lines.drain(..).next();

                if let Some((p, l)) = first_line {
                    draw_code_line(
                        renderer,
                        buffer,
                        &mut row_num,
                        &[],
                        p + line_start.line,
                        l,
                        show_code_change,
                        max_line_num_len,
                        &file_lines,
                        is_multiline,
                    );
                }

                let placeholder = renderer.decor_style.margin();
                let padding = str_width(placeholder);
                buffer.puts(
                    row_num,
                    max_line_num_len.saturating_sub(padding),
                    placeholder,
                    ElementStyle::LineNumber,
                );
                row_num += 1;

                if let Some((p, l)) = last_line {
                    draw_code_line(
                        renderer,
                        buffer,
                        &mut row_num,
                        &[],
                        p + line_start.line,
                        l,
                        show_code_change,
                        max_line_num_len,
                        &file_lines,
                        is_multiline,
                    );
                }
            }
        }
        draw_code_line(
            renderer,
            buffer,
            &mut row_num,
            &highlight_parts,
            line_pos + line_start.line,
            line,
            show_code_change,
            max_line_num_len,
            &file_lines,
            is_multiline,
        );
    }

    // This offset and the ones below need to be signed to account for replacement code
    // that is shorter than the original code.
    let mut offsets: Vec<(usize, isize)> = Vec::new();
    // Only show an underline in the suggestions if the suggestion is not the
    // entirety of the code being shown and the displayed code is not multiline.
    if let DisplaySuggestion::Diff | DisplaySuggestion::Underline | DisplaySuggestion::Add =
        show_code_change
    {
        let mut prev_lines: Option<(usize, usize)> = None;
        for part in parts {
            let snippet = sm.span_to_snippet(part.span.clone()).unwrap_or_default();
            let (span_start, span_end) = sm.span_to_locations(part.span.clone());
            let span_start_pos = span_start.display;
            let span_end_pos = span_end.display;

            // If this addition is _only_ whitespace, then don't trim it,
            // or else we're just not rendering anything.
            let is_whitespace_addition = part.replacement.trim().is_empty();

            // Do not underline the leading...
            let start = if is_whitespace_addition {
                0
            } else {
                part.replacement
                    .len()
                    .saturating_sub(part.replacement.trim_start().len())
            };
            // ...or trailing spaces. Account for substitutions containing unicode
            // characters.
            let sub_len: usize = str_width(if is_whitespace_addition {
                &part.replacement
            } else {
                part.replacement.trim()
            });

            let offset: isize = offsets
                .iter()
                .filter_map(|(start, v)| {
                    if span_start_pos < *start {
                        None
                    } else {
                        Some(v)
                    }
                })
                .sum();
            let underline_start = (span_start_pos + start) as isize + offset;
            let underline_end = (span_start_pos + start + sub_len) as isize + offset;
            assert!(underline_start >= 0 && underline_end >= 0);
            let padding: usize = max_line_num_len + 3;
            for p in underline_start..underline_end {
                if matches!(show_code_change, DisplaySuggestion::Underline) {
                    // If this is a replacement, underline with `~`, if this is an addition
                    // underline with `+`.
                    buffer.putc(
                        row_num,
                        (padding as isize + p) as usize,
                        if part.is_addition(sm) {
                            '+'
                        } else {
                            renderer.decor_style.diff()
                        },
                        ElementStyle::Addition,
                    );
                }
            }
            if let DisplaySuggestion::Diff = show_code_change {
                // Colorize removal with red in diff format.

                // Below, there's some tricky buffer indexing going on. `row_num` at this
                // point corresponds to:
                //
                //    |
                // LL | CODE
                //    | ++++  <- `row_num`
                //
                // in the buffer. When we have a diff format output, we end up with
                //
                //    |
                // LL - OLDER   <- row_num - 2
                // LL + NEWER
                //    |         <- row_num
                //
                // The `row_num - 2` is to select the buffer line that has the "old version
                // of the diff" at that point. When the removal is a single line, `i` is
                // `0`, `newlines` is `1` so `(newlines - i - 1)` ends up being `0`, so row
                // points at `LL - OLDER`. When the removal corresponds to multiple lines,
                // we end up with `newlines > 1` and `i` being `0..newlines - 1`.
                //
                //    |
                // LL - OLDER   <- row_num - 2 - (newlines - last_i - 1)
                // LL - CODE
                // LL - BEING
                // LL - REMOVED <- row_num - 2 - (newlines - first_i - 1)
                // LL + NEWER
                //    |         <- row_num

                let newlines = snippet.lines().count();
                if newlines > 0 && row_num > newlines {
                    let offset = match prev_lines {
                        Some((start, end)) => {
                            file_lines.len().saturating_sub(end.saturating_sub(start))
                        }
                        None => file_lines.len(),
                    };
                    // Account for removals where the part being removed spans multiple
                    // lines.
                    // FIXME: We check the number of rows because in some cases, like in
                    // `tests/ui/lint/invalid-nan-comparison-suggestion.rs`, the rendered
                    // suggestion will only show the first line of code being replaced. The
                    // proper way of doing this would be to change the suggestion rendering
                    // logic to show the whole prior snippet, but the current output is not
                    // too bad to begin with, so we side-step that issue here.
                    for (i, line) in snippet.lines().enumerate() {
                        let tabs: usize = line
                            .chars()
                            .take(span_start.char)
                            .map(|ch| match ch {
                                '\t' => 3,
                                _ => 0,
                            })
                            .sum();
                        let line = normalize_whitespace(line);
                        // Going lower than buffer_offset (+ 1) would mean
                        // overwriting existing content in the buffer
                        let min_row = buffer_offset + usize::from(!matches_previous_suggestion);
                        let row = (row_num - 2 - (offset - i - 1)).max(min_row);
                        // On the first line, we highlight between the start of the part
                        // span, and the end of that line.
                        // On the last line, we highlight between the start of the line, and
                        // the column of the part span end.
                        // On all others, we highlight the whole line.
                        let start = if i == 0 {
                            (padding as isize + (span_start.char + tabs) as isize) as usize
                        } else {
                            padding
                        };
                        let end = if i == 0 {
                            (padding as isize
                                + (span_start.char + tabs) as isize
                                + line.chars().count() as isize)
                                as usize
                        } else if i == newlines - 1 {
                            (padding as isize + (span_end.char + tabs) as isize) as usize
                        } else {
                            (padding as isize + line.chars().count() as isize) as usize
                        };
                        buffer.set_style_range(row, start, end, ElementStyle::Removal, true);
                    }
                } else {
                    let tabs: usize = snippet
                        .chars()
                        .take(span_start.char)
                        .map(|ch| match ch {
                            '\t' => 3,
                            _ => 0,
                        })
                        .sum();
                    // The removed code fits all in one line.
                    buffer.set_style_range(
                        row_num - 2,
                        (padding as isize + (span_start.char + tabs) as isize) as usize,
                        (padding as isize + (span_end.char + tabs) as isize) as usize,
                        ElementStyle::Removal,
                        true,
                    );
                }
                prev_lines = Some((span_start.line, span_end.line));
            }

            // length of the code after substitution
            let full_sub_len = str_width(&part.replacement) as isize;

            // length of the code to be substituted
            let snippet_len = span_end_pos as isize - span_start_pos as isize;
            // For multiple substitutions, use the position *after* the previous
            // substitutions have happened, only when further substitutions are
            // located strictly after.
            offsets.push((span_end_pos, full_sub_len - snippet_len));
        }
        row_num += 1;
    }

    // if we elided some lines, add an ellipsis
    if lines.next().is_some() {
        let placeholder = renderer.decor_style.margin();
        let padding = str_width(placeholder);
        buffer.puts(
            row_num,
            max_line_num_len.saturating_sub(padding),
            placeholder,
            ElementStyle::LineNumber,
        );
    } else {
        let row = match show_code_change {
            DisplaySuggestion::Diff | DisplaySuggestion::Add | DisplaySuggestion::Underline => {
                row_num - 1
            }
            DisplaySuggestion::None => row_num,
        };
        if is_cont {
            draw_col_separator_no_space(renderer, buffer, row, max_line_num_len + 1);
        } else {
            draw_col_separator_end(renderer, buffer, row, max_line_num_len + 1);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_code_line(
    renderer: &Renderer,
    buffer: &mut StyledBuffer,
    row_num: &mut usize,
    highlight_parts: &[SubstitutionHighlight],
    line_num: usize,
    line_to_add: &str,
    show_code_change: DisplaySuggestion,
    max_line_num_len: usize,
    file_lines: &[&LineInfo<'_>],
    is_multiline: bool,
) {
    if let DisplaySuggestion::Diff = show_code_change {
        // We need to print more than one line if the span we need to remove is multiline.
        // For more info: https://github.com/rust-lang/rust/issues/92741
        let lines_to_remove = file_lines.iter().take(file_lines.len() - 1);
        for (index, line_to_remove) in lines_to_remove.enumerate() {
            buffer.puts(
                *row_num - 1,
                0,
                &maybe_anonymized(renderer, line_num + index, max_line_num_len),
                ElementStyle::LineNumber,
            );
            buffer.puts(
                *row_num - 1,
                max_line_num_len + 1,
                "- ",
                ElementStyle::Removal,
            );
            let line = normalize_whitespace(line_to_remove.line);
            buffer.puts(
                *row_num - 1,
                max_line_num_len + 3,
                &line,
                ElementStyle::NoStyle,
            );
            *row_num += 1;
        }
        // If the last line is exactly equal to the line we need to add, we can skip both of
        // them. This allows us to avoid output like the following:
        // 2 - &
        // 2 + if true { true } else { false }
        // 3 - if true { true } else { false }
        // If those lines aren't equal, we print their diff
        let last_line = &file_lines.last().unwrap();
        if last_line.line == line_to_add {
            *row_num -= 2;
        } else {
            buffer.puts(
                *row_num - 1,
                0,
                &maybe_anonymized(renderer, line_num + file_lines.len() - 1, max_line_num_len),
                ElementStyle::LineNumber,
            );
            buffer.puts(
                *row_num - 1,
                max_line_num_len + 1,
                "- ",
                ElementStyle::Removal,
            );
            buffer.puts(
                *row_num - 1,
                max_line_num_len + 3,
                &normalize_whitespace(last_line.line),
                ElementStyle::NoStyle,
            );
            if line_to_add.trim().is_empty() {
                *row_num -= 1;
            } else {
                // Check if after the removal, the line is left with only whitespace. If so, we
                // will not show an "addition" line, as removing the whole line is what the user
                // would really want.
                // For example, for the following:
                //   |
                // 2 -     .await
                // 2 +     (note the left over whitespace)
                //   |
                // We really want
                //   |
                // 2 -     .await
                //   |
                // *row_num -= 1;
                buffer.puts(
                    *row_num,
                    0,
                    &maybe_anonymized(renderer, line_num, max_line_num_len),
                    ElementStyle::LineNumber,
                );
                buffer.puts(*row_num, max_line_num_len + 1, "+ ", ElementStyle::Addition);
                buffer.append(
                    *row_num,
                    &normalize_whitespace(line_to_add),
                    ElementStyle::NoStyle,
                );
            }
        }
    } else if is_multiline {
        buffer.puts(
            *row_num,
            0,
            &maybe_anonymized(renderer, line_num, max_line_num_len),
            ElementStyle::LineNumber,
        );
        match &highlight_parts {
            [SubstitutionHighlight { start: 0, end }] if *end == line_to_add.len() => {
                buffer.puts(*row_num, max_line_num_len + 1, "+ ", ElementStyle::Addition);
            }
            [] | [SubstitutionHighlight { start: 0, end: 0 }] => {
                // FIXME: needed? Doesn't get exercised in any test.
                draw_col_separator_no_space(renderer, buffer, *row_num, max_line_num_len + 1);
            }
            _ => {
                let diff = renderer.decor_style.diff();
                buffer.puts(
                    *row_num,
                    max_line_num_len + 1,
                    &format!("{diff} "),
                    ElementStyle::Addition,
                );
            }
        }
        //   LL | line_to_add
        //   ++^^^
        //    |  |
        //    |  magic `3`
        //    `max_line_num_len`
        buffer.puts(
            *row_num,
            max_line_num_len + 3,
            &normalize_whitespace(line_to_add),
            ElementStyle::NoStyle,
        );
    } else if let DisplaySuggestion::Add = show_code_change {
        buffer.puts(
            *row_num,
            0,
            &maybe_anonymized(renderer, line_num, max_line_num_len),
            ElementStyle::LineNumber,
        );
        buffer.puts(*row_num, max_line_num_len + 1, "+ ", ElementStyle::Addition);
        buffer.append(
            *row_num,
            &normalize_whitespace(line_to_add),
            ElementStyle::NoStyle,
        );
    } else {
        buffer.puts(
            *row_num,
            0,
            &maybe_anonymized(renderer, line_num, max_line_num_len),
            ElementStyle::LineNumber,
        );
        draw_col_separator(renderer, buffer, *row_num, max_line_num_len + 1);
        buffer.append(
            *row_num,
            &normalize_whitespace(line_to_add),
            ElementStyle::NoStyle,
        );
    }

    // Colorize addition/replacements with green.
    for &SubstitutionHighlight { start, end } in highlight_parts {
        // This is a no-op for empty ranges
        if start != end {
            // Account for tabs when highlighting (#87972).
            let tabs: usize = line_to_add
                .chars()
                .take(start)
                .map(|ch| match ch {
                    '\t' => 3,
                    _ => 0,
                })
                .sum();
            buffer.set_style_range(
                *row_num,
                max_line_num_len + 3 + start + tabs,
                max_line_num_len + 3 + end + tabs,
                ElementStyle::Addition,
                true,
            );
        }
    }
    *row_num += 1;
}

#[allow(clippy::too_many_arguments)]
fn draw_line(
    renderer: &Renderer,
    buffer: &mut StyledBuffer,
    source_string: &str,
    line_index: usize,
    line_offset: usize,
    width_offset: usize,
    code_offset: usize,
    max_line_num_len: usize,
    margin: Margin,
) -> usize {
    // Tabs are assumed to have been replaced by spaces in calling code.
    debug_assert!(!source_string.contains('\t'));
    let line_len = str_width(source_string);
    // Create the source line we will highlight.
    let mut left = margin.left(line_len);
    let right = margin.right(line_len);

    let mut taken = 0;
    let mut skipped = 0;
    let code: String = source_string
        .chars()
        .skip_while(|ch| {
            let w = char_width(*ch);
            // If `skipped` is less than `left`, always skip the next `ch`,
            // even if `ch` is a multi-width char that would make `skipped`
            // exceed `left`. This ensures that we do not exceed term width on
            // source lines.
            if skipped < left {
                skipped += w;
                true
            } else {
                false
            }
        })
        .take_while(|ch| {
            // Make sure that the trimming on the right will fall within the terminal width.
            taken += char_width(*ch);
            taken <= (right - left)
        })
        .collect();
    // If we skipped more than `left`, adjust `left` to account for it.
    if skipped > left {
        left += skipped - left;
    }
    let placeholder = renderer.decor_style.margin();
    let padding = str_width(placeholder);
    let (width_taken, bytes_taken) = if margin.was_cut_left() {
        // We have stripped some code/whitespace from the beginning, make it clear.
        let mut bytes_taken = 0;
        let mut width_taken = 0;
        for ch in code.chars() {
            width_taken += char_width(ch);
            bytes_taken += ch.len_utf8();

            if width_taken >= padding {
                break;
            }
        }

        buffer.puts(
            line_offset,
            code_offset,
            placeholder,
            ElementStyle::LineNumber,
        );
        (width_taken, bytes_taken)
    } else {
        (0, 0)
    };

    buffer.puts(
        line_offset,
        code_offset + width_taken,
        &code[bytes_taken..],
        ElementStyle::Quotation,
    );

    if line_len > right {
        // We have stripped some code/whitespace from the beginning, make it clear.
        let mut char_taken = 0;
        let mut width_taken_inner = 0;
        for ch in code.chars().rev() {
            width_taken_inner += char_width(ch);
            char_taken += 1;

            if width_taken_inner >= padding {
                break;
            }
        }

        buffer.puts(
            line_offset,
            code_offset + width_taken + code[bytes_taken..].chars().count() - char_taken,
            placeholder,
            ElementStyle::LineNumber,
        );
    }

    buffer.puts(
        line_offset,
        0,
        &maybe_anonymized(renderer, line_index, max_line_num_len),
        ElementStyle::LineNumber,
    );

    draw_col_separator_no_space(renderer, buffer, line_offset, width_offset - 2);

    left
}

fn draw_range(
    buffer: &mut StyledBuffer,
    symbol: char,
    line: usize,
    col_from: usize,
    col_to: usize,
    style: ElementStyle,
) {
    for col in col_from..col_to {
        buffer.putc(line, col, symbol, style);
    }
}

fn draw_multiline_line(
    renderer: &Renderer,
    buffer: &mut StyledBuffer,
    line: usize,
    offset: usize,
    depth: usize,
    style: ElementStyle,
) {
    let chr = match (style, renderer.decor_style) {
        (ElementStyle::UnderlinePrimary | ElementStyle::LabelPrimary, DecorStyle::Ascii) => '|',
        (_, DecorStyle::Ascii) => '|',
        (ElementStyle::UnderlinePrimary | ElementStyle::LabelPrimary, DecorStyle::Unicode) => '┃',
        (_, DecorStyle::Unicode) => '│',
    };
    buffer.putc(line, offset + depth - 1, chr, style);
}

fn draw_col_separator(renderer: &Renderer, buffer: &mut StyledBuffer, line: usize, col: usize) {
    let chr = renderer.decor_style.col_separator();
    buffer.puts(line, col, &format!("{chr} "), ElementStyle::LineNumber);
}

fn draw_col_separator_no_space(
    renderer: &Renderer,
    buffer: &mut StyledBuffer,
    line: usize,
    col: usize,
) {
    let chr = renderer.decor_style.col_separator();
    draw_col_separator_no_space_with_style(buffer, chr, line, col, ElementStyle::LineNumber);
}

fn draw_col_separator_start(
    renderer: &Renderer,
    buffer: &mut StyledBuffer,
    line: usize,
    col: usize,
) {
    match renderer.decor_style {
        DecorStyle::Ascii => {
            draw_col_separator_no_space_with_style(
                buffer,
                '|',
                line,
                col,
                ElementStyle::LineNumber,
            );
        }
        DecorStyle::Unicode => {
            draw_col_separator_no_space_with_style(
                buffer,
                '╭',
                line,
                col,
                ElementStyle::LineNumber,
            );
            draw_col_separator_no_space_with_style(
                buffer,
                '╴',
                line,
                col + 1,
                ElementStyle::LineNumber,
            );
        }
    }
}

fn draw_col_separator_end(renderer: &Renderer, buffer: &mut StyledBuffer, line: usize, col: usize) {
    match renderer.decor_style {
        DecorStyle::Ascii => {
            draw_col_separator_no_space_with_style(
                buffer,
                '|',
                line,
                col,
                ElementStyle::LineNumber,
            );
        }
        DecorStyle::Unicode => {
            draw_col_separator_no_space_with_style(
                buffer,
                '╰',
                line,
                col,
                ElementStyle::LineNumber,
            );
            draw_col_separator_no_space_with_style(
                buffer,
                '╴',
                line,
                col + 1,
                ElementStyle::LineNumber,
            );
        }
    }
}

fn draw_col_separator_no_space_with_style(
    buffer: &mut StyledBuffer,
    chr: char,
    line: usize,
    col: usize,
    style: ElementStyle,
) {
    buffer.putc(line, col, chr, style);
}

fn maybe_anonymized(renderer: &Renderer, line_num: usize, max_line_num_len: usize) -> String {
    format!(
        "{:>max_line_num_len$}",
        if renderer.anonymized_line_numbers {
            Cow::Borrowed(ANONYMIZED_LINE_NUM)
        } else {
            Cow::Owned(line_num.to_string())
        }
    )
}

fn draw_note_separator(
    renderer: &Renderer,
    buffer: &mut StyledBuffer,
    line: usize,
    col: usize,
    is_cont: bool,
) {
    let chr = renderer.decor_style.note_separator(is_cont);
    buffer.puts(line, col, chr, ElementStyle::LineNumber);
}

fn draw_line_separator(renderer: &Renderer, buffer: &mut StyledBuffer, line: usize, col: usize) {
    let (column, dots) = match renderer.decor_style {
        DecorStyle::Ascii => (0, "..."),
        DecorStyle::Unicode => (col - 2, "‡"),
    };
    buffer.puts(line, column, dots, ElementStyle::LineNumber);
}

trait MessageOrTitle {
    fn level(&self) -> &Level<'_>;
    fn id(&self) -> Option<&Id<'_>>;
    fn text(&self) -> &str;
    fn allows_styling(&self) -> bool;
}

impl MessageOrTitle for Title<'_> {
    fn level(&self) -> &Level<'_> {
        &self.level
    }
    fn id(&self) -> Option<&Id<'_>> {
        self.id.as_ref()
    }
    fn text(&self) -> &str {
        self.text.as_ref()
    }
    fn allows_styling(&self) -> bool {
        self.allows_styling
    }
}

impl MessageOrTitle for Message<'_> {
    fn level(&self) -> &Level<'_> {
        &self.level
    }
    fn id(&self) -> Option<&Id<'_>> {
        None
    }
    fn text(&self) -> &str {
        self.text.as_ref()
    }
    fn allows_styling(&self) -> bool {
        true
    }
}

// instead of taking the String length or dividing by 10 while > 0, we multiply a limit by 10 until
// we're higher. If the loop isn't exited by the `return`, the last multiplication will wrap, which
// is OK, because while we cannot fit a higher power of 10 in a usize, the loop will end anyway.
// This is also why we need the max number of decimal digits within a `usize`.
fn num_decimal_digits(num: usize) -> usize {
    #[cfg(target_pointer_width = "64")]
    const MAX_DIGITS: usize = 20;

    #[cfg(target_pointer_width = "32")]
    const MAX_DIGITS: usize = 10;

    #[cfg(target_pointer_width = "16")]
    const MAX_DIGITS: usize = 5;

    let mut lim = 10;
    for num_digits in 1..MAX_DIGITS {
        if num < lim {
            return num_digits;
        }
        lim = lim.wrapping_mul(10);
    }
    MAX_DIGITS
}

fn str_width(s: &str) -> usize {
    s.chars().map(char_width).sum()
}

pub(crate) fn char_width(ch: char) -> usize {
    // FIXME: `unicode_width` sometimes disagrees with terminals on how wide a `char` is. For now,
    // just accept that sometimes the code line will be longer than desired.
    match ch {
        '\t' => 4,
        // Keep the following list in sync with `rustc_errors::emitter::OUTPUT_REPLACEMENTS`. These
        // are control points that we replace before printing with a visible codepoint for the sake
        // of being able to point at them with underlines.
        '\u{0000}' | '\u{0001}' | '\u{0002}' | '\u{0003}' | '\u{0004}' | '\u{0005}'
        | '\u{0006}' | '\u{0007}' | '\u{0008}' | '\u{000B}' | '\u{000C}' | '\u{000D}'
        | '\u{000E}' | '\u{000F}' | '\u{0010}' | '\u{0011}' | '\u{0012}' | '\u{0013}'
        | '\u{0014}' | '\u{0015}' | '\u{0016}' | '\u{0017}' | '\u{0018}' | '\u{0019}'
        | '\u{001A}' | '\u{001B}' | '\u{001C}' | '\u{001D}' | '\u{001E}' | '\u{001F}'
        | '\u{007F}' | '\u{202A}' | '\u{202B}' | '\u{202D}' | '\u{202E}' | '\u{2066}'
        | '\u{2067}' | '\u{2068}' | '\u{202C}' | '\u{2069}' => 1,
        _ => unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1),
    }
}

pub(crate) fn num_overlap(
    a_start: usize,
    a_end: usize,
    b_start: usize,
    b_end: usize,
    inclusive: bool,
) -> bool {
    let extra = usize::from(inclusive);
    (b_start..b_end + extra).contains(&a_start) || (a_start..a_end + extra).contains(&b_start)
}

fn overlaps(a1: &LineAnnotation<'_>, a2: &LineAnnotation<'_>, padding: usize) -> bool {
    num_overlap(
        a1.start.display,
        a1.end.display + padding,
        a2.start.display,
        a2.end.display,
        false,
    )
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) enum LineAnnotationType {
    /// Annotation under a single line of code
    Singleline,

    // The Multiline type above is replaced with the following three in order
    // to reuse the current label drawing code.
    //
    // Each of these corresponds to one part of the following diagram:
    //
    //     x |   foo(1 + bar(x,
    //       |  _________^              < MultilineStart
    //     x | |             y),        < MultilineLine
    //       | |______________^ label   < MultilineEnd
    //     x |       z);
    /// Annotation marking the first character of a fully shown multiline span
    MultilineStart(usize),
    /// Annotation marking the last character of a fully shown multiline span
    MultilineEnd(usize),
    /// Line at the left enclosing the lines of a fully shown multiline span
    // Just a placeholder for the drawing algorithm, to know that it shouldn't skip the first 4
    // and last 2 lines of code. The actual line is drawn in `emit_message_default` and not in
    // `draw_multiline_line`.
    MultilineLine(usize),
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) struct LineAnnotation<'a> {
    /// Start column.
    /// Note that it is important that this field goes
    /// first, so that when we sort, we sort orderings by start
    /// column.
    pub start: Loc,

    /// End column within the line (exclusive)
    pub end: Loc,

    /// level
    pub kind: AnnotationKind,

    /// Optional label to display adjacent to the annotation.
    pub label: Option<Cow<'a, str>>,

    /// Is this a single line, multiline or multiline span minimized down to a
    /// smaller span.
    pub annotation_type: LineAnnotationType,

    /// Whether the source code should be highlighted
    pub highlight_source: bool,
}

impl LineAnnotation<'_> {
    pub(crate) fn is_primary(&self) -> bool {
        self.kind == AnnotationKind::Primary
    }

    /// Whether this annotation is a vertical line placeholder.
    pub(crate) fn is_line(&self) -> bool {
        matches!(self.annotation_type, LineAnnotationType::MultilineLine(_))
    }

    /// Length of this annotation as displayed in the stderr output
    pub(crate) fn len(&self) -> usize {
        // Account for usize underflows
        self.end.display.abs_diff(self.start.display)
    }

    pub(crate) fn has_label(&self) -> bool {
        if let Some(label) = &self.label {
            // Consider labels with no text as effectively not being there
            // to avoid weird output with unnecessary vertical lines, like:
            //
            //     X | fn foo(x: u32) {
            //       | -------^------
            //       | |      |
            //       | |
            //       |
            //
            // Note that this would be the complete output users would see.
            !label.is_empty()
        } else {
            false
        }
    }

    pub(crate) fn takes_space(&self) -> bool {
        // Multiline annotations always have to keep vertical space.
        matches!(
            self.annotation_type,
            LineAnnotationType::MultilineStart(_) | LineAnnotationType::MultilineEnd(_)
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum DisplaySuggestion {
    Underline,
    Diff,
    None,
    Add,
}

impl DisplaySuggestion {
    fn new(complete: &str, patches: &[TrimmedPatch<'_>], sm: &SourceMap<'_>) -> Self {
        let has_deletion = patches
            .iter()
            .any(|p| p.is_deletion(sm) || p.is_destructive_replacement(sm));
        let is_multiline = complete.lines().count() > 1;
        if has_deletion && !is_multiline {
            DisplaySuggestion::Diff
        } else if patches.len() == 1
            && patches.first().map_or(false, |p| {
                p.replacement.ends_with('\n') && p.replacement.trim() == complete.trim()
            })
        {
            // We are adding a line(s) of code before code that was already there.
            DisplaySuggestion::Add
        } else if (patches.len() != 1 || patches[0].replacement.trim() != complete.trim())
            && !is_multiline
        {
            DisplaySuggestion::Underline
        } else {
            DisplaySuggestion::None
        }
    }
}

// We replace some characters so the CLI output is always consistent and underlines aligned.
// Keep the following list in sync with `rustc_span::char_width`.
const OUTPUT_REPLACEMENTS: &[(char, &str)] = &[
    // In terminals without Unicode support the following will be garbled, but in *all* terminals
    // the underlying codepoint will be as well. We could gate this replacement behind a "unicode
    // support" gate.
    ('\0', "␀"),
    ('\u{0001}', "␁"),
    ('\u{0002}', "␂"),
    ('\u{0003}', "␃"),
    ('\u{0004}', "␄"),
    ('\u{0005}', "␅"),
    ('\u{0006}', "␆"),
    ('\u{0007}', "␇"),
    ('\u{0008}', "␈"),
    ('\t', "    "), // We do our own tab replacement
    ('\u{000b}', "␋"),
    ('\u{000c}', "␌"),
    ('\u{000d}', "␍"),
    ('\u{000e}', "␎"),
    ('\u{000f}', "␏"),
    ('\u{0010}', "␐"),
    ('\u{0011}', "␑"),
    ('\u{0012}', "␒"),
    ('\u{0013}', "␓"),
    ('\u{0014}', "␔"),
    ('\u{0015}', "␕"),
    ('\u{0016}', "␖"),
    ('\u{0017}', "␗"),
    ('\u{0018}', "␘"),
    ('\u{0019}', "␙"),
    ('\u{001a}', "␚"),
    ('\u{001b}', "␛"),
    ('\u{001c}', "␜"),
    ('\u{001d}', "␝"),
    ('\u{001e}', "␞"),
    ('\u{001f}', "␟"),
    ('\u{007f}', "␡"),
    ('\u{200d}', ""), // Replace ZWJ for consistent terminal output of grapheme clusters.
    ('\u{202a}', "�"), // The following unicode text flow control characters are inconsistently
    ('\u{202b}', "�"), // supported across CLIs and can cause confusion due to the bytes on disk
    ('\u{202c}', "�"), // not corresponding to the visible source code, so we replace them always.
    ('\u{202d}', "�"),
    ('\u{202e}', "�"),
    ('\u{2066}', "�"),
    ('\u{2067}', "�"),
    ('\u{2068}', "�"),
    ('\u{2069}', "�"),
];

pub(crate) fn normalize_whitespace(s: &str) -> String {
    // Scan the input string for a character in the ordered table above.
    // If it's present, replace it with its alternative string (it can be more than 1 char!).
    // Otherwise, retain the input char.
    s.chars().fold(String::with_capacity(s.len()), |mut s, c| {
        match OUTPUT_REPLACEMENTS.binary_search_by_key(&c, |(k, _)| *k) {
            Ok(i) => s.push_str(OUTPUT_REPLACEMENTS[i].1),
            _ => s.push(c),
        }
        s
    })
}

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) enum ElementStyle {
    MainHeaderMsg,
    HeaderMsg,
    LineAndColumn,
    LineNumber,
    Quotation,
    UnderlinePrimary,
    UnderlineSecondary,
    LabelPrimary,
    LabelSecondary,
    NoStyle,
    Level(LevelInner),
    Addition,
    Removal,
}

impl ElementStyle {
    pub(crate) fn color_spec(&self, level: &Level<'_>, stylesheet: &Stylesheet) -> Style {
        match self {
            ElementStyle::Addition => stylesheet.addition,
            ElementStyle::Removal => stylesheet.removal,
            ElementStyle::LineAndColumn => stylesheet.none,
            ElementStyle::LineNumber => stylesheet.line_num,
            ElementStyle::Quotation => stylesheet.none,
            ElementStyle::MainHeaderMsg => stylesheet.emphasis,
            ElementStyle::UnderlinePrimary | ElementStyle::LabelPrimary => level.style(stylesheet),
            ElementStyle::UnderlineSecondary | ElementStyle::LabelSecondary => stylesheet.context,
            ElementStyle::HeaderMsg | ElementStyle::NoStyle => stylesheet.none,
            ElementStyle::Level(lvl) => lvl.style(stylesheet),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct UnderlineParts {
    pub(crate) style: ElementStyle,
    pub(crate) underline: char,
    pub(crate) label_start: char,
    pub(crate) vertical_text_line: char,
    pub(crate) multiline_vertical: char,
    pub(crate) multiline_horizontal: char,
    pub(crate) multiline_whole_line: char,
    pub(crate) multiline_start_down: char,
    pub(crate) bottom_right: char,
    pub(crate) top_left: char,
    pub(crate) top_right_flat: char,
    pub(crate) bottom_left: char,
    pub(crate) multiline_end_up: char,
    pub(crate) multiline_end_same_line: char,
    pub(crate) multiline_bottom_right_with_text: char,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TitleStyle {
    MainHeader,
    Header,
    Secondary,
}

struct PreProcessedGroup<'a> {
    group: &'a Group<'a>,
    elements: Vec<PreProcessedElement<'a>>,
    primary_path: Option<&'a Cow<'a, str>>,
    max_depth: usize,
}

enum PreProcessedElement<'a> {
    Message(&'a Message<'a>),
    Cause(
        (
            &'a Snippet<'a, Annotation<'a>>,
            SourceMap<'a>,
            Vec<AnnotatedLineInfo<'a>>,
        ),
    ),
    Suggestion(
        (
            &'a Snippet<'a, Patch<'a>>,
            SourceMap<'a>,
            SplicedLines<'a>,
            DisplaySuggestion,
        ),
    ),
    Origin(&'a Origin<'a>),
    Padding(Padding),
}

fn pre_process<'a>(
    groups: &'a [Group<'a>],
) -> (usize, Option<&'a Cow<'a, str>>, Vec<PreProcessedGroup<'a>>) {
    let mut max_line_num = 0;
    let mut og_primary_path = None;
    let mut out = Vec::with_capacity(groups.len());
    for group in groups {
        let mut elements = Vec::with_capacity(group.elements.len());
        let mut primary_path = None;
        let mut max_depth = 0;
        for element in &group.elements {
            match element {
                Element::Message(message) => {
                    elements.push(PreProcessedElement::Message(message));
                }
                Element::Cause(cause) => {
                    let sm = SourceMap::new(&cause.source, cause.line_start);
                    let (depth, annotated_lines) =
                        sm.annotated_lines(cause.markers.clone(), cause.fold);

                    if cause.fold {
                        let end = cause
                            .markers
                            .iter()
                            .map(|a| a.span.end)
                            .max()
                            .unwrap_or(cause.source.len())
                            .min(cause.source.len());

                        max_line_num = max(
                            cause.line_start + newline_count(&cause.source[..end]),
                            max_line_num,
                        );
                    } else {
                        max_line_num = max(
                            cause.line_start + newline_count(&cause.source),
                            max_line_num,
                        );
                    }

                    if primary_path.is_none() {
                        primary_path = Some(cause.path.as_ref());
                    }
                    max_depth = max(depth, max_depth);
                    elements.push(PreProcessedElement::Cause((cause, sm, annotated_lines)));
                }
                Element::Suggestion(suggestion) => {
                    let sm = SourceMap::new(&suggestion.source, suggestion.line_start);
                    if let Some((complete, patches, highlights)) =
                        sm.splice_lines(suggestion.markers.clone(), suggestion.fold)
                    {
                        let display_suggestion = DisplaySuggestion::new(&complete, &patches, &sm);

                        if suggestion.fold {
                            if let Some(first) = patches.first() {
                                let (l_start, _) =
                                    sm.span_to_locations(first.original_span.clone());
                                let nc = newline_count(&complete);
                                let sugg_max_line_num = match display_suggestion {
                                    DisplaySuggestion::Underline => l_start.line,
                                    DisplaySuggestion::Diff => {
                                        let file_lines = sm.span_to_lines(first.span.clone());
                                        file_lines
                                            .last()
                                            .map_or(l_start.line + nc, |line| line.line_index)
                                    }
                                    DisplaySuggestion::None => l_start.line + nc,
                                    DisplaySuggestion::Add => l_start.line + nc,
                                };
                                max_line_num = max(sugg_max_line_num, max_line_num);
                            }
                        } else {
                            max_line_num = max(
                                suggestion.line_start + newline_count(&complete),
                                max_line_num,
                            );
                        }

                        elements.push(PreProcessedElement::Suggestion((
                            suggestion,
                            sm,
                            (complete, patches, highlights),
                            display_suggestion,
                        )));
                    }
                }
                Element::Origin(origin) => {
                    if primary_path.is_none() {
                        primary_path = Some(Some(&origin.path));
                    }
                    elements.push(PreProcessedElement::Origin(origin));
                }
                Element::Padding(padding) => {
                    elements.push(PreProcessedElement::Padding(padding.clone()));
                }
            }
        }
        let group = PreProcessedGroup {
            group,
            elements,
            primary_path: primary_path.unwrap_or_default(),
            max_depth,
        };
        if og_primary_path.is_none() && group.primary_path.is_some() {
            og_primary_path = group.primary_path;
        }
        out.push(group);
    }

    (max_line_num, og_primary_path, out)
}

fn newline_count(body: &str) -> usize {
    #[cfg(feature = "simd")]
    {
        memchr::memchr_iter(b'\n', body.as_bytes()).count()
    }
    #[cfg(not(feature = "simd"))]
    {
        body.lines().count().saturating_sub(1)
    }
}

#[cfg(test)]
mod test {
    use super::{newline_count, OUTPUT_REPLACEMENTS};
    use snapbox::IntoData;

    fn format_replacements(replacements: Vec<(char, &str)>) -> String {
        replacements
            .into_iter()
            .map(|r| format!("    {r:?}"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    /// The [`OUTPUT_REPLACEMENTS`] array must be sorted (for binary search to
    /// work) and must contain no duplicate entries
    fn ensure_output_replacements_is_sorted() {
        let mut expected = OUTPUT_REPLACEMENTS.to_owned();
        expected.sort_by_key(|r| r.0);
        expected.dedup_by_key(|r| r.0);
        let expected = format_replacements(expected);
        let actual = format_replacements(OUTPUT_REPLACEMENTS.to_owned());
        snapbox::assert_data_eq!(actual, expected.into_data().raw());
    }

    #[test]
    fn ensure_newline_count_correct() {
        let source = r#"
                cargo-features = ["path-bases"]

                [package]
                name = "foo"
                version = "0.5.0"
                authors = ["wycats@example.com"]

                [dependencies]
                bar = { base = '^^not-valid^^', path = 'bar' }
            "#;
        let actual_count = newline_count(source);
        let expected_count = 10;

        assert_eq!(expected_count, actual_count);
    }
}
