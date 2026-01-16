use anyhow::Result;
use async_trait::async_trait;
use collections::HashMap;
use dap::{DapLocator, DebugRequest, adapters::DebugAdapterName};
use gpui::{BackgroundExecutor, SharedString};
use serde::{Deserialize, Serialize};
use task::{DebugScenario, SpawnInTerminal, TaskTemplate};

pub(crate) struct GoLocator;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct DelveLaunchRequest {
    request: String,
    mode: String,
    program: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    cwd: Option<String>,
    args: Vec<String>,
    build_flags: Vec<String>,
    env: HashMap<String, String>,
}

fn is_debug_flag(arg: &str) -> Option<bool> {
    let mut part = if let Some(suffix) = arg.strip_prefix("test.") {
        suffix
    } else {
        arg
    };
    let mut might_have_arg = true;
    if let Some(idx) = part.find('=') {
        might_have_arg = false;
        part = &part[..idx];
    }
    match part {
        "benchmem" | "failfast" | "fullpath" | "fuzzworker" | "json" | "short" | "v"
        | "paniconexit0" => Some(false),
        "bench"
        | "benchtime"
        | "blockprofile"
        | "blockprofilerate"
        | "count"
        | "coverprofile"
        | "cpu"
        | "cpuprofile"
        | "fuzz"
        | "fuzzcachedir"
        | "fuzzminimizetime"
        | "fuzztime"
        | "gocoverdir"
        | "list"
        | "memprofile"
        | "memprofilerate"
        | "mutexprofile"
        | "mutexprofilefraction"
        | "outputdir"
        | "parallel"
        | "run"
        | "shuffle"
        | "skip"
        | "testlogfile"
        | "timeout"
        | "trace" => Some(might_have_arg),
        _ if arg.starts_with("test.") => Some(false),
        _ => None,
    }
}

fn is_build_flag(mut arg: &str) -> Option<bool> {
    let mut might_have_arg = true;
    if let Some(idx) = arg.find('=') {
        might_have_arg = false;
        arg = &arg[..idx];
    }
    match arg {
        "a" | "n" | "race" | "msan" | "asan" | "cover" | "work" | "x" | "v" | "buildvcs"
        | "json" | "linkshared" | "modcacherw" | "trimpath" => Some(false),

        "p" | "covermode" | "coverpkg" | "asmflags" | "buildmode" | "compiler" | "gccgoflags"
        | "gcflags" | "installsuffix" | "ldflags" | "mod" | "modfile" | "overlay" | "pgo"
        | "pkgdir" | "tags" | "toolexec" => Some(might_have_arg),
        _ => None,
    }
}

#[async_trait]
impl DapLocator for GoLocator {
    fn name(&self) -> SharedString {
        SharedString::new_static("go-debug-locator")
    }

    async fn create_scenario(
        &self,
        build_config: &TaskTemplate,
        resolved_label: &str,
        adapter: &DebugAdapterName,
    ) -> Option<DebugScenario> {
        if build_config.command != "go" {
            return None;
        }
        let go_action = build_config.args.first()?;

        match go_action.as_str() {
            "test" => {
                let mut program = ".".to_string();
                let mut args = Vec::default();
                let mut build_flags = Vec::default();

                let mut all_args_are_test = false;
                let mut next_arg_is_test = false;
                let mut next_arg_is_build = false;
                let mut seen_pkg = false;
                let mut seen_v = false;

                for arg in build_config.args.iter().skip(1) {
                    if all_args_are_test || next_arg_is_test {
                        // HACK: tasks assume that they are run in a shell context,
                        // so the -run regex has escaped specials. Delve correctly
                        // handles escaping, so we undo that here.
                        if let Some((left, right)) = arg.split_once("/")
                            && left.starts_with("\\^")
                            && left.ends_with("\\$")
                            && right.starts_with("\\^")
                            && right.ends_with("\\$")
                        {
                            let mut left = left[1..left.len() - 2].to_string();
                            left.push('$');

                            let mut right = right[1..right.len() - 2].to_string();
                            right.push('$');

                            args.push(format!("{left}/{right}"));
                        } else if arg.starts_with("\\^") && arg.ends_with("\\$") {
                            let mut arg = arg[1..arg.len() - 2].to_string();
                            arg.push('$');
                            args.push(arg);
                        } else {
                            args.push(arg.clone());
                        }
                        next_arg_is_test = false;
                    } else if next_arg_is_build {
                        build_flags.push(arg.clone());
                        next_arg_is_build = false;
                    } else if arg.starts_with('-') {
                        let flag = arg.trim_start_matches('-');
                        if flag == "args" {
                            all_args_are_test = true;
                        } else if let Some(has_arg) = is_debug_flag(flag) {
                            if flag == "v" || flag == "test.v" {
                                seen_v = true;
                            }
                            if flag.starts_with("test.") {
                                args.push(arg.clone());
                            } else {
                                args.push(format!("-test.{flag}"))
                            }
                            next_arg_is_test = has_arg;
                        } else if let Some(has_arg) = is_build_flag(flag) {
                            build_flags.push(arg.clone());
                            next_arg_is_build = has_arg;
                        }
                    } else if !seen_pkg {
                        program = arg.clone();
                        seen_pkg = true;
                    } else {
                        args.push(arg.clone());
                    }
                }
                if !seen_v {
                    args.push("-test.v".to_string());
                }

                let config: serde_json::Value = serde_json::to_value(DelveLaunchRequest {
                    request: "launch".to_string(),
                    mode: "test".to_string(),
                    program,
                    args,
                    build_flags,
                    cwd: build_config.cwd.clone(),
                    env: build_config.env.clone(),
                })
                .unwrap();

                Some(DebugScenario {
                    label: resolved_label.to_string().into(),
                    adapter: adapter.0.clone(),
                    build: None,
                    config,
                    tcp_connection: None,
                })
            }
            "run" => {
                let mut next_arg_is_build = false;
                let mut seen_pkg = false;

                let mut program = ".".to_string();
                let mut args = Vec::default();
                let mut build_flags = Vec::default();

                for arg in build_config.args.iter().skip(1) {
                    if seen_pkg {
                        args.push(arg.clone())
                    } else if next_arg_is_build {
                        build_flags.push(arg.clone());
                        next_arg_is_build = false;
                    } else if arg.starts_with("-") {
                        if let Some(has_arg) = is_build_flag(arg.trim_start_matches("-")) {
                            next_arg_is_build = has_arg;
                        }
                        build_flags.push(arg.clone())
                    } else {
                        program = arg.to_string();
                        seen_pkg = true;
                    }
                }

                let config: serde_json::Value = serde_json::to_value(DelveLaunchRequest {
                    cwd: build_config.cwd.clone(),
                    env: build_config.env.clone(),
                    request: "launch".to_string(),
                    mode: "debug".to_string(),
                    program,
                    args,
                    build_flags,
                })
                .unwrap();

                Some(DebugScenario {
                    label: resolved_label.to_string().into(),
                    adapter: adapter.0.clone(),
                    build: None,
                    config,
                    tcp_connection: None,
                })
            }
            _ => None,
        }
    }

    async fn run(
        &self,
        _build_config: SpawnInTerminal,
        _executor: BackgroundExecutor,
    ) -> Result<DebugRequest> {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use task::{HideStrategy, RevealStrategy, RevealTarget, Shell, TaskTemplate};

    #[gpui::test]
    async fn test_create_scenario_for_go_build(_: &mut TestAppContext) {
        let locator = GoLocator;
        let task = TaskTemplate {
            label: "go build".into(),
            command: "go".into(),
            args: vec!["build".into(), ".".into()],
            env: Default::default(),
            cwd: Some("${ZED_WORKTREE_ROOT}".into()),
            use_new_terminal: false,
            allow_concurrent_runs: false,
            reveal: RevealStrategy::Always,
            reveal_target: RevealTarget::Dock,
            hide: HideStrategy::Never,
            shell: Shell::System,
            tags: vec![],
            show_summary: true,
            show_command: true,
        };

        let scenario = locator
            .create_scenario(&task, "test label", &DebugAdapterName("Delve".into()))
            .await;

        assert!(scenario.is_none());
    }

    #[gpui::test]
    async fn test_skip_non_go_commands_with_non_delve_adapter(_: &mut TestAppContext) {
        let locator = GoLocator;
        let task = TaskTemplate {
            label: "cargo build".into(),
            command: "cargo".into(),
            args: vec!["build".into()],
            env: Default::default(),
            cwd: Some("${ZED_WORKTREE_ROOT}".into()),
            use_new_terminal: false,
            allow_concurrent_runs: false,
            reveal: RevealStrategy::Always,
            reveal_target: RevealTarget::Dock,
            hide: HideStrategy::Never,
            shell: Shell::System,
            tags: vec![],
            show_summary: true,
            show_command: true,
        };

        let scenario = locator
            .create_scenario(
                &task,
                "test label",
                &DebugAdapterName("SomeOtherAdapter".into()),
            )
            .await;
        assert!(scenario.is_none());

        let scenario = locator
            .create_scenario(&task, "test label", &DebugAdapterName("Delve".into()))
            .await;
        assert!(scenario.is_none());
    }
    #[gpui::test]
    async fn test_go_locator_run(_: &mut TestAppContext) {
        let locator = GoLocator;
        let delve = DebugAdapterName("Delve".into());

        let task = TaskTemplate {
            label: "go run with flags".into(),
            command: "go".into(),
            args: vec![
                "run".to_string(),
                "-race".to_string(),
                "-ldflags".to_string(),
                "-X main.version=1.0".to_string(),
                "./cmd/myapp".to_string(),
                "--config".to_string(),
                "production.yaml".to_string(),
                "--verbose".to_string(),
            ],
            env: {
                let mut env = HashMap::default();
                env.insert("GO_ENV".to_string(), "production".to_string());
                env
            },
            cwd: Some("/project/root".into()),
            ..Default::default()
        };

        let scenario = locator
            .create_scenario(&task, "test run label", &delve)
            .await
            .unwrap();

        let config: DelveLaunchRequest = serde_json::from_value(scenario.config).unwrap();

        assert_eq!(
            config,
            DelveLaunchRequest {
                request: "launch".to_string(),
                mode: "debug".to_string(),
                program: "./cmd/myapp".to_string(),
                build_flags: vec![
                    "-race".to_string(),
                    "-ldflags".to_string(),
                    "-X main.version=1.0".to_string()
                ],
                args: vec![
                    "--config".to_string(),
                    "production.yaml".to_string(),
                    "--verbose".to_string(),
                ],
                env: {
                    let mut env = HashMap::default();
                    env.insert("GO_ENV".to_string(), "production".to_string());
                    env
                },
                cwd: Some("/project/root".to_string()),
            }
        );
    }

    #[gpui::test]
    async fn test_go_locator_test(_: &mut TestAppContext) {
        let locator = GoLocator;
        let delve = DebugAdapterName("Delve".into());

        // Test with tags and run flag
        let task_with_tags = TaskTemplate {
            label: "test".into(),
            command: "go".into(),
            args: vec![
                "test".to_string(),
                "-tags".to_string(),
                "integration,unit".to_string(),
                "-run".to_string(),
                "Foo".to_string(),
                ".".to_string(),
            ],
            ..Default::default()
        };
        let result = locator
            .create_scenario(&task_with_tags, "", &delve)
            .await
            .unwrap();

        let config: DelveLaunchRequest = serde_json::from_value(result.config).unwrap();

        assert_eq!(
            config,
            DelveLaunchRequest {
                request: "launch".to_string(),
                mode: "test".to_string(),
                program: ".".to_string(),
                build_flags: vec!["-tags".to_string(), "integration,unit".to_string(),],
                args: vec![
                    "-test.run".to_string(),
                    "Foo".to_string(),
                    "-test.v".to_string()
                ],
                env: HashMap::default(),
                cwd: None,
            }
        );
    }

    #[gpui::test]
    async fn test_skip_unsupported_go_commands(_: &mut TestAppContext) {
        let locator = GoLocator;
        let task = TaskTemplate {
            label: "go clean".into(),
            command: "go".into(),
            args: vec!["clean".into()],
            env: Default::default(),
            cwd: Some("${ZED_WORKTREE_ROOT}".into()),
            use_new_terminal: false,
            allow_concurrent_runs: false,
            reveal: RevealStrategy::Always,
            reveal_target: RevealTarget::Dock,
            hide: HideStrategy::Never,
            shell: Shell::System,
            tags: vec![],
            show_summary: true,
            show_command: true,
        };

        let scenario = locator
            .create_scenario(&task, "test label", &DebugAdapterName("Delve".into()))
            .await;
        assert!(scenario.is_none());
    }
}
