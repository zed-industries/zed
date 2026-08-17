#![allow(unused)]
use nvim_rs::rpc::handler::Dummy as DummyHandler;

#[cfg(feature = "use_tokio")]
use nvim_rs::create::tokio as create;
#[cfg(feature = "use_tokio")]
use tokio::test as atest;

#[cfg(feature = "use_async-std")]
use async_std::test as atest;
#[cfg(feature = "use_async-std")]
use nvim_rs::create::async_std as create;

use std::{
  path::Path,
  process::Command,
  thread::sleep,
  time::{Duration, Instant},
};

#[cfg(unix)]
use tempfile::{Builder, TempDir};

#[path = "../common/mod.rs"]
mod common;
use common::*;

const HOST: &str = "127.0.0.1";
const PORT: u16 = 6666;

#[atest]
async fn can_connect_via_tcp() {
  let listen = HOST.to_string() + ":" + &PORT.to_string();

  let mut child = Command::new(nvim_path())
    .args(&["-u", "NONE", "--headless", "--listen", &listen])
    .spawn()
    .expect("Cannot start neovim");

  // wait at most 1 second for neovim to start and create the tcp socket
  let start = Instant::now();

  let (nvim, _io_handle) = loop {
    sleep(Duration::from_millis(100));

    let handler = DummyHandler::new();
    if let Ok(r) = create::new_tcp(&listen, handler).await {
      break r;
    } else {
      if Duration::from_secs(1) <= start.elapsed() {
        panic!("Unable to connect to neovim via tcp at {}", listen);
      }
    }
  };

  let servername = nvim
    .get_vvar("servername")
    .await
    .expect("Error retrieving servername from neovim");

  child.kill().expect("Could not kill neovim");

  assert_eq!(&listen, servername.as_str().unwrap());
}

#[cfg(unix)]
fn get_socket_path() -> (std::path::PathBuf, TempDir) {
  let dir = Builder::new()
    .prefix("neovim-lib.test")
    .tempdir()
    .expect("Cannot create temporary directory for test.");

  (dir.path().join("unix_socket"), dir)
}

#[cfg(windows)]
fn get_socket_path() -> (std::path::PathBuf, ()) {
  let rand = fastrand::u32(..);
  let name = format!(r"\\.\pipe\nvim-rs-test-{}", rand);
  (name.into(), ())
}

#[cfg(not(all(feature = "use_async-std", windows)))]
#[atest]
async fn can_connect_via_path() {
  let (socket_path, _guard) = get_socket_path();

  let mut child = Command::new(nvim_path())
    .args(&["-u", "NONE", "--headless"])
    .env("NVIM_LISTEN_ADDRESS", &socket_path)
    .spawn()
    .expect("Cannot start neovim");

  // wait at most 1 second for neovim to start and create the socket
  {
    let start = Instant::now();
    let one_second = Duration::from_secs(1);
    loop {
      sleep(Duration::from_millis(100));

      if let Ok(_) = std::fs::metadata(&socket_path) {
        break;
      }

      if one_second <= start.elapsed() {
        panic!("neovim socket not found at '{:?}'", &socket_path);
      }
    }
  }

  let handler = DummyHandler::new();

  let (nvim, _io_handle) = create::new_path(&socket_path, handler)
    .await
    .expect(&format!(
      "Unable to connect to neovim's unix socket at {:?}",
      &socket_path
    ));

  let servername = nvim
    .get_vvar("servername")
    .await
    .expect("Error retrieving servername from neovim")
    .as_str()
    .unwrap()
    .to_string();

  child.kill().expect("Could not kill neovim");

  assert_eq!(socket_path, Path::new(&servername));
}
