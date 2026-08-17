# Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0 OR ISC
#
# This CMakeLists.txt file defines the rules to build libjitterentropy.a.
# The compilation flags that are used are taken from Makefile in:
#   https://github.com/smuellerDD/jitterentropy-library,
# the same as the source code.

set(JITTER_SOURCES
        ${PROJECT_SOURCE_DIR}/third_party/jitterentropy/jitterentropy-library/src/jitterentropy-base.c
        ${PROJECT_SOURCE_DIR}/third_party/jitterentropy/jitterentropy-library/src/jitterentropy-gcd.c
        ${PROJECT_SOURCE_DIR}/third_party/jitterentropy/jitterentropy-library/src/jitterentropy-health.c
        ${PROJECT_SOURCE_DIR}/third_party/jitterentropy/jitterentropy-library/src/jitterentropy-noise.c
        ${PROJECT_SOURCE_DIR}/third_party/jitterentropy/jitterentropy-library/src/jitterentropy-sha3.c
        ${PROJECT_SOURCE_DIR}/third_party/jitterentropy/jitterentropy-library/src/jitterentropy-timer.c)

include_directories(${AWSLC_SOURCE_DIR}/include)

if(WIN32)
    if(MSVC)
        set(JITTER_COMPILE_FLAGS "/Od /W4 /DYNAMICBASE /DAWSLC")
    else()
        set(JITTER_COMPILE_FLAGS "-DAWSLC -fwrapv --param ssp-buffer-size=4 -fvisibility=hidden -Wcast-align -Wmissing-field-initializers -Wshadow -Wswitch-enum -Wextra -Wall -pedantic -O0 -fwrapv -Wconversion")
    endif()
else()
    set(CMAKE_POSITION_INDEPENDENT_CODE true)
    set(JITTER_COMPILE_FLAGS "-DAWSLC -fwrapv --param ssp-buffer-size=4 -fvisibility=hidden -Wcast-align -Wmissing-field-initializers -Wshadow -Wswitch-enum -Wextra -Wall -pedantic -O0 -fwrapv -Wp,-U_FORTIFY_SOURCE")
    if ((NOT GCC) OR (GCC AND CMAKE_C_COMPILER_VERSION VERSION_GREATER "4.3"))
        # -Wconversion was changed from GCC version 4.3. Prior it was meant as
        # an aid in translating code from old C to modern C. It was not meant
        # to help detect troublesome implicit conversions.
        # https://gcc.gnu.org/wiki/NewWconversion.
        set(JITTER_COMPILE_FLAGS "${JITTER_COMPILE_FLAGS} -Wconversion")
    endif()

    # See t/P319995887.
    # At the moment, CPU Jitter source code needs to disable all optimizations.
    # This means we can't inherit FORTITY_SOURCE and we therefore undefine
    # the flag through U_FORTIFY_SOURCE. If one were to use annobin as a plugin
    # and, simultaneously use LTO, annobin will emit a warning if FORTIFY_SOURCE
    # is undefined. This is fatal if all warnings are treated as errors.
    #
    # The correct signal to use for below is annobin. But since we know that
    # annobin is not necessarily being enabled through -fplugin, match with LTO
    # and disable turning warnings into errors. This is also kinda fragile; LTO
    # could be defined through other means similar to annobin. But it's what we
    # have to work with atm.
    if ("lto" MATCHES "${CMAKE_CFLAGS}")
        set(JITTER_COMPILE_FLAGS "${JITTER_COMPILE_FLAGS} -Wno-error")
    endif()
endif()

if(BORINGSSL_PREFIX)
  if(MSVC)
    set(JITTER_ENETROPY_PREFIX_INCLUDE "/FI \"${PROJECT_BINARY_DIR}/symbol_prefix_include/openssl/boringssl_prefix_symbols.h\"")
  else()
    set(JITTER_ENETROPY_PREFIX_INCLUDE "--include=\"${PROJECT_BINARY_DIR}/symbol_prefix_include/openssl/boringssl_prefix_symbols.h\"")
  endif()
endif()

set(JITTER_COMPILE_FLAGS "${JITTER_COMPILE_FLAGS} ${JITTER_ENETROPY_PREFIX_INCLUDE}")

set_source_files_properties(${JITTER_SOURCES} PROPERTIES COMPILE_FLAGS "${JITTER_COMPILE_FLAGS}")
add_library(jitterentropy OBJECT ${JITTER_SOURCES})

target_include_directories(jitterentropy BEFORE PRIVATE "${PROJECT_SOURCE_DIR}/third_party/jitterentropy/jitterentropy-library")
target_add_awslc_include_paths(TARGET jitterentropy SCOPE PRIVATE)
