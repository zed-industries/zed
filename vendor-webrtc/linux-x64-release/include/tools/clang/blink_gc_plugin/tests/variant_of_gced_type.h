// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_BLINK_GC_PLUGIN_TESTS_VARIANT_OF_GCED_TYPE_H_
#define TOOLS_CLANG_BLINK_GC_PLUGIN_TESTS_VARIANT_OF_GCED_TYPE_H_

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

#endif  // TOOLS_CLANG_BLINK_GC_PLUGIN_TESTS_VARIANT_OF_GCED_TYPE_H_
