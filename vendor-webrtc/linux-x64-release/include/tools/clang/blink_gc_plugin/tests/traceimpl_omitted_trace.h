// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TRACEIMPL_OMITTED_TRACE_H_
#define TRACEIMPL_OMITTED_TRACE_H_

#include "heap/stubs.h"

namespace blink {

class A : public GarbageCollected<A> {
 public:
  virtual void Trace(Visitor* visitor) const {}
};

class B : public A {
  // Trace() isn't necessary because we've got nothing to trace here.
};

class C : public B {
 public:
  void Trace(Visitor* visitor) const override {
    // B::Trace() is actually A::Trace(), and in certain cases we only get
    // limited information like "there is a function call that will be resolved
    // to A::Trace()". We still want to mark B as Traced.
    B::Trace(visitor);
  }
};

}

#endif  // TRACEIMPL_OMITTED_TRACE_H_
