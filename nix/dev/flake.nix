{
  description = "Private inputs for development purposes. These are used by the top level flake in the `dev` partition, but do not appear in consumers' lock files.";

  inputs = {
    treefmt-nix.url = "github:numtide/treefmt-nix";
    # Pinned to a nixpkgs revision that packages mdBook 0.4.40, the version the
    # docs require (see `crates/docs_preprocessor/Cargo.toml`). Newer mdBook
    # releases break the docs' double-nested subdirectories.
    nixpkgs-mdbook.url = "github:NixOS/nixpkgs/6ecabf9e3f617aeec1a23a27d0080cab066a9d5b";
  };

  # This flake is only used for its inputs.
  outputs = { ... }: { };
}
