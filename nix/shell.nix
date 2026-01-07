{
  mkShell,
  makeFontsConf,
  pkgsCross,

  zed-editor,

  rust-analyzer,
  rustup,
  cargo-nextest,
  cargo-hakari,
  cargo-machete,
  cargo-zigbuild,
  nixfmt-rfc-style,
  protobuf,
  nodejs_22,
  zig,
}:
mkShell {
  inputsFrom = [ zed-editor ];
  packages = [
    rust-analyzer
    rustup
    cargo-nextest
    cargo-hakari
    cargo-machete
    cargo-zigbuild
    nixfmt-rfc-style
    # TODO: package protobuf-language-server for editing zed.proto
    # TODO: add other tools used in our scripts

    # `build.nix` adds this to the `zed-editor` wrapper (see `postFixup`)
    # we'll just put it on `$PATH`:
    nodejs_22
    zig
  ];

  env = {
    # note: different than `$FONTCONFIG_FILE` in `build.nix` – this refers to relative paths
    # outside the nix store instead of to `$src`
    FONTCONFIG_FILE = makeFontsConf {
      fontDirectories = [
        "./assets/fonts/lilex"
        "./assets/fonts/ibm-plex-sans"
      ];
    };
    PROTOC = "${protobuf}/bin/protoc";
    ZED_ZSTD_MUSL_LIB = "${pkgsCross.musl64.pkgsStatic.zstd.out}/lib";
  };
}
