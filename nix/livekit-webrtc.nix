{
  pkgs,
  system,
  ...
}:

let
  releases = {
    "0001d84-2" = {
      "aarch64-darwin" = {
        os = "mac";
        arch = "arm64";
        hash = "sha256-8czUkr2djlBon75TMjErSpX47W654ofCtz2UzcaMuXQ=";
      };

      "x86_64-linux" = {
        os = "linux";
        arch = "x64";
        hash = pkgs.lib.fakeHash;
      };
    };
  };

  artifacts =
    system: version:
    let
      inherit (releases.${version}.${system}) os arch hash;
    in
    pkgs.fetchzip {
      inherit hash;
      url = "https://github.com/livekit/rust-sdks/releases/download/webrtc-${version}/webrtc-${os}-${arch}-release.zip";
    };

  package = version: {
    ${version} = pkgs.stdenv.mkDerivation {
      name = "webrtc-${version}";
      version = version;
      src = artifacts system version;
      sourceRoot = ".";
      installPhase = ''
        mkdir $out
        cp -r $src/* $out
      '';
    };
  };

in
rec {
  webrtc = {
    default = webrtc."0001d84-2";
    inherit (package "0001d84-2") "0001d84-2";
  };
}
