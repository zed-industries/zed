{
  mkShell,
  makeFontsConf,
  lib,

  zed-editor,

  rust-analyzer,
  cargo-nextest,
  cargo-hakari,
  cargo-machete,
  nixfmt-rfc-style,
  protobuf,
  nodejs_22,
  cowsay,
}:
(mkShell.override { inherit (zed-editor) stdenv; }) {
  inputsFrom = [ zed-editor ];
  packages = [
    rust-analyzer
    cargo-nextest
    cargo-hakari
    cargo-machete
    nixfmt-rfc-style
    # TODO: package protobuf-language-server for editing zed.proto
    # TODO: add other tools used in our scripts

    # `build.nix` adds this to the `zed-editor` wrapper (see `postFixup`)
    # we'll just put it on `$PATH`:
    nodejs_22
  ];

  env =
    let
      baseEnvs =
        (zed-editor.overrideAttrs (attrs: {
          passthru = { inherit (attrs) env; };
        })).env; # exfil `env`; it's not in drvAttrs
    in
    (removeAttrs baseEnvs [
      "LK_CUSTOM_WEBRTC" # download the staticlib during the build as usual
      "ZED_UPDATE_EXPLANATION" # allow auto-updates
      "CARGO_PROFILE" # let you specify the profile
      "TARGET_DIR"
    ])
    // {
      # note: different than `$FONTCONFIG_FILE` in `build.nix` – this refers to relative paths
      # outside the nix store instead of to `$src`
      FONTCONFIG_FILE = makeFontsConf {
        fontDirectories = [
          "./assets/fonts/lilex"
          "./assets/fonts/ibm-plex-sans"
        ];
      };
      PROTOC = "${protobuf}/bin/protoc";
    };

    shellHook = lib.optionalString zed-editor.stdenv.hostPlatform.isDarwin ''
      set -x
      metal_compiler="$(env -u SDKROOT /usr/bin/xcrun -f metal)"
      if ! $("$metal_compiler" --help &> /dev/null); then
        {
          echo 'Leave the devshell and run `xcodebuild -downloadComponent MetalToolchain`'
          echo "to download the (proprietary 😤) metal compiler, or you'll need to pass"
          echo '`--features gpui/runtime_shaders` to cargo.'
        } |
        ${lib.getExe cowsay} -W 80
      else
        XCRUN_PATH=/usr/bin/xcrun
      fi;
    '';
}
