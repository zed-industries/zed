// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef POLYMORPHIC_CLASS_WITH_NON_VIRTUAL_TRACE_H_
#define POLYMORPHIC_CLASS_WITH_NON_VIRTUAL_TRACE_H_

#include "heap/stubs.h"

namespace blink {

class HeapObject : public GarbageCollected<HeapObject> {
public:
 void Trace(Visitor*) const {}
};

class NonPolymorphicBase {
};

class PolymorphicBase {
public:
    virtual void foo();
};

class IsLeftMostPolymorphic
    : public GarbageCollected<IsLeftMostPolymorphic>,
      public PolymorphicBase {
public:
 void Trace(Visitor*) const;

private:
    Member<HeapObject> m_obj;
};

class IsNotLeftMostPolymorphic
    : public GarbageCollected<IsNotLeftMostPolymorphic>,
      public NonPolymorphicBase,
      public PolymorphicBase {
public:
 void Trace(Visitor*) const;

private:
    Member<HeapObject> m_obj;
};

template<typename T>
class TemplatedNonPolymorphicBase
    : public GarbageCollected<TemplatedNonPolymorphicBase<T> > {
public:
 void Trace(Visitor* visitor) const { visitor->Trace(m_obj); }

private:
    Member<HeapObject> m_obj;
};

// Looks OK, but will result in an incorrect object pointer when marking.
class TemplatedIsNotLeftMostPolymorphic
    : public TemplatedNonPolymorphicBase<TemplatedIsNotLeftMostPolymorphic>,
      public PolymorphicBase {
};

}

#endif
