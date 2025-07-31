#!/bin/sh

flatpak remote-add --if-not-exists --user flathub https://dl.flathub.org/repo/flathub.flatpakrepo

arch=$(arch)
fd_version=23.08
flatpak install -y --user org.freedesktop.Platform/${arch}/${fd_version}
flatpak install -y --user org.freedesktop.Sdk/${arch}/${fd_version}
flatpak install -y --user org.freedesktop.Sdk.Extension.rust-stable/${arch}/${fd_version}
