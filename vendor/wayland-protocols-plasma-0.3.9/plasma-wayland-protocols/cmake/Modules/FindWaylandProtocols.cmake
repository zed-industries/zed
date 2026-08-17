#.rst:
# FindWaylandProtocols
# -------
#
# Try to find wayland-protocols on a Unix system.
#
# This will define the following variables:
#
# ``WaylandProtocols_FOUND``
#     True if (the requested version of) wayland-protocols is available
# ``WaylandProtocols_VERSION``
#     The version of wayland-protocols
# ``WaylandProtocols_DATADIR``
#     The wayland protocols data directory

#=============================================================================
# SPDX-FileCopyrightText: 2019 Vlad Zahorodnii <vlad.zahorodnii@kde.org>
#
# SPDX-License-Identifier: BSD-3-Clause
#=============================================================================

find_package(PkgConfig)
pkg_check_modules(PKG_wayland_protocols QUIET wayland-protocols)

set(WaylandProtocols_VERSION ${PKG_wayland_protocols_VERSION})
pkg_get_variable(WaylandProtocols_DATADIR wayland-protocols pkgdatadir)

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(WaylandProtocols
    FOUND_VAR WaylandProtocols_FOUND
    REQUIRED_VARS WaylandProtocols_DATADIR
    VERSION_VAR WaylandProtocols_VERSION
)

include(FeatureSummary)
set_package_properties(WaylandProtocols PROPERTIES
    DESCRIPTION "Specifications of extended Wayland protocols"
    URL "https://wayland.freedesktop.org/"
)
