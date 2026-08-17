// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef CLASS_OVERRIDES_NEW_H_
#define CLASS_OVERRIDES_NEW_H_

#include "heap/stubs.h"

namespace blink {

class HeapObject : public GarbageCollected<HeapObject> {
 public:
  void* operator new(size_t);
  void Trace(Visitor*) const {}
};

class HeapObjectBase : public GarbageCollected<HeapObjectBase> {
 public:
  virtual ~HeapObjectBase() = default;
  virtual void Trace(Visitor*) const {}
};

class HeapObjectDerived : public HeapObjectBase {
 public:
  void* operator new(size_t);
  void Trace(Visitor*) const override;
};
}

#endif
