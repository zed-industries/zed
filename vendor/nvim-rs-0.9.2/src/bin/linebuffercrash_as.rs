
use nvim_rs::{
  create::async_std as create, 
  rpc::handler::Dummy as DummyHandler
};


#[async_std::main]
async fn main() {
  let handler = DummyHandler::new();
  let (nvim, _io_handler) = create::new_parent(handler).await.unwrap();
  let curbuf = nvim.get_current_buf().await.unwrap();

    // If our Stdout is linebuffered, this has a high chance of crashing neovim
    // Should probably befixed in neovim itself, but for now, let's just make
    // sure we're not using linebuffering, or at least don't crash neovim with
    // this.
    for i in 0..20 { 
        curbuf.set_name(&format!("a{i}")).await.unwrap();
    }

    let _ = nvim.command("quit!").await;

}
