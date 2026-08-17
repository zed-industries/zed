// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef MAKE_UNIQUE_GC_OBJECT_H_
#define MAKE_UNIQUE_GC_OBJECT_H_

#include "heap/stubs.h"

namespace blink {

class Base : public GarbageCollected<Base> {
 public:
  virtual void Trace(Visitor*) const {}
};

class Derived : public Base {
 public:
  void Trace(Visitor* visitor) const override { Base::Trace(visitor); }
};

class Mixin : public GarbageCollectedMixin {
 public:
  void Trace(Visitor*) const {}
};

}  // namespace blink

#endif  // MAKE_UNIQUE_GC_OBJECT_H_
