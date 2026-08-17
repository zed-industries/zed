// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TRACE_TEMPLATED_SUPER_H_
#define TRACE_TEMPLATED_SUPER_H_

#include "heap/stubs.h"

namespace blink {

class HeapObject;

class Mixin : public GarbageCollectedMixin {
public:
 virtual void Trace(Visitor*) const override {}
};

template<typename T>
class Super : public GarbageCollected<Super<T> >, public Mixin {
public:
 virtual void Trace(Visitor*) const override;
 void clearWeakMembers(Visitor*);

private:
    Member<HeapObject> m_obj;
    WeakMember<HeapObject> m_weak;
};

template<typename T>
class Sub : public Super<T> {
public:
 virtual void Trace(Visitor* visitor) const override;

private:
    Member<HeapObject> m_obj;
};

class HeapObject : public Sub<HeapObject> {
public:
 virtual void Trace(Visitor*) const override;

private:
    Member<HeapObject> m_obj;
};

}

#endif
