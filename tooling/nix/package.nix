{
  pkgs ? import <nixpkgs> {},
  fenix ? import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") {},
}: let
  inherit (pkgs) lib stdenv;
  rust-toolchain = (lib.importTOML ./../../rust-toolchain.toml).toolchain;
  complete-toolchain = fenix.fromToolchainName {
    name = rust-toolchain.channel;
    sha256 = "sha256-6eN/GKzjVSjEhGO9FhWObkRFaE1Jf+uqMSdQnb8lcB4=";
  };
  rustPlatform = pkgs.makeRustPlatform {
    inherit (complete-toolchain) cargo rustc;
  };
in
  rustPlatform.buildRustPackage rec {
    name = "zed-editor";
    version = "git";

    src = ./../..;

    nativeBuildInputs = with pkgs;
      [
        copyDesktopItems
        curl
        perl
        pkg-config
        protobuf
        rustPlatform.bindgenHook
      ]
      ++ lib.optionalString stdenv.isLinux [llvmPackages.clangUseLLVM llvmPackages.bintools mold]
      ++ lib.optionals stdenv.isDarwin [xcbuild.xcrun];

    buildInputs = with pkgs;
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
        with darwin.apple_sdk.frameworks; [
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

    cargoLock = {
      lockFile = ./../../Cargo.lock;
      outputHashes = {
        "alacritty_terminal-0.24.1-dev" = "sha256-aVB1CNOLjNh6AtvdbomODNrk00Md8yz8QzldzvDo1LI=";
        "async-pipe-0.1.3" = "sha256-g120X88HGT8P6GNCrzpS5SutALx5H+45Sf4iSSxzctE=";
        "blade-graphics-0.4.0" = "sha256-o3iYBrHcLXSrdvd0J/LXJb7VkTcFyB/S2Nk9WrmZupI=";
        "cosmic-text-0.11.2" = "sha256-TLPDnqixuW+aPAhiBhSvuZIa69vgV3xLcw32OlkdCcM=";
        "font-kit-0.14.1" = "sha256-qUKvmi+RDoyhMrZ7T6SoVAyMc/aasQ9Y/okzre4SzXo=";
        "fontdb-0.18.0" = "sha256-yYv+DWHBty1gko3I0e4UU856SJX0TuYtvOKgcwLcFpA=";
        "lsp-types-0.95.1" = "sha256-N4MKoU9j1p/Xeowki/+XiNQPwIcTm9DgmfM/Eieq4js=";
        "nvim-rs-0.6.0-pre" = "sha256-bdWWuCsBv01mnPA5e5zRpq48BgOqaqIcAu+b7y1NnM8=";
        "tree-sitter-0.22.6" = "sha256-P9pQcofDCIhOYWA1OC8TzB5UgWpD5GlDzX2DOS8SsH0=";
        "tree-sitter-gomod-1.0.2" = "sha256-/sjC117YAFniFws4F/8+Q5Wrd4l4v4nBUaO9IdkixSE=";
        "tree-sitter-gowork-0.0.1" = "sha256-803ujH5qwejQ2vQDDpma4JDC9a+vFX8ZQmr+77VyL2M=";
        "tree-sitter-heex-0.0.1" = "sha256-VakMZtWQ/h7dNy5ehk2Bh14a5s878AUgwY3Ipq8tPec=";
        "tree-sitter-md-0.2.3" = "sha256-Fa73P1h5GvKV3SxXr0KzHuNp4xa5wxUzI8ecXbGdrYE=";
        "xim-0.4.0" = "sha256-vxu3tjkzGeoRUj7vyP0vDGI7fweX8Drgy9hwOUOEQIA=";
        "xkbcommon-0.7.0" = "sha256-2RjZWiAaz8apYTrZ82qqH4Gv20WyCtPT+ldOzm0GWMo=";
      };
    };

    cargoBuildFlags = [
      "--package=zed"
      "--package=cli"
    ];

    buildFeatures = ["gpui/runtime_shaders"];

    env = {
      ZSTD_SYS_USE_PKG_CONFIG = true;
      OPENSSL_NO_VENDOR = 1;
      FONTCONFIG_FILE = pkgs.makeFontsConf {
        fontDirectories = [
          "${src}/assets/fonts/zed-mono"
          "${src}/assets/fonts/zed-sans"
        ];
      };
    };

    # Using fenix seems to have broken the bindgen hook.
    postFixup = let
      dynlibs = with pkgs; buildInputs ++ [vulkan-loader];
    in
      lib.optionalString stdenv.isLinux (pkgs.lib.concatStringsSep "; " (builtins.map (b: "patchelf --add-rpath ${b.out}/lib $out/libexec/*") dynlibs));
    doCheck = false;

    checkFlags = lib.optionals stdenv.hostPlatform.isLinux [
      # Fails with "On 2823 Failed to find test1:A"
      "--skip=test_base_keymap"
      # Fails with "called `Result::unwrap()` on an `Err` value: Invalid keystroke `cmd-k`"
      # https://github.com/zed-industries/zed/issues/10427
      "--skip=test_disabled_keymap_binding"
    ];

    installPhase = ''
      runHook preInstall

      mkdir -p $out/bin $out/libexec
      cp target/${stdenv.hostPlatform.rust.cargoShortTarget}/release/zed $out/libexec/zed-editor
      cp target/${stdenv.hostPlatform.rust.cargoShortTarget}/release/cli $out/bin/zed

      install -D ${src}/crates/zed/resources/app-icon@2x.png $out/share/icons/hicolor/1024x1024@2x/apps/zed.png
      install -D ${src}/crates/zed/resources/app-icon.png $out/share/icons/hicolor/512x512/apps/zed.png

      # extracted from https://github.com/zed-industries/zed/blob/v0.141.2/script/bundle-linux (envsubst)
      # and https://github.com/zed-industries/zed/blob/v0.141.2/script/install.sh (final desktop file name)
      (
        export DO_STARTUP_NOTIFY="true"
        export APP_CLI="zed"
        export APP_ICON="zed"
        export APP_NAME="Zed"
        export APP_ARGS="%U"
        mkdir -p "$out/share/applications"
        ${lib.getExe pkgs.envsubst} < "crates/zed/resources/zed.desktop.in" > "$out/share/applications/dev.zed.Zed.desktop"
      )

      runHook postInstall
    '';

    meta = with lib; {
      description = "High-performance, multiplayer code editor from the creators of Atom and Tree-sitter";
      homepage = "https://zed.dev";
      changelog = "https://github.com/zed-industries/zed/releases/tag/v${version}";
      license = licenses.gpl3Only;
      maintainers = with maintainers; [
        GaetanLepage
        niklaskorz
      ];
      mainProgram = "zed";
      platforms = platforms.all;
      # Currently broken on darwin: https://github.com/NixOS/nixpkgs/pull/303233#issuecomment-2048650618
      broken = stdenv.isDarwin;
    };
  }
