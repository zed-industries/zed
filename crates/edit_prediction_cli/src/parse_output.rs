use crate::{PredictionProvider, example::Example, format_prompt::TeacherPrompt};
use anyhow::{Context as _, Result};
use zeta_prompt::{CURSOR_MARKER, ZetaVersion};

pub fn run_parse_output(example: &mut Example) -> Result<()> {
    let provider = example
        .prompt
        .as_ref()
        .context("prompt required (run format-prompt first)")?
        .provider;
    example
        .prompt_inputs
        .as_ref()
        .context("prompt_inputs required")?;

    let parsed_patches: Vec<_> = example
        .predictions
        .iter()
        .enumerate()
        .filter(|(_, p)| !p.actual_output.is_empty())
        .map(|(ix, prediction)| {
            let result = parse_prediction_output(example, &prediction.actual_output, provider);
            result.map(|(patch, cursor_offset)| (ix, patch, cursor_offset))
        })
        .collect::<Result<Vec<_>>>()?;

    for (ix, actual_patch, actual_cursor_offset) in parsed_patches {
        example.predictions[ix].actual_patch = Some(actual_patch);
        example.predictions[ix].actual_cursor_offset = actual_cursor_offset;
        example.predictions[ix].provider = provider;
    }

    Ok(())
}

pub fn parse_prediction_output(
    example: &Example,
    actual_output: &str,
    provider: PredictionProvider,
) -> Result<(String, Option<usize>)> {
    match provider {
        PredictionProvider::Teacher(_) | PredictionProvider::TeacherNonBatching(_) => {
            TeacherPrompt::parse(example, actual_output)
        }
        PredictionProvider::Zeta2(version) => parse_zeta2_output(example, actual_output, version),
        _ => anyhow::bail!(
            "parse-output only supports Teacher and Zeta2 providers, got {:?}",
            provider
        ),
    }
}

fn extract_zeta2_current_region(prompt: &str, version: ZetaVersion) -> Result<String> {
    let (current_marker, end_marker) = match version {
        ZetaVersion::V0112MiddleAtEnd => ("<|fim_middle|>current\n", "<|fim_middle|>updated"),
        ZetaVersion::V0113Ordered | ZetaVersion::V0114180EditableRegion => {
            ("<|fim_middle|>current\n", "<|fim_suffix|>")
        }
        ZetaVersion::V0120GitMergeMarkers | ZetaVersion::V0131GitMergeMarkersPrefix => (
            zeta_prompt::v0120_git_merge_markers::START_MARKER,
            zeta_prompt::v0120_git_merge_markers::SEPARATOR,
        ),
    };

    let start = prompt.find(current_marker).with_context(|| {
        format!(
            "missing current marker '{}' in prompt",
            current_marker.trim()
        )
    })? + current_marker.len();

    let end = prompt[start..]
        .find(end_marker)
        .with_context(|| format!("missing end marker '{}' in prompt", end_marker.trim()))?
        + start;

    let region = &prompt[start..end];
    let region = region.strip_suffix('\n').unwrap_or(region);
    let region = region.replace(CURSOR_MARKER, "");

    Ok(region)
}

fn parse_zeta2_output(
    example: &Example,
    actual_output: &str,
    version: ZetaVersion,
) -> Result<(String, Option<usize>)> {
    let prompt = &example.prompt.as_ref().context("prompt required")?.input;
    let prompt_inputs = example
        .prompt_inputs
        .as_ref()
        .context("prompt_inputs required")?;

    let old_text = extract_zeta2_current_region(prompt, version)?;

    let mut new_text = actual_output.to_string();
    let cursor_offset = if let Some(offset) = new_text.find(CURSOR_MARKER) {
        new_text.replace_range(offset..offset + CURSOR_MARKER.len(), "");
        Some(offset)
    } else {
        None
    };

    let suffix = match version {
        ZetaVersion::V0131GitMergeMarkersPrefix => {
            zeta_prompt::v0131_git_merge_markers_prefix::END_MARKER
        }
        ZetaVersion::V0120GitMergeMarkers => zeta_prompt::v0120_git_merge_markers::END_MARKER,
        _ => "",
    };
    if !suffix.is_empty() {
        new_text = new_text
            .strip_suffix(suffix)
            .unwrap_or(&new_text)
            .to_string();
    }

    let mut old_text_normalized = old_text.clone();
    if !new_text.is_empty() && !new_text.ends_with('\n') {
        new_text.push('\n');
    }
    if !old_text_normalized.is_empty() && !old_text_normalized.ends_with('\n') {
        old_text_normalized.push('\n');
    }

    let old_text_trimmed = old_text.trim_end_matches('\n');
    let (editable_region_offset, _) = prompt_inputs
        .content
        .match_indices(old_text_trimmed)
        .min_by_key(|(index, _)| index.abs_diff(prompt_inputs.cursor_offset))
        .with_context(|| {
            format!(
                "could not find editable region in content.\nLooking for:\n{}\n\nIn content:\n{}",
                old_text_trimmed, &prompt_inputs.content
            )
        })?;

    let editable_region_start_line = prompt_inputs.content[..editable_region_offset]
        .matches('\n')
        .count();

    // Use full context so cursor offset (relative to editable region start) aligns with diff content
    let editable_region_lines = old_text_normalized.lines().count() as u32;
    let diff = language::unified_diff_with_context(
        &old_text_normalized,
        &new_text,
        editable_region_start_line as u32,
        editable_region_start_line as u32,
        editable_region_lines,
    );

    let formatted_diff = format!(
        "--- a/{path}\n+++ b/{path}\n{diff}",
        path = example.spec.cursor_path.to_string_lossy(),
    );

    Ok((formatted_diff, cursor_offset))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_zeta2_current_region_v0113() {
        let prompt = indoc::indoc! {"
            <|file_sep|>src/main.rs
            <|fim_prefix|>
            fn main() {
            <|fim_middle|>current
            println!(\"hello\");
            <|fim_suffix|>
            }
            <|fim_middle|>updated
        "};

        let region = extract_zeta2_current_region(prompt, ZetaVersion::V0113Ordered).unwrap();
        assert_eq!(region, "println!(\"hello\");");
    }

    #[test]
    fn test_extract_zeta2_current_region_v0112() {
        let prompt = indoc::indoc! {"
            <|file_sep|>src/main.rs
            <|fim_prefix|>
            fn main() {
            <|fim_suffix|>
            }
            <|fim_middle|>current
            println!(\"hello\");
            <|fim_middle|>updated
        "};

        let region = extract_zeta2_current_region(prompt, ZetaVersion::V0112MiddleAtEnd).unwrap();
        assert_eq!(region, "println!(\"hello\");");
    }

    #[test]
    fn test_extract_zeta2_current_region_with_cursor_marker() {
        let prompt = indoc::indoc! {"
            <|file_sep|>src/main.rs
            <|fim_prefix|>
            fn main() {
            <|fim_middle|>current
            print<|user_cursor|>ln!(\"hello\");
            <|fim_suffix|>
            }
            <|fim_middle|>updated
        "};

        let region = extract_zeta2_current_region(prompt, ZetaVersion::V0113Ordered).unwrap();
        assert_eq!(region, "println!(\"hello\");");
    }

    #[test]
    fn test_extract_zeta2_current_region_v0120_git_merge_markers() {
        let prompt = indoc::indoc! {"
            <|file_sep|>src/main.rs
            <|fim_prefix|>
            fn main() {
            <|fim_suffix|>
            }
            <|fim_middle|><<<<<<< CURRENT
            println!(\"hello\");
            =======
        "};

        let region =
            extract_zeta2_current_region(prompt, ZetaVersion::V0120GitMergeMarkers).unwrap();
        assert_eq!(region, "println!(\"hello\");");
    }

    #[test]
    fn test_extract_zeta2_current_region_v0120_with_cursor_marker() {
        let prompt = indoc::indoc! {"
            <|file_sep|>src/main.rs
            <|fim_prefix|>
            fn main() {
            <|fim_suffix|>
            }
            <|fim_middle|><<<<<<< CURRENT
            print<|user_cursor|>ln!(\"hello\");
            =======
        "};

        let region =
            extract_zeta2_current_region(prompt, ZetaVersion::V0120GitMergeMarkers).unwrap();
        assert_eq!(region, "println!(\"hello\");");
    }
}
