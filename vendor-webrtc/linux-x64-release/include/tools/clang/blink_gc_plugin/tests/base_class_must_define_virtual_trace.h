// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_CLASS_MUST_DEFINE_VIRTUAL_TRACE_H_
#define BASE_CLASS_MUST_DEFINE_VIRTUAL_TRACE_H_

#include "heap/stubs.h"

namespace blink {

class PartBase {
    DISALLOW_NEW();
    // Missing virtual Trace.
};

class PartDerived : public PartBase {
    DISALLOW_NEW();
public:
 virtual void Trace(Visitor*) const;
};

class HeapBase : public GarbageCollected<HeapBase> {
    // Missing virtual Trace.
};


class HeapDerived : public HeapBase {
public:
 virtual void Trace(Visitor*) const;

private:
    PartDerived m_part;
};


}

#endif
