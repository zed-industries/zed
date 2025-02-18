{
  lib,
  mkShell,
  stdenv,
  stdenvAdapters,

  rustPlatform,
  makeFontsConf,
  rust-analyzer,
  nodejs_22,
  zed-editor,
}: let
  moldStdenv = stdenvAdapters.useMoldLinker stdenv;
  mkShell' = mkShell.override { stdenv = moldStdenv; };
in mkShell' {
  inputsFrom = [ zed-editor ];
  packages =
    [
      # rustPlatform.bindgenHook # uhhh why? shouldn't this be on the package?
      rust-analyzer

      # `build.nix` adds this to the `zed-editor` wrapper (see `postFixup`); we'll just put it
      # on `$PATH`:
      nodejs_22
    ];

  # todo: why
  # PROTOC="${pkgs.protobuf}/bin/protoc";

  # todo(julia): try without? (also fix indentation if keeping..)
  # We set SDKROOT and DEVELOPER_DIR to the Xcode ones instead of the nixpkgs ones,
  # because we need Swift 6.0 and nixpkgs doesn't have it.
  # Xcode is required for development anyways
  shellHook = lib.optionalString stdenv.hostPlatform.isDarwin ''
      export SDKROOT="/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk";
      export DEVELOPER_DIR="/Applications/Xcode.app/Contents/Developer";
     '';

  env = let
    # exfil `env`; it's not in drvAttrs
    baseEnvs = (zed-editor.overrideAttrs (attrs: { passthru = { inherit (attrs) env; }; })).env;
  in baseEnvs // {
    # note: different than `$FONTCONFIG_FILE` in `build.nix` – this refers to relative paths
    # outside the nix store instead of to `$src`
    FONTCONFIG_FILE = makeFontsConf {
      fontDirectories = [
        "./assets/fonts/zed-mono"
        "./assets/fonts/zed-sans"
      ];
    };

    # todo:
    #   - see if the vulkan lib is actually leading to an `-rpath` being passed
    #   - if not, why
    #   - if we can't get ^ to work: just pass in an `-rpath` manually via `env.NIX_LDFLAGS`
    #   - if that doesn't work... add to `LD_LIBRARY_PATH`
  };
}
