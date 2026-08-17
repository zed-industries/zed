// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef MEMBER_IN_OFFHEAP_CLASS_H_
#define MEMBER_IN_OFFHEAP_CLASS_H_

#include "heap/stubs.h"

namespace blink {

class HeapObject : public GarbageCollected<HeapObject> { };

class OffHeapObject {
public:
 OffHeapObject(Member<HeapObject>& ref) : m_ref(ref) {}

 void Trace(Visitor*) const;

private:
    Member<HeapObject> m_obj; // Must not contain Member.
    WeakMember<HeapObject> m_weak;  // Must not contain WeakMember.
    Persistent<HeapVector<Member<HeapObject> > > m_objs; // OK
    Member<HeapObject>* m_ptr;                           // Member may move
    Member<HeapObject>& m_ref;                           // Member may move
};

class StackObject {
  STACK_ALLOCATED();
  StackObject(Member<HeapObject>& ref) : m_ref(ref) {}

 private:
  HeapObject* m_obj;                                        // OK
  Member<HeapObject>* m_ptr;                                // OK
  Member<HeapObject>& m_ref;                                // OK
  HeapVector<Member<OffHeapObject>> m_heapVectorMemberOff;  // NOT OK
};

class DerivedStackObject : public StackObject {
 private:
  HeapObject* m_obj1;                                        // OK
  HeapVector<Member<OffHeapObject>> m_heapVectorMemberOff1;  // NOT OK
};

class PartObject {
  DISALLOW_NEW();

 public:
  virtual void Trace(Visitor*) const;

 private:
  Member<HeapObject> m_obj;  // OK
};

class DerivedPartObject : public PartObject {
 public:
  void Trace(Visitor*) const override;

 private:
  Member<HeapObject> m_obj1;  // OK
};
}

#endif
