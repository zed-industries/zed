// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef GCED_AS_FIELD_OR_VARIABLE_H_
#define GCED_AS_FIELD_OR_VARIABLE_H_

#include "heap/stubs.h"

namespace blink {

class GCed : public GarbageCollected<GCed> {
 public:
  void Trace(Visitor*) const {}
};

class Mixin : public GarbageCollectedMixin {
 public:
  void Trace(Visitor*) const override {}
};

class OtherGCed : public GarbageCollected<OtherGCed> {
 public:
  void Trace(Visitor* v) const {
    v->Trace(gced_);
    v->Trace(mixin_);
    v->Trace(vector_);
    v->Trace(map_);
  }

 private:
  GCed gced_;
  Mixin mixin_;
  HeapVector<GCed> vector_;     // OK
  HeapHashMap<GCed, int> map_;  // OK
};

}  // namespace blink

#endif  // GCED_AS_FIELD_OR_VARIABLE_H_
