use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Range;

use crate::diagnostic::{Diagnostic, LabelStyle};
use crate::files::{Error, Files, Location};
use crate::term::renderer::{Locus, MultiLabel, Renderer, SingleLabel};
use crate::term::Config;

/// Calculate the number of decimal digits in `n`.
fn count_digits(n: usize) -> usize {
    n.ilog10() as usize + 1
}

/// Output a richly formatted diagnostic, with source code previews.
pub struct RichDiagnostic<'diagnostic, 'config, FileId> {
    diagnostic: &'diagnostic Diagnostic<FileId>,
    config: &'config Config,
}

impl<'diagnostic, 'config, FileId> RichDiagnostic<'diagnostic, 'config, FileId>
where
    FileId: Copy + PartialEq,
{
    pub fn new(
        diagnostic: &'diagnostic Diagnostic<FileId>,
        config: &'config Config,
    ) -> RichDiagnostic<'diagnostic, 'config, FileId> {
        RichDiagnostic { diagnostic, config }
    }

    pub fn render<'files>(
        &self,
        files: &'files (impl Files<'files, FileId = FileId> + ?Sized),
        renderer: &mut Renderer<'_, '_>,
    ) -> Result<(), Error>
    where
        FileId: 'files,
    {
        use alloc::collections::BTreeMap;

        struct LabeledFile<'diagnostic, FileId> {
            file_id: FileId,
            start: usize,
            name: String,
            location: Location,
            num_multi_labels: usize,
            lines: BTreeMap<usize, Line<'diagnostic>>,
            max_label_style: LabelStyle,
        }

        impl<'diagnostic, FileId> LabeledFile<'diagnostic, FileId> {
            fn get_or_insert_line(
                &mut self,
                line_index: usize,
                line_range: Range<usize>,
                line_number: usize,
            ) -> &mut Line<'diagnostic> {
                self.lines.entry(line_index).or_insert_with(|| Line {
                    range: line_range,
                    number: line_number,
                    single_labels: vec![],
                    multi_labels: vec![],
                    // This has to be false by default so we know if it must be rendered by another condition already.
                    must_render: false,
                })
            }
        }

        struct Line<'diagnostic> {
            number: usize,
            range: core::ops::Range<usize>,
            // TODO: How do we reuse these allocations?
            single_labels: Vec<SingleLabel<'diagnostic>>,
            multi_labels: Vec<(usize, LabelStyle, MultiLabel<'diagnostic>)>,
            must_render: bool,
        }

        // TODO: Make this data structure external, to allow for allocation reuse
        let mut labeled_files = Vec::<LabeledFile<'_, _>>::new();
        // Keep track of the outer padding to use when rendering the
        // snippets of source code.
        let mut outer_padding = 0;

        // Group labels by file
        for label in &self.diagnostic.labels {
            let start_line_index = files.line_index(label.file_id, label.range.start)?;
            let start_line_number = files.line_number(label.file_id, start_line_index)?;
            let start_line_range = files.line_range(label.file_id, start_line_index)?;
            let end_line_index = files.line_index(label.file_id, label.range.end)?;
            let end_line_number = files.line_number(label.file_id, end_line_index)?;
            let end_line_range = files.line_range(label.file_id, end_line_index)?;

            outer_padding = core::cmp::max(outer_padding, count_digits(start_line_number));
            outer_padding = core::cmp::max(outer_padding, count_digits(end_line_number));

            // NOTE: This could be made more efficient by using an associative
            // data structure like a hashmap or B-tree,  but we use a vector to
            // preserve the order that unique files appear in the list of labels.
            let labeled_file = labeled_files
                .iter_mut()
                .find(|labeled_file| label.file_id == labeled_file.file_id);
            let labeled_file = if let Some(labeled_file) = labeled_file {
                // another diagnostic also referenced this file
                if labeled_file.max_label_style > label.style
                    || (labeled_file.max_label_style == label.style
                        && labeled_file.start > label.range.start)
                {
                    // this label has a higher style or has the same style but starts earlier
                    labeled_file.start = label.range.start;
                    labeled_file.location = files.location(label.file_id, label.range.start)?;
                    labeled_file.max_label_style = label.style;
                }
                labeled_file
            } else {
                // no other diagnostic referenced this file yet
                labeled_files.push(LabeledFile {
                    file_id: label.file_id,
                    start: label.range.start,
                    name: files.name(label.file_id)?.to_string(),
                    location: files.location(label.file_id, label.range.start)?,
                    num_multi_labels: 0,
                    lines: BTreeMap::new(),
                    max_label_style: label.style,
                });
                // this unwrap should never fail because we just pushed an element
                labeled_files
                    .last_mut()
                    .expect("just pushed an element that disappeared")
            };

            // insert context lines before label
            // start from 1 because 0 would be the start of the label itself
            for offset in 1..=self.config.before_label_lines {
                let index = if let Some(index) = start_line_index.checked_sub(offset) {
                    index
                } else {
                    // we are going from smallest to largest offset, so if
                    // the offset can not be subtracted from the start we
                    // reached the first line
                    break;
                };

                if let Ok(range) = files.line_range(label.file_id, index) {
                    let line =
                        labeled_file.get_or_insert_line(index, range, start_line_number - offset);
                    line.must_render = true;
                } else {
                    break;
                }
            }

            // insert context lines after label
            // start from 1 because 0 would be the end of the label itself
            for offset in 1..=self.config.after_label_lines {
                let index = end_line_index
                    .checked_add(offset)
                    .expect("line index too big");

                if let Ok(range) = files.line_range(label.file_id, index) {
                    let line =
                        labeled_file.get_or_insert_line(index, range, end_line_number + offset);
                    line.must_render = true;
                } else {
                    break;
                }
            }

            if start_line_index == end_line_index {
                // Single line
                //
                // ```text
                // 2 │ (+ test "")
                //   │         ^^ expected `Int` but found `String`
                // ```
                let label_start = label.range.start - start_line_range.start;
                // Ensure that we print at least one caret, even when we
                // have a zero-length source range.
                let label_end =
                    usize::max(label.range.end - start_line_range.start, label_start + 1);

                let line = labeled_file.get_or_insert_line(
                    start_line_index,
                    start_line_range,
                    start_line_number,
                );

                // Ensure that the single line labels are lexicographically
                // sorted by the range of source code that they cover.
                let index = match line.single_labels.binary_search_by(|(_, range, _)| {
                    // `Range<usize>` doesn't implement `Ord`, so convert to `(usize, usize)`
                    // to piggyback off its lexicographic comparison implementation.
                    (range.start, range.end).cmp(&(label_start, label_end))
                }) {
                    // If the ranges are the same, order the labels in reverse
                    // to how they were originally specified in the diagnostic.
                    // This helps with printing in the renderer.
                    Ok(index) | Err(index) => index,
                };

                line.single_labels
                    .insert(index, (label.style, label_start..label_end, &label.message));

                // If this line is not rendered, the SingleLabel is not visible.
                line.must_render = true;
            } else {
                // Multiple lines
                //
                // ```text
                // 4 │   fizz₁ num = case (mod num 5) (mod num 3) of
                //   │ ╭─────────────^
                // 5 │ │     0 0 => "FizzBuzz"
                // 6 │ │     0 _ => "Fizz"
                // 7 │ │     _ 0 => "Buzz"
                // 8 │ │     _ _ => num
                //   │ ╰──────────────^ `case` clauses have incompatible types
                // ```

                let label_index = labeled_file.num_multi_labels;
                labeled_file.num_multi_labels += 1;

                // First labeled line
                let label_start = label.range.start - start_line_range.start;

                let start_line = labeled_file.get_or_insert_line(
                    start_line_index,
                    start_line_range.clone(),
                    start_line_number,
                );

                start_line.multi_labels.push((
                    label_index,
                    label.style,
                    MultiLabel::Top(label_start),
                ));

                // The first line has to be rendered so the start of the label is visible.
                start_line.must_render = true;

                // Marked lines
                //
                // ```text
                // 5 │ │     0 0 => "FizzBuzz"
                // 6 │ │     0 _ => "Fizz"
                // 7 │ │     _ 0 => "Buzz"
                // ```
                for line_index in (start_line_index + 1)..end_line_index {
                    let line_range = files.line_range(label.file_id, line_index)?;
                    let line_number = files.line_number(label.file_id, line_index)?;

                    outer_padding = core::cmp::max(outer_padding, count_digits(line_number));

                    let line = labeled_file.get_or_insert_line(line_index, line_range, line_number);

                    line.multi_labels
                        .push((label_index, label.style, MultiLabel::Left));

                    // The line should be rendered to match the configuration of how much context to show.
                    line.must_render |=
                        // Is this line part of the context after the start of the label?
                        line_index - start_line_index <= self.config.start_context_lines
                        ||
                        // Is this line part of the context before the end of the label?
                        end_line_index - line_index <= self.config.end_context_lines;
                }

                // Last labeled line
                //
                // ```text
                // 8 │ │     _ _ => num
                //   │ ╰──────────────^ `case` clauses have incompatible types
                // ```
                let label_end = label.range.end - end_line_range.start;

                let end_line = labeled_file.get_or_insert_line(
                    end_line_index,
                    end_line_range,
                    end_line_number,
                );

                end_line.multi_labels.push((
                    label_index,
                    label.style,
                    MultiLabel::Bottom(label_end, &label.message),
                ));

                // The last line has to be rendered so the end of the label is visible.
                end_line.must_render = true;
            }
        }

        // Header and message
        //
        // ```text
        // error[E0001]: unexpected type in `+` application
        // ```
        renderer.render_header(
            None,
            self.diagnostic.severity,
            self.diagnostic.code.as_deref(),
            self.diagnostic.message.as_str(),
        )?;

        // Source snippets
        //
        // ```text
        //   ┌─ test:2:9
        //   │
        // 2 │ (+ test "")
        //   │         ^^ expected `Int` but found `String`
        //   │
        // ```
        let mut labeled_files = labeled_files.into_iter().peekable();
        while let Some(labeled_file) = labeled_files.next() {
            let source = files.source(labeled_file.file_id)?;
            let source = source.as_ref();

            // Top left border and locus.
            //
            // ```text
            // ┌─ test:2:9
            // ```
            if !labeled_file.lines.is_empty() {
                renderer.render_snippet_start(
                    outer_padding,
                    &Locus {
                        name: labeled_file.name,
                        location: labeled_file.location,
                    },
                )?;
                renderer.render_snippet_empty(
                    outer_padding,
                    self.diagnostic.severity,
                    labeled_file.num_multi_labels,
                    &[],
                )?;
            }

            let mut lines = labeled_file
                .lines
                .iter()
                .filter(|(_, line)| line.must_render)
                .peekable();

            while let Some((line_index, line)) = lines.next() {
                renderer.render_snippet_source(
                    outer_padding,
                    line.number,
                    &source[line.range.clone()],
                    self.diagnostic.severity,
                    &line.single_labels,
                    labeled_file.num_multi_labels,
                    &line.multi_labels,
                )?;

                // Check to see if we need to render any intermediate stuff
                // before rendering the next line.
                if let Some((next_line_index, next_line)) = lines.peek() {
                    match next_line_index.checked_sub(*line_index) {
                        // Consecutive lines
                        Some(1) => {}
                        // One line between the current line and the next line
                        Some(2) => {
                            // Write a source line
                            let file_id = labeled_file.file_id;

                            // This line was not intended to be rendered initially.
                            // To render the line right, we have to get back the original labels.
                            let labels = labeled_file
                                .lines
                                .get(&(line_index + 1))
                                .map_or(&[][..], |line| &line.multi_labels[..]);

                            renderer.render_snippet_source(
                                outer_padding,
                                files.line_number(file_id, line_index + 1)?,
                                &source[files.line_range(file_id, line_index + 1)?],
                                self.diagnostic.severity,
                                &[],
                                labeled_file.num_multi_labels,
                                labels,
                            )?;
                        }
                        // More than one line between the current line and the next line.
                        Some(_) | None => {
                            // Source break
                            //
                            // ```text
                            // ·
                            // ```
                            renderer.render_snippet_break(
                                outer_padding,
                                self.diagnostic.severity,
                                labeled_file.num_multi_labels,
                                &next_line.multi_labels,
                            )?;
                        }
                    }
                }
            }

            // Check to see if we should render a trailing border after the
            // final line of the snippet.
            if labeled_files.peek().is_none() && self.diagnostic.notes.is_empty() {
                // We don't render a border if we are at the final newline
                // without trailing notes, because it would end up looking too
                // spaced-out in combination with the final new line.
            } else {
                // Render the trailing snippet border.
                renderer.render_snippet_empty(
                    outer_padding,
                    self.diagnostic.severity,
                    labeled_file.num_multi_labels,
                    &[],
                )?;
            }
        }

        // Additional notes
        //
        // ```text
        // = expected type `Int`
        //      found type `String`
        // ```
        for note in &self.diagnostic.notes {
            renderer.render_snippet_note(outer_padding, note)?;
        }
        renderer.render_empty()
    }
}

/// Output a short diagnostic, with a line number, severity, and message.
pub struct ShortDiagnostic<'diagnostic, FileId> {
    diagnostic: &'diagnostic Diagnostic<FileId>,
    show_notes: bool,
}

impl<'diagnostic, FileId> ShortDiagnostic<'diagnostic, FileId>
where
    FileId: Copy + PartialEq,
{
    #[must_use]
    pub fn new(
        diagnostic: &'diagnostic Diagnostic<FileId>,
        show_notes: bool,
    ) -> ShortDiagnostic<'diagnostic, FileId> {
        ShortDiagnostic {
            diagnostic,
            show_notes,
        }
    }

    pub fn render<'files>(
        &self,
        files: &'files (impl Files<'files, FileId = FileId> + ?Sized),
        renderer: &mut Renderer<'_, '_>,
    ) -> Result<(), Error>
    where
        FileId: 'files,
    {
        // Located headers
        //
        // ```text
        // test:2:9: error[E0001]: unexpected type in `+` application
        // ```
        let mut primary_labels_encountered = 0;
        let labels = self.diagnostic.labels.iter();
        for label in labels.filter(|label| label.style == LabelStyle::Primary) {
            primary_labels_encountered += 1;

            renderer.render_header(
                Some(&Locus {
                    name: files.name(label.file_id)?.to_string(),
                    location: files.location(label.file_id, label.range.start)?,
                }),
                self.diagnostic.severity,
                self.diagnostic.code.as_deref(),
                self.diagnostic.message.as_str(),
            )?;
        }

        // Fallback to printing a non-located header if no primary labels were encountered
        //
        // ```text
        // error[E0002]: Bad config found
        // ```
        if primary_labels_encountered == 0 {
            renderer.render_header(
                None,
                self.diagnostic.severity,
                self.diagnostic.code.as_deref(),
                self.diagnostic.message.as_str(),
            )?;
        }

        if self.show_notes {
            // Additional notes
            //
            // ```text
            // = expected type `Int`
            //      found type `String`
            // ```
            for note in &self.diagnostic.notes {
                renderer.render_snippet_note(0, note)?;
            }
        }

        Ok(())
    }
}
