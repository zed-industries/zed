# Linux sandbox VM tests

These are `pkgs.testers.nixosTest` checks exported by the root flake on Linux.
They require Nix with flakes and a Linux builder with usable `/dev/kvm`
(including access for the Nix build user). No Zed UI is involved.

The existing entrypoint discovers all `sandbox-*` checks:

```sh
cargo xtask sandbox-tests --list
cargo xtask sandbox-tests
cargo xtask sandbox-tests sandbox-linux-working sandbox-linux-userns-disabled
```

Run individual checks or debug interactively with the NixOS test driver:

```sh
nix build .#checks.x86_64-linux.sandbox-linux-working -L --accept-flake-config
nix build .#checks.x86_64-linux.sandbox-linux-userns-disabled -L --accept-flake-config
nix run .#checks.x86_64-linux.sandbox-linux-working.driverInteractive
```

Replace `x86_64-linux` with `aarch64-linux` on an appropriate builder.
`cargo xtask sandbox-tests --rebuild ...` passes `--rebuild` to Nix to rerun a
previously built check rather than reuse its successful cached result.

For an untracked working tree (Nix's Git flake source omits new files), or a
shallow checkout whose Nix version cannot determine `revCount`, use `path:.`
instead of `.` in the direct Nix commands. Do not commit just to run a check.

## Experimental `sandbox_linux`

`sandbox-linux-working` builds the crate's `end_to_end` example with Crane and
installs that executable in the VM. It is an example rather than a test target,
gated behind the `__end_to_end_harness` feature, so CI's
`cargo nextest run --workspace` never builds or runs it. Its hidden
supervisor/helper re-exec entrypoints are preserved. The harness runs as a
normal user, not root, with namespace creation enabled and a 180-second outer
timeout. `--require-sandbox` makes setup failure fatal instead of taking the
optional host-test skip path.

Coverage includes namespace separation, read/write globs and symlink/hard-link
escape attempts, host-side integrity checks, Ask decisions (including handler
panic and concurrent requests), live policy changes, process cleanup, TCP,
synthetic DNS, UDP and Unix socket permission checks, and nonblocking TCP
responsiveness/cancellation. Echo peers run locally outside the sandbox, inside
the disposable VM; no Internet service is needed. Escape targets and other
filesystem fixtures live under an automatically cleaned
`/tmp/sandbox-escape-test-*` directory.

`sandbox-linux-userns-disabled` boots with `user.max_user_namespaces=0`. It
requires namespace setup to fail and verifies that the untrusted command's
canary was never written. This is an expected failure assertion, not a skip.

For faster host-side checks (the strict test needs usable user namespaces,
seccomp notification support, Python 3, coreutils and util-linux on `PATH`):

```sh
cargo test -p sandbox_linux --lib --locked
./script/clippy -p sandbox_linux
cargo build -p sandbox_linux --example end_to_end --features __end_to_end_harness --locked
target/debug/examples/end_to_end --require-sandbox
```

Without `--require-sandbox`, the end-to-end harness may skip if host namespace
creation is unavailable. Do not use that optional invocation as VM evidence.

## CI

These VM checks exercise `sandbox_linux`, not the existing Bubblewrap backend.
The Bubblewrap backend and its WSL tests remain in place.

The generated `nix_build` GitHub workflow (`run-nix` / `run-bundling` labels)
currently builds only `.#default`; it does **not** execute these VM checks.
Run the commands above explicitly on a KVM-capable builder. The new checks are
also included in `nix flake check` on Linux and in the existing xtask's default
selection; no separate CI/test framework is introduced.
