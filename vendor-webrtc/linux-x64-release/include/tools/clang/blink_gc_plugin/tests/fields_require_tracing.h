// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef FIELDS_REQUIRE_TRACING_H_
#define FIELDS_REQUIRE_TRACING_H_

#include "heap/stubs.h"

namespace blink {

class HeapObject;
class PartObject;

class PartBObject {
    DISALLOW_NEW();
public:
 void Trace(Visitor*) const;

private:
    HeapHashSet<PartBObject> m_set;
    HeapVector<PartBObject> m_vector;
};

class PartObject {
    DISALLOW_NEW();
public:
 void Trace(Visitor*) const;

private:
    Member<HeapObject> m_obj1;
    Member<HeapObject> m_obj2;
    Member<HeapObject> m_obj3;

    HeapVector<PartBObject> m_parts;
};

class HeapObject : public GarbageCollected<HeapObject> {
  static constexpr int array_size = 2;
public:
 void Trace(Visitor*) const;

private:
    PartObject m_part;
    Member<HeapObject> m_array1[array_size];
    std::array<Member<HeapObject>, array_size> m_array2;
    Member<HeapObject> m_traced_array1[array_size];
    Member<HeapObject> m_traced_array2[array_size];
    Member<HeapObject> m_traced_array3[array_size];
    std::array<Member<HeapObject>, array_size> m_traced_array4;
    std::array<Member<HeapObject>, array_size> m_traced_array5;
    Member<HeapObject> m_obj;

    Member<HeapObject> m_key;
    Member<HeapObject> m_value;
};

}

#endif
