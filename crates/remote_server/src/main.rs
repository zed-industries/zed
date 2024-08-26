#![cfg_attr(target_os = "windows", allow(unused, dead_code))]

use fs::RealFs;
use futures::channel::mpsc;
use gpui::Context as _;
use remote::{
    json_log::LogRecord,
    protocol::{read_message, write_message},
    SshSession,
};
use remote_server::HeadlessProject;
use smol::{io::AsyncWriteExt, stream::StreamExt as _, Async};
use std::{
    env,
    io::{self, Write},
    mem, process,
    sync::Arc,
};

#[cfg(windows)]
fn main() {
    unimplemented!()
}

#[cfg(not(windows))]
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::builder()
        .format(|buf, record| {
            serde_json::to_writer(&mut *buf, &LogRecord::new(&record))?;
            buf.write_all(b"\n")?;
            Ok(())
        })
        .init();

    let subcommand = std::env::args().nth(1);
    match subcommand.as_deref() {
        Some("run") => {}
        Some("version") => {
            println!("{}", env!("ZED_PKG_VERSION"));
            return;
        }
        _ => {
            eprintln!("usage: remote <run|version>");
            process::exit(1);
        }
    }

    gpui::App::headless().run(move |cx| {
        HeadlessProject::init(cx);

        let (incoming_tx, incoming_rx) = mpsc::unbounded();
        let (outgoing_tx, mut outgoing_rx) = mpsc::unbounded();

        let mut stdin = Async::new(io::stdin()).unwrap();
        let mut stdout = Async::new(io::stdout()).unwrap();

        let session = SshSession::server(incoming_rx, outgoing_tx, cx);
        let project = cx.new_model(|cx| {
            HeadlessProject::new(
                session.clone(),
                Arc::new(RealFs::new(Default::default(), None)),
                cx,
            )
        });

        cx.background_executor()
            .spawn(async move {
                let mut output_buffer = Vec::new();
                while let Some(message) = outgoing_rx.next().await {
                    write_message(&mut stdout, &mut output_buffer, message).await?;
                    stdout.flush().await?;
                }
                anyhow::Ok(())
            })
            .detach();

        cx.background_executor()
            .spawn(async move {
                let mut input_buffer = Vec::new();
                loop {
                    let message = match read_message(&mut stdin, &mut input_buffer).await {
                        Ok(message) => message,
                        Err(error) => {
                            log::warn!("error reading message: {:?}", error);
                            process::exit(0);
                        }
                    };
                    incoming_tx.unbounded_send(message).ok();
                }
            })
            .detach();

        mem::forget(project);
    });
}
