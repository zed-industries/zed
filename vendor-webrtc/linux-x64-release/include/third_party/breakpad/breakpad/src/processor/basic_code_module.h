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

// basic_code_module.h: Carries information about code modules that are loaded
// into a process.
//
// This is a basic concrete implementation of CodeModule.  It cannot be
// instantiated directly, only based on other objects that implement
// the CodeModule interface.  It exists to provide a CodeModule implementation
// a place to store information when the life of the original object (such as
// a MinidumpModule) cannot be guaranteed.
//
// Author: Mark Mentovai

#ifndef PROCESSOR_BASIC_CODE_MODULE_H__
#define PROCESSOR_BASIC_CODE_MODULE_H__

#include <string>

#include "common/using_std_string.h"
#include "google_breakpad/processor/code_module.h"

namespace google_breakpad {

class BasicCodeModule : public CodeModule {
 public:
  // Creates a new BasicCodeModule given any existing CodeModule
  // implementation.  This is useful to make a copy of the data relevant to
  // the CodeModule interface without requiring all of the resources that
  // other CodeModule implementations may require.
  explicit BasicCodeModule(const CodeModule *that)
      : base_address_(that->base_address()),
        size_(that->size()),
        shrink_down_delta_(that->shrink_down_delta()),
        code_file_(that->code_file()),
        code_identifier_(that->code_identifier()),
        debug_file_(that->debug_file()),
        debug_identifier_(that->debug_identifier()),
        version_(that->version()),
        is_unloaded_(that->is_unloaded()) {}

  BasicCodeModule(uint64_t base_address, uint64_t size,
                  const string& code_file,
                  const string& code_identifier,
                  const string& debug_file,
                  const string& debug_identifier,
                  const string& version,
                  const bool is_unloaded = false)
      : base_address_(base_address),
        size_(size),
        shrink_down_delta_(0),
        code_file_(code_file),
        code_identifier_(code_identifier),
        debug_file_(debug_file),
        debug_identifier_(debug_identifier),
        version_(version),
        is_unloaded_(is_unloaded)
    {}
  virtual ~BasicCodeModule() {}

  // See code_module.h for descriptions of these methods and the associated
  // members.
  virtual uint64_t base_address() const { return base_address_; }
  virtual uint64_t size() const { return size_; }
  virtual uint64_t shrink_down_delta() const { return shrink_down_delta_; }
  virtual void SetShrinkDownDelta(uint64_t shrink_down_delta) {
    shrink_down_delta_ = shrink_down_delta;
  }
  virtual string code_file() const { return code_file_; }
  virtual string code_identifier() const { return code_identifier_; }
  virtual string debug_file() const { return debug_file_; }
  virtual string debug_identifier() const { return debug_identifier_; }
  virtual string version() const { return version_; }
  virtual CodeModule* Copy() const { return new BasicCodeModule(this); }
  virtual bool is_unloaded() const { return is_unloaded_; }

 private:
  uint64_t base_address_;
  uint64_t size_;
  uint64_t shrink_down_delta_;
  string code_file_;
  string code_identifier_;
  string debug_file_;
  string debug_identifier_;
  string version_;
  bool is_unloaded_;

  // Disallow copy constructor and assignment operator.
  BasicCodeModule(const BasicCodeModule& that);
  void operator=(const BasicCodeModule& that);
};

}  // namespace google_breakpad

#endif  // PROCESSOR_BASIC_CODE_MODULE_H__
