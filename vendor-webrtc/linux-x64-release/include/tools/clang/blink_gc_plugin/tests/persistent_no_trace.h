// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PERSISTENT_NO_TRACE_H_
#define PERSISTENT_NO_TRACE_H_

#include "heap/stubs.h"

namespace blink {

class HeapObject : public GarbageCollected<HeapObject> {
 public:
  void Trace(Visitor*) const;
};

class Object {
 public:
  void Trace(Visitor*) const;

 private:
  Persistent<HeapObject> m_persistent;
  WeakPersistent<HeapObject> m_weakPersistent;
  CrossThreadPersistent<HeapObject> m_crossThreadPersistent;
  CrossThreadWeakPersistent<HeapObject> m_crossThreadWeakPersistent;
};
}

#endif
