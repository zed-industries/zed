// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_REQUIRES_TRACING_H_
#define BASE_REQUIRES_TRACING_H_

#include "heap/stubs.h"

namespace blink {

class A : public GarbageCollected<A> {
public:
 virtual void Trace(Visitor*) const;
};

class B : public A {
    // Does not need Trace
};

class C : public B {
public:
 void Trace(Visitor*) const;

private:
    Member<A> m_a;
};

class D : public C {
public:
 void Trace(Visitor*) const;

private:
    Member<A> m_a;
};

}

#endif
