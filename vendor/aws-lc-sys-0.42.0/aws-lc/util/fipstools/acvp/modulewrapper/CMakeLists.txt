if(FIPS)
  add_executable(
    modulewrapper

    main.cc
    modulewrapper.cc
  )
  target_link_libraries(modulewrapper crypto)
  target_add_awslc_include_paths(TARGET modulewrapper SCOPE PRIVATE)
endif()
