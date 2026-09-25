use super::debug_log::{AcpDebugLog, AcpDebugMessageDirection};
use anyhow::{Context as _, Result};
use futures::{
    AsyncBufReadExt as _, AsyncWriteExt as _, FutureExt as _, Sink, StreamExt as _,
    future::BoxFuture, io::BufReader, stream::BoxStream,
};
use gpui::{AsyncApp, Entity};
use project::{Project, agent_server_store::AgentServerCommand};
use remote::remote_client::Interactive;
use std::{io, pin::Pin, process::Stdio};
use task::{Shell, ShellBuilder};
use util::{ResultExt as _, process::Child};

pub(super) struct StdioProcess {
    pub child: Child,
    pub incoming: BoxStream<'static, io::Result<String>>,
    pub outgoing: Pin<Box<dyn Sink<String, Error = io::Error> + Send>>,
    // The connection owns when this starts and when it is cancelled.
    pub stderr: BoxFuture<'static, Result<()>>,
    pub debug_log: AcpDebugLog,
}

pub(super) fn spawn_stdio(
    project: &Entity<Project>,
    command: AgentServerCommand,
    cx: &AsyncApp,
) -> Result<StdioProcess> {
    let root_dir = project.read_with(cx, |project, cx| {
        project
            .default_path_list(cx)
            .ordered_paths()
            .next()
            .cloned()
    });
    let (path, arguments, environment) = project
        .read_with(cx, |project, cx| {
            project.remote_client().and_then(|client| {
                let template = client
                    .read(cx)
                    .build_command(
                        Some(command.path.display().to_string()),
                        &command.args,
                        &command.env.clone().into_iter().flatten().collect(),
                        root_dir.as_ref().map(|path| path.display().to_string()),
                        None,
                        Interactive::No,
                    )
                    .log_err()?;
                Some((template.program, template.args, template.env))
            })
        })
        .unwrap_or_else(|| {
            (
                command.path.display().to_string(),
                command.args,
                command.env.unwrap_or_default(),
            )
        });

    let builder = ShellBuilder::new(&Shell::System, cfg!(windows)).non_interactive();
    let mut child = builder.build_std_command(Some(path.clone()), &arguments);
    child.envs(environment);
    if let Some(cwd) = project.read_with(cx, |project, _cx| {
        if project.is_local() {
            root_dir.as_ref()
        } else {
            None
        }
    }) {
        child.current_dir(cwd);
    }
    let mut child = Child::spawn(child, Stdio::piped(), Stdio::piped(), Stdio::piped())?;

    let stdout = child.stdout.take().context("Failed to take stdout")?;
    let stdin = child.stdin.take().context("Failed to take stdin")?;
    let stderr = child.stderr.take().context("Failed to take stderr")?;
    log::debug!(
        "Spawning external agent server: {:?}, {:?}",
        path,
        arguments
    );
    log::trace!("Spawned (pid: {})", child.id());

    let debug_log = AcpDebugLog::default();
    let incoming = BufReader::new(stdout)
        .lines()
        .inspect({
            let debug_log = debug_log.clone();
            move |result| match result {
                Ok(line) => debug_log.record_line(AcpDebugMessageDirection::Incoming, line),
                Err(error) => {
                    log::warn!("ACP transport read error: {error}");
                }
            }
        })
        .boxed();

    let outgoing = Box::pin(futures::sink::unfold(
        (Box::pin(stdin), debug_log.clone()),
        async move |(mut writer, debug_log), line: String| {
            debug_log.record_line(AcpDebugMessageDirection::Outgoing, &line);
            let mut bytes = line.into_bytes();
            bytes.push(b'\n');
            writer.write_all(&bytes).await?;
            Ok::<_, io::Error>((writer, debug_log))
        },
    ));

    let stderr = {
        let debug_log = debug_log.clone();
        async move {
            let mut stderr = BufReader::new(stderr);
            let mut line = String::new();
            while let Ok(bytes_read) = stderr.read_line(&mut line).await
                && bytes_read > 0
            {
                let trimmed = line.trim_end_matches(['\n', '\r']);
                log::warn!("agent stderr: {trimmed}");
                debug_log.record_line(AcpDebugMessageDirection::Stderr, trimmed);
                line.clear();
            }
            Ok(())
        }
        .boxed()
    };

    Ok(StdioProcess {
        child,
        incoming,
        outgoing,
        stderr,
        debug_log,
    })
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use crate::acp::AcpDebugMessageContent;
    use collections::HashMap;
    use futures::SinkExt as _;
    use gpui::TestAppContext;

    #[gpui::test]
    async fn repeated_spawns_preserve_command_and_stdio(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
        });
        cx.executor().allow_parking();
        let directory = tempfile::tempdir().expect("create working directory");
        let expected_directory = directory
            .path()
            .canonicalize()
            .expect("resolve working directory");
        let project = Project::example([directory.path()], &mut cx.to_async()).await;
        let command = AgentServerCommand {
            path: "/bin/sh".into(),
            args: vec![
                "-c".into(),
                r#"printf '%s\n' "$ACP_TRANSPORT_TEST"; pwd -P; printf 'transport stderr\r\n' >&2; IFS= read -r line && printf '%s\n' "$line""#.into(),
            ],
            env: Some(HashMap::from_iter([(
                "ACP_TRANSPORT_TEST".into(),
                "value with spaces and $literal".into(),
            )])),
        };

        for attempt in 0..2 {
            let StdioProcess {
                mut child,
                mut incoming,
                mut outgoing,
                stderr,
                debug_log,
            } = spawn_stdio(&project, command.clone(), &cx.to_async())
                .expect("spawn fresh transport");
            let stderr_task = cx.background_executor.spawn(stderr);
            let params = serde_json::json!({ "attempt": attempt });
            let request = serde_json::json!({
                "jsonrpc": "2.0",
                "id": attempt,
                "method": "transport/test",
                "params": params,
            })
            .to_string();
            let exchange = async {
                outgoing.send(request.clone()).await.expect("write request");
                drop(outgoing);

                let mut output = Vec::new();
                while let Some(line) = incoming.next().await {
                    output.push(line.expect("read output"));
                }
                assert_eq!(
                    output,
                    [
                        "value with spaces and $literal".to_string(),
                        expected_directory.to_string_lossy().into_owned(),
                        request,
                    ]
                );
                assert!(child.status().await.expect("wait for process").success());
                stderr_task.await.expect("read stderr");
            }
            .fuse();
            let timeout = cx
                .background_executor
                .timer(std::time::Duration::from_secs(5))
                .fuse();
            futures::pin_mut!(exchange, timeout);
            futures::select! {
                _ = exchange => {}
                _ = timeout => panic!("timed out waiting for stdio exchange on attempt {attempt}"),
            }

            let (messages, _receiver) = debug_log.subscribe();
            assert_eq!(messages.len(), 3, "each process must have its own log");
            for direction in [
                AcpDebugMessageDirection::Outgoing,
                AcpDebugMessageDirection::Incoming,
            ] {
                assert!(messages.iter().any(|message| {
                    message.direction == direction
                        && matches!(
                            &message.message,
                            AcpDebugMessageContent::Request { method, params: Some(recorded), .. }
                                if method.as_ref() == "transport/test" && recorded == &params
                        )
                }));
            }
            assert!(messages.iter().any(|message| {
                message.direction == AcpDebugMessageDirection::Stderr
                    && matches!(
                        &message.message,
                        AcpDebugMessageContent::Stderr { line } if line.as_ref() == "transport stderr"
                    )
            }));
        }
    }
}
