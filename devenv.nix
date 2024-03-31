{pkgs, ...}: rec {
  packages = with pkgs; [
    pkg-config
    openssl
    fontconfig
    alsa-lib
    wayland
    libGL
    vulkan-loader
    libxkbcommon
    xorg.libxcb
  ];
  env.PROTOC = "${pkgs.protobuf}/bin/protoc";
  env.LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath packages}";
  languages.rust.enable = true;
}
