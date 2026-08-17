// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef FINALIZE_AFTER_DISPATCH_H_
#define FINALIZE_AFTER_DISPATCH_H_

#include "heap/stubs.h"

namespace blink {

class NeedsDispatch : public GarbageCollected<NeedsDispatch> {
 public:
  void Trace(Visitor*) const;
  // Needs a TraceAfterDispatch method.
  void FinalizeGarbageCollectedObject() {}

 protected:
  NeedsDispatch() = default;
};

class NeedsFinalizedBase : public GarbageCollected<NeedsFinalizedBase> {
 public:
  void Trace(Visitor*) const {}
  void TraceAfterDispatch(Visitor*) const {}
  void FinalizeGarbageCollectedObject() {}

 protected:
  NeedsFinalizedBase() = default;
};

class A : GarbageCollected<A> {
 public:
  void Trace(Visitor*) const;
  void TraceAfterDispatch(Visitor*) const;
  void FinalizeGarbageCollectedObject();

 protected:
  enum Type { TB, TC, TD, TE };
  A(Type type) : m_type(type) {}

 private:
  Type m_type;
};

class B : public A {
public:
    B() : A(TB) { }
    ~B() { }
    void TraceAfterDispatch(Visitor*) const;

   private:
    Member<A> m_a;
};

class C : public A {
public:
    C() : A(TC) { }
    void TraceAfterDispatch(Visitor*) const;

   private:
    Member<A> m_a;
};

// This class is considered abstract does not need to be dispatched to.
class Abstract : public A {
protected:
    Abstract(Type type) : A(type) { }
};

class D : public Abstract {
public:
    D() : Abstract(TD) { }
    void TraceAfterDispatch(Visitor*) const;

   private:
    Member<A> m_a;
};

class E : public A {
 public:
  E() : A(TE) {}
  void TraceAfterDispatch(Visitor*) const;
  void FinalizeGarbageCollectedObject();

 private:
  Member<A> m_a;
};
}

#endif
