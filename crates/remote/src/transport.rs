use crate::{
    json_log::LogRecord,
    protocol::{MESSAGE_LEN_SIZE, message_len_from_buffer, read_message_with_len, write_message},
};
use anyhow::{Context as _, Result};
use futures::{
    AsyncReadExt as _, FutureExt as _, StreamExt as _,
    channel::mpsc::{Sender, UnboundedReceiver, UnboundedSender},
};
use gpui::{AppContext as _, AsyncApp, Task};
use rpc::proto::Envelope;
use smol::process::Child;

pub mod ssh;
pub mod wsl;

fn handle_rpc_messages_over_child_process_stdio(
    mut ssh_proxy_process: Child,
    incoming_tx: UnboundedSender<Envelope>,
    mut outgoing_rx: UnboundedReceiver<Envelope>,
    mut connection_activity_tx: Sender<()>,
    cx: &AsyncApp,
) -> Task<Result<i32>> {
    let mut child_stderr = ssh_proxy_process.stderr.take().unwrap();
    let mut child_stdout = ssh_proxy_process.stdout.take().unwrap();
    let mut child_stdin = ssh_proxy_process.stdin.take().unwrap();

    let mut stdin_buffer = Vec::new();
    let mut stdout_buffer = Vec::new();
    let mut stderr_buffer = Vec::new();
    let mut stderr_offset = 0;

    let stdin_task = cx.background_spawn(async move {
        while let Some(outgoing) = outgoing_rx.next().await {
            write_message(&mut child_stdin, &mut stdin_buffer, outgoing).await?;
        }
        anyhow::Ok(())
    });

    let stdout_task = cx.background_spawn({
        let mut connection_activity_tx = connection_activity_tx.clone();
        async move {
            loop {
                stdout_buffer.resize(MESSAGE_LEN_SIZE, 0);
                let len = child_stdout.read(&mut stdout_buffer).await?;

                if len == 0 {
                    return anyhow::Ok(());
                }

                if len < MESSAGE_LEN_SIZE {
                    child_stdout.read_exact(&mut stdout_buffer[len..]).await?;
                }

                let message_len = message_len_from_buffer(&stdout_buffer);
                let envelope =
                    read_message_with_len(&mut child_stdout, &mut stdout_buffer, message_len)
                        .await?;
                connection_activity_tx.try_send(()).ok();
                incoming_tx.unbounded_send(envelope).ok();
            }
        }
    });

    let stderr_task: Task<anyhow::Result<()>> = cx.background_spawn(async move {
        loop {
            stderr_buffer.resize(stderr_offset + 1024, 0);

            let len = child_stderr
                .read(&mut stderr_buffer[stderr_offset..])
                .await?;
            if len == 0 {
                return anyhow::Ok(());
            }

            stderr_offset += len;
            let mut start_ix = 0;
            while let Some(ix) = stderr_buffer[start_ix..stderr_offset]
                .iter()
                .position(|b| b == &b'\n')
            {
                let line_ix = start_ix + ix;
                let content = &stderr_buffer[start_ix..line_ix];
                start_ix = line_ix + 1;
                if let Ok(record) = serde_json::from_slice::<LogRecord>(content) {
                    record.log(log::logger())
                } else {
                    eprintln!("(remote) {}", String::from_utf8_lossy(content));
                }
            }
            stderr_buffer.drain(0..start_ix);
            stderr_offset -= start_ix;

            connection_activity_tx.try_send(()).ok();
        }
    });

    cx.background_spawn(async move {
        let result = futures::select! {
            result = stdin_task.fuse() => {
                result.context("stdin")
            }
            result = stdout_task.fuse() => {
                result.context("stdout")
            }
            result = stderr_task.fuse() => {
                result.context("stderr")
            }
        };

        let status = ssh_proxy_process.status().await?.code().unwrap_or(1);
        match result {
            Ok(_) => Ok(status),
            Err(error) => Err(error),
        }
    })
}

#[cfg(debug_assertions)]
async fn build_remote_server_from_source(
    platform: &crate::RemotePlatform,
    delegate: &dyn crate::RemoteClientDelegate,
    cx: &mut AsyncApp,
) -> Result<Option<std::path::PathBuf>> {
    use std::path::Path;

    let Some(build_remote_server) = std::env::var("ZED_BUILD_REMOTE_SERVER").ok() else {
        return Ok(None);
    };

    use smol::process::{Command, Stdio};
    use std::env::VarError;

    async fn run_cmd(command: &mut Command) -> Result<()> {
        let output = command
            .kill_on_drop(true)
            .stderr(Stdio::inherit())
            .output()
            .await?;
        anyhow::ensure!(
            output.status.success(),
            "Failed to run command: {command:?}"
        );
        Ok(())
    }

    let use_musl = !build_remote_server.contains("nomusl");
    let triple = format!(
        "{}-{}",
        platform.arch,
        match platform.os {
            "linux" =>
                if use_musl {
                    "unknown-linux-musl"
                } else {
                    "unknown-linux-gnu"
                },
            "macos" => "apple-darwin",
            _ => anyhow::bail!("can't cross compile for: {:?}", platform),
        }
    );
    let mut rust_flags = match std::env::var("RUSTFLAGS") {
        Ok(val) => val,
        Err(VarError::NotPresent) => String::new(),
        Err(e) => {
            log::error!("Failed to get env var `RUSTFLAGS` value: {e}");
            String::new()
        }
    };
    if platform.os == "linux" && use_musl {
        rust_flags.push_str(" -C target-feature=+crt-static");
    }
    if build_remote_server.contains("mold") {
        rust_flags.push_str(" -C link-arg=-fuse-ld=mold");
    }

    if platform.arch == std::env::consts::ARCH && platform.os == std::env::consts::OS {
        delegate.set_status(Some("Building remote server binary from source"), cx);
        log::info!("building remote server binary from source");
        run_cmd(
            Command::new("cargo")
                .args([
                    "build",
                    "--package",
                    "remote_server",
                    "--features",
                    "debug-embed",
                    "--target-dir",
                    "target/remote_server",
                    "--target",
                    &triple,
                ])
                .env("RUSTFLAGS", &rust_flags),
        )
        .await?;
    } else if build_remote_server.contains("cross") {
        use util::paths::SanitizedPath;

        delegate.set_status(Some("Installing cross.rs for cross-compilation"), cx);
        log::info!("installing cross");
        run_cmd(Command::new("cargo").args([
            "install",
            "cross",
            "--git",
            "https://github.com/cross-rs/cross",
        ]))
        .await?;

        delegate.set_status(
            Some(&format!(
                "Building remote server binary from source for {} with Docker",
                &triple
            )),
            cx,
        );
        log::info!("building remote server binary from source for {}", &triple);

        let src = SanitizedPath::new(&smol::fs::canonicalize("target").await?).to_string();

        run_cmd(
            Command::new("cross")
                .args([
                    "build",
                    "--package",
                    "remote_server",
                    "--features",
                    "debug-embed",
                    "--target-dir",
                    "target/remote_server",
                    "--target",
                    &triple,
                ])
                .env(
                    "CROSS_CONTAINER_OPTS",
                    format!("--mount type=bind,src={src},dst=/app/target"),
                )
                .env("RUSTFLAGS", &rust_flags),
        )
        .await?;
    } else {
        let which = cx
            .background_spawn(async move { which::which("zig") })
            .await;

        if which.is_err() {
            #[cfg(not(target_os = "windows"))]
            {
                anyhow::bail!(
                    "zig not found on $PATH, install zig (see https://ziglang.org/learn/getting-started or use zigup) or pass ZED_BUILD_REMOTE_SERVER=cross to use cross"
                )
            }
            #[cfg(target_os = "windows")]
            {
                anyhow::bail!(
                    "zig not found on $PATH, install zig (use `winget install -e --id zig.zig` or see https://ziglang.org/learn/getting-started or use zigup) or pass ZED_BUILD_REMOTE_SERVER=cross to use cross"
                )
            }
        }

        delegate.set_status(Some("Adding rustup target for cross-compilation"), cx);
        log::info!("adding rustup target");
        run_cmd(Command::new("rustup").args(["target", "add"]).arg(&triple)).await?;

        delegate.set_status(Some("Installing cargo-zigbuild for cross-compilation"), cx);
        log::info!("installing cargo-zigbuild");
        run_cmd(Command::new("cargo").args(["install", "--locked", "cargo-zigbuild"])).await?;

        delegate.set_status(
            Some(&format!(
                "Building remote binary from source for {triple} with Zig"
            )),
            cx,
        );
        log::info!("building remote binary from source for {triple} with Zig");
        run_cmd(
            Command::new("cargo")
                .args([
                    "zigbuild",
                    "--package",
                    "remote_server",
                    "--features",
                    "debug-embed",
                    "--target-dir",
                    "target/remote_server",
                    "--target",
                    &triple,
                ])
                .env("RUSTFLAGS", &rust_flags),
        )
        .await?;
    };
    let bin_path = Path::new("target")
        .join("remote_server")
        .join(&triple)
        .join("debug")
        .join("remote_server");

    let path = if !build_remote_server.contains("nocompress") {
        delegate.set_status(Some("Compressing binary"), cx);

        #[cfg(not(target_os = "windows"))]
        {
            run_cmd(Command::new("gzip").args(["-f", &bin_path.to_string_lossy()])).await?;
        }

        #[cfg(target_os = "windows")]
        {
            // On Windows, we use 7z to compress the binary
            let seven_zip = which::which("7z.exe").context("7z.exe not found on $PATH, install it (e.g. with `winget install -e --id 7zip.7zip`) or, if you don't want this behaviour, set $env:ZED_BUILD_REMOTE_SERVER=\"nocompress\"")?;
            let gz_path = format!("target/remote_server/{}/debug/remote_server.gz", triple);
            if smol::fs::metadata(&gz_path).await.is_ok() {
                smol::fs::remove_file(&gz_path).await?;
            }
            run_cmd(Command::new(seven_zip).args([
                "a",
                "-tgzip",
                &gz_path,
                &bin_path.to_string_lossy(),
            ]))
            .await?;
        }

        let mut archive_path = bin_path;
        archive_path.set_extension("gz");
        std::env::current_dir()?.join(archive_path)
    } else {
        bin_path
    };

    Ok(Some(path))
}
