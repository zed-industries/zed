use anyhow::{Context as _, Result, anyhow};
use language::{Anchor, BufferSnapshot, OffsetRangeExt as _, TextBufferSnapshot};
use std::ops::Range;
use std::path::Path;
use std::sync::Arc;

pub async fn parse_xml_edits<'a>(
    input: &'a str,
    get_buffer: impl Fn(&Path) -> Option<(&'a BufferSnapshot, &'a [Range<Anchor>])> + Send,
) -> Result<(&'a BufferSnapshot, Vec<(Range<Anchor>, Arc<str>)>)> {
    parse_xml_edits_inner(input, get_buffer)
        .await
        .with_context(|| format!("Failed to parse XML edits:\n{input}"))
}

async fn parse_xml_edits_inner<'a>(
    mut input: &'a str,
    get_buffer: impl Fn(&Path) -> Option<(&'a BufferSnapshot, &'a [Range<Anchor>])> + Send,
) -> Result<(&'a BufferSnapshot, Vec<(Range<Anchor>, Arc<str>)>)> {
    let edits_tag = parse_tag(&mut input, "edits")?.context("No edits tag")?;

    input = edits_tag.body;

    let file_path = edits_tag
        .attributes
        .trim_start()
        .strip_prefix("path")
        .context("no file attribute on edits tag")?
        .trim_end()
        .strip_prefix('=')
        .context("no value for path attribute")?
        .trim()
        .trim_start_matches('"')
        .trim_end_matches('"');

    let (buffer, context_ranges) = get_buffer(file_path.as_ref())
        .with_context(|| format!("no buffer for file {file_path}"))?;

    let mut edits = vec![];
    while let Some(old_text_tag) = parse_tag(&mut input, "old_text")? {
        let new_text_tag =
            parse_tag(&mut input, "new_text")?.context("no new_text tag following old_text")?;
        edits.extend(resolve_new_text_old_text_in_buffer(
            new_text_tag.body,
            old_text_tag.body,
            buffer,
            context_ranges,
        )?);
    }

    Ok((buffer, edits))
}

fn resolve_new_text_old_text_in_buffer(
    new_text: &str,
    old_text: &str,
    buffer: &TextBufferSnapshot,
    ranges: &[Range<Anchor>],
) -> Result<impl Iterator<Item = (Range<Anchor>, Arc<str>)>, anyhow::Error> {
    let context_offset = if old_text.is_empty() {
        Ok(0)
    } else {
        let mut offset = None;
        for range in ranges {
            let range = range.to_offset(buffer);
            let text = buffer.text_for_range(range.clone()).collect::<String>();
            for (match_offset, _) in text.match_indices(old_text) {
                if let Some(offset) = offset {
                    let offset_match_point = buffer.offset_to_point(offset);
                    let second_match_point = buffer.offset_to_point(range.start + match_offset);
                    anyhow::bail!(
                        "old_text is not unique enough:\n{}\nFound at {:?} and {:?}",
                        old_text,
                        offset_match_point,
                        second_match_point
                    );
                }
                offset = Some(range.start + match_offset);
            }
        }
        offset.ok_or_else(|| {
            #[cfg(any(debug_assertions, feature = "eval-support"))]
            if let Some(closest_match) = closest_old_text_match(buffer, old_text) {
                log::info!(
                    "Closest `old_text` match: {}",
                    pretty_assertions::StrComparison::new(old_text, &closest_match)
                )
            }
            anyhow!("Failed to match old_text:\n{}", old_text)
        })
    }?;

    let edits_within_hunk = language::text_diff(&old_text, &new_text);
    Ok(edits_within_hunk
        .into_iter()
        .map(move |(inner_range, inner_text)| {
            (
                buffer.anchor_after(context_offset + inner_range.start)
                    ..buffer.anchor_before(context_offset + inner_range.end),
                inner_text,
            )
        }))
}

#[cfg(any(debug_assertions, feature = "eval-support"))]
fn closest_old_text_match(buffer: &TextBufferSnapshot, old_text: &str) -> Option<String> {
    let buffer_text = buffer.text();
    let len = old_text.len();

    if len == 0 || buffer_text.len() < len {
        return None;
    }

    let mut min_score = usize::MAX;
    let mut min_start = 0;

    let old_text_bytes = old_text.as_bytes();
    let old_alpha_count = old_text_bytes
        .iter()
        .filter(|&&b| b.is_ascii_alphanumeric())
        .count();

    let old_line_count = old_text.lines().count();

    let mut cursor = 0;

    while cursor + len <= buffer_text.len() {
        let candidate = &buffer_text[cursor..cursor + len];
        let candidate_bytes = candidate.as_bytes();

        if usize::abs_diff(candidate.lines().count(), old_line_count) > 4 {
            cursor += 1;
            continue;
        }

        let candidate_alpha_count = candidate_bytes
            .iter()
            .filter(|&&b| b.is_ascii_alphanumeric())
            .count();

        // If alphanumeric character count differs by more than 30%, skip
        if usize::abs_diff(old_alpha_count, candidate_alpha_count) * 10 > old_alpha_count * 3 {
            cursor += 1;
            continue;
        }

        let score = strsim::levenshtein(candidate, old_text);
        if score < min_score {
            min_score = score;
            min_start = cursor;

            if min_score <= len / 10 {
                break;
            }
        }

        cursor += 1;
    }

    if min_score != usize::MAX {
        Some(buffer_text[min_start..min_start + len].to_string())
    } else {
        None
    }
}

struct ParsedTag<'a> {
    attributes: &'a str,
    body: &'a str,
}

fn parse_tag<'a>(input: &mut &'a str, tag: &str) -> Result<Option<ParsedTag<'a>>> {
    let open_tag = format!("<{}", tag);
    let close_tag = format!("</{}>", tag);
    let Some(start_ix) = input.find(&open_tag) else {
        return Ok(None);
    };
    let start_ix = start_ix + open_tag.len();
    let closing_bracket_ix = start_ix
        + input[start_ix..]
            .find('>')
            .with_context(|| format!("missing > after {tag}"))?;
    let attributes = &input[start_ix..closing_bracket_ix].trim();
    let end_ix = closing_bracket_ix
        + input[closing_bracket_ix..]
            .find(&close_tag)
            .with_context(|| format!("no `{close_tag}` tag"))?;
    let body = &input[closing_bracket_ix + '>'.len_utf8()..end_ix];
    let body = body.strip_prefix('\n').unwrap_or(body);
    *input = &input[end_ix + close_tag.len()..];
    Ok(Some(ParsedTag { attributes, body }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use indoc::indoc;
    use language::Point;
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    #[test]
    fn test_parse_tags() {
        let mut input = indoc! {r#"
            Prelude
            <tag attr="foo">
            tag value
            </tag>
            "# };
        let parsed = parse_tag(&mut input, "tag").unwrap().unwrap();
        assert_eq!(parsed.attributes, "attr=\"foo\"");
        assert_eq!(parsed.body, "tag value\n");
        assert_eq!(input, "\n");
    }

    #[gpui::test]
    async fn test_parse_xml_edits(cx: &mut TestAppContext) {
        let fs = init_test(cx);

        let buffer_1_text = indoc! {r#"
            one two three four
            five six seven eight
            nine ten eleven twelve
        "# };

        fs.insert_tree(
            path!("/root"),
            json!({
                "file1": buffer_1_text,
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/file1"), cx)
            })
            .await
            .unwrap();
        let buffer_snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());

        let edits = indoc! {r#"
            <edits path="root/file1">
            <old_text>
            five six seven eight
            </old_text>
            <new_text>
            five SIX seven eight!
            </new_text>
            </edits>
        "#};

        let (buffer, edits) = parse_xml_edits(edits, |_path| {
            Some((&buffer_snapshot, &[(Anchor::MIN..Anchor::MAX)] as &[_]))
        })
        .await
        .unwrap();

        let edits = edits
            .into_iter()
            .map(|(range, text)| (range.to_point(&buffer), text))
            .collect::<Vec<_>>();
        assert_eq!(
            edits,
            &[
                (Point::new(1, 5)..Point::new(1, 8), "SIX".into()),
                (Point::new(1, 20)..Point::new(1, 20), "!".into())
            ]
        );
    }

    fn init_test(cx: &mut TestAppContext) -> Arc<FakeFs> {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });

        FakeFs::new(cx.background_executor.clone())
    }
}
