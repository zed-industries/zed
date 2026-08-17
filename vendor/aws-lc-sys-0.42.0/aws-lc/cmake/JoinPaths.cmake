# This module provides function for joining paths
# known from most languages
#
# SPDX-License-Identifier: (MIT OR CC0-1.0)
# Copyright 2020 Jan Tojnar
# https://github.com/jtojnar/cmake-snips
#
# Modelled after Python’s os.path.join
# https://docs.python.org/3.7/library/os.path.html#os.path.join
function(join_paths joined_path first_path_segment)
  set(temp_path "${first_path_segment}")
  foreach(current_segment IN LISTS ARGN)
    if(NOT ("${current_segment}" STREQUAL ""))
      if(IS_ABSOLUTE "${current_segment}")
        set(temp_path "${current_segment}")
      else()
        set(temp_path "${temp_path}/${current_segment}")
      endif()
    endif()
  endforeach()
  file(TO_NATIVE_PATH "${temp_path}" temp_path)
  set(${joined_path} "${temp_path}" PARENT_SCOPE)
endfunction()
