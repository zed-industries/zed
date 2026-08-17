// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef CYCLE_SUPER_H_
#define CYCLE_SUPER_H_

#include "heap/stubs.h"

namespace blink {

class D;

// This contains a leaking cycle:
// D -per-> C -sup-> B -sup-> A -ref-> D

class A : public GarbageCollected<A> {
 public:
  virtual void Trace(Visitor*) const;

 private:
  scoped_refptr<D> m_d;
};

class B : public A {
public:
 virtual void Trace(Visitor*) const;
};

class C : public B {
public:
 virtual void Trace(Visitor*) const;
};

class D : public RefCounted<C> {
private:
    Persistent<C> m_c;
};

}

#endif
