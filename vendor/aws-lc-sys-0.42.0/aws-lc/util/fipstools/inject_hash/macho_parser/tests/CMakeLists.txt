if(FIPS AND APPLE)
  add_executable(
    test_macho_parser

    macho_tests.cc
    ../macho_parser.c
  )

  target_link_libraries(
    test_macho_parser

    test_support_lib
    boringssl_gtest_main
  )
endif()
