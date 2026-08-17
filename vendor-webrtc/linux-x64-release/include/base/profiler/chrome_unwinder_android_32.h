// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_CHROME_UNWINDER_ANDROID_32_H_
#define BASE_PROFILER_CHROME_UNWINDER_ANDROID_32_H_

#include <stdint.h>

#include <optional>

#include "base/base_export.h"
#include "base/containers/span.h"
#include "base/profiler/chrome_unwind_info_android_32.h"
#include "base/profiler/module_cache.h"
#include "base/profiler/register_context.h"
#include "base/profiler/unwinder.h"

namespace base {

// Chrome unwinder implementation for Android 32-bit ARM, using
// ChromeUnwindInfoAndroid32, a separate binary resource.
class BASE_EXPORT ChromeUnwinderAndroid32 : public Unwinder {
 public:
  ChromeUnwinderAndroid32(const ChromeUnwindInfoAndroid32& unwind_info,
                          uintptr_t chrome_module_base_address,
                          uintptr_t text_section_start_address);
  ChromeUnwinderAndroid32(const ChromeUnwinderAndroid32&) = delete;
  ChromeUnwinderAndroid32& operator=(const ChromeUnwinderAndroid32&) = delete;

  // Unwinder:
  bool CanUnwindFrom(const Frame& current_frame) const override;
  UnwindResult TryUnwind(UnwinderStateCapture* capture_state,
                         RegisterContext* thread_context,
                         uintptr_t stack_top,
                         std::vector<Frame>* stack) override;

 private:
  const ChromeUnwindInfoAndroid32 unwind_info_;
  const uintptr_t chrome_module_base_address_;
  const uintptr_t text_section_start_address_;
};

// Following functions are exposed for testing purpose only.
struct FunctionTableEntry;

enum class UnwindInstructionResult {
  kCompleted,           // Signals the end of unwind process.
  kInstructionPending,  // Continues to unwind next instruction.
  kAborted,             // Unable to unwind.
};

// Execute a single unwind instruction on the given thread_context, and moves
// `instruction` to point to next instruction right after the executed
// instruction.
//
// Arguments:
//   instruction: The pointer to the instruction to execute. This pointer will
//                be advanced by the size of the instruction executed after the
//                function call.
//   pc_was_updated: Set to true if the pc was updated by the instruction
//                   execution. Used to decide whether to copy lr to pc on
//                   COMPLETE.
//   thread_context: The thread_context the instruction operates on.
BASE_EXPORT UnwindInstructionResult
ExecuteUnwindInstruction(const uint8_t*& instruction,
                         bool& pc_was_updated,
                         RegisterContext* thread_context);

// Represents an index that can locate a specific entry on function offset
// table.
struct FunctionOffsetTableIndex {
  // Number of 2-byte instructions between the instruction of interest and
  // function start address.
  int instruction_offset_from_function_start;
  // The byte index of the first offset for the function in the function
  // offset table.
  uint16_t function_offset_table_byte_index;
};

// Given function offset table entry, finds the first unwind instruction to
// execute in unwind instruction table.
//
// Arguments:
//  function_offset_table_entry: An entry in function offset table. See
//    `ChromeUnwindInfoAndroid32::function_offset_table` for details.
//  instruction_offset_from_function_start: Number of 2-byte instructions
//    between the instruction of interest and function start address.
//
// Returns:
//   The index of the first unwind instruction to execute in
//   `ChromeUnwindInfoAndroid32::unwind_instruction_table`.
BASE_EXPORT uintptr_t
GetFirstUnwindInstructionIndexFromFunctionOffsetTableEntry(
    const uint8_t* function_offset_table_entry,
    int instruction_offset_from_function_start);

// Given an instruction_byte_offset_from_text_section_start, finds the
// corresponding `FunctionOffsetTableIndex`.
//
// Arguments:
//  page_start_instructions: A list of page_numbers. See
//    `ChromeUnwindInfoAndroid32::page_table` for details.
//  function_offsets_table_indices: A list of `FunctionTableEntry`. See
//    `ChromeUnwindInfoAndroid32::function_table` for details.
//  instruction_byte_offset_from_text_section_start: The distance in bytes
//    between the instruction of interest and text section start.
BASE_EXPORT const std::optional<FunctionOffsetTableIndex>
GetFunctionTableIndexFromInstructionOffset(
    span<const uint32_t> page_start_instructions,
    span<const FunctionTableEntry> function_offset_table_indices,
    uint32_t instruction_byte_offset_from_text_section_start);

}  // namespace base

#endif  // BASE_PROFILER_CHROME_UNWINDER_ANDROID_32_H_
