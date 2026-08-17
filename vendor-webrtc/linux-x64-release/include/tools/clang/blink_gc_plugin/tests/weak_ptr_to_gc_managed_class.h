// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef WEAK_PTR_TO_GC_MANAGED_CLASS_H_
#define WEAK_PTR_TO_GC_MANAGED_CLASS_H_

#include "heap/stubs.h"

namespace blink {

class NonGCed;

class Mixin : public GarbageCollectedMixin {
 public:
  void Trace(Visitor*) const override {}

 private:
  base::WeakPtrFactory<Mixin> m_factory{this};
};

class GCed : public GarbageCollected<GCed> {
 public:
  void Trace(Visitor*) const {}

 private:
  base::WeakPtr<GCed> m_gced;
  base::WeakPtr<Mixin> m_mixin;
  base::WeakPtr<NonGCed> m_nongced;  // OK

  base::WeakPtrFactory<GCed> m_factory{this};
};

class NonGCed {
 private:
  base::WeakPtr<GCed> m_gced;
  base::WeakPtr<Mixin> m_mixin;
  base::WeakPtr<NonGCed> m_nongced;  // OK

  GC_PLUGIN_IGNORE("test") base::WeakPtr<GCed> m_ignored_gced;
};

}  // namespace blink

#endif  // WEAK_PTR_TO_GC_MANAGED_CLASS_H_
