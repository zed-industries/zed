{ self, ... }:
{
  perSystem =
    { config, pkgs, ... }:
    {
      packages = {
        zed-editor = pkgs.callPackage ./zed-editor.nix {
          version = self.rev or self.dirtyRev or "unknown-nightly";
          rustPlatform = pkgs.makeRustPlatform {
            cargo = pkgs.rust-toolchain;
            rustc = pkgs.rust-toolchain;
          };
          zed-editor = config.packages.zed-editor;
        };
        default = config.packages.zed-editor;
      };
    };
}
