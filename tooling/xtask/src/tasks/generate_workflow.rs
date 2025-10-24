use anyhow::Result;
use clap::Parser;

use gh_workflow::*;

#[derive(Parser)]
pub struct GenerateWorkflowArgs {}

pub fn run_generate_workflow(_args: GenerateWorkflowArgs) -> Result<()> {
    // Create the "Run tests" composite action workflow
    let workflow = Workflow::default().name("Run tests").add_job(
        "run_tests",
        Job::default()
            .add_step(Step::new("Install Rust").run("cargo install cargo-nextest --locked"))
            .add_step(
                Step::new("Install Node")
                    .uses(
                        "actions",
                        "setup-node",
                        "49933ea5288caeca8642d1e84afbd3f7d6820020",
                    )
                    .add_with(("node-version", "18")),
            )
            .add_step(
                Step::new("Limit target directory size")
                    .run("script/clear-target-dir-if-larger-than ${{ env.MAX_SIZE }}")
                    .env(("MAX_SIZE", "${{ runner.os == 'macOS' && 300 || 100 }}")),
            )
            .add_step(Step::new("Run tests").run(
                "cargo nextest run --workspace --no-fail-fast --failure-output immediate-final",
            )),
    );

    // Generate and print the workflow YAML
    let yaml = workflow
        .to_string()
        .map_err(|e| anyhow::anyhow!("{:?}", e))?;
    println!("{}", yaml);

    Ok(())
}
