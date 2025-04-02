{
  lib,
  mkShellNoCC,
  stdenvNoCC,
  bintools,

  rust-toolchain,
  rust-analyzer,
  cargo-nextest,

  cmake,
  protobuf,
  nodejs_22,
}:
let
  # See: https://github.com/NixOS/nixpkgs/pull/392695, remove once it hits nixpkgs-unstable
  stdenvNoCCSDK = stdenvNoCC.override (
    lib.optionalAttrs stdenvNoCC.hostPlatform.isDarwin {
      extraBuildInputs = [ ];
    }
  );
in
(mkShellNoCC.override { stdenv = stdenvNoCCSDK; }) {
  packages = [
    # Ensure that CC doesn't get propagated
    (rust-toolchain.overrideAttrs {
      propagatedBuildInputs = [ ];
      depsTargetTargetPropagated = [ ];
      depsHostHostPropagated = [ bintools ];
    })
    rust-analyzer
    cargo-nextest

    cmake
    protobuf

    nodejs_22
  ];
}
