echo "Starting Zed polkit setup..."

if ! command -v pkexec >/dev/null 2>&1; then
    echo "Error: 'pkexec' command not found. Please install polkit on your system."
    echo "For Ubuntu/Debian: sudo apt install policykit-1"
    echo "For Fedora: sudo dnf install polkit"
    echo "For Arch Linux: sudo pacman -S polkit"
    exit 1
fi

LIBEXEC_DIR="/usr/libexec/zed"
ELEVATE_SCRIPT="$LIBEXEC_DIR/elevate.sh"
POLKIT_DIR="/usr/share/polkit-1/actions"

ELEVATE_SCRIPT_CONTENT='#!/bin/bash
eval "$@"'

POLICY_CONTENT='<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE policyconfig PUBLIC
 "-//freedesktop//DTD PolicyKit Policy Configuration 1.0//EN"
 "http://www.freedesktop.org/standards/PolicyKit/1/policyconfig.dtd">
<policyconfig>
  <action id="org.zed.app">
    <description>Run Zed with elevated privileges</description>
    <message>Zed needs temporary elevated access to make changes. Please enter your password.</message>
    <defaults>
      <allow_any>auth_admin</allow_any>
      <allow_inactive>auth_admin</allow_inactive>
      <allow_active>auth_admin</allow_active>
    </defaults>
    <annotate key="org.freedesktop.policykit.exec.path">/usr/libexec/zed/elevate.sh</annotate>
  </action>
</policyconfig>'


echo "Installing elevation script..."
if sudo mkdir -p "$LIBEXEC_DIR" && \
    echo "$ELEVATE_SCRIPT_CONTENT" | sudo tee "$ELEVATE_SCRIPT" > /dev/null && \
    sudo chmod 755 "$ELEVATE_SCRIPT"; then
    echo "Successfully set up elevation script"
else
    echo "Failed to set up elevation script"
    exit 1
fi

echo "Installing polkit policy..."
if sudo mkdir -p "$POLKIT_DIR" && \
    echo "$POLICY_CONTENT" | sudo tee "$POLKIT_DIR/org.zed.app.policy" > /dev/null && \
    sudo chmod 644 "$POLKIT_DIR/org.zed.app.policy"; then
    echo "Successfully set up policy file"
else
    echo "Failed to set up policy file"
    exit 1
fi

echo "Zed polkit setup completed successfully!"