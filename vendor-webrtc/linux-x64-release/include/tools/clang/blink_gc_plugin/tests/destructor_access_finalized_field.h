// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef DESTRUCTOR_ACCESS_FINALIZED_FIELD_H_
#define DESTRUCTOR_ACCESS_FINALIZED_FIELD_H_

#include "heap/stubs.h"

namespace blink {

class Other : public RefCounted<Other> {
public:
    bool foo() { return true; }
};

class HeapObject;

class PartOther {
  DISALLOW_NEW();

 public:
  void Trace(Visitor*) const;

  HeapObject* obj() { return m_obj; }

 private:
  Member<HeapObject> m_obj;
};

class HeapObject : public GarbageCollected<HeapObject> {
 public:
  ~HeapObject();
  void Trace(Visitor*) const;
  bool foo() { return true; }
  void bar(HeapObject*) {}

 private:
  scoped_refptr<Other> m_ref;
  Member<HeapObject> m_obj;
  HeapVector<Member<HeapObject>> m_objs;
  PartOther m_part;
};
}

#endif
