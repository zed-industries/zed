// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TRACEIMPL_DERIVED_FROM_TEMPLATED_BASE_H_
#define TRACEIMPL_DERIVED_FROM_TEMPLATED_BASE_H_

#include "heap/stubs.h"

namespace blink {

class X : public GarbageCollected<X> {
 public:
  virtual void Trace(Visitor*) const {}
};

template <int Y>
class TraceImplTemplatedBase
    : public GarbageCollected<TraceImplTemplatedBase<Y> > {
 public:
  void Trace(Visitor* visitor) const { visitor->Trace(x_); }

 private:
  Member<X> x_;
};

class TraceImplDerivedFromTemplatedBase : public TraceImplTemplatedBase<0> {
};

}

#endif  // TRACEIMPL_DERIVED_FROM_TEMPLATED_BASE_H_
