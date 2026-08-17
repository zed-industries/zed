// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TRACE_WRAPPER_H_
#define TRACE_WRAPPER_H_

#include "heap/stubs.h"

namespace v8 {
class String;
}

namespace blink {

class A : public GarbageCollected<A> {
 public:
  void Trace(Visitor*) const {
    // Missing visitor->Trace(str_);
  }

 private:
  TraceWrapperV8Reference<v8::String> str_;
};

class B : public GarbageCollected<B> {
 public:
  void Trace(Visitor* visitor) const;
  void TraceAfterDispatch(Visitor*) const {}

 protected:
  B() = default;
};

class C : public B {
 public:
  void TraceAfterDispatch(Visitor*) const {
    // Missing visitor->Trace(str_);
  }

 private:
  TraceWrapperV8Reference<v8::String> str_;
};

}  // namespace blink

#endif  // TRACE_WRAPPER_H_
