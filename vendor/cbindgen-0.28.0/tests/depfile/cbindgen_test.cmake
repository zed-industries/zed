# Common code used across the different tests

if(NOT DEFINED CBINDGEN_PATH)
    message(FATAL_ERROR "Path to cbindgen not specified")
endif()

# Promote to cache
set(CBINDGEN_PATH "${CBINDGEN_PATH}" CACHE INTERNAL "")

function(add_cbindgen_command custom_target_name header_destination)
    # Place the depfile always at the same location, so the outer test framework can locate the file easily
    set(depfile_destination     "${CMAKE_BINARY_DIR}/depfile.d")
    add_custom_command(
        OUTPUT
        "${header_destination}" "${depfile_destination}"
        COMMAND
        "${CBINDGEN_PATH}"
        --output "${header_destination}"
        --depfile "${depfile_destination}"
        ${ARGN}
        DEPFILE "${depfile_destination}"
        WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
        COMMENT "Running cbindgen"
        COMMAND_EXPAND_LISTS
    )
    add_custom_target("${custom_target_name}" ALL DEPENDS "${header_destination}")
endfunction()