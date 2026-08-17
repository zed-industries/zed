// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef RAW_PTR_TO_GC_MANAGED_CLASS_H_
#define RAW_PTR_TO_GC_MANAGED_CLASS_H_

#include "heap/stubs.h"

namespace blink {

class HeapObject;

class PartObject {
    DISALLOW_NEW();
private:
    PartObject();

    HeapObject* m_rawObj;
    HeapObject& m_refObj;
};

class HeapObject : public GarbageCollected<HeapObject> {
public:
 void Trace(Visitor*) const;

private:
    PartObject m_part;
    HeapVector<HeapObject*> m_objs;
};

}

#endif
