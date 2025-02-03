{
  description = "High-performance, multiplayer code editor from the creators of Atom and Tree-sitter";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    flake-compat.url = "github:edolstra/flake-compat";
  };

  outputs =
    {
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

      overlays = {
        rust-overlay = rust-overlay.overlays.default;
        rust-toolchain = final: prev: {
          rustToolchain = final.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        };
        zed-editor = final: prev: {
          zed-editor = final.callPackage ./nix/build.nix {
            crane = crane.mkLib final;
            rustToolchain = final.rustToolchain;
          };
        };
      };

      mkPkgs =
        system:
        import nixpkgs {
          inherit system;
          overlays = builtins.attrValues overlays;
        };

      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f (mkPkgs system));
    in
    {
      packages = forAllSystems (pkgs: {
        zed-editor = pkgs.zed-editor;
        default = pkgs.zed-editor;
      });

      devShells = forAllSystems (pkgs: {
        default = import ./nix/shell.nix { inherit pkgs; };
      });

      formatter = forAllSystems (pkgs: pkgs.nixfmt-rfc-style);

      overlays = overlays // {
        default = nixpkgs.lib.composeManyExtensions (builtins.attrValues overlays);
      };
    };
}
