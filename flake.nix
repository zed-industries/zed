{
  description = "High-performance, multiplayer code editor from the creators of Atom and Tree-sitter";

  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixpkgs-unstable/nixexprs.tar.xz";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    flake-compat.url = "github:edolstra/flake-compat";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      crane,
      ...
    }:
    let
      systems = [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
      mkZed =
        pkgs:
        let
          rustBin = rust-overlay.lib.mkRustBin { } pkgs;
        in
        pkgs.callPackage ./nix/build.nix {
          crane = crane.mkLib pkgs;
          rustToolchain = rustBin.fromRustupToolchainFile ./rust-toolchain.toml;
        };
      # Function to create zed-fhs with optional additional packages
      mkZedFHS =
        pkgs: zed:
        { additionalPkgs ? pkgs: [ ] }:
        pkgs.buildFHSEnv {
          name = "zed-fhs";

          # The target packages that will be available in the FHS environment
          targetPkgs =
            pkgs:
            (with pkgs; [
              # Requested packages
              nodejs
              python3
              yarn
              stdenv.cc.cc.lib # libgcc and libstdc++
              openssl

              # Core system libraries
              glibc
              curl
              icu
              libunwind
              libuuid
              zlib
              krb5

              # Additional development tools
              git
              pkg-config
              gcc

              # Graphics and X11 libraries (for GUI support)
              xorg.libX11
              xorg.libXcomposite
              xorg.libXdamage
              xorg.libXext
              xorg.libXfixes
              xorg.libXrandr
              xorg.libXrender
              xorg.libxcb
              xorg.libxkbfile
              xorg.libxshmfence

              # GTK/GUI dependencies
              glib
              gtk3
              cairo
              pango
              gdk-pixbuf
              atk
              at-spi2-atk
              at-spi2-core
              dbus

              # Audio
              alsa-lib

              # Browser/headless rendering support (for extensions like live preview)
              nspr
              nss
              cups
              libgbm
              mesa

              # System integration
              udev
              systemd

              # Fonts
              fontconfig
              freetype

            ] ++ additionalPkgs pkgs);

          # Multi-architecture support (enable 32-bit libraries on 64-bit systems)
          multiPkgs = pkgs: (with pkgs; [ stdenv.cc.cc.lib ]);

          # The actual program to run
          runScript = "${zed}/bin/zed";

          # Extra commands to set up the environment
          profile = ''
            export PATH="${zed}/bin:$PATH"
            export LD_LIBRARY_PATH="${pkgs.stdenv.cc.cc.lib}/lib:$LD_LIBRARY_PATH"
          '';

          # Mount additional directories
          extraBwrapArgs = [
            "--bind-try /etc/nixos/ /etc/nixos/"
            "--ro-bind-try /etc/xdg/ /etc/xdg/"
          ];

          # Allow the process to continue after parent exits
          dieWithParent = false;

          meta = with pkgs.lib; {
            description = "Zed editor in an FHS-compliant environment";
            platforms = platforms.linux;
            mainProgram = "zed-fhs";
          };
        };
    in
    rec {
      packages = forAllSystems (pkgs: rec {
        default = mkZed pkgs;
        debug = default.override { profile = "dev"; };
        zed-fhs = mkZedFHS pkgs default { };
        zedFhsWithPackages = additionalPkgs: mkZedFHS pkgs default { inherit additionalPkgs; };
      });
      devShells = forAllSystems (pkgs: {
        default = pkgs.callPackage ./nix/shell.nix {
          zed-editor = packages.${pkgs.hostPlatform.system}.default;
        };
      });
      formatter = forAllSystems (pkgs: pkgs.nixfmt-rfc-style);
      overlays.default = final: _: rec {
        zed-editor = mkZed final;
        zed-fhs = mkZedFHS final zed-editor { };
      };
    };

  nixConfig = {
    extra-substituters = [
      "https://zed.cachix.org"
      "https://cache.garnix.io"
    ];
    extra-trusted-public-keys = [
      "zed.cachix.org-1:/pHQ6dpMsAZk2DiP4WCL0p9YDNKWj2Q5FL20bNmw1cU="
      "cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g="
    ];
  };
}
