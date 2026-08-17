// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef STACK_ALLOCATED_H_
#define STACK_ALLOCATED_H_

#include "heap/stubs.h"

namespace blink {

class HeapObject;

class PartObject {
    DISALLOW_NEW();
private:
    Member<HeapObject> m_obj; // Needs tracing.
};

class StackObject {
    STACK_ALLOCATED();

    // Redundant trace() method, warning/error expected.
    void Trace(Visitor* visitor) const { visitor->Trace(m_obj); }

   private:
    HeapObject* m_obj; // Does not need tracing.
};

class HeapObject : public GarbageCollected<HeapObject> {
public:
 void Trace(Visitor*) const;

private:
    StackObject m_part; // Cannot embed a stack allocated object.
};

// Cannot derive from both heap- and stack-allocated objects.
class DerivedHeapObject : public HeapObject, public StackObject {
};

// Cannot be stack-allocated and derive from a heap-allocated object.
class DerivedHeapObject2 : public HeapObject {
  STACK_ALLOCATED();
};

// STACK_ALLOCATED is inherited.
class DerivedStackObject : public StackObject {
private:
    StackObject m_anotherPart; // Also fine.
};

class IgnoringStackAllocated {
 private:
  STACK_ALLOCATED_IGNORE("for testing") StackObject m_stackObject;
};
}

#endif
