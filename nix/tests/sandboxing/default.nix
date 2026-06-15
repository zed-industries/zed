# NixOS VM tests for the Linux Bubblewrap (`bwrap`) sandbox.
#
# Unlike the old Landlock tests, bwrap's filesystem isolation does not scale
# with the kernel ABI: any kernel with unprivileged user namespaces gives the
# same guarantees. So instead of a kernel matrix we keep scenarios that capture
# the cases that actually change behavior:
#
#   sandbox-userns-enabled  - bwrap present and unprivileged user namespaces
#                             available; the sandbox must be enforced. The helper
#                             asserts every grant the sandbox makes and every
#                             restriction it imposes.
#   sandbox-userns-disabled - bwrap present but user namespaces disabled via
#                             sysctl; the launcher must report SandboxProbeFailed
#                             and degrade to an unsandboxed run.
#   sandbox-no-bwrap        - no bwrap on PATH at all; the launcher must report
#                             BwrapNotFound and degrade. This mirrors a machine
#                             where the sandboxing feature is enabled but bwrap
#                             simply isn't installed.
#
# The point of a VM is that whether bwrap can enforce a sandbox depends on
# kernel configuration we can only set at boot. A second node runs a TCP echo
# server so network isolation can be exercised against a real peer.
#
# Build one scenario:
#   nix build .#checks.x86_64-linux.sandbox-userns-enabled
# Interactive:
#   nix run .#checks.x86_64-linux.sandbox-userns-enabled.driverInteractive
{
  pkgs,
  inputs,
}:
let
  lib = pkgs.lib;

  bwrap-test-helper = import ./helper.nix { inherit pkgs inputs; };

  echoPort = 7000;

  mkTest =
    {
      name,
      # Whether a `bwrap` binary is on the machine's PATH.
      installBwrap ? true,
      # Whether to disable unprivileged user namespaces at boot.
      usernsDisabled ? false,
      # For degraded scenarios, the exact status the launcher should report.
      expectedDegradeStatus ? null,
    }:
    let
      # The sandbox is only actually enforced when bwrap is present *and* user
      # namespaces are available.
      enforced = installBwrap && !usernsDisabled;
    in
    pkgs.testers.nixosTest {
      inherit name;

      nodes = {
        # TCP echo server, so the network checks have a real peer to (fail to)
        # reach from inside the sandbox.
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

        # The machine under test.
        machine =
          { ... }:
          {
            boot.consoleLogLevel = lib.mkForce 3; # be quiet pls :)
            services.journald.extraConfig = lib.mkAfter "MaxLevelConsole=notice";

            # Disable unprivileged user namespaces for the degradation scenario.
            # Capping the count at 0 makes `bwrap --unshare-user` fail, which is
            # exactly what a hardened host looks like.
            boot.kernel.sysctl = lib.mkIf usernsDisabled {
              "user.max_user_namespaces" = 0;
            };

            # `bwrap` itself (the helper locates it on PATH) plus the helper.
            # The no-bwrap scenario deliberately omits bubblewrap.
            environment.systemPackages = [
              bwrap-test-helper
            ]
            ++ lib.optionals installBwrap [ pkgs.bubblewrap ];

            # The helper reads its expectations from the environment; setting
            # them here means `machine.execute` picks them up via /etc/profile.
            environment.variables = {
              ZED_TEST_SANDBOX_ENFORCED = if enforced then "1" else "0";
              ZED_TEST_ECHO_ADDR = "echo:${toString echoPort}";
            }
            // lib.optionalAttrs (expectedDegradeStatus != null) {
              ZED_TEST_EXPECTED_DEGRADE_STATUS = expectedDegradeStatus;
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

        # The helper logs each check to stdout/stderr tagged `[sandbox_test]:`,
        # which the test backdoor wires to the serial console, so the lines
        # appear inline in this log. `succeed` fails the test on a non-zero exit.
        machine.succeed("bwrap_test_helper")
      '';
    };
in
{
  sandbox-userns-enabled = mkTest {
    name = "sandbox-userns-enabled";
  };

  sandbox-userns-disabled = mkTest {
    name = "sandbox-userns-disabled";
    usernsDisabled = true;
    expectedDegradeStatus = "probe_failed";
  };

  sandbox-no-bwrap = mkTest {
    name = "sandbox-no-bwrap";
    installBwrap = false;
    expectedDegradeStatus = "bwrap_not_found";
  };
}
