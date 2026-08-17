use neovim_lib::{Neovim, NeovimApi, Session};

fn main() {
  let mut session = Session::new_parent().unwrap();
  let receiver = session.start_event_loop_channel();
  let mut nvim = Neovim::new(session);

  loop {
    match receiver.recv().unwrap().0.as_ref() {
      "file" => {
        let c = nvim.get_current_buf().unwrap();
        for _ in 0..1_000_usize {
          let _x = c.get_lines(&mut nvim, 0, -1, false);
        }
        nvim
          .command("let g:finished_file = reltimestr(reltime(g:started_file))")
          .unwrap();
      }
      "buffer" => {
        nvim.command("let g:started_buffer = reltime()").unwrap();
        for _ in 0..10_000_usize {
          let _ = nvim.get_current_buf().unwrap();
        }
        nvim
          .command(
            "let g:finished_buffer = reltimestr(reltime(g:started_buffer))",
          )
          .unwrap();
      }
      "api" => {
        nvim.command("let g:started_api = reltime()").unwrap();
        for _ in 0..1_000_usize {
          let _ = nvim.get_api_info().unwrap();
        }
        nvim
          .command("let g:finished_api = reltimestr(reltime(g:started_api))")
          .unwrap();
      }
      _ => break,
    }
  }
}
