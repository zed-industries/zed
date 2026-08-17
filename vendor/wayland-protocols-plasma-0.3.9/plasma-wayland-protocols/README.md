# Plasma Wayland Protocols

This project provides the xml files of the non-standard wayland
protocols we use in Plasma.

They are installed to $PREFIX/share/plasma-wayland-protocols.

## Usage
You can get the directory where they're installed by using

    find_package(PlasmaWaylandProtocols)

Then they can be accessed using `${PLASMA_WAYLAND_PROTOCOLS_DIR}`.

You can learn more about such protocol files in
https://wayland.freedesktop.org/docs/html/ch04.html.
