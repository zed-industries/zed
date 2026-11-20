# Builds the re-executable sandbox_linux end-to-end harness for the VM tests.
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
    name = "sandbox-linux-test-helper-source";
  };

  commonArgs = {
    pname = "sandbox-linux-test-helper";
    # An example, not a test target (see crates/sandbox_linux/Cargo.toml):
    # built and run here only, never by CI's `cargo nextest run --workspace`.
    cargoExtraArgs = "-p sandbox_linux --example end_to_end --features __end_to_end_harness --locked";
    cargoBuildCommand = "cargoWithProfile test --no-run";
    version = "0.0.0";
    inherit src;
    cargoLock = ../../../Cargo.lock;
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
      # Dependency artifacts can contain a dummy harness with another hash.
      helper="$(${pkgs.jq}/bin/jq -ers '
        [.[] | select(.reason == "compiler-artifact"
          and .target.name == "end_to_end" and .profile.test == true)
          | .executable | select(. != null)]
        | if length == 1 then .[0] else error("expected one test executable") end
      ' "$cargoBuildLog")"
      cp "$helper" $out/bin/sandbox-linux-test-helper
      runHook postInstall
    '';

    meta = {
      description = "Sandbox behavior test helper";
      platforms = lib.platforms.linux;
    };
  }
)
