{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    cargo-tarpaulin
    rustup
    pkgsCross.mingwW64.stdenv.cc
    wineWowPackages.stable
    emscripten
  ];
  buildInputs = with pkgs; [
    pkgsCross.mingwW64.windows.pthreads
    nodejs
  ];
  shellHook = ''
    rustup default stable
    rustup component add rust-src
    rustup target add x86_64-unknown-linux-gnu
    rustup target add x86_64-pc-windows-gnu
    rustup target add wasm32-unknown-emscripten
  '';
}
