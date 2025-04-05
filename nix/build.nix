{
  lib,
  stdenv,

  apple-sdk_15,
  darwin,
  darwinMinVersionHook,

  cargo-about,
  cargo-bundle,
  crane,
  rustPlatform,
  rustToolchain,

  copyDesktopItems,
  envsubst,
  fetchFromGitHub,
  makeFontsConf,
  makeWrapper,

  alsa-lib,
  cmake,
  curl,
  fontconfig,
  freetype,
  git,
  libgit2,
  libglvnd,
  libxkbcommon,
  livekit-libwebrtc,
  nodejs_22,
  openssl,
  perl,
  pkg-config,
  protobuf,
  sqlite,
  vulkan-loader,
  wayland,
  xorg,
  zlib,
  zstd,

  pkgs, # TODO(nixpkgs-livekit-bump): temporary, should drop; see below

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
        ".cargo"
      ];
      firstComp = builtins.head (lib.path.subpath.components relPath);
    in
    builtins.elem firstComp topLevelIncludes;

  craneLib = crane.overrideToolchain rustToolchain;
  gpu-lib = if withGLES then libglvnd else vulkan-loader;
  commonArgs =
    let
      zedCargoLock = builtins.fromTOML (builtins.readFile ../crates/zed/Cargo.toml);
      stdenv' = stdenv;
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
          cmake
          copyDesktopItems
          curl
          perl
          pkg-config
          protobuf
          cargo-about
          rustPlatform.bindgenHook
        ]
        ++ lib.optionals stdenv'.hostPlatform.isLinux [ makeWrapper ]
        ++ lib.optionals stdenv'.hostPlatform.isDarwin [
          (cargo-bundle.overrideAttrs (
            new: old: {
              version = "0.6.1-zed";
              src = fetchFromGitHub {
                owner = "zed-industries";
                repo = "cargo-bundle";
                rev = "2be2669972dff3ddd4daf89a2cb29d2d06cad7c7";
                hash = "sha256-cSvW0ND148AGdIGWg/ku0yIacVgW+9f1Nsi+kAQxVrI=";
              };
              cargoHash = "sha256-urn+A3yuw2uAO4HGmvQnKvWtHqvG9KHxNCCWTiytE4k=";

              # NOTE: can drop once upstream uses `finalAttrs` here:
              # https://github.com/NixOS/nixpkgs/blob/10214747f5e6e7cb5b9bdf9e018a3c7b3032f5af/pkgs/build-support/rust/build-rust-package/default.nix#L104
              #
              # See (for context): https://github.com/NixOS/nixpkgs/pull/382550
              cargoDeps = rustPlatform.fetchCargoVendor {
                inherit (new) src;
                hash = new.cargoHash;
                patches = new.cargoPatches or [];
                name = new.cargoDepsName or new.finalPackage.name;
              };
            }
          ))
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
        ++ lib.optionals stdenv'.hostPlatform.isLinux [
          alsa-lib
          libxkbcommon
          wayland
          gpu-lib
          xorg.libxcb
          xorg.libX11
        ]
        ++ lib.optionals stdenv'.hostPlatform.isDarwin [
          apple-sdk_15
          darwin.apple_sdk.frameworks.System
          (darwinMinVersionHook "10.15")
        ];

      cargoExtraArgs = "-p zed -p cli --locked --features=gpui/runtime_shaders";

      stdenv =
        pkgs:
        let
          base = pkgs.llvmPackages.stdenv;
          addBinTools = old: {
            cc = old.cc.override {
              inherit (pkgs.llvmPackages) bintools;
            };
          };
          custom = lib.pipe base [
            (stdenv: stdenv.override addBinTools)
            pkgs.stdenvAdapters.useMoldLinker
          ];
        in
        if stdenv'.hostPlatform.isLinux then custom else base;

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

        # TODO(nixpkgs-livekit-bump): stopgap until livekit is updated to m125
        # in nixpkgs: https://github.com/NixOS/nixpkgs/pull/396016
        LK_CUSTOM_WEBRTC = livekit-libwebrtc.overrideAttrs (prev: {
          src = "${pkgs.callPackage ./webrtc-sources.nix {}}/src";

          # just so we don't forget...
          version = let
            msg = ''
              livekit-libwebrtc seems to have been updated in upstream, see if
              we can drop our patches?

              check: https://github.com/NixOS/nixpkgs/pull/396016
            '';
          in lib.warnIfNot (prev.version == "m114") msg "125-unstable-2025-03-04";

          patches = lib.pipe prev.patches [
            # drop: https://github.com/NixOS/nixpkgs/blob/2c8d3f48d33929642c1c12cd243df4cc7d2ce434/pkgs/by-name/li/livekit-libwebrtc/package.nix#L98
            # https://github.com/zed-industries/webrtc/commit/08f7a701a2eda6407670508fc2154257a3c90308
            (lib.ifilter0 (i: _: i != 3))
            # add: https://boringssl.googlesource.com/boringssl/+/c70190368c7040c37c1d655f0690bcde2b109a0d
            (p: p ++ [(pkgs.fetchpatch {
              url = "https://github.com/google/boringssl/commit/c70190368c7040c37c1d655f0690bcde2b109a0d.patch";
              hash = "sha256-xkmYulDOw5Ny5LOCl7rsheZSFbSF6md2NkZ3+azjFQk=";
              stripLen = 1;
              extraPrefix = "third_party/boringssl/src/";
            })])
          ];

          # alt: add `src/third_party/depot_tools/` to $PATH (for vpython3)
          nativeBuildInputs = prev.nativeBuildInputs ++ [
            (pkgs.runCommand "vpython3" { } "mkdir -p $out/bin && ln -s ${pkgs.python3}/bin/python $out/bin/vpython3")
          ];

          # note: `pc:peerconnection` is now `pc:peer_connection`
          ninjaFlags = builtins.map
            (flag: if flag == "pc:peerconnection" then "pc:peer_connection" else flag)
            prev.ninjaFlags;
        });

        CARGO_PROFILE = profile;
        # need to handle some profiles specially https://github.com/rust-lang/cargo/issues/11053
        TARGET_DIR = "target/" + (if profile == "dev" then "debug" else profile);

        # for some reason these deps being in buildInputs isn't enough, the only thing
        # about them that's special is that they're manually dlopened at runtime
        NIX_LDFLAGS = lib.optionalString stdenv'.hostPlatform.isLinux "-rpath ${
          lib.makeLibraryPath [
            gpu-lib
            wayland
          ]
        }";
      };

      # prevent nix from removing the "unused" wayland/gpu-lib rpaths
      dontPatchELF = stdenv'.hostPlatform.isLinux;

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
            postPatch =
              ''
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

    dontUseCmakeConfigure = true;

    # without the env var generate-licenses fails due to crane's fetchCargoVendor, see:
    # https://github.com/zed-industries/zed/issues/19971#issuecomment-2688455390
    # TODO: put this in a separate derivation that depends on src to avoid running it on every build
    preBuild = ''
      ALLOW_MISSING_LICENSES=yes bash script/generate-licenses
      echo nightly > crates/zed/RELEASE_CHANNEL
    '';

    # we can't set $RUSTFLAGS because that clobbers the cargo config
    # see https://github.com/rust-lang/cargo/issues/5376#issuecomment-2163350032
    postPatch = let
      glesConfig = builtins.toFile "config.toml" ''
        [target.'cfg(all())']
        rustflags = ["--cfg", "gles"]
      '';
    in lib.optionalString withGLES ''
      cat ${glesConfig} >> .cargo/config/config.toml
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
