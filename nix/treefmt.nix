{ inputs, ... }:
{
  imports = [ inputs.treefmt-nix.flakeModule ];
  perSystem =
    { pkgs, ... }:
    {
      treefmt = {
        projectRootFile = "flake.nix";
        programs.nixfmt.enable = true;
        programs.rustfmt = {
          enable = true;
          package = pkgs.rust-toolchain;
          edition = "2024";
        };
        settings.excludes = [
          "target/**"
          ".direnv/**"
        ];
        settings.allow-missing-formatter = true;
      };
    };
}
