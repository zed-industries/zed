{
  pkgs,
  zed,
  ...
}: let
  user = "alice";
  run = command: "sudo -u ${user} ${command}";
in
  pkgs.testers.runNixOSTest {
    name = "simple file reading";

    # sshBackdoor.enable = true;
    # enableDebugHook = true;
    enableOCR = true;

    nodes.machine = {config, ...}: {
      imports = [./common.nix];

      test-support.user = user;
      test-support.desktop-environment = "icewm";

      environment.systemPackages = [zed];
    };

    testScript =
      /*
      python
      */
      ''
        machine.wait_for_x()

        machine.succeed("${run "zeditor /home/${user}/foo.md"}")
        machine.wait_for_text("foo.md")

        machine.send_chars("hello world\n");
        machine.wait_for_text("hello world");

        machine.send_key("ctrl-s");
        machine.wait_for_file("/home/${user}/foo.md");
      '';
  }
