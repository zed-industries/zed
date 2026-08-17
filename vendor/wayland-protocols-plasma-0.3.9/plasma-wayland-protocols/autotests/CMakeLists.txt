# SPDX-License-Identifier: BSD-3-Clause
# SPDX-FileCopyrightText: 2024 David Redondo <kde@david-redondo.de>

file(GLOB PROTOCOLS CONFIGURE_DEPENDS ${PROJECT_SOURCE_DIR}/src/protocols/*.xml)

foreach(PROTOCOL IN LISTS PROTOCOLS)
    get_filename_component(PROTOCOL_BASENAME ${PROTOCOL} NAME_WE)
    set(CLIENT_HEADER  ${PROTOCOL_BASENAME}-client-protocol.h)
    set(SERVER_HEADER ${PROTOCOL_BASENAME}-server-protocol.h)
    set(CODE ${PROTOCOL_BASENAME}-protocol.c)
    add_test(NAME ${PROTOCOL_BASENAME}_client-header COMMAND Wayland::Scanner --strict client-header ${PROTOCOL} ${CLIENT_HEADER})
    add_test(NAME ${PROTOCOL_BASENAME}_server-header COMMAND Wayland::Scanner --strict client-header ${PROTOCOL} ${SERVER_HEADER})
    add_test(NAME ${PROTOCOL_BASENAME}_code COMMAND Wayland::Scanner --strict private-code ${PROTOCOL} ${CODE})
    # fail if there is any warning from wayland-scanner, except plasma-window-managment which has preexisting issues
    if (NOT ${PROTOCOL_BASENAME} STREQUAL "plasma-window-management")
        set_tests_properties(${PROTOCOL_BASENAME}_client-header ${PROTOCOL_BASENAME}_server-header ${PROTOCOL_BASENAME}_code PROPERTIES FAIL_REGULAR_EXPRESSION ".+")
    endif()
    configure_file(build.c build-${PROTOCOL_BASENAME}.c @ONLY)
    add_test(NAME ${PROTOCOL_BASENAME}_build COMMAND ${CMAKE_C_COMPILER} build-${PROTOCOL_BASENAME}.c ${CODE} -I ${Wayland_INCLUDE_DIRS} -Wl,--unresolved-symbols=ignore-all)
endforeach()
