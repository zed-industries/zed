# bssl_shim requires socket support (BIO_new_socket, etc.)
# Skip on Generic systems (e.g., WASI) which don't have BSD sockets
if(NOT CMAKE_SYSTEM_NAME STREQUAL "Generic")
  add_executable(
    bssl_shim

    async_bio.cc
    bssl_shim.cc
    handshake_util.cc
    mock_quic_transport.cc
    packeted_bio.cc
    settings_writer.cc
    ssl_transfer.cc
    test_config.cc
    test_state.cc
  )

  target_link_libraries(bssl_shim test_support_lib ssl crypto)
  if(WIN32)
    target_link_libraries(bssl_shim ws2_32)
  endif()
  target_add_awslc_include_paths(TARGET bssl_shim SCOPE PRIVATE)

  if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    add_executable(
      handshaker

      async_bio.cc
      handshake_util.cc
      handshaker.cc
      mock_quic_transport.cc
      packeted_bio.cc
      settings_writer.cc
      test_config.cc
      test_state.cc
    )

    target_link_libraries(handshaker test_support_lib ssl crypto)
    target_add_awslc_include_paths(TARGET handshaker SCOPE PRIVATE)
  else()
    # Declare a dummy target for run_tests to depend on.
    add_custom_target(handshaker)
  endif()

endif()
