cmake_minimum_required(VERSION 3.12)
project(wasmtime C)

set(WASMTIME_USER_CARGO_BUILD_OPTIONS "" CACHE STRING "Additional cargo flags (such as --features) to apply to the build command")
option(BUILD_SHARED_LIBS "Build using shared libraries" OFF)
option(BUILD_TESTS "Build tests" OFF)
option(WASMTIME_ALWAYS_BUILD "If cmake should always invoke cargo to build wasmtime" ON)
option(WASMTIME_FASTEST_RUNTIME "Set flags designed to optimize runtime performance" OFF)
set(WASMTIME_TARGET "" CACHE STRING "Rust target to build for")

if(WASMTIME_TARGET STREQUAL "")
  execute_process(COMMAND rustc -vV OUTPUT_VARIABLE RUSTC_VERSION)
  string(REGEX MATCH "host: ([^ \n]*)" RUSTC_HOST ${RUSTC_VERSION})
  string(STRIP ${CMAKE_MATCH_1} RUSTC_HOST_TARGET)
  set(WASMTIME_TARGET ${RUSTC_HOST_TARGET})
endif()

include(cmake/features.cmake)

if(WASMTIME_FASTEST_RUNTIME)
  set(WASMTIME_BUILD_TYPE_FLAG "--profile=fastest-runtime")
  set(WASMTIME_BUILD_TYPE "fastest-runtime")
  set(CARGO_PROFILE_PANIC CARGO_PROFILE_RELEASE_PANIC)
else()
  if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(WASMTIME_BUILD_TYPE "debug")
    set(CARGO_PROFILE_PANIC CARGO_PROFILE_DEBUG_PANIC)
  else()
    set(WASMTIME_BUILD_TYPE_FLAG "--release")
    set(WASMTIME_BUILD_TYPE "release")
    set(CARGO_PROFILE_PANIC CARGO_PROFILE_RELEASE_PANIC)
  endif()
endif()

set(WASMTIME_TARGET_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../../target/${WASMTIME_TARGET}/${WASMTIME_BUILD_TYPE})

if(WASMTIME_TARGET MATCHES "darwin")
  set(WASMTIME_SHARED_FILES libwasmtime.dylib)
  set(WASMTIME_STATIC_FILES libwasmtime.a)
elseif(WASMTIME_TARGET MATCHES "windows-gnu")
  set(WASMTIME_SHARED_FILES libwasmtime.dll.a wasmtime.dll)
  set(WASMTIME_STATIC_FILES libwasmtime.a)
elseif(WASMTIME_TARGET MATCHES "windows-msvc")
  set(WASMTIME_SHARED_FILES wasmtime.dll.lib wasmtime.dll)
  set(WASMTIME_STATIC_FILES wasmtime.lib)
else()
  set(WASMTIME_SHARED_FILES libwasmtime.so)
  set(WASMTIME_STATIC_FILES libwasmtime.a)
endif()

list(TRANSFORM WASMTIME_SHARED_FILES PREPEND ${WASMTIME_TARGET_DIR}/)
list(TRANSFORM WASMTIME_STATIC_FILES PREPEND ${WASMTIME_TARGET_DIR}/)

list(APPEND WASMTIME_BUILD_ENV "${CARGO_PROFILE_PANIC}=abort")
if (APPLE AND CMAKE_OSX_DEPLOYMENT_TARGET)
    # On macOS, we need to set the deployment target for the build
    list(APPEND WASMTIME_BUILD_ENV "MACOSX_DEPLOYMENT_TARGET=${CMAKE_OSX_DEPLOYMENT_TARGET}")
endif()

include(ExternalProject)
find_program(WASMTIME_CARGO_BINARY cargo)
if(NOT WASMTIME_CARGO_BINARY)
    message(FATAL_ERROR [["cargo" was not found. Ensure "cargo" is in PATH. Aborting...]])
endif()
ExternalProject_Add(
    wasmtime-crate
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    INSTALL_COMMAND "${WASMTIME_INSTALL_COMMAND}"
    BUILD_COMMAND
        ${CMAKE_COMMAND} -E env ${WASMTIME_BUILD_ENV}
        ${WASMTIME_CARGO_BINARY} build
          --target ${WASMTIME_TARGET}
          --package wasmtime-c-api
          ${WASMTIME_BUILD_TYPE_FLAG}
          ${WASMTIME_FEATURES}
          ${WASMTIME_USER_CARGO_BUILD_OPTIONS}
    USES_TERMINAL_BUILD TRUE
    # Note that this is used as the cwd for the cargo invocation itself, build
    # byproducts go in the `target` directory at the top-level.
    BINARY_DIR ${CMAKE_CURRENT_SOURCE_DIR}
    BUILD_ALWAYS ${WASMTIME_ALWAYS_BUILD}
    BUILD_BYPRODUCTS ${WASMTIME_SHARED_FILES} ${WASMTIME_STATIC_FILES})
add_library(wasmtime INTERFACE)
add_dependencies(wasmtime wasmtime-crate)

if(BUILD_SHARED_LIBS)
    if(NOT WASMTIME_TARGET MATCHES "windows")
        set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -Wl,-rpath='$ORIGIN'")
    endif()
    list(GET WASMTIME_SHARED_FILES 0 WASMTIME_SHARED_LIB_TO_LINK)
    target_link_libraries(wasmtime INTERFACE ${WASMTIME_SHARED_LIB_TO_LINK})
else()
    if(WASMTIME_TARGET MATCHES "windows")
        target_compile_options(wasmtime INTERFACE -DWASM_API_EXTERN= -DWASI_API_EXTERN=)
        target_link_libraries(wasmtime INTERFACE ${WASMTIME_STATIC_FILES}
            ws2_32 advapi32 userenv ntdll shell32 ole32 bcrypt)
    elseif(WASMTIME_TARGET MATCHES "darwin")
        target_link_libraries(wasmtime INTERFACE ${WASMTIME_STATIC_FILES}
            "-framework CoreFoundation")
    else()
        target_link_libraries(wasmtime INTERFACE ${WASMTIME_STATIC_FILES}
            pthread dl m)
    endif()
endif()

target_include_directories(wasmtime INTERFACE ${CMAKE_BINARY_DIR}/include)
set(WASMTIME_HEADER_DST ${CMAKE_BINARY_DIR}/include)
include(cmake/install-headers.cmake)

include(GNUInstallDirs)
install(SCRIPT ${CMAKE_CURRENT_SOURCE_DIR}/cmake/install-headers.cmake)
install(FILES ${WASMTIME_SHARED_FILES} ${WASMTIME_STATIC_FILES}
        DESTINATION ${CMAKE_INSTALL_LIBDIR})

if(WASMTIME_TARGET MATCHES "darwin")
  # Postprocess the macOS dylib a bit to have a more reasonable `LC_ID_DYLIB`
  # directive than the default one that comes out of the linker when typically
  # doing `cargo build`. For more info see #984
  set(INSTALLED_LIB ${CMAKE_INSTALL_PREFIX}/${CMAKE_INSTALL_LIBDIR}/libwasmtime.dylib)
  install(CODE "execute_process(COMMAND install_name_tool -id \"@rpath/libwasmtime.dylib\" ${INSTALLED_LIB})")
endif()

set(DOXYGEN_CONF_IN ${CMAKE_CURRENT_SOURCE_DIR}/doxygen.conf.in)
set(DOXYGEN_CONF_OUT ${CMAKE_BINARY_DIR}/doxygen.conf)
configure_file(${DOXYGEN_CONF_IN} ${DOXYGEN_CONF_OUT})
add_custom_target(doc
    COMMAND doxygen ${DOXYGEN_CONF_OUT}
    DEPENDS ${WASMTIME_GENERATED_CONF_H} ${DOXYGEN_CONF_OUT}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
add_dependencies(doc headers-to-doc)

file(GLOB headers "include/*.h")
add_custom_target(headers-to-doc
  COMMAND
    ${CMAKE_COMMAND}
      -DWASMTIME_HEADER_DST=${CMAKE_BINARY_DIR}/include
      -P ${CMAKE_CURRENT_SOURCE_DIR}/cmake/install-headers.cmake
  DEPENDS ${headers})

if (NOT CMAKE_CXX_STANDARD)
  message(STATUS "Cannot detect C++ Standard. Switching to C++17 by default !!")
  set(CMAKE_CXX_STANDARD 17)
endif()
message(STATUS "CMAKE_CXX_STANDARD is ${CMAKE_CXX_STANDARD}")
if (NOT CMAKE_CXX_STANDARD GREATER_EQUAL 17)
  message(FATAL_ERROR "WASMTIME_CPP library does not support ${CMAKE_CXX_STANDARD}")
endif()
set(CMAKE_CXX_STANDARD_REQUIRED True)

option(ENABLE_CODE_ANALYSIS "Run code analysis" OFF)
message(STATUS "ENABLE_CODE_ANALYSIS       ${ENABLE_CODE_ANALYSIS}")

if ("${CMAKE_CXX_COMPILER_ID}" STREQUAL "GNU")
   add_compile_options (-fdiagnostics-color=always)
elseif ("${CMAKE_CXX_COMPILER_ID}" STREQUAL "Clang")
   add_compile_options (-fcolor-diagnostics)
endif ()

add_library(wasmtime-cpp INTERFACE)
target_link_libraries(wasmtime-cpp INTERFACE wasmtime)
if (MSVC)
  target_compile_options(wasmtime-cpp INTERFACE /DWASM_API_EXTERN= /DWASI_API_EXTERN=)
  target_link_libraries(wasmtime-cpp INTERFACE ws2_32 bcrypt advapi32 userenv ntdll shell32 ole32)
else()
  target_link_libraries(wasmtime-cpp INTERFACE stdc++ pthread)
endif()

target_include_directories(
        wasmtime-cpp
        INTERFACE
        ${PROJECT_SOURCE_DIR}/include)

if (BUILD_TESTS)
  message(STATUS "Building tests")
  set(CMAKE_CXX_STANDARD 20)

  enable_language(CXX)
  set(INSTALL_GTEST OFF CACHE BOOL "" FORCE)

  enable_testing()
  add_subdirectory(tests)
endif()
