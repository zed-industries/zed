{ lib
, rustPlatform
, clang
, copyDesktopItems
, curl
, perl
, pkg-config
, protobuf
, fontconfig
, freetype
, libgit2
, openssl
, sqlite
, zlib
, zstd
, alsa-lib
, libxkbcommon
, wayland
, libglvnd
, xorg
, makeFontsConf
, vulkan-loader
, envsubst
, nix-gitignore
, stdenv
, toolchain

, withGLES ? false
}:

let
  workspaceLock = {
    lockFile = ../Cargo.lock;
    outputHashes = {
      "alacritty_terminal-0.24.1-dev" = "sha256-b4oSDhsAAYjpYGfFgA1Q1642JoJQ9k5RTsPgFUpAFmc=";
      "async-pipe-0.1.3" = "sha256-g120X88HGT8P6GNCrzpS5SutALx5H+45Sf4iSSxzctE=";
      "blade-graphics-0.4.0" = "sha256-3hK7BFoLTnzirf5QgCoCKJnl6iKcMxEIHA8ryT/LvtM=";
      "cosmic-text-0.11.2" = "sha256-TLPDnqixuW+aPAhiBhSvuZIa69vgV3xLcw32OlkdCcM=";
      "font-kit-0.14.1" = "sha256-qUKvmi+RDoyhMrZ7T6SoVAyMc/aasQ9Y/okzre4SzXo=";
      "lsp-types-0.95.1" = "sha256-N4MKoU9j1p/Xeowki/+XiNQPwIcTm9DgmfM/Eieq4js=";
      "nvim-rs-0.8.0-pre" = "sha256-VA8zIynflul1YKBlSxGCXCwa2Hz0pT3mH6OPsfS7Izo=";
      "tree-sitter-0.22.6" = "sha256-P9pQcofDCIhOYWA1OC8TzB5UgWpD5GlDzX2DOS8SsH0=";
      "tree-sitter-gomod-1.0.2" = "sha256-/sjC117YAFniFws4F/8+Q5Wrd4l4v4nBUaO9IdkixSE=";
      "tree-sitter-gowork-0.0.1" = "sha256-803ujH5qwejQ2vQDDpma4JDC9a+vFX8ZQmr+77VyL2M=";
      "tree-sitter-heex-0.0.1" = "sha256-VakMZtWQ/h7dNy5ehk2Bh14a5s878AUgwY3Ipq8tPec=";
      "tree-sitter-md-0.2.3" = "sha256-Fa73P1h5GvKV3SxXr0KzHuNp4xa5wxUzI8ecXbGdrYE=";
      "xim-0.4.0" = "sha256-vxu3tjkzGeoRUj7vyP0vDGI7fweX8Drgy9hwOUOEQIA=";
      "xkbcommon-0.7.0" = "sha256-2RjZWiAaz8apYTrZ82qqH4Gv20WyCtPT+ldOzm0GWMo=";
    };
  };
  zedCargoToml = builtins.fromTOML (builtins.readFile ../crates/zed/Cargo.toml);
  version = zedCargoToml.package.version;
  src = nix-gitignore.gitignoreSource [] ../.;

in
rustPlatform.buildRustPackage rec {
  pname = "zed-editor";
  inherit version src;

  cargoLock = workspaceLock;

  nativeBuildInputs = [
    clang
    copyDesktopItems
    curl
    perl
    pkg-config
    protobuf
    rustPlatform.bindgenHook
    toolchain
  ];

  buildInputs = [
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

  cargoBuildFlags = [
    "--package=zed"
    "--package=cli"
  ];
  buildFeatures = [ "gpui/runtime_shaders" ];

  env = {
    ZSTD_SYS_USE_PKG_CONFIG = true;
    FONTCONFIG_FILE = makeFontsConf {
      fontDirectories = [
        "../assets/fonts/zed-mono"
        "../assets/fonts/zed-sans"
      ];
    };
    ZED_UPDATE_EXPLANATION = "zed has been installed using nix. Auto-updates have thus been disabled.";
  };

  RUSTFLAGS = if withGLES then "--cfg gles" else "";
  gpu-lib = if withGLES then libglvnd else vulkan-loader;

  postFixup = ''
    patchelf --add-rpath ${gpu-lib}/lib $out/libexec/*
    patchelf --add-rpath ${wayland}/lib $out/libexec/*
  '';

  preCheck = ''
    export HOME=$(mktemp -d);
  '';

  checkFlags = [
    "--skip=test_open_paths_action"
    "--skip=zed::tests::test_window_edit_state_restoring_enabled"
  ];

  installPhase = ''
    runHook preInstall

    mkdir -p $out/bin $out/libexec
    cp target/${stdenv.hostPlatform.rust.cargoShortTarget}/release/zed $out/libexec/zed-editor
    cp target/${stdenv.hostPlatform.rust.cargoShortTarget}/release/cli $out/bin/zed

    install -D crates/zed/resources/app-icon@2x.png $out/share/icons/hicolor/1024x1024@2x/apps/zed.png
    install -D crates/zed/resources/app-icon.png $out/share/icons/hicolor/512x512/apps/zed.png

    (
      export DO_STARTUP_NOTIFY="true"
      export APP_CLI="zed"
      export APP_ICON="zed"
      export APP_NAME="Zed"
      export APP_ARGS="%U"
      mkdir -p "$out/share/applications"
      ${lib.getExe envsubst} < "crates/zed/resources/zed.desktop.in" > "$out/share/applications/dev.zed.Zed.desktop"
    )

    runHook postInstall
  '';

  meta = {
    description = "High-performance, multiplayer code editor from the creators of Atom and Tree-sitter";
    homepage = "https://zed.dev";
    changelog = "https://zed.dev/releases/preview";
    license = lib.licenses.gpl3Only;
    mainProgram = "zed";
    platforms = lib.platforms.linux;
  };
}
