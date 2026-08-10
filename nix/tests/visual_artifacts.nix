# NixOS VM test for damage-tracking visual correctness (X11).
#
# Runs the real Zed editor twice through an identical scripted interaction
# sequence (typing, hovering, scrolling), once with partial rendering and
# present-skipping disabled (the legacy full-redraw path, as reference) and
# once with them enabled. The Zed window is screenshotted at fixed
# checkpoints in both runs, and every pair of screenshots must be
# pixel-identical: partial rendering may change *how* pixels get to the
# screen, but never *which* pixels.
#
# Each run also performs a self-check: after the interaction phases, the
# window is resized away and back (forcing a full re-render of the same
# scene), and the before/after screenshots must match. In the enabled run
# this directly exposes stale pixels accumulated by partial rendering; in the
# reference run it validates that the methodology itself is deterministic.
#
# Determinism notes: cursor blinking and scrollbars are disabled through
# settings, runs start from a fresh HOME, the window is pinned to a fixed
# position and size, the mouse ends each phase at a fixed position, and
# screenshots capture only the Zed window (not the desktop).
#
# Automated run:
#   nix build .#checks.x86_64-linux.visual-artifacts
# Interactive:
#   nix run .#checks.x86_64-linux.visual-artifacts.driverInteractive
{
  pkgs,
  inputs,
}:
let
  mkZed = import ../toolchain.nix { inherit inputs; };
  zed = mkZed pkgs;

  zedSettings = pkgs.writeText "zed-test-settings.json" (
    builtins.toJSON {
      cursor_blink = false;
      scrollbar.show = "never";
      minimap.show = "never";
      auto_update = false;
      telemetry = {
        diagnostics = false;
        metrics = false;
      };
    }
  );

  sampleFile = pkgs.writeText "sample.txt" (
    pkgs.lib.concatMapStrings (i: "line ${toString i}: the quick brown fox jumps over the lazy dog\n") (
      pkgs.lib.range 1 120
    )
  );
in
pkgs.testers.nixosTest {
  name = "zed-visual-artifacts";

  nodes.machine =
    { pkgs, ... }:
    {
      # Minimal X11: lightdm autologin into icewm. No desktop environment, so
      # nothing (clocks, notifications) can leak nondeterminism into the
      # screen, and screenshots capture the Zed window only anyway.
      services.xserver = {
        enable = true;
        displayManager.lightdm.enable = true;
        windowManager.icewm.enable = true;
      };
      services.displayManager.autoLogin = {
        enable = true;
        user = "user";
      };

      # Software Vulkan (lavapipe) for the wgpu renderer.
      hardware.graphics.enable = true;

      users.users.user = {
        isNormalUser = true;
        password = "pass";
      };

      environment.systemPackages = [
        pkgs.xdotool
        pkgs.imagemagick # `import` (screenshots) and `compare`
      ];

      environment.variables.RUST_BACKTRACE = "1";

      virtualisation = {
        memorySize = 8192;
        cores = 4;
        resolution = {
          x = 1280;
          y = 800;
        };
        qemu.options = [ "-vga virtio" ];
      };
    };

  testScript = ''
    import shlex

    ZED = "${zed}/libexec/zed-editor"
    SHOTS = "/tmp/shots"

    MODES = {
        # The pre-damage-tracking rendering path, as ground truth.
        "reference": "GPUI_DISABLE_PARTIAL_RENDER=1 GPUI_DISABLE_PRESENT_SKIP=1 GPUI_DAMAGE_STRICT_ORDER=1",
        # Everything enabled (the defaults).
        "partial": "",
    }
    CHECKPOINTS = ["00-baseline", "01-typed", "02-hovered", "03-scrolled", "04-after-full-redraw"]


    def user_cmd(cmd, env=""):
        return f"su - user -c {shlex.quote(f'DISPLAY=:0 WAYLAND_DISPLAY= {env} {cmd}')}"


    def screenshot(mode, name, window_id):
        machine.succeed(user_cmd(f"import -window {window_id} {SHOTS}/{mode}/{name}.png"))


    def run_mode(mode, env):
        machine.succeed(f"mkdir -p {SHOTS}/{mode}")

        # Fresh, identical state for every run.
        machine.succeed(
            "rm -rf /home/user/.config/zed /home/user/.local/share/zed /home/user/project"
            " && mkdir -p /home/user/.config/zed /home/user/project"
            " && cp ${zedSettings} /home/user/.config/zed/settings.json"
            " && cp ${sampleFile} /home/user/project/sample.txt"
            " && chown -R user:users /home/user"
        )

        machine.succeed(
            user_cmd(
                f"{ZED} /home/user/project/sample.txt > /tmp/zed-{mode}.log 2>&1 &",
                env,
            )
        )
        machine.wait_until_succeeds(
            user_cmd("xdotool search --name sample.txt"), timeout=180
        )
        window_id = machine.succeed(
            user_cmd("xdotool search --name sample.txt | head -n1")
        ).strip()

        # Pin geometry so both runs render the exact same layout, and park the
        # mouse at a fixed position.
        machine.succeed(
            user_cmd(f"xdotool windowmove {window_id} 0 0")
            + " && "
            + user_cmd(f"xdotool windowsize {window_id} 1000 700")
            + " && "
            + user_cmd(f"xdotool windowactivate {window_id}")
            + " && "
            + user_cmd("xdotool mousemove 640 760")
        )
        # Let startup settle: worktree scan, first paint, any one-shot UI.
        machine.sleep(15)
        screenshot(mode, "00-baseline", window_id)

        # Phase 1: typing (small localized damage, insertions).
        machine.succeed(user_cmd("xdotool type --delay 100 'damage tracking probe text'"))
        machine.sleep(4)
        screenshot(mode, "01-typed", window_id)

        # Phase 2: a mouse sweep across the window (hover churn; produced the
        # unchanged-scene draw storm this work eliminates). Ends parked.
        for x, y in [(120, 40), (500, 40), (900, 40), (500, 350), (120, 650), (640, 760)]:
            machine.succeed(user_cmd(f"xdotool mousemove {x} {y}"))
            machine.sleep(1)
        machine.sleep(4)
        screenshot(mode, "02-hovered", window_id)

        # Phase 3: scroll down and back up (large shifting damage).
        machine.succeed(user_cmd("xdotool mousemove 500 350"))
        for button in [5] * 6 + [4] * 6:
            machine.succeed(user_cmd(f"xdotool click {button}"))
            machine.sleep(0.5)
        machine.succeed(user_cmd("xdotool mousemove 640 760"))
        machine.sleep(4)
        screenshot(mode, "03-scrolled", window_id)

        # Self-check: force a full re-render of the same scene by resizing
        # away and back. Any difference from 03 is state accumulated wrongly
        # by the renderer (or, in the reference run, test nondeterminism).
        machine.succeed(user_cmd(f"xdotool windowsize {window_id} 1000 716"))
        machine.sleep(3)
        machine.succeed(user_cmd(f"xdotool windowsize {window_id} 1000 700"))
        machine.sleep(4)
        screenshot(mode, "04-after-full-redraw", window_id)

        machine.execute("pkill -f libexec/zed-editor")
        machine.wait_until_fails("pgrep -f libexec/zed-editor", timeout=30)


    def assert_identical(a, b, label):
        # `compare` exits 0 when identical, 1 when different; AE is the count
        # of differing pixels.
        status, out = machine.execute(
            f"compare -metric AE {a} {b} {SHOTS}/diff-{label}.png 2>&1"
        )
        assert status == 0, (
            f"{label}: screenshots differ ({out.strip()} pixels); "
            f"see diff-{label}.png in the test output"
        )
        machine.log(f"{label}: identical")


    machine.wait_for_x()
    machine.wait_for_unit("graphical.target")
    machine.sleep(5)

    for mode, env in MODES.items():
        with subtest(f"run zed ({mode})"):
            run_mode(mode, env)

    # Preserve all screenshots (and any diffs produced below) in the test
    # output for human inspection regardless of pass/fail.
    machine.execute(f"cp -r {SHOTS} /tmp/shots-copy")

    with subtest("reference run is internally deterministic"):
        # If this fails the *methodology* is flaky (timing, animations),
        # independent of damage tracking; fix the test, not the renderer.
        assert_identical(
            f"{SHOTS}/reference/03-scrolled.png",
            f"{SHOTS}/reference/04-after-full-redraw.png",
            "reference-self-check",
        )

    with subtest("partial rendering leaves no stale pixels after interactions"):
        assert_identical(
            f"{SHOTS}/partial/03-scrolled.png",
            f"{SHOTS}/partial/04-after-full-redraw.png",
            "partial-self-check",
        )

    with subtest("partial rendering matches the reference pixel-for-pixel"):
        for name in CHECKPOINTS:
            assert_identical(
                f"{SHOTS}/reference/{name}.png",
                f"{SHOTS}/partial/{name}.png",
                f"cross-{name}",
            )

    machine.copy_from_vm("/tmp/shots")
  '';
}
