{
  lib,
  fetchpatch,
  nix-gitignore,
  rustPlatform,
  cmake,
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
  stdenv,
  makeFontsConf,
  vulkan-loader,
  envsubst,
  cargo-about,
  cargo-bundle,
  git,
  apple-sdk_15,
  darwinMinVersionHook,
  makeWrapper,
  nodejs,
  libGL,
  libX11,
  libXext,
  livekit-libwebrtc,
  writableTmpDirAsHomeHook,
  zed-editor,

  version ? "unknown-nightly",
  buildType ? "release",
  withGLES ? false,
  buildRemoteServer ? true,
}:

assert withGLES -> stdenv.hostPlatform.isLinux;

rustPlatform.buildRustPackage rec {
  pname = "zed-editor";
  inherit version buildType;

  outputs = [ "out" ] ++ lib.optional buildRemoteServer "remote_server";

  src = ../.;

  patches = [
    # Zed uses cargo-install to install cargo-about during the script execution.
    # We provide cargo-about ourselves and can skip this step.
    # Until https://github.com/zed-industries/zed/issues/19971 is fixed,
    # we also skip any crate for which the license cannot be determined.
    (fetchpatch {
      url = "https://raw.githubusercontent.com/NixOS/nixpkgs/1fd02d90c6c097f91349df35da62d36c19359ba7/pkgs/by-name/ze/zed-editor/0001-generate-licenses.patch";
      hash = "sha256-cLgqLDXW1JtQ2OQFLd5UolAjfy7bMoTw40lEx2jA2pk=";
    })
    # See https://github.com/zed-industries/zed/pull/21661#issuecomment-2524161840
    "script/patches/use-cross-platform-livekit.patch"
  ];

  # Dynamically link WebRTC instead of static
  postPatch = ''
    substituteInPlace ../cargo-vendor-dir/webrtc-sys-*/build.rs \
      --replace-fail "cargo:rustc-link-lib=static=webrtc" "cargo:rustc-link-lib=dylib=webrtc"
  '';

  cargoLock = {
    lockFile = ../Cargo.lock;
    allowBuiltinFetchGit = true;
  };

  nativeBuildInputs =
    [
      cmake
      copyDesktopItems
      curl
      perl
      pkg-config
      protobuf
      rustPlatform.bindgenHook
      cargo-about
    ]
    ++ lib.optionals stdenv.hostPlatform.isLinux [ makeWrapper ]
    ++ lib.optionals stdenv.hostPlatform.isDarwin [ cargo-bundle ];

  dontUseCmakeConfigure = true;

  buildInputs =
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
    ++ lib.optionals stdenv.hostPlatform.isLinux [
      alsa-lib
      libxkbcommon
      wayland
      xorg.libxcb
      # required by livekit:
      libGL
      libX11
      libXext
    ]
    ++ lib.optionals stdenv.hostPlatform.isDarwin [
      apple-sdk_15
      # ScreenCaptureKit, required by livekit, is only available on 12.3 and up:
      # https://developer.apple.com/documentation/screencapturekit
      (darwinMinVersionHook "12.3")
    ];

  cargoBuildFlags = [
    "--package=zed"
    "--package=cli"
  ] ++ lib.optional buildRemoteServer "--package=remote_server";

  # Required on darwin because we don't have access to the
  # proprietary Metal shader compiler.
  buildFeatures = lib.optionals stdenv.hostPlatform.isDarwin [ "gpui/runtime_shaders" ];

  env = {
    ZSTD_SYS_USE_PKG_CONFIG = true;
    FONTCONFIG_FILE = makeFontsConf {
      fontDirectories = [
        "${src}/assets/fonts/plex-mono"
        "${src}/assets/fonts/plex-sans"
      ];
    };
    # Setting this environment variable allows to disable auto-updates
    # https://zed.dev/docs/development/linux#notes-for-packaging-zed
    ZED_UPDATE_EXPLANATION = "Zed has been installed using Nix. Auto-updates have thus been disabled.";
    # Used by `zed --version`
    RELEASE_VERSION = version;
    LK_CUSTOM_WEBRTC = livekit-libwebrtc;
  };

  RUSTFLAGS = if withGLES then "--cfg gles" else "";
  gpu-lib = if withGLES then libglvnd else vulkan-loader;

  preBuild = ''
    bash script/generate-licenses
  '';

  postFixup = lib.optionalString stdenv.hostPlatform.isLinux ''
    patchelf --add-rpath ${gpu-lib}/lib $out/libexec/*
    patchelf --add-rpath ${wayland}/lib $out/libexec/*
    wrapProgram $out/libexec/zed-editor --suffix PATH : ${lib.makeBinPath [ nodejs ]}
  '';

  nativeCheckInputs = [
    writableTmpDirAsHomeHook
  ];

  checkFlags =
    [
      # Flaky: unreliably fails on certain hosts (including Hydra)
      "--skip=zed::tests::test_window_edit_state_restoring_enabled"
    ]
    ++ lib.optionals stdenv.hostPlatform.isLinux [
      # Fails on certain hosts (including Hydra) for unclear reason
      "--skip=test_open_paths_action"
    ];

  installPhase =
    ''
      runHook preInstall

      release_target="target/${stdenv.hostPlatform.rust.cargoShortTarget}/${buildType}"
    ''
    + lib.optionalString stdenv.hostPlatform.isDarwin ''
      # cargo-bundle expects the binary in target/release
      mv $release_target/zed target/${buildType}/zed

      pushd crates/zed

      # Note that this is GNU sed, bundle-mac uses BSD sed
      sed -i "s/package.metadata.bundle-stable/package.metadata.bundle/" Cargo.toml
      export CARGO_BUNDLE_SKIP_BUILD=true
      app_path=$(cargo bundle ${if buildType == "release" then "release" else ""} | xargs)

      # We're not using Zed's fork of cargo-bundle, so we must manually append their plist extensions
      # Remove closing tags from Info.plist (last two lines)
      head -n -2 $app_path/Contents/Info.plist > Info.plist
      # Append extensions
      cat resources/info/*.plist >> Info.plist
      # Add closing tags
      printf "</dict>\n</plist>\n" >> Info.plist
      mv Info.plist $app_path/Contents/Info.plist

      popd

      mkdir -p $out/Applications $out/bin
      # Zed expects git next to its own binary
      ln -s ${lib.getExe git} $app_path/Contents/MacOS/git
      mv $release_target/cli $app_path/Contents/MacOS/cli
      mv $app_path $out/Applications/

      # Physical location of the CLI must be inside the app bundle as this is used
      # to determine which app to start
      ln -s $out/Applications/Zed.app/Contents/MacOS/cli $out/bin/zed
    ''
    + lib.optionalString stdenv.hostPlatform.isLinux ''
      install -Dm755 $release_target/zed $out/libexec/zed-editor
      install -Dm755 $release_target/cli $out/bin/zed

      install -Dm644 ${src}/crates/zed/resources/app-icon@2x.png $out/share/icons/hicolor/1024x1024@2x/apps/zed.png
      install -Dm644 ${src}/crates/zed/resources/app-icon.png $out/share/icons/hicolor/512x512/apps/zed.png

      # extracted from https://github.com/zed-industries/zed/blob/v0.141.2/script/bundle-linux (envsubst)
      # and https://github.com/zed-industries/zed/blob/v0.141.2/script/install.sh (final desktop file name)
      (
        export DO_STARTUP_NOTIFY="true"
        export APP_CLI="zed"
        export APP_ICON="zed"
        export APP_NAME="Zed"
        export APP_ARGS="%U"
        mkdir -p "$out/share/applications"
        ${lib.getExe envsubst} < "crates/zed/resources/zed.desktop.in" > "$out/share/applications/dev.zed.Zed.desktop"
      )
    ''
    + lib.optionalString buildRemoteServer ''
      install -Dm755 $release_target/remote_server $remote_server/bin/zed-remote-server-stable-$version
    ''
    + ''
      runHook postInstall
    '';

  doInstallCheck = false;

  passthru = {
    debug = zed-editor.override { buildType = "debug"; };
  };

  meta = {
    description = "High-performance, multiplayer code editor from the creators of Atom and Tree-sitter";
    homepage = "https://zed.dev";
    changelog = "https://github.com/zed-industries/zed/releases/tag/v${version}";
    license = lib.licenses.gpl3Only;
    mainProgram = "zed";
    platforms = lib.platforms.linux ++ lib.platforms.darwin;
  };
}
