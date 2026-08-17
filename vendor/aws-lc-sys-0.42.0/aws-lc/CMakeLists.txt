cmake_minimum_required(VERSION 3.5..3.31)

if(POLICY CMP0091)
cmake_policy(SET CMP0091 NEW)
endif()

set(SOFTWARE_NAME "awslc")
set(SOFTWARE_VERSION "5.1.0")
set(AWSLC_FIPS_VERSION 5)
set(ABI_VERSION 0)
set(CRYPTO_LIB_NAME "crypto")
set(SSL_LIB_NAME "ssl")
set(OPENSSL_VERSION_NUMBER "0x1010107f")

# Note this is set to 1.1.1 for compatibility purposes
# with projects that use the pkg-config information to determine
# the OpenSSL version.
set(REPORTED_PKGCONFIG_VERSION "1.1.1")

# Defer enabling C and CXX languages.
project(AWSLC VERSION "${SOFTWARE_VERSION}" LANGUAGES NONE)

if(DEFINED ENABLE_PRE_SONAME_BUILD)
  message(WARNING "ENABLE_PRE_SONAME_BUILD option will be deprecated in a future release. Please see ENABLE_DIST_PKG and ENABLE_DIST_PKG_OPENSSL_SHIM")
endif()

option(BUILD_TESTING "Build all test targets for AWS-LC" ON)
option(BUILD_LIBSSL "Build libssl for AWS-LC" ON)
option(BUILD_TOOL "Build bssl tool for AWS-LC" ON)
option(DISABLE_PERL "Disable Perl for AWS-LC" OFF)
option(DISABLE_GO "Disable Go for AWS-LC" OFF)
# Keeping this flag for now, for compatibility with existing build configs.
option(ENABLE_FIPS_ENTROPY_CPU_JITTER "Enable FIPS entropy source: CPU Jitter" OFF)
option(ENABLE_DATA_INDEPENDENT_TIMING "Enable automatic setting/resetting Data-Independent Timing
(DIT) flag in cryptographic functions. Currently only applicable to Arm64 (except on Windows)" OFF)
option(ENABLE_PRE_SONAME_BUILD "Build AWS-LC without SONAME configuration for shared library builds" ON)
option(ENABLE_SOURCE_MODIFICATION "Allow the build to update files in the source directory. This is typically done to update versioning." ON)
option(DISABLE_CPU_JITTER_ENTROPY "Disable usage of CPU Jitter Entropy as an entropy source. This option cannot be used with the FIPS build. With this configuration, randomness generation might not use two independent entropy sources." OFF)
option(GENERATE_RUST_BINDINGS "Generate Rust bindings using bindgen-cli" OFF)
option(ENABLE_DIST_PKG "Enables a set of packaging that take highest precedence to any other packaging configuration i.e. ENABLE_PRE_SONAME_BUILD" OFF)
option(ENABLE_DIST_PKG_OPENSSL_SHIM "Controls whether the OpenSSL shim components are installed when ENABLE_DIST_PKG is enabled" OFF)

if(MSVC)
  # On Windows, prefer cl over gcc if both are available. By default most of
  # the CMake generators prefer gcc, even on Windows.
  set(CMAKE_GENERATOR_CC cl)
endif()

include(sources.cmake)
include(TestBigEndian)
include(CheckCCompilerFlag)

message(STATUS "Versioning name:${SOFTWARE_NAME} version:${SOFTWARE_VERSION} abi:${ABI_VERSION}")
if(BUILD_LIBSSL)
  message(STATUS "Libraries crypto:${CRYPTO_LIB_NAME} ssl:${SSL_LIB_NAME}")
else()
  message(STATUS "Libraries crypto:${CRYPTO_LIB_NAME}")
endif()

macro(add_flag_if_supported VARIABLE FLAG)
  # Create a safe, unique variable name to cache the result of the check
  string(MAKE_C_IDENTIFIER "HAVE_C_FLAG_${FLAG}" _CHECK_VAR_NAME)

  # Keep configure output tidy
  set(CMAKE_REQUIRED_QUIET ${CMAKE_NOT_VERBOSE})

  # Run the check only once to speed up re-configuration
  if(NOT DEFINED ${_CHECK_VAR_NAME})
    check_c_compiler_flag("${FLAG}" ${_CHECK_VAR_NAME})
  endif()

  # If the check passed, append the flag to the specified variable
  if(${_CHECK_VAR_NAME})
    set(${VARIABLE} "${${VARIABLE}} ${FLAG}")
    message(STATUS "Compiler supports '${FLAG}', adding to ${VARIABLE}")
  endif()
endmacro()

if(POLICY CMP0077)
  cmake_policy(SET CMP0077 NEW) #option does nothing when a normal variable of the same name exists.
endif()

set(RUST_BINDINGS_TARGET_VERSION "1.70" CACHE STRING "Minimum Rust version for generated bindings")

include(cmake/go.cmake)

if(ENABLE_DIST_PKG_OPENSSL_SHIM AND NOT ENABLE_DIST_PKG)
  message(FATAL_ERROR "ENABLE_DIST_PKG_OPENSSL_SHIM requires ENABLE_DIST_PKG to be enabled and will be ignored.")
endif()

if(ENABLE_DIST_PKG)
  if(NOT UNIX OR APPLE)
    message(FATAL_ERROR "ENABLE_DIST_PKG is not supported on macOS or Windows and will be ignored.")
  else()
    set(SET_LIB_SONAME 1)
    set(COHABITANT_HEADERS 1)
    set(COHABITANT_BINARIES 1)
    if(ENABLE_DIST_PKG_OPENSSL_SHIM)
      set(INSTALL_OPENSSL_SHIM 1)
    else()
      set(INSTALL_OPENSSL_SHIM 0)
    endif()
  endif()
elseif(NOT ENABLE_PRE_SONAME_BUILD AND BUILD_SHARED_LIBS AND UNIX AND NOT APPLE)
  set(SET_LIB_SONAME 1)
  set(COHABITANT_HEADERS 0)
  set(COHABITANT_BINARIES 0)
  set(INSTALL_OPENSSL_SHIM 1)
else()
  set(SET_LIB_SONAME 0)
  set(COHABITANT_HEADERS 0)
  set(COHABITANT_BINARIES 0)
  set(INSTALL_OPENSSL_SHIM 1)
endif()

message(STATUS "SET_LIB_SONAME: ${SET_LIB_SONAME}")
message(STATUS "COHABITANT_HEADERS: ${COHABITANT_HEADERS}")
message(STATUS "COHABITANT_BINARIES: ${COHABITANT_BINARIES}")
message(STATUS "INSTALL_OPENSSL_SHIM: ${INSTALL_OPENSSL_SHIM}")

if(SET_LIB_SONAME)
  set(CRYPTO_LIB_NAME "${CRYPTO_LIB_NAME}-${SOFTWARE_NAME}")
  set(SSL_LIB_NAME "${SSL_LIB_NAME}-${SOFTWARE_NAME}")
endif()

enable_language(C)

if (NOT WIN32 AND NOT APPLE)
  include(GNUInstallDirs)
else()
  if(NOT DEFINED CMAKE_INSTALL_LIBDIR)
    set(CMAKE_INSTALL_LIBDIR "lib")
  endif()
  if(NOT DEFINED CMAKE_INSTALL_INCLUDEDIR)
    set(CMAKE_INSTALL_INCLUDEDIR "include")
  endif()
  if(NOT DEFINED CMAKE_INSTALL_BINDIR)
    set(CMAKE_INSTALL_BINDIR "bin")
  endif()
endif()

# Set the install include directory based on whether a prefix subdirectory is desired
if(COHABITANT_HEADERS)
  set(AWSLC_INSTALL_INCLUDEDIR "${CMAKE_INSTALL_INCLUDEDIR}/aws-lc")
else()
  set(AWSLC_INSTALL_INCLUDEDIR "${CMAKE_INSTALL_INCLUDEDIR}")
endif()

# Set the install bin prefix based on whether cohabitation is desired
if(COHABITANT_BINARIES)
  set(AWSLC_BIN_PREFIX "aws-lc-")
else()
  set(AWSLC_BIN_PREFIX "")
endif()

function(target_add_awslc_include_paths)
  set(options EXCLUDE_PREFIX_HEADERS)
  set(oneValueArgs TARGET SCOPE)
  set(multiValueArgs)

  if(CMAKE_VERSION VERSION_LESS "3.7")
    cmake_parse_arguments(arg "${options}" "${oneValueArgs}" "${multiValueArgs}" ${ARGN})
  else()
    cmake_parse_arguments(PARSE_ARGV 0 arg "${options}" "${oneValueArgs}" "${multiValueArgs}")
  endif()

  if(arg_UNPARSED_ARGUMENTS)
    message(FATAL_ERROR "Unparsed arguments to target_add_awslc_include_paths: ${arg_UNPARSED_ARGUMENTS}")
  endif()

  if(arg_KEYWORDS_MISSING_VALUES)
    message(FATAL_ERROR "Missing value(s) for keyword(s) to target_add_awslc_include_paths: ${arg_KEYWORDS_MISSING_VALUES}")
  endif()

  if(EXCLUDE_PREFIX_HEADERS)
    set(INCLUDE_PREFIX_HEADERS 0)
  else()
    set(INCLUDE_PREFIX_HEADERS 1)
  endif()

  add_dependencies(${arg_TARGET} boringssl_prefix_symbols)
  target_include_directories(${arg_TARGET} BEFORE ${arg_SCOPE}
    $<$<BOOL:${INCLUDE_PREFIX_HEADERS}>:$<BUILD_INTERFACE:${AWSLC_BINARY_DIR}/symbol_prefix_include>>
    $<BUILD_INTERFACE:${AWSLC_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:${AWSLC_INSTALL_INCLUDEDIR}>)
endfunction()

# Validate Rust bindings prerequisites
if(GENERATE_RUST_BINDINGS)
  find_program(BINDGEN_EXECUTABLE NAMES bindgen)
  if(NOT BINDGEN_EXECUTABLE)
    message(FATAL_ERROR "GENERATE_RUST_BINDINGS is enabled but bindgen-cli was not found. "
                        "Install it with: cargo install --force --locked bindgen-cli")
  endif()

  # Verify minimum version (0.69.5)
  execute_process(
    COMMAND ${BINDGEN_EXECUTABLE} --version
    OUTPUT_VARIABLE BINDGEN_VERSION_OUTPUT
    OUTPUT_STRIP_TRAILING_WHITESPACE
    RESULT_VARIABLE BINDGEN_VERSION_RESULT
  )

  if(NOT BINDGEN_VERSION_RESULT EQUAL 0)
    message(FATAL_ERROR "Failed to get bindgen version")
  endif()

  # Extract version number (format: "bindgen X.Y.Z")
  string(REGEX MATCH "[0-9]+\\.[0-9]+\\.[0-9]+" BINDGEN_VERSION "${BINDGEN_VERSION_OUTPUT}")

  if(BINDGEN_VERSION VERSION_LESS "0.69.5")
    message(FATAL_ERROR "bindgen version ${BINDGEN_VERSION} is too old. "
                        "Minimum required version is 0.69.5. "
                        "Upgrade with: cargo install --force --locked bindgen-cli")
  endif()

  message(STATUS "Found bindgen-cli: ${BINDGEN_EXECUTABLE} (version ${BINDGEN_VERSION})")
  message(STATUS "Rust bindings target version: ${RUST_BINDINGS_TARGET_VERSION}")

  # Check for rustfmt (optional: used to format the generated bindings if available)
  find_program(RUSTFMT_EXECUTABLE NAMES rustfmt)
  if(RUSTFMT_EXECUTABLE)
    message(STATUS "Found rustfmt: ${RUSTFMT_EXECUTABLE}")
  else()
    message(STATUS "rustfmt not found; generated Rust bindings will not be formatted. "
                   "Install it with: rustup component add rustfmt")
  endif()
endif()

if(DISABLE_CPU_JITTER_ENTROPY)
  if(FIPS)
    message(FATAL_ERROR "Cannot opt-out of CPU Jitter for the FIPS build")
  endif()
  add_definitions(-DDISABLE_CPU_JITTER_ENTROPY)
  message(STATUS "Entropy source configuration: CPU Jitter opt-out")
  message(STATUS "Entropy source configured: Dynamic (default: Operating system)")
else()
  # The validated entropy source will always be configured to be CPU Jitter
  # Entropy by default; because it's the root of the Tree DRBG.
  message(STATUS "Entropy source configured: Dynamic (default: CPU Jitter)")
endif()

if(${CMAKE_SYSTEM_NAME} STREQUAL "OpenBSD")
  # OpenBSD by defaults links with --execute-only this is problematic for two reasons:
  # 1. The FIPS shared and static builds need to compute the module signature hash by reading the .text section
  # 2. s2n-bignum x86 assembly sources still have references to static data in the .text section
  if(NOT BUILD_SHARED_LIBS)
    set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -Wl,--no-execute-only")
  else()
    set(CMAKE_SHARED_LINKER_FLAGS "${CMAKE_SHARED_LINKER_FLAGS} -Wl,--no-execute-only")
  endif()
endif()

if(CMAKE_HOST_SYSTEM_NAME STREQUAL "SunOS" AND NOT CMAKE_CROSSCOMPILING)
  # Determine if the host is running an illumos distribution:
  execute_process(COMMAND /usr/bin/uname -o OUTPUT_VARIABLE UNAME_O
    OUTPUT_STRIP_TRAILING_WHITESPACE)

  if (UNAME_O STREQUAL "illumos")
    set(HOST_ILLUMOS 1)
  endif()

  if (UNAME_O STREQUAL "Solaris")
    set(HOST_SOLARIS 1)
  endif()

  if (HOST_ILLUMOS)
    #
    # illumos systems require linking libsocket and libnsl to get various
    # networking routines sometimes found in libc on other platforms:
    #
    if(NOT BUILD_SHARED_LIBS)
      set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -lsocket -lnsl")
    else()
      set(CMAKE_SHARED_LINKER_FLAGS "${CMAKE_SHARED_LINKER_FLAGS} -lsocket -lnsl")
    endif()
  endif()
endif()

# Tests and libssl both require the CXX language to be enabled. If a consumer
# chooses to disable building the tests and libssl, do not enable CXX
if(BUILD_TESTING OR BUILD_LIBSSL)
  enable_language(CXX)
endif()

if(ENABLE_CLANG_TIDY)
  find_program(CLANG_TIDY_CMD NAMES clang-tidy)
  set(CMAKE_VERBOSE_MAKEFILE ON)

  if(NOT CLANG_TIDY_CMD)
    message(WARNING "clang-tidy is not found!")
    set(CMAKE_CXX_CLANG_TIDY "" CACHE STRING "" FORCE)
  else()
    message(STATUS "clang-tidy found: ${CLANG_TIDY_CMD}")
    set(CLANG_TIDY_EXTRA_ARGS
            "-extra-arg=-Wno-unknown-warning-option"
    )
    set(CMAKE_C_CLANG_TIDY ${CLANG_TIDY_CMD} "--config-file=${AWSLC_SOURCE_DIR}/.clang-tidy")
    set(CMAKE_CXX_CLANG_TIDY ${CLANG_TIDY_CMD} "--config-file=${AWSLC_SOURCE_DIR}/.clang-tidy")
  endif()
endif()

if (NOT DEFINED CMAKE_CXX_STANDARD)
  set(CMAKE_CXX_STANDARD 11)
endif()
set(CMAKE_CXX_STANDARD_REQUIRED ON)
if (NOT DEFINED CMAKE_C_STANDARD)
  try_compile(
          RESULT
          ${AWSLC_BINARY_DIR}
          SOURCES "${CMAKE_CURRENT_LIST_DIR}/tests/compiler_features_tests/c11.c"
          COMPILE_DEFINITIONS -c -std=c11)
  if(RESULT)
    set(CMAKE_C_STANDARD 11)
  else()
    set(CMAKE_C_STANDARD 99)
  endif()
endif ()
set(CMAKE_C_STANDARD_REQUIRED ON)

if(CMAKE_C_COMPILER_ID MATCHES "Clang")
  set(CLANG 1)
elseif(CMAKE_C_COMPILER_ID MATCHES "GNU")
  set(GCC 1)
endif()

install(DIRECTORY include/openssl
        DESTINATION ${AWSLC_INSTALL_INCLUDEDIR}
        COMPONENT Development
        PATTERN boringssl_prefix_symbols.h EXCLUDE
        PATTERN boringssl_prefix_symbols_asm.h EXCLUDE
        PATTERN boringssl_prefix_symbols_nasm.inc EXCLUDE
        PATTERN "*.in" EXCLUDE
)

if (TEST_SYSGENID_PATH)
  message(STATUS "Setting AWSLC_VM_UBE_TESTING=1")
  add_definitions(-DAWSLC_VM_UBE_TESTING=1)
  message(STATUS "Setting AWSLC_SYSGENID_PATH=${TEST_SYSGENID_PATH}")
  add_definitions(-DAWSLC_SYSGENID_PATH=\"${TEST_SYSGENID_PATH}\")
endif()

if(NOT DISABLE_PERL)
  find_package(Perl REQUIRED)
endif()

if("${CMAKE_SYSTEM_NAME}" STREQUAL "Linux" AND BUILD_TESTING AND NOT CMAKE_CROSSCOMPILING)
  find_package(PkgConfig QUIET)
  if (PkgConfig_FOUND)
    pkg_check_modules(LIBUNWIND libunwind-generic)
    if(LIBUNWIND_FOUND)
      add_definitions(-DBORINGSSL_HAVE_LIBUNWIND)
    else()
      message("libunwind not found. Disabling unwind tests.")
    endif()
  else()
    message("pkgconfig not found. Disabling unwind tests.")
  endif()
endif()

if(NOT GO_EXECUTABLE)
  message(STATUS "Go not found. Disabling some code generation and using pre-generated code in generated-src/")
endif()
if (NOT PERL_EXECUTABLE)
  message(STATUS "Perl not found. Disabling some code generation and using pre-generated code in generated-src/")
endif()

set(GENERATE_CODE_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/generated-src")

if(USE_CUSTOM_LIBCXX)
  set(BORINGSSL_ALLOW_CXX_RUNTIME 1)
endif()

if(BORINGSSL_ALLOW_CXX_RUNTIME)
  add_definitions(-DBORINGSSL_ALLOW_CXX_RUNTIME)
endif()

string(TOLOWER "${CMAKE_BUILD_TYPE}" CMAKE_BUILD_TYPE_LOWER)
if(NOT FIPS)
  if("${CMAKE_BUILD_TYPE_LOWER}" STREQUAL "relwithassert" OR
     NOT CMAKE_BUILD_TYPE_LOWER MATCHES "rel")
    add_definitions(-DBORINGSSL_DISPATCH_TEST)
    # CMake automatically connects include_directories to the NASM
    # command-line, but not add_definitions.
    set(CMAKE_ASM_NASM_FLAGS "${CMAKE_ASM_NASM_FLAGS} -DBORINGSSL_DISPATCH_TEST")
  elseif(CMAKE_BUILD_TYPE_LOWER MATCHES "rel")
    add_definitions(-DBORINGSSL_RELEASE_BUILD)
  endif()
endif()

# Add a RelWithAsserts build configuration. It is the same as Release, except it
# does not define NDEBUG, so asserts run.
foreach(VAR CMAKE_C_FLAGS CMAKE_CXX_FLAGS CMAKE_ASM_FLAGS)
  string(REGEX REPLACE "(^| )[/-]DNDEBUG( |$)" " " "${VAR}_RELWITHASSERTS"
         "${${VAR}_RELEASE}")
endforeach()

if(BORINGSSL_PREFIX AND BORINGSSL_PREFIX_SYMBOLS AND GO_EXECUTABLE)

  message(STATUS "Prefix build configured: building headers using prefix \"${BORINGSSL_PREFIX}\" and symbols file \"${BORINGSSL_PREFIX_SYMBOLS}\"")

  if(IS_ABSOLUTE ${BORINGSSL_PREFIX_SYMBOLS})
    set(BORINGSSL_PREFIX_SYMBOLS_PATH ${BORINGSSL_PREFIX_SYMBOLS})
  else()
    set(BORINGSSL_PREFIX_SYMBOLS_PATH ${AWSLC_BINARY_DIR}/${BORINGSSL_PREFIX_SYMBOLS})
  endif()

  add_custom_command(
    OUTPUT symbol_prefix_include/openssl/boringssl_prefix_symbols.h
           symbol_prefix_include/openssl/boringssl_prefix_symbols_asm.h
           symbol_prefix_include/openssl/boringssl_prefix_symbols_nasm.inc
    COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_CURRENT_BINARY_DIR}/symbol_prefix_include/openssl
    COMMAND ${GO_EXECUTABLE} run ${CMAKE_CURRENT_SOURCE_DIR}/util/make_prefix_headers.go -out ${CMAKE_CURRENT_BINARY_DIR}/symbol_prefix_include/openssl -prefix ${BORINGSSL_PREFIX} ${BORINGSSL_PREFIX_SYMBOLS_PATH}
    COMMAND ${CMAKE_COMMAND} -E remove
          ${CMAKE_CURRENT_BINARY_DIR}/symbol_prefix_include/openssl/boringssl_prefix_symbols.h.bak
          ${CMAKE_CURRENT_BINARY_DIR}/symbol_prefix_include/openssl/boringssl_prefix_symbols_asm.h.bak
          ${CMAKE_CURRENT_BINARY_DIR}/symbol_prefix_include/openssl/boringssl_prefix_symbols_nasm.inc.bak
    DEPENDS util/make_prefix_headers.go
            ${BORINGSSL_PREFIX_SYMBOLS_PATH})
  # add_dependencies needs a target, not a file, so we add an intermediate
  # target.
  add_custom_target(
    boringssl_prefix_symbols
    DEPENDS symbol_prefix_include/openssl/boringssl_prefix_symbols.h
            symbol_prefix_include/openssl/boringssl_prefix_symbols_asm.h
            symbol_prefix_include/openssl/boringssl_prefix_symbols_nasm.inc)

  install(DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/symbol_prefix_include/openssl
          DESTINATION ${AWSLC_INSTALL_INCLUDEDIR}
          COMPONENT Development
  )
elseif(BORINGSSL_PREFIX AND BORINGSSL_PREFIX_HEADERS)

  message(STATUS "Prefix build configured: performing build using prefix \"${BORINGSSL_PREFIX}\" and headers path \"${BORINGSSL_PREFIX_HEADERS}\"")

  if(IS_ABSOLUTE ${BORINGSSL_PREFIX_HEADERS})
    set(BORINGSSL_PREFIX_HEADERS_PATH ${BORINGSSL_PREFIX_HEADERS})
  else()
    set(BORINGSSL_PREFIX_HEADERS_PATH ${AWSLC_BINARY_DIR}/${BORINGSSL_PREFIX_HEADERS})
  endif()

  file(COPY ${BORINGSSL_PREFIX_HEADERS_PATH}/openssl/boringssl_prefix_symbols.h DESTINATION ${CMAKE_CURRENT_BINARY_DIR}/symbol_prefix_include/openssl)
  file(COPY ${BORINGSSL_PREFIX_HEADERS_PATH}/openssl/boringssl_prefix_symbols_asm.h DESTINATION ${CMAKE_CURRENT_BINARY_DIR}/symbol_prefix_include/openssl)
  file(COPY ${BORINGSSL_PREFIX_HEADERS_PATH}/openssl/boringssl_prefix_symbols_nasm.inc DESTINATION ${CMAKE_CURRENT_BINARY_DIR}/symbol_prefix_include/openssl)

  add_custom_target(boringssl_prefix_symbols)

  install(DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/symbol_prefix_include/openssl
          DESTINATION ${AWSLC_INSTALL_INCLUDEDIR}
          COMPONENT Development
  )
elseif(BORINGSSL_PREFIX AND BORINGSSL_PREFIX_SYMBOLS AND NOT GO_EXECUTABLE)
  # This case is reached when both BORINGSSL_PREFIX and BORINGSSL_PREFIX_SYMBOLS are set,
  # but GO_EXECUTABLE is not available (checked after the first if-branch).
  message(FATAL_ERROR "Must have Go installed when using BORINGSSL_PREFIX and BORINGSSL_PREFIX_SYMBOLS")
elseif(BORINGSSL_PREFIX OR BORINGSSL_PREFIX_SYMBOLS)
  # Only one of BORINGSSL_PREFIX or BORINGSSL_PREFIX_SYMBOLS is set
  message(FATAL_ERROR "Must specify both or neither of BORINGSSL_PREFIX and BORINGSSL_PREFIX_SYMBOLS")
else()
  add_custom_target(boringssl_prefix_symbols)

  install(DIRECTORY include/openssl
          DESTINATION ${AWSLC_INSTALL_INCLUDEDIR}
          COMPONENT Development
          FILES_MATCHING
          PATTERN boringssl_prefix_symbols.h
          PATTERN boringssl_prefix_symbols_asm.h
          PATTERN boringssl_prefix_symbols_nasm.inc
  )
endif()

# Rust bindings generation
if(GENERATE_RUST_BINDINGS)
  include(cmake/rust_bindings.cmake)

  # Generate the rust_wrapper.h header from template
  generate_rust_wrapper_header()

  set(RUST_BINDINGS_OUTPUT "${CMAKE_CURRENT_BINARY_DIR}/rust/aws_lc_bindings.rs")

  # Build list of include directories for bindgen.
  # Prefix symbols header is intentionally excluded; see cmake/rust_bindings.cmake.
  set(RUST_BINDINGS_INCLUDE_DIRS
    "${CMAKE_CURRENT_BINARY_DIR}/include"   # Generated rust_wrapper.h location
    "${AWSLC_SOURCE_DIR}/include"           # Main AWS-LC headers (unprefixed)
  )

  # Conditionally set optional arguments for generate_rust_bindings
  if(BORINGSSL_PREFIX)
    set(_rust_prefix_arg PREFIX "${BORINGSSL_PREFIX}")
  else()
    set(_rust_prefix_arg "")
  endif()

  if(BUILD_LIBSSL)
    set(_rust_ssl_arg INCLUDE_SSL)
  else()
    set(_rust_ssl_arg "")
  endif()

  generate_rust_bindings(
    OUTPUT_FILE "${RUST_BINDINGS_OUTPUT}"
    INCLUDE_DIRS ${RUST_BINDINGS_INCLUDE_DIRS}
    ${_rust_prefix_arg}
    ${_rust_ssl_arg}
  )

  # Install the generated Rust bindings.
  # The rust_bindings target is not part of ALL, so we build it on-demand during install.
  install(CODE "execute_process(COMMAND \"${CMAKE_COMMAND}\" --build \"${CMAKE_BINARY_DIR}\" --target rust_bindings)")

  install(
    FILES "${CMAKE_CURRENT_BINARY_DIR}/rust/aws_lc_bindings.rs"
    DESTINATION "share/rust"
    COMPONENT Development
  )

  # Install the generated rust_wrapper.h header
  install(
    FILES "${CMAKE_CURRENT_BINARY_DIR}/include/rust_wrapper.h"
    DESTINATION "${CMAKE_INSTALL_INCLUDEDIR}"
    COMPONENT Development
  )
endif()


if("${CMAKE_SYSTEM_NAME}" STREQUAL "Emscripten")
  set(EMSCRIPTEN 1)
endif()

# Checks if test file compiles. If so, sets flag_to_set both as a CMake variable
# and as a preprocessor define. Additional arguments are interpreted as CFLAGS
# for the attempted compilation.
macro(check_compiler file_to_test flag_to_set)
  try_compile(
          ${flag_to_set} # Use CMake variable of the same name as the C flag to allow later reference
          ${AWSLC_BINARY_DIR}
          SOURCES "${AWSLC_SOURCE_DIR}/tests/compiler_features_tests/${file_to_test}"
          COMPILE_DEFINITIONS "-Werror ${ARGN}"
          OUTPUT_VARIABLE ERROR_MESSAGE)
  if(${flag_to_set})
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -D${flag_to_set}")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -D${flag_to_set}")
    message(STATUS "${file_to_test} probe is positive, enabling ${flag_to_set}")
  else()
    message(STATUS "${file_to_test} probe is negative, NOT enabling ${flag_to_set}:")
    # Some build applications use regexes on build output to highlight build
    # errors. Below, we modify a compiler error message to avoid that a negative
    # probe is considered an error in such build applications.
    if(FEATURE_PROBE_VERBOSE)
      string(REPLACE ": error:"
        ": compiler_error:" ERROR_MESSAGE_PROCESSED
        ${ERROR_MESSAGE})
      message(STATUS "    ${ERROR_MESSAGE_PROCESSED}")
    else()
      message(STATUS "Use -DFEATURE_PROBE_VERBOSE=ON to view feature probe error output.")
    endif()
  endif()
endmacro()

macro(check_run file_to_test flag_to_set compile_flags)
  message(STATUS "Run check_run file_to_test '${file_to_test}', "
          "flag_to_set '${flag_to_set}', "
          "and compile_flags '${compile_flags}'.")
  try_run(
          ${flag_to_set}
          COMPILE_RESULT
          "${CMAKE_CURRENT_BINARY_DIR}"
          "${CMAKE_CURRENT_LIST_DIR}/tests/compiler_features_tests/${file_to_test}"
          COMPILE_DEFINITIONS "${compile_flags}"
          OUTPUT_VARIABLE COMPILE_AND_RUN_OUTPUT)
  if (NOT COMPILE_RESULT)
    message(WARNING "COMPILE_AND_RUN_OUTPUT ${COMPILE_AND_RUN_OUTPUT}")
  endif()
endmacro()

# Some ancient assemblers don't know about AVX instructions, which is an
# assumption we make for some of the assembly implementations. This flag
# can be set to handle such cases.
option(MY_ASSEMBLER_IS_TOO_OLD_FOR_AVX "Exclude AVX code from the build" OFF)

# Some assemblers know about AVX but not ADX, AVX2 or AVX512 instructions, e.g. gcc 4.8.2.
# This flag can be set to handle such cases.
# Note:
# * Although this flag name implies an effect on AVX512 instructions, it's also
#   intended to avoid generating ADX and AVX2 instructions.
# * This flag name has "512AVX" instead of "AVX512" so that it doesn't
#   include the entire flag -DMY_ASSEMBLER_IS_TOO_OLD_FOR_AVX and match
#   to it in the Perl files checks.
option(MY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX "Exclude AVX512 code from the build" OFF)

if(MY_ASSEMBLER_IS_TOO_OLD_FOR_AVX)
  add_definitions(-DMY_ASSEMBLER_IS_TOO_OLD_FOR_AVX)
  add_definitions(-DMY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX)
  set(MY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX ON)
  message(STATUS "MY_ASSEMBLER_IS_TOO_OLD_FOR_AVX selected, removing all AVX optimisations")
endif()

if(MY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX)
  add_definitions(-DMY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX)
  message(STATUS "MY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX selected, removing ADX, AVX2 and AVX512 optimisations")
endif()

if (GCC)
  # All versions of GCC that AWS-LC supports has this warning
  set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wredundant-decls")

  # Detect if memcmp is wrongly stripped like strcmp.
  # If exists, let CMake generate a warning.
  # memcmp bug link https://gcc.gnu.org/bugzilla/show_bug.cgi?id=95189.
  # CMake try_run requires these variables must be preset.
  # https://cmake.org/cmake/help/latest/command/try_run.html
  set(MEMCMP_INVALID_STRIPPED "")
  set(MEMCMP_INVALID_STRIPPED__TRYRUN_OUTPUT "")

  # See https://gitlab.kitware.com/cmake/cmake/-/issues/16920 for why we set this to 2
  # During the `try_run` commands invoked by `check_run` a test program is run for the threading library.
  # This test is run expecting two threads, and this variable isn't set, for some systems
  # (particularly for systems using cross-compilation) causing our test to run without 2 threads and fail.
  # We set this variable here to allow these builds to succeed and keep these tests around to make sure
  # that we properly detect the compiler bugs that we are trying to test for with these `try_run` calls.

  set(THREADS_PTHREAD_ARG "2" CACHE STRING "Forcibly set by CMakeLists.txt." FORCE)

  if (CMAKE_BUILD_TYPE_LOWER MATCHES "release")
    # CMAKE_C_FLAGS_RELEASE enables `-O3`.
    check_run(memcmp_invalid_stripped_check.c MEMCMP_INVALID_STRIPPED "${CMAKE_C_FLAGS_RELEASE}")
  elseif(CMAKE_BUILD_TYPE_LOWER MATCHES "relwithdebinfo")
    # CMAKE_C_FLAGS_RELEASE enables `-O2`.
    check_run(memcmp_invalid_stripped_check.c MEMCMP_INVALID_STRIPPED "${CMAKE_C_FLAGS_RELWITHDEBINFO}")
  endif()

  if (MEMCMP_INVALID_STRIPPED)
    message(WARNING "Currently, GCC ${CMAKE_C_COMPILER_VERSION} is not supported due to a memcmp related bug reported in "
            "https://gcc.gnu.org/bugzilla/show_bug.cgi?id=95189.\n"
            "We strongly recommend against using the GCC ${CMAKE_C_COMPILER_VERSION} compiler.")
  endif ()
endif ()

if(GCC OR CLANG)
  # Note clang-cl is odd and sets both CLANG and MSVC. We base our configuration
  # primarily on our normal Clang one.
  if (NOT CMAKE_VERSION VERSION_GREATER "3.1.0")
    message(STATUS "Adding CMAKE_C_FLAGS -std=c${CMAKE_C_STANDARD}")
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -std=c${CMAKE_C_STANDARD}")
  else()
    message(STATUS "Setting CMAKE_C_STANDARD=${CMAKE_C_STANDARD}")
  endif()

  # TODO(CryptoAlg-759): enable '-Wpedantic' if awslc has to follow c99 spec.
  if(CLANG OR (GCC AND CMAKE_C_COMPILER_VERSION VERSION_GREATER "4.1.3"))
    # GCC 4.1.3 and below do not support all of these flags or they raise false positives.
    if (MSVC)
      # clang-cl sets different default warnings than clang. It also treats -Wall
      # as -Weverything, to match MSVC. Instead -W3 is the alias for -Wall.
      # See http://llvm.org/viewvc/llvm-project?view=revision&revision=319116
      set(C_CXX_FLAGS "${C_CXX_FLAGS} -W3 -fmsc-version=1900")
      set(C_CXX_FLAGS "${C_CXX_FLAGS} -Wno-deprecated-declarations")
    else()
      set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wextra")
      set(C_CXX_FLAGS "${C_CXX_FLAGS} -Wall -fvisibility=hidden -fno-common")
      if(CLANG AND MINGW)
        set(C_CXX_FLAGS "${C_CXX_FLAGS} -fms-extensions")
      endif()
      set(CMAKE_ASM_FLAGS "${CMAKE_ASM_FLAGS} -Wno-newline-eof")
    endif()
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wunused -Wcomment -Wchar-subscripts -Wuninitialized -Wshadow")
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wwrite-strings -Wformat-security -Wunused-result -Wno-overlength-strings")
    set(C_CXX_FLAGS "${C_CXX_FLAGS} -Wvla -Wtype-limits -Wno-unused-parameter")
    if(CLANG)
      set(C_CXX_FLAGS "${C_CXX_FLAGS} -Wno-c11-extensions")
    endif()
  endif()
  set(C_CXX_FLAGS "${C_CXX_FLAGS} -Werror -Wformat=2 -Wsign-compare -Wmissing-field-initializers -Wwrite-strings")

  # GCC 8.x added a warning called -Wcast-function-type to the -Wextra umbrella.
  # Also suppress for all clang versions supporting this warning.
  add_flag_if_supported(CMAKE_C_FLAGS "-Wno-cast-function-type")

  if(CMAKE_BUILD_TYPE_LOWER STREQUAL "debug" OR CMAKE_BUILD_TYPE_LOWER STREQUAL "relwithdebinfo")
    if (MSVC)
      set(C_CXX_FLAGS "${C_CXX_FLAGS} /Zi")
    else()
      if(EMSCRIPTEN)
        # emscripten's emcc/clang does not accept the "-ggdb" flag.
        set(C_CXX_FLAGS "${C_CXX_FLAGS} -g")
      else()
        set(C_CXX_FLAGS "${C_CXX_FLAGS} -ggdb")
      endif()
    endif()
  endif()

  if(MINGW AND NOT CLANG)
    # Some MinGW compilers set _WIN32_WINNT to an older version (Windows Server 2003)
    # See: https://learn.microsoft.com/en-us/cpp/porting/modifying-winver-and-win32-winnt?view=msvc-170
    # Support Windows 7 and later.
    add_definitions(-D_WIN32_WINNT=_WIN32_WINNT_WIN7)
  endif()

  if(CLANG)
    set(C_CXX_FLAGS "${C_CXX_FLAGS} -Wnewline-eof -fcolor-diagnostics")
  else()
    # GCC (at least 4.8.4) has a bug where it'll find unreachable free() calls
    # and declare that the code is trying to free a stack pointer. GCC 4.1.3 and lower
    # doesn't support this flag and can't use it.
    add_flag_if_supported(C_CXX_FLAGS "-Wno-free-nonheap-object")
    # GCC (from at least 4.8) does not include -Wmissing-braces in -Wall due to Bug 25137.
    # This warning is turned on everywhere internally however, so we have to define it here
    # to check that our changes don't break the build.
    # See https://gcc.gnu.org/bugzilla/show_bug.cgi?id=25137
    add_flag_if_supported(C_CXX_FLAGS "-Wmissing-braces")
  endif()

  # -Wstring-concatenation was added in Clang 12.0.0, which corresponds to
  # AppleClang 13.0.0 per the table in
  # https://en.wikipedia.org/wiki/Xcode#Toolchain_versions
  add_flag_if_supported(C_CXX_FLAGS "-Wstring-concatenation")

  add_flag_if_supported(C_CXX_FLAGS "-Wimplicit-fallthrough")

  if(CMAKE_COMPILER_IS_GNUCXX AND CMAKE_CXX_COMPILER_VERSION VERSION_GREATER "5")
    set(C_CXX_FLAGS "${C_CXX_FLAGS} -Wformat-signedness")
  endif()

  set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} ${C_CXX_FLAGS} -Wmissing-prototypes -Wold-style-definition -Wstrict-prototypes")
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${C_CXX_FLAGS} -Wmissing-declarations")

  if(NOT MSVC)
    if (NOT CMAKE_CXX_STANDARD)
      set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")
    endif ()
    if(APPLE)
      set(CMAKE_MACOSX_RPATH 1)
    endif()
    if(NOT BORINGSSL_ALLOW_CXX_RUNTIME)
      set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fno-exceptions -fno-rtti")
    endif()
  endif()

  # In GCC, -Wmissing-declarations is the C++ spelling of -Wmissing-prototypes
  # and using the wrong one is an error. In Clang, -Wmissing-prototypes is the
  # spelling for both and -Wmissing-declarations is some other warning.
  #
  # https://gcc.gnu.org/onlinedocs/gcc-7.1.0/gcc/Warning-Options.html#Warning-Options
  # https://clang.llvm.org/docs/DiagnosticsReference.html#wmissing-prototypes
  # https://clang.llvm.org/docs/DiagnosticsReference.html#wmissing-declarations
  if(CLANG)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wmissing-prototypes")
  endif()

  if(GCC AND "4.8" VERSION_GREATER CMAKE_C_COMPILER_VERSION AND CMAKE_C_COMPILER_VERSION VERSION_GREATER "4.1.3")
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wno-array-bounds")
  endif()

  check_compiler("linux_random_h.c" HAVE_LINUX_RANDOM_H)
  if(NOT HAVE_LINUX_RANDOM_H)
    check_compiler("linux_random_h.c" AWS_LC_URANDOM_NEEDS_U32 "-DDEFINE_U32")
    if(AWS_LC_URANDOM_NEEDS_U32)
      add_definitions("-DHAVE_LINUX_RANDOM_H")
    endif()
  endif()

  check_compiler("stdalign_check.c" AWS_LC_STDALIGN_AVAILABLE)
  check_compiler("builtin_swap_check.c" AWS_LC_BUILTIN_SWAP_SUPPORTED)

elseif(MSVC)
  set(MSVC_DISABLED_WARNINGS_LIST
      "C4061" # enumerator 'identifier' in switch of enum 'enumeration' is not
              # explicitly handled by a case label
              # Disable this because it flags even when there is a default.
      "C4100" # 'exarg' : unreferenced formal parameter
      "C4127" # conditional expression is constant
      "C4146" # unary minus operator applied to unsigned type, result still unsigned
      "C4200" # nonstandard extension used : zero-sized array in
              # struct/union.
      "C4204" # nonstandard extension used: non-constant aggregate initializer
      "C4221" # nonstandard extension used : 'identifier' : cannot be
              # initialized using address of automatic variable
      "C4242" # 'function' : conversion from 'int' to 'uint8_t',
              # possible loss of data
      "C4244" # 'function' : conversion from 'int' to 'uint8_t',
              # possible loss of data
      "C4267" # conversion from 'size_t' to 'int', possible loss of data
      "C4291" # 'void *operator new(std::size_t,void *) throw()': no matching
              # operator delete found; memory will not be freed if initialization
              # throws an exception
      "C4296" # '>=' : expression is always true
      "C4371" # layout of class may have changed from a previous version of the
              # compiler due to better packing of member '...'
      "C4388" # signed/unsigned mismatch
      "C4350" # behavior change: 'std::_Wrap_alloc...'
      "C4365" # '=' : conversion from 'size_t' to 'int',
              # signed/unsigned mismatch
      "C4389" # '!=' : signed/unsigned mismatch
      "C4464" # relative include path contains '..'
      "C4510" # 'argument' : default constructor could not be generated
      "C4512" # 'argument' : assignment operator could not be generated
      "C4514" # 'function': unreferenced inline function has been removed
      "C4548" # expression before comma has no effect; expected expression with
              # side-effect" caused by FD_* macros.
      "C4571" # Informational: catch(...) semantics changed since Visual C++ 7.1;
              # structured exceptions (SEH) are no longer caught
      "C4610" # struct 'argument' can never be instantiated - user defined
              # constructor required.
      "C4623" # default constructor was implicitly defined as deleted
      "C4625" # copy constructor could not be generated because a base class
              # copy constructor is inaccessible or deleted
      "C4626" # assignment operator could not be generated because a base class
              # assignment operator is inaccessible or deleted
      "C4628" # digraphs not supported with -Ze
      "C4668" # 'symbol' is not defined as a preprocessor macro, replacing with
              # '0' for 'directives'
              # Disable this because GTest uses it everywhere.
      "C4706" # assignment within conditional expression
      "C4710" # 'function': function not inlined
      "C4711" # function 'function' selected for inline expansion
      "C4800" # 'int' : forcing value to bool 'true' or 'false'
              # (performance warning)
      "C4820" # 'bytes' bytes padding added after construct 'member_name'
      "C5026" # move constructor was implicitly defined as deleted
      "C5027" # move assignment operator was implicitly defined as deleted
      "C5039" # pointer or reference to potentially throwing function passed to 'extern "C"' function under -EHc
      "C5045" # Compiler will insert Spectre mitigation for memory load if
              # /Qspectre switch specified
      "C4255" # no function prototype given: converting '()' to '(void)'
      "C4152" # non standard extension, function/data ptr conversion in expression
              # used in bcm.c to check functions are inside the FIPS module memory region
      "C4295" # array is too small to include a terminating null character
      "C4701" # potentially uninitialized local
      "C4505" # unreferenced local function has been removed
      "C4702" # unreachable code in bcm.c power on tests
      "C4191" # unsafe conversion from 'type of expression' to 'type required'
              # TODO remove once https://github.com/openssl/openssl/issues/18957 is resolved
      "C4996" # deprecated warnings using low level functions in OpenSSL 3.0
      "C4746" # consider using __iso_volatile_load/store intrinsic functions
      "C5264" # 'variable-name': 'const' variable is not used
      "C5266" # 'const' qualifier on return type has no effect
      "C5267" # definition of implicit copy constructor for '...' is deprecated because it has a user-provided destructor
          )
  set(MSVC_LEVEL4_WARNINGS_LIST
      # See https://connect.microsoft.com/VisualStudio/feedback/details/1217660/warning-c4265-when-using-functional-header
      "C4265" # class has virtual functions, but destructor is not virtual
      )
  string(REPLACE "C" " -wd" MSVC_DISABLED_WARNINGS_STR
                            ${MSVC_DISABLED_WARNINGS_LIST})
  string(REPLACE "C" " -w4" MSVC_LEVEL4_WARNINGS_STR
                            ${MSVC_LEVEL4_WARNINGS_LIST})
  # Newer MSVC (VS2026/14.51) emits C4865 from Windows SDK headers; with -WX
  # that fails the build over warnings we don't own. Mark the toolchain/SDK
  # include dirs external so -WX only covers our code. /external: is stable
  # since MSVC 19.29; exclude clang-cl (it never emits C4865 and can error on
  # an unrecognized /external:env: arg under -WX).
  if(MSVC_VERSION GREATER_EQUAL 1929 AND CMAKE_C_COMPILER_ID STREQUAL "MSVC")
    set(MSVC_EXTERNAL_WARNINGS_STR
        "/external:W0 /external:env:INCLUDE /external:env:EXTERNAL_INCLUDE")
  else()
    set(MSVC_EXTERNAL_WARNINGS_STR "")
  endif()
  set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -utf-8 -Wall -WX ${MSVC_DISABLED_WARNINGS_STR} ${MSVC_LEVEL4_WARNINGS_STR} ${MSVC_EXTERNAL_WARNINGS_STR}")
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -utf-8 -Wall -WX ${MSVC_DISABLED_WARNINGS_STR} ${MSVC_LEVEL4_WARNINGS_STR} ${MSVC_EXTERNAL_WARNINGS_STR}")
endif()

# If we're using MSVC on Windows in FIPS mode with RelWithDebInfo then we want to override some of the default RelWithDebInfo flags.
# This fixes the problem we run into with RelWithDebInfo and FIPS mode on Windows where the FIPS module wouldn't span the expected symbol.
if(MSVC AND CMAKE_BUILD_TYPE_LOWER MATCHES "relwithdebinfo" AND FIPS)
  # /Zi requires the /debug flag for executables/libraries that we want .pdb files for.
  # We want to replace the default /debug flag with /DEBUG:FULL, to explicitly make sure that the .pdb files can be used on machines other than one on which it's built.
  string(REPLACE "/debug" "/DEBUG:FULL" CMAKE_SHARED_LINKER_FLAGS_RELWITHDEBINFO "${CMAKE_SHARED_LINKER_FLAGS_RELWITHDEBINFO}")
  string(REPLACE "/debug" "/DEBUG:FULL" CMAKE_EXE_LINKER_FLAGS_RELWITHDEBINFO "${CMAKE_EXE_LINKER_FLAGS_RELWITHDEBINFO}")

  # The /debug flag also turns off the /OPT linker flag so we want to turn them back on across the board.
  set(CMAKE_SHARED_LINKER_FLAGS_RELWITHDEBINFO "${CMAKE_SHARED_LINKER_FLAGS_RELWITHDEBINFO} /OPT:REF,ICF,LBR")
  set(CMAKE_EXE_LINKER_FLAGS_RELWITHDEBINFO "${CMAKE_EXE_LINKER_FLAGS_RELWITHDEBINFO} /OPT:REF,ICF,LBR")
endif()

if(WIN32)
  add_definitions(-D_HAS_EXCEPTIONS=0)
  add_definitions(-DWIN32_LEAN_AND_MEAN)
  add_definitions(-DNOMINMAX)
  # Allow use of fopen.
  add_definitions(-D_CRT_SECURE_NO_WARNINGS)
  # VS 2017 and higher supports STL-only warning suppressions.
  # _STL_EXTRA_DISABLED_WARNINGS is only consumed by MSVC STL headers,
  # so scope the definition to C++. Using add_compile_options with a
  # CMake-escaped space keeps the value as a single argument under
  # Ninja + clang-cl, where add_definitions with a space-containing
  # value is not quoted robustly (see aws/aws-lc-rs#981).
  add_compile_options($<$<COMPILE_LANGUAGE:CXX>:-D_STL_EXTRA_DISABLED_WARNINGS=4774\ 4987>)
endif()

add_flag_if_supported(C_CXX_FLAGS "-Wshadow")

# pthread_rwlock_t on Linux requires a feature flag. We limit this to Linux
# because, on Apple platforms, it instead disables APIs we use. See compat(5)
# and sys/cdefs.h. Reportedly, FreeBSD also breaks when this is set. See
# https://crbug.com/boringssl/471.
if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
  set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -D_XOPEN_SOURCE=700")
endif()

if(FUZZ)
  if(NOT CLANG)
    message(FATAL_ERROR "You need to build with Clang for fuzzing to work")
  endif()

  if(CMAKE_C_COMPILER_VERSION VERSION_LESS "6.0.0")
    message(FATAL_ERROR "You need Clang ≥ 6.0.0")
  endif()

  add_definitions(-DBORINGSSL_UNSAFE_DETERMINISTIC_MODE)
  set(RUNNER_ARGS "-deterministic")

  if(NOT NO_FUZZER_MODE)
    add_definitions(-DBORINGSSL_UNSAFE_FUZZER_MODE)
    set(RUNNER_ARGS ${RUNNER_ARGS} "-fuzzer" "-shim-config" "fuzzer_mode.json")
  endif()

  set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -fsanitize=address,fuzzer-no-link -fsanitize-coverage=edge,indirect-calls")
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fsanitize=address,fuzzer-no-link -fsanitize-coverage=edge,indirect-calls")
endif()

if(SSL_RUNNER_SHIM_CONFIG)
  set(RUNNER_ARGS ${RUNNER_ARGS} "-shim-config" "${AWSLC_SOURCE_DIR}/${SSL_RUNNER_SHIM_CONFIG}")
endif()

if(BUILD_SHARED_LIBS)
  add_definitions(-DBORINGSSL_SHARED_LIBRARY)
  # Enable position-independent code globally. This is needed because
  # some library targets are OBJECT libraries.
  set(CMAKE_POSITION_INDEPENDENT_CODE TRUE)
elseif(NOT DEFINED CMAKE_POSITION_INDEPENDENT_CODE)
  set(CMAKE_POSITION_INDEPENDENT_CODE TRUE)
endif()

if(MSAN)
  if(NOT CLANG)
    message(FATAL_ERROR "Cannot enable MSAN unless using Clang")
  endif()

  if(ASAN)
    message(FATAL_ERROR "ASAN and MSAN are mutually exclusive")
  endif()

  set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -fsanitize=memory -fsanitize-memory-track-origins -fno-omit-frame-pointer")
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fsanitize=memory -fsanitize-memory-track-origins -fno-omit-frame-pointer")
  set(CMAKE_ASM_FLAGS "${CMAKE_ASM_FLAGS} -fsanitize=memory -fsanitize-memory-track-origins -fno-omit-frame-pointer")
endif()

if(ASAN)
  if(NOT CLANG)
    message(FATAL_ERROR "Cannot enable ASAN unless using Clang")
  endif()

  set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -fsanitize=address -fsanitize-address-use-after-scope -fno-omit-frame-pointer")
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fsanitize=address -fsanitize-address-use-after-scope -fno-omit-frame-pointer")
endif()

if(CFI)
  if(NOT CLANG)
    message(FATAL_ERROR "Cannot enable CFI unless using Clang")
  endif()

  set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -fsanitize=cfi -fno-sanitize-trap=cfi -flto=thin")
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fsanitize=cfi -fno-sanitize-trap=cfi -flto=thin")
  # We use Chromium's copy of clang, which requires -fuse-ld=lld if building
  # with -flto. That, in turn, can't handle -ggdb.
  set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -fuse-ld=lld")
  string(REPLACE "-ggdb" "-g" CMAKE_C_FLAGS "${CMAKE_C_FLAGS}")
  string(REPLACE "-ggdb" "-g" CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS}")
  # -flto causes object files to contain LLVM bitcode. Mixing those with
  # assembly output in the same static library breaks the linker.
  set(OPENSSL_NO_ASM "1")
endif()

if(TSAN)
  if(NOT CLANG)
    message(FATAL_ERROR "Cannot enable TSAN unless using Clang")
  endif()

  set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -fsanitize=thread")
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fsanitize=thread")
  set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -fsanitize=thread")
endif()

if(UBSAN)
  if(NOT CLANG)
    message(FATAL_ERROR "Cannot enable UBSAN unless using Clang")
  endif()

  set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -fsanitize=undefined")
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fsanitize=undefined")
  set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -fsanitize=undefined")

  if(NOT UBSAN_RECOVER)
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -fno-sanitize-recover=undefined")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fno-sanitize-recover=undefined")
    set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -fno-sanitize-recover=undefined")
  endif()
endif()

if(GCOV)
  set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} --coverage -O0")
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} --coverage -O0")
  set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} --coverage")
endif()

if(KEEP_ASM_LOCAL_SYMBOLS)
  set(CMAKE_ASM_FLAGS "${CMAKE_ASM_FLAGS} -Wa,-L")
endif()

if(FIPS)
  if(NOT GO_EXECUTABLE OR NOT PERL_EXECUTABLE)
    message(FATAL_ERROR "Building AWS-LC for FIPS requires Go and Perl")
  endif()

  if(NOT BUILD_SHARED_LIBS AND (WIN32 OR APPLE))
    message(FATAL_ERROR "Static FIPS build of AWS-LC is supported only on Linux")
  endif()

  if(CMAKE_C_FLAGS MATCHES "(^| )-DAWSLC_FIPS_FAILURE_CALLBACK($| )" AND BUILD_SHARED_LIBS)
    message(FATAL_ERROR "AWSLC_FIPS_FAILURE_CALLBACK only supported with the static library build of AWS-LC")
  endif()

  # The FIPS integrity check does not work for ASan and MSan builds.
  if(NOT ASAN AND NOT MSAN)
    if(BUILD_SHARED_LIBS)
      set(FIPS_SHARED "1")
    else()
      set(FIPS_DELOCATE "1")
    endif()
  endif()

  add_definitions(-DBORINGSSL_FIPS)
endif()

# The Android CMake files set -ffunction-sections and -fdata-sections, which
# is incompatible with FIPS_SHARED.
if(FIPS_SHARED AND ANDROID)
  set(CMAKE_C_FLAGS
      "${CMAKE_C_FLAGS} -fno-function-sections -fno-data-sections")
  set(CMAKE_CXX_FLAGS
      "${CMAKE_CXX_FLAGS} -fno-function-sections -fno-data-sections")
endif()

if(OPENSSL_SMALL)
  add_definitions(-DOPENSSL_SMALL)
endif()

if(CONSTANT_TIME_VALIDATION)
  add_definitions(-DBORINGSSL_CONSTANT_TIME_VALIDATION)
endif()

# CMake's iOS support uses Apple's multiple-architecture toolchain. It takes an
# architecture list from CMAKE_OSX_ARCHITECTURES, leaves CMAKE_SYSTEM_PROCESSOR
# alone, and expects all architecture-specific logic to be conditioned within
# the source files rather than the build. This does not work for our assembly
# files, so we fix CMAKE_SYSTEM_PROCESSOR and only support single-architecture
# builds.
if(NOT OPENSSL_NO_ASM AND CMAKE_OSX_ARCHITECTURES)
  list(LENGTH CMAKE_OSX_ARCHITECTURES NUM_ARCHES)
  if(NOT NUM_ARCHES EQUAL 1)
    message(FATAL_ERROR "Universal binaries not supported.")
  endif()
  list(GET CMAKE_OSX_ARCHITECTURES 0 CMAKE_SYSTEM_PROCESSOR)
endif()

if(MALLOC_FAILURE_TESTING)
  add_definitions(-DBORINGSSL_MALLOC_FAILURE_TESTING)
endif()

TEST_BIG_ENDIAN(BIG_ENDIAN)

if(OPENSSL_NO_SSE2_FOR_TESTING)
  add_definitions(-DOPENSSL_NO_SSE2_FOR_TESTING)
endif()

if(HOST_ILLUMOS OR HOST_SOLARIS)
  #
  # CMAKE_SYSTEM_PROCESSOR unfortunately comes from the output of "uname -p",
  # which on illumos systems emits "i386".  Instead, use the value from
  # "isainfo -n", which prints "the name of the native instruction set used by
  # portable applications"; e.g., "amd64".
  #
  execute_process(COMMAND /usr/bin/isainfo -n OUTPUT_VARIABLE
    CMAKE_SYSTEM_PROCESSOR_LOWER OUTPUT_STRIP_TRAILING_WHITESPACE)
else()
  # Some consumers might use upper-case (e.g.) "X86" or "X86_64".
  # Matching below is based on lower-case.
  string(TOLOWER "${CMAKE_SYSTEM_PROCESSOR}" CMAKE_SYSTEM_PROCESSOR_LOWER)
endif()

if(OPENSSL_NO_ASM)
  add_definitions(-DOPENSSL_NO_ASM)
  set(ARCH "generic")
elseif(CMAKE_SYSTEM_PROCESSOR_LOWER MATCHES "x86_64|amd64")
  # If ARCH is originally detected as 64-bit, perform an additional check
  # to determine whether to build as 32-bit or 64-bit. This happens in some
  # cases such as when building in Docker, where the host-level architecture is 64-bit
  # but the Docker image should result in building for a 32-bit architecture.
  if(CMAKE_SIZEOF_VOID_P EQUAL 8)
    set(ARCH "x86_64")
  else()
    set(ARCH "x86")
  endif()
elseif(CMAKE_SYSTEM_PROCESSOR_LOWER MATCHES "x86|i386|i686")
  set(ARCH "x86")
elseif(CMAKE_SYSTEM_PROCESSOR_LOWER MATCHES "arm64.*|aarch64")
  # If ARCH is originally detected as 64-bit, perform an additional check
  # to determine whether to build as 32-bit or 64-bit. This happens in some
  # cases such as when building on an 64-bit CPU where the OS is 32-bit.
  if(CMAKE_SIZEOF_VOID_P EQUAL 8)
    set(ARCH "aarch64")
  else()
    set(ARCH "arm")
  endif()
elseif(CMAKE_SYSTEM_PROCESSOR_LOWER MATCHES "^arm*")
  set(ARCH "arm")
elseif(CMAKE_SYSTEM_PROCESSOR_LOWER MATCHES "powerpc64le|ppc64le")
  set(ARCH "ppc64le")
elseif(CMAKE_SYSTEM_PROCESSOR_LOWER STREQUAL "riscv64")
  set(ARCH "riscv64")
elseif(CMAKE_SYSTEM_PROCESSOR_LOWER STREQUAL "s390x")
  set(ARCH "s390x")
elseif(CMAKE_SYSTEM_PROCESSOR_LOWER STREQUAL "loongarch64")
  set(ARCH "loongarch64")
else()
  message(STATUS "Unknown processor found. Using generic implementations. Processor: " ${CMAKE_SYSTEM_PROCESSOR})
  set(ARCH "generic")
endif()

if(ARCH STREQUAL "aarch64" AND CMAKE_GENERATOR MATCHES "Visual Studio" AND NOT "${CMAKE_VS_PLATFORM_TOOLSET}" MATCHES "ClangCL")
  message(FATAL_ERROR "AWS-LC Windows/ARM64 assembly code requires ClangCL. Current toolset: ${CMAKE_VS_PLATFORM_TOOLSET}")
endif()

if(WIN32 AND FIPS AND (CMAKE_BUILD_TYPE_LOWER STREQUAL "debug" OR (CMAKE_BUILD_TYPE_LOWER STREQUAL "relwithdebinfo" AND ARCH STREQUAL "aarch64")))
  message(FATAL_ERROR "Windows Debug and RelWithDebInfo builds are not supported with FIPS, use Release")
endif()

# If target ARCH is 32-bit x86, ensure SSE2 is enabled since it's used by the optimized assembly.
# To build for targets that do not support SSE2, use the `OPENSSL_NO_ASM` flag.
if(ARCH STREQUAL "x86" AND NOT OPENSSL_NO_SSE2_FOR_TESTING)
  # Most compilers enable SSE2 in 32-bit x86 by default, but in some cases GCC and Clang don't.
  # See: https://github.com/aws/aws-lc/commit/6fe8dcbe96e580ea85233fdb98a142e42951b70b
  if(GCC OR CLANG)
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -msse2")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -msse2")
    set(CMAKE_ASM_FLAGS "${CMAKE_ASM_FLAGS} -msse2")
  endif()
endif()

if(ENABLE_DATA_INDEPENDENT_TIMING)
  add_definitions(-DENABLE_AUTO_SET_RESET_DIT)
endif()

if(USE_CUSTOM_LIBCXX)
  if(NOT CLANG)
    message(FATAL_ERROR "USE_CUSTOM_LIBCXX only supported with Clang")
  endif()

  # The docker images set an environment variable to the llvm project directory which the sandbox builds will use,
  # you can also pass in the llvm project path as a CMake parameter which takes precedence over the environment variable
  if(DEFINED ENV{LLVM_PROJECT_HOME} AND NOT LLVM_PROJECT_HOME)
    set(LLVM_PROJECT_HOME $ENV{LLVM_PROJECT_HOME})
  endif()

  if(NOT LLVM_PROJECT_HOME)
    message(FATAL "Could not find path to LLVM project, must set LLVM_PROJECT_HOME environment variable or pass in -DLLVM_PROJECT_HOME")
  endif()

  # CMAKE_CXX_FLAGS ends up in the linker flags as well, so use
  # add_compile_options. There does not appear to be a way to set
  # language-specific compile-only flags.
  add_compile_options("-nostdinc++")
  set(CMAKE_CXX_LINK_FLAGS "${CMAKE_CXX_LINK_FLAGS} -nostdlib++")
  include_directories(
    SYSTEM
    util/bot/libcxx-config
    ${LLVM_PROJECT_HOME}/libcxx/include
    ${LLVM_PROJECT_HOME}/libcxxabi/include
  )

  # This is patterned after buildtools/third_party/libc++/BUILD.gn and
  # buildtools/third_party/libc++abi/BUILD.gn in Chromium.

  file(GLOB LIBCXX_SOURCES "${LLVM_PROJECT_HOME}/libcxx/src/*.cpp" "${LLVM_PROJECT_HOME}/libcxx/src/ryu/*.cpp")
  file(GLOB LIBCXXABI_SOURCES "${LLVM_PROJECT_HOME}/libcxxabi/src/*.cpp")

  # This file is meant for exception-less builds.
  list(REMOVE_ITEM LIBCXXABI_SOURCES "${LLVM_PROJECT_HOME}/libcxxabi/src/cxa_noexception.cpp")

  # libc++ also defines new and delete.
  list(REMOVE_ITEM LIBCXXABI_SOURCES "${LLVM_PROJECT_HOME}/libcxxabi/src/stdlib_new_delete.cpp")
  if(TSAN)
    # ThreadSanitizer tries to intercept these symbols. Skip them to avoid
    # symbol conflicts.
    list(REMOVE_ITEM LIBCXXABI_SOURCES "${LLVM_PROJECT_HOME}/libcxxabi/src/cxa_guard.cpp")
  endif()

  add_library(libcxxabi ${LIBCXXABI_SOURCES})
  target_compile_definitions(
    libcxxabi PRIVATE
    -D_LIBCPP_ENABLE_CXX17_REMOVED_UNEXPECTED_FUNCTIONS
  )

  add_library(libcxx ${LIBCXX_SOURCES})
  if(ASAN OR MSAN OR TSAN)
    # Sanitizers try to intercept new and delete.
    target_compile_definitions(
      libcxx PRIVATE
      -D_LIBCPP_DISABLE_NEW_DELETE_DEFINITIONS
    )
  endif()
  target_compile_definitions(
    libcxx PRIVATE
    -D_LIBCPP_BUILDING_LIBRARY
    -DLIBCXX_BUILDING_LIBCXXABI
  )
  set_target_properties(
    libcxx libcxxabi PROPERTIES
    COMPILE_FLAGS "-Wno-missing-prototypes -Wno-implicit-fallthrough"
    # libc++ and libc++abi must be built in C++23 mode.
    # LLVM 19+ added expected.cpp which requires C++23.
    CXX_STANDARD 23
    CXX_STANDARD_REQUIRED TRUE
  )
  # libc++abi depends on libc++ internal headers.
  set_property(TARGET libcxx libcxxabi APPEND PROPERTY INCLUDE_DIRECTORIES "${LLVM_PROJECT_HOME}/libcxx/src")
  target_link_libraries(libcxx libcxxabi)
endif()

if(BUILD_TESTING)
  # Add minimal googletest targets. The provided one has many side-effects, and
  # googletest has a very straightforward build.
  add_library(boringssl_gtest third_party/googletest/src/gtest-all.cc)
  if(USE_CUSTOM_LIBCXX)
    target_link_libraries(boringssl_gtest libcxx)
  endif()
  if(BUILD_SHARED_LIBS)
    # This is needed for the Windows build to correctly annotate GTest's API with __declspec(dllexport)
    target_compile_options(boringssl_gtest PRIVATE -DGTEST_CREATE_SHARED_LIBRARY=1)
  endif()
  target_include_directories(
    boringssl_gtest
    PUBLIC third_party/googletest/include
    PRIVATE third_party/googletest
  )

  # WASI does not support fork() or process exit code detection required by
  # GTest death tests, nor dup()/dup2()/mkstemp() required for stream
  # redirection. Define these before gtest headers are included to prevent
  # gtest-port.h from auto-detecting based on the host OS.
  if(CMAKE_SYSTEM_NAME STREQUAL "Generic" AND CMAKE_SYSTEM_PROCESSOR STREQUAL "wasm32")
    add_compile_definitions(GTEST_HAS_DEATH_TEST=0)
    add_compile_definitions(GTEST_HAS_STREAM_REDIRECTION=0)
  endif()

  # Declare a dummy target to build all unit tests. Test targets should inject
  # themselves as dependencies next to the target definition.
  add_custom_target(all_tests)

  # On Windows, CRYPTO_TEST_DATA is too long to fit in command-line limits.
  # TODO(davidben): CMake 3.12 has a list(JOIN) command. Use that when we've
  # updated the minimum version.
  set(EMBED_TEST_DATA_ARGS "")
  foreach(arg ${CRYPTO_TEST_DATA})
    set(EMBED_TEST_DATA_ARGS "${EMBED_TEST_DATA_ARGS}${arg}\n")
  endforeach()
  file(WRITE "${CMAKE_CURRENT_BINARY_DIR}/embed_test_data_args.txt"
      "${EMBED_TEST_DATA_ARGS}")

  if(GO_EXECUTABLE)
    add_custom_command(
        OUTPUT crypto_test_data.cc
        COMMAND ${GO_EXECUTABLE} run util/embed_test_data.go -file-list
        "${CMAKE_CURRENT_BINARY_DIR}/embed_test_data_args.txt" >
        "${CMAKE_CURRENT_BINARY_DIR}/crypto_test_data.cc"
        DEPENDS util/embed_test_data.go ${CRYPTO_TEST_DATA}
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
  else()
    add_custom_command(
        OUTPUT crypto_test_data.cc
        COMMAND ${CMAKE_COMMAND} -E tar "jxvf" ${GENERATE_CODE_ROOT}/crypto_test_data.cc.tar.bz2
        DEPENDS  ${GENERATE_CODE_ROOT}/crypto_test_data.cc.tar.bz2
        WORKING_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}")
  endif()
  add_library(crypto_test_data OBJECT crypto_test_data.cc)

  # sets tests to compile an so executable file for testing in our Android app CI.
  if(ANDROIDTESTRUNNER)
    set(TEST_EXECUTABLE_EXT ".so")
  endif()

  set(CRYPTO_TEST_EXEC crypto_test${TEST_EXECUTABLE_EXT})
  set(RANDOM_TEST_EXEC urandom_test${TEST_EXECUTABLE_EXT})
  set(SSL_TEST_EXEC ssl_test${TEST_EXECUTABLE_EXT})
  set(MEM_TEST_EXEC mem_test${TEST_EXECUTABLE_EXT})
  set(MEM_SET_TEST_EXEC mem_set_test${TEST_EXECUTABLE_EXT})
  set(INTEGRATION_TEST_EXEC integration_test${TEST_EXECUTABLE_EXT})
  set(DYNAMIC_LOADING_TEST_EXEC dynamic_loading_test${TEST_EXECUTABLE_EXT})
  set(RWLOCK_STATIC_INIT_TEST_EXEC rwlock_static_init${TEST_EXECUTABLE_EXT})
  set(RANDOM_ISOLATED_EXEC rand_isolated_test${TEST_EXECUTABLE_EXT})
  set(TREE_DRBG_JITTER_ENTROPY_ISOLATED_EXEC tree_drbg_jitter_entropy_isolated_test${TEST_EXECUTABLE_EXT})

  add_subdirectory(util/fipstools/acvp/modulewrapper)

  macro(set_test_location executable_name)
    if(CMAKE_GENERATOR MATCHES "Visual Studio")
      message(NOTICE "Location  for ${executable_name} is: ${CMAKE_CURRENT_BINARY_DIR}")
      # Set the output directory for the executable
      set_target_properties(${executable_name} PROPERTIES
        RUNTIME_OUTPUT_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}"
        RUNTIME_OUTPUT_DIRECTORY_DEBUG "${CMAKE_CURRENT_BINARY_DIR}"
        RUNTIME_OUTPUT_DIRECTORY_RELEASE "${CMAKE_CURRENT_BINARY_DIR}"
        RUNTIME_OUTPUT_DIRECTORY_RELWITHDEBINFO "${CMAKE_CURRENT_BINARY_DIR}"
        RUNTIME_OUTPUT_DIRECTORY_MINSIZEREL "${CMAKE_CURRENT_BINARY_DIR}"
      )
    endif()
  endmacro()
endif()

# By default, include jitter entropy.
if(NOT DISABLE_CPU_JITTER_ENTROPY)
  add_subdirectory(third_party/jitterentropy)
endif()

# CMAKE_SYSTEM_NAME is "Generic" for embedded OSes:
# https://cmake.org/cmake/help/book/mastering-cmake/chapter/Cross%20Compiling%20With%20CMake.html#toolchain-files
#
# For now we assume embedded OSes do not have threads. Additionally, the Threads
# package does not work with Android, but Android does not require any extra
# parameters to link pthreads. Emscripten provides its own pthread implementation
# via Web Workers and SharedArrayBuffer, configured through compiler flags.
if(NOT CMAKE_SYSTEM_NAME MATCHES "^(Generic|Android|Emscripten)$")
  find_package(Threads REQUIRED)
  set(AWSLC_LINK_THREADS TRUE)
  # CMAKE_THREAD_LIBS_INIT contains the actual linker flags (e.g., -lpthread)
  # set by find_package(Threads). Use this for pkgconfig instead of the
  # imported target name.
  set(PKGCONFIG_LIBS_PRIVATE "${CMAKE_THREAD_LIBS_INIT}")
else()
  set(AWSLC_LINK_THREADS FALSE)
  set(PKGCONFIG_LIBS_PRIVATE "")
endif()


add_subdirectory(crypto)
if(BUILD_LIBSSL)
  add_subdirectory(ssl)
  if(BUILD_TOOL)
    add_subdirectory(tool)
    add_subdirectory(tool-openssl)
  endif()
endif()
add_subdirectory(util/fipstools)

if(FUZZ)
  if(LIBFUZZER_FROM_DEPS)
    file(GLOB LIBFUZZER_SOURCES "util/bot/libFuzzer/*.cpp")
    add_library(Fuzzer STATIC ${LIBFUZZER_SOURCES})
    # libFuzzer does not pass our aggressive warnings. It also must be built
    # without -fsanitize-coverage options or clang crashes.
    set_target_properties(Fuzzer PROPERTIES COMPILE_FLAGS "-Wno-shadow -Wno-format-nonliteral -Wno-missing-prototypes -fsanitize-coverage=0")
  endif()

  add_subdirectory(fuzz)
endif()


if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
  set(HANDSHAKER_ARGS "-handshaker-path" $<TARGET_FILE:handshaker>)
  set(SSL_TRANSFER_ARGS "-ssl-transfer-test-file" ${AWSLC_SOURCE_DIR}/ssl/test/runner/ssl_transfer/test_case_names.txt)
  if(DEFINED ENV{AWS_LC_SSL_RUNNER_START_INDEX})
    set(AWS_LC_SSL_RUNNER_INDEX_FILTER "-test-case-start-index" $ENV{AWS_LC_SSL_RUNNER_START_INDEX})
  endif()
  if(DEFINED ENV{AWS_LC_SSL_RUNNER_END_INDEX})
    set(AWS_LC_SSL_RUNNER_INDEX_FILTER "${AWS_LC_SSL_RUNNER_INDEX_FILTER}" "-test-case-end-index" $ENV{AWS_LC_SSL_RUNNER_END_INDEX})
  endif()
endif()

# Define GO_TEST_TIMEOUT based on env variable.
# This is needed because sanitizer test in aarch64 takes 45 mins, which exceeds `go test` default timeout(10m).
# https://golang.org/pkg/cmd/go/internal/test/
if(DEFINED ENV{AWS_LC_GO_TEST_TIMEOUT})
  set(GO_TEST_TIMEOUT "$ENV{AWS_LC_GO_TEST_TIMEOUT}")
else()
  set(GO_TEST_TIMEOUT "10m")
endif()

# Allow overriding the SSL test runner's idle timeout (default in runner.go is 30s).
# Useful for resource-constrained environments (e.g. FreeBSD VMs) where the shim
# may be slow to respond.
if(DEFINED ENV{AWS_LC_SSL_RUNNER_IDLE_TIMEOUT})
  set(RUNNER_ARGS ${RUNNER_ARGS} "-idle-timeout" "$ENV{AWS_LC_SSL_RUNNER_IDLE_TIMEOUT}")
endif()

# Allow enabling retry on runner-side TCP timeouts.
# Useful for resource-constrained environments (e.g. emulated FreeBSD VMs) where
# transient timeouts do not indicate real bugs.
if(DEFINED ENV{AWS_LC_SSL_RUNNER_RETRY_ON_TIMEOUT})
  set(RUNNER_ARGS ${RUNNER_ARGS} "-retry-on-timeout")
endif()

if (NOT ${CMAKE_VERSION} VERSION_LESS "3.2")
  # USES_TERMINAL is only available in CMake 3.2 or later.
  set(MAYBE_USES_TERMINAL USES_TERMINAL)
endif()

if(BUILD_TESTING)
  if(GO_EXECUTABLE)
    if(FIPS)
      if(WIN32)
        set(ACVP_TOOL ${AWSLC_BINARY_DIR}/acvptool.exe)
        set(TEST_WRAPPER ${AWSLC_BINARY_DIR}/testmodulewrapper.exe)
      else()
        set(ACVP_TOOL ${AWSLC_BINARY_DIR}/acvptool)
        set(TEST_WRAPPER ${AWSLC_BINARY_DIR}/testmodulewrapper)
      endif()

      # Read util/go_fips_tests.txt into a CMake variable.
      file(READ util/go_fips_tests.txt GO_FIPS_TESTS)
      string(REPLACE "\n" ";" GO_FIPS_TESTS "${GO_FIPS_TESTS}")
      list(REMOVE_ITEM GO_FIPS_TESTS "")
      set_property(DIRECTORY APPEND PROPERTY CMAKE_CONFIGURE_DEPENDS
        util/go_fips_tests.txt)

      if(WIN32)
        # Delocator doesn't support Windows. So, this test would fail.
        string(REGEX REPLACE "./util/fipstools/delocate" "" GO_FIPS_TESTS "${GO_FIPS_TESTS}")
      endif()

      # We want to run the SHA tests iteratively because they contain Large Data Tests, which make
      # us hash anywhere from 1-8 gigabytes of memory, so running multiple at the same time cause a
      # failure on most machines due to a lack of memory.
      set(acvp_sha_test_commands)
      file(GLOB acvp_sha_tests "${AWSLC_SOURCE_DIR}/util/fipstools/acvp/acvptool/test/sha-tests/*.json")
      foreach(test_path ${acvp_sha_tests})
        get_filename_component(test_name ${test_path} NAME)
        list(APPEND acvp_sha_test_commands
              COMMAND ${GO_EXECUTABLE} run check_expected.go
                        -tool ${ACVP_TOOL}
                        -module-wrappers modulewrapper:$<TARGET_FILE:modulewrapper>,testmodulewrapper:${TEST_WRAPPER}
                        -tests "sha-tests/${test_name}")
      endforeach()

      add_custom_target(
        build_acvp_tool
        COMMAND ${GO_EXECUTABLE} build -o ${ACVP_TOOL}
                github.com/aws/aws-lc/util/fipstools/acvp/acvptool
        COMMAND ${GO_EXECUTABLE} build -o ${TEST_WRAPPER}
                github.com/aws/aws-lc/util/fipstools/acvp/acvptool/testmodulewrapper
        WORKING_DIRECTORY ${AWSLC_SOURCE_DIR}
        DEPENDS modulewrapper
        ${MAYBE_USES_TERMINAL})

      add_custom_target(
        acvp_tests
        COMMAND ${CMAKE_COMMAND} -E echo
        COMMAND ${CMAKE_COMMAND} -E echo "Running ACVP tests"
        COMMAND ${GO_EXECUTABLE} run check_expected.go
                  -tool ${ACVP_TOOL}
                  -module-wrappers modulewrapper:$<TARGET_FILE:modulewrapper>,testmodulewrapper:${TEST_WRAPPER}
                  -tests tests.json
        ${acvp_sha_test_commands}
        WORKING_DIRECTORY ${AWSLC_SOURCE_DIR}/util/fipstools/acvp/acvptool/test
        DEPENDS build_acvp_tool
        ${MAYBE_USES_TERMINAL})

      add_custom_target(
        fips_specific_tests_if_any
        DEPENDS acvp_tests
      )
    else()
      add_custom_target(fips_specific_tests_if_any)
    endif()

    # Add macho parser tests if FIPS and on MacOS
    if(FIPS AND APPLE)
      add_custom_target(
        macho_parser_tests
        COMMAND ./util/fipstools/inject_hash/macho_parser/tests/test_macho_parser
        WORKING_DIRECTORY ${AWSLC_BINARY_DIR}
        DEPENDS test_macho_parser
      )
      add_dependencies(fips_specific_tests_if_any macho_parser_tests)
    endif()

    # Read util/go_tests.txt into a CMake variable.
    file(READ util/go_tests.txt GO_TESTS)
    foreach(fips_specific_test ${GO_FIPS_TESTS})
      set(GO_TESTS "${GO_TESTS}\n${fips_specific_test}")
    endforeach()
    string(REPLACE "\n" ";" GO_TESTS "${GO_TESTS}")
    list(REMOVE_ITEM GO_TESTS "")
    set_property(DIRECTORY APPEND PROPERTY CMAKE_CONFIGURE_DEPENDS
      util/go_tests.txt)

    if(BUILD_LIBSSL)
      add_custom_target(
          run_ssl_runner_tests
          COMMAND ${CMAKE_COMMAND} -E echo "Running SSL tests"
          COMMAND cd ssl/test/runner &&
                  ${GO_EXECUTABLE} test -timeout ${GO_TEST_TIMEOUT} -shim-path $<TARGET_FILE:bssl_shim>
                    ${HANDSHAKER_ARGS} ${RUNNER_ARGS} ${AWS_LC_SSL_RUNNER_INDEX_FILTER} ${SSL_TRANSFER_ARGS}
          WORKING_DIRECTORY ${AWSLC_SOURCE_DIR}
          DEPENDS bssl_shim handshaker fips_specific_tests_if_any
          ${MAYBE_USES_TERMINAL})

      add_custom_target(
          run_ssl_runner_tests_valgrind
          COMMAND ${CMAKE_COMMAND} -E echo "Running SSL tests"
          COMMAND cd ssl/test/runner &&
          ${GO_EXECUTABLE} test -timeout ${GO_TEST_TIMEOUT} -shim-path $<TARGET_FILE:bssl_shim> -valgrind
          ${HANDSHAKER_ARGS} ${RUNNER_ARGS} ${AWS_LC_SSL_RUNNER_INDEX_FILTER} ${SSL_TRANSFER_ARGS}
          WORKING_DIRECTORY ${AWSLC_SOURCE_DIR}
          DEPENDS bssl_shim handshaker fips_specific_tests_if_any
          ${MAYBE_USES_TERMINAL})

      add_custom_target(
          run_tests
          COMMAND ${CMAKE_COMMAND} -E echo
          COMMAND ${CMAKE_COMMAND} -E echo "Running Go tests"
          COMMAND ${GO_EXECUTABLE} test ${GO_TESTS}
          COMMAND ${CMAKE_COMMAND} -E echo
          COMMAND ${CMAKE_COMMAND} -E echo "Running unit tests"
          COMMAND ${GO_EXECUTABLE} run util/all_tests.go -build-dir
                 ${AWSLC_BINARY_DIR}
          WORKING_DIRECTORY ${AWSLC_SOURCE_DIR}
          DEPENDS all_tests run_ssl_runner_tests
          ${MAYBE_USES_TERMINAL})
    else()
      add_custom_target(
          run_tests
          COMMAND ${CMAKE_COMMAND} -E echo "Running Go tests"
          COMMAND ${GO_EXECUTABLE} test ${GO_TESTS}
          COMMAND ${CMAKE_COMMAND} -E echo
          COMMAND ${CMAKE_COMMAND} -E echo "Running unit tests"
          COMMAND ${GO_EXECUTABLE} run util/all_tests.go -build-dir
                  ${AWSLC_BINARY_DIR} -ssl-tests=false
          WORKING_DIRECTORY ${AWSLC_SOURCE_DIR}
          DEPENDS all_tests fips_specific_tests_if_any
          ${MAYBE_USES_TERMINAL}
      )
    endif()

    add_custom_target(
        run_tests_valgrind
        COMMAND ${GO_EXECUTABLE} run util/all_tests.go -build-dir
                ${AWSLC_BINARY_DIR} -valgrind=true -valgrind-supp-dir="tests/ci"
        WORKING_DIRECTORY ${AWSLC_SOURCE_DIR}
        DEPENDS all_tests
        ${MAYBE_USES_TERMINAL})

    add_custom_target(
        run_tests_with_sde
        COMMAND ${GO_EXECUTABLE} run util/all_tests.go -build-dir
                ${AWSLC_BINARY_DIR} -sde=true -sde-path="$ENV{SDEROOT}/sde"
        WORKING_DIRECTORY ${AWSLC_SOURCE_DIR}
        DEPENDS all_tests
        ${MAYBE_USES_TERMINAL})
  else()
    if(BUILD_LIBSSL)
      add_custom_target(
          run_minimal_tests
          COMMAND crypto/crypto_test
          COMMAND crypto/urandom_test
          COMMAND ssl/ssl_test
          WORKING_DIRECTORY ${AWSLC_BINARY_DIR}
          DEPENDS all_tests
          ${MAYBE_USES_TERMINAL})
    else()
      add_custom_target(
          run_minimal_tests
          COMMAND crypto/crypto_test
          COMMAND crypto/urandom_test
          WORKING_DIRECTORY ${AWSLC_BINARY_DIR}
          DEPENDS all_tests
          ${MAYBE_USES_TERMINAL}
      )
    endif()
  endif()
endif()

if(NOT MSVC AND NOT CLANG AND NOT GCC)
  message(STATUS "Alternative compiler '${CMAKE_C_COMPILER_ID}' detected. Not all flags may be set, check final options with 'cmake --build . -- VERBOSE=1'")
endif()

include(cmake/JoinPaths.cmake)
join_paths(LIBDIR_FOR_PC_FILE "\${prefix}" "${CMAKE_INSTALL_LIBDIR}")
join_paths(INCLUDEDIR_FOR_PC_FILE "\${prefix}" "${AWSLC_INSTALL_INCLUDEDIR}")

function(install_pkgconfig_file)
  set(options "")
  set(oneValueArgs TEMPLATE DEST)
  set(multiValueArgs)
  if(CMAKE_VERSION VERSION_LESS "3.7")
    cmake_parse_arguments(arg "${options}" "${oneValueArgs}" "${multiValueArgs}" ${ARGN})
  else()
    cmake_parse_arguments(PARSE_ARGV 0 arg "${options}" "${oneValueArgs}" "${multiValueArgs}")
  endif()

  configure_file(pkgconfig/${arg_TEMPLATE} ${CMAKE_CURRENT_BINARY_DIR}/pkgconfig/${arg_DEST} @ONLY)
  install(FILES ${CMAKE_CURRENT_BINARY_DIR}/pkgconfig/${arg_DEST} DESTINATION ${CMAKE_INSTALL_LIBDIR}/pkgconfig)
endfunction()

if(ENABLE_SOURCE_MODIFICATION)
  configure_file(include/openssl/base.h.in ${AWSLC_SOURCE_DIR}/include/openssl/base.h @ONLY)
  configure_file(include/openssl/opensslv.h.in ${AWSLC_SOURCE_DIR}/include/openssl/opensslv.h @ONLY)
  if(EXISTS "${CMAKE_CURRENT_SOURCE_DIR}/util/check-linkage.sh.in")
    configure_file(util/check-linkage.sh.in check-linkage.sh @ONLY)
  endif()
endif()

install_pkgconfig_file(TEMPLATE product.pc.in DEST aws-lc.pc)
install_pkgconfig_file(TEMPLATE libcrypto.pc.in DEST lib${CRYPTO_LIB_NAME}.pc)
install_pkgconfig_file(TEMPLATE libssl.pc.in DEST lib${SSL_LIB_NAME}.pc)

if(INSTALL_OPENSSL_SHIM)
  install_pkgconfig_file(TEMPLATE product.pc.in DEST openssl.pc)

  # Create OpenSSL compatibility symlinks
  if(BUILD_SHARED_LIBS)
    if(SET_LIB_SONAME)
      # When SONAME build is enabled, libraries have -awslc suffix
      install(CODE "
        execute_process(COMMAND \${CMAKE_COMMAND} -E create_symlink
          lib${CRYPTO_LIB_NAME}.so \"\$ENV{DESTDIR}\${CMAKE_INSTALL_PREFIX}/${CMAKE_INSTALL_LIBDIR}/libcrypto.so\")
        execute_process(COMMAND \${CMAKE_COMMAND} -E create_symlink
          lib${SSL_LIB_NAME}.so \"\$ENV{DESTDIR}\${CMAKE_INSTALL_PREFIX}/${CMAKE_INSTALL_LIBDIR}/libssl.so\")
      ")
    endif()
  else()
    if(SET_LIB_SONAME)
      # When SONAME build is enabled, libraries have -awslc suffix
      install(CODE "
        execute_process(COMMAND \${CMAKE_COMMAND} -E create_symlink
          lib${CRYPTO_LIB_NAME}.a \"\$ENV{DESTDIR}\${CMAKE_INSTALL_PREFIX}/${CMAKE_INSTALL_LIBDIR}/libcrypto.a\")
        execute_process(COMMAND \${CMAKE_COMMAND} -E create_symlink
          lib${SSL_LIB_NAME}.a \"\$ENV{DESTDIR}\${CMAKE_INSTALL_PREFIX}/${CMAKE_INSTALL_LIBDIR}/libssl.a\")
      " COMPONENT Development)
    endif()
  endif()

  if(COHABITANT_HEADERS)
    # Always create the include directory symlink for OpenSSL compatibility
    install(CODE "
      execute_process(COMMAND \${CMAKE_COMMAND} -E create_symlink
        aws-lc/openssl \"\$ENV{DESTDIR}\${CMAKE_INSTALL_PREFIX}/${CMAKE_INSTALL_INCLUDEDIR}/openssl\")
    " COMPONENT Development)
  endif()
endif()
