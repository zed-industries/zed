use std::path::Path;

use anyhow::{Result, bail};
use async_trait::async_trait;
use dap::{DapLocator, DebugRequest, adapters::DebugAdapterName};
use gpui::SharedString;

use task::{DebugScenario, SpawnInTerminal, TaskTemplate};

pub(crate) struct NodeLocator;

#[async_trait]
impl DapLocator for NodeLocator {
    fn name(&self) -> SharedString {
        SharedString::new_static("Node")
    }

    /// Determines whether this locator can generate debug target for given task.
    fn create_scenario(
        &self,
        build_config: &TaskTemplate,
        resolved_label: &str,
        adapter: DebugAdapterName,
    ) -> Option<DebugScenario> {
        if adapter.as_ref() != "JavaScript" {
            return None;
        }
        //  ./node_modules/.bin/jest
        //
        //
        // TODO this should be npm/deno/yarn/etc. + mocha/jake/jasmine/etc.
        let valid_program = build_config.command == "npx"
            && build_config
                .args
                .first()
                .is_some_and(|s| s.as_str() == "jest");
        if !valid_program {
            return None;
        }
        let args = Some("--runInBand".to_owned())
            .into_iter()
            .chain(build_config.args[1..].iter().cloned())
            .collect::<Vec<_>>();

        // npx --node-options="--inspect-brk" jest --testNamePattern foobar folder/file.ts
        let program_path = "$ZED_WORKTREE_ROOT/node_modules/.bin/jest";
        let mut config = serde_json::json!({
            "request": "launch",
            "program": program_path,
            "args": args,
            "cwd": build_config.cwd.clone(),
            "runtimeArgs": ["--inspect-brk"]
        });

        Some(DebugScenario {
            adapter: adapter.0,
            label: resolved_label.to_string().into(),
            build: None,
            config,
            tcp_connection: None,
        })
    }

    async fn run(&self, _: SpawnInTerminal) -> Result<DebugRequest> {
        bail!("Python locator should not require DapLocator::run to be ran");
    }
}
