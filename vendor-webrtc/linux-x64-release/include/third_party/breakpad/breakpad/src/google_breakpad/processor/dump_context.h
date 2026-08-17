// Copyright 2014 Google LLC
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

// dump_context.h: A (mini/micro) dump CPU-specific context.

#ifndef GOOGLE_BREAKPAD_PROCESSOR_DUMP_CONTEXT_H__
#define GOOGLE_BREAKPAD_PROCESSOR_DUMP_CONTEXT_H__

#include "google_breakpad/common/minidump_format.h"
#include "google_breakpad/processor/dump_object.h"

namespace google_breakpad {

// DumpContext carries a CPU-specific MDRawContext structure, which contains CPU
// context such as register states.
class DumpContext : public DumpObject {
 public:
  virtual ~DumpContext();

  // Returns an MD_CONTEXT_* value such as MD_CONTEXT_X86 or MD_CONTEXT_PPC
  // identifying the CPU type that the context was collected from.  The
  // returned value will identify the CPU only, and will have any other
  // MD_CONTEXT_* bits masked out.  Returns 0 on failure.
  uint32_t GetContextCPU() const;

  // Return the raw value of |context_flags_|
  uint32_t GetContextFlags() const;

  // Returns raw CPU-specific context data for the named CPU type.  If the
  // context data does not match the CPU type or does not exist, returns NULL.
  const MDRawContextAMD64*   GetContextAMD64() const;
  const MDRawContextARM*     GetContextARM() const;
  const MDRawContextARM64*   GetContextARM64() const;
  const MDRawContextMIPS*    GetContextMIPS() const;
  const MDRawContextPPC*     GetContextPPC() const;
  const MDRawContextPPC64*   GetContextPPC64() const;
  const MDRawContextSPARC*   GetContextSPARC() const;
  const MDRawContextX86*     GetContextX86() const;
  const MDRawContextRISCV*   GetContextRISCV() const;
  const MDRawContextRISCV64* GetContextRISCV64() const;

  // A convenience method to get the instruction pointer out of the
  // MDRawContext, since it varies per-CPU architecture.
  bool GetInstructionPointer(uint64_t* ip) const;

  // Similar to the GetInstructionPointer method, this method gets the stack
  // pointer for all CPU architectures.
  bool GetStackPointer(uint64_t* sp) const;

  // Print a human-readable representation of the object to stdout.
  void Print();

 protected:
  DumpContext();

  // Sets row CPU-specific context data for the names CPU type.
  void SetContextFlags(uint32_t context_flags);
  void SetContextX86(MDRawContextX86* x86);
  void SetContextPPC(MDRawContextPPC* ppc);
  void SetContextPPC64(MDRawContextPPC64* ppc64);
  void SetContextAMD64(MDRawContextAMD64* amd64);
  void SetContextSPARC(MDRawContextSPARC* ctx_sparc);
  void SetContextARM(MDRawContextARM* arm);
  void SetContextARM64(MDRawContextARM64* arm64);
  void SetContextMIPS(MDRawContextMIPS* ctx_mips);
  void SetContextRISCV(MDRawContextRISCV* riscv);
  void SetContextRISCV64(MDRawContextRISCV64* riscv64);

  // Free the CPU-specific context structure.
  void FreeContext();

 private:
  // The CPU-specific context structure.
  union {
    MDRawContextBase*    base;
    MDRawContextX86*     x86;
    MDRawContextPPC*     ppc;
    MDRawContextPPC64*   ppc64;
    MDRawContextAMD64*   amd64;
    // on Solaris SPARC, sparc is defined as a numeric constant,
    // so variables can NOT be named as sparc
    MDRawContextSPARC*   ctx_sparc;
    MDRawContextARM*     arm;
    MDRawContextARM64*   arm64;
    MDRawContextMIPS*    ctx_mips;
    MDRawContextRISCV*   riscv;
    MDRawContextRISCV64* riscv64;
  } context_;

  // Store this separately because of the weirdo AMD64 context
  uint32_t context_flags_;
};

}  // namespace google_breakpad

#endif  // GOOGLE_BREAKPAD_PROCESSOR_DUMP_CONTEXT_H__
