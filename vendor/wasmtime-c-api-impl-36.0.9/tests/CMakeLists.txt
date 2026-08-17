include(FetchContent)
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/refs/tags/release-1.12.1.zip
)
# For Windows: Prevent overriding the parent project's compiler/linker settings
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

include(GoogleTest)

function(add_capi_test name)
  cmake_parse_arguments(PARSE_ARGV 1 arg "" "" "FILES")
  add_executable(test-${name} ${arg_FILES})
  target_link_libraries(test-${name} PRIVATE wasmtime-cpp gtest_main)
  gtest_discover_tests(test-${name}
    # GitHub Actions on Windows is pretty slow, let's give it lots more time
    # than the default 5 seconds.
    DISCOVERY_TIMEOUT 60)
endfunction()

add_capi_test(tests FILES
  simple.cc
  types.cc
  memory_type.cc
  val_type.cc
  global_type.cc
  table_type.cc
  func_type.cc
  import_type.cc
  export_type.cc
  extern_type.cc
  func.cc
  component/instantiate.cc
  component/define_module.cc
  component/lookup_func.cc
  component/call_func.cc
  component/values.cc
  error.cc
  config.cc
  wat.cc
  module.cc
  engine.cc
  trap.cc
  wasi.cc
  store.cc
  val.cc
  table.cc
  global.cc
  memory.cc
  instance.cc
  linker.cc
  wasip2.cc
)

# Create a list of all wasmtime headers with `GLOB_RECURSE`, then emit a file
# into the current binary directory which tests that if the header is included
# that the file compiles correctly.
#
# Gather everything into a list and then create a test which links all these
# compilations together. This binary doesn't actually run any tests itself but
# it does test that all headers compile in isolation at least. Not perfect but
# hopefully at least something.
cmake_path(APPEND CMAKE_CURRENT_SOURCE_DIR "../include" OUTPUT_VARIABLE header_root)
file(GLOB_RECURSE cpp_headers "${header_root}/*.h*")
foreach(header IN LISTS cpp_headers)
  cmake_path(GET header EXTENSION header_extension)
  if(header_extension STREQUAL ".h.in")
    continue()
  endif()
  cmake_path(GET header FILENAME header_filename)
  cmake_path(APPEND CMAKE_CURRENT_BINARY_DIR "header-test" "${header_filename}.cc"
    OUTPUT_VARIABLE test_filename)
  list(APPEND header_tests ${test_filename})

  cmake_path(
    RELATIVE_PATH header
    BASE_DIRECTORY ${header_root}
    OUTPUT_VARIABLE rel_header)
  file(WRITE ${test_filename} "#include <${rel_header}>")
endforeach()
add_capi_test(standalone-headers FILES ${header_tests})

# Add a custom test where two files include `wasmtime.hh` and are compiled into
# the same executable (basically makes sure any defined functions in the header
# are tagged with `inline`).
add_capi_test(test-double-include FILES double-include-a.cc double-include-b.cc)
