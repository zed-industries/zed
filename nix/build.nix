{
  lib,
  craneLib,
  rustPlatform,
  clang,
  llvmPackages_18,
  mold-wrapped,
  copyDesktopItems,
  curl,
  perl,
  pkg-config,
  protobuf,
  fontconfig,
  freetype,
  libgit2,
  openssl,
  sqlite,
  zlib,
  zstd,
  alsa-lib,
  libxkbcommon,
  wayland,
  libglvnd,
  xorg,
  makeFontsConf,
  vulkan-loader,
  envsubst,
  stdenvAdapters,
  nix-gitignore,
  withGLES ? false,
}: let
  includeFilter = path: type: let
    baseName = baseNameOf (toString path);
    parentDir = dirOf path;
    inRootDir = type == "directory" && parentDir == ../.;
  in
    !(inRootDir && (baseName == "docs" || baseName == ".github" || baseName == "script" || baseName == ".git" || baseName == "target"));

  src = lib.cleanSourceWith {
    src = nix-gitignore.gitignoreSource [] ../.;
    filter = includeFilter;
    name = "source";
  };

  stdenv = stdenvAdapters.useMoldLinker llvmPackages_18.stdenv;

  commonArgs =
    craneLib.crateNameFromCargoToml {cargoToml = ../crates/zed/Cargo.toml;}
    // {
      inherit src stdenv;

      nativeBuildInputs = [
        clang
        copyDesktopItems
        curl
        mold-wrapped
        perl
        pkg-config
        protobuf
        rustPlatform.bindgenHook
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

      ZSTD_SYS_USE_PKG_CONFIG = true;
      FONTCONFIG_FILE = makeFontsConf {
        fontDirectories = [
          "../assets/fonts/zed-mono"
          "../assets/fonts/zed-sans"
        ];
      };
      ZED_UPDATE_EXPLANATION = "zed has been installed using nix. Auto-updates have thus been disabled.";
    };

  cargoArtifacts = craneLib.buildDepsOnly commonArgs;

  gpu-lib =
    if withGLES
    then libglvnd
    else vulkan-loader;

  zed = craneLib.buildPackage (commonArgs
    // {
      inherit cargoArtifacts;
      cargoExtraArgs = "--package=zed --package=cli";
      buildFeatures = ["gpui/runtime_shaders"];
      doCheck = false;

      RUSTFLAGS =
        if withGLES
        then "--cfg gles"
        else "";

      postFixup = ''
        patchelf --add-rpath ${gpu-lib}/lib $out/libexec/*
        patchelf --add-rpath ${wayland}/lib $out/libexec/*
      '';

      postInstall = ''
        mkdir -p $out/bin $out/libexec
        mv $out/bin/zed $out/libexec/zed-editor
        mv $out/bin/cli $out/bin/zed

        install -D crates/zed/resources/app-icon@2x.png $out/share/icons/hicolor/1024x1024@2x/apps/zed.png
        install -D crates/zed/resources/app-icon.png $out/share/icons/hicolor/512x512/apps/zed.png

        export DO_STARTUP_NOTIFY="true"
        export APP_CLI="zed"
        export APP_ICON="zed"
        export APP_NAME="Zed"
        export APP_ARGS="%U"
        mkdir -p "$out/share/applications"
        ${lib.getExe envsubst} < "crates/zed/resources/zed.desktop.in" > "$out/share/applications/dev.zed.Zed.desktop"
      '';
    });
in
  zed
  // {
    meta = with lib; {
      description = "High-performance, multiplayer code editor from the creators of Atom and Tree-sitter";
      homepage = "https://zed.dev";
      changelog = "https://zed.dev/releases/preview";
      license = licenses.gpl3Only;
      mainProgram = "zed";
      platforms = platforms.linux;
    };
  }
