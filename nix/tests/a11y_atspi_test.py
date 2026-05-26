"""AT-SPI integration test for the GPUI a11y example app.

Walks the AT-SPI tree, finds the GPUI app, and exercises the counter
(spin button with increment/decrement), reset button, and toggle switch
— asserting accessible state after each interaction.
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


def do_action_by_name(node, action_name):
    """Perform a named action on a node."""
    actions = node.queryAction()
    for i in range(actions.nActions):
        if actions.getName(i).lower() == action_name.lower():
            actions.doAction(i)
            time.sleep(0.5)
            return
    available = [actions.getName(i) for i in range(actions.nActions)]
    raise AssertionError(
        f"No '{action_name}' action on node: {node.name} "
        f"(role={node.getRoleName()}). Available: {available}"
    )


def click(node):
    """Perform the Click action on a node."""
    do_action_by_name(node, "click")


def get_toggled_state(node):
    """Return whether the node is in a 'checked'/'pressed' state."""
    state_set = node.getState()
    return state_set.contains(pyatspi.STATE_CHECKED) or state_set.contains(pyatspi.STATE_PRESSED)


def get_counter(app):
    counter = find_by_role_and_label(app, pyatspi.ROLE_SPIN_BUTTON, "Counter:")
    assert counter is not None, "Counter (spin button) not found"
    return counter


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
    """Assert the counter's label contains the expected count."""
    counter = get_counter(app)
    expected_str = f"Counter: {expected}"
    assert expected_str in counter.name, (
        f"Expected label to contain '{expected_str}', got: '{counter.name}'"
    )
    print(f"  OK: count is {expected}")


def get_numeric_value(node):
    """Get the current numeric value from the AT-SPI Value interface."""
    value = node.queryValue()
    return value.currentValue


def run_tests():
    print("Finding GPUI app in AT-SPI tree...")
    app = find_app()
    print(f"Found app: {app.name}")

    # --- Counter (spin button) ---
    print("\n--- Counter spin button tests ---")

    print("Checking initial count is 0...")
    assert_count(app, 0)

    # Verify the Value interface reports 0
    counter = get_counter(app)
    val = get_numeric_value(counter)
    print(f"  Value interface reports: {val}")
    assert val == 0.0, f"Expected numeric value 0.0, got {val}"
    print("  OK: numeric value is 0")

    # Test click (increments)
    for i in range(1, 4):
        print(f"Clicking counter (expecting {i})...")
        counter = get_counter(app)
        click(counter)
        assert_count(app, i)

    # Verify the Value interface tracks the count
    counter = get_counter(app)
    val = get_numeric_value(counter)
    assert val == 3.0, f"Expected numeric value 3.0, got {val}"
    print("  OK: numeric value is 3 after 3 clicks")

    # List available actions for diagnostics
    counter = get_counter(app)
    actions = counter.queryAction()
    available = [actions.getName(i) for i in range(actions.nActions)]
    print(f"  Available actions on counter: {available}")

    # Test reset button
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

    # --- Window bounds / Component extents ---
    print("\n--- Component extents tests ---")

    counter = get_counter(app)
    component = counter.queryComponent()
    extents = component.getExtents(pyatspi.DESKTOP_COORDS)
    print(f"  Counter extents (desktop coords): x={extents.x}, y={extents.y}, "
          f"width={extents.width}, height={extents.height}")
    assert extents.width > 0 and extents.height > 0, (
        f"Expected non-zero extents from Component interface, got {extents}. "
        f"This likely means a11y_update_window_bounds is not reporting bounds."
    )
    print("  OK: counter has non-zero extents")

    print("\n=== ALL TESTS PASSED ===")


if __name__ == "__main__":
    try:
        run_tests()
    except Exception as e:
        print(f"\nFAILED: {e}", file=sys.stderr)
        sys.exit(1)
