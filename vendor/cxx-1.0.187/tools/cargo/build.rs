use std::io::{self, Write};
#[cfg(windows)]
use std::os::windows::fs as windows;
use std::path::Path;
use std::process;
#[cfg(windows)]
use std::{env, fs};

const MISSING: &str = "
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
When building `cxx` from a git clone, git's symlink support needs
to be enabled on platforms that have it off by default (Windows).
Either use:

   $ git config --global core.symlinks true

prior to cloning, or else use:

   $ git clone -c core.symlinks=true https://github.com/dtolnay/cxx

for the clone.

Symlinks are only required when compiling locally from a clone of
the git repository---they are NOT required when building `cxx` as
a Cargo-managed (possibly transitive) build dependency downloaded
through crates.io.
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
";

#[cfg(windows)]
const DENIED: &str = "
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
When building `cxx` from a git clone on Windows we need Developer
Mode enabled for symlink support.

To enable Developer Mode: go under Settings to Update & Security,
then 'For developers', and turn on the toggle for Developer Mode.

For more explanation of symlinks in Windows, see these resources:
> https://blogs.windows.com/windowsdeveloper/2016/12/02/symlinks-windows-10/
> https://docs.microsoft.com/windows/uwp/get-started/enable-your-device-for-development

Symlinks are only required when compiling locally from a clone of
the git repository---they are NOT required when building `cxx` as
a Cargo-managed (possibly transitive) build dependency downloaded
through crates.io.
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-cfg=check_cfg");
    println!("cargo:rustc-check-cfg=cfg(check_cfg)");

    if Path::new("src/syntax/mod.rs").exists() {
        return;
    }

    #[cfg_attr(not(windows), expect(unused_mut))]
    let mut message = MISSING;

    #[cfg(windows)]
    if let Some(out_dir) = env::var_os("OUT_DIR") {
        let parent_dir = Path::new(&out_dir).join("symlink");
        let original_dir = parent_dir.join("original");
        let link_dir = parent_dir.join("link");
        if fs::create_dir_all(&original_dir).is_ok()
            && (!link_dir.exists() || fs::remove_dir(&link_dir).is_ok())
            && windows::symlink_dir(&original_dir, &link_dir).is_err()
        {
            message = DENIED;
        }
    }

    let _ = io::stderr().write_all(message.as_bytes());
    process::exit(1);
}
