// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef VIRTUAL_AND_TRACE_AFTER_DISPATCH_H_
#define VIRTUAL_AND_TRACE_AFTER_DISPATCH_H_

#include "heap/stubs.h"

namespace blink {

class A : public GarbageCollected<A> {
public:
 void Trace(Visitor*) const;
 void TraceAfterDispatch(Visitor*) const;

protected:
 enum Type { TB };
 A(Type type) : m_type(type) {}

private:
    Type m_type;
};

class B : public A {
public:
    B() : A(TB) { }
    void TraceAfterDispatch(Visitor*) const;
    virtual void foo() { }
private:
    Member<A> m_a;
};

}

#endif
