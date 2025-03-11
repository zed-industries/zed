{ 
  system ? builtins.currentSystem,
  lock ? builtins.fromJSON (builtins.readFile ./flake.lock),
  rust-overlay ? import (builtins.fetchTarball {
    url = "github:oxalica/rust-overlay/${lock.nodes.rust-overlay.locked.rev}";
    sha256 = lock.nodes.rust-overlay.locked.narHash;
  }),
  pkgs ? import <nixpkgs> { inherit system; overlays = [ rust-overlay ]; }, 
  crane ? import (builtins.fetchTarball {
    url = "github:ipetkov/crane/${lock.nodes.crane.locked.rev}";
    sha256 = lock.nodes.crane.locked.narHash;
  }) { inherit pkgs; }
}:
pkgs.callPackage ./nix/build.nix {
  inherit crane;
  rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
}