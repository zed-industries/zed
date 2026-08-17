// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_DEBUG_DWARF_LINE_NO_H_
#define BASE_DEBUG_DWARF_LINE_NO_H_

#include <cstddef>
#include <cstdint>

namespace base {
namespace debug {

// Finds the compile unit offset in .debug_info for each frame in `trace`.
//
// Expects `trace` and `cu_offsets` to be `num_frames` in size. If a frame
// cannot be found, the corresponding value stored in `cu_offsets` is 0.
void GetDwarfCompileUnitOffsets(const void* const* trace,
                                uint64_t* cu_offsets,
                                size_t num_frames);

// Formats the source file, line number and column for `pc` and into `out`.
//
// The `cu_offsets` is the offset in the .debug_info section for the compile
// unit or partial unit DIE corresponding to the `pc`. It can be found using
// GetDwarfCompileUnitOffsets() and must not be 0.
//
// Example:
//   ../../base/debug/stack_trace_unittest.cc:120,16
//
// This means `pc` was from line 120, column 16, of stack_trace_unittest.cc.
bool GetDwarfSourceLineNumber(const void* pc,
                              uint64_t cu_offsets,
                              char* out,
                              size_t out_size);
}  // namespace debug
}  // namespace base

#endif  // BASE_DEBUG_DWARF_LINE_NO_H_
