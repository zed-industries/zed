{
  lib,
  mkShell,
  stdenv,
  stdenvAdapters,
  makeFontsConf,

  zed-editor,

  rust-analyzer,
  cargo-nextest,
  nixfmt-rfc-style,
  protobuf,
  nodejs_22,
}:
let
  moldStdenv = stdenvAdapters.useMoldLinker stdenv;
  mkShell' =
    if stdenv.hostPlatform.isLinux then mkShell.override { stdenv = moldStdenv; } else mkShell;
in
mkShell' {
  inputsFrom = [ zed-editor ];
  packages = [
    rust-analyzer
    cargo-nextest
    nixfmt-rfc-style
    # TODO: package protobuf-language-server for editing zed.proto
    # TODO: add other tools used in our scripts

    # `build.nix` adds this to the `zed-editor` wrapper (see `postFixup`)
    # we'll just put it on `$PATH`:
    nodejs_22
  ];

  # We set SDKROOT and DEVELOPER_DIR to the Xcode ones instead of the nixpkgs ones, because
  # we need Swift 6.0 and nixpkgs doesn't have it
  shellHook = lib.optionalString stdenv.hostPlatform.isDarwin ''
    export SDKROOT="/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk";
    export DEVELOPER_DIR="/Applications/Xcode.app/Contents/Developer";
  '';

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
          "./assets/fonts/plex-mono"
          "./assets/fonts/plex-sans"
        ];
      };
      PROTOC = "${protobuf}/bin/protoc";
    };
}
