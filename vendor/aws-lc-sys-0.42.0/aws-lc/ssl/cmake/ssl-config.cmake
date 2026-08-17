include(CMakeFindDependencyMacro)

find_dependency(crypto)

# Allow static or shared lib to be used.
# If both are installed, choose based on BUILD_SHARED_LIBS.
if (BUILD_SHARED_LIBS)
    if (EXISTS "${CMAKE_CURRENT_LIST_DIR}/shared/ssl-targets.cmake")
        include(${CMAKE_CURRENT_LIST_DIR}/shared/ssl-targets.cmake)
    else()
        include(${CMAKE_CURRENT_LIST_DIR}/static/ssl-targets.cmake)
    endif()
else()
    if (EXISTS "${CMAKE_CURRENT_LIST_DIR}/static/ssl-targets.cmake")
        include(${CMAKE_CURRENT_LIST_DIR}/static/ssl-targets.cmake)
    else()
        include(${CMAKE_CURRENT_LIST_DIR}/shared/ssl-targets.cmake)
    endif()
endif()
