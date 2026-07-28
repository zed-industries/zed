# NixOS VM integration test for GPUI X11 presentation across i3 workspaces.
#
# Automated:
#   nix build .#checks.x86_64-linux.i3-workspace-redraw -L
#
# Interactive:
#   nix run .#checks.x86_64-linux.i3-workspace-redraw.driverInteractive
{
  pkgs,
  inputs,
}:
let
  lib = pkgs.lib;

  rustBin = inputs.rust-overlay.lib.mkRustBin { } pkgs;
  rustToolchain = rustBin.fromRustupToolchainFile ../../rust-toolchain.toml;
  craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

  fixture =
    let
      src = builtins.path {
        path = ../../.;
        filter =
          path: _type:
          let
            root = toString ../../. + "/";
            relativePath = lib.removePrefix root path;
            firstComponent = builtins.head (lib.path.subpath.components relativePath);
          in
          builtins.elem firstComponent [
            "crates"
            "assets"
            "extensions"
            "script"
            "tooling"
            "Cargo.toml"
            ".config"
            ".cargo"
          ];
        name = "gpui-i3-workspace-redraw-source";
      };
      commonArgs = {
        pname = "gpui-i3-workspace-redraw-fixture";
        version = "0.0.0";
        inherit src;
        cargoLock = ../../Cargo.lock;
        cargoExtraArgs = "-p gpui --example x11_workspace_redraw --locked --features=gpui_platform/x11";
        CARGO_PROFILE = "dev";

        nativeBuildInputs = with pkgs; [
          cmake
          pkg-config
          rustPlatform.bindgenHook
        ];

        buildInputs = with pkgs; [
          alsa-lib
          fontconfig
          freetype
          libdrm
          libgbm
          libglvnd
          libx11
          libxcb
          libxcomposite
          libxdamage
          libxext
          libxfixes
          libxkbcommon
          libxrandr
          openssl
          vulkan-loader
          wayland
          zlib
          zstd
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
            (stdenv: stdenv.override addBinTools)
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
          install -D -m 755 \
            target/debug/examples/x11_workspace_redraw \
            $out/bin/gpui-i3-workspace-redraw-fixture
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
          description = "Deterministic GPUI X11 workspace redraw fixture";
          platforms = lib.platforms.linux;
        };
      }
    );

  testScript = pkgs.writeShellApplication {
    name = "test-i3-workspace-redraw";
    runtimeInputs = with pkgs; [
      bash
      coreutils
      gawk
      gnugrep
      i3
      imagemagick
      jq
      xdotool
      xorg.xprop
    ];
    text = builtins.readFile ../../script/test-i3-workspace-redraw;
  };

  i3Config = pkgs.writeText "gpui-i3-workspace-redraw-config" ''
    set $mod Mod4
    font pango:monospace 8
    focus_follows_mouse no
    mouse_warping none
    workspace_auto_back_and_forth no
    focus_wrapping no
    default_border none
    default_floating_border none
  '';

  lavapipeIcd = "${pkgs.mesa}/share/vulkan/icd.d/lvp_icd.${pkgs.stdenv.hostPlatform.linuxArch}.json";
in
pkgs.testers.nixosTest {
  name = "gpui-i3-workspace-redraw-x11";

  nodes.machine =
    { pkgs, ... }:
    {
      services.xserver = {
        enable = true;
        displayManager.lightdm.enable = true;
        windowManager.i3 = {
          enable = true;
          configFile = i3Config;
          extraPackages = [ ];
        };
      };
      services.displayManager = {
        defaultSession = "none+i3";
        autoLogin = {
          enable = true;
          user = "gpui-test";
        };
      };

      hardware.graphics = {
        enable = true;
        package = pkgs.mesa;
      };

      fonts.packages = [ pkgs.dejavu_fonts ];

      environment.systemPackages = [
        fixture
        testScript
        pkgs.gawk
        pkgs.i3
        pkgs.imagemagick
        pkgs.jq
        pkgs.mesa-demos
        pkgs.vulkan-loader
        pkgs.vulkan-tools
        pkgs.xdotool
        pkgs.xorg.xprop
        pkgs.xorg.xwininfo
      ];

      environment.variables = {
        LIBGL_ALWAYS_SOFTWARE = "1";
        RUST_BACKTRACE = "1";
        VK_DRIVER_FILES = lavapipeIcd;
        WGPU_BACKEND = "vulkan";
        XDG_SESSION_TYPE = "x11";
      };

      users.users.gpui-test = {
        isNormalUser = true;
        password = "gpui-test";
        extraGroups = [
          "render"
          "video"
        ];
      };

      virtualisation = {
        memorySize = 4096;
        cores = 2;
        resolution = {
          x = 1280;
          y = 800;
        };
        qemu.options = [ "-vga virtio" ];
      };
    };

  testScript = ''
    import os

    machine.wait_for_x()
    machine.wait_for_unit("graphical.target")
    machine.wait_until_succeeds(
        "runuser -u gpui-test -- env DISPLAY=:0 "
        "XAUTHORITY=/home/gpui-test/.Xauthority i3-msg -t get_version",
        timeout=30,
    )

    machine.succeed("install -d -o gpui-test -g users /tmp/gpui-i3-artifacts")
    machine.succeed(
        "runuser -u gpui-test -- env "
        "VK_DRIVER_FILES=${lavapipeIcd} "
        "vulkaninfo --summary > /tmp/gpui-i3-artifacts/vulkaninfo.txt 2>&1"
    )
    machine.succeed(
        "{ "
        "i3 --version; "
        "Xorg -version 2>&1 | head -n 2; "
        "convert -version | head -n 1; "
        "} > /tmp/gpui-i3-artifacts/package-versions.txt"
    )

    status, output = machine.execute(
        "runuser -u gpui-test -- env "
        "DISPLAY=:0 "
        "XAUTHORITY=/home/gpui-test/.Xauthority "
        "XDG_RUNTIME_DIR=/run/user/1000 "
        "XDG_SESSION_TYPE=x11 "
        "WAYLAND_DISPLAY= "
        "LIBGL_ALWAYS_SOFTWARE=1 "
        "VK_DRIVER_FILES=${lavapipeIcd} "
        "WGPU_BACKEND=vulkan "
        "RUST_LOG=gpui_linux=info,gpui_wgpu=info "
        "test-i3-workspace-redraw "
        "--candidate ${fixture}/bin/gpui-i3-workspace-redraw-fixture "
        "--iterations 5 "
        "--settle 0.5 "
        "--threshold 0.01 "
        "--artifacts /tmp/gpui-i3-artifacts",
        timeout=300,
    )
    print(output)

    machine.execute(
        "cp /var/log/X.0.log /tmp/gpui-i3-artifacts/Xorg.0.log 2>/dev/null || true"
    )
    machine.execute(
        "journalctl --no-pager -b > /tmp/gpui-i3-artifacts/system-journal.log"
    )
    machine.copy_from_vm("/tmp/gpui-i3-artifacts", os.environ["out"])

    if status != 0:
        machine.log("fixture log tail:")
        print(machine.execute("tail -n 100 /tmp/gpui-i3-artifacts/candidate-target.log")[1])
        machine.log("i3 journal tail:")
        print(machine.execute("journalctl --no-pager -b -n 100 | tail -n 100")[1])
        raise Exception(f"workspace redraw test failed with status {status}")

    machine.succeed(
        "grep -E 'Selected GPU.*llvmpipe.*Vulkan' "
        "/tmp/gpui-i3-artifacts/candidate-target.log"
    )
    machine.fail("pgrep -f gpui-i3-workspace-redraw-fixture")
  '';
}
