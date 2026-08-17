use std::error::Error;

use async_trait::async_trait;

use rmpv::Value;

use async_std::{self, fs::File as ASFile};

use nvim_rs::{create::async_std as create, Handler, Neovim};

#[derive(Clone)]
struct NeovimHandler {}

#[async_trait]
impl Handler for NeovimHandler {
  type Writer = ASFile;

  async fn handle_request(
    &self,
    name: String,
    _args: Vec<Value>,
    neovim: Neovim<ASFile>,
  ) -> Result<Value, Value> {
    match name.as_ref() {
      "file" => {
        let c = neovim.get_current_buf().await.unwrap();
        for _ in 0..1_000_usize {
          let _x = c.get_lines(0, -1, false).await;
        }
        Ok(Value::Nil)
      }
      "buffer" => {
        for _ in 0..10_000_usize {
          let _ = neovim.get_current_buf().await.unwrap();
        }
        Ok(Value::Nil)
      }
      "api" => {
        for _ in 0..1_000_usize {
          let _ = neovim.get_api_info().await.unwrap();
        }
        Ok(Value::Nil)
      }
      _ => Ok(Value::Nil),
    }
  }
}

#[async_std::main]
async fn main() {
  let handler: NeovimHandler = NeovimHandler {};

  let (nvim, io_handler) = create::new_parent(handler).await.unwrap();

  // Any error should probably be logged, as stderr is not visible to users.
  match io_handler.await {
    Err(err) => {
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
    Ok(()) => {}
  }
}
