{
  description = "High-performance, multiplayer code editor from the creators of Atom and Tree-sitter";

  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixpkgs-unstable/nixexprs.tar.xz";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    flake-compat.url = "github:edolstra/flake-compat";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      crane,
      ...
    }:
    let
      systems = [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
      mkWorkspace =
        pkgs:
        let
          rustBin = rust-overlay.lib.mkRustBin { } pkgs;
          toolchain = rustBin.fromRustupToolchainFile ./rust-toolchain.toml;
        in
        (pkgs.callPackage ./Cargo.nix {
          buildRustCrateForPkgs =
            pkgs:
            pkgs.buildRustCrate.override {
              rustc = toolchain;
              cargo = toolchain;
              defaultCodegenUnits = 16;
              defaultCrateOverrides = pkgs.defaultCrateOverrides // (pkgs.callPackage ./nix/overrides.nix { });
            };
        });
      # Pull just the zed binary out of the workspace
      mkZed = pkgs: (mkWorkspace pkgs).workspaceMembers.zed.build;
    in
    {
      workspace = forAllSystems mkWorkspace;
      packages = forAllSystems (pkgs: rec {
        default = mkZed pkgs;
        debug = default.override { profile = "dev"; };
      });
      devShells = forAllSystems (pkgs: {
        default = pkgs.callPackage ./nix/shell.nix {
          zed-editor = mkZed pkgs;
        };
      });
      formatter = forAllSystems (pkgs: pkgs.nixfmt-rfc-style);
      overlays.default = final: _: {
        zed-editor = mkZed final;
      };
    };

  nixConfig = {
    extra-substituters = [
      "https://zed.cachix.org"
      "https://cache.garnix.io"
    ];
    extra-trusted-public-keys = [
      "zed.cachix.org-1:/pHQ6dpMsAZk2DiP4WCL0p9YDNKWj2Q5FL20bNmw1cU="
      "cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g="
    ];
  };
}
