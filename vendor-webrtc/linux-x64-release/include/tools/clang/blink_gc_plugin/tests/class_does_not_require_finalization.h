// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef CLASS_DOES_NOT_REQUIRE_FINALIZATION_BASE_H_
#define CLASS_DOES_NOT_REQUIRE_FINALIZATION_BASE_H_

#include "heap/stubs.h"

namespace blink {

class DoesNeedFinalizer : public GarbageCollected<DoesNeedFinalizer> {
 public:
  ~DoesNeedFinalizer() { ; }
  void Trace(Visitor*) const;
};

class DoesNotNeedFinalizer : public GarbageCollected<DoesNotNeedFinalizer> {
 public:
  void Trace(Visitor*) const;
};

class DoesNotNeedFinalizer2 : public GarbageCollected<DoesNotNeedFinalizer2> {
 public:
  ~DoesNotNeedFinalizer2();
  void Trace(Visitor*) const;
};

class HasEmptyDtor {
public:
    virtual ~HasEmptyDtor() { }
};

// If there are any virtual destructors involved, give up.

class DoesNeedFinalizer2 : public GarbageCollected<DoesNeedFinalizer2>,
                           public HasEmptyDtor {
 public:
  void Trace(Visitor*) const;
};
}

#endif
