{pkgs ? import <nixpkgs> {}}: let
  stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.llvmPackages_18.stdenv;
in
  if pkgs.stdenv.isDarwin
  then
    # See https://github.com/NixOS/nixpkgs/issues/320084
    throw "zed: nix dev-shell isn't supported on darwin yet."
  else let
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
      vulkan-loader
    ];
  in
    pkgs.mkShell.override {inherit stdenv;} {
      nativeBuildInputs = with pkgs; [
        clang
        curl
        perl
        pkg-config
        protobuf
        rustPlatform.bindgenHook
      ];

      inherit buildInputs;

      shellHook = ''
        export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath ([
            pkgs.vulkan-loader
          ]
          ++ buildInputs)}:$LD_LIBRARY_PATH"
        export PROTOC="${pkgs.protobuf}/bin/protoc"
      '';

      FONTCONFIG_FILE = pkgs.makeFontsConf {
        fontDirectories = [
          "./assets/fonts/zed-mono"
          "./assets/fonts/zed-sans"
        ];
      };
      ZSTD_SYS_USE_PKG_CONFIG = true;
    }
