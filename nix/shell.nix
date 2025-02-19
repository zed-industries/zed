{
  lib,
  mkShell,
  stdenv,
  stdenvAdapters,

  zed-editor,

  makeFontsConf,
  rust-analyzer,
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
    nixfmt-rfc-style
    # TODO: package protobuf-language-server

    # `build.nix` adds this to the `zed-editor` wrapper (see `postFixup`); we'll just put it
    # on `$PATH`:
    nodejs_22
  ];

  # todo(julia): try removing, this is maybe handled by stdenv wrapped linker?
  # We set SDKROOT and DEVELOPER_DIR to the Xcode ones instead of the nixpkgs ones,
  # because we need Swift 6.0 and nixpkgs doesn't have it.
  # Xcode is required for development anyways
  shellHook = lib.optionalString stdenv.hostPlatform.isDarwin ''
    export SDKROOT="/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk";
    export DEVELOPER_DIR="/Applications/Xcode.app/Contents/Developer";
  '';

  env =
    let
      # exfil `env`; it's not in drvAttrs
      baseEnvs =
        (zed-editor.overrideAttrs (attrs: {
          passthru = { inherit (attrs) env; };
        })).env;
    in
    baseEnvs
    // {
      # note: different than `$FONTCONFIG_FILE` in `build.nix` – this refers to relative paths
      # outside the nix store instead of to `$src`
      FONTCONFIG_FILE = makeFontsConf {
        fontDirectories = [
          "./assets/fonts/plex-mono"
          "./assets/fonts/plex-sans"
        ];
      };
      PROTOC = "${protobuf}/bin/protoc"; # needed for crates/proto
      # LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs; # TODO: try this?
    };
}
