// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TRACEIMPL_ERROR_H_
#define TRACEIMPL_ERROR_H_

#include "heap/stubs.h"

namespace blink {

class X : public GarbageCollected<X> {
 public:
  virtual void Trace(Visitor*) const {}
};

class TraceImplInlinedWithUntracedMember
    : public GarbageCollected<TraceImplInlinedWithUntracedMember> {
 public:
  void Trace(Visitor* visitor) const {
    // Empty; should get complaints from the plugin for untraced x_.
  }

 private:
  Member<X> x_;
};

class TraceImplExternWithUntracedMember
    : public GarbageCollected<TraceImplExternWithUntracedMember> {
 public:
  void Trace(Visitor* visitor) const;

 private:
  Member<X> x_;
};

class Base : public GarbageCollected<Base> {
 public:
  virtual void Trace(Visitor*) const {}
};

class TraceImplInlineWithUntracedBase : public Base {
 public:
  void Trace(Visitor* visitor) const override {
    // Empty; should get complaints from the plugin for untraced Base.
  }
};

class TraceImplExternWithUntracedBase : public Base {
 public:
  void Trace(Visitor*) const override;
};

}

#endif  // TRACEIMPL_ERROR_H_
