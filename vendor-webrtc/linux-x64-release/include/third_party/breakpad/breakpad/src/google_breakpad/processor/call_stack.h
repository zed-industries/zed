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

// call_stack.h: A call stack comprised of stack frames.
//
// This class manages a vector of stack frames.  It is used instead of
// exposing the vector directly to allow the CallStack to own StackFrame
// pointers without having to publicly export the linked_ptr class.  A
// CallStack must be composed of pointers instead of objects to allow for
// CPU-specific StackFrame subclasses.
//
// By convention, the stack frame at index 0 is the innermost callee frame,
// and the frame at the highest index in a call stack is the outermost
// caller.  CallStack only allows stacks to be built by pushing frames,
// beginning with the innermost callee frame.
//
// Author: Mark Mentovai

#ifndef GOOGLE_BREAKPAD_PROCESSOR_CALL_STACK_H__
#define GOOGLE_BREAKPAD_PROCESSOR_CALL_STACK_H__

#include <stdint.h>

#include <vector>

namespace google_breakpad {

using std::vector;

struct StackFrame;
template<typename T> class linked_ptr;

class CallStack {
 public:
  CallStack() { Clear(); }
  ~CallStack();

  // Resets the CallStack to its initial empty state
  void Clear();

  const vector<StackFrame*>* frames() const { return &frames_; }

  // Set the TID associated with this call stack.
  void set_tid(uint32_t tid) { tid_ = tid; }

  uint32_t tid() const { return tid_; }

 private:
  // Stackwalker is responsible for building the frames_ vector.
  friend class Stackwalker;

  // Storage for pushed frames.
  vector<StackFrame*> frames_;

  // The TID associated with this call stack. Default to 0 if it's not
  // available.
  uint32_t tid_;
};

}  // namespace google_breakpad

#endif  // GOOGLE_BREAKPAD_PROCSSOR_CALL_STACK_H__
