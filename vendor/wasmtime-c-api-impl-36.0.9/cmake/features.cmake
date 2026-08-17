set(WASMTIME_FEATURES "--no-default-features")

option(WASMTIME_DISABLE_ALL_FEATURES
       "disable all features by default instead of enabling them"
       OFF)

macro(feature rust_name default)
  string(TOUPPER "wasmtime_feature_${rust_name}" cmake_name)
  string(REPLACE "-" "_" cmake_name ${cmake_name})
  if(${default})
    if(${WASMTIME_DISABLE_ALL_FEATURES})
      set(feature_default OFF)
    else()
      set(feature_default ON)
    endif()
  else()
    set(feature_default OFF)
  endif()

  option(${cmake_name} "enable the Cargo feature ${rust_name}" ${feature_default})

  if(${cmake_name})
    list(APPEND WASMTIME_FEATURES "--features=${rust_name}")
    message(STATUS "Enabling feature ${rust_name}")
  endif()
endmacro()

# WASMTIME_FEATURE_LIST
feature(profiling ON)
feature(wat ON)
feature(cache ON)
feature(parallel-compilation ON)
feature(wasi ON)
feature(logging ON)
feature(disable-logging OFF)
feature(coredump ON)
feature(addr2line ON)
feature(demangle ON)
feature(threads ON)
feature(gc ON)
feature(gc-drc ON)
feature(gc-null ON)
feature(async ON)
feature(cranelift ON)
feature(winch ON)
feature(debug-builtins ON)
feature(pooling-allocator ON)
feature(component-model ON)
# ... if you add a line above this be sure to change the other locations
# marked WASMTIME_FEATURE_LIST
