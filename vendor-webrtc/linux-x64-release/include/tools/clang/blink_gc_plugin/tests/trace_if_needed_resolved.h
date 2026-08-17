// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TRACE_IF_NEEDED_RESOLVED_H_
#define TRACE_IF_NEEDED_RESOLVED_H_

#include "heap/stubs.h"

namespace blink {

struct Foo {};

class HeapObject : public GarbageCollected<HeapObject> {
 public:
  virtual void Trace(Visitor*) const;

 private:
  Member<HeapObject> m_one;
  int m_two;
  std::unique_ptr<Foo> m_three;
};

}  // namespace blink

#endif
