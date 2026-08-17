// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TRACEIMPL_OVERLOADED_ERROR_H_
#define TRACEIMPL_OVERLOADED_ERROR_H_

#include "heap/stubs.h"

namespace blink {

class X : public GarbageCollected<X> {
 public:
  void Trace(Visitor*) const {}
};

class InlinedBase : public GarbageCollected<InlinedBase> {
 public:
  virtual void Trace(Visitor* visitor) const {
    // Missing visitor->Trace(x_base_).
  }

 private:
  Member<X> x_base_;
};

class InlinedDerived : public InlinedBase {
 public:
  void Trace(Visitor* visitor) const override {
    // Missing visitor->Trace(x_derived_) and InlinedBase::Trace(visitor).
  }

 private:
  Member<X> x_derived_;
};

class ExternBase : public GarbageCollected<ExternBase> {
 public:
  virtual void Trace(Visitor*) const;

 private:
  Member<X> x_base_;
};

class ExternDerived : public ExternBase {
 public:
  void Trace(Visitor*) const override;

 private:
  Member<X> x_derived_;
};

}

#endif  // TRACEIMPL_OVERLOADED_ERROR_H_
