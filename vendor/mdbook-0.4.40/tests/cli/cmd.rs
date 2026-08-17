use assert_cmd::Command;

pub(crate) fn mdbook_cmd() -> Command {
    let mut cmd = Command::cargo_bin("mdbook").unwrap();
    cmd.env_remove("RUST_LOG");
    cmd
}
