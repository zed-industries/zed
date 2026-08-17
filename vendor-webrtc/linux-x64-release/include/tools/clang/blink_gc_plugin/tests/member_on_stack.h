// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef MEMBER_ON_STACK_H
#define MEMBER_ON_STACK_H

#include "heap/stubs.h"

namespace blink {

class HeapObject : public GarbageCollected<HeapObject> {
 public:
  void Trace(Visitor*) const;

  void DoSomething() {
    Member<HeapObject> strong;
    WeakMember<HeapObject> weak;
    Member<HeapObject>* ptr;
    Member<HeapObject>& ref = strong;
  }
};

class GCedWithMember : public GarbageCollected<GCedWithMember> {
 public:
  void Trace(Visitor*) const;

  Member<HeapObject> member_;
};

}  // namespace blink

#endif  // MEMBER_ON_STACK_H