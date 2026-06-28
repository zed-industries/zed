# NixOS VM tests for the Linux Bubblewrap (`bwrap`) sandbox.
#
# What actually changes the sandbox's behavior is the *host configuration* —
# whether a usable `bwrap` exists and whether the kernel lets us create user
# namespaces. We can only set those at boot, which is exactly what a VM gives
# us. So we run one machine per host configuration:
#
#   working          - `bwrap` present, unprivileged user namespaces available;
#                      the sandbox must be enforced.
#   no-bwrap         - no `bwrap` on PATH; `Sandbox::can_create` must report
#                      `BwrapNotFound` and the consumer must fail closed.
#   setuid-bwrap     - the only `bwrap` is setuid-root; `can_create` must report
#                      `BwrapSetuidRejected` (we refuse to run setuid bwrap).
#   userns-disabled  - `bwrap` present but unprivileged user namespaces are
#                      disabled via sysctl; `can_create` must report
#                      `SandboxProbeFailed`.
#
# The behavior to assert is declared *here* as a list of `checks`, serialized to
# JSON and handed to the `bwrap_test_helper` binary, which builds the described
# sandbox policy via the sandbox crate's public API, performs the operation, and
# asserts the outcome. Each check is one of:
#
#   { read = "/path"; succeeds = true; }          # read a host file
#   { write = "/path"; succeeds = false; }        # write a host file
#   { network = "echo1"; succeeds = true; }       # connect to an echo server
#   { canCreate = false; error = "bwrap_not_found"; }   # Sandbox::can_create
#
# plus optional policy fields applied to that check (defaults shown):
#
#   fs = "restricted";          # or "unrestricted"
#   writablePaths = [ ];        # writable subtrees when fs = "restricted"
#   networkAccess = "blocked";  # or "unrestricted" / "restricted"
#   allowedDomains = [ ];       # allowed hosts when networkAccess = "restricted"
#
# Two echo servers (`echo1`, `echo2`) on separate nodes give the network checks
# real peers, so a restricted-network policy that allowlists `echo1` can be
# shown to reach `echo1` while `echo2` is blocked.
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

  # Both echo nodes listen on the same port; the helper appends it to a bare
  # hostname (`echo1` -> `echo1:7000`).
  echoPort = 7000;

  # A node running a raw TCP echo server (socat EXEC:cat). The helper round-trips
  # a byte through it, directly or via the sandbox's HTTP CONNECT proxy.
  mkEchoNode = {
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

  # Build a VM test for `machine` that runs `checks` through the helper.
  mkTest =
    {
      name,
      machine,
      checks,
    }:
    let
      checksFile = pkgs.writeText "${name}-checks.json" (builtins.toJSON checks);
    in
    pkgs.testers.nixosTest {
      inherit name;

      nodes = {
        echo1 = { ... }: mkEchoNode;
        echo2 = { ... }: mkEchoNode;
        machine = machine;
      };

      testScript = ''
        start_all()

        echo1.wait_for_unit("echo-server.service")
        echo1.wait_for_open_port(${toString echoPort})
        echo2.wait_for_unit("echo-server.service")
        echo2.wait_for_open_port(${toString echoPort})

        machine.wait_for_unit("multi-user.target")
        machine.wait_until_succeeds("getent hosts echo1", timeout=30)
        machine.wait_until_succeeds("getent hosts echo2", timeout=30)

        # The helper logs each check tagged `[sandbox_test]:`. `succeed` fails the
        # whole test on a non-zero exit; we print its output so the per-check
        # results show up in the build log.
        print(machine.succeed(
            "ZED_SANDBOX_CHECKS=${checksFile} "
            "ZED_TEST_ECHO_PORT=${toString echoPort} "
            "bwrap_test_helper 2>&1"
        ))
      '';
    };

in
{
  sandbox-working = mkTest {
    name = "sandbox-working";
    machine = machineConfigs.working;
    # Each check is written out in full (no shared fragments) so the policy and
    # expected outcome for every case are obvious at a glance.
    checks = [
      # Reads of host files are always allowed, regardless of the write policy.
      {
        fs = "restricted";
        writablePaths = [ "/sandbox-test/writable" ];
        networkAccess = "blocked";
        read = "/sandbox-test/readable/host.txt";
        succeeds = true;
      }

      # Write inside the granted subtree: allowed.
      {
        fs = "restricted";
        writablePaths = [ "/sandbox-test/writable" ];
        networkAccess = "blocked";
        write = "/sandbox-test/writable/ok.txt";
        succeeds = true;
      }

      # Write outside the granted subtree: denied.
      {
        fs = "restricted";
        writablePaths = [ "/sandbox-test/writable" ];
        networkAccess = "blocked";
        write = "/sandbox-test/forbidden/no.txt";
        succeeds = false;
      }

      # A writable path that doesn't exist yet must be bound at its exact
      # location (created up front), never widened to an existing ancestor. If it
      # widened to `/sandbox-test`, this write to a read-only file under that
      # ancestor would succeed — the scope-widening sandbox escape this guards
      # against. It must be denied.
      {
        fs = "restricted";
        writablePaths = [ "/sandbox-test/not-yet-created/deep" ];
        networkAccess = "blocked";
        write = "/sandbox-test/readable/host.txt";
        succeeds = false;
      }

      # ...and the not-yet-existing path itself is created and writable.
      {
        fs = "restricted";
        writablePaths = [ "/sandbox-test/not-yet-created/deep" ];
        networkAccess = "blocked";
        write = "/sandbox-test/not-yet-created/deep/ok.txt";
        succeeds = true;
      }

      # The fs escape hatch lets a command write anywhere.
      {
        fs = "unrestricted";
        networkAccess = "blocked";
        write = "/sandbox-test/forbidden/anywhere.txt";
        succeeds = true;
      }

      # Blocked network: the echo server is unreachable.
      {
        fs = "restricted";
        writablePaths = [ "/sandbox-test/writable" ];
        networkAccess = "blocked";
        network = "echo1";
        succeeds = false;
      }

      # Unrestricted network: both echo servers are reachable.
      {
        fs = "restricted";
        writablePaths = [ "/sandbox-test/writable" ];
        networkAccess = "unrestricted";
        network = "echo1";
        succeeds = true;
      }
      {
        fs = "restricted";
        writablePaths = [ "/sandbox-test/writable" ];
        networkAccess = "unrestricted";
        network = "echo2";
        succeeds = true;
      }

      # Restricted network: only the allowlisted host is reachable.
      {
        fs = "restricted";
        writablePaths = [ "/sandbox-test/writable" ];
        networkAccess = "restricted";
        allowedDomains = [ "echo1" ];
        network = "echo1";
        succeeds = true;
      }
      {
        fs = "restricted";
        writablePaths = [ "/sandbox-test/writable" ];
        networkAccess = "restricted";
        allowedDomains = [ "echo1" ];
        network = "echo2";
        succeeds = false;
      }

      # On a working host the sandbox can be created.
      {
        fs = "restricted";
        networkAccess = "blocked";
        canCreate = true;
      }
    ];
  };

  sandbox-no-bwrap = mkTest {
    name = "sandbox-no-bwrap";
    machine = machineConfigs.no-bwrap;
    checks = [ { canCreate = false; error = "bwrap_not_found"; } ];
  };

  sandbox-setuid-bwrap = mkTest {
    name = "sandbox-setuid-bwrap";
    machine = machineConfigs.setuid-bwrap;
    checks = [ { canCreate = false; error = "setuid_rejected"; } ];
  };

  sandbox-userns-disabled = mkTest {
    name = "sandbox-userns-disabled";
    machine = machineConfigs.userns-disabled;
    checks = [ { canCreate = false; error = "probe_failed"; } ];
  };
}
