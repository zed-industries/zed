{
  pkgs ? import <nixpkgs> { },
}:
let
  inherit (pkgs) lib;
in
pkgs.mkShell rec {
  packages =
    [
      pkgs.clang
      pkgs.curl
      pkgs.cmake
      pkgs.perl
      pkgs.pkg-config
      pkgs.protobuf
      pkgs.rustPlatform.bindgenHook
      pkgs.rust-analyzer
    ]
    ++ lib.optionals pkgs.stdenv.hostPlatform.isLinux [
      pkgs.mold
    ];

  buildInputs =
    [
      pkgs.bzip2
      pkgs.curl
      pkgs.fontconfig
      pkgs.freetype
      pkgs.libgit2
      pkgs.openssl
      pkgs.sqlite
      pkgs.stdenv.cc.cc
      pkgs.zlib
      pkgs.zstd
      pkgs.rustToolchain
    ]
    ++ lib.optionals pkgs.stdenv.hostPlatform.isLinux [
      pkgs.alsa-lib
      pkgs.libxkbcommon
      pkgs.wayland
      pkgs.xorg.libxcb
      pkgs.vulkan-loader
    ]
    ++ lib.optional pkgs.stdenv.hostPlatform.isDarwin pkgs.apple-sdk_15;

  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;

  PROTOC="${pkgs.protobuf}/bin/protoc";

  # We set SDKROOT and DEVELOPER_DIR to the Xcode ones instead of the nixpkgs ones,
  # because we need Swift 6.0 and nixpkgs doesn't have it.
  # Xcode is required for development anyways
  shellHook = lib.optionalString pkgs.stdenv.hostPlatform.isDarwin ''
      export SDKROOT="/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk";
      export DEVELOPER_DIR="/Applications/Xcode.app/Contents/Developer";
    '';

  FONTCONFIG_FILE = pkgs.makeFontsConf {
    fontDirectories = [
      "./assets/fonts/zed-mono"
      "./assets/fonts/zed-sans"
    ];
  };
  ZSTD_SYS_USE_PKG_CONFIG = true;
}
