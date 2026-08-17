// Copyright 2013 Google LLC
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

// stackwalker_address_list.h: a pseudo stackwalker.
//
// Doesn't actually walk a stack, rather initializes a CallStack given an
// explicit list of already walked return addresses.
//
// Author: Chris Hamilton <chrisha@chromium.org>

#ifndef PROCESSOR_STACKWALKER_ADDRESS_LIST_H_
#define PROCESSOR_STACKWALKER_ADDRESS_LIST_H_

#include "google_breakpad/common/breakpad_types.h"
#include "google_breakpad/processor/stackwalker.h"

namespace google_breakpad {

class CodeModules;

class StackwalkerAddressList : public Stackwalker {
 public:
  // Initializes this stack walker with an explicit set of frame addresses.
  // |modules| and |frame_symbolizer| are passed directly through to the base
  // Stackwalker constructor.
  StackwalkerAddressList(const uint64_t* frames,
                         size_t frame_count,
                         const CodeModules* modules,
                         StackFrameSymbolizer* frame_symbolizer);
  StackwalkerAddressList(const StackwalkerAddressList&) = delete;
  void operator=(const StackwalkerAddressList&) = delete;

 private:
  // Implementation of Stackwalker.
  virtual StackFrame* GetContextFrame();
  virtual StackFrame* GetCallerFrame(const CallStack* stack,
                                     bool stack_scan_allowed);

  const uint64_t* frames_;
  size_t frame_count_;
  size_t next_frame_index_;
};

}  // namespace google_breakpad

#endif  // PROCESSOR_STACKWALKER_ADDRESS_LIST_H_
