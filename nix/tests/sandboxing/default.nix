# NixOS VM tests for the Linux Bubblewrap (`bwrap`) sandbox.
#
# bwrap's isolation doesn't scale with the kernel ABI the way Landlock did: any
# kernel with unprivileged user namespaces gives the same guarantees. What
# actually changes the sandbox's behavior is the *host configuration* — whether
# a usable `bwrap` exists and whether the kernel lets us create user namespaces.
# We can only set those at boot, which is exactly what a VM gives us. So we run
# one machine per host configuration:
#
#   working          - `bwrap` present, unprivileged user namespaces available;
#                      the sandbox must be enforced. The helper runs the full
#                      filesystem x network policy matrix and asserts every
#                      grant the sandbox makes and every restriction it imposes.
#   no-bwrap         - no `bwrap` on PATH; `Sandbox::can_create` must report
#                      `BwrapNotFound` and the consumer must fail closed.
#   setuid-bwrap     - the only `bwrap` is setuid-root; `can_create` must report
#                      `BwrapSetuidRejected` (we refuse to run setuid bwrap).
#   userns-disabled  - `bwrap` present but unprivileged user namespaces are
#                      disabled via sysctl; `can_create` must report
#                      `SandboxProbeFailed`.
#
# Two echo servers (`echo1`, `echo2`) on separate nodes give the network checks
# real peers: the restricted-network policy allowlists `echo1` only, so the
# helper can prove `echo1` is reachable while `echo2` is blocked.
#
# The helper drives the sandbox crate's *public* API only, so these tests double
# as a check that the public API can express and enforce the agent's policies.
#
# Build one scenario:
#   nix build .#checks.x86_64-linux.sandbox-working
# Interactive:
#   nix run .#checks.x86_64-linux.sandbox-working.driverInteractive
{
  pkgs,
  inputs,
}:
let
  lib = pkgs.lib;

  bwrap-test-helper = import ./helper.nix { inherit pkgs inputs; };

  echo1Port = 7001;
  echo2Port = 7002;

  # A node running a raw TCP echo server (socat EXEC:cat). The helper round-trips
  # a byte through it, directly or via the sandbox's HTTP CONNECT proxy.
  mkEchoNode = port: {
    networking.firewall.allowedTCPPorts = [ port ];
    systemd.services.echo-server = {
      description = "TCP echo server";
      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ];
      serviceConfig = {
        ExecStart = "${pkgs.socat}/bin/socat -d TCP-LISTEN:${toString port},reuseaddr,fork EXEC:cat";
        DynamicUser = true;
        Restart = "on-failure";
      };
    };
  };

  # Quiet boot + a couple of cores; shared by every machine-under-test.
  baseMachine = {
    boot.consoleLogLevel = lib.mkForce 3; # be quiet pls :)
    services.journald.extraConfig = lib.mkAfter "MaxLevelConsole=notice";
    environment.systemPackages = [ bwrap-test-helper ];
    virtualisation = {
      memorySize = 1024;
      cores = 2;
    };
  };

  # Per-scenario host configuration. Each entry layers onto `baseMachine`.
  machineConfigs = {
    # Plain, working host: bwrap on PATH, user namespaces available.
    working =
      { ... }:
      lib.mkMerge [
        baseMachine
        { environment.systemPackages = [ pkgs.bubblewrap ]; }
      ];

    # No bwrap anywhere on PATH.
    no-bwrap = { ... }: baseMachine;

    # The only bwrap is a setuid-root wrapper. We deliberately do NOT also put a
    # plain `pkgs.bubblewrap` on PATH: `locate_bwrap` returns the first `bwrap`
    # it finds on PATH, and we want that to be the setuid one so the helper sees
    # `BwrapSetuidRejected`.
    setuid-bwrap =
      { ... }:
      lib.mkMerge [
        baseMachine
        {
          security.wrappers.bwrap = {
            source = "${pkgs.bubblewrap}/bin/bwrap";
            owner = "root";
            group = "root";
            setuid = true;
          };
        }
      ];

    # bwrap present, but unprivileged user namespaces disabled at boot. Capping
    # the count at 0 makes `bwrap --unshare-user` fail, exactly like a hardened
    # host, so the probe reports `SandboxProbeFailed`.
    userns-disabled =
      { ... }:
      lib.mkMerge [
        baseMachine
        {
          environment.systemPackages = [ pkgs.bubblewrap ];
          boot.kernel.sysctl."user.max_user_namespaces" = 0;
        }
      ];
  };

  mkTest =
    {
      name,
      # One of: enforced | bwrap_not_found | setuid_rejected | probe_failed.
      expect,
      machine,
    }:
    pkgs.testers.nixosTest {
      inherit name;

      nodes = {
        echo1 = { ... }: mkEchoNode echo1Port;
        echo2 = { ... }: mkEchoNode echo2Port;
        machine = machine;
      };

      # Env is passed inline so it doesn't depend on /etc/profile being sourced
      # by the test driver's shell.
      testScript = ''
        start_all()

        echo1.wait_for_unit("echo-server.service")
        echo1.wait_for_open_port(${toString echo1Port})
        echo2.wait_for_unit("echo-server.service")
        echo2.wait_for_open_port(${toString echo2Port})

        machine.wait_for_unit("multi-user.target")
        machine.wait_until_succeeds("getent hosts echo1", timeout=30)
        machine.wait_until_succeeds("getent hosts echo2", timeout=30)

        # The helper logs each check tagged `[sandbox_test]:`. `succeed` fails the
        # whole test on a non-zero exit; we print its output so the per-check
        # results (and any SKIP) show up in the build log.
        print(machine.succeed(
            "ZED_TEST_EXPECT=${expect} "
            "ZED_TEST_ECHO1=echo1:${toString echo1Port} "
            "ZED_TEST_ECHO2=echo2:${toString echo2Port} "
            "bwrap_test_helper 2>&1"
        ))
      '';
    };
in
{
  sandbox-working = mkTest {
    name = "sandbox-working";
    expect = "enforced";
    machine = machineConfigs.working;
  };

  sandbox-no-bwrap = mkTest {
    name = "sandbox-no-bwrap";
    expect = "bwrap_not_found";
    machine = machineConfigs.no-bwrap;
  };

  sandbox-setuid-bwrap = mkTest {
    name = "sandbox-setuid-bwrap";
    expect = "setuid_rejected";
    machine = machineConfigs.setuid-bwrap;
  };

  sandbox-userns-disabled = mkTest {
    name = "sandbox-userns-disabled";
    expect = "probe_failed";
    machine = machineConfigs.userns-disabled;
  };
}
