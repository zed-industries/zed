{
  pkgs ? import <nixpkgs> {},
  fenix ? import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") {},
}: let
  inherit (pkgs) lib;
  stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.llvmPackages_18.stdenv;
  toolchain = fenix.fromToolchainName {
    name = (lib.importTOML ./../../rust-toolchain.toml).toolchain.channel;
    sha256 = "sha256-6eN/GKzjVSjEhGO9FhWObkRFaE1Jf+uqMSdQnb8lcB4=";
  };
  rustPlatform = pkgs.makeRustPlatform {
    inherit (toolchain) cargo rustc;
    inherit stdenv;
  };
  dynlibs = with pkgs; lib.optionals stdenv.isLinux [vulkan-loader];
  version = (lib.importTOML ./../../crates/zed/Cargo.toml).package.version;
in
  rustPlatform.buildRustPackage rec {
    pname = "zed-editor";
    inherit version;

    src = ./../..;

    patches = [
      ./hardcode_nodejs.patch
    ];

    # Nix needs to configure cargo in order to correctly link in dependencies and vendored crates. It currently uses
    # the legacy `./.cargo/config` to do this. config.toml will take precedence, so remove it.
    prePatch = ''
      rm ./.cargo/config.toml
    '';

    nativeBuildInputs = with pkgs;
      [
        copyDesktopItems
        curl
        perl
        pkg-config
        protobuf
        rustPlatform.bindgenHook
      ]
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

      outputHashes = lib.importJSON ./pins.json;
    };

    cargoBuildFlags = [
      "--package=zed"
      "--package=cli"
    ];

    RUSTFLAGS = "-C symbol-mangling-version=v0 --cfg tokio_unstable";
    buildFeatures = ["gpui/runtime_shaders" "mimalloc"];

    env = {
      ZSTD_SYS_USE_PKG_CONFIG = true;
      OPENSSL_NO_VENDOR = 1;
      FONTCONFIG_FILE = pkgs.makeFontsConf {
        fontDirectories = [
          "${src}/assets/fonts/zed-mono"
          "${src}/assets/fonts/zed-sans"
        ];
      };
      NODE_PATH = "${pkgs.nodejs_22}";
    };

    postFixup = pkgs.lib.concatStringsSep "; " (builtins.map
      (b: "patchelf --add-rpath ${b.out}/lib $out/libexec/*")
      dynlibs);

    preCheck = ''
      # the check phase seems to be ignoring the linking parameters
      export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath (buildInputs ++ dynlibs)}:''${LD_LIBRARY_PATH:-}

      # Nix creates an inaccessible homedir because it doesn't make sense for a build to interact with one. It does
      # make sense for tests, though.
      export HOME=$(mktemp -d)
    '';

    checkFlags = lib.optionals stdenv.hostPlatform.isLinux [
      # Fails with "On 2823 Failed to find test1:A"
      "--skip=test_base_keymap"
      # Fails with "called `Result::unwrap()` on an `Err` value: Invalid keystroke `cmd-k`"
      # https://github.com/zed-industries/zed/issues/10427
      "--skip=test_disabled_keymap_binding"
      # Fails with "FOREIGN KEY constraint failed" in the logs
      "--skip=test_window_edit_state_restoring_enabled"
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
      mainProgram = "zed";
      platforms = platforms.all;
      # Currently broken on darwin: https://github.com/NixOS/nixpkgs/pull/303233#issuecomment-2048650618
      broken = stdenv.isDarwin;
    };
  }
