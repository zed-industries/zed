{ self, inputs, ... }:
{
  perSystem =
    { system, ... }:
    {
      _module.args.pkgs = import inputs.nixpkgs {
        inherit system;
        overlays = [
          inputs.rust-overlay.overlays.default
          self.overlays.default
        ];
      };
    };

  flake = {
    overlays.default = final: prev: {
      rust-toolchain = final.rust-bin.fromRustupToolchainFile ../rust-toolchain.toml;

      zed-editor = final.callPackage ./build.nix {
        crane = inputs.crane.mkLib final;
      };
    };
  };
}
