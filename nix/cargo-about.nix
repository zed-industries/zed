{
  cargo-about,
  fetchFromGitHub,
  rustPlatform,
}:
cargo-about.overrideAttrs (
  new: old: rec {
    version = "0.8.2";

    src = fetchFromGitHub {
      owner = "EmbarkStudios";
      repo = "cargo-about";
      tag = version;
      sha256 = "sha256-cNKZpDlfqEXeOE5lmu79AcKOawkPpk4PQCsBzNtIEbs=";
    };

    cargoHash = "sha256-NnocSs6UkuF/mCM3lIdFk+r51Iz2bHuYzMT/gEbT/nk=";

    # NOTE: can drop once upstream uses `finalAttrs` here:
    # https://github.com/NixOS/nixpkgs/blob/10214747f5e6e7cb5b9bdf9e018a3c7b3032f5af/pkgs/build-support/rust/build-rust-package/default.nix#L104
    #
    # See (for context): https://github.com/NixOS/nixpkgs/pull/382550
    cargoDeps = rustPlatform.fetchCargoVendor {
      inherit (new) src;
      hash = new.cargoHash;
      patches = new.cargoPatches or [ ];
      name = new.cargoDepsName or new.finalPackage.name;
    };
  }
)
