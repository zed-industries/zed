{
  description = "High-performance, multiplayer code editor from the creators of Atom and Tree-sitter";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-unstable";
    nixfenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    nixfenix,
  }: let
    inherit (self) outputs;
    systems = [
      "aarch64-linux"
      "x86_64-linux"
      "aarch64-darwin"
      "x86_64-darwin"
    ];
    forAllSystems = nixpkgs.lib.genAttrs systems;
  in {
    formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.alejandra);

    devShells = forAllSystems (
      system: let
        pkgs = import nixpkgs {inherit system;};
        fenix = nixfenix.packages.${system};
        rust-toolchain = (pkgs.lib.importTOML ./rust-toolchain.toml).toolchain;
        complete-toolchain = fenix.fromToolchainName {
          name = rust-toolchain.channel;
          sha256 = "sha256-6eN/GKzjVSjEhGO9FhWObkRFaE1Jf+uqMSdQnb8lcB4=";
        };
        toolchain = complete-toolchain.withComponents (rust-toolchain.components
          ++ [
            "cargo"
            "rust-src"
            "rust-analyzer"
            "clippy"
          ]);
      in rec
      {
        default = import ./shell.nix {inherit pkgs;};

        # Usage, either:
        #   a: `nix develop .#with-toolchain`
        #   b: `echo "use flake .#with-toolchain" > .envrc`
        with-toolchain = default.overrideAttrs (old: {
          nativeBuildInputs =
            old.nativeBuildInputs
            ++ [
              toolchain
            ];
        });
      }
    );
  };
}
