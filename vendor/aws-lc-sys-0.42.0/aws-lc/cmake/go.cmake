if(NOT DISABLE_GO)
find_program(GO_EXECUTABLE go)
endif()

if(NOT GO_EXECUTABLE AND NOT DISABLE_GO)
  message(FATAL_ERROR "Could not find Go")
elseif(NOT DISABLE_GO)
  execute_process(
          COMMAND ${GO_EXECUTABLE} version
          OUTPUT_VARIABLE go_version_output
          OUTPUT_STRIP_TRAILING_WHITESPACE
  )
  # Example: 'go version go1.21.3 darwin/arm64' match any number of '#.' and one '#'
  string(REGEX MATCH "([0-9]+\\.)*[0-9]+" go_version ${go_version_output})

  # This should track /go.mod and /BUILDING.md
  set(minimum_go_version "1.20")
  if(go_version VERSION_LESS minimum_go_version)
    message(FATAL_ERROR "Go compiler version must be at least ${minimum_go_version}. Found version ${go_version}")
  else()
    message(STATUS "Go compiler ${go_version} found")
  endif()
endif()

function(go_executable dest package)
  set(godeps "${PROJECT_SOURCE_DIR}/util/godeps.go")
  if(NOT CMAKE_GENERATOR STREQUAL "Ninja")
    # The DEPFILE parameter to add_custom_command only works with Ninja. Query
    # the sources at configure time. Additionally, everything depends on go.mod.
    # That affects what external packages to use.
    #
    # TODO(davidben): Starting CMake 3.20, it also works with Make. Starting
    # 3.21, it works with Visual Studio and Xcode too.
    execute_process(COMMAND ${GO_EXECUTABLE} run ${godeps} -format cmake
                            -pkg ${package}
                    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
                    OUTPUT_VARIABLE sources
                    RESULT_VARIABLE godeps_result)
    add_custom_command(OUTPUT ${dest}
                       COMMAND ${GO_EXECUTABLE} build
                               -o ${CMAKE_CURRENT_BINARY_DIR}/${dest} ${package}
                       WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
                       DEPENDS ${sources} ${PROJECT_SOURCE_DIR}/go.mod)
  else()
    # Ninja expects the target in the depfile to match the output. This is a
    # relative path from the build directory.
    string(LENGTH "${CMAKE_BINARY_DIR}" root_dir_length)
    math(EXPR root_dir_length "${root_dir_length} + 1")
    string(SUBSTRING "${CMAKE_CURRENT_BINARY_DIR}" ${root_dir_length} -1 target)
    set(target "${target}/${dest}")

    if(CMAKE_VERSION VERSION_GREATER "3.19")
      # Silences warning about CMP0116:
      # https://cmake.org/cmake/help/latest/policy/CMP0116.html
      cmake_policy(SET CMP0116 OLD)
    endif()

    if(CMAKE_VERSION VERSION_LESS "3.7")
      add_custom_command(OUTPUT ${dest}
                         COMMAND ${GO_EXECUTABLE} build
                         -o ${CMAKE_CURRENT_BINARY_DIR}/${dest} ${package}
                         WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
                         DEPENDS ${PROJECT_SOURCE_DIR}/go.mod)
    else()
      set(depfile "${CMAKE_CURRENT_BINARY_DIR}/${dest}.d")
      add_custom_command(OUTPUT ${dest}
                         COMMAND ${GO_EXECUTABLE} build
                         -o ${CMAKE_CURRENT_BINARY_DIR}/${dest} ${package}
                         COMMAND ${GO_EXECUTABLE} run ${godeps} -format depfile
                         -target ${target} -pkg ${package} -out ${depfile}
                         WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
                         DEPENDS ${godeps} ${PROJECT_SOURCE_DIR}/go.mod
                         DEPFILE ${depfile})
    endif()
  endif()
endfunction()

