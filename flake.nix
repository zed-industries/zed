{
  description = "High-performance, multiplayer code editor from the creators of Atom and Tree-sitter";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-compat.url = "github:edolstra/flake-compat";
  };

  outputs = {
    nixpkgs,
    crane,
    fenix,
    ...
  }: let
    forAllSystems = function:
      nixpkgs.lib.genAttrs [
        "x86_64-linux"
        "aarch64-linux"
      ] (system:
        function (import nixpkgs {
          inherit system;
          overlays = [fenix.overlays.default];
        }));
  in {
    packages = forAllSystems (pkgs: let
      craneLib = (crane.mkLib pkgs).overrideToolchain (p: p.fenix.stable.toolchain);
      rustPlatform = pkgs.makeRustPlatform {
        inherit (pkgs.fenix.stable.toolchain) cargo rustc;
      };
      nightlyBuild = pkgs.callPackage ./nix/build.nix {
        inherit craneLib rustPlatform;
      };
    in {
      zed-editor = nightlyBuild;
      default = nightlyBuild;
    });

    devShells = forAllSystems (pkgs: {
      default = import ./nix/shell.nix {inherit pkgs;};
    });

    formatter = forAllSystems (pkgs: pkgs.alejandra);

    overlays.default = final: prev: {
      zed-editor = final.callPackage ./nix/build.nix {
        craneLib = (crane.mkLib final).overrideToolchain (p: p.fenix.stable.toolchain);
        rustPlatform = final.makeRustPlatform {
          inherit (final.fenix.stable.toolchain) cargo rustc;
        };
      };
    };
  };
}
