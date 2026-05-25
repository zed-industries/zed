"""AT-SPI integration test for the GPUI a11y example app.

Walks the AT-SPI tree, finds the GPUI app, and exercises the counter
button, reset button, and toggle switch — asserting accessible state
after each interaction.
"""

import sys
import time
import pyatspi


def find_app():
    """Find the GPUI a11y example in the AT-SPI desktop."""
    desktop = pyatspi.Registry.getDesktop(0)
    for app in desktop:
        if "gpui" in app.name.lower() or "a11y" in app.name.lower():
            return app
    names = [a.name for a in desktop]
    raise AssertionError(f"GPUI app not found in AT-SPI desktop. Apps: {names}")


def find_by_role_and_label(root, role, label_substring):
    """Depth-first search for a node matching role and label substring."""
    for child in root:
        if child.getRole() == role and label_substring in (child.name or ""):
            return child
        result = find_by_role_and_label(child, role, label_substring)
        if result is not None:
            return result
    return None


def find_by_role(root, role):
    """Depth-first search for all nodes matching role."""
    results = []
    for child in root:
        if child.getRole() == role:
            results.append(child)
        results.extend(find_by_role(child, role))
    return results


def click(node):
    """Perform the Click action on a node."""
    actions = node.queryAction()
    for i in range(actions.nActions):
        if actions.getName(i) == "click":
            actions.doAction(i)
            time.sleep(0.5)
            return
    raise AssertionError(f"No 'click' action on node: {node.name} (role={node.getRoleName()})")


def get_toggled_state(node):
    """Return whether the node is in a 'checked'/'pressed' state."""
    state_set = node.getState()
    return state_set.contains(pyatspi.STATE_CHECKED) or state_set.contains(pyatspi.STATE_PRESSED)


def get_increment_button(app):
    button = find_by_role_and_label(app, pyatspi.ROLE_PUSH_BUTTON, "Count is")
    assert button is not None, "Increment button not found"
    return button


def get_reset_button(app):
    button = find_by_role_and_label(app, pyatspi.ROLE_PUSH_BUTTON, "Reset counter")
    assert button is not None, "Reset button not found"
    return button


def get_toggle_switch(app):
    switches = find_by_role(app, pyatspi.ROLE_TOGGLE_BUTTON)
    if not switches:
        raise AssertionError(
            f"No toggle switch found. Roles present: "
            f"{[(c.getRoleName(), c.name) for c in find_by_role(app, None) if True]}"
        )
    switch = None
    for s in switches:
        if "feature" in (s.name or "").lower() or "enable" in (s.name or "").lower():
            switch = s
            break
    if switch is None:
        switch = switches[0]
    return switch


def assert_count(app, expected):
    """Assert the increment button's label contains the expected count."""
    button = get_increment_button(app)
    expected_str = f"Count is {expected}."
    assert expected_str in button.name, (
        f"Expected label to contain '{expected_str}', got: '{button.name}'"
    )
    print(f"  OK: count is {expected}")


def run_tests():
    print("Finding GPUI app in AT-SPI tree...")
    app = find_app()
    print(f"Found app: {app.name}")

    # --- Counter button ---
    print("\n--- Counter button tests ---")

    print("Checking initial count is 0...")
    assert_count(app, 0)

    for i in range(1, 4):
        print(f"Clicking increment (expecting {i})...")
        button = get_increment_button(app)
        click(button)
        assert_count(app, i)

    print("Clicking reset...")
    reset = get_reset_button(app)
    click(reset)
    assert_count(app, 0)

    # --- Toggle switch ---
    print("\n--- Toggle switch tests ---")

    switch = get_toggle_switch(app)
    print(f"Switch: role={switch.getRoleName()}, name={switch.name}")

    toggled = get_toggled_state(switch)
    print(f"Initial toggle state: {toggled}")
    assert not toggled, f"Expected switch to be OFF initially, got {toggled}"
    print("  OK: switch is OFF")

    print("Toggling switch ON...")
    click(switch)
    switch = get_toggle_switch(app)
    toggled = get_toggled_state(switch)
    assert toggled, f"Expected switch to be ON after toggle, got {toggled}"
    print("  OK: switch is ON")

    print("Toggling switch OFF...")
    click(switch)
    switch = get_toggle_switch(app)
    toggled = get_toggled_state(switch)
    assert not toggled, f"Expected switch to be OFF after second toggle, got {toggled}"
    print("  OK: switch is OFF")

    print("\n=== ALL TESTS PASSED ===")


if __name__ == "__main__":
    try:
        run_tests()
    except Exception as e:
        print(f"\nFAILED: {e}", file=sys.stderr)
        sys.exit(1)
