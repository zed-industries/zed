// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef REGISTER_WEAK_MEMBERS_TEMPLATE_H_
#define REGISTER_WEAK_MEMBERS_TEMPLATE_H_

#include "heap/stubs.h"

namespace blink {

class X : public GarbageCollected<X> {
 public:
  void Trace(Visitor* visitor) const {}
};

class HasUntracedWeakMembers : public GarbageCollected<HasUntracedWeakMembers> {
 public:
  void Trace(Visitor* visitor) const {
    visitor->template RegisterWeakMembers<
        HasUntracedWeakMembers, &HasUntracedWeakMembers::ClearWeakMembers>(
        this);
  }

  void ClearWeakMembers(Visitor* visitor);

 private:
  WeakMember<X> x_;
};

}

#endif  // REGISTER_WEAK_MEMBERS_TEMPLATE_H_
