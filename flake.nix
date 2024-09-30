{
  description = "High-performance, multiplayer code editor from the creators of Atom and Tree-sitter";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    flake-compat.url = "github:edolstra/flake-compat";
  };

  outputs = {
    nixpkgs,
    crane,
    fenix,
    ...
  }: let
    systems = ["x86_64-linux" "aarch64-linux"];

    overlays = {
      fenix = fenix.overlays.default;
      rust-toolchain = final: prev: {
        rustToolchain = final.fenix.stable.toolchain;
      };
      zed-editor = final: prev: {
        zed-editor = final.callPackage ./nix/build.nix {
          craneLib = (crane.mkLib final).overrideToolchain final.rustToolchain;
          rustPlatform = final.makeRustPlatform {
            inherit (final.rustToolchain) cargo rustc;
          };
        };
      };
    };

    mkPkgs = system:
      import nixpkgs {
        inherit system;
        overlays = builtins.attrValues overlays;
      };

    forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f (mkPkgs system));
  in {
    packages = forAllSystems (pkgs: {
      zed-editor = pkgs.zed-editor;
      default = pkgs.zed-editor;
    });

    devShells = forAllSystems (pkgs: {
      default = import ./nix/shell.nix {inherit pkgs;};
    });

    formatter = forAllSystems (pkgs: pkgs.alejandra);

    overlays =
      overlays
      // {
        default = nixpkgs.lib.composeManyExtensions (builtins.attrValues overlays);
      };
  };
}
