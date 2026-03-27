{ inputs, ... }:
pkgs:
let
  rustBin = inputs.rust-overlay.lib.mkRustBin { } pkgs;
in
pkgs.callPackage ./build.nix {
  crane = inputs.crane.mkLib pkgs;
  rustToolchain = rustBin.fromRustupToolchainFile ../rust-toolchain.toml;
}
