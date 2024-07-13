use futures::channel::mpsc;
use gpui::{AppContext, Context as _};
use project::WorktreeSettings;
use remote::{
    protocol::{read_message, write_message},
    SshSession,
};
use remote_server::headless_project::HeadlessProject;
use settings::{Settings as _, SettingsStore};
use smol::{io::AsyncWriteExt, stream::StreamExt as _, Async};
use std::{env, io, mem};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "remote=trace");

    env_logger::init();

    gpui::App::new().headless().run(move |cx| {
        init(cx);

        let (incoming_tx, incoming_rx) = mpsc::unbounded();
        let (outgoing_tx, mut outgoing_rx) = mpsc::unbounded();

        let mut stdin = Async::new(io::stdin()).unwrap();
        let mut stdout = Async::new(io::stdout()).unwrap();

        let session = SshSession::server(incoming_rx, outgoing_tx, cx);
        let project = cx.new_model(|cx| HeadlessProject::new(session.clone(), cx));

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
                            break;
                        }
                    };
                    incoming_tx.unbounded_send(message).ok();
                }
            })
            .detach();

        mem::forget(project);
    });
}

pub fn init(cx: &mut AppContext) {
    cx.set_global(SettingsStore::default());
    WorktreeSettings::register(cx);
}
