//! Scorched earth. See src/examples/scorched_earth.rs for documentation
use std::{error::Error, sync::Arc};

use async_trait::async_trait;

use rmpv::Value;

use futures::lock::Mutex;

use nvim_rs::{create::async_std as create, Handler, Neovim};

use async_std::{self, fs::File as ASFile};

struct Posis {
  cursor_start: Option<(u64, u64)>,
  cursor_end: Option<(u64, u64)>,
}

fn the_larger(
  first: Option<(u64, u64)>,
  second: (u64, u64),
) -> Option<(u64, u64)> {
  if first.map(|t| t >= second) == Some(true) {
    first
  } else {
    Some(second)
  }
}

fn the_smaller(
  first: Option<(u64, u64)>,
  second: (u64, u64),
) -> Option<(u64, u64)> {
  if first.map(|t| t <= second) == Some(true) {
    first
  } else {
    Some(second)
  }
}

#[derive(Clone)]
struct NeovimHandler(Arc<Mutex<Posis>>);

#[async_trait]
impl Handler for NeovimHandler {
  type Writer = ASFile;

  async fn handle_notify(
    &self,
    name: String,
    args: Vec<Value>,
    neovim: Neovim<ASFile>,
  ) {
    match name.as_ref() {
      "cursor-moved-i" => {
        let line = args[0].as_u64().unwrap();
        let column = args[1].as_u64().unwrap();

        let posis = &mut *(self.0).lock().await;

        posis.cursor_start = the_smaller(posis.cursor_start, (line, column));
        posis.cursor_end = the_larger(posis.cursor_end, (line, column));

        let cmd = format!(
          "syntax region ScorchedEarth start=/\\%{}l\\%{}c/ end=/\\%{}l\\%{}c/",
          posis.cursor_start.unwrap().0,
          posis.cursor_start.unwrap().1,
          posis.cursor_end.unwrap().0,
          posis.cursor_end.unwrap().1
        );

        neovim.command(&cmd).await.unwrap();
      }
      "insert-enter" => {
        let _mode = args[0].as_str().unwrap();
        let line = args[1].as_u64().unwrap();
        let column = args[2].as_u64().unwrap();

        let posis = &mut *(self.0).lock().await;

        posis.cursor_start = Some((line, column));
        posis.cursor_end = Some((line, column));
      }
      "insert-leave" => {
        let posis = &mut *(self.0).lock().await;

        posis.cursor_start = None;
        posis.cursor_end = None;
        neovim.command("syntax clear ScorchedEarth").await.unwrap();
      }
      _ => {}
    }
  }
}

#[async_std::main]
async fn main() {
  let p = Posis {
    cursor_start: None,
    cursor_end: None,
  };
  let handler: NeovimHandler = NeovimHandler(Arc::new(Mutex::new(p)));

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
