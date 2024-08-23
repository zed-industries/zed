{
  description = "High-performance, multiplayer code editor from the creators of Atom and Tree-sitter";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, fenix, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ fenix.overlays.default ];
        };
        toolchain = pkgs.fenix.stable.toolchain;
      in
      {
        packages = {
          zed-editor = pkgs.callPackage ./nix/build.nix { inherit toolchain; };
          default = self.packages.${system}.zed-editor;
        };

        devShells.default = import ./nix/shell.nix { inherit pkgs; };

        overlays.default = final: prev: {
          zed-editor = final.callPackage ./nix/build.nix { inherit toolchain; };
        };
      }
    ) // {
      overlays.default = final: prev: {
        zed-editor = final.callPackage ./nix/build.nix { toolchain = final.fenix.stable.toolchain; };
      };
    };
}
