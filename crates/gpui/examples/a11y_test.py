#!/usr/bin/env python3
"""Drive the GPUI a11y example app via AT-SPI.

Usage:
    python3 crates/gpui/examples/a11y_test.py
"""

import os
import signal
import subprocess
import sys
import time

import pyatspi


def find_app():
    """Walk the AT-SPI desktop looking for the a11y example."""
    desktop = pyatspi.Registry.getDesktop(0)
    for app in desktop:
        if app is None:
            continue
        if "a11y" in app.name.lower():
            return app
    return None


def find_by_role(root, role, name_substring=None):
    """DFS for the first accessible matching role (and optional name substring)."""
    for child in root:
        if child is None:
            continue
        if child.getRole() == role:
            if name_substring is None or name_substring.lower() in child.name.lower():
                return child
        result = find_by_role(child, role, name_substring)
        if result is not None:
            return result
    return None


def find_all_by_role(root, role):
    """Collect every accessible under root that matches role."""
    results = []
    for child in root:
        if child is None:
            continue
        if child.getRole() == role:
            results.append(child)
        results.extend(find_all_by_role(child, role))
    return results


def do_action(node, action_name):
    """Invoke a named action on a node via the AT-SPI Action interface."""
    try:
        actions = node.queryAction()
    except NotImplementedError:
        return False
    for i in range(actions.nActions):
        if actions.getName(i).lower() == action_name.lower():
            actions.doAction(i)
            return True
    return False


def dump_tree(node, indent=0):
    """Print the a11y tree for debugging."""
    role = node.getRole()
    name = node.name or ""
    print(f"{'  ' * indent}{role.value_nick:30s}  {name!r}")
    for child in node:
        if child is not None:
            dump_tree(child, indent + 1)


def main():
    # Launch the example app in its own process group so we can kill the
    # entire tree (cargo + the child binary) reliably.
    print("Starting a11y example app...")
    proc = subprocess.Popen(
        [
            "cargo",
            "run",
            "-p",
            "gpui",
            "--features",
            "gpui_platform/wayland,gpui_platform/x11",
            "--example",
            "a11y",
        ],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        preexec_fn=os.setsid,
    )

    def cleanup():
        try:
            os.killpg(os.getpgid(proc.pid), signal.SIGTERM)
        except ProcessLookupError:
            pass
        proc.wait()

    signal.signal(signal.SIGINT, lambda *_: (cleanup(), sys.exit(1)))
    signal.signal(signal.SIGTERM, lambda *_: (cleanup(), sys.exit(1)))

    # Wait for the app to appear in the AT-SPI tree
    app = None
    for attempt in range(30):
        time.sleep(1)
        app = find_app()
        if app is not None:
            break
        print(f"  waiting... ({attempt + 1}s)")

    if app is None:
        print("Could not find the app after 30 seconds.")
        sys.exit(1)

    print(f"Found app: {app.name!r}")
    print()

    # Give the app a moment to render its first frame with a11y active
    time.sleep(2)

    # Dump the tree
    print("=== Accessibility tree ===")
    dump_tree(app)
    print()

    # --- Buttons: test Click ---
    increment = find_by_role(app, pyatspi.ROLE_PUSH_BUTTON, "count")
    if increment:
        print(f"Found increment button: {increment.name!r}")
        print("Clicking 3 times...")
        for i in range(3):
            time.sleep(1)
            do_action(increment, "click")
            time.sleep(0.5)
            print(f"  After click {i + 1}: {increment.name!r}")
    else:
        print("Could not find increment button")

    time.sleep(1)

    reset = find_by_role(app, pyatspi.ROLE_PUSH_BUTTON, "reset")
    if reset:
        print(f"\nFound reset button: {reset.name!r}")
        print("Clicking reset...")
        time.sleep(1)
        do_action(reset, "click")
        time.sleep(0.5)
        if increment:
            print(f"  Increment after reset: {increment.name!r}")
    else:
        print("\nCould not find reset button")

    time.sleep(1)

    # --- Toggle switch ---
    switch = find_by_role(app, pyatspi.ROLE_TOGGLE_BUTTON, "enable")
    if switch:
        print(f"\nFound switch: {switch.name!r}")
        print("Clicking switch on...")
        time.sleep(1)
        do_action(switch, "click")
        time.sleep(0.5)
        print(f"  After click: {switch.name!r}")

        print("Clicking switch off...")
        time.sleep(1)
        do_action(switch, "click")
        time.sleep(0.5)
        print(f"  After click: {switch.name!r}")
    else:
        print("\nCould not find switch")

    # --- List items ---
    list_items = find_all_by_role(app, pyatspi.ROLE_LIST_ITEM)
    if list_items:
        print(f"\nFound {len(list_items)} list items:")
        for item in list_items:
            print(f"  - {item.name!r}")

    print("\nDone!")
    cleanup()


if __name__ == "__main__":
    main()
