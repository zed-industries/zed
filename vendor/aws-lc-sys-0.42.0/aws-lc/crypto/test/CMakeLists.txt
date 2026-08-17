add_library(
  test_support_lib

  STATIC

  abi_test.cc
  file_test.cc
  malloc.cc
  test_util.cc
  wycheproof_util.cc
)

if (LIBUNWIND_FOUND)
  target_compile_options(test_support_lib PRIVATE ${LIBUNWIND_CFLAGS_OTHER})
  target_include_directories(test_support_lib PRIVATE ${LIBUNWIND_INCLUDE_DIRS})
  target_link_libraries(test_support_lib ${LIBUNWIND_LDFLAGS})
endif()
if(WIN32)
  target_link_libraries(test_support_lib dbghelp)
endif()
target_link_libraries(test_support_lib boringssl_gtest crypto)
target_add_awslc_include_paths(TARGET test_support_lib SCOPE PRIVATE)

# For WASI (Generic systems), use OBJECT library instead of STATIC to ensure
# the main() symbol is directly included in executables. Static libraries on
# WASI may not have their 'main' symbol pulled in, resulting in
# "undefined_weak:main" runtime errors.
if(CMAKE_SYSTEM_NAME STREQUAL "Generic")
  add_library(boringssl_gtest_main OBJECT gtest_main.cc)
else()
  add_library(boringssl_gtest_main STATIC gtest_main.cc)
endif()
target_link_libraries(boringssl_gtest_main boringssl_gtest crypto test_support_lib)
