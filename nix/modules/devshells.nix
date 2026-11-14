{ inputs, ... }:
{
  perSystem =
    { pkgs, system, ... }:
    let
      # NOTE: Duplicated because this is in a separate flake-parts partition
      # than ./packages.nix
      mkZed = import ../toolchain.nix { inherit inputs; };
      zed-editor = mkZed pkgs;

      # mdBook pinned to 0.4.40 via a dedicated nixpkgs input, because the docs
      # rely on behavior that newer mdBook releases break (see
      # `crates/docs_preprocessor/Cargo.toml`).
      mdbook = (import inputs.nixpkgs-mdbook { inherit system; }).mdbook;

      # Prebuilt docs preprocessor/postprocessor binary. `docs/book.toml`
      # defaults to `cargo run -p docs_preprocessor` so non-Nix contributors are
      # unaffected; in the devshell we point mdBook at this prebuilt binary via
      # the `MDBOOK_*` env vars below so `mdbook build docs` doesn't have to
      # compile the preprocessor on every run.
      #
      # We reuse `zed-editor`'s crane builder and shared arguments (exposed via
      # `passthru`) rather than `overrideAttrs`, because crane bakes
      # `cargoExtraArgs` into the build command at evaluation time.
      docs-preprocessor = zed-editor.passthru.craneLib.buildPackage (
        zed-editor.passthru.commonArgs
        // {
          inherit (zed-editor.passthru) cargoArtifacts;
          pname = "zed-docs-preprocessor";
          cargoExtraArgs = "-p docs_preprocessor --locked";
          dontUseCmakeConfigure = true;
          meta = {
            description = "mdBook preprocessor and postprocessor for the Zed docs";
            mainProgram = "docs-preprocessor";
          };
        }
      );

      rustBin = inputs.rust-overlay.lib.mkRustBin { } pkgs;
      rustToolchain = rustBin.fromRustupToolchainFile ../../rust-toolchain.toml;

      corgi = (import "${inputs.corgi-src}/nix/toolchain.nix" { inherit inputs; }) pkgs;

      baseEnv =
        (zed-editor.overrideAttrs (attrs: {
          passthru.env = attrs.env;
        })).env; # exfil `env`; it's not in drvAttrs

      # Musl cross-compiler for building remote_server
      muslCross = pkgs.pkgsCross.musl64;

      # Cargo build timings wrapper script
      wrappedCargo = pkgs.writeShellApplication {
        name = "cargo";
        runtimeInputs = [ pkgs.nodejs ];
        text =
          let
            pathToCargoScript = ./. + "/../../script/cargo";
          in
          ''
            NIX_WRAPPER=1 CARGO=${rustToolchain}/bin/cargo ${pathToCargoScript} "$@"
          '';
      };

      defaultShell = (pkgs.mkShell.override { inherit (zed-editor) stdenv; }) {
        name = "zed-editor-dev";
        inputsFrom = [ zed-editor ];

        shellHook = pkgs.lib.optionalString pkgs.stdenv.hostPlatform.isLinux ''
          # The pkg-config setup hook replaces PKG_CONFIG with a relative command,
          # which corgi's sandboxed build scripts cannot resolve.
          export PKG_CONFIG="${pkgs.pkg-config}/bin/pkg-config"
        '';

        packages =
          with pkgs;
          [
            wrappedCargo # must be first, to shadow the `cargo` provided by `rustToolchain`
            rustToolchain # cargo, rustc, and rust-toolchain.toml components included
            cargo-nextest
            cargo-hakari
            cargo-shear
            cargo-zigbuild
            # TODO: package protobuf-language-server for editing zed.proto
            # TODO: add other tools used in our scripts

            # `build.nix` adds this to the `zed-editor` wrapper (see `postFixup`)
            # we'll just put it on `$PATH`:
            nodejs_22
            zig

            # Documentation tooling: `nix develop -c mdbook build docs`
            mdbook
            docs-preprocessor

            # A11y testing infra
            gobject-introspection
            at-spi2-core
            (python3.withPackages (ps: [
              ps.pyatspi
              ps.pygobject3
            ]))
          ]
          ++ lib.optionals stdenv.hostPlatform.isLinux [
            accerciser
            corgi
          ];

        env =
          (removeAttrs baseEnv [
            "LK_CUSTOM_WEBRTC" # download the staticlib during the build as usual
            "ZED_UPDATE_EXPLANATION" # allow auto-updates
            "CARGO_PROFILE" # let you specify the profile
            "TARGET_DIR"
          ])
          // {
            # note: different than `$FONTCONFIG_FILE` in `build.nix` – this refers to relative paths
            # outside the nix store instead of to `$src`
            FONTCONFIG_FILE = pkgs.makeFontsConf {
              fontDirectories = [
                "./assets/fonts/lilex"
                "./assets/fonts/ibm-plex-sans"
              ];
            };
            PROTOC = "${pkgs.protobuf}/bin/protoc";

            # Point mdBook at the prebuilt preprocessor/postprocessor binary
            # instead of `cargo run`. mdBook lowercases these keys and turns `_`
            # into `-`, so they map to `preprocessor.zed-docs-preprocessor.command`
            # and `output.zed-html.command` in `docs/book.toml`.
            MDBOOK_PREPROCESSOR__ZED_DOCS_PREPROCESSOR__COMMAND = "${docs-preprocessor}/bin/docs-preprocessor";
            MDBOOK_OUTPUT__ZED_HTML__COMMAND = "${docs-preprocessor}/bin/docs-preprocessor postprocess";

            ZED_ZSTD_MUSL_LIB = "${pkgs.pkgsCross.musl64.pkgsStatic.zstd.out}/lib";
            # For aws-lc-sys musl cross-compilation
            CC_x86_64_unknown_linux_musl = "${muslCross.stdenv.cc}/bin/x86_64-unknown-linux-musl-gcc";
          }
          // pkgs.lib.optionalAttrs pkgs.stdenv.hostPlatform.isLinux {
            # The cmake that corgi.toml downloads for Linux expects a standard
            # dynamic loader, which NixOS does not have.
            CMAKE = "${pkgs.cmake}/bin/cmake";
            CORGI_RUST_TOOLCHAIN = "${rustToolchain}";
            CORGI_CC = "${zed-editor.stdenv.cc}/bin/clang";
            CORGI_NATIVE_RUNTIME_ROOTS = pkgs.lib.concatStringsSep ":" [
              "${pkgs.pkg-config}"
              "${pkgs.cmake}"
              # The hook references libclang and the compiler inputs in its flags.
              "${pkgs.rustPlatform.bindgenHook}"
            ];
            # Dev shells have no installed output; its rpath would embed the checkout.
            NIX_NO_SELF_RPATH = "1";
          };
      };
    in
    {
      devShells = {
        default = defaultShell;
      }
      // pkgs.lib.optionalAttrs pkgs.stdenv.hostPlatform.isLinux {
        corgi = defaultShell;
      };
    };
}
