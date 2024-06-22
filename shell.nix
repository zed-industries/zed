{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  nativeBuildInputs =
    with pkgs;
    [
      copyDesktopItems
      curl
      perl
      pkg-config
      protobuf
      rustPlatform.bindgenHook
    ]
    ++ lib.optionals stdenv.isDarwin [ pkgs.xcbuild.xcrun ];

  buildInputs =
    with pkgs;
    [
      curl
      fontconfig
      freetype
      libgit2
      openssl
      sqlite
      zlib
      zstd
    ]
    ++ lib.optionals stdenv.isLinux [
      alsa-lib
      libxkbcommon
      wayland
      xorg.libxcb
    ]
    ++ lib.optionals stdenv.isDarwin (
      with darwin.apple_sdk.frameworks;
      [
        AppKit
        CoreAudio
        CoreFoundation
        CoreGraphics
        CoreMedia
        CoreServices
        CoreText
        Foundation
        IOKit
        Metal
        Security
        SystemConfiguration
        VideoToolbox
      ]
    );

  env = {
    ZSTD_SYS_USE_PKG_CONFIG = true;
    FONTCONFIG_FILE = pkgs.makeFontsConf {
      fontDirectories = [
        "assets/fonts/zed-mono"
        "assets/fonts/zed-sans"
      ];
    };
  };
}
