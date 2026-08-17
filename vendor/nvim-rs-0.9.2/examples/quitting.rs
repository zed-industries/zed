//! Quitting. See src/examples/quitting.rs for documentation
use nvim_rs::{create::tokio as create, rpc::handler::Dummy as DummyHandler};

use std::error::Error;

use tokio::process::Command;

const NVIMPATH: &str = "neovim/build/bin/nvim";

#[tokio::main]
async fn main() {
  let handler = DummyHandler::new();

  let (nvim, _io_handle, _child) = create::new_child_cmd(
    Command::new(NVIMPATH)
      .args(&["-u", "NONE", "--embed", "--headless"])
      .env("NVIM_LOG_FILE", "nvimlog"),
    handler,
  )
  .await
  .unwrap();

  let chan = nvim.get_api_info().await.unwrap()[0].as_i64().unwrap();
  let close = format!("call chanclose({})", chan);

  if let Err(e) = nvim.command(&close).await {
    eprintln!("Error in last command: {}", e);
    eprintln!("Caused by : {:?}", e.as_ref().source());

    if e.is_channel_closed() {
      eprintln!("Channel closed, quitting!");
    } else {
      eprintln!("Channel was not closed, no idea what happened!");
    }
  }
}
