{
  description = "A security-focused IDE";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust
            cargo
            rustc
            clippy
            rust-analyzer
            cargo-watch

            # Node.js
            nodejs
            pnpm

            # Other
            concurrently
            gvisor
            cosign
            wireguard-tools
            devour-cli
            shellcheck
          ];
        };
      });
}
