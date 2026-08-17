// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef WEAK_FIELDS_REQUIRE_TRACING_H_
#define WEAK_FIELDS_REQUIRE_TRACING_H_

#include "heap/stubs.h"

namespace blink {

class HeapObject : public GarbageCollected<HeapObject> {
public:
 void Trace(Visitor*) const;
 void clearWeakMembers(Visitor*);

private:
    Member<HeapObject> m_obj1;
    WeakMember<HeapObject> m_obj2;
    WeakMember<HeapObject> m_obj3;
    HeapHashSet<WeakMember<HeapObject> > m_set1;
    HeapHashSet<WeakMember<HeapObject> > m_set2;
};

}

#endif
