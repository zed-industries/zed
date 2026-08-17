// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TRACEIMPL_DEPENDENT_SCOPE_H_
#define TRACEIMPL_DEPENDENT_SCOPE_H_

#include "heap/stubs.h"

namespace blink {

class X : public GarbageCollected<X> {
 public:
  virtual void Trace(Visitor*) const {}
};

class Y : public GarbageCollected<Y> {
 public:
  virtual void Trace(Visitor*) const {}
};

template <typename T>
class Base : public GarbageCollected<Base<T> > {
 public:
  virtual void Trace(Visitor* visitor) const {}
};

template <typename T>
class Derived : public Base<T> {
 public:
  void Trace(Visitor* visitor) const override { Base<T>::Trace(visitor); }
};

template <typename T>
class DerivedMissingTrace : public Base<T> {
 public:
  void Trace(Visitor* visitor) const override {
    // Missing Base<T>::Trace(visitor).
  }
};

template <typename T>
class Mixin : T {
 public:
  void Trace(Visitor* visitor) const override { T::Trace(visitor); }
};

template <typename T>
class MixinMissingTrace : T {
 public:
  void Trace(Visitor* visitor) const override {
    // Missing T::Trace(visitor).
  }
};

template <typename T1, typename T2>
class MixinTwoBases : T1, T2 {
 public:
  void Trace(Visitor* visitor) const override {
    T1::Trace(visitor);
    T2::Trace(visitor);
  }
};

template <typename T1, typename T2>
class MixinTwoBasesMissingTrace : T1, T2 {
 public:
  void Trace(Visitor* visitor) const override {
    T1::Trace(visitor);
    // Missing T2::Trace(visitor).
  }
};
}

#endif  // TRACEIMPL_DEPENDENT_SCOPE_H_
