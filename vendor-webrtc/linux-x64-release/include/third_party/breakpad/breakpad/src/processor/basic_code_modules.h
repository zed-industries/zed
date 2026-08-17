// Copyright 2006 Google LLC
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

// basic_code_modules.h: Contains all of the CodeModule objects that
// were loaded into a single process.
//
// This is a basic concrete implementation of CodeModules.  It cannot be
// instantiated directly, only based on other objects that implement
// the CodeModules interface.  It exists to provide a CodeModules
// implementation a place to store information when the life of the original
// object (such as a MinidumpModuleList) cannot be guaranteed.
//
// Author: Mark Mentovai

#ifndef PROCESSOR_BASIC_CODE_MODULES_H__
#define PROCESSOR_BASIC_CODE_MODULES_H__

#include <vector>

#include "google_breakpad/processor/code_modules.h"
#include "processor/linked_ptr.h"
#include "processor/range_map.h"

namespace google_breakpad {

class BasicCodeModules : public CodeModules {
 public:
  // Creates a new BasicCodeModules object given any existing CodeModules
  // implementation.  This is useful to make a copy of the data relevant to
  // the CodeModules and CodeModule interfaces without requiring all of the
  // resources that other implementations may require.  A copy will be
  // made of each contained CodeModule using CodeModule::Copy.
  BasicCodeModules(const CodeModules *that, MergeRangeStrategy strategy);

  virtual ~BasicCodeModules();

  // See code_modules.h for descriptions of these methods.
  virtual unsigned int module_count() const;
  virtual const CodeModule* GetModuleForAddress(uint64_t address) const;
  virtual const CodeModule* GetMainModule() const;
  virtual const CodeModule* GetModuleAtSequence(unsigned int sequence) const;
  virtual const CodeModule* GetModuleAtIndex(unsigned int index) const;
  virtual const CodeModules* Copy() const;
  virtual std::vector<linked_ptr<const CodeModule> >
  GetShrunkRangeModules() const;

 protected:
  BasicCodeModules();

  // The base address of the main module.
  uint64_t main_address_;

  // The map used to contain each CodeModule, keyed by each CodeModule's
  // address range.
  RangeMap<uint64_t, linked_ptr<const CodeModule> > map_;

  // A vector of all CodeModules that were shrunk downs due to
  // address range conflicts.
  std::vector<linked_ptr<const CodeModule> > shrunk_range_modules_;

 private:
  // Disallow copy constructor and assignment operator.
  BasicCodeModules(const BasicCodeModules& that);
  void operator=(const BasicCodeModules& that);
};

}  // namespace google_breakpad

#endif  // PROCESSOR_BASIC_CODE_MODULES_H__
