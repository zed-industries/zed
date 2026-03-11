{ inputs, ... }:
{
  perSystem =
    { pkgs, ... }:
    let
      # NOTE: Duplicated because this is in a separate flake-parts partition
      # than ./packages.nix
      mkZed = import ../toolchain.nix { inherit inputs; };
      zed-editor = mkZed pkgs;

      rustBin = inputs.rust-overlay.lib.mkRustBin { } pkgs;
      rustToolchain = rustBin.fromRustupToolchainFile ../../rust-toolchain.toml;

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
    in
    {
      devShells.default = (pkgs.mkShell.override { inherit (zed-editor) stdenv; }) {
        name = "zed-editor-dev";
        inputsFrom = [ zed-editor ];

        packages = with pkgs; [
          wrappedCargo # must be first, to shadow the `cargo` provided by `rustToolchain`
          rustToolchain # cargo, rustc, and rust-toolchain.toml components included
          cargo-nextest
          cargo-hakari
          cargo-machete
          cargo-zigbuild
          # TODO: package protobuf-language-server for editing zed.proto
          # TODO: add other tools used in our scripts

          # `build.nix` adds this to the `zed-editor` wrapper (see `postFixup`)
          # we'll just put it on `$PATH`:
          nodejs_22
          zig
          
          # A11y testing script `script/a11y-inspector`
          gobject-introspection # shell hook that populates GI_TYPELIB_PATH
          at-spi2-core          # provides Atspi-2.0.typelib and DBus-1.0.typelib
          (python3.withPackages (ps: [ps.pyatspi ps.pygobject3]))
          accerciser
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
            ZED_ZSTD_MUSL_LIB = "${pkgs.pkgsCross.musl64.pkgsStatic.zstd.out}/lib";
            # For aws-lc-sys musl cross-compilation
            CC_x86_64_unknown_linux_musl = "${muslCross.stdenv.cc}/bin/x86_64-unknown-linux-musl-gcc";
          };
      };
    };
}
