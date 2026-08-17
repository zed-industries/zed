# Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0 OR ISC

cmake_minimum_required(VERSION 3.5...3.31)

project(AWS_LC_RUST NONE)
enable_language(C)

function(set_my_target_properties ...)
    set_target_properties(crypto PROPERTIES ${ARGV})
    if (BUILD_LIBSSL)
        set_target_properties(ssl PROPERTIES ${ARGV})
    endif()
endfunction()

if(BUILD_SHARED_LIBS)
    add_definitions(-DBORINGSSL_SHARED_LIBRARY)
endif()

add_subdirectory(aws-lc aws-lc EXCLUDE_FROM_ALL)
if(NOT EXISTS "${AWS_LC_RUST_SOURCE_DIR}/aws-lc/CMakeLists.txt")
message(WARNING "###### WARNING: MISSING GIT SUBMODULE ###### Did you initialize the repo's git submodules? CMakeLists.txt not found.\n -- run 'git submodule update --init --recursive' to initialize.")
endif()

if (BUILD_LIBSSL)
    add_definitions(-DAWS_LC_RUST_INCLUDE_SSL)
endif()

# Custom target needed because aws-lc is added with EXCLUDE_FROM_ALL, so its targets aren't built by default.
if (BUILD_LIBSSL)
    add_custom_target(aws_lc_sys ALL DEPENDS crypto ssl)
else()
    add_custom_target(aws_lc_sys ALL DEPENDS crypto)
endif()

# The aws-lc Windows FIPS shared-library build may define a fips_integrity
# target that patches the correct integrity hash into crypto.dll after it is
# linked. Because aws-lc is added with EXCLUDE_FROM_ALL, that target would not
# be part of this project's ALL target and would otherwise never be built.
if (FIPS AND TARGET fips_integrity)
    add_dependencies(aws_lc_sys fips_integrity)
endif()

set(FINAL_ARTIFACTS_DIRECTORY ${CMAKE_BINARY_DIR}/artifacts)

# Places the compiled library files at the root of the binary directory so we can have a consistent
# location to find the artifacts cross-platform.
set_my_target_properties(
        ARCHIVE_OUTPUT_DIRECTORY ${FINAL_ARTIFACTS_DIRECTORY}
        RUNTIME_OUTPUT_DIRECTORY ${FINAL_ARTIFACTS_DIRECTORY}
        LIBRARY_OUTPUT_DIRECTORY ${FINAL_ARTIFACTS_DIRECTORY})

# Based on https://stackoverflow.com/a/7750816 as some generators, like MSVC, will try to prefix the output directory
# which is not needed in our case.
foreach (OUT_NAME ${CMAKE_CONFIGURATION_TYPES})
    string(TOUPPER ${OUT_NAME} OUT_NAME)
    set_my_target_properties(
            ARCHIVE_OUTPUT_DIRECTORY_${OUT_NAME} ${FINAL_ARTIFACTS_DIRECTORY}
            RUNTIME_OUTPUT_DIRECTORY_${OUT_NAME} ${FINAL_ARTIFACTS_DIRECTORY}
            LIBRARY_OUTPUT_DIRECTORY_${OUT_NAME} ${FINAL_ARTIFACTS_DIRECTORY})
endforeach ()

if (BORINGSSL_PREFIX)
    if (MSVC)
        set(TARGET_PREFIX "${BORINGSSL_PREFIX}")
        set_my_target_properties(IMPORT_PREFIX ${TARGET_PREFIX})
    else()
        set(TARGET_PREFIX "lib${BORINGSSL_PREFIX}")
    endif()
    set_my_target_properties(PREFIX ${TARGET_PREFIX})

    # This BORINGSSL_PREFIX has an "_" appended, so we must remove it
    string(REGEX REPLACE "_$" "" BORINGSSL_PREFIX_MACRO ${BORINGSSL_PREFIX})
    add_definitions(-DBORINGSSL_PREFIX=${BORINGSSL_PREFIX_MACRO})
endif ()
