{
  pkgs ? import <nixpkgs> { },
}:

let
  stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.llvmPackages_18.stdenv;
in
(pkgs.mkShell.override { inherit stdenv; }) rec {
  nativeBuildInputs = with pkgs; [
    copyDesktopItems
    curl
    perl
    pkg-config
    protobuf
    rustPlatform.bindgenHook
  ];

  buildInputs = with pkgs; [
    curl
    fontconfig
    freetype
    libgit2
    openssl
    sqlite
    zlib
    zstd

    alsa-lib
    libxkbcommon
    wayland
    xorg.libxcb
  ];

  env = {
    LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath (buildInputs ++ [
      stdenv.cc.cc.lib
      vulkan-loader
    ]);
    ZSTD_SYS_USE_PKG_CONFIG = true;
    FONTCONFIG_FILE = pkgs.makeFontsConf {
      fontDirectories = [
        "assets/fonts/zed-mono"
        "assets/fonts/zed-sans"
      ];
    };
  };
}
