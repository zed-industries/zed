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

// code_module.h: Carries information about code modules that are loaded
// into a process.
//
// Author: Mark Mentovai

#ifndef GOOGLE_BREAKPAD_PROCESSOR_CODE_MODULE_H__
#define GOOGLE_BREAKPAD_PROCESSOR_CODE_MODULE_H__

#include <string>

#include "common/using_std_string.h"
#include "google_breakpad/common/breakpad_types.h"

namespace google_breakpad {

class CodeModule {
 public:
  virtual ~CodeModule() {}

  // The base address of this code module as it was loaded by the process.
  // (uint64_t)-1 on error.
  virtual uint64_t base_address() const = 0;

  // The size of the code module.  0 on error.
  virtual uint64_t size() const = 0;

  // The path or file name that the code module was loaded from.  Empty on
  // error.
  virtual string code_file() const = 0;

  // An identifying string used to discriminate between multiple versions and
  // builds of the same code module.  This may contain a uuid, timestamp,
  // version number, or any combination of this or other information, in an
  // implementation-defined format.  Empty on error.
  virtual string code_identifier() const = 0;

  // The filename containing debugging information associated with the code
  // module.  If debugging information is stored in a file separate from the
  // code module itself (as is the case when .pdb or .dSYM files are used),
  // this will be different from code_file.  If debugging information is
  // stored in the code module itself (possibly prior to stripping), this
  // will be the same as code_file.  Empty on error.
  virtual string debug_file() const = 0;

  // An identifying string similar to code_identifier, but identifies a
  // specific version and build of the associated debug file.  This may be
  // the same as code_identifier when the debug_file and code_file are
  // identical or when the same identifier is used to identify distinct
  // debug and code files.
  virtual string debug_identifier() const = 0;

  // A human-readable representation of the code module's version.  Empty on
  // error.
  virtual string version() const = 0;

  // Creates a new copy of this CodeModule object, which the caller takes
  // ownership of.  The new CodeModule may be of a different concrete class
  // than the CodeModule being copied, but will behave identically to the
  // copied CodeModule as far as the CodeModule interface is concerned.
  virtual CodeModule* Copy() const = 0;

  // Getter and setter for shrink_down_delta.  This is used when the address
  // range for a module is shrunk down due to address range conflicts with
  // other modules.  The base_address and size fields are not updated and they
  // should always reflect the original values (reported in the minidump).
  virtual uint64_t shrink_down_delta() const = 0;
  virtual void SetShrinkDownDelta(uint64_t shrink_down_delta) = 0;

  // Whether the module was unloaded from memory.
  virtual bool is_unloaded() const = 0;
};

}  // namespace google_breakpad

#endif  // GOOGLE_BREAKPAD_PROCESSOR_CODE_MODULE_H__
