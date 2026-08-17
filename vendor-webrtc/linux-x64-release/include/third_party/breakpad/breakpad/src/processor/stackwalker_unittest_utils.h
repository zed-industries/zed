// -*- mode: C++ -*-

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

// Original author: Jim Blandy <jimb@mozilla.com> <jimb@red-bean.com>

// Mock classes for writing stackwalker tests, shared amongst architectures.

#ifndef PROCESSOR_STACKWALKER_UNITTEST_UTILS_H_
#define PROCESSOR_STACKWALKER_UNITTEST_UTILS_H_

#include <assert.h>
#include <stdlib.h>
#include <string>
#include <vector>

#include "common/using_std_string.h"
#include "google_breakpad/common/breakpad_types.h"
#include "google_breakpad/processor/code_module.h"
#include "google_breakpad/processor/code_modules.h"
#include "google_breakpad/processor/memory_region.h"
#include "google_breakpad/processor/symbol_supplier.h"
#include "google_breakpad/processor/system_info.h"
#include "processor/linked_ptr.h"

class MockMemoryRegion: public google_breakpad::MemoryRegion {
 public:
  MockMemoryRegion(): base_address_(0) { }

  // Set this region's address and contents. If we have placed an
  // instance of this class in a test fixture class, individual tests
  // can use this to provide the region's contents.
  void Init(uint64_t base_address, const string& contents) {
    base_address_ = base_address;
    contents_ = contents;
  }

  uint64_t GetBase() const { return base_address_; }
  uint32_t GetSize() const { return contents_.size(); }

  bool GetMemoryAtAddress(uint64_t address, uint8_t*  value) const {
    return GetMemoryLittleEndian(address, value);
  }
  bool GetMemoryAtAddress(uint64_t address, uint16_t* value) const {
    return GetMemoryLittleEndian(address, value);
  }
  bool GetMemoryAtAddress(uint64_t address, uint32_t* value) const {
    return GetMemoryLittleEndian(address, value);
  }
  bool GetMemoryAtAddress(uint64_t address, uint64_t* value) const {
    return GetMemoryLittleEndian(address, value);
  }
  void Print() const {
    assert(false);
  }

 private:
  // Fetch a little-endian value from ADDRESS in contents_ whose size
  // is BYTES, and store it in *VALUE. Return true on success.
  template<typename ValueType>
  bool GetMemoryLittleEndian(uint64_t address, ValueType* value) const {
    if (address < base_address_ ||
        address - base_address_ + sizeof(ValueType) > contents_.size())
      return false;
    ValueType v = 0;
    int start = address - base_address_;
    // The loop condition is odd, but it's correct for size_t.
    for (size_t i = sizeof(ValueType) - 1; i < sizeof(ValueType); i--)
      v = (v << 8) | static_cast<unsigned char>(contents_[start + i]);
    *value = v;
    return true;
  }

  uint64_t base_address_;
  string contents_;
};

class MockCodeModule: public google_breakpad::CodeModule {
 public:
  MockCodeModule(uint64_t base_address, uint64_t size,
                 const string& code_file, const string& version)
      : base_address_(base_address), size_(size), code_file_(code_file) { }

  uint64_t base_address()       const { return base_address_; }
  uint64_t size()               const { return size_; }
  string code_file()        const { return code_file_; }
  string code_identifier()  const { return code_file_; }
  string debug_file()       const { return code_file_; }
  string debug_identifier() const { return code_file_; }
  string version()          const { return version_; }
  google_breakpad::CodeModule* Copy() const {
    abort(); // Tests won't use this.
  }
  virtual bool is_unloaded() const { return false; }
  virtual uint64_t shrink_down_delta() const { return 0; }
  virtual void SetShrinkDownDelta(uint64_t shrink_down_delta) {}

 private:
  uint64_t base_address_;
  uint64_t size_;
  string code_file_;
  string version_;
};

class MockCodeModules: public google_breakpad::CodeModules {
 public:
  typedef google_breakpad::CodeModule CodeModule;
  typedef google_breakpad::CodeModules CodeModules;

  void Add(const MockCodeModule* module) {
    modules_.push_back(module);
  }

  unsigned int module_count() const { return modules_.size(); }

  const CodeModule* GetModuleForAddress(uint64_t address) const {
    for (ModuleVector::const_iterator i = modules_.begin();
         i != modules_.end(); i++) {
      const MockCodeModule* module = *i;
      if (module->base_address() <= address &&
          address - module->base_address() < module->size())
        return module;
    }
    return nullptr;
  };

  const CodeModule* GetMainModule() const { return modules_[0]; }

  const CodeModule* GetModuleAtSequence(unsigned int sequence) const {
    return modules_.at(sequence);
  }

  const CodeModule* GetModuleAtIndex(unsigned int index) const {
    return modules_.at(index);
  }

  CodeModules* Copy() const { abort(); }  // Tests won't use this

  virtual std::vector<google_breakpad::linked_ptr<const CodeModule> >
  GetShrunkRangeModules() const {
    return std::vector<google_breakpad::linked_ptr<const CodeModule> >();
  }

 private:
  typedef std::vector<const MockCodeModule*> ModuleVector;
  ModuleVector modules_;
};

class MockSymbolSupplier: public google_breakpad::SymbolSupplier {
 public:
  typedef google_breakpad::CodeModule CodeModule;
  typedef google_breakpad::SystemInfo SystemInfo;
  MOCK_METHOD3(GetSymbolFile, SymbolResult(const CodeModule* module,
                                           const SystemInfo* system_info,
                                           string* symbol_file));
  MOCK_METHOD4(GetSymbolFile, SymbolResult(const CodeModule* module,
                                           const SystemInfo* system_info,
                                           string* symbol_file,
                                           string* symbol_data));
  MOCK_METHOD5(GetCStringSymbolData, SymbolResult(const CodeModule* module,
                                                  const SystemInfo* system_info,
                                                  string* symbol_file,
                                                  char** symbol_data,
                                                  size_t* symbol_data_size));
  MOCK_METHOD1(FreeSymbolData, void(const CodeModule* module));

  // Copies the passed string contents into a newly allocated buffer.
  // The newly allocated buffer will be freed during destruction.
  char* CopySymbolDataAndOwnTheCopy(const string& info,
                                    size_t* symbol_data_size) {
    *symbol_data_size = info.size() + 1;
    char* symbol_data = new char[*symbol_data_size];
    memcpy(symbol_data, info.c_str(), info.size());
    symbol_data[info.size()] = '\0';
    symbol_data_to_free_.push_back(symbol_data);
    return symbol_data;
  }

  virtual ~MockSymbolSupplier() {
    for (SymbolDataVector::const_iterator i = symbol_data_to_free_.begin();
         i != symbol_data_to_free_.end(); i++) {
      char* symbol_data = *i;
      delete [] symbol_data;
    }
  }

 private:
  // List of symbol data to be freed upon destruction
  typedef std::vector<char*> SymbolDataVector;
  SymbolDataVector symbol_data_to_free_;
};

#endif // PROCESSOR_STACKWALKER_UNITTEST_UTILS_H_
