use anyhow::Result;
use async_trait::async_trait;
use collections::HashMap;
use dap::{DapLocator, DebugRequest, adapters::DebugAdapterName};
use gpui::SharedString;
use serde::{Deserialize, Serialize};
use task::{DebugScenario, SpawnInTerminal, TaskTemplate};

pub(crate) struct GoLocator;

#[derive(Serialize, Deserialize, Debug)]
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

    fn create_scenario(
        &self,
        build_config: &TaskTemplate,
        resolved_label: &str,
        adapter: DebugAdapterName,
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

                for arg in build_config.args.iter().skip(1) {
                    if all_args_are_test || next_arg_is_test {
                        // HACK: tasks assume that they are run in a shell context,
                        // so the -run regex has escaped specials. Delve correctly
                        // handles escaping, so we undo that here.
                        if arg.starts_with("\\^") && arg.ends_with("\\$") {
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

                let config: serde_json::Value = serde_json::to_value(DelveLaunchRequest {
                    request: "launch".to_string(),
                    mode: "test".to_string(),
                    program,
                    args: args,
                    build_flags,
                    cwd: build_config.cwd.clone(),
                    env: build_config.env.clone(),
                })
                .unwrap();

                Some(DebugScenario {
                    label: resolved_label.to_string().into(),
                    adapter: adapter.0,
                    build: None,
                    config: config,
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
                        if let Some(has_arg) = is_build_flag(arg) {
                            next_arg_is_build = has_arg;
                        }
                        args.push(arg.clone())
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
                    args: args,
                    build_flags,
                })
                .unwrap();

                Some(DebugScenario {
                    label: resolved_label.to_string().into(),
                    adapter: adapter.0,
                    build: None,
                    config,
                    tcp_connection: None,
                })
            }
            _ => None,
        }
    }

    async fn run(&self, build_config: SpawnInTerminal) -> Result<DebugRequest> {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use task::{HideStrategy, RevealStrategy, RevealTarget, Shell, TaskId, TaskTemplate};

    #[test]
    fn test_create_scenario_for_go_run() {
        let locator = GoLocator;
        let task = TaskTemplate {
            label: "go run main.go".into(),
            command: "go".into(),
            args: vec!["run".into(), "main.go".into()],
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

        let scenario =
            locator.create_scenario(&task, "test label", DebugAdapterName("Delve".into()));

        assert!(scenario.is_some());
        let scenario = scenario.unwrap();
        assert_eq!(scenario.adapter, "Delve");
        assert_eq!(scenario.label, "test label");
        assert!(scenario.build.is_some());

        if let Some(BuildTaskDefinition::Template { task_template, .. }) = &scenario.build {
            assert_eq!(task_template.command, "go");
            assert!(task_template.args.contains(&"build".into()));
            assert!(
                task_template
                    .args
                    .contains(&"-gcflags \"all=-N -l\"".into())
            );
            assert!(task_template.args.contains(&"main.go".into()));
        } else {
            panic!("Expected BuildTaskDefinition::Template");
        }

        assert!(
            scenario.config.is_null(),
            "Initial config should be null to ensure it's invalid"
        );
    }

    #[test]
    fn test_create_scenario_for_go_build() {
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

        let scenario =
            locator.create_scenario(&task, "test label", DebugAdapterName("Delve".into()));

        assert!(scenario.is_none());
    }

    #[test]
    fn test_skip_non_go_commands_with_non_delve_adapter() {
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

        let scenario = locator.create_scenario(
            &task,
            "test label",
            DebugAdapterName("SomeOtherAdapter".into()),
        );
        assert!(scenario.is_none());

        let scenario =
            locator.create_scenario(&task, "test label", DebugAdapterName("Delve".into()));
        assert!(scenario.is_none());
    }

    #[test]
    fn test_create_scenario_for_go_test() {
        let locator = GoLocator;
        let task = TaskTemplate {
            label: "go test".into(),
            command: "go".into(),
            args: vec!["test".into(), ".".into()],
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

        let scenario =
            locator.create_scenario(&task, "test label", DebugAdapterName("Delve".into()));

        assert!(scenario.is_some());
        let scenario = scenario.unwrap();
        assert_eq!(scenario.adapter, "Delve");
        assert_eq!(scenario.label, "test label");
        assert!(scenario.build.is_some());

        if let Some(BuildTaskDefinition::Template { task_template, .. }) = &scenario.build {
            assert_eq!(task_template.command, "go");
            assert!(task_template.args.contains(&"test".into()));
            assert!(task_template.args.contains(&"-c".into()));
            assert!(
                task_template
                    .args
                    .contains(&"-gcflags \"all=-N -l\"".into())
            );
            assert!(task_template.args.contains(&"-o".into()));
            assert!(
                task_template
                    .args
                    .iter()
                    .any(|arg| arg.starts_with("__debug_"))
            );
        } else {
            panic!("Expected BuildTaskDefinition::Template");
        }

        assert!(
            scenario.config.is_null(),
            "Initial config should be null to ensure it's invalid"
        );
    }

    #[test]
    fn test_create_scenario_for_go_test_with_tags() {
        let locator = GoLocator;
        let task = TaskTemplate {
            label: "go test with tags".into(),
            command: "go".into(),
            args: vec![
                "test".into(),
                "-tags".into(),
                "integration,e2e".into(),
                ".".into(),
            ],
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

        let scenario =
            locator.create_scenario(&task, "test label", DebugAdapterName("Delve".into()));

        assert!(scenario.is_some());
        let scenario = scenario.unwrap();

        if let Some(BuildTaskDefinition::Template { task_template, .. }) = &scenario.build {
            assert!(task_template.args.contains(&"test".into()));
            assert!(task_template.args.contains(&"-c".into()));
            assert!(task_template.args.contains(&"-tags".into()));
            assert!(task_template.args.contains(&"integration,e2e".into()));
            assert!(
                task_template
                    .args
                    .contains(&"-gcflags \"all=-N -l\"".into())
            );
        } else {
            panic!("Expected BuildTaskDefinition::Template");
        }
    }

    #[test]
    fn test_get_build_tags() {
        let locator = GoLocator;

        // Test with tags
        let task_with_tags = TaskTemplate {
            label: "test".into(),
            command: "go".into(),
            args: vec![
                "test".to_string(),
                "-tags".to_string(),
                "integration,unit".to_string(),
                ".".to_string(),
            ],
            env: FxHashMap::default(),
            cwd: None,
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
        let tags = locator.get_build_tags(&task_with_tags);
        assert_eq!(
            tags,
            vec!["-tags".to_string(), "integration,unit".to_string()]
        );

        // Test without tags
        let task_without_tags = TaskTemplate {
            label: "test".into(),
            command: "go".into(),
            args: vec!["test".to_string(), ".".to_string()],
            env: FxHashMap::default(),
            cwd: None,
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
        let tags = locator.get_build_tags(&task_without_tags);
        assert!(tags.is_empty());

        let task_multiple_tags = TaskTemplate {
            label: "test".into(),
            command: "go".into(),
            args: vec![
                "test".to_string(),
                "-tags".to_string(),
                "unit".to_string(),
                "-tags".to_string(),
                "integration".to_string(),
            ],
            env: FxHashMap::default(),
            cwd: None,
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
        let tags = locator.get_build_tags(&task_multiple_tags);
        assert_eq!(
            tags,
            vec![
                "-tags".to_string(),
                "unit".to_string(),
                "-tags".to_string(),
                "integration".to_string()
            ]
        );
    }

    #[test]
    fn test_create_scenario_for_go_test_with_cwd_binary() {
        let locator = GoLocator;

        let task = TaskTemplate {
            label: "go test".into(),
            command: "go".into(),
            args: vec!["test".into(), ".".into()],
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

        let scenario =
            locator.create_scenario(&task, "test label", DebugAdapterName("Delve".into()));

        assert!(scenario.is_some());
        let scenario = scenario.unwrap();

        if let Some(BuildTaskDefinition::Template { task_template, .. }) = &scenario.build {
            assert!(
                task_template
                    .args
                    .iter()
                    .any(|arg| arg.starts_with("__debug_"))
            );
        } else {
            panic!("Expected BuildTaskDefinition::Template");
        }
    }

    #[test]
    fn test_skip_unsupported_go_commands() {
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

        let scenario =
            locator.create_scenario(&task, "test label", DebugAdapterName("Delve".into()));
        assert!(scenario.is_none());
    }

    #[test]
    fn test_run_go_test_missing_binary_path() {
        let locator = GoLocator;
        let build_config = SpawnInTerminal {
            id: TaskId("test_task".to_string()),
            full_label: "go test".to_string(),
            label: "go test".to_string(),
            command: "go".into(),
            args: vec![
                "test".into(),
                "-c".into(),
                "-gcflags \"all=-N -l\"".into(),
                "-o".into(),
            ], // Missing the binary path after -o
            command_label: "go test -c -gcflags \"all=-N -l\" -o".to_string(),
            env: Default::default(),
            cwd: Some(PathBuf::from("/test/path")),
            use_new_terminal: false,
            allow_concurrent_runs: false,
            reveal: RevealStrategy::Always,
            reveal_target: RevealTarget::Dock,
            hide: HideStrategy::Never,
            shell: Shell::System,
            show_summary: true,
            show_command: true,
            show_rerun: true,
        };

        let result = futures::executor::block_on(locator.run(build_config));
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("can't locate debug binary")
        );
    }

    #[test]
    fn test_run_go_test_with_tags() {
        let locator = GoLocator;
        let build_config = SpawnInTerminal {
            id: TaskId("test_task".to_string()),
            full_label: "go test".to_string(),
            label: "go test".to_string(),
            command: "go".into(),
            args: vec![
                "test".into(),
                "-c".into(),
                "-tags".into(),
                "integration".into(),
                "-gcflags \"all=-N -l\"".into(),
                "-o".into(),
                "__debug_binary".into(),
            ],
            command_label: "go test -c -tags integration -gcflags \"all=-N -l\" -o __debug_binary"
                .to_string(),
            env: Default::default(),
            cwd: Some(PathBuf::from("/test/path")),
            use_new_terminal: false,
            allow_concurrent_runs: false,
            reveal: RevealStrategy::Always,
            reveal_target: RevealTarget::Dock,
            hide: HideStrategy::Never,
            shell: Shell::System,
            show_summary: true,
            show_command: true,
            show_rerun: true,
        };

        let result = futures::executor::block_on(locator.run(build_config));
        assert!(result.is_ok());

        if let Ok(DebugRequest::Launch(launch_request)) = result {
            assert!(launch_request.program.ends_with("__debug_binary"));
            assert_eq!(
                launch_request.args,
                vec!["-test.v".to_string(), "-test.run=${ZED_SYMBOL}".to_string()]
            );
        } else {
            panic!("Expected LaunchRequest");
        }
    }
}
