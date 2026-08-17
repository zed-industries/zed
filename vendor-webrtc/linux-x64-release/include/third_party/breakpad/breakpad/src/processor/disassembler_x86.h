// Copyright 2010 Google LLC
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//     * Neither the name of Google LLC nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

// disassembler_x86.h: Basic x86 bytecode disassembler
//
// Provides a simple disassembler which wraps libdisasm. This allows simple
// tests to be run against bytecode to test for various properties.
//
// Author: Cris Neckar

#ifndef GOOGLE_BREAKPAD_PROCESSOR_DISASSEMBLER_X86_H_
#define GOOGLE_BREAKPAD_PROCESSOR_DISASSEMBLER_X86_H_

#include <sys/types.h>

#include "google_breakpad/common/breakpad_types.h"

namespace libdis {
#include "third_party/libdisasm/libdis.h"
}

namespace google_breakpad {

enum {
  DISX86_NONE =                 0x0,
  DISX86_BAD_BRANCH_TARGET =    0x1,
  DISX86_BAD_ARGUMENT_PASSED =  0x2,
  DISX86_BAD_WRITE =            0x4,
  DISX86_BAD_BLOCK_WRITE =      0x8,
  DISX86_BAD_READ =             0x10,
  DISX86_BAD_BLOCK_READ =       0x20,
  DISX86_BAD_COMPARISON =       0x40
};

class DisassemblerX86 {
  public:
    // TODO(cdn): Modify this class to take a MemoryRegion instead of just
    // a raw buffer. This will make it easier to use this on arbitrary
    // minidumps without first copying out the code segment.
    DisassemblerX86(const uint8_t* bytecode, uint32_t, uint32_t);
    ~DisassemblerX86();

    // This walks to the next instruction in the memory region and
    // sets flags based on the type of instruction and previous state
    // including any registers marked as bad through setBadRead()
    // or setBadWrite(). This method can be called in a loop to
    // disassemble until the end of a region.
    uint32_t NextInstruction();

    // Indicates whether the current disassembled instruction was valid.
    bool currentInstructionValid() { return instr_valid_; }

    // Returns the current instruction as defined in libdis.h,
    // or NULL if the current instruction is not valid.
    const libdis::x86_insn_t* currentInstruction() {
      return instr_valid_ ? &current_instr_ : nullptr;
    }

    // Returns the type of the current instruction as defined in libdis.h.
    libdis::x86_insn_group currentInstructionGroup() {
      return current_instr_.group;
    }

    // Indicates whether a return instruction has been encountered.
    bool endOfBlock() { return end_of_block_; }

    // The flags set so far for the disassembly.
    uint16_t flags() { return flags_; }

    // This sets an indicator that the register used to determine
    // src or dest for the current instruction is tainted. These can
    // be used after examining the current instruction to indicate,
    // for example that a bad read or write occurred and the pointer
    // stored in the register is currently invalid.
    bool setBadRead();
    bool setBadWrite();

  protected:
    const uint8_t* bytecode_;
    uint32_t size_;
    uint32_t virtual_address_;
    uint32_t current_byte_offset_;
    uint32_t current_inst_offset_;

    bool instr_valid_;
    libdis::x86_insn_t current_instr_;

    // TODO(cdn): Maybe also track an expression's index register.
    // ex: mov eax, [ebx + ecx]; ebx is base, ecx is index.
    bool register_valid_;
    libdis::x86_reg_t bad_register_;

    bool pushed_bad_value_;
    bool end_of_block_;

    uint16_t flags_;
};

}  // namespace google_breakpad

#endif  // GOOGLE_BREAKPAD_PROCESSOR_DISASSEMBLER_X86_H_
