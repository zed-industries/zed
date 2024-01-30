#!/bin/bash
set -e

apt-get install -y xvfb

cat <<EOT >/etc/systemd/system/xvfb.service
[Unit]
Description=X Virtual Frame Buffer Service
After=network.target
[Service]
ExecStart=/usr/bin/Xvfb :99 -screen 0 1024x768x24
[Install]
WantedBy=multi-user.target
EOT

systemctl enable xvfb.service
systemctl start xvfb.service
