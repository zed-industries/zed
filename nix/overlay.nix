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

      cargo-bundle = prev.cargo-bundle.overrideAttrs (
        new: old: {
          version = "0.6.1-zed";
          src = final.fetchFromGitHub {
            owner = "zed-industries";
            repo = "cargo-bundle";
            rev = "2be2669972dff3ddd4daf89a2cb29d2d06cad7c7";
            hash = "sha256-cSvW0ND148AGdIGWg/ku0yIacVgW+9f1Nsi+kAQxVrI=";
          };
          # https://nixos.asia/en/buildRustPackage
          cargoDeps = final.rustPlatform.fetchCargoVendor {
            inherit (new) pname version src;
            hash = "sha256-urn+A3yuw2uAO4HGmvQnKvWtHqvG9KHxNCCWTiytE4k=";
          };
        }
      );
    };
  };
}
