use crate::{
    PredictionProvider,
    example::{ActualCursor, Example},
    format_prompt::{TeacherMultiRegionPrompt, TeacherPrompt},
    repair,
};
use anyhow::{Context as _, Result};
use zeta_prompt::{ZetaFormat, parse_zeta2_model_output, parsed_output_to_patch};

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
        PredictionProvider::Teacher(_, _) | PredictionProvider::TeacherNonBatching(_, _) => {
            TeacherPrompt::parse(example, actual_output)
        }
        PredictionProvider::TeacherMultiRegion(_)
        | PredictionProvider::TeacherMultiRegionNonBatching(_) => {
            TeacherMultiRegionPrompt::parse(example, actual_output)
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
    let range_in_excerpt = parsed.range_in_excerpt.clone();
    let excerpt = prompt_inputs.cursor_excerpt.as_ref();
    let editable_region_offset = range_in_excerpt.start;
    let editable_region_start_line = excerpt[..editable_region_offset].matches('\n').count();

    let mut new_text = parsed.new_editable_region.clone();
    if !new_text.is_empty() && !new_text.ends_with('\n') {
        new_text.push('\n');
    }

    let cursor_offset = parsed.cursor_offset_in_new_editable_region;
    let formatted_diff = parsed_output_to_patch(prompt_inputs, parsed)?;

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
