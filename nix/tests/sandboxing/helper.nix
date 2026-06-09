# Builds the `landlock_test_helper` binary (sandbox crate, `nixos-test`
# feature) used by the Landlock VM tests in this directory.
{
  pkgs,
  inputs,
}:
let
  lib = pkgs.lib;

  rustBin = inputs.rust-overlay.lib.mkRustBin { } pkgs;
  rustToolchain = rustBin.fromRustupToolchainFile ../../../rust-toolchain.toml;
  craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

  src = builtins.path {
    path = ../../../.;
    filter =
      path: type:
      let
        root = toString ../../../. + "/";
        relPath = lib.removePrefix root path;
        firstComp = builtins.head (lib.path.subpath.components relPath);
      in
      builtins.elem firstComp [
        "crates"
        "assets"
        "extensions"
        "script"
        "tooling"
        "Cargo.toml"
        ".config"
        ".cargo"
      ];
    name = "landlock-test-helper-source";
  };

  commonArgs = {
    pname = "landlock-test-helper";
    version = "0.0.0";
    inherit src;
    cargoLock = ../../../Cargo.lock;
    cargoExtraArgs = "-p sandbox --bin landlock_test_helper --features sandbox/nixos-test --locked";
    CARGO_PROFILE = "dev";
    doCheck = false;

    cargoVendorDir = craneLib.vendorCargoDeps {
      inherit src;
      cargoLock = ../../../Cargo.lock;
    };
  };

  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
in
craneLib.buildPackage (
  commonArgs
  // {
    inherit cargoArtifacts;

    installPhase = ''
      runHook preInstall
      mkdir -p $out/bin
      cp target/debug/landlock_test_helper $out/bin/landlock_test_helper
      runHook postInstall
    '';

    meta = {
      description = "Landlock sandbox behavior test helper";
      platforms = lib.platforms.linux;
    };
  }
)
