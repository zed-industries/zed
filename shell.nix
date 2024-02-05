{ pkgs, lib }:
let
  toolchain = pkgs.rust-bin.stable.latest.default;
in
pkgs.mkShell rec {
  buildInputs = with pkgs; [
    (toolchain.override {
      extensions = ["rust-src" "clippy"];
      targets = ["x86_64-unknown-linux-gnu" "wasm32-wasi"];
    })
    rust-analyzer
    openssl
    fontconfig
    protobuf
    clang
    rustfmt
    alsa-lib
    wayland
    libxkbcommon
    libGL
    libdrm
    libelf
    xorg.libxcb
    glslang
    vulkan-headers
    vulkan-loader
    vulkan-validation-layers
    vulkan-tools
    xorg.libX11
    xorg.libXi
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXext
    xorg.libxshmfence
    xorg.libXxf86vm
    wayland-protocols
    udev
    pkgs.llvmPackages_latest.bintools
    pkgs.llvmPackages_latest.llvm
    pkgs.llvmPackages_latest.clang
    pkgs.llvmPackages_latest.libclang
    pkgs.llvmPackages_latest.lldb
  ];
  nativeBuildInputs = with pkgs; [
    pkg-config
  ];
  RUST_BACKTRACE = "1";
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
  VK_LAYER_PATH = "${pkgs.vulkan-validation-layers}/share/vulkan/explicit_layer.d"; 
}
