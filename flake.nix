{
  description = "High-performance, multiplayer code editor from the creators of Atom and Tree-sitter";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-compat.url = "github:nix-community/flake-compat";
  };

  outputs =
    { nixpkgs, ... }@inputs:
    let
      overlays = [ inputs.rust-overlay.overlays.default ];

      forAllSystems =
        f:
        nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed (
          system: f (import nixpkgs { inherit system overlays; })
        );
    in
    {
      packages = forAllSystems (_: {
        default = throw "Nix package was removed from repo, see PR #22825 for an approach you can use instead";
      });

      devShells = forAllSystems (pkgs: {
        default = import ./nix/shell.nix { inherit pkgs; };
      });

      formatter = forAllSystems (pkgs: pkgs.nixfmt-rfc-style);
    };
}
