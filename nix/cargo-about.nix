# Vendored from https://raw.githubusercontent.com/NixOS/nixpkgs/1d0bb7b61b251a261b0963aacf4b141e770a4f1d/pkgs/by-name/ca/cargo-about/package.nix
# This lets us pin cargo-about to 0.8.2.
# See https://github.com/zed-industries/zed/pull/44012 for more context.
{
  lib,
  rustPlatform,
  fetchFromGitHub,
  pkg-config,
  zstd,
}:

rustPlatform.buildRustPackage rec {
  pname = "cargo-about";
  version = "0.8.2";

  src = fetchFromGitHub {
    owner = "EmbarkStudios";
    repo = "cargo-about";
    tag = version;
    sha256 = "sha256-cNKZpDlfqEXeOE5lmu79AcKOawkPpk4PQCsBzNtIEbs=";
  };

  cargoHash = "sha256-NnocSs6UkuF/mCM3lIdFk+r51Iz2bHuYzMT/gEbT/nk=";

  nativeBuildInputs = [ pkg-config ];

  buildInputs = [ zstd ];

  env = {
    ZSTD_SYS_USE_PKG_CONFIG = true;
  };

  meta = {
    description = "Cargo plugin to generate list of all licenses for a crate";
    homepage = "https://github.com/EmbarkStudios/cargo-about";
    changelog = "https://github.com/EmbarkStudios/cargo-about/blob/${version}/CHANGELOG.md";
    license = with lib.licenses; [
      mit # or
      asl20
    ];
    maintainers = with lib.maintainers; [
      evanjs
      matthiasbeyer
    ];
    mainProgram = "cargo-about";
  };
}
