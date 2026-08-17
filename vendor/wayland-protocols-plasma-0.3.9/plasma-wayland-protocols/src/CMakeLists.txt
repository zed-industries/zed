# SPDX-FileCopyrightText: 2020 Aleix Pol Gonzalez <aleixpol@kde.org>
#
# SPDX-License-Identifier: BSD-3-Clause

install(FILES
    protocols/appmenu.xml
    protocols/blur.xml
    protocols/contrast.xml
    protocols/dpms.xml
    protocols/fake-input.xml
    protocols/fullscreen-shell.xml
    protocols/idle.xml
    protocols/kde-external-brightness-v1.xml
    protocols/kde-lockscreen-overlay-v1.xml
    protocols/kde-output-device-v2.xml
    protocols/kde-output-management-v2.xml
    protocols/kde-output-order-v1.xml
    protocols/kde-primary-output-v1.xml
    protocols/kde-screen-edge-v1.xml
    protocols/keystate.xml
    protocols/org-kde-plasma-virtual-desktop.xml
    protocols/output-management.xml
    protocols/outputdevice.xml
    protocols/plasma-shell.xml
    protocols/plasma-window-management.xml
    protocols/remote-access.xml
    protocols/server-decoration-palette.xml
    protocols/server-decoration.xml
    protocols/shadow.xml
    protocols/slide.xml
    protocols/surface-extension.xml
    protocols/text-input-unstable-v2.xml
    protocols/text-input.xml
    protocols/wayland-eglstream-controller.xml
    protocols/zkde-screencast-unstable-v1.xml

    DESTINATION ${KDE_INSTALL_DATADIR}/plasma-wayland-protocols)

# Backward compatibility for previously used non-standard protocol file names
# TODO KF6 remove
install(FILES protocols/zkde-screencast-unstable-v1.xml RENAME screencast.xml DESTINATION ${KDE_INSTALL_DATADIR}/plasma-wayland-protocols)
install(FILES protocols/org-kde-plasma-virtual-desktop.xml RENAME plasma-virtual-desktop.xml DESTINATION ${KDE_INSTALL_DATADIR}/plasma-wayland-protocols)
