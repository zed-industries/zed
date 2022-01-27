#!/bin/bash

export GPUTOOLS_LOAD_GTMTLCAPTURE=1
export DYLD_LIBRARY_PATH="/usr/lib/system/introspection"
export METAL_LOAD_INTERPOSER=1
export DYLD_INSERT_LIBRARIES="/usr/lib/libMTLCapture.dylib"
export DYMTL_TOOLS_DYLIB_PATH="/usr/lib/libMTLCapture.dylib"
export METAL_DEVICE_WRAPPER_TYPE=1
export GPUProfilerEnabled="YES"
export METAL_DEBUG_ERROR_MODE=0
export LD_LIBRARY_PATH="/Applications/Xcode.app/Contents/Developer/../SharedFrameworks/"

cargo run $@
