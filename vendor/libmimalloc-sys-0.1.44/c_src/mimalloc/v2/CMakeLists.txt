cmake_minimum_required(VERSION 3.18)
project(libmimalloc C CXX)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

option(MI_SECURE            "Use full security mitigations (like guard pages, allocation randomization, double-free mitigation, and free-list corruption detection)" OFF)
option(MI_DEBUG_FULL        "Use full internal heap invariant checking in DEBUG mode (expensive)" OFF)
option(MI_PADDING           "Enable padding to detect heap block overflow (always on in DEBUG or SECURE mode, or with Valgrind/ASAN)" OFF)
option(MI_OVERRIDE          "Override the standard malloc interface (i.e. define entry points for 'malloc', 'free', etc)" ON)
option(MI_XMALLOC           "Enable abort() call on memory allocation failure by default" OFF)
option(MI_SHOW_ERRORS       "Show error and warning messages by default (only enabled by default in DEBUG mode)" OFF)
option(MI_TRACK_VALGRIND    "Compile with Valgrind support (adds a small overhead)" OFF)
option(MI_TRACK_ASAN        "Compile with address sanitizer support (adds a small overhead)" OFF)
option(MI_TRACK_ETW         "Compile with Windows event tracing (ETW) support (adds a small overhead)" OFF)
option(MI_USE_CXX           "Use the C++ compiler to compile the library (instead of the C compiler)" OFF)
option(MI_OPT_ARCH          "Only for optimized builds: turn on architecture specific optimizations (for arm64: '-march=armv8.1-a' (2016))" OFF)
option(MI_SEE_ASM           "Generate assembly files" OFF)
option(MI_OSX_INTERPOSE     "Use interpose to override standard malloc on macOS" ON)
option(MI_OSX_ZONE          "Use malloc zone to override standard malloc on macOS" ON)
option(MI_WIN_REDIRECT      "Use redirection module ('mimalloc-redirect') on Windows if compiling mimalloc as a DLL" ON)
option(MI_WIN_USE_FIXED_TLS "Use a fixed TLS slot on Windows to avoid extra tests in the malloc fast path" OFF)
option(MI_LOCAL_DYNAMIC_TLS "Use local-dynamic-tls, a slightly slower but dlopen-compatible thread local storage mechanism (Unix)" OFF)
option(MI_LIBC_MUSL         "Set this when linking with musl libc" OFF)
option(MI_BUILD_SHARED      "Build shared library" ON)
option(MI_BUILD_STATIC      "Build static library" ON)
option(MI_BUILD_OBJECT      "Build object library" ON)
option(MI_BUILD_TESTS       "Build test executables" ON)
option(MI_DEBUG_TSAN        "Build with thread sanitizer (needs clang)" OFF)
option(MI_DEBUG_UBSAN       "Build with undefined-behavior sanitizer (needs clang++)" OFF)
option(MI_GUARDED           "Build with guard pages behind certain object allocations (implies MI_NO_PADDING=ON)" OFF)
option(MI_SKIP_COLLECT_ON_EXIT "Skip collecting memory on program exit" OFF)
option(MI_NO_PADDING        "Force no use of padding even in DEBUG mode etc." OFF)
option(MI_INSTALL_TOPLEVEL  "Install directly into $CMAKE_INSTALL_PREFIX instead of PREFIX/lib/mimalloc-version" OFF)
option(MI_NO_THP            "Disable transparent huge pages support on Linux/Android for the mimalloc process only" OFF)
option(MI_EXTRA_CPPDEFS     "Extra pre-processor definitions (use as `-DMI_EXTRA_CPPDEFS=\"opt1=val1;opt2=val2\"`)" "")

# negated options for vcpkg features
option(MI_NO_USE_CXX        "Use plain C compilation (has priority over MI_USE_CXX)" OFF)
option(MI_NO_OPT_ARCH       "Do not use architecture specific optimizations (like '-march=armv8.1-a' for example) (has priority over MI_OPT_ARCH)" OFF)

# deprecated options
option(MI_WIN_USE_FLS       "Use Fiber local storage on Windows to detect thread termination (deprecated)" OFF)
option(MI_CHECK_FULL        "Use full internal invariant checking in DEBUG mode (deprecated, use MI_DEBUG_FULL instead)" OFF)
option(MI_USE_LIBATOMIC     "Explicitly link with -latomic (on older systems) (deprecated and detected automatically)" OFF)

include(CheckLinkerFlag)    # requires cmake 3.18
include(CheckIncludeFiles)
include(GNUInstallDirs)
include("cmake/mimalloc-config-version.cmake")

set(mi_sources
    src/alloc.c
    src/alloc-aligned.c
    src/alloc-posix.c
    src/arena.c
    src/bitmap.c
    src/heap.c
    src/init.c
    src/libc.c
    src/options.c
    src/os.c
    src/page.c
    src/random.c
    src/segment.c
    src/segment-map.c
    src/stats.c
    src/prim/prim.c)

set(mi_cflags "")
set(mi_cflags_static "")            # extra flags for a static library build
set(mi_cflags_dynamic "")           # extra flags for a shared-object library build
set(mi_libraries "")

if(MI_EXTRA_CPPDEFS)
 set(mi_defines ${MI_EXTRA_CPPDEFS})
else()
 set(mi_defines "")
endif()

# pass git revision as a define
if(EXISTS "${CMAKE_SOURCE_DIR}/.git/index")
  find_package(Git)
  if(GIT_FOUND)
    execute_process(COMMAND ${GIT_EXECUTABLE} "describe" OUTPUT_VARIABLE mi_git_describe RESULT_VARIABLE mi_git_res ERROR_QUIET OUTPUT_STRIP_TRAILING_WHITESPACE)
    if(mi_git_res EQUAL "0")
      list(APPEND mi_defines "MI_GIT_DESCRIBE=${mi_git_describe}")
      # add to dependencies so we rebuild if the git head commit changes
      set_property(GLOBAL APPEND PROPERTY CMAKE_CONFIGURE_DEPENDS "${CMAKE_SOURCE_DIR}/.git/index")
    endif()
  endif()
endif()

# -----------------------------------------------------------------------------
# Convenience: set default build type and compiler depending on the build directory
# -----------------------------------------------------------------------------

message(STATUS "")
if (NOT CMAKE_BUILD_TYPE)
  if ("${CMAKE_BINARY_DIR}" MATCHES ".*((D|d)ebug|asan|tsan|ubsan|valgrind)$" OR MI_DEBUG_FULL)
    message(STATUS "No build type selected, default to 'Debug'")
    set(CMAKE_BUILD_TYPE "Debug")
  else()
    message(STATUS "No build type selected, default to 'Release'")
    set(CMAKE_BUILD_TYPE "Release")
  endif()
endif()

if (CMAKE_GENERATOR MATCHES "^Visual Studio.*$")
  message(STATUS "Note: when building with Visual Studio the build type is specified when building.")
  message(STATUS "For example: 'cmake --build . --config=Release")
endif()

if("${CMAKE_BINARY_DIR}" MATCHES ".*(S|s)ecure$")
  message(STATUS "Default to secure build")
  set(MI_SECURE "ON")
endif()


# Determine architecture
set(MI_OPT_ARCH_FLAGS "")
set(MI_ARCH "unknown")
if(CMAKE_SYSTEM_PROCESSOR MATCHES "^(x86|i[3456]86)$" OR CMAKE_GENERATOR_PLATFORM MATCHES "^(x86|Win32)$")
  set(MI_ARCH "x86")
elseif(CMAKE_SYSTEM_PROCESSOR MATCHES "^(x86_64|x64|amd64|AMD64)$" OR CMAKE_GENERATOR_PLATFORM STREQUAL "x64" OR "x86_64" IN_LIST CMAKE_OSX_ARCHITECTURES) # must be before arm64
  set(MI_ARCH "x64")
elseif(CMAKE_SYSTEM_PROCESSOR MATCHES "^(aarch64|arm64|armv[89].?|ARM64)$" OR CMAKE_GENERATOR_PLATFORM STREQUAL "ARM64" OR "arm64" IN_LIST CMAKE_OSX_ARCHITECTURES)
  set(MI_ARCH "arm64")
elseif(CMAKE_SYSTEM_PROCESSOR MATCHES "^(arm|armv[34567].?|ARM)$")
  set(MI_ARCH "arm32")
elseif(CMAKE_SYSTEM_PROCESSOR MATCHES "^(riscv|riscv32|riscv64)$")
  if(CMAKE_SIZEOF_VOID_P==4)
    set(MI_ARCH "riscv32")
  else()
    set(MI_ARCH "riscv64")
  endif()
else()
  set(MI_ARCH ${CMAKE_SYSTEM_PROCESSOR})
endif()
message(STATUS "Architecture: ${MI_ARCH}") # (${CMAKE_SYSTEM_PROCESSOR}, ${CMAKE_GENERATOR_PLATFORM}, ${CMAKE_GENERATOR})")

# negative overrides (mainly to support vcpkg features)
if(MI_NO_USE_CXX)
  set(MI_USE_CXX "OFF")
endif()
if(MI_NO_OPT_ARCH)
  set(MI_OPT_ARCH "OFF")
elseif(MI_ARCH STREQUAL "arm64")
  set(MI_OPT_ARCH "ON")  # enable armv8.1-a by default on arm64 unless MI_NO_OPT_ARCH is set
endif()


# -----------------------------------------------------------------------------
# Process options
# -----------------------------------------------------------------------------

if(CMAKE_C_COMPILER_ID STREQUAL "Clang" AND CMAKE_CXX_COMPILER_FRONTEND_VARIANT STREQUAL "MSVC")
  set(MI_CLANG_CL "ON")
endif()

# put -Wall early so other warnings can be disabled selectively
if(CMAKE_C_COMPILER_ID MATCHES "AppleClang|Clang")
  if (MI_CLANG_CL)
    list(APPEND mi_cflags -W)
  else()
    list(APPEND mi_cflags -Wall -Wextra -Wpedantic)
  endif()
endif()
if(CMAKE_C_COMPILER_ID MATCHES "GNU")
    list(APPEND mi_cflags -Wall -Wextra)
endif()
if(CMAKE_C_COMPILER_ID MATCHES "Intel")
    list(APPEND mi_cflags -Wall)
endif()

# force C++ compilation with msvc or clang-cl to use modern C++ atomics
if(CMAKE_C_COMPILER_ID MATCHES "MSVC|Intel" OR MI_CLANG_CL)
  set(MI_USE_CXX "ON")
endif()

if(MI_OVERRIDE)
  message(STATUS "Override standard malloc (MI_OVERRIDE=ON)")
  if(APPLE)
    if(MI_OSX_ZONE)
      # use zone's on macOS
      message(STATUS "  Use malloc zone to override malloc (MI_OSX_ZONE=ON)")
      list(APPEND mi_sources src/prim/osx/alloc-override-zone.c)
      list(APPEND mi_defines MI_OSX_ZONE=1)
      if (NOT MI_OSX_INTERPOSE)
        message(STATUS "  WARNING: zone overriding usually also needs interpose (use -DMI_OSX_INTERPOSE=ON)")
      endif()
    endif()
    if(MI_OSX_INTERPOSE)
      # use interpose on macOS
      message(STATUS "  Use interpose to override malloc (MI_OSX_INTERPOSE=ON)")
      list(APPEND mi_defines MI_OSX_INTERPOSE=1)
      if (NOT MI_OSX_ZONE)
        message(STATUS "  WARNING: interpose usually also needs zone overriding (use -DMI_OSX_INTERPOSE=ON)")
      endif()
    endif()
    if(MI_USE_CXX AND MI_OSX_INTERPOSE)
      message(STATUS "  WARNING: if dynamically overriding malloc/free, it is more reliable to build mimalloc as C code (use -DMI_USE_CXX=OFF)")
    endif()
  endif()
endif()

if(WIN32)
  if (NOT MI_WIN_REDIRECT)
    # use a negative define for backward compatibility
    list(APPEND mi_defines MI_WIN_NOREDIRECT=1)
  endif()
endif()

if(MI_SECURE)
  message(STATUS "Set full secure build (MI_SECURE=ON)")
  list(APPEND mi_defines MI_SECURE=4)
endif()

if(MI_TRACK_VALGRIND)
  CHECK_INCLUDE_FILES("valgrind/valgrind.h;valgrind/memcheck.h" MI_HAS_VALGRINDH)
  if (NOT MI_HAS_VALGRINDH)
    set(MI_TRACK_VALGRIND OFF)
    message(WARNING "Cannot find the 'valgrind/valgrind.h' and 'valgrind/memcheck.h' -- install valgrind first?")
    message(STATUS  "Disabling Valgrind support (MI_TRACK_VALGRIND=OFF)")
  else()
    message(STATUS "Compile with Valgrind support (MI_TRACK_VALGRIND=ON)")
    list(APPEND mi_defines MI_TRACK_VALGRIND=1)
  endif()
endif()

if(MI_TRACK_ASAN)
  if (APPLE AND MI_OVERRIDE)
    set(MI_TRACK_ASAN OFF)
    message(WARNING "Cannot enable address sanitizer support on macOS if MI_OVERRIDE is ON (MI_TRACK_ASAN=OFF)")
  endif()
  if (MI_TRACK_VALGRIND)
    set(MI_TRACK_ASAN OFF)
    message(WARNING "Cannot enable address sanitizer support with also Valgrind support enabled (MI_TRACK_ASAN=OFF)")
  endif()
  if(MI_TRACK_ASAN)
    CHECK_INCLUDE_FILES("sanitizer/asan_interface.h" MI_HAS_ASANH)
    if (NOT MI_HAS_ASANH)
      set(MI_TRACK_ASAN OFF)
      message(WARNING "Cannot find the 'sanitizer/asan_interface.h' -- install address sanitizer support first")
      message(STATUS  "Compile **without** address sanitizer support (MI_TRACK_ASAN=OFF)")
    else()
      message(STATUS "Compile with address sanitizer support (MI_TRACK_ASAN=ON)")
      list(APPEND mi_defines MI_TRACK_ASAN=1)
      list(APPEND mi_cflags -fsanitize=address)
      list(APPEND mi_libraries -fsanitize=address)
    endif()
  endif()
endif()

if(MI_TRACK_ETW)
  if(NOT WIN32)
    set(MI_TRACK_ETW OFF)
    message(WARNING "Can only enable ETW support on Windows (MI_TRACK_ETW=OFF)")
  endif()
  if (MI_TRACK_VALGRIND OR MI_TRACK_ASAN)
    set(MI_TRACK_ETW OFF)
    message(WARNING "Cannot enable ETW support with also Valgrind or ASAN support enabled (MI_TRACK_ETW=OFF)")
  endif()
  if(MI_TRACK_ETW)
    message(STATUS "Compile with Windows event tracing support (MI_TRACK_ETW=ON)")
    list(APPEND mi_defines MI_TRACK_ETW=1)
  endif()
endif()

if(MI_GUARDED)
  message(STATUS "Compile guard pages behind certain object allocations (MI_GUARDED=ON)")
  list(APPEND mi_defines MI_GUARDED=1)
  if(NOT MI_NO_PADDING)
    message(STATUS "  Disabling padding due to guard pages (MI_NO_PADDING=ON)")
    set(MI_NO_PADDING ON)
  endif()
endif()

if(MI_SEE_ASM)
  message(STATUS "Generate assembly listings (MI_SEE_ASM=ON)")
  list(APPEND mi_cflags -save-temps)
  if(CMAKE_C_COMPILER_ID MATCHES "AppleClang|Clang")
    message(STATUS "No GNU Line marker")
    list(APPEND mi_cflags -Wno-gnu-line-marker)
  endif()
endif()

if(MI_CHECK_FULL)
  message(STATUS "The MI_CHECK_FULL option is deprecated, use MI_DEBUG_FULL instead")
  set(MI_DEBUG_FULL "ON")
endif()

if (MI_SKIP_COLLECT_ON_EXIT)
  message(STATUS "Skip collecting memory on program exit (MI_SKIP_COLLECT_ON_EXIT=ON)")
  list(APPEND mi_defines MI_SKIP_COLLECT_ON_EXIT=1)
endif()

if(MI_DEBUG_FULL)
  message(STATUS "Set debug level to full internal invariant checking (MI_DEBUG_FULL=ON)")
  list(APPEND mi_defines MI_DEBUG=3)   # full invariant checking
endif()

if(MI_NO_PADDING)
  message(STATUS "Suppress any padding of heap blocks (MI_NO_PADDING=ON)")
  list(APPEND mi_defines MI_PADDING=0)
else()
  if(MI_PADDING)
    message(STATUS "Enable explicit padding of heap blocks (MI_PADDING=ON)")
    list(APPEND mi_defines MI_PADDING=1)
  endif()
endif()

if(MI_XMALLOC)
  message(STATUS "Enable abort() calls on memory allocation failure (MI_XMALLOC=ON)")
  list(APPEND mi_defines MI_XMALLOC=1)
endif()

if(MI_SHOW_ERRORS)
  message(STATUS "Enable printing of error and warning messages by default (MI_SHOW_ERRORS=ON)")
  list(APPEND mi_defines MI_SHOW_ERRORS=1)
endif()

if(MI_DEBUG_TSAN)
  if(CMAKE_C_COMPILER_ID MATCHES "Clang")
    message(STATUS "Build with thread sanitizer (MI_DEBUG_TSAN=ON)")
    list(APPEND mi_defines MI_TSAN=1)
    list(APPEND mi_cflags -fsanitize=thread -g -O1)
    list(APPEND mi_libraries -fsanitize=thread)
  else()
    message(WARNING "Can only use thread sanitizer with clang (MI_DEBUG_TSAN=ON but ignored)")
  endif()
endif()

if(MI_DEBUG_UBSAN)
  if(CMAKE_BUILD_TYPE MATCHES "Debug")
    if(CMAKE_CXX_COMPILER_ID MATCHES "Clang")
      message(STATUS "Build with undefined-behavior sanitizer (MI_DEBUG_UBSAN=ON)")
      list(APPEND mi_defines MI_UBSAN=1)
      list(APPEND mi_cflags -fsanitize=undefined -g -fno-sanitize-recover=undefined)
      list(APPEND mi_libraries -fsanitize=undefined)
      if (NOT MI_USE_CXX)
        message(STATUS "(switch to use C++ due to MI_DEBUG_UBSAN)")
        set(MI_USE_CXX "ON")
      endif()
    else()
      message(WARNING "Can only use undefined-behavior sanitizer with clang++ (MI_DEBUG_UBSAN=ON but ignored)")
    endif()
  else()
    message(WARNING "Can only use undefined-behavior sanitizer with a debug build (CMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE})")
  endif()
endif()

if(MI_USE_CXX)
  message(STATUS "Use the C++ compiler to compile (MI_USE_CXX=ON)")
  set_source_files_properties(${mi_sources} PROPERTIES LANGUAGE CXX )
  set_source_files_properties(src/static.c test/test-api.c test/test-api-fill test/test-stress PROPERTIES LANGUAGE CXX )
  if(CMAKE_CXX_COMPILER_ID MATCHES "AppleClang|Clang")
    list(APPEND mi_cflags -Wno-deprecated)
  endif()
  if(CMAKE_CXX_COMPILER_ID MATCHES "Intel" AND NOT CMAKE_CXX_COMPILER_ID MATCHES "IntelLLVM")
    list(APPEND mi_cflags -Kc++)
  endif()
endif()

if(CMAKE_SYSTEM_NAME MATCHES "Linux|Android")
  if(MI_NO_THP)
    message(STATUS "Disable transparent huge pages support (MI_NO_THP=ON)")
    list(APPEND mi_defines MI_NO_THP=1)
  endif()
endif()

if(MI_LIBC_MUSL)
  message(STATUS "Assume using musl libc (MI_LIBC_MUSL=ON)")
  list(APPEND mi_defines MI_LIBC_MUSL=1)
endif()

if(MI_WIN_USE_FLS)
  message(STATUS "Use the Fiber API to detect thread termination (deprecated) (MI_WIN_USE_FLS=ON)")
  list(APPEND mi_defines MI_WIN_USE_FLS=1)
endif()

if(MI_WIN_USE_FIXED_TLS)
  message(STATUS "Use fixed TLS slot on Windows to avoid extra tests in the malloc fast path (MI_WIN_USE_FIXED_TLS=ON)")
  list(APPEND mi_defines MI_WIN_USE_FIXED_TLS=1)
endif()

# Check /proc/cpuinfo for an SV39 MMU and limit the virtual address bits.
# (this will skip the aligned hinting in that case. Issue #939, #949)
if (EXISTS /proc/cpuinfo)
  file(STRINGS /proc/cpuinfo mi_sv39_mmu REGEX "^mmu[ \t]+:[ \t]+sv39$")
  if (mi_sv39_mmu)
    MESSAGE( STATUS "Set virtual address bits to 39 (SV39 MMU detected)" )
    list(APPEND mi_defines MI_DEFAULT_VIRTUAL_ADDRESS_BITS=39)
  endif()
endif()

# On Haiku use `-DCMAKE_INSTALL_PREFIX` instead, issue #788
# if(CMAKE_SYSTEM_NAME MATCHES "Haiku")
#   SET(CMAKE_INSTALL_LIBDIR ~/config/non-packaged/lib)
#   SET(CMAKE_INSTALL_INCLUDEDIR ~/config/non-packaged/headers)
# endif()

# Compiler flags
if(CMAKE_C_COMPILER_ID MATCHES "AppleClang|Clang|GNU" AND NOT MI_CLANG_CL)
  list(APPEND mi_cflags -Wno-unknown-pragmas -fvisibility=hidden)
  if(NOT MI_USE_CXX)
    list(APPEND mi_cflags -Wstrict-prototypes)
  endif()
  if(CMAKE_C_COMPILER_ID MATCHES "AppleClang|Clang")
    list(APPEND mi_cflags -Wno-static-in-inline)
  endif()
endif()

if(CMAKE_C_COMPILER_ID MATCHES "Intel")
  list(APPEND mi_cflags -fvisibility=hidden)
endif()

if(CMAKE_C_COMPILER_ID MATCHES "AppleClang|Clang|GNU|Intel" AND NOT CMAKE_SYSTEM_NAME MATCHES "Haiku" AND NOT MI_CLANG_CL)
  if(MI_LOCAL_DYNAMIC_TLS)
    list(APPEND mi_cflags -ftls-model=local-dynamic)
  else()
    if(MI_LIBC_MUSL)
      # with musl we use local-dynamic for the static build, see issue #644
      list(APPEND mi_cflags_static  -ftls-model=local-dynamic)
      list(APPEND mi_cflags_dynamic -ftls-model=initial-exec)
      message(STATUS "Use local dynamic TLS for the static build (since MI_LIBC_MUSL=ON)")
    else()
      list(APPEND mi_cflags -ftls-model=initial-exec)
    endif()
  endif()
  if(MI_OVERRIDE)
    list(APPEND mi_cflags -fno-builtin-malloc)
  endif()
endif()

if(CMAKE_C_COMPILER_ID MATCHES "AppleClang|Clang|GNU|Intel" AND NOT CMAKE_SYSTEM_NAME MATCHES "Haiku")
  if(MI_OPT_ARCH)
    if(APPLE AND CMAKE_C_COMPILER_ID MATCHES "AppleClang|Clang" AND CMAKE_OSX_ARCHITECTURES)   # to support multi-arch binaries (#999)
      if("arm64" IN_LIST CMAKE_OSX_ARCHITECTURES)
        list(APPEND MI_OPT_ARCH_FLAGS "-Xarch_arm64;-march=armv8.1-a")
      endif()
    elseif(MI_ARCH STREQUAL "arm64")
      set(MI_OPT_ARCH_FLAGS "-march=armv8.1-a")  # fast atomics
    endif()
  endif()
endif()

if (MSVC AND MSVC_VERSION GREATER_EQUAL 1914)
  list(APPEND mi_cflags /Zc:__cplusplus)
  if(MI_OPT_ARCH AND NOT MI_CLANG_CL)
    if(MI_ARCH STREQUAL "arm64")
      set(MI_OPT_ARCH_FLAGS "/arch:armv8.1")           # fast atomics
    endif()
  endif()
endif()

if(MINGW)
  add_definitions(-D_WIN32_WINNT=0x600)                # issue #976
endif()

if(MI_OPT_ARCH_FLAGS)
  list(APPEND mi_cflags ${MI_OPT_ARCH_FLAGS})
  message(STATUS "Architecture specific optimization is enabled (with ${MI_OPT_ARCH_FLAGS}) (MI_OPT_ARCH=ON)")
endif()

# extra needed libraries

# we prefer -l<lib> test over `find_library` as sometimes core libraries
# like `libatomic` are not on the system path (see issue #898)
function(find_link_library libname outlibname)
  check_linker_flag(C "-l${libname}" mi_has_lib${libname})
  if (mi_has_lib${libname})
    message(VERBOSE "link library: -l${libname}")
    set(${outlibname} ${libname} PARENT_SCOPE)
  else()
    find_library(MI_LIBPATH libname)
    if (MI_LIBPATH)
      message(VERBOSE "link library ${libname} at ${MI_LIBPATH}")
      set(${outlibname} ${MI_LIBPATH} PARENT_SCOPE)
    else()
      message(VERBOSE "link library not found: ${libname}")
      set(${outlibname} "" PARENT_SCOPE)
    endif()
  endif()
endfunction()

if(WIN32)
  list(APPEND mi_libraries psapi shell32 user32 advapi32 bcrypt)
else()
  find_link_library("pthread" MI_LIB_PTHREAD)
  if(MI_LIB_PTHREAD)
    list(APPEND mi_libraries "${MI_LIB_PTHREAD}")
  endif()
  find_link_library("rt" MI_LIB_RT)
  if(MI_LIB_RT)
    list(APPEND mi_libraries "${MI_LIB_RT}")
  endif()
  find_link_library("atomic" MI_LIB_ATOMIC)
  if(MI_LIB_ATOMIC)
    list(APPEND mi_libraries "${MI_LIB_ATOMIC}")
  endif()
endif()


# -----------------------------------------------------------------------------
# Install and output names
# -----------------------------------------------------------------------------

# dynamic/shared library and symlinks always go to /usr/local/lib equivalent
# we use ${CMAKE_INSTALL_BINDIR} and ${CMAKE_INSTALL_LIBDIR}.

# static libraries and object files, includes, and cmake config files
# are either installed at top level, or use versioned directories for side-by-side installation (default)
if (MI_INSTALL_TOPLEVEL)
  set(mi_install_objdir     "${CMAKE_INSTALL_LIBDIR}")
  set(mi_install_incdir     "${CMAKE_INSTALL_INCLUDEDIR}")
  set(mi_install_cmakedir   "${CMAKE_INSTALL_LIBDIR}/cmake/mimalloc")
else()
  set(mi_install_objdir     "${CMAKE_INSTALL_LIBDIR}/mimalloc-${mi_version}")       # for static library and object files
  set(mi_install_incdir     "${CMAKE_INSTALL_INCLUDEDIR}/mimalloc-${mi_version}")   # for includes
  set(mi_install_cmakedir   "${CMAKE_INSTALL_LIBDIR}/cmake/mimalloc-${mi_version}") # for cmake package info
endif()

set(mi_libname "mimalloc")
if(MI_SECURE)
  set(mi_libname "${mi_libname}-secure")
endif()
if(MI_TRACK_VALGRIND)
  set(mi_libname "${mi_libname}-valgrind")
endif()
if(MI_TRACK_ASAN)
  set(mi_libname "${mi_libname}-asan")
endif()
string(TOLOWER "${CMAKE_BUILD_TYPE}" CMAKE_BUILD_TYPE_LC)
list(APPEND mi_defines "MI_CMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE_LC}")  #todo: multi-config project needs $<CONFIG> ?
if(CMAKE_BUILD_TYPE_LC MATCHES "^(release|relwithdebinfo|minsizerel|none)$")
  list(APPEND mi_defines MI_BUILD_RELEASE)
else()
  set(mi_libname "${mi_libname}-${CMAKE_BUILD_TYPE_LC}") #append build type (e.g. -debug) if not a release version
endif()

if(MI_BUILD_SHARED)
  list(APPEND mi_build_targets "shared")
endif()
if(MI_BUILD_STATIC)
  list(APPEND mi_build_targets "static")
endif()
if(MI_BUILD_OBJECT)
  list(APPEND mi_build_targets "object")
endif()
if(MI_BUILD_TESTS)
  list(APPEND mi_build_targets "tests")
endif()

message(STATUS "")
message(STATUS "Library name     : ${mi_libname}")
message(STATUS "Version          : ${mi_version}.${mi_version_patch}")
message(STATUS "Build type       : ${CMAKE_BUILD_TYPE_LC}")
if(MI_USE_CXX)
  message(STATUS "C++ Compiler     : ${CMAKE_CXX_COMPILER}")
else()
  message(STATUS "C Compiler       : ${CMAKE_C_COMPILER}")
endif()
message(STATUS "Compiler flags   : ${mi_cflags}")
message(STATUS "Compiler defines : ${mi_defines}")
message(STATUS "Link libraries   : ${mi_libraries}")
message(STATUS "Build targets    : ${mi_build_targets}")
message(STATUS "")

# -----------------------------------------------------------------------------
# Main targets
# -----------------------------------------------------------------------------

# shared library
if(MI_BUILD_SHARED)
  add_library(mimalloc SHARED ${mi_sources})
  set_target_properties(mimalloc PROPERTIES VERSION ${mi_version} SOVERSION ${mi_version_major} OUTPUT_NAME ${mi_libname} )
  target_compile_definitions(mimalloc PRIVATE ${mi_defines} MI_SHARED_LIB MI_SHARED_LIB_EXPORT)
  target_compile_options(mimalloc PRIVATE ${mi_cflags} ${mi_cflags_dynamic})
  target_link_libraries(mimalloc PRIVATE ${mi_libraries})
  target_include_directories(mimalloc PUBLIC
      $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
      $<INSTALL_INTERFACE:${mi_install_incdir}>
  )
  install(TARGETS mimalloc EXPORT mimalloc ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR} RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR} LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
  install(EXPORT mimalloc DESTINATION ${mi_install_cmakedir})

  if(WIN32 AND NOT MINGW)
    # On windows, the import library name for the dll would clash with the static mimalloc.lib library
    # so we postfix the dll import library with `.dll.lib` (and also the .pdb debug file)
    set_property(TARGET mimalloc PROPERTY ARCHIVE_OUTPUT_NAME "${mi_libname}.dll" )
    install(FILES "$<TARGET_FILE_DIR:mimalloc>/${mi_libname}.dll.lib" DESTINATION ${CMAKE_INSTALL_LIBDIR})
    set_property(TARGET mimalloc PROPERTY PDB_NAME "${mi_libname}.dll")
    # don't try to install the pdb since it may not be generated depending on the configuration
    # install(FILES "$<TARGET_FILE_DIR:mimalloc>/${mi_libname}.dll.pdb" DESTINATION ${CMAKE_INSTALL_LIBDIR})
  endif()
  if(WIN32 AND MI_WIN_REDIRECT)
    if(MINGW)
      set_property(TARGET mimalloc PROPERTY PREFIX "")
    endif()
    # On windows, link and copy the mimalloc redirection dll too.
    if(CMAKE_GENERATOR_PLATFORM STREQUAL "arm64ec")
      set(MIMALLOC_REDIRECT_SUFFIX "-arm64ec")
    elseif(MI_ARCH STREQUAL "x64")
      set(MIMALLOC_REDIRECT_SUFFIX "")
      if(CMAKE_SYSTEM_PROCESSOR STREQUAL "ARM64")
        message(STATUS "Note: x64 code emulated on Windows for arm64 should use an arm64ec build of 'mimalloc.dll'")
        message(STATUS "      together with 'mimalloc-redirect-arm64ec.dll'. See the 'bin\\readme.md' for more information.")
      endif()
    elseif(MI_ARCH STREQUAL "x86")
      set(MIMALLOC_REDIRECT_SUFFIX "32")
    else()
      set(MIMALLOC_REDIRECT_SUFFIX "-${MI_ARCH}")  # -arm64 etc.
    endif()

    target_link_libraries(mimalloc PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/bin/mimalloc-redirect${MIMALLOC_REDIRECT_SUFFIX}.lib)  # the DLL import library
    add_custom_command(TARGET mimalloc POST_BUILD
      COMMAND "${CMAKE_COMMAND}" -E copy "${CMAKE_CURRENT_SOURCE_DIR}/bin/mimalloc-redirect${MIMALLOC_REDIRECT_SUFFIX}.dll" $<TARGET_FILE_DIR:mimalloc>
      COMMENT "Copy mimalloc-redirect${MIMALLOC_REDIRECT_SUFFIX}.dll to output directory")
    install(FILES "$<TARGET_FILE_DIR:mimalloc>/mimalloc-redirect${MIMALLOC_REDIRECT_SUFFIX}.dll" DESTINATION ${CMAKE_INSTALL_BINDIR})
  endif()
endif()


# static library
if (MI_BUILD_STATIC)
  add_library(mimalloc-static STATIC ${mi_sources})
  set_property(TARGET mimalloc-static PROPERTY OUTPUT_NAME ${mi_libname})
  set_property(TARGET mimalloc-static PROPERTY POSITION_INDEPENDENT_CODE ON)
  target_compile_definitions(mimalloc-static PRIVATE ${mi_defines} MI_STATIC_LIB)
  target_compile_options(mimalloc-static PRIVATE ${mi_cflags} ${mi_cflags_static})
  target_link_libraries(mimalloc-static PRIVATE ${mi_libraries})
  target_include_directories(mimalloc-static PUBLIC
      $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
      $<INSTALL_INTERFACE:${mi_install_incdir}>
  )
  install(TARGETS mimalloc-static EXPORT mimalloc DESTINATION ${mi_install_objdir} LIBRARY)
  install(EXPORT mimalloc DESTINATION ${mi_install_cmakedir})
endif()

# install include files
install(FILES include/mimalloc.h DESTINATION ${mi_install_incdir})
install(FILES include/mimalloc-override.h DESTINATION ${mi_install_incdir})
install(FILES include/mimalloc-new-delete.h DESTINATION ${mi_install_incdir})
install(FILES include/mimalloc-stats.h DESTINATION ${mi_install_incdir})
install(FILES cmake/mimalloc-config.cmake DESTINATION ${mi_install_cmakedir})
install(FILES cmake/mimalloc-config-version.cmake DESTINATION ${mi_install_cmakedir})


# single object file for more predictable static overriding
if (MI_BUILD_OBJECT)
  add_library(mimalloc-obj OBJECT src/static.c)
  set_property(TARGET mimalloc-obj PROPERTY POSITION_INDEPENDENT_CODE ON)
  target_compile_definitions(mimalloc-obj PRIVATE ${mi_defines})
  target_compile_options(mimalloc-obj PRIVATE ${mi_cflags} ${mi_cflags_static})
  target_include_directories(mimalloc-obj PUBLIC
      $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
      $<INSTALL_INTERFACE:${mi_install_incdir}>
  )

  # Copy the generated object file (`static.o`) to the output directory (as `mimalloc.o`)
  if(CMAKE_GENERATOR MATCHES "^Visual Studio.*$")
    set(mimalloc-obj-static "${CMAKE_CURRENT_BINARY_DIR}/mimalloc-obj.dir/$<CONFIG>/static${CMAKE_C_OUTPUT_EXTENSION}")
  else()
    set(mimalloc-obj-static "${CMAKE_CURRENT_BINARY_DIR}/CMakeFiles/mimalloc-obj.dir/src/static.c${CMAKE_C_OUTPUT_EXTENSION}")
  endif()
  set(mimalloc-obj-out "${CMAKE_CURRENT_BINARY_DIR}/${mi_libname}${CMAKE_C_OUTPUT_EXTENSION}")
  add_custom_command(OUTPUT ${mimalloc-obj-out} DEPENDS mimalloc-obj COMMAND "${CMAKE_COMMAND}" -E copy "${mimalloc-obj-static}" "${mimalloc-obj-out}")
  add_custom_target(mimalloc-obj-target ALL DEPENDS ${mimalloc-obj-out})


  # the following seems to lead to cmake warnings/errors on some systems, disable for now :-(
  # install(TARGETS mimalloc-obj EXPORT mimalloc DESTINATION ${mi_install_objdir})

  # the FILES expression can also be: $<TARGET_OBJECTS:mimalloc-obj>
  # but that fails cmake versions less than 3.10 so we leave it as is for now
  install(FILES ${mimalloc-obj-static}
          DESTINATION ${mi_install_objdir}
          RENAME ${mi_libname}${CMAKE_C_OUTPUT_EXTENSION} )
endif()


# pkg-config file support
set(mi_pc_libraries "")
foreach(item IN LISTS mi_libraries)
  if(item MATCHES " *[-].*")
    set(mi_pc_libraries "${mi_pc_libraries} ${item}")
  else()
    set(mi_pc_libraries "${mi_pc_libraries} -l${item}")
  endif()
endforeach()

include("cmake/JoinPaths.cmake")
join_paths(mi_pc_includedir "\${prefix}" "${CMAKE_INSTALL_INCLUDEDIR}")
join_paths(mi_pc_libdir "\${prefix}" "${CMAKE_INSTALL_LIBDIR}")

configure_file(mimalloc.pc.in mimalloc.pc @ONLY)
install(FILES "${CMAKE_CURRENT_BINARY_DIR}/mimalloc.pc"
        DESTINATION "${CMAKE_INSTALL_LIBDIR}/pkgconfig/")



# -----------------------------------------------------------------------------
# API surface testing
# -----------------------------------------------------------------------------

if (MI_BUILD_TESTS)
  enable_testing()

  # static link tests
  foreach(TEST_NAME api api-fill stress)
    add_executable(mimalloc-test-${TEST_NAME} test/test-${TEST_NAME}.c)
    target_compile_definitions(mimalloc-test-${TEST_NAME} PRIVATE ${mi_defines})
    target_compile_options(mimalloc-test-${TEST_NAME} PRIVATE ${mi_cflags})
    target_include_directories(mimalloc-test-${TEST_NAME} PRIVATE include)
    if(MI_BUILD_STATIC AND NOT MI_DEBUG_TSAN)
      target_link_libraries(mimalloc-test-${TEST_NAME} PRIVATE mimalloc-static ${mi_libraries})
    elseif(MI_BUILD_SHARED)
      target_link_libraries(mimalloc-test-${TEST_NAME} PRIVATE mimalloc ${mi_libraries})
    else()
      message(STATUS "cannot build TSAN tests without MI_BUILD_SHARED being enabled")
    endif()
    add_test(NAME test-${TEST_NAME} COMMAND mimalloc-test-${TEST_NAME})
  endforeach()

  # dynamic override test
  if(MI_BUILD_SHARED AND NOT (MI_TRACK_ASAN OR MI_DEBUG_TSAN OR MI_DEBUG_UBSAN))
    add_executable(mimalloc-test-stress-dynamic test/test-stress.c)
    target_compile_definitions(mimalloc-test-stress-dynamic PRIVATE ${mi_defines} "USE_STD_MALLOC=1")
    target_compile_options(mimalloc-test-stress-dynamic PRIVATE ${mi_cflags})
    target_include_directories(mimalloc-test-stress-dynamic PRIVATE include)
    if(WIN32)
      target_compile_definitions(mimalloc-test-stress-dynamic PRIVATE "MI_LINK_VERSION=1")  # link mi_version
      target_link_libraries(mimalloc-test-stress-dynamic PRIVATE mimalloc ${mi_libraries})  # link mi_version
      add_test(NAME test-stress-dynamic COMMAND ${CMAKE_COMMAND} -E env MIMALLOC_VERBOSE=1 $<TARGET_FILE:mimalloc-test-stress-dynamic>)
    else()
      if(APPLE)
        set(LD_PRELOAD "DYLD_INSERT_LIBRARIES")
      else()
        set(LD_PRELOAD "LD_PRELOAD")
      endif()
      add_test(NAME test-stress-dynamic COMMAND ${CMAKE_COMMAND} -E env MIMALLOC_VERBOSE=1 ${LD_PRELOAD}=$<TARGET_FILE:mimalloc> $<TARGET_FILE:mimalloc-test-stress-dynamic>)
    endif()
  endif()
endif()

# -----------------------------------------------------------------------------
# Set override properties
# -----------------------------------------------------------------------------
if (MI_OVERRIDE)
  if (MI_BUILD_SHARED)
    target_compile_definitions(mimalloc PRIVATE MI_MALLOC_OVERRIDE)
  endif()
  if(NOT WIN32)
    # It is only possible to override malloc on Windows when building as a DLL.
    if (MI_BUILD_STATIC)
      target_compile_definitions(mimalloc-static PRIVATE MI_MALLOC_OVERRIDE)
    endif()
    if (MI_BUILD_OBJECT)
      target_compile_definitions(mimalloc-obj PRIVATE MI_MALLOC_OVERRIDE)
    endif()
  endif()
endif()
