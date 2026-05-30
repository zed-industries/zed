{ inputs, ... }:
{
  perSystem =
    {
      pkgs,
      lib,
      system,
      ...
    }:
    let
      mkZed = import ../toolchain.nix { inherit inputs; };
      zed-editor = mkZed pkgs;
    in
    {
      packages = {
        default = zed-editor;
        debug = zed-editor.override { profile = "dev"; };
      };
    }
    // lib.optionalAttrs (lib.hasSuffix "linux" system) {
      checks.a11y-test = import ../tests/a11y.nix {
        inherit pkgs inputs;
      };
    };
}
