{
  description = "High-performance, multiplayer code editor from the creators of Atom and Tree-sitter";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    flake-compat.url = "github:edolstra/flake-compat";
  };

  nixConfig.extra-substituters = [ "https://zed-industries.cachix.org" ];
  nixConfig.extra-trusted-public-keys = [
    "zed-industries.cachix.org-1:QW3RoXK0Lm4ycmU5/3bmYRd3MLf4RbTGPqRulGlX5W0="
  ];

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        ./nix/overlay.nix
        ./nix/treefmt.nix
      ];

      systems = [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      perSystem =
        { self', pkgs, ... }:
        {
          packages = {
            default = pkgs.zed-editor;
            debug = self'.packages.default.override { profile = "dev"; };
          };

          devShells = {
            darwin = pkgs.callPackage ./nix/shell-darwin.nix { };
            common = pkgs.callPackage ./nix/shell.nix { };
            default =
              if pkgs.stdenv.hostPlatform.isDarwin then self'.devShells.darwin else self'.devShells.common;
          };
        };
    };
}
