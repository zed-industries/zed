use std::{os::unix::fs::PermissionsExt, path::PathBuf};

use anyhow::{anyhow, Context, Result};
use gpui::AsyncApp;
use smol::{
    io::{AsyncWriteExt as _, BufReader},
    net::unix::UnixListener,
};
use ui::{App, Window};
use util::{maybe, ResultExt as _};
use workspace::Workspace;

pub fn get_askpass_dir() -> PathBuf {
    // TODO: bundle this script instead of creating it
    let temp_dir = tempfile::Builder::new()
        .prefix("zed-git-askpass-session")
        .tempdir()
        .unwrap();

    // Create a domain socket listener to handle requests from the askpass program.
    let askpass_socket = temp_dir.path().join("git_askpass.sock");

    // Create an askpass script that communicates back to this process.
    let askpass_script = format!(
        "{shebang}\n{print_args} | {nc} -U {askpass_socket} 2> /dev/null \n",
        // on macOS `brew install netcat` provides the GNU netcat implementation
        // which does not support -U.
        nc = if cfg!(target_os = "macos") {
            "/usr/bin/nc"
        } else {
            "nc"
        },
        askpass_socket = askpass_socket.display(),
        print_args = "printf '%s\\0' \"$@\"",
        shebang = "#!/bin/sh",
    );
    let askpass_script_path = temp_dir.path().join("askpass.sh");
    std::fs::write(&askpass_script_path, &askpass_script).unwrap();
    std::fs::set_permissions(&askpass_script_path, std::fs::Permissions::from_mode(0o755)).unwrap();

    PathBuf::from(askpass_script)
}

pub fn setup_git_askpass(askpasss_file: PathBuf, cx: &mut App) {
    maybe!({
        anyhow::ensure!(
            which::which("nc").is_ok(),
            "Cannot find `nc` command (netcat), which is required to connect over SSH."
        );

        // TODO: REMOVE THIS ONCE WE HAVE A WAY OF BUNDLING AN ASKPASS SCRIPT
        let askpass_socket = askpasss_file.parent().unwrap().join("git_askpass.sock");

        let listener =
            UnixListener::bind(&askpass_socket).context("failed to create askpass socket")?;

        cx.spawn({
            |mut cx| async move {
                while let Ok((mut stream, _)) = listener.accept().await {
                    let mut buffer = Vec::new();
                    let mut reader = BufReader::new(&mut stream);
                    if smol::io::AsyncBufReadExt::read_until(&mut reader, b'\0', &mut buffer)
                        .await
                        .is_err()
                    {
                        buffer.clear();
                    }
                    let password_prompt = String::from_utf8_lossy(&buffer);
                    if let Some(Ok(password)) = ask_password(&password_prompt, &mut cx)
                        .await
                        .context("failed to get ssh password")
                        .log_err()
                    {
                        stream.write_all(password.as_bytes()).await.log_err();
                    } else {
                        stream.write("\n".as_bytes()).await.log_err();
                    }

                    stream.flush().await.log_err();
                    stream.close().await.log_err();
                }
            }
        })
        .detach();

        Ok(())
    })
    .log_err();
}

async fn ask_password(prompt: &str, cx: &mut AsyncApp) -> Option<Result<String>> {
    let mut workspace = get_workspace(cx, |window| window.is_window_active());
    if workspace.is_none() {
        workspace = get_workspace(cx, |_| true);
    }

    let Some(workspace) = workspace else {
        return None;
    };

    // DO THINGS WITH THE WORKSPACE
    // pop the askpass modal, get the output out of a oneshot, and we're good to go
    None
}

fn get_workspace(
    cx: &mut AsyncApp,
    predicate: impl Fn(&mut Window) -> bool,
) -> Option<gpui::Entity<Workspace>> {
    let workspace = cx
        .update(|cx| {
            for window in cx.windows() {
                let workspace = window
                    .update(cx, |view, window, _| {
                        if predicate(window) {
                            if let Ok(workspace) = view.downcast::<Workspace>() {
                                return Some(workspace);
                            }
                        }
                        return None;
                    })
                    .ok()
                    .flatten();

                if let Some(workspace) = workspace {
                    return Some(workspace);
                } else {
                    continue;
                }
            }

            None
        })
        .ok()?;

    workspace
}
