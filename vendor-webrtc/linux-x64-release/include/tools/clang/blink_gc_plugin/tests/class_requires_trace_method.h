// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef CLASS_REQUIRES_TRACE_METHOD_H_
#define CLASS_REQUIRES_TRACE_METHOD_H_

#include "heap/stubs.h"

namespace blink {

class HeapObject;

class PartObject {
    DISALLOW_NEW();
private:
    Member<HeapObject> m_obj;
};

class HeapObject : public GarbageCollected<HeapObject> {
private:
    PartObject m_part;
};

class Mixin : public GarbageCollectedMixin {
public:
 virtual void Trace(Visitor*) const override;
 Member<Mixin> m_self;
};

class HeapObjectMixin : public GarbageCollected<HeapObjectMixin>, public Mixin {
};

class Mixin2 : public Mixin {
public:
 virtual void Trace(Visitor*) const override;
};

class HeapObjectMixin2
    : public GarbageCollected<HeapObjectMixin2>, public Mixin2 {
};

class Mixin3 : public Mixin {
public:
 virtual void Trace(Visitor*) const override;
};

class HeapObjectMixin3
    : public GarbageCollected<HeapObjectMixin3>, public Mixin {
public:
 virtual void Trace(Visitor*) const override;
};

}

#endif
