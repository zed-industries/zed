{ inputs, ... }:
{
  imports = [
    inputs.treefmt-nix.flakeModule
  ];
  perSystem =
    { ... }:
    {
      treefmt = {
        projectRootFile = "flake.nix";
        options = {
          allow-missing-formatter = true;
        };
        programs = {
          rustfmt.enable = true;
          nixfmt.enable = true;
          shfmt.enable = true;
          taplo.enable = true;
        };
      };
    };
}
