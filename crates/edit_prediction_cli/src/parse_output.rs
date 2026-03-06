use crate::{
    PredictionProvider,
    example::{ActualCursor, Example},
    format_prompt::TeacherPrompt,
    repair,
};
use anyhow::{Context as _, Result};
use edit_prediction::example_spec::encode_cursor_in_patch;
use zeta_prompt::{CURSOR_MARKER, ZetaFormat, parse_zeta2_model_output};

pub fn run_parse_output(example: &mut Example) -> Result<()> {
    example
        .prompt_inputs
        .as_ref()
        .context("prompt_inputs required")?;

    let to_parse: Vec<_> = example
        .predictions
        .iter()
        .enumerate()
        .filter(|(_, p)| !p.actual_output.is_empty())
        .map(|(ix, p)| (ix, p.actual_output.clone(), p.provider))
        .collect();

    for (ix, actual_output, provider) in to_parse {
        let (actual_patch, actual_cursor) =
            parse_prediction_output(example, &actual_output, provider)?;
        example.predictions[ix].actual_patch = Some(actual_patch);
        example.predictions[ix].actual_cursor = actual_cursor;
    }

    Ok(())
}

pub fn parse_prediction_output(
    example: &Example,
    actual_output: &str,
    provider: PredictionProvider,
) -> Result<(String, Option<ActualCursor>)> {
    match provider {
        PredictionProvider::Teacher(_) | PredictionProvider::TeacherNonBatching(_) => {
            TeacherPrompt::parse(example, actual_output)
        }
        PredictionProvider::Zeta2(version) => parse_zeta2_output(example, actual_output, version),
        PredictionProvider::Repair => repair::parse(example, actual_output),
        _ => anyhow::bail!(
            "parse-output only supports Teacher and Zeta2 providers, got {:?}",
            provider
        ),
    }
}

fn parse_zeta2_output(
    example: &Example,
    actual_output: &str,
    format: ZetaFormat,
) -> Result<(String, Option<ActualCursor>)> {
    let prompt_inputs = example
        .prompt_inputs
        .as_ref()
        .context("prompt_inputs required")?;

    let parsed = parse_zeta2_model_output(actual_output, format, prompt_inputs)?;
    let range_in_excerpt = parsed.range_in_excerpt;

    let excerpt = prompt_inputs.cursor_excerpt.as_ref();
    let old_text = excerpt[range_in_excerpt.clone()].to_string();
    let mut new_text = parsed.new_editable_region;

    let cursor_offset = if let Some(offset) = new_text.find(CURSOR_MARKER) {
        new_text.replace_range(offset..offset + CURSOR_MARKER.len(), "");
        Some(offset)
    } else {
        None
    };

    // Normalize trailing newlines for diff generation
    let mut old_text_normalized = old_text;
    if !new_text.is_empty() && !new_text.ends_with('\n') {
        new_text.push('\n');
    }
    if !old_text_normalized.is_empty() && !old_text_normalized.ends_with('\n') {
        old_text_normalized.push('\n');
    }

    let editable_region_offset = range_in_excerpt.start;
    let editable_region_start_line = excerpt[..editable_region_offset].matches('\n').count();
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

    let formatted_diff = encode_cursor_in_patch(&formatted_diff, cursor_offset);

    let actual_cursor = cursor_offset.map(|editable_region_cursor_offset| {
        ActualCursor::from_editable_region(
            &example.spec.cursor_path,
            editable_region_cursor_offset,
            &new_text,
            excerpt,
            editable_region_offset,
            editable_region_start_line,
        )
    });

    Ok((formatted_diff, actual_cursor))
}
