// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PERSISTENT_FIELD_IN_GC_MANAGED_CLASS_H_
#define PERSISTENT_FIELD_IN_GC_MANAGED_CLASS_H_

#include "heap/stubs.h"

namespace blink {

class HeapObject;

class PartObject {
    DISALLOW_NEW();
private:
    Persistent<HeapObject> m_obj;
};

class HeapObject : public GarbageCollected<HeapObject> {
public:
 explicit HeapObject(Persistent<HeapObject>& ref) : m_ref(ref) {}

 void Trace(Visitor*) const;

private:
    PartObject m_part;
    HeapVector<PartObject> m_parts;
    std::unique_ptr<PartObject> m_unique_part;
    Persistent<HeapVector<Member<HeapObject>>> m_objs;
    WeakPersistent<HeapObject> m_weakPersistent;
    Persistent<HeapObject>& m_ref;
    Persistent<HeapObject>* m_ptr;
};

}

#endif
