use std::{
  path::PathBuf,
  env,
};

#[allow(dead_code)]
pub const NVIM_BIN: &str = if cfg!(windows) {
  "nvim.exe"
} else {
  "nvim"
};
const NVIM_PATH: &str = if cfg!(windows) {
  "neovim/build/bin/nvim.exe"
} else {
  "neovim/build/bin/nvim"
};

pub fn nvim_path() -> PathBuf {
  let (path_str, have_env) = match env::var("NVIMRS_TEST_BIN") {
    Ok(path) => (path, true),
    Err(_) => (NVIM_PATH.into(), false),
  };

  let path = PathBuf::from(&path_str);
  if !path.exists() {
    if have_env {
      panic!("nvim bin from $NVIMRS_TEST_BIN \"{}\" does not exist", path_str)
    } else {
      panic!(
        "\"{}\" not found, maybe you need to build it or set \
        $NVIMRS_TEST_BIN?",
        NVIM_PATH
      );
    }
  }
  path
}
