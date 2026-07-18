use crate::example::{Example, read_example_files};
use crate::reorder_patch::{Hunk, Patch, PatchLine};
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug, Clone)]
pub struct TruncatePatchArgs {
    /// Number of logical groups ahead to leave
    #[arg(long)]
    pub num_groups: usize,

    /// Leave only edits in the file under the cursor
    #[arg(long, default_value_t = false)]
    pub current_file_only: bool,
}

pub fn run_truncate_expected_patch(
    args: &TruncatePatchArgs,
    inputs: &[PathBuf],
) -> anyhow::Result<()> {
    let stdin_path = PathBuf::from("-");
    let inputs = if inputs.is_empty() {
        std::slice::from_ref(&stdin_path)
    } else {
        inputs
    };

    let mut examples = read_example_files(inputs);
    for example in &mut examples {
        run_one_input(example, args)?;

        println!("{}", serde_json::to_string(&example)?);
    }
    Ok(())
}

fn run_one_input(example: &mut Example, args: &TruncatePatchArgs) -> anyhow::Result<()> {
    let mut patch = Patch::parse_unified_diff(&example.spec.expected_patches[0]);
    let mut groups_left = args.num_groups;

    patch.hunks.retain(|hunk| {
        if groups_left == 0 {
            return false;
        }
        if starts_new_group(hunk) {
            groups_left -= 1;
        }

        if args.current_file_only {
            return hunk.filename == example.spec.cursor_path.display().to_string();
        }

        true
    });

    // Remove all group headers
    patch.header = String::new();
    patch.hunks.iter_mut().for_each(|hunk| {
        hunk.lines.retain(|line| match line {
            PatchLine::Garbage(line) => !line.starts_with("//"),
            _ => true,
        });
    });

    example.spec.expected_patches[0] = patch.to_string();

    Ok(())
}

fn starts_new_group(hunk: &Hunk) -> bool {
    hunk.lines.iter().any(|line| match line {
        PatchLine::Garbage(content) => content.starts_with("///"),
        _ => false,
    })
}
