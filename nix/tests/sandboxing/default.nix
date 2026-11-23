{ pkgs, inputs }:
let
  helper = import ./helper.nix {
    inherit pkgs inputs;
  };

  mkTest =
    { name, userNamespaces }:
    pkgs.testers.nixosTest {
      inherit name;

      nodes.machine = {
        boot.consoleLogLevel = pkgs.lib.mkForce 3;
        services.journald.settings.Journal.MaxLevelConsole = pkgs.lib.mkForce "notice";
        boot.kernel.sysctl."user.max_user_namespaces" = if userNamespaces then 1024 else 0;
        users.users.sandbox-test = {
          isNormalUser = true;
          uid = 1000;
        };
        environment.systemPackages = [
          helper
          pkgs.python3
          pkgs.util-linux
        ];
        virtualisation = {
          memorySize = 1024;
          cores = 2;
        };
      };

      testScript = ''
        machine.start()
        machine.wait_for_unit("multi-user.target")
        print(machine.succeed(
            "su -l sandbox-test -c 'timeout 180 sandbox-linux-test-helper ${
              if userNamespaces then "--require-sandbox" else "--expect-userns-disabled"
            }' 2>&1",
            timeout=200,
        ))
      '';
    };
in
{
  sandbox-linux-working = mkTest {
    name = "sandbox-linux-working";
    userNamespaces = true;
  };
  sandbox-linux-userns-disabled = mkTest {
    name = "sandbox-linux-userns-disabled";
    userNamespaces = false;
  };
}
