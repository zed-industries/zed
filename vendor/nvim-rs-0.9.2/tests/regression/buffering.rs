#[path = "../common/mod.rs"]
mod common;
use common::*;

use std::{path::PathBuf, process::Command};

fn viml_escape(in_str: &str) -> String {
  in_str.replace('\\', r"\\")
}

fn linebuffercrashbin() -> &'static str {
  #[cfg(feature = "use_tokio")]
  return "linebuffercrash";
  #[cfg(feature = "use_async-std")]
  return "linebuffercrash_as";
}

#[test]
fn linebuffer_crash() {
  let c1 = format!(
    "let jobid = jobstart([\"{}\"], {{\"rpc\": v:true}})",
    viml_escape(
      PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("debug")
        .join(linebuffercrashbin())
        .to_str()
        .unwrap()
    )
  );

  let status = Command::new(nvim_path())
    .args(&[
      "-u",
      "NONE",
      "--headless",
      "-c",
      &c1,
    ])
    .status()
    .unwrap();

  assert!(status.success());

}
