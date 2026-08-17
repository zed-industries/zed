# SPDX-FileCopyrightText: 2020 Aleix Pol Gonzalez <aleixpol@kde.org>
#
# SPDX-License-Identifier: BSD-3-Clause

cmake_minimum_required(VERSION 3.5)

project(PlasmaWaylandProtocols VERSION 1.16.0)

include(FeatureSummary)
include(CMakePackageConfigHelpers)

# ECM setup
find_package(ECM 5.69.0  NO_MODULE)
set_package_properties(ECM PROPERTIES TYPE REQUIRED DESCRIPTION "Extra CMake Modules." URL "https://commits.kde.org/extra-cmake-modules")
feature_summary(WHAT REQUIRED_PACKAGES_NOT_FOUND FATAL_ON_MISSING_REQUIRED_PACKAGES)
set(CMAKE_MODULE_PATH ${ECM_MODULE_PATH} ${CMAKE_SOURCE_DIR}/cmake/Modules)
include(KDEInstallDirs)
include(KDECMakeSettings)
include(ECMSetupVersion)

add_subdirectory(src)

if (BUILD_TESTING)
    find_package(WaylandScanner REQUIRED)
    find_package(Wayland COMPONENTS Client REQUIRED)
    add_subdirectory(autotests)
endif()

# create a Config.cmake and a ConfigVersion.cmake file and install them
set(CMAKECONFIG_INSTALL_DIR "${KDE_INSTALL_CMAKEPACKAGEDIR}/PlasmaWaylandProtocols")

configure_package_config_file("${CMAKE_CURRENT_SOURCE_DIR}/PlasmaWaylandProtocolsConfig.cmake.in"
                              "${CMAKE_CURRENT_BINARY_DIR}/PlasmaWaylandProtocolsConfig.cmake"
                              INSTALL_DESTINATION  ${CMAKECONFIG_INSTALL_DIR}
                              PATH_VARS KDE_INSTALL_DATADIR
                              )
ecm_setup_version(PROJECT VARIABLE_PREFIX PLASMA_WAYLAND_PROTOCOLS
                        VERSION_HEADER "${CMAKE_CURRENT_BINARY_DIR}/plasma_wayland_protocols_version.h"
                        PACKAGE_VERSION_FILE "${CMAKE_CURRENT_BINARY_DIR}/PlasmaWaylandProtocolsConfigVersion.cmake"
                        SOVERSION 5)

install(FILES  "${CMAKE_CURRENT_BINARY_DIR}/PlasmaWaylandProtocolsConfig.cmake"
               "${CMAKE_CURRENT_BINARY_DIR}/PlasmaWaylandProtocolsConfigVersion.cmake"
        DESTINATION "${CMAKECONFIG_INSTALL_DIR}"
        COMPONENT Devel )

feature_summary(WHAT ALL FATAL_ON_MISSING_REQUIRED_PACKAGES)
