// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TRACEIMPL_H_
#define TRACEIMPL_H_

#include "heap/stubs.h"

namespace blink {

class X : public GarbageCollected<X> {
 public:
  virtual void Trace(Visitor*) const {}
};

class TraceImplInlined : public GarbageCollected<TraceImplInlined> {
 public:
  void Trace(Visitor* visitor) const { visitor->Trace(x_); }

 private:
  Member<X> x_;
};

class TraceImplExtern : public GarbageCollected<TraceImplExtern> {
 public:
  void Trace(Visitor* visitor) const;

 private:
  Member<X> x_;
};

class Base : public GarbageCollected<Base> {
 public:
  virtual void Trace(Visitor* visitor) const {}
};

class TraceImplBaseInlined : public Base {
 public:
  void Trace(Visitor* visitor) const override { Base::Trace(visitor); }
};

class TraceImplBaseExtern : public Base {
 public:
  void Trace(Visitor* visitor) const override;

 private:
  Member<X> x_;
};

}

#endif
