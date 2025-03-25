{
  lib,
  crane,
  rustToolchain,
  rustPlatform,
  cmake,
  copyDesktopItems,
  fetchFromGitHub,
  curl,
  clang,
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
  livekit-libwebrtc,
  apple-sdk_15,
  darwin,
  darwinMinVersionHook,
  makeWrapper,
  nodejs_22,

  withGLES ? false,
  profile ? "release",
}:

assert withGLES -> stdenv.hostPlatform.isLinux;

let
  mkIncludeFilter =
    root': path: type:
    let
      # note: under lazy-trees this introduces an extra copy
      root = toString root' + "/";
      relPath = lib.removePrefix root path;
      topLevelIncludes = [
        "crates"
        "assets"
        "extensions"
        "script"
        "tooling"
        "Cargo.toml"
        ".config" # nextest?
      ];
      firstComp = builtins.head (lib.path.subpath.components relPath);
    in
    builtins.elem firstComp topLevelIncludes;

  craneLib = crane.overrideToolchain rustToolchain;
  gpu-lib = if withGLES then libglvnd else vulkan-loader;
  commonArgs =
    let
      zedCargoLock = builtins.fromTOML (builtins.readFile ../crates/zed/Cargo.toml);
    in
    rec {
      pname = "zed-editor";
      version = zedCargoLock.package.version + "-nightly";
      src = builtins.path {
        path = ../.;
        filter = mkIncludeFilter ../.;
        name = "source";
      };

      cargoLock = ../Cargo.lock;

      nativeBuildInputs =
        [
          clang # TODO: use pkgs.clangStdenv or ignore cargo config?
          cmake
          copyDesktopItems
          curl
          perl
          pkg-config
          protobuf
          cargo-about
          rustPlatform.bindgenHook
        ]
        ++ lib.optionals stdenv.hostPlatform.isLinux [ makeWrapper ]
        ++ lib.optionals stdenv.hostPlatform.isDarwin [
          # TODO: move to overlay so it's usable in the shell
          (cargo-bundle.overrideAttrs (old: {
            version = "0.6.0-zed";
            src = fetchFromGitHub {
              owner = "zed-industries";
              repo = "cargo-bundle";
              rev = "zed-deploy";
              hash = "sha256-OxYdTSiR9ueCvtt7Y2OJkvzwxxnxu453cMS+l/Bi5hM=";
            };
          }))
        ];

      buildInputs =
        [
          curl
          fontconfig
          freetype
          # TODO: need staticlib of this for linking the musl remote server.
          # should make it a separate derivation/flake output
          # see https://crane.dev/examples/cross-musl.html
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
          gpu-lib
          xorg.libxcb
        ]
        ++ lib.optionals stdenv.hostPlatform.isDarwin [
          apple-sdk_15
          darwin.apple_sdk.frameworks.System
          (darwinMinVersionHook "10.15")
        ];

      cargoExtraArgs = "-p zed -p cli --locked --features=gpui/runtime_shaders";

      env = {
        ZSTD_SYS_USE_PKG_CONFIG = true;
        FONTCONFIG_FILE = makeFontsConf {
          fontDirectories = [
            ../assets/fonts/plex-mono
            ../assets/fonts/plex-sans
          ];
        };
        ZED_UPDATE_EXPLANATION = "Zed has been installed using Nix. Auto-updates have thus been disabled.";
        RELEASE_VERSION = version;
        RUSTFLAGS = if withGLES then "--cfg gles" else "";
        LK_CUSTOM_WEBRTC = livekit-libwebrtc;

        CARGO_PROFILE = profile;
        # need to handle some profiles specially https://github.com/rust-lang/cargo/issues/11053
        TARGET_DIR = "target/" + (if profile == "dev" then "debug" else profile);

        # for some reason these deps being in buildInputs isn't enough, the only thing
        # about them that's special is that they're manually dlopened at runtime
        NIX_LDFLAGS = lib.optionalString stdenv.hostPlatform.isLinux "-rpath ${
          lib.makeLibraryPath [
            gpu-lib
            wayland
          ]
        }";
      };

      # prevent nix from removing the "unused" wayland/gpu-lib rpaths
      dontPatchELF = stdenv.hostPlatform.isLinux;

      # TODO: try craneLib.cargoNextest separate output
      # for now we're not worried about running our test suite (or tests for deps) in the nix sandbox
      doCheck = false;

      cargoVendorDir = craneLib.vendorCargoDeps {
        inherit src cargoLock;
        overrideVendorGitCheckout =
          let
            hasWebRtcSys = builtins.any (crate: crate.name == "webrtc-sys");
            # `webrtc-sys` expects a staticlib; nixpkgs' `livekit-webrtc` has been patched to
            # produce a `dylib`... patching `webrtc-sys`'s build script is the easier option
            # TODO: send livekit sdk a PR to make this configurable
            postPatch = ''
              substituteInPlace webrtc-sys/build.rs --replace-fail \
                "cargo:rustc-link-lib=static=webrtc" "cargo:rustc-link-lib=dylib=webrtc"
            '';
          in
          crates: drv:
          if hasWebRtcSys crates then
            drv.overrideAttrs (o: {
              postPatch = (o.postPatch or "") + postPatch;
            })
          else
            drv;
      };
    };
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
in
craneLib.buildPackage (
  lib.recursiveUpdate commonArgs {
    inherit cargoArtifacts;

    patches = lib.optionals stdenv.hostPlatform.isDarwin [
      # Livekit requires Swift 6
      # We need this until livekit-rust sdk is used
      ../script/patches/use-cross-platform-livekit.patch
    ];

    dontUseCmakeConfigure = true;

    # without the env var generate-licenses fails due to crane's fetchCargoVendor, see:
    # https://github.com/zed-industries/zed/issues/19971#issuecomment-2688455390
    # TODO: put this in a separate derivation that depends on src to avoid running it on every build
    preBuild = ''
      ALLOW_MISSING_LICENSES=yes bash script/generate-licenses
      echo nightly > crates/zed/RELEASE_CHANNEL
    '';

    installPhase =
      if stdenv.hostPlatform.isDarwin then
        ''
          runHook preInstall

          pushd crates/zed
          sed -i "s/package.metadata.bundle-nightly/package.metadata.bundle/" Cargo.toml
          export CARGO_BUNDLE_SKIP_BUILD=true
          app_path="$(cargo bundle --profile $CARGO_PROFILE | xargs)"
          popd

          mkdir -p $out/Applications $out/bin
          # Zed expects git next to its own binary
          ln -s ${git}/bin/git "$app_path/Contents/MacOS/git"
          mv $TARGET_DIR/cli "$app_path/Contents/MacOS/cli"
          mv "$app_path" $out/Applications/

          # Physical location of the CLI must be inside the app bundle as this is used
          # to determine which app to start
          ln -s "$out/Applications/Zed Nightly.app/Contents/MacOS/cli" $out/bin/zed

          runHook postInstall
        ''
      else
        ''
          runHook preInstall

          mkdir -p $out/bin $out/libexec
          cp $TARGET_DIR/zed $out/libexec/zed-editor
          cp $TARGET_DIR/cli $out/bin/zed

          install -D "crates/zed/resources/app-icon-nightly@2x.png" \
            "$out/share/icons/hicolor/1024x1024@2x/apps/zed.png"
          install -D crates/zed/resources/app-icon-nightly.png \
            $out/share/icons/hicolor/512x512/apps/zed.png

          # TODO: icons should probably be named "zed-nightly"
          (
            export DO_STARTUP_NOTIFY="true"
            export APP_CLI="zed"
            export APP_ICON="zed"
            export APP_NAME="Zed Nightly"
            export APP_ARGS="%U"
            mkdir -p "$out/share/applications"
            ${lib.getExe envsubst} < "crates/zed/resources/zed.desktop.in" > "$out/share/applications/dev.zed.Zed-Nightly.desktop"
          )

          runHook postInstall
        '';

    # TODO: why isn't this also done on macOS?
    postFixup = lib.optionalString stdenv.hostPlatform.isLinux ''
      wrapProgram $out/libexec/zed-editor --suffix PATH : ${lib.makeBinPath [ nodejs_22 ]}
    '';

    meta = {
      description = "High-performance, multiplayer code editor from the creators of Atom and Tree-sitter";
      homepage = "https://zed.dev";
      changelog = "https://zed.dev/releases/preview";
      license = lib.licenses.gpl3Only;
      mainProgram = "zed";
      platforms = lib.platforms.linux ++ lib.platforms.darwin;
    };
  }
)
