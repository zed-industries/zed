// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef CYCLE_SUPER_NEG_H_
#define CYCLE_SUPER_NEG_H_

#include "heap/stubs.h"

namespace blink {

class C;

// The chain:
//   C -per-> B -sup-> A -sub-> D -ref-> C
// is not a leaking cycle, because the super-class relationship
// should not transitively imply sub-class relationships.
// I.e. B -/-> D

class A : public GarbageCollected<A> {
 public:
  virtual void Trace(Visitor*) const {}
};

class B : public A {
public:
 virtual void Trace(Visitor*) const;
};

class C : public RefCounted<C> {
private:
    Persistent<B> m_b;
};

class D : public A {
public:
 virtual void Trace(Visitor*) const;

private:
    scoped_refptr<C> m_c;
};

}

#endif
