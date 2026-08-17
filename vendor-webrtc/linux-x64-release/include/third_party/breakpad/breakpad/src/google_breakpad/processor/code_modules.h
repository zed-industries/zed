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

// code_modules.h: Contains all of the CodeModule objects that were loaded
// into a single process.
//
// Author: Mark Mentovai

#ifndef GOOGLE_BREAKPAD_PROCESSOR_CODE_MODULES_H__
#define GOOGLE_BREAKPAD_PROCESSOR_CODE_MODULES_H__

#include <vector>

#include "google_breakpad/common/breakpad_types.h"
#include "processor/linked_ptr.h"

namespace google_breakpad {

class CodeModule;

class CodeModules {
 public:
  virtual ~CodeModules() {}

  // The number of contained CodeModule objects.
  virtual unsigned int module_count() const = 0;

  // Random access to modules.  Returns the module whose code is present
  // at the address indicated by |address|.  If no module is present at this
  // address, returns NULL.  Ownership of the returned CodeModule is retained
  // by the CodeModules object; pointers returned by this method are valid for
  // comparison with pointers returned by the other Get methods.
  virtual const CodeModule* GetModuleForAddress(uint64_t address) const = 0;

  // Returns the module corresponding to the main executable.  If there is
  // no main executable, returns NULL.  Ownership of the returned CodeModule
  // is retained by the CodeModules object; pointers returned by this method
  // are valid for comparison with pointers returned by the other Get
  // methods.
  virtual const CodeModule* GetMainModule() const = 0;

  // Sequential access to modules.  A sequence number of 0 corresponds to the
  // module residing lowest in memory.  If the sequence number is out of
  // range, returns NULL.  Ownership of the returned CodeModule is retained
  // by the CodeModules object; pointers returned by this method are valid for
  // comparison with pointers returned by the other Get methods.
  virtual const CodeModule* GetModuleAtSequence(
      unsigned int sequence) const = 0;

  // Sequential access to modules.  This is similar to GetModuleAtSequence,
  // except no ordering requirement is enforced.  A CodeModules implementation
  // may return CodeModule objects from GetModuleAtIndex in any order it
  // wishes, provided that the order remain the same throughout the life of
  // the CodeModules object.  Typically, GetModuleAtIndex would be used by
  // a caller to enumerate all CodeModule objects quickly when the enumeration
  // does not require any ordering.  If the index argument is out of range,
  // returns NULL.  Ownership of the returned CodeModule is retained by
  // the CodeModules object; pointers returned by this method are valid for
  // comparison with pointers returned by the other Get methods.
  virtual const CodeModule* GetModuleAtIndex(unsigned int index) const = 0;

  // Creates a new copy of this CodeModules object, which the caller takes
  // ownership of.  The new object will also contain copies of the existing
  // object's child CodeModule objects.  The new CodeModules object may be of
  // a different concrete class than the object being copied, but will behave
  // identically to the copied object as far as the CodeModules and CodeModule
  // interfaces are concerned, except that the order that GetModuleAtIndex
  // returns objects in may differ between a copy and the original CodeModules
  // object.
  virtual const CodeModules* Copy() const = 0;

  // Returns a vector of all modules which address ranges needed to be shrunk
  // down due to address range conflicts with other modules.
  virtual std::vector<linked_ptr<const CodeModule> >
  GetShrunkRangeModules() const = 0;
};

}  // namespace google_breakpad

#endif  // GOOGLE_BREAKPAD_PROCESSOR_CODE_MODULES_H__
