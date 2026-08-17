// Copyright (c) 2022, Google LLC
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

// disassembler_objdump.h: Disassembler that invokes objdump for disassembly.
//
// Author: Mark Brand

#ifndef GOOGLE_BREAKPAD_PROCESSOR_DISASSEMBLER_OBJDUMP_H_
#define GOOGLE_BREAKPAD_PROCESSOR_DISASSEMBLER_OBJDUMP_H_

#include <string>

#include "common/using_std_string.h"
#include "google_breakpad/common/breakpad_types.h"
#include "google_breakpad/processor/dump_context.h"
#include "google_breakpad/processor/memory_region.h"

namespace google_breakpad {

// Uses objdump to disassemble a single instruction.
//
// Currently supports disassembly for x86 and x86_64 on linux hosts only; on
// unsupported platform or for unsupported architectures disassembly will fail.
//
// If disassembly is successful, then this allows extracting the instruction
// opcode, source and destination operands, and computing the source and
// destination addresses for instructions that operate on memory.
//
// Example:
//   DisassemblerObjdump disassembler(context->GetContextCPU(), memory_region,
//                                    instruction_ptr);
//   if (disassembler.IsValid()) {
//     uint64_t src_address = 0;
//     std::cerr << disassembler.operation() << " " << disassembler.src()
//               << ", " << disassembler.dest() << std::endl;
//     if (disassembler.CalculateSrcAddress(*context, src_address)) {
//       std::cerr << "[src_address = " << std::hex << src_address << "]\n";
//     }
//   }
class DisassemblerObjdump {
 public:
  // Construct an ObjdumpDisassembler for the provided `cpu` type, where this is
  // one of MD_CONTEXT_X86 or MD_CONTEXT_AMD64. Provided that `address` is
  // within `memory_region`, and the memory referenced is a valid instruction,
  // this will then be initialized with the disassembly for that instruction.
  DisassemblerObjdump(uint32_t cpu,
                      const MemoryRegion* memory_region,
                      uint64_t address);
  ~DisassemblerObjdump() = default;

  // If the source operand of the instruction is a memory operand, compute the
  // address referred to by the operand, and store this in `address`. On success
  // returns true, otherwise (if computation fails, or if the source operand is
  // not a memory operand) returns false and sets `address` to 0.
  bool CalculateSrcAddress(const DumpContext& context, uint64_t& address);

  // If the destination operand of the instruction is a memory operand, compute
  // the address referred to by the operand, and store this in `address`. On
  // success returns true, otherwise (if computation fails, or if the source
  // operand is not a memory operand) returns false and sets `address` to 0.
  bool CalculateDestAddress(const DumpContext& context, uint64_t& address);

  // If the instruction was disassembled successfully, this will be true.
  bool IsValid() const { return operation_.size() != 0; }

  // Returns the operation part of the disassembly, without any prefixes:
  //   "pop" eax
  //   lock "xchg" eax, edx
  const string& operation() const { return operation_; }

  // Returns the destination operand of the disassembly, without memory operand
  // size prefixes:
  //   mov DWORD PTR "[rax + 16]", edx
  const string& dest() const { return dest_; }

  // Returns the source operand of the disassembly, without memory operand
  // size prefixes:
  //   mov rax, QWORD PTR "[rdx]"
  const string& src() const { return src_; }

 private:
  friend class DisassemblerObjdumpForTest;

  // Writes out the provided `raw_bytes` to a temporary file, and executes objdump
  // to disassemble according to `cpu`, which must be either MD_CONTEXT_X86 or
  // MD_CONTEXT_AMD64. Once objdump has completed, parses out the instruction
  // string from the first instruction in the output and stores it in
  // `instruction`.
  static bool DisassembleInstruction(uint32_t cpu, const uint8_t* raw_bytes,
                                     unsigned int raw_bytes_len,
                                     string& instruction);

  // Splits an `instruction` into three parts, the "main" `operation` and
  // the `dest` and `src` operands.
  // Example:
  //   instruction = "lock cmpxchg QWORD PTR [rdi], rsi"
  //   operation = "cmpxchg", dest = "[rdi]", src = "rsi"
  static bool TokenizeInstruction(const string& instruction, string& operation,
                                  string& dest, string& src);

  // Compute the address referenced by `expression` in `context`.
  // Supports memory operands in the form
  //   (segment:)[base_reg(+index_reg*index_stride)(+-offset)]
  // Returns false if evaluation fails, or if the operand is not a supported
  // memory operand.
  static bool CalculateAddress(const DumpContext& context,
                               const string& expression,
                               uint64_t& address);

  // The parsed components of the disassembly for the instruction.
  string operation_ = "";
  string dest_ = "";
  string src_ = "";
};
}  // namespace google_breakpad

#endif  // GOOGLE_BREAKPAD_PROCESSOR_DISASSEMBLER_OBJDUMP_H_