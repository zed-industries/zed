// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_CHROME_UNWIND_INFO_ANDROID_32_H_
#define BASE_PROFILER_CHROME_UNWIND_INFO_ANDROID_32_H_

#include <stdint.h>

#include "base/base_export.h"
#include "base/containers/span.h"
#include "base/memory/raw_span.h"

namespace base {

// Represents each entry in the function table (i.e. the second level of the
// function address table).
struct FunctionTableEntry {
  // The offset into the 128kb page containing this function. Indexed by bits
  // 1-16 of the pc offset from the start of the text section.
  uint16_t function_start_address_page_instruction_offset;

  // The byte index of the first offset for the function in the function
  // offset table.
  uint16_t function_offset_table_byte_index;
};

// The header at the start of the unwind info resource, with offsets/sizes for
// the tables contained within the resource.
//
// The unwind info provides 4 tables which can translate an instruction address
// to a set of unwind instructions to unwind the function frame the instruction
// belongs to.
//
// `page_table` and `function_table` together locates which function the
// instruction address belongs to given an instruction address.
//
// `function_offset_table` and `unwind_instruction_table` together locates
// which sets of unwind instructions to execute given the function info
// obtained from `page_table` and `function_table`, and the offset between the
// instruction address and function start address.
//
// Design Doc:
// https://docs.google.com/document/d/1IYTmGCJZoiQ242xPUZX1fATD6ivsjU1TAt_fPv74ocs/edit?usp=sharing
struct BASE_EXPORT ChromeUnwindInfoAndroid32Header {
  // The offset in bytes from the start of the unwind info resource to the
  // page table (i.e. the first level of the function address table). The page
  // table represents discrete 128kb 'pages' of memory in the text section,
  // each of which contains functions. The page table is indexed by bits 17
  // and greater of the pc offset from the start of the text section.
  // Indexing into page_table produces an index of function_table.
  uint32_t page_table_byte_offset;
  uint32_t page_table_entries;

  // The offset in bytes from the start of the unwind info resource to the
  // function table (i.e. the second level of the function address table). The
  // function table represents the individual functions within a 128kb page.
  // Each function is represented as a `FunctionTableEntry`. The relevant entry
  // for a pc offset from the start of the text section is the one with the
  // largest function_start_address_page_instruction_offset <= (pc_offset >> 1)
  // & 0xffff.
  uint32_t function_table_byte_offset;
  uint32_t function_table_entries;

  // The offset in bytes from the start of the unwind info resource to the
  // function offset table. The function offset table represents the pc
  // offsets from the start of each function along with indices into the
  // unwind instructions for the offsets. The pc offsets and unwind indices
  // are represented as (ULEB128, ULEB128) pairs in decreasing order of
  // offset. Distinct sequences of (offset, index) pairs are concatenated in
  // the table.
  uint32_t function_offset_table_byte_offset;
  uint32_t function_offset_table_size_in_bytes;

  // The offset in bytes from the start of the unwind info resource to the
  // unwind instruction table. The unwind instruction table represents
  // distinct sequences of ARM compact unwind instructions[1] used across all
  // functions in Chrome. The compact unwind instruction is a byte-oriented
  // variable length encoding so is indexed by byte position.
  // 1. See Exception handling ABI for the ARM architecture ABI, §9.3.
  // https://developer.arm.com/documentation/ihi0038/b.
  uint32_t unwind_instruction_table_byte_offset;
  uint32_t unwind_instruction_table_size_in_bytes;
};

struct BASE_EXPORT ChromeUnwindInfoAndroid32 {
  ChromeUnwindInfoAndroid32(span<const uint8_t> unwind_instruction_table,
                            span<const uint8_t> function_offset_table,
                            span<const FunctionTableEntry> function_table,
                            span<const uint32_t> page_table);
  ~ChromeUnwindInfoAndroid32();
  ChromeUnwindInfoAndroid32(const ChromeUnwindInfoAndroid32& other);
  ChromeUnwindInfoAndroid32& operator=(const ChromeUnwindInfoAndroid32& other);

  ChromeUnwindInfoAndroid32(ChromeUnwindInfoAndroid32&& other);
  ChromeUnwindInfoAndroid32& operator=(ChromeUnwindInfoAndroid32&& other);

  // Unwind instruction table is expected to have following memory layout:
  // +-----------------------------+
  // | <--1 byte--->               |
  // +-----------------------------+
  // | pop {r4, r5, lr}            | <- FUNC1 offset 10
  // +-----------------------------+
  // | add sp, 16                  | <- FUNC1 offset 4
  // +-----------------------------+
  // | mov pc, lr                  | <- FUNC1 offset 0 (COMPLETE)
  // +-----------------------------+
  // | pop {r4, r11} [byte 1/2]    | <- FUNC2 offset 8
  // +-----------------------------+
  // | pop {r4, r11} [byte 2/2]    |
  // +-----------------------------+
  // | ...                         |
  // +-----------------------------+
  // Because we are unwinding the function, the next unwind instruction to
  // execute always has smaller function offset.
  // The function offsets are often discontinuous as not all instructions in
  // the function have corresponding unwind instructions.
  //
  // See Exception handling ABI for the ARM architecture ABI, §9.3.
  // https://developer.arm.com/documentation/ihi0038/b.
  // for details in unwind instruction encoding.
  // Only following instruction encodings are handled:
  // - 00xxxxxx
  // - 01xxxxxx
  // - 1000iiii iiiiiiii
  // - 1001nnnn
  // - 10100nnn
  // - 10101nnn
  // - 10110000
  // - 10110010 uleb128
  raw_span<const uint8_t> unwind_instruction_table;

  // Function offset table is expected to have following memory layout:
  // +---------------------+---------------------+
  // | <-----ULEB128-----> | <-----ULEB128-----> |
  // +---------------------+---------------------+
  // | Offset              | Unwind Index        |
  // +---------------------+---------------------+-----
  // | 8                   | XXX                 |  |
  // +---------------------+---------------------+  |
  // | 3                   | YYY                 |Function 1
  // +---------------------+---------------------+  |
  // | 0                   | ZZZ                 |  |
  // +---------------------+---------------------+-----
  // | 5                   | AAA                 |  |
  // +---------------------+---------------------+Function 2
  // | 0                   | BBB                 |  |
  // +---------------------+---------------------+-----
  // | ...                 | ....                |
  // +---------------------+---------------------+
  // The function offset table contains [offset, unwind index] pairs, where
  // - offset: offset from function start address of an instruction that affects
  //           the unwind state, measured in two-byte instructions.
  // - unwind index: unwind instruction location in unwind instruction table.
  //
  // Note:
  // - Each function always ends at 0 offset, which corresponds to a terminal
  //   instruction in unwind instruction table.
  // - Within each function section, offset strictly decreases. By doing so,
  //   each function's own terminal instruction will serve as termination
  //   condition when searching in the table.
  raw_span<const uint8_t> function_offset_table;

  // The function table represents the individual functions within a 128kb page.
  // The relevant entry for an instruction offset from the start of the text
  // section is the one with the largest function_start_address_page_offset <=
  // instruction_byte_offset_from_text_section_start.
  //
  // Function table is expected to have following memory layout:
  // +--------------------+--------------------+
  // | <-----2 byte-----> | <-----2 byte-----> |
  // +--------------------+--------------------+
  // | Page Offset        | Offset Table Index |
  // +--------------------+--------------------+-----
  // | 10                 | XXX                |  |
  // +--------------------+--------------------+  |
  // | ...                | ...                |Page 0x100
  // +--------------------+--------------------+  |
  // | 65500              | ZZZ                |  |
  // +--------------------+--------------------+-----
  // | 200                | AAA                |  |
  // +--------------------+--------------------+  |
  // | ...                | ...                |Page 0x101
  // +--------------------+--------------------+  |
  // | 65535              | BBB                |  |
  // +--------------------+--------------------+-----
  //
  // Note:
  // - Within each page, `Page Offset` strictly increases.
  // - Each `FunctionTableEntry` represents a function where the start
  // address falls into the page memory address range.
  raw_span<const FunctionTableEntry> function_table;

  // The page table represents discrete 128kb 'pages' of memory in the text
  // section, each of which contains functions. The page table is indexed by
  // bits 17 and greater of the pc offset from the start of the text section.
  // Indexing into page_table produces an index of function_table.
  //
  // The page table is expected to have following memory layout:
  // +----------------+
  // | <-- 4 byte --> |
  // +----------------+
  // | 0              |
  // +----------------+
  // | 18             |
  // +----------------+
  // | 18             |
  // +----------------+
  // | 80             |
  // +----------------+
  // | ...            |
  // +----------------+
  // Note:
  // - The page start instructions in page table non-strictly increases, i.e
  // empty page is allowed.
  raw_span<const uint32_t> page_table;
};

// Creates `ChromeUnwindInfoAndroid32` struct based on binary `data` assuming
// `data` starts with `ChromeUnwindInfoAndroid32Header`.
BASE_EXPORT ChromeUnwindInfoAndroid32
CreateChromeUnwindInfoAndroid32(span<const uint8_t> data);

}  // namespace base

#endif  // BASE_PROFILER_CHROME_UNWIND_INFO_ANDROID_32_H_
