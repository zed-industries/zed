# NixOS VM tests for Linux Landlock sandboxing, run across several kernels.
#
# The point of running these in VMs (rather than as ordinary unit tests) is
# that Landlock behavior depends entirely on the running kernel: the available
# ABI level is tied to the kernel version, and the LSM can be disabled at boot.
# Each scenario boots a kernel, tells the helper what to expect via environment
# variables, and asserts the sandbox behaves accordingly. A second node runs a
# TCP echo server so network sandboxing can be exercised against a real peer.
#
# Build one scenario:
#   nix build .#checks.x86_64-linux.landlock-6_12
# Interactive:
#   nix run .#checks.x86_64-linux.landlock-6_12.driverInteractive
{
  pkgs,
  inputs,
}:
let
  lib = pkgs.lib;

  landlock-test-helper = import ./helper.nix { inherit pkgs inputs; };

  echoPort = 7000;

  mkTest =
    {
      name,
      kernelPackages,
      abi ? null,
      landlockDisabled ? false,
    }:
    pkgs.testers.nixosTest {
      inherit name;

      nodes = {
        # TCP echo server
        echo =
          { pkgs, ... }:
          {
            networking.firewall.allowedTCPPorts = [ echoPort ];
            systemd.services.echo-server = {
              description = "TCP echo server";
              wantedBy = [ "multi-user.target" ];
              after = [ "network.target" ];
              serviceConfig = {
                ExecStart = "${pkgs.socat}/bin/socat -d TCP-LISTEN:${toString echoPort},reuseaddr,fork EXEC:cat";
                DynamicUser = true;
                Restart = "on-failure";
              };
            };
          };

        # The machine under test
        machine =
          { ... }:
          {
            boot.kernelPackages = kernelPackages;

            boot.consoleLogLevel = lib.mkForce 3; # be quiet pls :)
            services.journald.extraConfig = lib.mkAfter "MaxLevelConsole=notice";

            # When disabling Landlock, override NixOS's default LSM list (which
            # force-adds `landlock`) with one that omits it. `capability` is
            # always enabled by the kernel regardless.
            security.lsm = lib.mkIf landlockDisabled (
              lib.mkForce [
                "yama"
                "bpf"
              ]
            );

            environment.systemPackages = [ landlock-test-helper ];

            # The helper reads its expectations from the environment; setting
            # them here means `machine.execute` picks them up via /etc/profile.
            environment.variables = {
              ZED_TEST_LANDLOCK_ENABLED = if landlockDisabled then "0" else "1";
              ZED_TEST_ECHO_ADDR = "echo:${toString echoPort}";
            }
            // lib.optionalAttrs (!landlockDisabled) {
              ZED_TEST_LANDLOCK_ABI = toString abi;
            };

            virtualisation = {
              memorySize = 1024;
              cores = 2;
            };
          };
      };

      testScript = ''
        start_all()

        echo.wait_for_unit("echo-server.service")
        echo.wait_for_open_port(${toString echoPort})

        machine.wait_for_unit("multi-user.target")
        machine.wait_until_succeeds("getent hosts echo", timeout=30)

        # The helper logs each step to stderr tagged `[sandbox_test]:`, which the
        # test backdoor wires to the serial console, so the lines appear inline
        # in this log. `succeed` fails the test on a non-zero exit.
        machine.succeed("landlock_test_helper")
      '';
    };
in
{
  # ABI per kernel (Landlock ABI -> first kernel that ships it):
  #   V1=5.13  V2=5.19  V3=6.2  V4=6.7  V5=6.10  V6=6.12  V7=6.15
  # A kernel reports the highest ABI it supports, so e.g. 6.6 reports V3.
  # Network (TCP) restrictions only exist from V4, so the V2/V3 kernels verify
  # that a sandboxed process can still connect, while V6 verifies it cannot.

  landlock-6_1 = mkTest {
    name = "landlock-6_1";
    kernelPackages = pkgs.linuxPackages_6_1;
    abi = 2;
  };

  landlock-6_6 = mkTest {
    name = "landlock-6_6";
    kernelPackages = pkgs.linuxPackages_6_6;
    abi = 3;
  };

  landlock-6_12 = mkTest {
    name = "landlock-6_12";
    kernelPackages = pkgs.linuxPackages_6_12;
    abi = 6;
  };

  landlock-disabled = mkTest {
    name = "landlock-disabled";
    kernelPackages = pkgs.linuxPackages_6_12;
    landlockDisabled = true;
  };
}
