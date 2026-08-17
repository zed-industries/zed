const child_process = require('child_process');
const toolchain = process.env.INPUT_TOOLCHAIN;
const fs = require('fs');

function set_env(name, val) {
  fs.appendFileSync(process.env['GITHUB_ENV'], `${name}=${val}\n`)
}

// Needed for now to get 1.24.2 which fixes a bug in 1.24.1 that causes issues
// on Windows.
if (process.platform === 'win32') {
  child_process.execFileSync('rustup', ['self', 'update']);
}

child_process.execFileSync('rustup', ['set', 'profile', 'minimal']);
child_process.execFileSync('rustup', ['update', toolchain, '--no-self-update']);
child_process.execFileSync('rustup', ['default', toolchain]);

// Deny warnings on CI to keep our code warning-free as it lands in-tree. Don't
// do this on nightly though since there's a fair amount of warning churn there.
//
// Disable this for now to avoid `unexpected_cfgs` in a way that works with
// Rust 1.34.
//if (!toolchain.startsWith('nightly')) {
//  set_env("RUSTFLAGS", "-D warnings");
//}

// Save disk space by avoiding incremental compilation, and also we don't use
// any caching so incremental wouldn't help anyway.
set_env("CARGO_INCREMENTAL", "0");

// Turn down debuginfo from 2 to 1 to help save disk space
set_env("CARGO_PROFILE_DEV_DEBUG", "1");
set_env("CARGO_PROFILE_TEST_DEBUG", "1");

if (process.platform === 'darwin') {
  set_env("CARGO_PROFILE_DEV_SPLIT_DEBUGINFO", "unpacked");
  set_env("CARGO_PROFILE_TEST_SPLIT_DEBUGINFO", "unpacked");
}
