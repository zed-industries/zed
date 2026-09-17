# NixOS VM integration test for GPUI AccessKit (X11).
#
# Interactive use:
#   nix run .#checks.x86_64-linux.a11y-test.driverInteractive
#
# Then in the Python REPL:
#   start_all()
#   machine.wait_for_x()
#   machine.succeed("su - user -c 'DISPLAY=:0 gpui-a11y-example &'")
#
# Automated run:
#   nix build .#checks.x86_64-linux.a11y-test
{
  pkgs,
  inputs,
}:
let
  lib = pkgs.lib;

  rustBin = inputs.rust-overlay.lib.mkRustBin { } pkgs;
  rustToolchain = rustBin.fromRustupToolchainFile ../../rust-toolchain.toml;
  craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

  gpui-a11y-example =
    let
      src = builtins.path {
        path = ../../.;
        filter =
          path: type:
          let
            root = toString ../../. + "/";
            relPath = lib.removePrefix root path;
            firstComp = builtins.head (lib.path.subpath.components relPath);
          in
          builtins.elem firstComp [
            "crates"
            "assets"
            "extensions"
            "script"
            "tooling"
            "Cargo.toml"
            ".config"
            ".cargo"
          ];
        name = "gpui-a11y-source";
      };
      commonArgs = {
        pname = "gpui-a11y-example";
        version = "0.0.0";
        inherit src;
        cargoLock = ../../Cargo.lock;
        cargoExtraArgs = "-p gpui --example a11y --locked --features=gpui_platform/runtime_shaders";
        CARGO_PROFILE = "dev";

        nativeBuildInputs = with pkgs; [
          cmake
          pkg-config
          rustPlatform.bindgenHook
        ];

        buildInputs = with pkgs; [
          fontconfig
          freetype
          openssl
          zlib
          zstd
          alsa-lib
          libxkbcommon
          wayland
          vulkan-loader
          libglvnd
          libx11
          libxcb
          libdrm
          libgbm
          libxcomposite
          libxdamage
          libxext
          libxfixes
          libxrandr
        ];

        cargoVendorDir = craneLib.vendorCargoDeps {
          inherit src;
          cargoLock = ../../Cargo.lock;
        };

        env = {
          ZSTD_SYS_USE_PKG_CONFIG = true;
          FONTCONFIG_FILE = pkgs.makeFontsConf {
            fontDirectories = [
              ../../assets/fonts/lilex
              ../../assets/fonts/ibm-plex-sans
            ];
          };
        };

        doCheck = false;

        stdenv =
          let
            base = pkgs.llvmPackages.stdenv;
            addBinTools = old: {
              cc = old.cc.override {
                inherit (pkgs.llvmPackages) bintools;
              };
            };
          in
          lib.pipe base [
            (s: s.override addBinTools)
            pkgs.stdenvAdapters.useMoldLinker
          ];
      };
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
    in
    craneLib.buildPackage (
      lib.recursiveUpdate commonArgs {
        inherit cargoArtifacts;
        dontUseCmakeConfigure = true;

        installPhase = ''
          runHook preInstall
          mkdir -p $out/bin
          cp target/debug/examples/a11y $out/bin/gpui-a11y-example
          runHook postInstall
        '';

        NIX_LDFLAGS = "-rpath ${
          lib.makeLibraryPath [
            pkgs.vulkan-loader
            pkgs.wayland
          ]
        }";
        dontPatchELF = true;

        meta = {
          description = "GPUI accessibility (AccessKit) example app";
          platforms = lib.platforms.linux;
        };
      }
    );

  atspiTestScript = pkgs.writeTextFile {
    name = "a11y-atspi-test";
    text = builtins.readFile ./a11y_atspi_test.py;
    destination = "/bin/a11y-atspi-test";
    executable = true;
    checkPhase = ''
      ${pkgs.python3.interpreter} -m py_compile $target
    '';
  };

  testPython = pkgs.python3.withPackages (ps: [ ps.pyatspi ps.pygobject3 ]);

  giTypelibPath = lib.makeSearchPath "lib/girepository-1.0" [
    pkgs.at-spi2-core
    pkgs.glib
    pkgs.gtk3
    pkgs.gobject-introspection
  ];
in
pkgs.testers.nixosTest {
  name = "gpui-a11y-x11";

  nodes.machine =
    { pkgs, ... }:
    {
      imports = [ ];

      # Minimal X11 desktop
      services.xserver = {
        enable = true;
        desktopManager.xfce.enable = true;
        displayManager.lightdm.enable = true;
      };

      # Auto-login so the test doesn't need to type a password
      services.displayManager.autoLogin = {
        enable = true;
        user = "user";
      };

      # AT-SPI2 accessibility bus
      services.gnome.at-spi2-core.enable = true;

      # dconf + GSettings schemas required for Orca / AT-SPI
      programs.dconf = {
        enable = true;
        profiles.user.databases = [
          {
            settings = {
              "org/gnome/desktop/interface".toolkit-accessibility = true;
              "org/gnome/desktop/a11y/applications".screen-reader-enabled = true;
            };
          }
        ];
      };

      # Environment variables for debugging
      environment.variables = {
        RUST_BACKTRACE = "1";
      };

      # Start Orca automatically on login
      systemd.user.services.orca = {
        description = "Orca screen reader";
        wantedBy = [ "graphical-session.target" ];
        partOf = [ "graphical-session.target" ];
        after = [ "graphical-session.target" ];
        serviceConfig = {
          ExecStart = "${pkgs.orca}/bin/orca --debug";
          Restart = "on-failure";
        };
        environment = {
          DISPLAY = ":0";
        };
      };

      # Accessibility tools available in the VM
      environment.systemPackages = [
        gpui-a11y-example
        atspiTestScript
        testPython
        pkgs.accerciser
        pkgs.gsettings-desktop-schemas
        pkgs.orca
        pkgs.xdotool
      ];

      # Test user
      users.users.user = {
        isNormalUser = true;
        password = "pass";
        extraGroups = [ "wheel" ];
      };

      # Give the VM enough resources for a GUI
      virtualisation = {
        memorySize = 4096;
        cores = 2;
        qemu.options = [
          "-vga virtio"
        ];
      };
    };

  testScript = ''
machine.wait_for_x()
machine.wait_for_unit("graphical.target")

# Let the desktop and Orca settle
machine.sleep(5)

# Launch the a11y example, capturing logs to a file
machine.succeed(
    "su - user -c 'DISPLAY=:0 WAYLAND_DISPLAY= RUST_LOG=gpui=info gpui-a11y-example > /tmp/gpui.log 2>&1 &'"
)

# Wait for the window to appear
machine.wait_until_succeeds("su - user -c 'DISPLAY=:0 xdotool search --name \"GPUI Accessibility Demo\"'", timeout=15)

# Wait for accessibility activation
machine.wait_until_succeeds("grep -q 'Accessibility activated' /tmp/gpui.log", timeout=15)
machine.log("Accessibility activation confirmed in logs")

# Give AccessKit time to register on AT-SPI
machine.sleep(3)

# Run the AT-SPI test script
machine.succeed(
    "su - user -c 'DISPLAY=:0 GI_TYPELIB_PATH=${giTypelibPath} ${testPython}/bin/python3 ${atspiTestScript}/bin/a11y-atspi-test'"
)
machine.log("AT-SPI tests passed (first run)")

# Kill the app, restart Orca, and re-run
machine.execute("pkill -f gpui-a11y-example")
machine.sleep(1)
machine.succeed("su - user -c 'XDG_RUNTIME_DIR=/run/user/1000 systemctl --user restart orca'")
machine.sleep(3)

# Relaunch the app
machine.succeed(
    "su - user -c 'DISPLAY=:0 WAYLAND_DISPLAY= RUST_LOG=gpui=info gpui-a11y-example > /tmp/gpui2.log 2>&1 &'"
)
machine.wait_until_succeeds("su - user -c 'DISPLAY=:0 xdotool search --name \"GPUI Accessibility Demo\"'", timeout=15)
machine.wait_until_succeeds("grep -q 'Accessibility activated' /tmp/gpui2.log", timeout=15)
machine.log("Accessibility activation confirmed after Orca restart")
machine.sleep(3)

# Run the AT-SPI test script again
machine.succeed(
    "su - user -c 'DISPLAY=:0 GI_TYPELIB_PATH=${giTypelibPath} ${testPython}/bin/python3 ${atspiTestScript}/bin/a11y-atspi-test'"
)
machine.log("AT-SPI tests passed (second run, after Orca restart)")
  '';
}
