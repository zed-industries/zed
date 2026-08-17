mod common;
use common::*;

use std::{fs, path::PathBuf, process::Command};

use tempfile::Builder;

fn viml_escape(in_str: &str) -> String {
  in_str.replace('\\', r"\\")
}

#[test]
fn basic() {
  let dir = Builder::new().prefix("nvim-rs.test").tempdir().unwrap();
  let dir_path = dir.path();
  let buf_path = dir_path.join("curbuf.txt");
  let pong_path = dir_path.join("pong.txt");

  let c1 = format!(
    "let jobid = jobstart([\"{}\", \"{}\"], {{\"rpc\": v:true}})",
    viml_escape(
      PathBuf::from(env!("EXAMPLES_PATH"))
        .join("basic")
        .to_str()
        .unwrap()
    ),
    viml_escape(buf_path.to_str().unwrap())
  );
  let c2 = r#"sleep 100m | let pong = rpcrequest(jobid, "ping")"#;
  let c3 = format!(
    "edit {}| put =pong",
    viml_escape(pong_path.to_str().unwrap())
  );
  let c4 = r#"wqa!"#;

  let status = Command::new(nvim_path())
    .args(&[
      "-u",
      "NONE",
      "--headless",
      "-c",
      &c1,
      "-c",
      c2,
      "-c",
      &c3,
      "-c",
      c4,
    ])
    .status()
    .unwrap();

  assert!(status.success());

  let pong = fs::read_to_string(pong_path).unwrap();
  let buf = fs::read_to_string(buf_path).unwrap();

  assert_eq!("pong", pong.trim());
  assert_eq!("Ext(0, [1])", buf.trim());
}
