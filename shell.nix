(import (
  let
    rev = "v1.1.0";
    sha256 = "sha256:19d2z6xsvpxm184m41qrpi1bplilwipgnzv9jy17fgw421785q1m";
  in
  fetchTarball {
    inherit sha256;
    url = "https://github.com/NixOS/flake-compat/archive/${rev}.tar.gz";
  }
) { src = ./.; }).shellNix
