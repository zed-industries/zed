{ inputs, ... }:
{
  imports = [
    inputs.flake-parts.flakeModules.partitions
  ];

  partitionedAttrs = {
    devShells = "dev";
    formatter = "dev";
    checks = "dev";
  };

  partitions.dev = {
    extraInputsFlake = ../dev;
    module =
      { inputs, ... }:
      {
        imports = [
          inputs.treefmt-nix.flakeModule
          ./devshells.nix
          ./treefmt.nix
        ];
      };
  };
}
