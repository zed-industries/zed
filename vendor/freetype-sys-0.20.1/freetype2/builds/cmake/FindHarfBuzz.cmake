# Copyright (c) 2012, Intel Corporation
# Copyright (c) 2019 Sony Interactive Entertainment Inc.
#
# Redistribution and use in source and binary forms, with or without
# modification, are permitted provided that the following conditions are met:
#
# * Redistributions of source code must retain the above copyright notice, this
#   list of conditions and the following disclaimer.
# * Redistributions in binary form must reproduce the above copyright notice,
#   this list of conditions and the following disclaimer in the documentation
#   and/or other materials provided with the distribution.
# * Neither the name of Intel Corporation nor the names of its contributors may
#   be used to endorse or promote products derived from this software without
#   specific prior written permission.
#
# THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
# AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
# IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
# ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
# LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
# CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
# SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
# INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
# CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
# ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
# POSSIBILITY OF SUCH DAMAGE.
#
# Try to find Harfbuzz include and library directories.
#
# After successful discovery, this will set for inclusion where needed:
# HarfBuzz_INCLUDE_DIRS - containg the HarfBuzz headers
# HarfBuzz_LIBRARIES - containg the HarfBuzz library

#[=======================================================================[.rst:
FindHarfBuzz
--------------

Find HarfBuzz headers and libraries.

Imported Targets
^^^^^^^^^^^^^^^^

``HarfBuzz::HarfBuzz``
  The HarfBuzz library, if found.

``HarfBuzz::ICU``
  The HarfBuzz ICU library, if found.

Result Variables
^^^^^^^^^^^^^^^^

This will define the following variables in your project:

``HarfBuzz_FOUND``
  true if (the requested version of) HarfBuzz is available.
``HarfBuzz_VERSION``
  the version of HarfBuzz.
``HarfBuzz_LIBRARIES``
  the libraries to link against to use HarfBuzz.
``HarfBuzz_INCLUDE_DIRS``
  where to find the HarfBuzz headers.
``HarfBuzz_COMPILE_OPTIONS``
  this should be passed to target_compile_options(), if the
  target is not used for linking

#]=======================================================================]

find_package(PkgConfig QUIET)
pkg_check_modules(PC_HARFBUZZ QUIET harfbuzz)
set(HarfBuzz_COMPILE_OPTIONS ${PC_HARFBUZZ_CFLAGS_OTHER})
set(HarfBuzz_VERSION ${PC_HARFBUZZ_CFLAGS_VERSION})

find_path(HarfBuzz_INCLUDE_DIR
    NAMES hb.h
    HINTS ${PC_HARFBUZZ_INCLUDEDIR} ${PC_HARFBUZZ_INCLUDE_DIRS}
    PATH_SUFFIXES harfbuzz
)

find_library(HarfBuzz_LIBRARY
    NAMES ${HarfBuzz_NAMES} harfbuzz
    HINTS ${PC_HARFBUZZ_LIBDIR} ${PC_HARFBUZZ_LIBRARY_DIRS}
)

if (HarfBuzz_INCLUDE_DIR AND NOT HarfBuzz_VERSION)
    if (EXISTS "${HarfBuzz_INCLUDE_DIR}/hb-version.h")
        file(READ "${HarfBuzz_INCLUDE_DIR}/hb-version.h" _harfbuzz_version_content)

        string(REGEX MATCH "#define +HB_VERSION_STRING +\"([0-9]+\\.[0-9]+\\.[0-9]+)\"" _dummy "${_harfbuzz_version_content}")
        set(HarfBuzz_VERSION "${CMAKE_MATCH_1}")
    endif ()
endif ()

if ("${HarfBuzz_FIND_VERSION}" VERSION_GREATER "${HarfBuzz_VERSION}")
  if (HarfBuzz_FIND_REQUIRED)
    message(FATAL_ERROR
      "Required version (" ${HarfBuzz_FIND_VERSION} ")"
      " is higher than found version (" ${HarfBuzz_VERSION} ")")
  else ()
    message(WARNING
      "Required version (" ${HarfBuzz_FIND_VERSION} ")"
      " is higher than found version (" ${HarfBuzz_VERSION} ")")
    unset(HarfBuzz_VERSION)
    unset(HarfBuzz_INCLUDE_DIRS)
    unset(HarfBuzz_LIBRARIES)
    return ()
  endif ()
endif ()

# Find components
if (HarfBuzz_INCLUDE_DIR AND HarfBuzz_LIBRARY)
    set(_HarfBuzz_REQUIRED_LIBS_FOUND ON)
    set(HarfBuzz_LIBS_FOUND "HarfBuzz (required): ${HarfBuzz_LIBRARY}")
else ()
    set(_HarfBuzz_REQUIRED_LIBS_FOUND OFF)
    set(HarfBuzz_LIBS_NOT_FOUND "HarfBuzz (required)")
endif ()

if (NOT CMAKE_VERSION VERSION_LESS 3.3)
  if ("ICU" IN_LIST HarfBuzz_FIND_COMPONENTS)
      pkg_check_modules(PC_HARFBUZZ_ICU QUIET harfbuzz-icu)
      set(HarfBuzz_ICU_COMPILE_OPTIONS ${PC_HARFBUZZ_ICU_CFLAGS_OTHER})

      find_path(HarfBuzz_ICU_INCLUDE_DIR
          NAMES hb-icu.h
          HINTS ${PC_HARFBUZZ_ICU_INCLUDEDIR} ${PC_HARFBUZZ_ICU_INCLUDE_DIRS}
          PATH_SUFFIXES harfbuzz
      )

      find_library(HarfBuzz_ICU_LIBRARY
          NAMES ${HarfBuzz_ICU_NAMES} harfbuzz-icu
          HINTS ${PC_HARFBUZZ_ICU_LIBDIR} ${PC_HARFBUZZ_ICU_LIBRARY_DIRS}
      )

      if (HarfBuzz_ICU_LIBRARY)
          if (HarfBuzz_FIND_REQUIRED_ICU)
              list(APPEND HarfBuzz_LIBS_FOUND "ICU (required): ${HarfBuzz_ICU_LIBRARY}")
          else ()
            list(APPEND HarfBuzz_LIBS_FOUND "ICU (optional): ${HarfBuzz_ICU_LIBRARY}")
          endif ()
      else ()
          if (HarfBuzz_FIND_REQUIRED_ICU)
            set(_HarfBuzz_REQUIRED_LIBS_FOUND OFF)
            list(APPEND HarfBuzz_LIBS_NOT_FOUND "ICU (required)")
          else ()
            list(APPEND HarfBuzz_LIBS_NOT_FOUND "ICU (optional)")
          endif ()
      endif ()
  endif ()
endif ()

if (NOT HarfBuzz_FIND_QUIETLY)
    if (HarfBuzz_LIBS_FOUND)
        message(STATUS "Found the following HarfBuzz libraries:")
        foreach (found ${HarfBuzz_LIBS_FOUND})
            message(STATUS " ${found}")
        endforeach ()
    endif ()
    if (HarfBuzz_LIBS_NOT_FOUND)
        message(STATUS "The following HarfBuzz libraries were not found:")
        foreach (found ${HarfBuzz_LIBS_NOT_FOUND})
            message(STATUS " ${found}")
        endforeach ()
    endif ()
endif ()

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(HarfBuzz
    FOUND_VAR HarfBuzz_FOUND
    REQUIRED_VARS HarfBuzz_INCLUDE_DIR HarfBuzz_LIBRARY _HarfBuzz_REQUIRED_LIBS_FOUND
    VERSION_VAR HarfBuzz_VERSION
)

if (NOT CMAKE_VERSION VERSION_LESS 3.1)
  if (HarfBuzz_LIBRARY AND NOT TARGET HarfBuzz::HarfBuzz)
      add_library(HarfBuzz::HarfBuzz UNKNOWN IMPORTED GLOBAL)
      set_target_properties(HarfBuzz::HarfBuzz PROPERTIES
          IMPORTED_LOCATION "${HarfBuzz_LIBRARY}"
          INTERFACE_COMPILE_OPTIONS "${HarfBuzz_COMPILE_OPTIONS}"
          INTERFACE_INCLUDE_DIRECTORIES "${HarfBuzz_INCLUDE_DIR}"
      )
  endif ()

  if (HarfBuzz_ICU_LIBRARY AND NOT TARGET HarfBuzz::ICU)
      add_library(HarfBuzz::ICU UNKNOWN IMPORTED GLOBAL)
      set_target_properties(HarfBuzz::ICU PROPERTIES
          IMPORTED_LOCATION "${HarfBuzz_ICU_LIBRARY}"
          INTERFACE_COMPILE_OPTIONS "${HarfBuzz_ICU_COMPILE_OPTIONS}"
          INTERFACE_INCLUDE_DIRECTORIES "${HarfBuzz_ICU_INCLUDE_DIR}"
      )
  endif ()
endif ()

mark_as_advanced(
    HarfBuzz_INCLUDE_DIR
    HarfBuzz_ICU_INCLUDE_DIR
    HarfBuzz_LIBRARY
    HarfBuzz_ICU_LIBRARY
)

if (HarfBuzz_FOUND)
   set(HarfBuzz_LIBRARIES ${HarfBuzz_LIBRARY} ${HarfBuzz_ICU_LIBRARY})
   set(HarfBuzz_INCLUDE_DIRS ${HarfBuzz_INCLUDE_DIR} ${HarfBuzz_ICU_INCLUDE_DIR})
endif ()
