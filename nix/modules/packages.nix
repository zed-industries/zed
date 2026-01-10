{ inputs, ... }:
{
  perSystem =
    { pkgs, ... }:
    let
      mkZed = import ../toolchain.nix { inherit inputs; };
      zed-editor = mkZed pkgs;
    in
    {
      packages = {
        default = zed-editor;
        debug = zed-editor.override { profile = "dev"; };
      };
    };
}
