cmake_minimum_required(VERSION 3.12)

include(${CMAKE_CURRENT_LIST_DIR}/features.cmake)

if(WASMTIME_HEADER_DST)
  set(dst "${WASMTIME_HEADER_DST}")
else()
  set(dst "${CMAKE_INSTALL_PREFIX}/include")
endif()
set(include_src "${CMAKE_CURRENT_LIST_DIR}/../include")

message(STATUS "Installing: ${dst}/wasmtime/conf.h")
file(READ "${include_src}/wasmtime/conf.h.in" conf_h)
file(CONFIGURE OUTPUT "${dst}/wasmtime/conf.h" CONTENT "${conf_h}"
     NEWLINE_STYLE CRLF)
file(INSTALL "${include_src}/"
     DESTINATION "${dst}"
     FILES_MATCHING REGEX "\\.hh?$")
