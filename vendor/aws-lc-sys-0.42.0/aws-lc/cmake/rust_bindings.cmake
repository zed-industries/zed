# Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0 OR ISC

# Module for generating Rust bindings using bindgen-cli
#
# Background: Symbol Prefix Mechanism
# ------------------------------------
# AWS-LC supports building with a symbol prefix (via BORINGSSL_PREFIX) to avoid
# symbol collisions when multiple copies of the library are linked into one binary.
# The C build achieves this by generating a boringssl_prefix_symbols.h header that
# #define-renames every public symbol (e.g., #define SSL_new AWSLC_SSL_new), then
# force-including it during compilation.
#
# For Rust bindings, we must NOT include that prefix header when invoking bindgen.
# Instead, we use bindgen's --prefix-link-name option, which adds the prefix to
# #[link_name] attributes (for the linker) while keeping the Rust function names
# unprefixed. If we included the prefix header, bindgen would see already-prefixed
# names and --prefix-link-name would double-prefix them, producing broken bindings.
#
# This produces bindings like:
#   extern "C" {
#       #[link_name = "AWSLC_SSL_new"]  // Prefixed for linker
#       pub fn SSL_new(...) -> ...;      // Unprefixed for Rust API
#   }

# Headers to exclude from Rust bindings generation.
# Each exclusion should have a comment explaining why.
set(RUST_WRAPPER_EXCLUDED_HEADERS
  # Template files (not actual headers, used by configure_file)
  "base.h.in"
  "opensslv.h.in"

  # Prefix symbol headers must not be included. See module-level comment above.
  "boringssl_prefix_symbols.h"
  "boringssl_prefix_symbols_asm.h"
  "boringssl_prefix_symbols_nasm.inc"

  # Platform/architecture-specific internal headers (not part of public API)
  "arm_arch.h"
  "asm_base.h"
  "target.h"

  # SSL headers (conditionally included via AWS_LC_RUST_INCLUDE_SSL define)
  "ssl.h"
  "ssl3.h"
  "tls1.h"
  "dtls1.h"
)

# Discover all OpenSSL headers for dependency tracking
# Uses CONFIGURE_DEPENDS on CMake 3.12+ for automatic reconfiguration
function(discover_openssl_headers OUT_VAR OUT_EXPERIMENTAL_VAR)
  if(CMAKE_VERSION VERSION_GREATER_EQUAL "3.12")
    file(GLOB _all_headers
         CONFIGURE_DEPENDS
         "${AWSLC_SOURCE_DIR}/include/openssl/*.h")
    file(GLOB _experimental_headers
         CONFIGURE_DEPENDS
         "${AWSLC_SOURCE_DIR}/include/openssl/experimental/*.h")
  else()
    file(GLOB _all_headers "${AWSLC_SOURCE_DIR}/include/openssl/*.h")
    file(GLOB _experimental_headers "${AWSLC_SOURCE_DIR}/include/openssl/experimental/*.h")
  endif()

  set(${OUT_VAR} "${_all_headers}" PARENT_SCOPE)
  set(${OUT_EXPERIMENTAL_VAR} "${_experimental_headers}" PARENT_SCOPE)
endfunction()

# Generate rust_wrapper.h from template by discovering headers
function(generate_rust_wrapper_header)
  discover_openssl_headers(_all_headers _experimental_headers)

  # Build the include list, filtering out excluded headers
  set(_includes "")
  foreach(_header ${_all_headers})
    get_filename_component(_header_name "${_header}" NAME)
    list(FIND RUST_WRAPPER_EXCLUDED_HEADERS "${_header_name}" _excluded_idx)
    if(_excluded_idx EQUAL -1)
      string(APPEND _includes "#include <openssl/${_header_name}>\n")
    endif()
  endforeach()

  # Build the experimental include list
  set(_experimental_includes "")
  foreach(_header ${_experimental_headers})
    get_filename_component(_header_name "${_header}" NAME)
    string(APPEND _experimental_includes "#include <openssl/experimental/${_header_name}>\n")
  endforeach()

  # Set variables for configure_file substitution
  set(RUST_WRAPPER_INCLUDES "${_includes}")
  set(RUST_WRAPPER_EXPERIMENTAL_INCLUDES "${_experimental_includes}")

  # Generate the header from template
  configure_file(
    "${AWSLC_SOURCE_DIR}/cmake/rust_wrapper.h.in"
    "${AWSLC_BINARY_DIR}/include/rust_wrapper.h"
    @ONLY
  )

  message(STATUS "Generated rust_wrapper.h with discovered headers")
endfunction()

# Determine the symbol prefix format based on target platform
#
# Different platforms have different symbol naming conventions:
# - Apple platforms (macOS, iOS): Symbols have a leading underscore (_SSL_new)
# - 32-bit Windows: Symbols have a leading underscore (_SSL_new)
# - Other platforms (Linux, 64-bit Windows): No leading underscore (SSL_new)
#
# When we add our prefix, we need to account for this:
# - Apple/Win32: _AWSLC_SSL_new (prefix goes after the leading underscore)
# - Others: AWSLC_SSL_new
#
# The --prefix-link-name option prepends the given string to link names,
# so we format it appropriately for each platform.
function(get_symbol_prefix_format PREFIX OUT_VAR)
  # Use CMAKE_SYSTEM_NAME for cross-compilation support
  # (WIN32/APPLE variables reflect the HOST, not the TARGET)
  if(CMAKE_SYSTEM_NAME STREQUAL "Darwin" OR CMAKE_SYSTEM_NAME STREQUAL "iOS")
    set(${OUT_VAR} "_${PREFIX}_" PARENT_SCOPE)
  elseif(CMAKE_SYSTEM_NAME STREQUAL "Windows" AND CMAKE_SIZEOF_VOID_P EQUAL 4)
    set(${OUT_VAR} "_${PREFIX}_" PARENT_SCOPE)
  else()
    set(${OUT_VAR} "${PREFIX}_" PARENT_SCOPE)
  endif()
endfunction()

# Generate Rust bindings using bindgen-cli
#
# Arguments:
#   OUTPUT_FILE  - Path where the generated .rs file will be written
#   INCLUDE_DIRS - List of include directories for clang
#   PREFIX       - (Optional) Symbol prefix (e.g., "AWSLC")
#   INCLUDE_SSL  - (Optional) Flag to include SSL headers
function(generate_rust_bindings)
  set(options INCLUDE_SSL)
  set(oneValueArgs OUTPUT_FILE PREFIX)
  set(multiValueArgs INCLUDE_DIRS)
  cmake_parse_arguments(ARG "${options}" "${oneValueArgs}" "${multiValueArgs}" ${ARGN})

  # Discover headers for dependency tracking
  discover_openssl_headers(_all_openssl_headers _experimental_headers)

  # Build include path arguments for clang
  set(_clang_include_args "")
  foreach(_dir ${ARG_INCLUDE_DIRS})
    list(APPEND _clang_include_args "-I${_dir}")
  endforeach()

  # Build bindgen arguments (based on aws-lc-rs/aws-lc-sys configuration)
  #
  # Note on regex escaping for --allowlist-file:
  # - CMake requires \\ to represent a single backslash in strings
  # - The regex itself needs \\ to match a literal backslash (for Windows paths)
  # - Combined: \\\\ in CMake -> \\ passed to bindgen -> \\ in regex -> matches one backslash
  # - The pattern matches paths like: .../openssl/ssl.h or ...\openssl\ssl.h
  set(_bindgen_args
    "--allowlist-file" ".*(/|\\\\)openssl((/|\\\\)[^/\\\\]+)+\\.h"
    "--allowlist-file" ".*(/|\\\\)rust_wrapper\\.h"
    "--rustified-enum" "point_conversion_form_t"
    "--default-macro-constant-type" "signed"
    "--with-derive-default"
    "--with-derive-partialeq"
    "--with-derive-eq"
    "--generate" "functions,types,vars,methods,constructors,destructors"
    "--rust-target" "${RUST_BINDINGS_TARGET_VERSION}"
  )
  # Use rustfmt as the formatter only if it was discovered. Otherwise fall back
  # to bindgen's default ("prettyplease") so the build doesn't require rustfmt.
  if(RUSTFMT_EXECUTABLE)
    list(APPEND _bindgen_args "--formatter" "rustfmt")
  endif()
  # Add symbol prefix if specified. See module-level comment for why we use
  # --prefix-link-name instead of including the prefix symbols header.
  if(ARG_PREFIX AND NOT ARG_PREFIX STREQUAL "")
    get_symbol_prefix_format("${ARG_PREFIX}" _sym_prefix)
    list(APPEND _bindgen_args "--prefix-link-name" "${_sym_prefix}")
    message(STATUS "Rust bindings will use symbol prefix: ${_sym_prefix}")
  endif()

  # Build clang arguments (prefix symbols header is intentionally excluded)
  set(_clang_args "")

  # Add SSL support if requested
  if(ARG_INCLUDE_SSL)
    list(APPEND _clang_args "-DAWS_LC_RUST_INCLUDE_SSL")
    message(STATUS "Rust bindings: SSL support enabled (INCLUDE_SSL=ON)")
  else()
    message(STATUS "Rust bindings: SSL support disabled (INCLUDE_SSL not set)")
  endif()

  # Add include paths (unprefixed headers only)
  list(APPEND _clang_args ${_clang_include_args})

  # Ensure output directory exists
  get_filename_component(_output_dir "${ARG_OUTPUT_FILE}" DIRECTORY)

  message(STATUS "Rust bindings clang args: ${_clang_args}")
  message(STATUS "Rust bindings bindgen args: ${_bindgen_args}")

  # Create custom command for generating bindings
  # Dependencies include all OpenSSL headers to ensure regeneration when headers change
  #
  # Note: VERBATIM is used to ensure proper argument escaping across platforms,
  # especially important for the regex patterns on Windows. If debugging command-line
  # issues, be aware that VERBATIM affects how CMake quotes arguments.
  add_custom_command(
    OUTPUT "${ARG_OUTPUT_FILE}"
    COMMAND ${CMAKE_COMMAND} -E make_directory "${_output_dir}"
    COMMAND ${BINDGEN_EXECUTABLE}
      ${_bindgen_args}
      "${AWSLC_BINARY_DIR}/include/rust_wrapper.h"
      "--output" "${ARG_OUTPUT_FILE}"
      "--"
      ${_clang_args}
    DEPENDS
      "${AWSLC_BINARY_DIR}/include/rust_wrapper.h"
      "${AWSLC_SOURCE_DIR}/cmake/rust_wrapper.h.in"
      ${_all_openssl_headers}
      ${_experimental_headers}
    COMMENT "Generating Rust bindings with bindgen-cli"
    VERBATIM
  )

  # Create target for bindings generation (not part of ALL; built on-demand or via install)
  add_custom_target(rust_bindings DEPENDS "${ARG_OUTPUT_FILE}")
endfunction()
