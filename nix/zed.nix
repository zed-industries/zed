{
  pkgs,
  lib,
  stdenv,
  git,
  makeFontsConf,
  envsubst,
  rust-overlay,
  crane,
  crate2nix,
}:
let
  rustBin = rust-overlay.lib.mkRustBin { } pkgs;
  toolchain = rustBin.fromRustupToolchainFile ../rust-toolchain.toml;
  cargoHome = (crane.mkLib pkgs).vendorCargoDeps {
    cargoLock = ../Cargo.lock;
  };
  licenses =
    pkgs.runCommand "zed-licenses"
      {
        nativeBuildInputs = with pkgs; [
          cargo
          (callPackage ./cargo-about.nix { })
        ];
      }
      ''
        cd ${../.}
        CARGO_HOME=${cargoHome} \
        ALLOW_MISSING_LICENSES=yes \
        ${pkgs.bash}/bin/bash script/generate-licenses $out
      '';
  fontconfigFile = makeFontsConf {
    fontDirectories = [
      ../assets/fonts/lilex
      ../assets/fonts/ibm-plex-sans
    ];
  };
  tools = crate2nix.tools.${stdenv.system};
  cargoNix = tools.generatedCargoNix {
    name = "zed";
    src = ../.;
  };
  # cargoNix = ../Cargo.nix;
  workspace = (
    pkgs.callPackage cargoNix {
      buildRustCrateForPkgs =
        pkgs:
        pkgs.buildRustCrate.override {
          rustc = toolchain;
          cargo = toolchain;
          defaultCodegenUnits = 16;
          defaultCrateOverrides = pkgs.defaultCrateOverrides // (pkgs.callPackage ./overrides.nix { });
        };
    }
  );
in
{
  inherit workspace cargoNix;
  zed = pkgs.runCommand "zed" { } (
    if stdenv.hostPlatform.isDarwin then
      ''
        cp -r ${./..} .
        pushd crates/zed
        sed -i "s/package.metadata.bundle-nightly/package.metadata.bundle/" Cargo.toml
        export CARGO_BUNDLE_SKIP_BUILD=true
        app_path="$(cargo bundle --profile release | xargs)"
        popd

        mkdir -p $out/Applications $out/bin
        # Zed expects git next to its own binary
        ln -s ${git}/bin/git "$app_path/Contents/MacOS/git"
        mv ${workspace.workspaceMembers.cli.build}/bin/cli "$app_path/Contents/MacOS/cli"
        mv "$app_path" $out/Applications/

        # Physical location of the CLI must be inside the app bundle as this is used
        # to determine which app to start
        ln -s "$out/Applications/Zed Nightly.app/Contents/MacOS/cli" $out/bin/zed
      ''
    else
      ''
        mkdir -p $out/bin $out/libexec
        cp ${workspace.workspaceMembers.zed.build}/bin/zed $out/libexec/zed-editor
        cp ${workspace.workspaceMembers.cli.build}/bin/cli  $out/bin/zed
        ln -s $out/bin/zed $out/bin/zeditor  # home-manager expects the CLI binary to be here

        install -D "${../crates/zed/resources}/app-icon-nightly@2x.png" \
          "$out/share/icons/hicolor/1024x1024@2x/apps/zed.png"
        install -D ${../crates/zed/resources}/app-icon-nightly.png \
          $out/share/icons/hicolor/512x512/apps/zed.png

        # TODO: icons should probably be named "zed-nightly"
        (
          export DO_STARTUP_NOTIFY="true"
          export APP_CLI="zed"
          export APP_ICON="zed"
          export APP_NAME="Zed Nightly"
          export APP_ARGS="%U"
          mkdir -p "$out/share/applications"
          ${lib.getExe envsubst} < "${../crates/zed/resources/zed.desktop.in}" > "$out/share/applications/dev.zed.Zed-Nightly.desktop"
          chmod +x "$out/share/applications/dev.zed.Zed-Nightly.desktop"
        )
      ''
  );
}
