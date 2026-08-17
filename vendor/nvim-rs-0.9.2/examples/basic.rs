//! A basic example. Mainly for use in a test, but also shows off some basic
//! functionality.
use std::{env, error::Error, fs};

use async_trait::async_trait;

use rmpv::Value;

use tokio::fs::File as TokioFile;

use nvim_rs::{
  compat::tokio::Compat, create::tokio as create, rpc::IntoVal, Handler, Neovim,
};

#[derive(Clone)]
struct NeovimHandler {}

#[async_trait]
impl Handler for NeovimHandler {
  type Writer = Compat<TokioFile>;

  async fn handle_request(
    &self,
    name: String,
    _args: Vec<Value>,
    _neovim: Neovim<Compat<TokioFile>>,
  ) -> Result<Value, Value> {
    match name.as_ref() {
      "ping" => Ok(Value::from("pong")),
      _ => unimplemented!(),
    }
  }
}

#[tokio::main]
async fn main() {
  let handler: NeovimHandler = NeovimHandler {};
  let (nvim, io_handler) = create::new_parent(handler).await.unwrap();
  let curbuf = nvim.get_current_buf().await.unwrap();

  let mut envargs = env::args();
  let _ = envargs.next();
  let testfile = envargs.next().unwrap();

  fs::write(testfile, &format!("{:?}", curbuf.into_val())).unwrap();

  // Any error should probably be logged, as stderr is not visible to users.
  match io_handler.await {
    Err(joinerr) => eprintln!("Error joining IO loop: '{}'", joinerr),
    Ok(Err(err)) => {
      if !err.is_reader_error() {
        // One last try, since there wasn't an error with writing to the
        // stream
        nvim
          .err_writeln(&format!("Error: '{}'", err))
          .await
          .unwrap_or_else(|e| {
            // We could inspect this error to see what was happening, and
            // maybe retry, but at this point it's probably best
            // to assume the worst and print a friendly and
            // supportive message to our users
            eprintln!("Well, dang... '{}'", e);
          });
      }

      if !err.is_channel_closed() {
        // Closed channel usually means neovim quit itself, or this plugin was
        // told to quit by closing the channel, so it's not always an error
        // condition.
        eprintln!("Error: '{}'", err);

        let mut source = err.source();

        while let Some(e) = source {
          eprintln!("Caused by: '{}'", e);
          source = e.source();
        }
      }
    }
    Ok(Ok(())) => {}
  }
}
