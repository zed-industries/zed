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

// microdump.h: A microdump reader.  Microdump is a minified variant of a
// minidump (see minidump.h for documentation) which contains the minimum
// amount of information required to get a stack trace for the crashing thread.
// The information contained in a microdump is:
// - the crashing thread stack
// - system information (os type / version)
// - cpu context (state of the registers)
// - list of mmaps

#ifndef GOOGLE_BREAKPAD_PROCESSOR_MICRODUMP_H__
#define GOOGLE_BREAKPAD_PROCESSOR_MICRODUMP_H__

#include <memory>
#include <string>
#include <vector>

#include "common/using_std_string.h"
#include "google_breakpad/processor/dump_context.h"
#include "google_breakpad/processor/memory_region.h"
#include "google_breakpad/processor/system_info.h"
#include "processor/basic_code_modules.h"

namespace google_breakpad {

// MicrodumpModuleList contains all of the loaded code modules for a process
// in the form of MicrodumpModules.  It maintains a vector of these modules
// and provides access to a code module corresponding to a specific address.
class MicrodumpModules : public BasicCodeModules {
 public:
  // Takes over ownership of |module|.
  void Add(const CodeModule* module);

  // Enables/disables module address range shrink.
  void SetEnableModuleShrink(bool is_enabled);
};

// MicrodumpContext carries a CPU-specific context.
// See dump_context.h for documentation.
class MicrodumpContext : public DumpContext {
 public:
  virtual void SetContextARM(MDRawContextARM* arm);
  virtual void SetContextARM64(MDRawContextARM64* arm64);
  virtual void SetContextX86(MDRawContextX86* x86);
  virtual void SetContextMIPS(MDRawContextMIPS* mips32);
  virtual void SetContextMIPS64(MDRawContextMIPS* mips64);
};

// This class provides access to microdump memory regions.
// See memory_region.h for documentation.
class MicrodumpMemoryRegion : public MemoryRegion {
 public:
  MicrodumpMemoryRegion();
  virtual ~MicrodumpMemoryRegion() {}

  // Set this region's address and contents. If we have placed an
  // instance of this class in a test fixture class, individual tests
  // can use this to provide the region's contents.
  void Init(uint64_t base_address, const std::vector<uint8_t>& contents);

  virtual uint64_t GetBase() const;
  virtual uint32_t GetSize() const;

  virtual bool GetMemoryAtAddress(uint64_t address, uint8_t* value) const;
  virtual bool GetMemoryAtAddress(uint64_t address, uint16_t* value) const;
  virtual bool GetMemoryAtAddress(uint64_t address, uint32_t* value) const;
  virtual bool GetMemoryAtAddress(uint64_t address, uint64_t* value) const;

  // Print a human-readable representation of the object to stdout.
  virtual void Print() const;

 private:
  // Fetch a little-endian value from ADDRESS in contents_ whose size
  // is BYTES, and store it in *VALUE.  Returns true on success.
  template<typename ValueType>
  bool GetMemoryLittleEndian(uint64_t address, ValueType* value) const;

  uint64_t base_address_;
  std::vector<uint8_t> contents_;
};

// Microdump is the user's interface to a microdump file.  It provides access to
// the microdump's context, memory regions and modules.
class Microdump {
 public:
  explicit Microdump(const string& contents);
  virtual ~Microdump() {}

  DumpContext* GetContext() { return context_.get(); }
  MicrodumpMemoryRegion* GetMemory() { return stack_region_.get(); }
  MicrodumpModules* GetModules() { return modules_.get(); }
  SystemInfo* GetSystemInfo() { return system_info_.get(); }

  string GetCrashReason() { return crash_reason_; }
  uint64_t GetCrashAddress() { return crash_address_; }
 private:
  std::unique_ptr<MicrodumpContext> context_;
  std::unique_ptr<MicrodumpMemoryRegion> stack_region_;
  std::unique_ptr<MicrodumpModules> modules_;
  std::unique_ptr<SystemInfo> system_info_;
  string crash_reason_;
  uint64_t crash_address_;
};

}  // namespace google_breakpad

#endif  // GOOGLE_BREAKPAD_PROCESSOR_MICRODUMP_H__
